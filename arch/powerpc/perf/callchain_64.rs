// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Performance counter callchain support - powerpc architecture code
 *
 * Copyright © 2009 Paul Mackerras, IBM Corporation.
 */
// Dependencies supplied by the Linux kernel and callchain.h remain external.

use core::ffi::c_void;

extern "C" {
    fn __read_user_stack(ptr: *const c_ulong, ret: *mut c_ulong, size: usize) -> c_int;
    fn perf_arch_instruction_pointer(regs: *const PtRegs) -> c_ulong;
    fn invalid_user_sp(sp: c_ulong) -> c_int;
    fn perf_callchain_store_context(entry: *mut PerfCallchainEntryCtx, context: c_ulong);
    fn perf_callchain_store(entry: *mut PerfCallchainEntryCtx, ip: c_ulong);
    static mut current: *mut TaskStruct;
}

type c_int = i32;
type c_ulong = usize;

// External kernel types and constants.
#[repr(C)]
pub struct PerfCallchainEntryCtx {
    pub nr: c_ulong,
    pub max_stack: c_ulong,
}
#[repr(C)]
pub struct PtRegs {
    pub gpr: [c_ulong; 32],
    pub link: c_ulong,
}
#[repr(C)]
pub struct TaskStruct {
    pub mm: *mut MmStruct,
}
#[repr(C)]
pub struct MmStruct {
    pub context: MmContext,
}
#[repr(C)]
pub struct MmContext {
    pub vdso: *mut c_void,
}
#[repr(C)]
pub struct UContext {
    pub uc_mcontext: MContext,
}
#[repr(C)]
pub struct MContext {
    pub gp_regs: *mut c_ulong,
}
#[repr(C)]
pub struct SigInfo {
    _opaque: [u8; 0],
}

extern "C" {
    fn VDSO64_SYMBOL(vdso: *mut c_void, symbol: *const c_void) -> c_ulong;
    static sigtramp_rt64: c_void;
}

const PERF_CONTEXT_USER: c_ulong = 0;
const PT_NIP: usize = 0;
const PT_LNK: usize = 0;
const PT_R1: usize = 1;

fn read_user_stack_64(ptr: *const c_ulong, ret: *mut c_ulong) -> c_int {
    unsafe { __read_user_stack(ptr, ret, core::mem::size_of::<c_ulong>()) }
}

/*
 * 64-bit user processes use the same stack frame for RT and non-RT signals.
 */
#[repr(C)]
pub struct SignalFrame64 {
    pub dummy: [u8; __SIGNAL_FRAMESIZE],
    pub uc: UContext,
    pub unused: [c_ulong; 2],
    pub tramp: [u32; 6],
    pub pinfo: *mut SigInfo,
    pub puc: *mut c_void,
    pub info: SigInfo,
    pub abigap: [u8; 288],
}

const __SIGNAL_FRAMESIZE: usize = 0;

unsafe fn is_sigreturn_64_address(nip: c_ulong, fp: c_ulong) -> c_int {
    if nip == fp + core::mem::offset_of!(SignalFrame64, tramp) {
        return 1;
    }
    let mm = (*current).mm;
    if !(*mm).context.vdso.is_null()
        && nip == VDSO64_SYMBOL((*mm).context.vdso, &sigtramp_rt64)
    {
        return 1;
    }
    0
}

/*
 * Do some sanity checking on the signal frame pointed to by sp.
 * We check the pinfo and puc pointers in the frame.
 */
unsafe fn sane_signal_64_frame(sp: c_ulong) -> c_int {
    let sf = sp as *mut SignalFrame64;
    let mut pinfo = 0usize;
    let mut puc = 0usize;
    if read_user_stack_64(core::ptr::addr_of!((*sf).pinfo) as *const c_ulong, &mut pinfo)
        != 0
        || read_user_stack_64(core::ptr::addr_of!((*sf).puc) as *const c_ulong, &mut puc) != 0
    {
        return 0;
    }
    (pinfo == core::ptr::addr_of!((*sf).info) as c_ulong
        && puc == core::ptr::addr_of!((*sf).uc) as c_ulong) as c_int
}

pub unsafe fn perf_callchain_user_64(
    entry: *mut PerfCallchainEntryCtx,
    regs: *mut PtRegs,
) {
    let mut sp: c_ulong;
    let mut next_sp = 0usize;
    let mut next_ip: c_ulong = perf_arch_instruction_pointer(regs);
    let mut lr = (*regs).link;
    let mut sp = (*regs).gpr[1];
    let mut level: isize = 0;

    while (*entry).nr < (*entry).max_stack {
        let fp = sp as *mut c_ulong;
        if invalid_user_sp(sp) != 0 || read_user_stack_64(fp, &mut next_sp) != 0 {
            return;
        }
        if level > 0 && read_user_stack_64(fp.add(2), &mut next_ip) != 0 {
            return;
        }

        /* The next_sp - sp >= signal frame size check is also true when next_sp < sp. */
        if next_sp.wrapping_sub(sp) >= core::mem::size_of::<SignalFrame64>()
            && (is_sigreturn_64_address(next_ip, sp) != 0
                || (level <= 1 && is_sigreturn_64_address(lr, sp) != 0))
            && sane_signal_64_frame(sp) != 0
        {
            let sigframe = sp as *mut SignalFrame64;
            let uregs = (*sigframe).uc.uc_mcontext.gp_regs;
            if read_user_stack_64(uregs.add(PT_NIP), &mut next_ip) != 0
                || read_user_stack_64(uregs.add(PT_LNK), &mut lr) != 0
                || read_user_stack_64(uregs.add(PT_R1), &mut sp) != 0
            {
                return;
            }
            level = 0;
            perf_callchain_store_context(entry, PERF_CONTEXT_USER);
            perf_callchain_store(entry, next_ip);
            continue;
        }

        if level == 0 {
            next_ip = lr;
        }
        perf_callchain_store(entry, next_ip);
        level += 1;
        sp = next_sp;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
