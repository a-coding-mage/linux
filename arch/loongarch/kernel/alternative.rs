// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the surrounding kernel translation.

pub static mut alternatives_patched: i32 = 0;

const MAX_PATCH_SIZE: usize = (u8::MAX as usize) / LOONGARCH_INSN_SIZE;

static mut debug_alternative: i32 = 0;

unsafe extern "C" {
    static _dummy: i32;
    fn sign_extend64(value: u64, bits: u32) -> i64;
    fn in_alt_jump(jump: usize, start: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> bool;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn wbflush();
    fn flush_icache_range(start: usize, end: usize);
    fn cpu_has(feature: i32) -> bool;
    fn is_pc_ins(insn: *const loongarch_instruction) -> bool;
    fn is_branch_ins(insn: *const loongarch_instruction) -> bool;
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn bug_on(condition: bool);
}

// Build-time kernel constants and symbols are supplied externally.
extern "C" {
    static LOONGARCH_INSN_SIZE: usize;
}

#[repr(C)]
pub union loongarch_instruction {
    pub word: u32,
    pub reg0i26_format: Reg0i26Format,
    pub reg1i21_format: Reg1i21Format,
    pub reg2i16_format: Reg2i16Format,
}

#[repr(C)]
pub struct Reg0i26Format { pub immediate_l: u32, pub immediate_h: u32, pub opcode: u32 }
#[repr(C)]
pub struct Reg1i21Format { pub immediate_l: u32, pub immediate_h: u32, pub opcode: u32, pub rj: u32 }
#[repr(C)]
pub struct Reg2i16Format { pub immediate: u32, pub opcode: u32 }

#[repr(C)]
pub struct alt_instr {
    pub instr_offset: isize,
    pub replace_offset: isize,
    pub feature: i32,
    pub instrlen: u16,
    pub replacementlen: u16,
}

unsafe fn add_nops(mut insn: *mut loongarch_instruction, mut count: i32) {
    while count != 0 {
        (*insn).word = INSN_NOP;
        insn = insn.add(1);
        count -= 1;
    }
}

unsafe fn recompute_jump(buf: *mut loongarch_instruction, dest: *mut loongarch_instruction,
                         src: *mut loongarch_instruction, start: *mut core::ffi::c_void,
                         end: *mut core::ffi::c_void) {
    let cur_pc = src as usize;
    let pc = dest as usize;
    let mut si_l = (*src).reg0i26_format.immediate_l;
    let mut si_h = (*src).reg0i26_format.immediate_h;
    match (*src).reg0i26_format.opcode {
        b_op | bl_op => {
            let jump_addr = (cur_pc as i64).wrapping_add(sign_extend64((((si_h << 16) | si_l) << 2) as u64, 27)) as usize;
            if in_alt_jump(jump_addr, start, end) { return; }
            let offset = jump_addr.wrapping_sub(pc) as isize;
            bug_on(offset < -SZ_128M || offset >= SZ_128M);
            let offset = offset >> 2;
            (*buf).reg0i26_format.immediate_h = (offset >> 16) as u32;
            (*buf).reg0i26_format.immediate_l = offset as u32;
            return;
        }
        _ => {}
    }
    si_l = (*src).reg1i21_format.immediate_l;
    si_h = (*src).reg1i21_format.immediate_h;
    match (*src).reg1i21_format.opcode {
        bceqz_op | beqz_op | bnez_op => {
            bug_on((*buf).reg1i21_format.rj & BIT(4) != 0);
            let jump_addr = (cur_pc as i64).wrapping_add(sign_extend64((((si_h << 16) | si_l) << 2) as u64, 22)) as usize;
            if in_alt_jump(jump_addr, start, end) { return; }
            let offset = jump_addr.wrapping_sub(pc) as isize;
            bug_on(offset < -SZ_4M || offset >= SZ_4M);
            let offset = offset >> 2;
            (*buf).reg1i21_format.immediate_h = (offset >> 16) as u32;
            (*buf).reg1i21_format.immediate_l = offset as u32;
            return;
        }
        _ => {}
    }
    let si = (*src).reg2i16_format.immediate;
    match (*src).reg2i16_format.opcode {
        beq_op | bne_op | blt_op | bge_op | bltu_op | bgeu_op => {
            let jump_addr = (cur_pc as i64).wrapping_add(sign_extend64((si << 2) as u64, 17)) as usize;
            if in_alt_jump(jump_addr, start, end) { return; }
            let offset = jump_addr.wrapping_sub(pc) as isize;
            bug_on(offset < -SZ_128K || offset >= SZ_128K);
            (*buf).reg2i16_format.immediate = (offset >> 2) as u32;
        }
        _ => {}
    }
}

unsafe fn copy_alt_insns(buf: *mut loongarch_instruction, dest: *mut loongarch_instruction,
                         src: *mut loongarch_instruction, nr: i32) -> i32 {
    for i in 0..nr {
        (*buf.add(i as usize)).word = (*src.add(i as usize)).word;
        if is_pc_ins(src.add(i as usize)) { pr_err(c"Not support pcrel instruction at present!".as_ptr()); return -22; }
        if is_branch_ins(src.add(i as usize)) && (*src.add(i as usize)).reg2i16_format.opcode != jirl_op {
            recompute_jump(buf.add(i as usize), dest.add(i as usize), src.add(i as usize), src as _, src.add(nr as usize) as _);
        }
    }
    0
}

unsafe fn text_poke_early(insn: *mut loongarch_instruction, buf: *mut loongarch_instruction, nr: usize) -> *mut loongarch_instruction {
    let mut flags = 0usize;
    local_irq_save(&mut flags);
    for i in 0..nr { (*insn.add(i)).word = (*buf.add(i)).word; }
    local_irq_restore(flags);
    wbflush();
    flush_icache_range(insn as usize, insn.add(nr) as usize);
    insn
}

pub unsafe fn apply_alternatives(start: *mut alt_instr, end: *mut alt_instr) {
    let mut a = start;
    let mut insnbuf: [loongarch_instruction; MAX_PATCH_SIZE] = core::mem::zeroed();
    while a < end {
        let instr = (&mut (*a).instr_offset as *mut isize as *mut u8).offset((*a).instr_offset) as *mut loongarch_instruction;
        let replacement = (&mut (*a).replace_offset as *mut isize as *mut u8).offset((*a).replace_offset) as *mut loongarch_instruction;
        bug_on((*a).instrlen as usize > core::mem::size_of_val(&insnbuf));
        bug_on((*a).instrlen & 0x3 != 0);
        bug_on((*a).replacementlen & 0x3 != 0);
        let nr_instr = (*a).instrlen as usize / LOONGARCH_INSN_SIZE;
        let nr_repl = (*a).replacementlen as usize / LOONGARCH_INSN_SIZE;
        if cpu_has((*a).feature) {
            copy_alt_insns(insnbuf.as_mut_ptr(), instr, replacement, nr_repl as i32);
            if nr_instr > nr_repl { add_nops(insnbuf.as_mut_ptr().add(nr_repl), (nr_instr - nr_repl) as i32); }
            text_poke_early(instr, insnbuf.as_mut_ptr(), nr_instr);
        }
        a = a.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
