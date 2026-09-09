// SPDX-License-Identifier: GPL-2.0
/*
 * 'traps.c' handles hardware traps and faults after we have saved some
 * state in 'entry.S'.
 *
 *  SuperH version: Copyright (C) 1999 Niibe Yutaka
 *                  Copyright (C) 2000 Philipp Rumpf
 *                  Copyright (C) 2000 David Howells
 *                  Copyright (C) 2002 - 2010 Paul Mundt
 */

#[cfg(feature = "cpu_sh2")]
const TRAP_RESERVED_INST: u32 = 4;
#[cfg(feature = "cpu_sh2")]
const TRAP_ILLEGAL_SLOT_INST: u32 = 6;
#[cfg(feature = "cpu_sh2")]
const TRAP_ADDRESS_ERROR: u32 = 9;
#[cfg(all(feature = "cpu_sh2", feature = "cpu_sh2a"))]
const TRAP_UBC: u32 = 12;
#[cfg(all(feature = "cpu_sh2", feature = "cpu_sh2a"))]
const TRAP_FPU_ERROR: u32 = 13;
#[cfg(all(feature = "cpu_sh2", feature = "cpu_sh2a"))]
const TRAP_DIVZERO_ERROR: u32 = 17;
#[cfg(all(feature = "cpu_sh2", feature = "cpu_sh2a"))]
const TRAP_DIVOVF_ERROR: u32 = 18;
#[cfg(not(feature = "cpu_sh2"))]
const TRAP_RESERVED_INST: u32 = 12;
#[cfg(not(feature = "cpu_sh2"))]
const TRAP_ILLEGAL_SLOT_INST: u32 = 13;

unsafe fn sign_extend(count: u32, dst: *mut u8) {
    #[cfg(target_endian = "little")]
    {
        if count == 1 && *dst & 0x80 != 0 { *dst.add(1) = 0xff; *dst.add(2) = 0xff; *dst.add(3) = 0xff; }
        if count == 2 && *dst.add(1) & 0x80 != 0 { *dst.add(2) = 0xff; *dst.add(3) = 0xff; }
    }
    #[cfg(target_endian = "big")]
    {
        if count == 1 && *dst.add(3) & 0x80 != 0 { *dst.add(2) = 0xff; *dst.add(1) = 0xff; *dst = 0xff; }
        if count == 2 && *dst.add(2) & 0x80 != 0 { *dst.add(1) = 0xff; *dst = 0xff; }
    }
}

extern "C" {
    fn copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn copy_from_kernel_nofault(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn copy_to_kernel_nofault(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
}

#[repr(C)]
pub struct mem_access {
    pub from: unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize) -> usize,
    pub to: unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize) -> usize,
}

unsafe extern "C" fn copy_from_kernel_wrapper(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, cnt: usize) -> usize {
    copy_from_kernel_nofault(dst, src, cnt)
}
unsafe extern "C" fn copy_to_kernel_wrapper(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, cnt: usize) -> usize {
    copy_to_kernel_nofault(dst, src, cnt)
}

static mut user_mem_access: mem_access = mem_access { from: copy_from_user, to: copy_to_user };
static mut kernel_mem_access: mem_access = mem_access { from: copy_from_kernel_wrapper, to: copy_to_kernel_wrapper };

// External kernel types and helpers are supplied by the surrounding translation unit.
#[repr(C)] pub struct pt_regs { pub regs: [usize; 16], pub pc: usize, pub pr: usize, pub sr: usize }
pub type insn_size_t = u16;
extern "C" {
    fn instruction_size(i: insn_size_t) -> usize;
    fn inc_unaligned_byte_access(); fn inc_unaligned_word_access(); fn inc_unaligned_dword_access(); fn inc_unaligned_multi_access();
    fn inc_unaligned_user_access(); fn inc_unaligned_kernel_access(); fn unaligned_user_action() -> u32;
    fn unaligned_fixups_notify(current: *mut core::ffi::c_void, i: insn_size_t, r: *mut pt_regs);
    fn die_if_no_fixup(s: *const u8, r: *mut pt_regs, e: usize); fn die(s: *const u8, r: *mut pt_regs, e: usize);
    fn user_mode(r: *mut pt_regs) -> bool; fn local_irq_enable(); fn force_sig_fault(a: i32, c: i32, p: *mut core::ffi::c_void);
    fn current_pt_regs() -> *mut pt_regs; fn lookup_exception_vector() -> usize; fn force_sig(a: i32);
    fn kprobe_handle_illslot(pc: usize) -> i32; fn clear_bl_bit();
}

unsafe fn handle_unaligned_ins(instruction: insn_size_t, regs: *mut pt_regs, ma: *mut mem_access) -> i32 {
    let index = ((instruction >> 8) & 15) as usize;
    let rn = &mut (*regs).regs[index] as *mut usize;
    let rm = &mut (*regs).regs[((instruction >> 4) & 15) as usize] as *mut usize;
    let count = 1usize << (instruction & 3);
    match count { 1 => inc_unaligned_byte_access(), 2 => inc_unaligned_word_access(), 4 => inc_unaligned_dword_access(), 8 => inc_unaligned_multi_access(), _ => {} }
    let mut ret = -14;
    match instruction >> 12 {
        0 => {
            if instruction & 8 != 0 {
                let src = ((*rm).wrapping_add((*regs).regs[0])) as *const _;
                let dst = rn as *mut u8; core::ptr::write_bytes(dst, 0, 4);
                let d = if cfg!(target_endian = "little") { dst } else { dst.add(4-count) };
                if ((*ma).from)(d as *mut _, src, count) != 0 { die_if_no_fixup(b"Fault in unaligned fixup\0".as_ptr(), regs, 0); return -14; }
                sign_extend(count as u32, d);
            } else {
                let src = if cfg!(target_endian = "little") { rm as *mut u8 } else { (rm as *mut u8).add(4-count) };
                let dst = (*rn) as *mut _;
                if ((*ma).to)(dst, src as *const _, count) != 0 { die_if_no_fixup(b"Fault in unaligned fixup\0".as_ptr(), regs, 0); return -14; }
            }
            ret = 0;
        }
        1 => { let dst = (*rn + (((instruction & 15) as usize) << 2)) as *mut _; if ((*ma).to)(dst, rm as *const _, 4) == 0 { ret = 0; } }
        2 => { if instruction & 4 != 0 { *rn = (*rn).wrapping_sub(count); } let src = if cfg!(target_endian = "little") { rm as *mut u8 } else { (rm as *mut u8).add(4-count) }; if ((*ma).to)((*rn) as *mut _, src as *const _, count) == 0 { ret = 0; } }
        5 => { let src = ((*rm + (((instruction & 15) as usize) << 2)) as *const _); let dst = rn as *mut u8; core::ptr::write_bytes(dst, 0, 4); if ((*ma).from)(dst as *mut _, src, 4) == 0 { ret = 0; } }
        6 => { let src = *rm as *const _; if instruction & 4 != 0 { *rm = (*rm).wrapping_add(count); } let dst = rn as *mut u8; core::ptr::write_bytes(dst, 0, 4); let d = if cfg!(target_endian = "little") { dst } else { dst.add(4-count) }; if ((*ma).from)(d as *mut _, src, count) == 0 { sign_extend(count as u32, d); ret = 0; } }
        8 if instruction & 0xff00 == 0x8100 => { let src = (&(*regs).regs[0] as *const usize as *const u8).add(if cfg!(target_endian = "little") { 0 } else { 2 }); if ((*ma).to)((*rm + (((instruction & 15) as usize) << 1)) as *mut _, src as *const _, 2) == 0 { ret = 0; } }
        8 if instruction & 0xff00 == 0x8500 => { let dst = &mut (*regs).regs[0] as *mut usize as *mut u8; core::ptr::write_bytes(dst, 0, 4); let d = if cfg!(target_endian = "little") { dst } else { dst.add(2) }; if ((*ma).from)(d as *mut _, (*rm + (((instruction & 15) as usize) << 1)) as *const _, 2) == 0 { sign_extend(2, d); ret = 0; } }
        9 => { let dst = rn as *mut u8; core::ptr::write_bytes(dst, 0, 4); let d = if cfg!(target_endian = "little") { dst } else { dst.add(2) }; if ((*ma).from)(d as *mut _, ((*regs).pc + 4 + (((instruction & 255) as usize) << 1)) as *const _, 2) == 0 { sign_extend(2, d); ret = 0; } }
        0xd => { let dst = rn as *mut u8; core::ptr::write_bytes(dst, 0, 4); if ((*ma).from)(dst as *mut _, (((*regs).pc & !3) + 4 + (((instruction & 255) as usize) << 2)) as *const _, 4) == 0 { ret = 0; } }
        _ => {}
    }
    ret
}

unsafe fn handle_delayslot(regs: *mut pt_regs, old_instruction: insn_size_t, ma: *mut mem_access) -> i32 {
    let mut instruction: insn_size_t = 0;
    let addr = ((*regs).pc + instruction_size(old_instruction)) as *const core::ffi::c_void;
    if copy_from_user(&mut instruction as *mut _ as *mut _, addr, core::mem::size_of::<insn_size_t>()) != 0 {
        if user_mode(regs) { return -14; }
        die(b"delay-slot-insn faulting in handle_unaligned_delayslot\0".as_ptr(), regs, 0);
    }
    handle_unaligned_ins(instruction, regs, ma)
}

pub unsafe fn handle_unaligned_access(instruction: insn_size_t, regs: *mut pt_regs, ma: *mut mem_access, expected: i32, address: usize) -> i32 {
    let rm = (*regs).regs[((instruction >> 8) & 15) as usize];
    if instruction_size(instruction) != 2 { return -22; }
    if expected == 0 { unaligned_fixups_notify(core::ptr::null_mut(), instruction, regs); }
    let mut ret = -14;
    match instruction & 0xf000 {
        0x0000 if instruction == 0x000b => { ret = handle_delayslot(regs, instruction, ma); if ret == 0 { (*regs).pc = (*regs).pr; } }
        0x0000 if instruction & 0xff == 0x23 => { ret = handle_delayslot(regs, instruction, ma); if ret == 0 { (*regs).pc = (*regs).pc.wrapping_add(rm + 4); } }
        0x0000 if instruction & 0xff == 3 => { ret = handle_delayslot(regs, instruction, ma); if ret == 0 { (*regs).pr = (*regs).pc + 4; (*regs).pc = (*regs).pc.wrapping_add(rm + 4); } }
        0x0000 | 0x1000 | 0x2000 | 0x5000 | 0x6000 | 0x9000 | 0xd000 => { ret = handle_unaligned_ins(instruction, regs, ma); if ret == 0 { (*regs).pc += instruction_size(instruction); } }
        0x4000 if instruction & 0xff == 0x2b => { ret = handle_delayslot(regs, instruction, ma); if ret == 0 { (*regs).pc = rm; } }
        0x4000 if instruction & 0xff == 0x0b => { ret = handle_delayslot(regs, instruction, ma); if ret == 0 { (*regs).pr = (*regs).pc + 4; (*regs).pc = rm; } }
        0xa000 => { ret = handle_delayslot(regs, instruction, ma); if ret == 0 { (*regs).pc = (*regs).pc.wrapping_add((((instruction << 4) as i16 >> 3) as isize + 4) as usize); } }
        0xb000 => { ret = handle_delayslot(regs, instruction, ma); if ret == 0 { (*regs).pr = (*regs).pc + 4; (*regs).pc = (*regs).pc.wrapping_add((((instruction << 4) as i16 >> 3) as isize + 4) as usize); } }
        _ => {}
    }
    ret
}

pub unsafe extern "C" fn do_address_error(regs: *mut pt_regs, _writeaccess: usize, address: usize) {
    let mut instruction: insn_size_t = 0;
    if user_mode(regs) {
        local_irq_enable(); inc_unaligned_user_access();
        if copy_from_user(&mut instruction as *mut _ as *mut _, ((*regs).pc & !1) as *const _, 2) != 0 { force_sig_fault(7, 1, address as *mut _); return; }
        let action = unaligned_user_action();
        if action & 1 != 0 { if handle_unaligned_access(instruction, regs, &mut user_mem_access, 0, address) == 0 { return; } }
        if action & 2 == 0 { (*regs).pc += instruction_size(instruction); return; }
        force_sig_fault(7, 1, address as *mut _);
    } else {
        inc_unaligned_kernel_access();
        if copy_from_kernel_nofault(&mut instruction as *mut _ as *mut _, (*regs).pc as *const _, 2) != 0 { die(b"insn faulting in do_address_error\0".as_ptr(), regs, 0); }
        handle_unaligned_access(instruction, regs, &mut kernel_mem_access, 0, address);
    }
}

pub unsafe extern "C" fn do_reserved_inst() {
    let regs = current_pt_regs(); let error_code = lookup_exception_vector(); local_irq_enable(); force_sig(4); die_if_no_fixup(b"reserved instruction\0".as_ptr(), regs, error_code);
}
pub unsafe extern "C" fn do_illegal_slot_inst() {
    let regs = current_pt_regs(); if kprobe_handle_illslot((*regs).pc) == 0 { return; }
    let inst = lookup_exception_vector(); local_irq_enable(); force_sig(4); die_if_no_fixup(b"illegal slot instruction\0".as_ptr(), regs, inst);
}
pub unsafe extern "C" fn do_exception_error() { let ex = lookup_exception_vector(); die_if_no_fixup(b"exception\0".as_ptr(), current_pt_regs(), ex); }

extern "C" { static mut vbr_base: *mut core::ffi::c_void; }
pub unsafe fn per_cpu_trap_init() { core::arch::asm!("ldc {0}, vbr", in(reg) &vbr_base, options(nostack)); clear_bl_bit(); }
extern "C" { static mut exception_handling_table: [*mut core::ffi::c_void; 256]; }
pub unsafe fn set_exception_table_vec(vec: u32, handler: *mut core::ffi::c_void) -> *mut core::ffi::c_void { let old = exception_handling_table[vec as usize]; exception_handling_table[vec as usize] = handler; old }
pub unsafe fn trap_init() { set_exception_table_vec(TRAP_RESERVED_INST, do_reserved_inst as *mut _); set_exception_table_vec(TRAP_ILLEGAL_SLOT_INST, do_illegal_slot_inst as *mut _); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
