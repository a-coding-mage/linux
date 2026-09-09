// SPDX-License-Identifier: GPL-2.0
/*
 * proc_tty.c -- handles /proc/tty
 *
 * Copyright 1997, Theodore Ts'o
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

static mut proc_tty_driver: *mut proc_dir_entry = core::ptr::null_mut();

/*
 * This is the handler for /proc/tty/drivers
 */
unsafe fn show_tty_range(m: *mut seq_file, p: *mut tty_driver, from: dev_t, num: i32) {
    seq_printf(m, c"%-20s ".as_ptr(), if !(*p).driver_name.is_null() { (*p).driver_name } else { c"unknown".as_ptr() });
    seq_printf(m, c"/dev/%-8s ".as_ptr(), (*p).name);
    if (*p).num > 1 {
        seq_printf(m, c"%3d %d-%d ".as_ptr(), MAJOR(from), MINOR(from), MINOR(from) + num - 1);
    } else {
        seq_printf(m, c"%3d %7d ".as_ptr(), MAJOR(from), MINOR(from));
    }
    match (*p).type_ {
        TTY_DRIVER_TYPE_SYSTEM => {
            seq_puts(m, c"system".as_ptr());
            if (*p).subtype == SYSTEM_TYPE_TTY { seq_puts(m, c":/dev/tty".as_ptr()); }
            else if (*p).subtype == SYSTEM_TYPE_SYSCONS { seq_puts(m, c":console".as_ptr()); }
            else if (*p).subtype == SYSTEM_TYPE_CONSOLE { seq_puts(m, c":vtmaster".as_ptr()); }
        }
        TTY_DRIVER_TYPE_CONSOLE => seq_puts(m, c"console".as_ptr()),
        TTY_DRIVER_TYPE_SERIAL => seq_puts(m, c"serial".as_ptr()),
        TTY_DRIVER_TYPE_PTY => {
            if (*p).subtype == PTY_TYPE_MASTER { seq_puts(m, c"pty:master".as_ptr()); }
            else if (*p).subtype == PTY_TYPE_SLAVE { seq_puts(m, c"pty:slave".as_ptr()); }
            else { seq_puts(m, c"pty".as_ptr()); }
        }
        _ => seq_printf(m, c"type:%d.%d".as_ptr(), (*p).type_, (*p).subtype),
    }
    seq_putc(m, b'\n' as i32);
}

unsafe fn show_tty_driver(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let p = list_entry!(v, tty_driver, tty_drivers);
    let from = MKDEV((*p).major, (*p).minor_start);
    let to = from + (*p).num as dev_t;

    if core::ptr::addr_of!((*p).tty_drivers) == tty_drivers.next {
        /* pseudo-drivers first */
        seq_printf(m, c"%-20s /dev/%-8s ".as_ptr(), c"/dev/tty".as_ptr(), c"tty".as_ptr());
        seq_printf(m, c"%3d %7d ".as_ptr(), TTYAUX_MAJOR, 0);
        seq_puts(m, c"system:/dev/tty\n".as_ptr());
        seq_printf(m, c"%-20s /dev/%-8s ".as_ptr(), c"/dev/console".as_ptr(), c"console".as_ptr());
        seq_printf(m, c"%3d %7d ".as_ptr(), TTYAUX_MAJOR, 1);
        seq_puts(m, c"system:console\n".as_ptr());
        // CONFIG_UNIX98_PTYS conditionally adds the ptmx pseudo-driver.
        // CONFIG_VT conditionally adds the vc/0 pseudo-driver.
    }

    let mut current = from;
    while MAJOR(current) < MAJOR(to) {
        let next = MKDEV(MAJOR(current) + 1, 0);
        show_tty_range(m, p, current, (next - current) as i32);
        current = next;
    }
    if current != to { show_tty_range(m, p, current, (to - current) as i32); }
    0
}

/* iterator */
unsafe fn t_start(m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    mutex_lock(&mut tty_mutex);
    seq_list_start(&mut tty_drivers, *pos)
}

unsafe fn t_next(m: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    seq_list_next(v, &mut tty_drivers, pos)
}

unsafe fn t_stop(m: *mut seq_file, v: *mut core::ffi::c_void) { mutex_unlock(&mut tty_mutex); }

static tty_drivers_op: seq_operations = seq_operations {
    start: Some(t_start), next: Some(t_next), stop: Some(t_stop), show: Some(show_tty_driver),
};

/*
 * This function is called by tty_register_driver() to handle
 * registering the driver's /proc handler into /proc/tty/driver/<foo>
 */
unsafe fn proc_tty_register_driver(driver: *mut tty_driver) {
    if (*driver).driver_name.is_null() || !(*driver).proc_entry.is_null() || (*driver).ops.proc_show.is_none() { return; }
    let ent = proc_create_single_data((*driver).driver_name, 0, proc_tty_driver, (*driver).ops.proc_show, driver as *mut _);
    (*driver).proc_entry = ent;
}

/* This function is called by tty_unregister_driver() */
unsafe fn proc_tty_unregister_driver(driver: *mut tty_driver) {
    let ent = (*driver).proc_entry;
    if ent.is_null() { return; }
    remove_proc_entry((*ent).name, proc_tty_driver);
    (*driver).proc_entry = core::ptr::null_mut();
}

/* Called by proc_root_init() to initialize the /proc/tty subtree */
unsafe fn proc_tty_init() {
    if proc_mkdir(c"tty".as_ptr(), core::ptr::null_mut()).is_null() { return; }
    proc_mkdir(c"tty/ldisc".as_ptr(), core::ptr::null_mut()); /* Preserved: it's userspace visible */
    /* See the source comment: serial character counts can expose passwords. */
    proc_tty_driver = proc_mkdir_mode(c"tty/driver".as_ptr(), S_IRUSR | S_IXUSR, core::ptr::null_mut());
    proc_create_seq(c"tty/ldiscs".as_ptr(), 0, core::ptr::null_mut(), &tty_ldiscs_seq_ops);
    proc_create_seq(c"tty/drivers".as_ptr(), 0, core::ptr::null_mut(), &tty_drivers_op);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
