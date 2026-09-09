// SPDX-License-Identifier: GPL-2.0
/*
 * kdb helper for dumping the ftrace buffer
 *
 * Copyright (C) 2010 Jason Wessel <jason.wessel@windriver.com>
 *
 * ftrace_dump_buf based on ftrace_dump:
 * Copyright (C) 2007-2008 Steven Rostedt <srostedt@redhat.com>
 * Copyright (C) 2008 Ingo Molnar <mingo@redhat.com>
 *
 */

// Dependencies supplied by the kernel tracing and kdb subsystems are external.

static mut ITER: trace_iterator = unsafe { core::mem::zeroed() };
static mut BUFFER_ITER: [*mut ring_buffer_iter; CONFIG_NR_CPUS] =
    [core::ptr::null_mut(); CONFIG_NR_CPUS];

unsafe fn ftrace_dump_buf(mut skip_entries: i32, cpu_file: i64) {
    let tr: *mut trace_array = ITER.tr;
    let old_userobj: u32 = (*tr).trace_flags;
    let mut cnt: i32 = 0;
    let mut cpu: i32;

    // don't look at user memory in panic mode
    (*tr).trace_flags &= !TRACE_ITER(SYM_USEROBJ);

    kdb_printf(c"Dumping ftrace buffer:\n".as_ptr());
    if skip_entries != 0 {
        kdb_printf(c"(skipping %d entries)\n".as_ptr(), skip_entries);
    }

    trace_iterator_reset(&mut ITER);
    ITER.iter_flags |= TRACE_FILE_LAT_FMT;

    if cpu_file == RING_BUFFER_ALL_CPUS {
        // Equivalent of for_each_tracing_cpu(cpu).
        for cpu in 0..NR_CPUS {
            ITER.buffer_iter[cpu as usize] = ring_buffer_read_start(
                (*ITER.array_buffer).buffer,
                cpu,
                GFP_ATOMIC,
            );
            tracing_iter_reset(&mut ITER, cpu);
        }
    } else {
        ITER.cpu_file = cpu_file;
        ITER.buffer_iter[cpu_file as usize] = ring_buffer_read_start(
            (*ITER.array_buffer).buffer,
            cpu_file as i32,
            GFP_ATOMIC,
        );
        tracing_iter_reset(&mut ITER, cpu_file as i32);
    }

    while trace_find_next_entry_inc(&mut ITER) {
        if cnt == 0 {
            kdb_printf(c"---------------------------------\n".as_ptr());
        }
        cnt += 1;

        if skip_entries == 0 {
            print_trace_line(&mut ITER);
            trace_printk_seq(&mut ITER.seq);
        } else {
            skip_entries -= 1;
        }

        if KDB_FLAG(CMD_INTERRUPT) {
            break;
        }
    }

    if cnt == 0 {
        kdb_printf(c"   (ftrace buffer empty)\n".as_ptr());
    } else {
        kdb_printf(c"---------------------------------\n".as_ptr());
    }

    (*tr).trace_flags = old_userobj;

    // Equivalent of for_each_tracing_cpu(cpu).
    for cpu in 0..NR_CPUS {
        let i = cpu as usize;
        if !ITER.buffer_iter[i].is_null() {
            ring_buffer_read_finish(ITER.buffer_iter[i]);
            ITER.buffer_iter[i] = core::ptr::null_mut();
        }
    }
}

/*
 * kdb_ftdump - Dump the ftrace log buffer
 */
unsafe fn kdb_ftdump(argc: i32, argv: *const *const c_char) -> i32 {
    let mut skip_entries: i32 = 0;
    let cpu_file: i64;
    let mut err: i32;
    let mut cnt: i32;

    if argc > 2 {
        return KDB_ARGCOUNT;
    }

    if argc != 0 {
        if kstrtoint(*argv.add(1), 0, &mut skip_entries) != 0 {
            return KDB_BADINT;
        }
    }

    if argc == 2 {
        let mut parsed_cpu: i64 = 0;
        err = kstrtol(*argv.add(2), 0, &mut parsed_cpu);
        if err != 0 || parsed_cpu >= NR_CPUS as i64 || parsed_cpu < 0 || !cpu_online(parsed_cpu) {
            return KDB_BADINT;
        }
        cpu_file = parsed_cpu;
    } else {
        cpu_file = RING_BUFFER_ALL_CPUS;
    }

    kdb_trap_printk += 1;

    trace_init_global_iter(&mut ITER);
    ITER.buffer_iter = BUFFER_ITER.as_mut_ptr();

    tracer_tracing_disable(ITER.tr);

    // A negative skip_entries means skip all but the last entries
    if skip_entries < 0 {
        if cpu_file == RING_BUFFER_ALL_CPUS {
            cnt = trace_total_entries(core::ptr::null_mut());
        } else {
            cnt = trace_total_entries_cpu(core::ptr::null_mut(), cpu_file);
        }
        skip_entries = core::cmp::max(cnt + skip_entries, 0);
    }

    ftrace_dump_buf(skip_entries, cpu_file);

    tracer_tracing_enable(ITER.tr);

    kdb_trap_printk -= 1;

    0
}

static mut FTDUMP_CMD: kdbtab_t = kdbtab_t {
    name: c"ftdump".as_ptr(),
    func: Some(kdb_ftdump),
    usage: c"[skip_#entries] [cpu]".as_ptr(),
    help: c"Dump ftrace log; -skip dumps last #entries".as_ptr(),
    flags: KDB_ENABLE_ALWAYS_SAFE,
};

unsafe extern "C" fn kdb_ftrace_register() -> i32 {
    kdb_register(&mut FTDUMP_CMD);
    0
}

late_initcall!(kdb_ftrace_register);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
