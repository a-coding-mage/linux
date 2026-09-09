// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Dynamic Ftrace based Kprobes Optimization
 *
 * Copyright (C) Hitachi Ltd., 2012
 * Copyright 2016 Naveen N. Rao <naveen.n.rao@linux.vnet.ibm.com>
 *		  IBM Corporation
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kprobes.h, linux/ptrace.h, linux/hardirq.h, linux/preempt.h,
// and linux/ftrace.h.

use core::ffi::c_int;

type KprobeOpcode = u32;

#[repr(C)]
pub struct FtraceOps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FtraceRegs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PtRegs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct KprobeArchInsn {
    pub insn: *mut KprobeOpcode,
    pub boostable: i32,
}

#[repr(C)]
pub struct Kprobe {
    pub ainsn: KprobeArchInsn,
    pub pre_handler: Option<unsafe extern "C" fn(*mut Kprobe, *mut PtRegs) -> c_int>,
    pub post_handler: Option<unsafe extern "C" fn(*mut Kprobe, *mut PtRegs, c_int)>,
}

#[repr(C)]
pub struct KprobeCtlblk {
    pub kprobe_status: c_int,
}

extern "C" {
    static mut kprobe_ftrace_disabled: c_int;
    static mut current_kprobe: *mut Kprobe;

    fn ftrace_test_recursion_trylock(nip: usize, parent_nip: usize) -> c_int;
    fn ftrace_get_regs(fregs: *mut FtraceRegs) -> *mut PtRegs;
    fn get_kprobe(addr: *mut KprobeOpcode) -> *mut Kprobe;
    fn kprobe_disabled(p: *mut Kprobe) -> bool;
    fn get_kprobe_ctlblk() -> *mut KprobeCtlblk;
    fn kprobe_running() -> bool;
    fn kprobes_inc_nmissed_count(p: *mut Kprobe);
    fn regs_add_return_ip(regs: *mut PtRegs, offset: isize);
    fn ftrace_test_recursion_unlock(bit: c_int);
}

const MCOUNT_INSN_SIZE: isize = 4;
const KPROBE_HIT_ACTIVE: c_int = 1;
const KPROBE_HIT_SSDONE: c_int = 2;

/* Ftrace callback handler for kprobes */
pub unsafe extern "C" fn kprobe_ftrace_handler(
    nip: usize,
    parent_nip: usize,
    _ops: *mut FtraceOps,
    fregs: *mut FtraceRegs,
) {
    let p: *mut Kprobe;
    let kcb: *mut KprobeCtlblk;
    let regs: *mut PtRegs;
    let bit: c_int;

    if kprobe_ftrace_disabled != 0 {
        return;
    }

    bit = ftrace_test_recursion_trylock(nip, parent_nip);
    if bit < 0 {
        return;
    }

    regs = ftrace_get_regs(fregs);
    p = get_kprobe(nip as *mut KprobeOpcode);
    if p.is_null() || kprobe_disabled(p) {
        ftrace_test_recursion_unlock(bit);
        return;
    }

    kcb = get_kprobe_ctlblk();
    if kprobe_running() {
        kprobes_inc_nmissed_count(p);
    } else {
        /*
         * On powerpc, NIP is *before* this instruction for the
         * pre handler
         */
        regs_add_return_ip(regs, -MCOUNT_INSN_SIZE);

        current_kprobe = p;
        (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
        let pre_result = match (*p).pre_handler {
            Some(handler) => handler(p, regs),
            None => 0,
        };
        if pre_result == 0 {
            /*
             * Emulate singlestep (and also recover regs->nip)
             * as if there is a nop
             */
            regs_add_return_ip(regs, MCOUNT_INSN_SIZE);
            if let Some(handler) = (*p).post_handler {
                (*kcb).kprobe_status = KPROBE_HIT_SSDONE;
                handler(p, regs, 0);
            }
        }
        /*
         * If pre_handler returns !0, it changes regs->nip. We have to
         * skip emulating post_handler.
         */
        current_kprobe = core::ptr::null_mut();
    }

    ftrace_test_recursion_unlock(bit);
}

pub unsafe extern "C" fn arch_prepare_kprobe_ftrace(p: *mut Kprobe) -> c_int {
    (*p).ainsn.insn = core::ptr::null_mut();
    (*p).ainsn.boostable = -1;
    0
}

// NOKPROBE_SYMBOL(kprobe_ftrace_handler);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
