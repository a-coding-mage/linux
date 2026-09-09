// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2008 Steven Rostedt <srostedt@redhat.com>
 */

// Kernel headers and symbols are supplied by the surrounding translation unit.

const STACK_TRACE_ENTRIES: usize = 500;

static mut stack_dump_trace: [c_ulong; STACK_TRACE_ENTRIES] = [0; STACK_TRACE_ENTRIES];
static mut stack_trace_index: [c_uint; STACK_TRACE_ENTRIES] = [0; STACK_TRACE_ENTRIES];
static mut stack_trace_nr_entries: c_uint = 0;
static mut stack_trace_max_size: c_ulong = 0;
static mut stack_trace_max_lock: arch_spinlock_t = unsafe { __ARCH_SPIN_LOCK_UNLOCKED };
static mut disable_stack_tracer: c_int = 0; // DEFINE_PER_CPU
static mut stack_sysctl_mutex: mutex = unsafe { MUTEX_INITIALIZER };
static mut stack_tracer_enabled: c_int = 0;

unsafe fn print_max_stack() {
    let mut i: c_long;
    let mut size: c_int;
    pr_emerg!("        Depth    Size   Location    ({} entries)\n        -----    ----   --------\n", stack_trace_nr_entries);
    i = 0;
    while i < stack_trace_nr_entries as c_long {
        if i + 1 == stack_trace_nr_entries as c_long {
            size = stack_trace_index[i as usize] as c_int;
        } else {
            size = (stack_trace_index[i as usize] - stack_trace_index[(i + 1) as usize]) as c_int;
        }
        pr_emerg!("{:3}) {:8}   {:5}   {:pS}\n", i, stack_trace_index[i as usize], size,
                  stack_dump_trace[i as usize] as *const c_void);
        i += 1;
    }
}

unsafe fn check_stack(ip: c_ulong, stack: *mut c_ulong) {
    let mut this_size: c_ulong;
    let mut flags: c_ulong = 0;
    let mut p: *mut c_ulong;
    let mut top: *mut c_ulong;
    let mut start: *mut c_ulong;
    static mut tracer_frame: c_int = 0;
    let frame_size = READ_ONCE!(tracer_frame);
    let mut i: c_int;
    let mut x: c_int;

    this_size = (stack as c_ulong) & (THREAD_SIZE - 1);
    this_size = THREAD_SIZE - this_size;
    this_size -= frame_size as c_ulong;
    if this_size <= stack_trace_max_size || !object_is_on_stack(stack) || in_nmi() { return; }

    local_irq_save!(flags);
    arch_spin_lock(&mut stack_trace_max_lock);
    if frame_size == 0 { this_size -= tracer_frame as c_ulong; }
    if this_size <= stack_trace_max_size { goto_out!(arch_spin_unlock(&mut stack_trace_max_lock); local_irq_restore!(flags)); }
    stack_trace_max_size = this_size;
    stack_trace_nr_entries = stack_trace_save(stack_dump_trace.as_mut_ptr(), STACK_TRACE_ENTRIES - 1, 0);
    i = 0;
    while i < stack_trace_nr_entries as c_int && stack_dump_trace[i as usize] != ip { i += 1; }
    if i == stack_trace_nr_entries as c_int { i = 0; }
    x = 0;
    start = stack;
    top = ((((start as c_ulong) & !(THREAD_SIZE - 1)) + THREAD_SIZE) as *mut c_ulong);
    while i < stack_trace_nr_entries as c_int {
        let mut found = false;
        stack_trace_index[x as usize] = this_size as c_uint;
        p = start;
        while p < top && i < stack_trace_nr_entries as c_int {
            if READ_ONCE_NOCHECK!(*p) == stack_dump_trace[i as usize] {
                stack_dump_trace[x as usize] = stack_dump_trace[i as usize]; i += 1;
                this_size = (top.offset_from(p) as c_ulong) * core::mem::size_of::<c_ulong>() as c_ulong;
                stack_trace_index[x as usize] = this_size as c_uint; x += 1; found = true; start = p.add(1);
                if tracer_frame == 0 { tracer_frame = p.offset_from(stack) as c_int * core::mem::size_of::<c_ulong>() as c_int; stack_trace_max_size -= tracer_frame as c_ulong; }
            }
            p = p.add(1);
        }
        if !found { i += 1; }
    }
    // ARCH_FTRACE_SHIFT_STACK_TRACER shifts the recorded indexes by one.
    #[cfg(ARCH_FTRACE_SHIFT_STACK_TRACER)]
    if x > 1 { core::ptr::copy(stack_trace_index.as_ptr().add(1), stack_trace_index.as_mut_ptr(), (x - 1) as usize); x -= 1; }
    stack_trace_nr_entries = x as c_uint;
    if task_stack_end_corrupted(current) { print_max_stack(); BUG!(); }
    arch_spin_unlock(&mut stack_trace_max_lock);
    local_irq_restore!(flags);
}

// MCOUNT_INSN_SIZE defaults to zero when the architecture does not define it.
#[cfg(not(MCOUNT_INSN_SIZE))] const MCOUNT_INSN_SIZE: c_ulong = 0;

unsafe fn stack_trace_call(mut ip: c_ulong, _parent_ip: c_ulong, _op: *mut ftrace_ops, _fregs: *mut ftrace_regs) {
    let mut stack: c_ulong = 0;
    preempt_disable_notrace();
    this_cpu_inc!(disable_stack_tracer);
    if this_cpu_read!(disable_stack_tracer) == 1 && rcu_is_watching() { ip += MCOUNT_INSN_SIZE; check_stack(ip, &mut stack); }
    this_cpu_dec!(disable_stack_tracer);
    preempt_enable_notrace();
}

static mut trace_ops: ftrace_ops = ftrace_ops { func: Some(stack_trace_call), ..unsafe { core::mem::zeroed() } };

unsafe fn stack_max_size_read(filp: *mut file, ubuf: *mut c_char, count: usize, ppos: *mut loff_t) -> isize {
    let ptr = (*filp).private_data as *mut c_ulong;
    let mut buf = [0 as c_char; 64];
    let r = snprintf(buf.as_mut_ptr(), buf.len(), b"%ld\n\0".as_ptr() as *const c_char, *ptr);
    simple_read_from_buffer(ubuf, count, ppos, buf.as_ptr(), r.min(buf.len()))
}
unsafe fn stack_max_size_write(filp: *mut file, ubuf: *const c_char, count: usize, ppos: *mut loff_t) -> isize {
    let ptr = (*filp).private_data as *mut c_ulong; let mut val = 0; let mut flags = 0;
    let ret = kstrtoul_from_user(ubuf, count, 10, &mut val); if ret != 0 { return ret as isize; }
    local_irq_save!(flags); this_cpu_inc!(disable_stack_tracer); arch_spin_lock(&mut stack_trace_max_lock);
    *ptr = val; arch_spin_unlock(&mut stack_trace_max_lock); this_cpu_dec!(disable_stack_tracer); local_irq_restore!(flags); count as isize
}
unsafe fn __next(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void { let n = *pos - 1; if n >= stack_trace_nr_entries as loff_t { return core::ptr::null_mut(); } (*m).private = n as *mut c_void; &mut (*m).private }
unsafe fn t_next(m: *mut seq_file, _v: *mut c_void, pos: *mut loff_t) -> *mut c_void { *pos += 1; __next(m, pos) }
unsafe fn t_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void { local_irq_disable(); this_cpu_inc!(disable_stack_tracer); arch_spin_lock(&mut stack_trace_max_lock); if *pos == 0 { SEQ_START_TOKEN } else { __next(m, pos) } }
unsafe fn t_stop(_m: *mut seq_file, _p: *mut c_void) { arch_spin_unlock(&mut stack_trace_max_lock); this_cpu_dec!(disable_stack_tracer); local_irq_enable(); }
unsafe fn trace_lookup_stack(m: *mut seq_file, i: c_long) { seq_printf(m, b"%pS\n\0".as_ptr() as *const c_char, stack_dump_trace[i as usize] as *mut c_void); }
unsafe fn print_disabled(m: *mut seq_file) { seq_puts(m, b"#\n#  Stack tracer disabled\n#\n# To enable the stack tracer, either add 'stacktrace' to the\n# kernel command line\n# or 'echo 1 > /proc/sys/kernel/stack_tracer_enabled'\n#\n\0".as_ptr() as *const c_char); }
unsafe fn t_show(m: *mut seq_file, v: *mut c_void) -> c_int { if v == SEQ_START_TOKEN { seq_printf(m, b"        Depth    Size   Location    (%d entries)\n        -----    ----   --------\n\0".as_ptr() as *const c_char, stack_trace_nr_entries); if stack_tracer_enabled == 0 && stack_trace_max_size == 0 { print_disabled(m); } return 0; } let i = *(v as *mut c_long); if i < stack_trace_nr_entries as c_long { let size = if i + 1 == stack_trace_nr_entries as c_long { stack_trace_index[i as usize] } else { stack_trace_index[i as usize] - stack_trace_index[i as usize + 1] }; seq_printf(m, b"%3ld) %8d   %5d   \0".as_ptr() as *const c_char, i, stack_trace_index[i as usize], size); trace_lookup_stack(m, i); } 0 }
unsafe fn stack_trace_sysctl(table: *const ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    let was_enabled = (stack_tracer_enabled != 0) as c_int;
    let ret = proc_dointvec(table, write, buffer, lenp, ppos);
    if ret != 0 || write == 0 || was_enabled == (stack_tracer_enabled != 0) as c_int { return ret; }
    if stack_tracer_enabled != 0 { register_ftrace_function(&mut trace_ops) } else { unregister_ftrace_function(&mut trace_ops) }; ret
}

static mut stack_trace_filter_buf: [c_char; COMMAND_LINE_SIZE + 1] = [0; COMMAND_LINE_SIZE + 1];
unsafe fn enable_stacktrace(str_: *mut c_char) -> c_int {
    let len = str_has_prefix(str_, b"_filter=\0".as_ptr() as *const c_char);
    if len != 0 { strscpy(stack_trace_filter_buf.as_mut_ptr(), str_.add(len as usize)); }
    stack_tracer_enabled = 1; 1
}

unsafe fn stack_trace_open(_inode: *mut inode, file: *mut file) -> c_int { seq_open(file, &stack_trace_seq_ops) }
unsafe fn stack_trace_init() -> c_int { if tracing_init_dentry() != 0 { return 0; } trace_create_file(b"stack_max_size\0".as_ptr(), TRACE_MODE_WRITE, core::ptr::null_mut(), &mut stack_trace_max_size as *mut _, &stack_max_size_fops); trace_create_file(b"stack_trace\0".as_ptr(), TRACE_MODE_READ, core::ptr::null_mut(), core::ptr::null_mut(), &stack_trace_fops); if stack_tracer_enabled != 0 { register_ftrace_function(&mut trace_ops); } 0 }
unsafe fn init_trace_stack_sysctls() -> c_int { register_sysctl_init(b"kernel\0".as_ptr(), trace_stack_sysctl_table.as_ptr()); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
