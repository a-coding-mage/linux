// SPDX-License-Identifier: GPL-2.0-only
/*
 * kretprobe_example.c
 *
 * Here's a sample kernel module showing the use of return probes to
 * report the return value and total time taken for probed function
 * to run.
 *
 * usage: insmod kretprobe_example.ko func=<func_name>
 *
 * If no func_name is specified, kernel_clone is instrumented
 *
 * For more information on theory of operation of kretprobes, see
 * Documentation/trace/kprobes.rst
 *
 * Build and insert the kernel module as done in the kprobe example.
 * You will see the trace data in /var/log/messages and on the console
 * whenever the probed function returns. (Some messages may be suppressed
 * if syslogd is configured to eliminate duplicate messages.)
 */

// Dependencies supplied by the Linux kernel bindings are referenced below.

const KSYM_NAME_LEN: usize = 256;

static mut FUNC_NAME: [u8; KSYM_NAME_LEN] = {
    let mut value = [0u8; KSYM_NAME_LEN];
    value[0] = b'k';
    value[1] = b'e';
    value[2] = b'r';
    value[3] = b'n';
    value[4] = b'e';
    value[5] = b'l';
    value[6] = b'_';
    value[7] = b'c';
    value[8] = b'l';
    value[9] = b'o';
    value[10] = b'n';
    value[11] = b'e';
    value
};

// module_param_string(func, func_name, KSYM_NAME_LEN, 0644);
// MODULE_PARM_DESC(func, "Function to kretprobe; this module will report the function's execution time");

/* per-instance private data */
#[repr(C)]
struct MyData {
    entry_stamp: ktime_t,
}

// Here we use the entry_handler to timestamp function entry
unsafe fn entry_handler(ri: *mut kretprobe_instance, _regs: *mut pt_regs) -> i32 {
    let data: *mut MyData;

    if (*current).mm.is_null() {
        return 1; // Skip kernel threads
    }

    data = (*ri).data as *mut MyData;
    (*data).entry_stamp = ktime_get();
    0
}

// NOKPROBE_SYMBOL(entry_handler);

/*
 * Return-probe handler: Log the return value and duration. Duration may turn
 * out to be zero consistently, depending upon the granularity of time
 * accounting on the platform.
 */
unsafe fn ret_handler(ri: *mut kretprobe_instance, regs: *mut pt_regs) -> i32 {
    let retval: c_ulong = regs_return_value(regs);
    let data: *mut MyData = (*ri).data as *mut MyData;
    let delta: s64;
    let now: ktime_t;

    now = ktime_get();
    delta = ktime_to_ns(ktime_sub(now, (*data).entry_stamp));
    pr_info!(
        "{} returned {} and took {} ns to execute\n",
        FUNC_NAME.as_ptr(),
        retval,
        delta as c_longlong
    );
    0
}

// NOKPROBE_SYMBOL(ret_handler);

static mut MY_KRETPROBE: kretprobe = kretprobe {
    handler: Some(ret_handler),
    entry_handler: Some(entry_handler),
    data_size: core::mem::size_of::<MyData>(),
    // Probe up to 20 instances concurrently.
    maxactive: 20,
    ..kretprobe::default()
};

unsafe fn kretprobe_init() -> i32 {
    let ret: i32;

    MY_KRETPROBE.kp.symbol_name = FUNC_NAME.as_mut_ptr() as *mut c_char;
    ret = register_kretprobe(&mut MY_KRETPROBE);
    if ret < 0 {
        pr_err!("register_kretprobe failed, returned {}\n", ret);
        return ret;
    }
    pr_info!(
        "Planted return probe at {}: {:p}\n",
        MY_KRETPROBE.kp.symbol_name,
        MY_KRETPROBE.kp.addr
    );
    0
}

unsafe fn kretprobe_exit() {
    unregister_kretprobe(&mut MY_KRETPROBE);
    pr_info!("kretprobe at {:p} unregistered\n", MY_KRETPROBE.kp.addr);

    /* nmissed > 0 suggests that maxactive was set too low. */
    pr_info!(
        "Missed probing {} instances of {}\n",
        MY_KRETPROBE.nmissed,
        MY_KRETPROBE.kp.symbol_name
    );
}

// module_init(kretprobe_init)
// module_exit(kretprobe_exit)
// MODULE_DESCRIPTION("sample kernel module showing the use of return probes");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
