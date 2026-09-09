/*
 * `ptrace' system call
 *
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2004-2007 John Williams <john.williams@petalogix.com>
 *
 * derived from arch/v850/kernel/ptrace.c
 *
 * Copyright (C) 2002,03 NEC Electronics Corporation
 * Copyright (C) 2002,03 Miles Bader <miles@gnu.org>
 *
 * Derived from arch/mips/kernel/ptrace.c:
 *
 * Copyright (C) 1992 Ross Biro
 * Copyright (C) Linus Torvalds
 * Copyright (C) 1994, 95, 96, 97, 98, 2000 Ralf Baechle
 * Copyright (C) 1996 David S. Miller
 * Kevin D. Kissell, kevink@mips.com and Carsten Langgaard, carstenl@mips.com
 * Copyright (C) 1999 MIPS Technologies, Inc.
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License. See the file COPYING in the main directory of this
 * archive for more details.
 */

/* Linux and MicroBlaze dependencies supplied by other translation units. */

/* Returns the address where the register at REG_OFFS in P is stashed away. */
unsafe fn reg_save_addr(reg_offs: c_uint, t: *mut task_struct) -> *mut microblaze_reg_t {
    let regs: *mut pt_regs;

    /*
     * Three basic cases:
     *
     * (1) A register normally saved before calling the scheduler, is
     *     available in the kernel entry pt_regs structure at the top
     *     of the kernel stack. The kernel trap/irq exit path takes
     *     care to save/restore almost all registers for ptrace'd
     *     processes.
     *
     * (2) A call-clobbered register, where the process P entered the
     *     kernel via [syscall] trap, is not stored anywhere; that's
     *     OK, because such registers are not expected to be preserved
     *     when the trap returns anyway (so we don't actually bother to
     *     test for this case).
     *
     * (3) A few registers not used at all by the kernel, and so
     *     normally never saved except by context-switches, are in the
     *     context switch state.
     */

    /* Register saved during kernel entry (or not available). */
    regs = task_pt_regs(t);

    (regs as *mut u8).add(reg_offs as usize) as *mut microblaze_reg_t
}

unsafe fn arch_ptrace(
    child: *mut task_struct,
    request: c_long,
    addr: c_ulong,
    data: c_ulong,
) -> c_long {
    let mut rval: c_int;
    let mut val: c_ulong = 0;

    match request {
        PTRACE_PEEKUSR | PTRACE_POKEUSR => {
            pr_debug!("PEEKUSR/POKEUSR : 0x%08lx\n", addr);
            rval = 0;
            if addr >= PT_SIZE && request == PTRACE_PEEKUSR {
                /* Special requests that don't actually correspond to offsets in struct pt_regs. */
                if addr == PT_TEXT_ADDR {
                    val = (*(*child).mm).start_code;
                } else if addr == PT_DATA_ADDR {
                    val = (*(*child).mm).start_data;
                } else if addr == PT_TEXT_LEN {
                    val = (*(*child).mm).end_code - (*(*child).mm).start_code;
                } else {
                    rval = -EIO;
                }
            } else if addr < PT_SIZE && (addr & 0x3) == 0 {
                let reg_addr = reg_save_addr(addr as c_uint, child);
                if request == PTRACE_PEEKUSR {
                    val = *reg_addr as c_ulong;
                } else {
                    /* The active source selects the direct register write. */
                    *reg_addr = data as microblaze_reg_t;
                    /*
                     * Alternative disabled C path:
                     * virt_to_phys, invalidate_icache_range, the write, and
                     * flush_dcache_range would be used on a WB system.
                     */
                }
            } else {
                rval = -EIO;
            }

            if rval == 0 && request == PTRACE_PEEKUSR {
                rval = put_user(val, data as *mut c_ulong);
            }
        }
        _ => {
            rval = ptrace_request(child, request, addr, data);
        }
    }
    rval as c_long
}

unsafe fn do_syscall_trace_enter(regs: *mut pt_regs) -> c_ulong {
    let mut ret: c_ulong = 0;

    secure_computing_strict((*regs).r12);

    if test_thread_flag(TIF_SYSCALL_TRACE) != 0
        && ptrace_report_syscall_permit_entry(regs) == 0
    {
        /*
         * Tracing decided this syscall should not happen.
         * We'll return a bogus call number to get an ENOSYS
         * error, but leave the original number in regs->regs[0].
         */
        ret = (-1i64) as c_ulong;
    }

    audit_syscall_entry((*regs).r12, (*regs).r5, (*regs).r6, (*regs).r7, (*regs).r8);

    if ret != 0 { ret } else { (*regs).r12 }
}

unsafe fn do_syscall_trace_leave(regs: *mut pt_regs) {
    let step: c_int;

    audit_syscall_exit(regs);

    step = test_thread_flag(TIF_SINGLESTEP);
    if step != 0 || test_thread_flag(TIF_SYSCALL_TRACE) != 0 {
        ptrace_report_syscall_exit(regs, step);
    }
}

unsafe fn ptrace_disable(_child: *mut task_struct) {
    /* nothing to do */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
