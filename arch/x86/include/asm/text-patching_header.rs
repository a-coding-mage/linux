/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. Required kernel types, macros, and symbols are
// supplied by the surrounding translation unit.

pub const TEXT_POKE_MAX_OPCODE_SIZE: usize = 5;

extern "C" {
    pub fn text_poke_early(addr: *mut core::ffi::c_void, opcode: *const core::ffi::c_void, len: usize);
    pub fn text_poke_apply_relocation(
        buf: *mut u8,
        instr: *const u8,
        instrlen: usize,
        repl: *mut u8,
        repl_len: usize,
    );
    pub fn text_poke(addr: *mut core::ffi::c_void, opcode: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
    pub fn smp_text_poke_sync_each_cpu();
    pub fn text_poke_kgdb(addr: *mut core::ffi::c_void, opcode: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
    pub fn text_poke_copy(addr: *mut core::ffi::c_void, opcode: *const core::ffi::c_void, len: usize) -> *mut core::ffi::c_void;
    pub fn text_poke_copy_locked(addr: *mut core::ffi::c_void, opcode: *const core::ffi::c_void, len: usize, core_ok: bool) -> *mut core::ffi::c_void;
    pub fn text_poke_set(addr: *mut core::ffi::c_void, c: i32, len: usize) -> *mut core::ffi::c_void;
    pub fn smp_text_poke_int3_handler(regs: *mut pt_regs) -> i32;
    pub fn smp_text_poke_single(addr: *mut core::ffi::c_void, opcode: *const core::ffi::c_void, len: usize, emulate: *const core::ffi::c_void);
    pub fn smp_text_poke_batch_add(addr: *mut core::ffi::c_void, opcode: *const core::ffi::c_void, len: usize, emulate: *const core::ffi::c_void);
    pub fn smp_text_poke_batch_finish();
}

pub const INT3_INSN_SIZE: usize = 1;
pub const INT3_INSN_OPCODE: u8 = 0xCC;
pub const RET_INSN_SIZE: usize = 1;
pub const RET_INSN_OPCODE: u8 = 0xC3;
pub const CALL_INSN_SIZE: usize = 5;
pub const CALL_INSN_OPCODE: u8 = 0xE8;
pub const JMP32_INSN_SIZE: usize = 5;
pub const JMP32_INSN_OPCODE: u8 = 0xE9;
pub const JMP8_INSN_SIZE: usize = 2;
pub const JMP8_INSN_OPCODE: u8 = 0xEB;
pub const DISP32_SIZE: usize = 4;

pub unsafe fn text_opcode_size(opcode: u8) -> i32 {
    match opcode {
        INT3_INSN_OPCODE => INT3_INSN_SIZE as i32,
        RET_INSN_OPCODE => RET_INSN_SIZE as i32,
        CALL_INSN_OPCODE => CALL_INSN_SIZE as i32,
        JMP32_INSN_OPCODE => JMP32_INSN_SIZE as i32,
        JMP8_INSN_OPCODE => JMP8_INSN_SIZE as i32,
        _ => 0,
    }
}

#[repr(C)]
pub union text_poke_insn {
    pub text: [u8; TEXT_POKE_MAX_OPCODE_SIZE],
    pub fields: text_poke_insn_fields,
}

#[repr(C, packed)]
pub struct text_poke_insn_fields {
    pub opcode: u8,
    pub disp: i32,
}

pub unsafe fn __text_gen_insn(buf: *mut core::ffi::c_void, opcode: u8, addr: *const core::ffi::c_void, dest: *const core::ffi::c_void, size: i32) {
    let insn = buf as *mut text_poke_insn;
    // BUG_ON(size < text_opcode_size(opcode));
    (*insn).fields.opcode = opcode;
    if size > 1 {
        (*insn).fields.disp = (dest as isize).wrapping_sub((addr as isize).wrapping_add(size as isize)) as i32;
        if size == 2 {
            // BUG_ON((insn->disp >> 31) != (insn->disp >> 7));
        }
    }
}

pub unsafe fn text_gen_insn(opcode: u8, addr: *const core::ffi::c_void, dest: *const core::ffi::c_void) -> *mut u8 {
    static mut INSN: text_poke_insn = text_poke_insn { text: [0; TEXT_POKE_MAX_OPCODE_SIZE] };
    __text_gen_insn(&mut INSN as *mut _ as *mut core::ffi::c_void, opcode, addr, dest, text_opcode_size(opcode));
    INSN.text.as_mut_ptr()
}

extern "C" {
    pub static mut after_bootmem: i32;
    pub static mut text_poke_mm: *mut mm_struct;
    pub static mut text_poke_mm_addr: usize;
}

#[cfg(not(CONFIG_UML_X86))]
pub unsafe fn int3_emulate_jmp(regs: *mut pt_regs, ip: usize) { (*regs).ip = ip; }

#[cfg(not(CONFIG_UML_X86))]
pub unsafe fn int3_emulate_push(regs: *mut pt_regs, val: usize) {
    (*regs).sp -= core::mem::size_of::<usize>();
    *(*regs).sp as *mut usize = val;
}

#[cfg(not(CONFIG_UML_X86))]
pub unsafe fn int3_emulate_pop(regs: *mut pt_regs) -> usize {
    let val = *((*regs).sp as *const usize);
    (*regs).sp += core::mem::size_of::<usize>();
    val
}

#[cfg(not(CONFIG_UML_X86))]
pub unsafe fn int3_emulate_call(regs: *mut pt_regs, func: usize) {
    int3_emulate_push(regs, (*regs).ip - INT3_INSN_SIZE + CALL_INSN_SIZE);
    int3_emulate_jmp(regs, func);
}

#[cfg(not(CONFIG_UML_X86))]
pub unsafe fn int3_emulate_ret(regs: *mut pt_regs) { int3_emulate_jmp(regs, int3_emulate_pop(regs)); }

#[cfg(not(CONFIG_UML_X86))]
pub unsafe fn __emulate_cc(flags: usize, cc: u8) -> bool {
    let mask = [X86_EFLAGS_OF, X86_EFLAGS_CF, X86_EFLAGS_ZF, X86_EFLAGS_CF | X86_EFLAGS_ZF, X86_EFLAGS_SF, X86_EFLAGS_PF];
    let invert = cc & 1 != 0;
    let mut matched;
    if cc < 0xc { matched = flags & mask[(cc >> 1) as usize] != 0; }
    else {
        matched = ((flags & X86_EFLAGS_SF) >> X86_EFLAGS_SF_BIT) ^ ((flags & X86_EFLAGS_OF) >> X86_EFLAGS_OF_BIT) != 0;
        if cc >= 0xe { matched = matched || (flags & X86_EFLAGS_ZF != 0); }
    }
    (matched && !invert) || (!matched && invert)
}

#[cfg(not(CONFIG_UML_X86))]
pub unsafe fn int3_emulate_jcc(regs: *mut pt_regs, cc: u8, mut ip: usize, disp: usize) {
    if __emulate_cc((*regs).flags, cc) { ip = ip.wrapping_add(disp); }
    int3_emulate_jmp(regs, ip);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
