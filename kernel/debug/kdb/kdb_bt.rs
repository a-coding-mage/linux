// SPDX-License-Identifier: GPL-2.0
/*
 * Kernel Debugger Architecture Independent Stack Traceback
 *
 * Copyright (c) 1999-2004 Silicon Graphics, Inc.  All Rights Reserved.
 * Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
 */

// Linux kernel headers and "kdb_private.h" supply the declarations used below.

unsafe fn kdb_show_stack(p: *mut task_struct, addr: *mut core::ffi::c_void) {
    kdb_trap_printk += 1;

    if addr.is_null() && kdb_task_has_cpu(p) {
        let old_lvl: i32 = console_loglevel;

        console_loglevel = CONSOLE_LOGLEVEL_MOTORMOUTH;
        kdb_dump_stack_on_cpu(kdb_process_cpu(p));
        console_loglevel = old_lvl;
    } else {
        show_stack(p, addr, KERN_EMERG);
    }

    kdb_trap_printk -= 1;
}

/*
 * kdb_bt
 *
 *	This function implements the 'bt' command.  Print a stack
 *	traceback.
 *
 *	bt [<address-expression>]	(addr-exp is for alternate stacks)
 *	btp <pid>			Kernel stack for <pid>
 *	btt <address-expression>	Kernel stack for task structure at
 *					<address-expression>
 *	bta [state_chars>|A]		All useful processes, optionally
 *					filtered by state
 *	btc [<cpu>]			The current process on one cpu,
 *					default is all cpus
 *
 *	bt <address-expression> refers to a address on the stack, that location
 *	is assumed to contain a return address.
 *
 *	btt <address-expression> refers to the address of a struct task.
 *
 * Inputs:
 *	argc	argument count
 *	argv	argument vector
 * Outputs:
 *	None.
 * Returns:
 *	zero for success, a kdb diagnostic if error
 * Locking:
 *	none.
 * Remarks:
 *	Backtrack works best when the code uses frame pointers.  But even
 *	without frame pointers we should get a reasonable trace.
 *
 *	mds comes in handy when examining the stack to do a manual traceback or
 *	to get a starting point for bt <address-expression>.
 */

unsafe fn kdb_bt1(p: *mut task_struct, mask: *const core::ffi::c_char, btaprompt: bool) -> i32 {
    let mut ch: core::ffi::c_char;

    if kdb_getarea(&mut ch, p as usize) != 0
        || kdb_getarea(&mut ch, (p.add(1) as usize).wrapping_sub(1)) != 0
    {
        return KDB_BADADDR;
    }
    if !kdb_task_state(p, mask) {
        return 0;
    }
    kdb_printf("Stack traceback for pid %d\n", (*p).pid);
    kdb_ps1(p);
    kdb_show_stack(p, core::ptr::null_mut());
    if btaprompt {
        kdb_printf("Enter <q> to end, <cr> or <space> to continue:");
        loop {
            ch = kdb_getchar();
            if b"\r\n q\0".contains(&(ch as u8)) {
                break;
            }
        }
        kdb_printf("\n");

        /* reset the pager */
        kdb_nextline = 1;

        if ch == b'q' as i8 {
            return 1;
        }
    }
    touch_nmi_watchdog();
    0
}

unsafe fn kdb_bt_cpu(cpu: u64) {
    let mut kdb_tsk: *mut task_struct;

    if cpu >= num_possible_cpus() || !cpu_online(cpu) {
        kdb_printf("WARNING: no process for cpu %ld\n", cpu);
        return;
    }

    /* If a CPU failed to round up we could be here */
    kdb_tsk = KDB_TSK(cpu);
    if kdb_tsk.is_null() {
        kdb_printf("WARNING: no task for cpu %ld\n", cpu);
        return;
    }

    kdb_bt1(kdb_tsk, c"A".as_ptr(), false);
}

pub unsafe fn kdb_bt(argc: i32, mut argv: *const *const core::ffi::c_char) -> i32 {
    let mut diag: i32;
    let mut btaprompt: i32 = 1;
    let mut nextarg: i32;
    let mut addr: u64;
    let mut offset: i64;

    /* Prompt after each proc in bta */
    kdbgetintenv(c"BTAPROMPT".as_ptr(), &mut btaprompt);

    if strcmp(*argv, c"bta".as_ptr()) == 0 {
        let mut g: *mut task_struct;
        let mut p: *mut task_struct;
        let mut cpu: u64;
        let mask: *const core::ffi::c_char = if argc != 0 { *argv.add(1) } else { kdbgetenv(c"PS".as_ptr()) };

        if argc == 0 {
            kdb_ps_suppressed();
        }
        // for_each_online_cpu(cpu)
        for_each_online_cpu!(cpu, {
            p = curr_task(cpu);
            if kdb_bt1(p, mask, btaprompt != 0) != 0 {
                return 0;
            }
        });
        // for_each_process_thread(g, p)
        for_each_process_thread!(g, p, {
            if KDB_FLAG(CMD_INTERRUPT) {
                return 0;
            }
            if task_curr(p) {
                continue;
            }
            if kdb_bt1(p, mask, btaprompt != 0) != 0 {
                return 0;
            }
        });
    } else if strcmp(*argv, c"btp".as_ptr()) == 0 {
        let mut p: *mut task_struct;
        let mut pid: u64;
        if argc != 1 {
            return KDB_ARGCOUNT;
        }
        diag = kdbgetularg(*argv.add(1) as *mut core::ffi::c_char, &mut pid);
        if diag != 0 {
            return diag;
        }
        p = find_task_by_pid_ns(pid, &init_pid_ns);
        if !p.is_null() {
            return kdb_bt1(p, c"A".as_ptr(), false);
        }
        kdb_printf("No process with pid == %ld found\n", pid);
        return 0;
    } else if strcmp(*argv, c"btt".as_ptr()) == 0 {
        if argc != 1 {
            return KDB_ARGCOUNT;
        }
        diag = kdbgetularg(*argv.add(1) as *mut core::ffi::c_char, &mut addr);
        if diag != 0 {
            return diag;
        }
        return kdb_bt1(addr as *mut task_struct, c"A".as_ptr(), false);
    } else if strcmp(*argv, c"btc".as_ptr()) == 0 {
        let mut cpu: u64 = !0;
        if argc > 1 {
            return KDB_ARGCOUNT;
        }
        if argc == 1 {
            diag = kdbgetularg(*argv.add(1) as *mut core::ffi::c_char, &mut cpu);
            if diag != 0 {
                return diag;
            }
        }
        if cpu != !0 {
            kdb_bt_cpu(cpu);
        } else {
            /*
             * Recursive use of kdb_parse, do not use argv after
             * this point.
             */
            argv = core::ptr::null();
            kdb_printf("btc: cpu status: ");
            kdb_parse(c"cpu\n".as_ptr());
            // for_each_online_cpu(cpu)
            for_each_online_cpu!(cpu, {
                kdb_bt_cpu(cpu);
                touch_nmi_watchdog();
            });
        }
        return 0;
    } else {
        if argc != 0 {
            nextarg = 1;
            diag = kdbgetaddrarg(argc, argv, &mut nextarg, &mut addr, &mut offset, core::ptr::null_mut());
            if diag != 0 {
                return diag;
            }
            kdb_show_stack(kdb_current_task, addr as *mut core::ffi::c_void);
            return 0;
        } else {
            return kdb_bt1(kdb_current_task, c"A".as_ptr(), false);
        }
    }

    /* NOTREACHED */
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
