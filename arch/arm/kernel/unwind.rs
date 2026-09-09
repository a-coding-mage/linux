// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/kernel/unwind.c
 *
 * Copyright (C) 2008 ARM Limited
 *
 * Stack unwinding support for ARM
 */

// Kernel and architecture dependencies are supplied by the surrounding tree.

extern "C" {
    static __start_unwind_idx: unwind_idx;
    static __stop_unwind_idx: unwind_idx;
}

#[repr(C)]
struct unwind_ctrl_block {
    vrs: [c_ulong; 16],
    insn: *const c_ulong,
    sp_high: c_ulong,
    lr_addr: *mut c_ulong,
    check_each_pop: c_int,
    entries: c_int,
    byte: c_int,
}

#[repr(C)]
struct unwind_idx {
    addr_offset: c_ulong,
    insn: c_ulong,
}

#[repr(C)]
struct stackframe {
    fp: c_ulong,
    sp: c_ulong,
    lr: c_ulong,
    pc: c_ulong,
    lr_addr: *mut c_ulong,
}

type c_ulong = usize;
type c_int = i32;

const URC_OK: c_int = 0;
const URC_FAILURE: c_int = 9;
const FP: usize = 11;
const SP: usize = 13;
const LR: usize = 14;
const PC: usize = 15;

static mut __origin_unwind_idx: *const unwind_idx = core::ptr::null();

extern "C" {
    fn core_kernel_text(addr: c_ulong) -> c_int;
    fn kernel_text_address(addr: c_ulong) -> c_int;
    fn in_module_plt(addr: c_ulong) -> c_int;
    fn call_with_stack();
    fn thread_size() -> c_ulong;
    fn thread_align() -> c_ulong;
}

#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr0() {}

#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr1() {}

#[no_mangle]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr2() {}

#[inline]
unsafe fn prel31_to_addr(ptr: *const c_ulong) -> c_ulong {
    let offset = ((*ptr as u32) << 1) as i32 >> 1;
    (ptr as c_ulong).wrapping_add(offset as c_ulong)
}

unsafe fn search_index(mut addr: c_ulong, mut start: *const unwind_idx,
                       origin: *const unwind_idx, mut stop: *const unwind_idx)
                       -> *const unwind_idx {
    if addr < start as c_ulong { stop = origin; } else { start = origin; }
    let mut addr_prel31 = addr.wrapping_sub(start as c_ulong) & 0x7fffffff;
    while start < stop.sub(1) {
        let mid = start.add(stop.offset_from(start) as usize >> 1);
        if addr_prel31.wrapping_sub(mid as c_ulong - start as c_ulong) < (*mid).addr_offset {
            stop = mid;
        } else {
            addr_prel31 = addr_prel31.wrapping_sub(mid as c_ulong - start as c_ulong);
            start = mid;
        }
    }
    if (*start).addr_offset <= addr_prel31 { start } else { core::ptr::null() }
}

unsafe fn unwind_find_origin(mut start: *const unwind_idx, mut stop: *const unwind_idx) -> *const unwind_idx {
    while start < stop {
        let mid = start.add(stop.offset_from(start) as usize >> 1);
        if (*mid).addr_offset >= 0x40000000 { start = mid.add(1); } else { stop = mid; }
    }
    stop
}

unsafe fn unwind_find_idx(addr: c_ulong) -> *const unwind_idx {
    if core_kernel_text(addr) != 0 {
        if __origin_unwind_idx.is_null() {
            __origin_unwind_idx = unwind_find_origin(&__start_unwind_idx, &__stop_unwind_idx);
        }
        search_index(addr, &__start_unwind_idx, __origin_unwind_idx, &__stop_unwind_idx)
    } else { core::ptr::null() }
}

unsafe fn unwind_get_byte(ctrl: &mut unwind_ctrl_block) -> c_ulong {
    if ctrl.entries <= 0 { return 0; }
    let ret = (*ctrl.insn >> (ctrl.byte * 8)) & 0xff;
    if ctrl.byte == 0 { ctrl.insn = ctrl.insn.add(1); ctrl.entries -= 1; ctrl.byte = 3; }
    else { ctrl.byte -= 1; }
    ret
}

unsafe fn unwind_pop_register(ctrl: &mut unwind_ctrl_block, vsp: &mut *mut c_ulong, reg: usize) -> c_int {
    if ctrl.check_each_pop != 0 && *vsp as c_ulong >= ctrl.sp_high { return -URC_FAILURE; }
    ctrl.vrs[reg] = **vsp;
    if reg == 14 { ctrl.lr_addr = *vsp; }
    *vsp = (*vsp).add(1);
    URC_OK
}

unsafe fn unwind_exec_pop_subset_r4_to_r13(ctrl: &mut unwind_ctrl_block, mut mask: c_ulong) -> c_int {
    let mut vsp = ctrl.vrs[SP] as *mut c_ulong;
    let load_sp = mask & (1 << 9);
    let mut reg = 4;
    while mask != 0 { if mask & 1 != 0 && unwind_pop_register(ctrl, &mut vsp, reg) != 0 { return -URC_FAILURE; } mask >>= 1; reg += 1; }
    if load_sp == 0 { ctrl.vrs[SP] = vsp as c_ulong; }
    URC_OK
}

unsafe fn unwind_exec_pop_r4_to_rn(ctrl: &mut unwind_ctrl_block, insn: c_ulong) -> c_int {
    let mut vsp = ctrl.vrs[SP] as *mut c_ulong;
    for reg in 4..=4 + (insn & 7) as usize { if unwind_pop_register(ctrl, &mut vsp, reg) != 0 { return -URC_FAILURE; } }
    if insn & 8 != 0 && unwind_pop_register(ctrl, &mut vsp, 14) != 0 { return -URC_FAILURE; }
    ctrl.vrs[SP] = vsp as c_ulong; URC_OK
}

unsafe fn unwind_exec_pop_subset_r0_to_r3(ctrl: &mut unwind_ctrl_block, mut mask: c_ulong) -> c_int {
    let mut vsp = ctrl.vrs[SP] as *mut c_ulong; let mut reg = 0;
    while mask != 0 { if mask & 1 != 0 && unwind_pop_register(ctrl, &mut vsp, reg) != 0 { return -URC_FAILURE; } mask >>= 1; reg += 1; }
    ctrl.vrs[SP] = vsp as c_ulong; URC_OK
}

unsafe fn unwind_decode_uleb128(ctrl: &mut unwind_ctrl_block) -> c_ulong {
    let mut bytes = 0; let mut result = 0; let mut insn;
    loop { insn = unwind_get_byte(ctrl); result |= (insn & 0x7f) << (bytes * 7); bytes += 1; if insn & 0x80 == 0 || bytes == core::mem::size_of::<c_ulong>() { break; } }
    result
}

unsafe fn unwind_exec_insn(ctrl: &mut unwind_ctrl_block) -> c_int {
    let mut insn = unwind_get_byte(ctrl); let mut ret = URC_OK;
    if insn & 0xc0 == 0 { ctrl.vrs[SP] += (insn & 0x3f) * 4 + 4; }
    else if insn & 0xc0 == 0x40 { ctrl.vrs[SP] -= (insn & 0x3f) * 4 + 4; }
    else if insn & 0xf0 == 0x80 { insn = (insn << 8) | unwind_get_byte(ctrl); let mask = insn & 0xfff; if mask == 0 { return -URC_FAILURE; } ret = unwind_exec_pop_subset_r4_to_r13(ctrl, mask); }
    else if insn & 0xf0 == 0x90 && insn & 0xd != 0xd { ctrl.vrs[SP] = ctrl.vrs[(insn & 0xf) as usize]; }
    else if insn & 0xf0 == 0xa0 { ret = unwind_exec_pop_r4_to_rn(ctrl, insn); }
    else if insn == 0xb0 { if ctrl.vrs[PC] == 0 { ctrl.vrs[PC] = ctrl.vrs[LR]; } ctrl.entries = 0; }
    else if insn == 0xb1 { let mask = unwind_get_byte(ctrl); if mask == 0 || mask & 0xf0 != 0 { return -URC_FAILURE; } ret = unwind_exec_pop_subset_r0_to_r3(ctrl, mask); }
    else if insn == 0xb2 { ctrl.vrs[SP] += 0x204 + (unwind_decode_uleb128(ctrl) << 2); }
    else { return -URC_FAILURE; }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn unwind_frame(frame: *mut stackframe) -> c_int {
    let idx = unwind_find_idx((*frame).pc); if idx.is_null() { if (*frame).pc != 0 && kernel_text_address((*frame).pc) != 0 && in_module_plt((*frame).pc) != 0 && (*frame).pc != (*frame).lr { (*frame).pc = (*frame).lr; return URC_OK; } return -URC_FAILURE; }
    let mut ctrl = unwind_ctrl_block { vrs: [0; 16], insn: core::ptr::null(), sp_high: (*frame).sp, lr_addr: core::ptr::null_mut(), check_each_pop: 0, entries: 0, byte: 0 };
    ctrl.vrs[FP] = (*frame).fp; ctrl.vrs[SP] = (*frame).sp; ctrl.vrs[LR] = (*frame).lr;
    if (*idx).insn == 1 { return -URC_FAILURE; }
    ctrl.insn = if (*idx).insn & 0x80000000 == 0 { prel31_to_addr(&(*idx).insn) as *const c_ulong } else if (*idx).insn & 0xff000000 == 0x80000000 { &(*idx).insn } else { return -URC_FAILURE; };
    ctrl.byte = if *ctrl.insn & 0xff000000 == 0x80000000 { 2 } else { 1 }; ctrl.entries = if ctrl.byte == 2 { 1 } else { 1 + ((*ctrl.insn & 0xff0000) >> 16) as c_int };
    while ctrl.entries > 0 { if unwind_exec_insn(&mut ctrl) < 0 { return -URC_FAILURE; } }
    if ctrl.vrs[PC] == 0 { ctrl.vrs[PC] = ctrl.vrs[LR]; }
    if (*frame).pc == ctrl.vrs[PC] && (*frame).sp == ctrl.vrs[SP] { return -URC_FAILURE; }
    (*frame).fp = ctrl.vrs[FP]; (*frame).sp = ctrl.vrs[SP]; (*frame).lr = ctrl.vrs[LR]; (*frame).pc = ctrl.vrs[PC]; (*frame).lr_addr = ctrl.lr_addr; URC_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
