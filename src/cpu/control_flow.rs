use log::debug;

use crate::Gamecube;

use super::{instr::Instruction, util::sext_26};

pub fn bc(gc: &mut Gamecube, instr: &Instruction) {
    let bo = instr.bo();
    let bi = instr.bi();

    if bo & 0x4 == 0 {
	gc.cpu.ctr -= 1;
    }

    let ctr_ok = ((bo >> 2) & 1) != 0 || (((gc.cpu.ctr != 0) as u8 ^ (bo as u8 >> 1)) & 1) != 0;
    let cond_ok = ((bo >> 4) & 1) != 0 || (((gc.cpu.cr.0 >> (31 - bi)) & 1) as usize == (bo >> 3) & 1);

    if ctr_ok && cond_ok {
	let addr = i32::from((instr.bd() << 2) as i16) as u32;
	
	if instr.aa() {
	    gc.cpu.nia = addr;
	} else {
	    gc.cpu.nia = gc.cpu.cia.wrapping_add(addr);
	}

	if instr.lk() {
	    gc.cpu.lr = gc.cpu.cia.wrapping_add(4);
	}
    }
}

pub fn bclr(gc: &mut Gamecube, instr: &Instruction) {
    let bo = instr.bo();
    let bi = instr.bi();

    if bo & 0x4 == 0 {
	gc.cpu.ctr -= 1;
    }

    let ctr_ok = ((bo >> 2) & 1) != 0 || (((gc.cpu.ctr != 0) as u8 ^ (bo as u8 >> 1)) & 1) != 0;
    let cond_ok = ((bo >> 4) & 1) != 0 || (((gc.cpu.cr.0 >> (31 - bi)) & 1) as usize == (bo >> 3) & 1);


    if ctr_ok && cond_ok {
	gc.cpu.nia = gc.cpu.lr & !3;

	if instr.lk() {
	    gc.cpu.lr = gc.cpu.cia.wrapping_add(4);
	}
    }
}

pub fn b(gc: &mut Gamecube, instr: &Instruction) {
    let addr = sext_26(instr.li() << 2) as u32;

    if instr.aa() {
	gc.cpu.nia = addr;
    } else {
	gc.cpu.nia = gc.cpu.cia.wrapping_add(addr);
    }

    if instr.lk() {
	gc.cpu.lr = gc.cpu.cia.wrapping_add(4);
    }
}

pub fn bcctr(gc: &mut Gamecube, instr: &Instruction) {
    let bo = instr.bo();
    let bi = instr.bi();

    let cond_ok = ((bo >> 4) & 1) != 0 || (((gc.cpu.cr.0 >> (31 - bi)) & 1) as usize == (bo >> 3) & 1);

    if cond_ok {
	gc.cpu.nia = gc.cpu.ctr & !3;
	if instr.lk() {
	    gc.cpu.lr = gc.cpu.cia.wrapping_add(4);
	}
    }
}

pub fn rfi(gc: &mut Gamecube, instr: &Instruction) {
    let mask = 0x87C0_FF73;

    gc.cpu.msr.0 = (gc.cpu.msr.0 & !mask) | (gc.cpu.srr1 & mask);

    gc.cpu.msr.0 &= 0xFFFB_FFFF;

    gc.cpu.nia = gc.cpu.srr0 & 0xFFFF_FFFC;
}
