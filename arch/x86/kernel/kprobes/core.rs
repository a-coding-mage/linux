// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Kernel Probes (KProbes), translated literally from the x86 implementation.
 * Linux headers and symbols referenced below are supplied by the surrounding
 * kernel translation unit.
 */

use core::mem::offset_of;

extern "C" {
    static mut twobyte_is_boostable: [u32; 8];
    static mut kretprobe_blacklist: [kretprobe_blackpoint; 2];
    static kretprobe_blacklist_size: i32;
}

#[repr(C)]
pub struct __arch_relative_insn { pub op: u8, pub raddr: i32 }

#[repr(C)]
pub struct kretprobe_blackpoint { pub name: *const i8, pub addr: *const core::ffi::c_void }

unsafe extern "C" {
    fn text_poke(addr: *mut u8, opcode: *const u8, len: usize);
    fn smp_text_poke_sync_each_cpu();
    fn perf_event_text_poke(addr: *mut u8, old: *const u8, oldlen: usize, new: *const u8, newlen: usize);
    fn insn_decode_kernel(insn: *mut insn, kaddr: *const core::ffi::c_void) -> i32;
    fn recover_probed_instruction(buf: *mut u8, addr: usize) -> usize;
    fn get_kprobe(addr: *mut core::ffi::c_void) -> *mut kprobe;
    fn get_kprobe_ctlblk() -> *mut kprobe_ctlblk;
    fn kprobe_running() -> *mut kprobe;
    fn reset_current_kprobe();
    fn setup_detour_execution(p: *mut kprobe, regs: *mut pt_regs, reenter: i32) -> bool;
    fn int3_emulate_ret(regs: *mut pt_regs);
    fn int3_emulate_call(regs: *mut pt_regs, func: usize);
    fn int3_emulate_jmp(regs: *mut pt_regs, ip: usize);
    fn int3_emulate_jcc(regs: *mut pt_regs, ty: u8, ip: usize, rel: i32);
    fn int3_emulate_push(regs: *mut pt_regs, value: usize);
    fn int3_emulate_pop(regs: *mut pt_regs) -> usize;
}

#[inline(never)]
pub unsafe fn __synthesize_relative_insn(dest: *mut u8, from: *mut u8, to: *mut u8, op: u8) {
    let insn = dest as *mut __arch_relative_insn;
    (*insn).raddr = (to as isize - (from as isize + 5)) as i32;
    (*insn).op = op;
}

pub unsafe fn synthesize_reljump(dest: *mut u8, from: *mut u8, to: *mut u8) {
    __synthesize_relative_insn(dest, from, to, JMP32_INSN_OPCODE as u8);
}
pub unsafe fn synthesize_relcall(dest: *mut u8, from: *mut u8, to: *mut u8) {
    __synthesize_relative_insn(dest, from, to, CALL_INSN_OPCODE as u8);
}

pub unsafe fn can_boost(insn: *mut insn, addr: *mut core::ffi::c_void) -> bool {
    if search_exception_tables(addr as usize) != 0 { return false; }
    if (*insn).opcode.nbytes == 2 {
        let b = (*insn).opcode.bytes[1] as usize;
        return ((*twobyte_is_boostable[b / 32] >> (b % 32)) & 1) != 0;
    }
    if (*insn).opcode.nbytes != 1 { return false; }
    let opcode = (*insn).opcode.bytes[0];
    match opcode {
        0x62 | 0x70..=0x7f | 0x9a | 0xcc..=0xce | 0xd6 | 0xd8..=0xdf |
        0xe0..=0xe3 | 0xe8..=0xe9 | 0xeb | 0xf0..=0xf4 => false,
        0xc0..=0xc1 | 0xd0..=0xd3 => X86_MODRM_REG((*insn).modrm.bytes[0]) != 0b110,
        0xf6..=0xf7 => X86_MODRM_REG((*insn).modrm.bytes[0]) != 0b001,
        0xfe => { let r = X86_MODRM_REG((*insn).modrm.bytes[0]); r == 0 || r == 1 },
        0xff => { let r = X86_MODRM_REG((*insn).modrm.bytes[0]); r == 0 || r == 1 || r == 4 },
        _ => true,
    }
}

pub unsafe fn arch_adjust_kprobe_addr(addr: usize, mut offset: usize, entry: *mut bool) -> *mut u8 {
    if is_endbr(addr as *mut u32) { *entry = offset == 0 || offset == 4; if *entry { offset = 4; } }
    else { *entry = offset == 0; }
    (addr + offset) as *mut u8
}

pub unsafe fn arch_prepare_kprobe(p: *mut kprobe) -> i32 {
    if !can_probe((*p).addr as usize) { return -EILSEQ; }
    core::ptr::write_bytes(&mut (*p).ainsn, 0, 1);
    (*p).ainsn.insn = get_insn_slot();
    if (*p).ainsn.insn.is_null() { return -ENOMEM; }
    let ret = arch_copy_kprobe(p);
    if ret != 0 { free_insn_slot((*p).ainsn.insn, 0); (*p).ainsn.insn = core::ptr::null_mut(); }
    ret
}

pub unsafe fn arch_arm_kprobe(p: *mut kprobe) { let int3 = INT3_INSN_OPCODE as u8; text_poke((*p).addr, &int3, 1); smp_text_poke_sync_each_cpu(); perf_event_text_poke((*p).addr, &(*p).opcode, 1, &int3, 1); }
pub unsafe fn arch_disarm_kprobe(p: *mut kprobe) { let int3 = INT3_INSN_OPCODE as u8; perf_event_text_poke((*p).addr, &int3, 1, &(*p).opcode, 1); text_poke((*p).addr, &(*p).opcode, 1); smp_text_poke_sync_each_cpu(); }
pub unsafe fn arch_remove_kprobe(p: *mut kprobe) { if !(*p).ainsn.insn.is_null() { perf_event_text_poke((*p).ainsn.insn, (*p).ainsn.insn, (*p).ainsn.tp_len as usize, core::ptr::null(), 0); free_insn_slot((*p).ainsn.insn, (*p).ainsn.boostable); (*p).ainsn.insn = core::ptr::null_mut(); } }

pub unsafe fn arch_init_kprobes() -> i32 { 0 }
pub unsafe fn arch_trampoline_kprobe(_p: *mut kprobe) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
