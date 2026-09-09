// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance counter callchain support - powerpc architecture code
 *
 * Copyright © 2009 Paul Mackerras, IBM Corporation.
 */

// Translated dependencies supplied by the surrounding kernel build.

#[cfg(target_pointer_width = "64")]
use crate::syscalls_32::*;

#[cfg(not(target_pointer_width = "64"))]
type SignalFramesize32 = __SIGNAL_FRAMESIZE;
#[cfg(not(target_pointer_width = "64"))]
type Sigcontext32 = sigcontext;
#[cfg(not(target_pointer_width = "64"))]
type Mcontext32 = mcontext;
#[cfg(not(target_pointer_width = "64"))]
type Ucontext32 = ucontext;
#[cfg(not(target_pointer_width = "64"))]
type CompatSiginfoT = siginfo;

#[cfg(target_pointer_width = "64")]
type SignalFramesize32 = __SIGNAL_FRAMESIZE32;
#[cfg(target_pointer_width = "64")]
type Sigcontext32 = sigcontext32;
#[cfg(target_pointer_width = "64")]
type Mcontext32 = mcontext32;
#[cfg(target_pointer_width = "64")]
type Ucontext32 = ucontext32;
#[cfg(target_pointer_width = "64")]
type CompatSiginfoT = compat_siginfo_t;

#[repr(C)]
pub struct signal_frame_32 {
    pub dummy: [core::ffi::c_char; SignalFramesize32],
    pub sctx: Sigcontext32,
    pub mctx: Mcontext32,
    pub abigap: [core::ffi::c_int; 56],
}

#[repr(C)]
pub struct rt_signal_frame_32 {
    pub dummy: [core::ffi::c_char; SignalFramesize32 + 16],
    pub info: CompatSiginfoT,
    pub uc: Ucontext32,
    pub abigap: [core::ffi::c_int; 56],
}

unsafe fn read_user_stack_32(ptr: *const u32, ret: *mut u32) -> i32 {
    __read_user_stack(ptr, ret, core::mem::size_of::<u32>())
}

unsafe fn is_sigreturn_32_address(nip: u32, fp: u32) -> i32 {
    if nip == fp.wrapping_add(core::mem::offset_of!(signal_frame_32, mctx.mc_pad) as u32) {
        return 1;
    }
    if !(*current).mm.context.vdso.is_null()
        && nip == VDSO32_SYMBOL((*current).mm.context.vdso, sigtramp32)
    {
        return 1;
    }
    0
}

unsafe fn is_rt_sigreturn_32_address(nip: u32, fp: u32) -> i32 {
    if nip == fp.wrapping_add(core::mem::offset_of!(rt_signal_frame_32, uc.uc_mcontext.mc_pad) as u32) {
        return 1;
    }
    if !(*current).mm.context.vdso.is_null()
        && nip == VDSO32_SYMBOL((*current).mm.context.vdso, sigtramp_rt32)
    {
        return 1;
    }
    0
}

unsafe fn sane_signal_32_frame(sp: u32) -> i32 {
    let sf = sp as usize as *mut signal_frame_32;
    let mut regs = 0u32;
    if read_user_stack_32(core::ptr::addr_of!((*sf).sctx.regs) as *const u32, &mut regs) != 0 {
        return 0;
    }
    (regs == core::ptr::addr_of!((*sf).mctx) as usize as u32) as i32
}

unsafe fn sane_rt_signal_32_frame(sp: u32) -> i32 {
    let sf = sp as usize as *mut rt_signal_frame_32;
    let mut regs = 0u32;
    if read_user_stack_32(core::ptr::addr_of!((*sf).uc.uc_regs) as *const u32, &mut regs) != 0 {
        return 0;
    }
    (regs == core::ptr::addr_of!((*sf).uc.uc_mcontext) as usize as u32) as i32
}

unsafe fn signal_frame_32_regs(sp: u32, next_sp: u32, next_ip: u32) -> *mut u32 {
    let mut mctx: *mut Mcontext32 = core::ptr::null_mut();
    if next_sp.wrapping_sub(sp) >= core::mem::size_of::<signal_frame_32>() as u32
        && is_sigreturn_32_address(next_ip, sp) != 0
        && sane_signal_32_frame(sp) != 0
    {
        let sf = sp as usize as *mut signal_frame_32;
        mctx = core::ptr::addr_of_mut!((*sf).mctx);
    }
    if mctx.is_null()
        && next_sp.wrapping_sub(sp) >= core::mem::size_of::<rt_signal_frame_32>() as u32
        && is_rt_sigreturn_32_address(next_ip, sp) != 0
        && sane_rt_signal_32_frame(sp) != 0
    {
        let rt_sf = sp as usize as *mut rt_signal_frame_32;
        mctx = core::ptr::addr_of_mut!((*rt_sf).uc.uc_mcontext);
    }
    if mctx.is_null() { return core::ptr::null_mut(); }
    (*mctx).mc_gregs
}

pub unsafe fn perf_callchain_user_32(entry: *mut perf_callchain_entry_ctx, regs: *mut pt_regs) {
    let mut sp: u32;
    let mut next_sp: u32 = 0;
    let mut next_ip = perf_arch_instruction_pointer(regs);
    let mut lr = (*regs).link;
    let mut level: i64 = 0;
    let mut fp: *mut u32;
    let mut uregs: *mut u32;

    sp = (*regs).gpr[1];
    while (*entry).nr < (*entry).max_stack {
        fp = sp as usize as *mut u32;
        if invalid_user_sp(sp) != 0 || read_user_stack_32(fp, &mut next_sp) != 0 { return; }
        if level > 0 && read_user_stack_32(fp.add(1), &mut next_ip) != 0 { return; }
        uregs = signal_frame_32_regs(sp, next_sp, next_ip);
        if uregs.is_null() && level <= 1 { uregs = signal_frame_32_regs(sp, next_sp, lr); }
        if !uregs.is_null() {
            if read_user_stack_32(uregs.add(PT_NIP), &mut next_ip) != 0
                || read_user_stack_32(uregs.add(PT_LNK), &mut lr) != 0
                || read_user_stack_32(uregs.add(PT_R1), &mut sp) != 0 { return; }
            level = 0;
            perf_callchain_store_context(entry, PERF_CONTEXT_USER);
            perf_callchain_store(entry, next_ip);
            continue;
        }
        if level == 0 { next_ip = lr; }
        perf_callchain_store(entry, next_ip);
        level += 1;
        sp = next_sp;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
