mod ext;
mod backend;
mod disassembler;

use std::{env, sync::{atomic::{AtomicU32, Ordering}, Arc}};

use backend::{start_emu, Command, Message, SharedInstructionBuffer, SharedProcessorState};
use disassembler::disassemble;
use ext::System;
use imgui::{Condition, ImColor32, InputTextCallback, StyleColor};
use log::LevelFilter;
use ppc750cl::Ins;

fn main() {
    let mut instruction_data = Vec::with_capacity(20);
    instruction_data.resize_with(20, || (AtomicU32::new(0), AtomicU32::new(0)));
    let instruction_buffer: SharedInstructionBuffer = Arc::from(instruction_data);
    let processor_state = Arc::new(SharedProcessorState::new());
    let (emu_commander, emu_messages) = start_emu(env::args().nth(1).unwrap(), instruction_buffer.clone(), processor_state.clone());
    simple_logger::SimpleLogger::new().with_level(LevelFilter::Debug).init().unwrap();
    let mut breakpoint_text = String::new();
    System::new("shmeeeep :3").unwrap().run((), move |_, ui, _| {
	ui.window("Emu control")
	    .size([500.0, 110.0], Condition::FirstUseEver)
	    .build(|| {
		if ui.button("Step") {
		    emu_commander.send(Command::Step).unwrap();
		}
		ui.same_line();
		if ui.button("Run") {
		    emu_commander.send(Command::Run).unwrap();
		}
		ui.same_line();
		if ui.button("Stop") {
		    emu_commander.send(Command::Stop).unwrap();
		}

		ui.input_text("Breakpoint Addr", &mut breakpoint_text).always_overwrite(true).chars_hexadecimal(true).build();
		ui.same_line();
		if ui.button("Add") {
		    let addr = u32::from_str_radix(&breakpoint_text, 16).unwrap();
		    emu_commander.send(Command::Breakpoint(addr)).unwrap();
		}
	    });

	ui.window("Instructions")
	    .size([400.0, 600.0], Condition::Always)
	    .build(|| {
		let mut offset = 0f32;
		for (i, (addr, instr)) in instruction_buffer.iter().enumerate() {
		    let addr = addr.load(Ordering::Relaxed);
		    let instr = Ins::new(instr.load(Ordering::Relaxed), addr);
		    let text = format!("{addr:#010X}: {}", instr.simplified());
					 
		    if i == 5 {
			let style = ui.push_style_color(StyleColor::Text, [0.1, 1.0, 0.1, 1.0]);
			ui.text_wrapped(text);
			style.pop();
		    } else {
			ui.text_wrapped(text);	
		    }
		}
	    });

	ui.window("Processor State")
	    .size([400.0, 700.0], Condition::Always)
	    .build(|| {
		ui.text_wrapped(format!("cia: {:#010X}", processor_state.cia.load(Ordering::Relaxed)));
		ui.text_wrapped(format!("nia: {:#010X}", processor_state.nia.load(Ordering::Relaxed)));
		ui.text_wrapped(format!("lr: {:#010X}", processor_state.lr.load(Ordering::Relaxed)));
		ui.text_wrapped(format!("ctr: {:#010X}", processor_state.ctr.load(Ordering::Relaxed)));
		ui.text_wrapped(format!("srr0: {:#010X}", processor_state.srr0.load(Ordering::Relaxed)));
		ui.text_wrapped(format!("srr1: {:#010X}", processor_state.srr1.load(Ordering::Relaxed)));
		for (i, gpr) in processor_state.gprs.iter().enumerate() {
		    ui.text_wrapped(format!("r{}: {:#010X}", i, gpr.load(Ordering::Relaxed)));
		}
	    });
    }).unwrap();
}
