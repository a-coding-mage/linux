// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Dynamic Ftrace based Kprobes Optimization
 *
 * Copyright (C) Hitachi Ltd., 2012
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kprobes.h, linux/ptrace.h, linux/hardirq.h, linux/preempt.h,
// linux/ftrace.h, asm/text-patching.h, and common.h.

/* Ftrace callback handler for kprobes -- called under preempt disabled */
pub unsafe fn kprobe_ftrace_handler(
    ip: ::core::ffi::c_ulong,
    parent_ip: ::core::ffi::c_ulong,
    ops: *mut ftrace_ops,
    fregs: *mut ftrace_regs,
) {
    let regs: *mut pt_regs = ftrace_get_regs(fregs);
    let mut p: *mut kprobe;
    let kcb: *mut kprobe_ctlblk;
    let bit: i32;

    let _ = ops;

    if unlikely(kprobe_ftrace_disabled) {
        return;
    }

    bit = ftrace_test_recursion_trylock(ip, parent_ip);
    if bit < 0 {
        return;
    }

    p = get_kprobe(ip as *mut kprobe_opcode_t);
    if unlikely(p.is_null()) || kprobe_disabled(p) {
        ftrace_test_recursion_unlock(bit);
        return;
    }

    kcb = get_kprobe_ctlblk();
    if kprobe_running() {
        kprobes_inc_nmissed_count(p);
    } else {
        let orig_ip: ::core::ffi::c_ulong = instruction_pointer(regs);

        /* Kprobe handler expects regs->ip = ip + 1 as breakpoint hit */
        instruction_pointer_set(regs, ip.wrapping_add(INT3_INSN_SIZE));

        __this_cpu_write(current_kprobe, p);
        (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
        if (*p).pre_handler.is_none() || !((*p).pre_handler.unwrap())(p, regs) {
            if unlikely((*p).post_handler.is_some()) {
                /*
                 * Emulate singlestep (and also recover regs->ip)
                 * as if there is a 5byte nop
                 */
                instruction_pointer_set(regs, ip.wrapping_add(MCOUNT_INSN_SIZE));
                (*kcb).kprobe_status = KPROBE_HIT_SSDONE;
                ((*p).post_handler.unwrap())(p, regs, 0);
            }
            /* Recover IP address */
            instruction_pointer_set(regs, orig_ip);
        }
        /*
         * If pre_handler returns !0, it changes regs->ip. We have to
         * skip emulating post_handler.
         */
        __this_cpu_write(current_kprobe, ::core::ptr::null_mut());
    }
    ftrace_test_recursion_unlock(bit);
}
// NOKPROBE_SYMBOL(kprobe_ftrace_handler);

pub unsafe fn arch_prepare_kprobe_ftrace(p: *mut kprobe) -> i32 {
    (*p).ainsn.insn = ::core::ptr::null_mut();
    (*p).ainsn.boostable = false;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
