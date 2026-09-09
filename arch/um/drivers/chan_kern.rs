// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
 */

// Kernel and UML dependencies are supplied by the surrounding translation unit.

#[cfg(CONFIG_NOCONFIG_CHAN)]
unsafe fn not_configged_init(_str: *mut i8, _device: i32,
                             _opts: *const chan_opts) -> *mut core::ffi::c_void {
    printk(KERN_ERR, b"Using a channel type which is configured out of UML\0".as_ptr() as *const i8);
    core::ptr::null_mut()
}
#[cfg(CONFIG_NOCONFIG_CHAN)]
unsafe fn not_configged_open(_input: i32, _output: i32, _primary: i32,
                             _data: *mut core::ffi::c_void,
                             _dev_out: *mut *mut i8) -> i32 {
    printk(KERN_ERR, b"Using a channel type which is configured out of UML\0".as_ptr() as *const i8); -ENODEV
}
#[cfg(CONFIG_NOCONFIG_CHAN)]
unsafe fn not_configged_close(_fd: i32, _data: *mut core::ffi::c_void) {
    printk(KERN_ERR, b"Using a channel type which is configured out of UML\0".as_ptr() as *const i8);
}
#[cfg(CONFIG_NOCONFIG_CHAN)]
unsafe fn not_configged_read(_fd: i32, _c_out: *mut u8, _data: *mut core::ffi::c_void) -> i32 {
    printk(KERN_ERR, b"Using a channel type which is configured out of UML\0".as_ptr() as *const i8); -EIO
}
#[cfg(CONFIG_NOCONFIG_CHAN)]
unsafe fn not_configged_write(_fd: i32, _buf: *const u8, _len: usize, _data: *mut core::ffi::c_void) -> i32 {
    printk(KERN_ERR, b"Using a channel type which is configured out of UML\0".as_ptr() as *const i8); -EIO
}
#[cfg(CONFIG_NOCONFIG_CHAN)]
unsafe fn not_configged_console_write(_fd: i32, _buf: *const i8, _len: i32) -> i32 {
    printk(KERN_ERR, b"Using a channel type which is configured out of UML\0".as_ptr() as *const i8); -EIO
}
#[cfg(CONFIG_NOCONFIG_CHAN)]
unsafe fn not_configged_window_size(_fd: i32, _data: *mut core::ffi::c_void,
                                    _rows: *mut u16, _cols: *mut u16) -> i32 {
    printk(KERN_ERR, b"Using a channel type which is configured out of UML\0".as_ptr() as *const i8); -ENODEV
}
#[cfg(CONFIG_NOCONFIG_CHAN)]
unsafe fn not_configged_free(_data: *mut core::ffi::c_void) {
    printk(KERN_ERR, b"Using a channel type which is configured out of UML\0".as_ptr() as *const i8);
}

#[inline]
unsafe fn need_output_blocking() -> bool {
    time_travel_mode == TT_MODE_INFCPU || time_travel_mode == TT_MODE_EXTERNAL
}

unsafe fn open_one_chan(chan: *mut chan) -> i32 {
    if (*chan).opened != 0 { return 0; }
    let mut fd = if (*(*chan).ops).open.is_null() { 0 } else {
        ((*(*chan).ops).open.unwrap())((*chan).input, (*chan).output, (*chan).primary,
                                        (*chan).data, &mut (*chan).dev)
    };
    if fd < 0 { return fd; }
    let mut err = os_set_fd_block(fd, 0);
    if err != 0 { ((*(*chan).ops).close.unwrap())(fd, (*chan).data); return err; }
    (*chan).fd_in = fd; (*chan).fd_out = fd;
    if (*chan).output != 0 && need_output_blocking() {
        err = os_dup_file((*chan).fd_out);
        if err < 0 { ((*(*chan).ops).close.unwrap())(fd, (*chan).data); return err; }
        (*chan).fd_out = err;
        err = os_set_fd_block((*chan).fd_out, 1);
        if err != 0 { os_close_file((*chan).fd_out); ((*(*chan).ops).close.unwrap())(fd, (*chan).data); return err; }
    }
    (*chan).opened = 1; 0
}

unsafe fn open_chan(chans: *mut list_head) -> i32 {
    let mut err = 0;
    // list_for_each: supplied by the kernel list implementation.
    list_for_each_mut(chans, |ele: *mut list_head| {
        let chan = list_entry_chan(ele);
        let ret = open_one_chan(chan);
        if (*chan).primary != 0 { err = ret; }
    });
    err
}

pub unsafe fn chan_enable_winch(chan: *mut chan, port: *mut tty_port) {
    if !chan.is_null() && (*chan).primary != 0 && !(*(*chan).ops).winch.is_null() {
        register_winch((*chan).fd_in, port);
    }
}

unsafe fn line_timer_cb(work: *mut work_struct) {
    let line = container_of_line(work);
    if (*line).throttled == 0 { chan_interrupt(line, (*line).read_irq); }
}

pub unsafe fn enable_chan(line: *mut line) -> i32 {
    INIT_DELAYED_WORK(&mut (*line).task, line_timer_cb);
    let mut err = 0;
    list_for_each_mut(&mut (*line).chan_list, |ele: *mut list_head| {
        let chan = list_entry_chan(ele);
        err = open_one_chan(chan);
        if err != 0 { if (*chan).primary != 0 { return; } return; }
        if (*chan).enabled != 0 { return; }
        err = line_setup_irq((*chan).fd_in, (*chan).input,
                             (*chan).output != 0 && !need_output_blocking(), line, chan);
        if err == 0 { (*chan).enabled = 1; }
    });
    if err != 0 { close_chan(line); }
    err
}

static mut irqs_to_free_lock: raw_spinlock_t = RAW_SPINLOCK_INITIALIZER;
static mut irqs_to_free: list_head = LIST_HEAD_INIT;

pub unsafe fn free_irqs() {
    let mut list = LIST_HEAD_INIT;
    let mut flags: ulong = 0;
    raw_spin_lock_irqsave(&mut irqs_to_free_lock, &mut flags);
    list_splice_init(&mut irqs_to_free, &mut list);
    raw_spin_unlock_irqrestore(&mut irqs_to_free_lock, flags);
    list_for_each_mut(&mut list, |ele: *mut list_head| {
        let chan = list_entry_free_chan(ele);
        if (*chan).input != 0 && (*chan).enabled != 0 { um_free_irq((*(*chan).line).read_irq, chan); }
        if (*chan).output != 0 && (*chan).enabled != 0 && !need_output_blocking() { um_free_irq((*(*chan).line).write_irq, chan); }
        (*chan).enabled = 0;
    });
}

unsafe fn close_one_chan(chan: *mut chan, delay_free_irq: i32) {
    if (*chan).opened == 0 { return; }
    if delay_free_irq != 0 {
        let mut flags: ulong = 0;
        raw_spin_lock_irqsave(&mut irqs_to_free_lock, &mut flags);
        list_add(&mut (*chan).free_list, &mut irqs_to_free);
        raw_spin_unlock_irqrestore(&mut irqs_to_free_lock, flags);
    } else {
        if (*chan).input != 0 && (*chan).enabled != 0 { um_free_irq((*(*chan).line).read_irq, chan); }
        if (*chan).output != 0 && (*chan).enabled != 0 && !need_output_blocking() { um_free_irq((*(*chan).line).write_irq, chan); }
        (*chan).enabled = 0;
    }
    if (*chan).fd_out != (*chan).fd_in { os_close_file((*chan).fd_out); }
    if !(*(*chan).ops).close.is_null() { ((*(*chan).ops).close.unwrap())((*chan).fd_in, (*chan).data); }
    (*chan).opened = 0; (*chan).fd_in = -1; (*chan).fd_out = -1;
}

pub unsafe fn close_chan(line: *mut line) {
    list_for_each_entry_reverse(&mut (*line).chan_list, |chan: *mut chan| close_one_chan(chan, 0));
}

pub unsafe fn deactivate_chan(chan: *mut chan, irq: i32) {
    if !chan.is_null() && (*chan).enabled != 0 { deactivate_fd((*chan).fd_in, irq); }
}

pub unsafe fn write_chan(chan: *mut chan, buf: *const u8, len: usize, _write_irq: i32) -> i32 {
    if len == 0 || chan.is_null() || (*(*chan).ops).write.is_null() { return 0; }
    let n = ((*(*chan).ops).write.unwrap())((*chan).fd_out, buf, len, (*chan).data);
    if (*chan).primary != 0 { n } else { 0 }
}

pub unsafe fn console_write_chan(chan: *mut chan, buf: *const i8, len: i32) -> i32 {
    if chan.is_null() || (*(*chan).ops).console_write.is_null() { return 0; }
    let n = ((*(*chan).ops).console_write.unwrap())((*chan).fd_out, buf, len);
    if (*chan).primary != 0 { n } else { 0 }
}

pub unsafe fn console_open_chan(line: *mut line, co: *mut console) -> i32 {
    let err = open_chan(&mut (*line).chan_list);
    if err != 0 { return err; }
    printk(KERN_INFO, b"Console initialized on /dev/%s%d\0".as_ptr() as *const i8, (*co).name, (*co).index); 0
}

pub unsafe fn chan_window_size(line: *mut line, rows: *mut u16, cols: *mut u16) -> i32 {
    let mut c = (*line).chan_in;
    if !c.is_null() && (*c).primary != 0 {
        if (*(*c).ops).window_size.is_null() { return 0; }
        return ((*(*c).ops).window_size.unwrap())((*c).fd_in, (*c).data, rows, cols);
    }
    c = (*line).chan_out;
    if !c.is_null() && (*c).primary != 0 {
        if (*(*c).ops).window_size.is_null() { return 0; }
        return ((*(*c).ops).window_size.unwrap())((*c).fd_in, (*c).data, rows, cols);
    }
    0
}

unsafe fn free_one_chan(chan: *mut chan) {
    list_del(&mut (*chan).list); close_one_chan(chan, 0);
    if !(*(*chan).ops).free.is_null() { ((*(*chan).ops).free.unwrap())((*chan).data); }
    if (*chan).primary != 0 && (*chan).output != 0 { ignore_sigio_fd((*chan).fd_in); }
    kfree(chan as *mut core::ffi::c_void);
}
unsafe fn free_chan(chans: *mut list_head) {
    list_for_each_safe_mut(chans, |ele: *mut list_head, _next: *mut list_head| free_one_chan(list_entry_chan(ele)));
}

unsafe fn one_chan_config_string(chan: *mut chan, str_: *mut i8, size: i32, error_out: *mut *mut i8) -> i32 {
    let mut n = 0;
    if chan.is_null() { CONFIG_CHUNK(str_, size, n, b"none\0".as_ptr() as *const i8, 1); return n; }
    CONFIG_CHUNK(str_, size, n, (*(*chan).ops).type_, 0);
    if (*chan).dev.is_null() { CONFIG_CHUNK(str_, size, n, b"\0".as_ptr() as *const i8, 1); return n; }
    CONFIG_CHUNK(str_, size, n, b":\0".as_ptr() as *const i8, 0);
    CONFIG_CHUNK(str_, size, n, (*chan).dev, 0); n
}
unsafe fn chan_pair_config_string(input: *mut chan, output: *mut chan, str_: *mut i8, size: i32, error_out: *mut *mut i8) -> i32 {
    let mut n = one_chan_config_string(input, str_, size, error_out); let mut p = str_.add(n as usize); let mut left = size - n;
    if input == output { CONFIG_CHUNK(p, left, n, b"\0".as_ptr() as *const i8, 1); return n; }
    CONFIG_CHUNK(p, left, n, b",\0".as_ptr() as *const i8, 1);
    n = one_chan_config_string(output, p, left, error_out); p = p.add(n as usize); left -= n;
    CONFIG_CHUNK(p, left, n, b"\0".as_ptr() as *const i8, 1); n
}
pub unsafe fn chan_config_string(line: *mut line, str_: *mut i8, size: i32, error_out: *mut *mut i8) -> i32 {
    let mut input = (*line).chan_in; let mut output = (*line).chan_out;
    if !input.is_null() && (*input).primary == 0 { input = core::ptr::null_mut(); }
    if !output.is_null() && (*output).primary == 0 { output = core::ptr::null_mut(); }
    chan_pair_config_string(input, output, str_, size, error_out)
}

#[repr(C)] struct chan_type { key: *mut i8, ops: *const chan_ops }
// CONFIG_* branches below preserve the source build-time configuration intent.
static chan_table: &[chan_type] = &[
    chan_type { key: b"fd\0".as_ptr() as *mut i8, ops: &fd_ops },
    chan_type { key: b"null\0".as_ptr() as *mut i8, ops: &null_ops },
    chan_type { key: b"port\0".as_ptr() as *mut i8, ops: &port_ops },
    chan_type { key: b"pty\0".as_ptr() as *mut i8, ops: &pty_ops },
    chan_type { key: b"pts\0".as_ptr() as *mut i8, ops: &pts_ops },
    chan_type { key: b"tty\0".as_ptr() as *mut i8, ops: &tty_ops },
    chan_type { key: b"xterm\0".as_ptr() as *mut i8, ops: &xterm_ops },
];

unsafe fn parse_chan(line: *mut line, str_: *mut i8, device: i32, opts: *const chan_opts, error_out: *mut *mut i8) -> *mut chan {
    let mut ops: *const chan_ops = core::ptr::null(); let mut s = str_; let mut i = 0;
    while i < chan_table.len() { let e = &chan_table[i]; if !strncmp(s, e.key, strlen(e.key)) { ops = e.ops; s = s.add(strlen(e.key)); break; } i += 1; }
    if ops.is_null() { *error_out = b"No match for configured backends\0".as_ptr() as *mut i8; return core::ptr::null_mut(); }
    let data = ((*ops).init.unwrap())(s, device, opts);
    if data.is_null() { *error_out = b"Configuration failed\0".as_ptr() as *mut i8; return core::ptr::null_mut(); }
    let chan = kmalloc_chan(GFP_ATOMIC); if chan.is_null() { *error_out = b"Memory allocation failed\0".as_ptr() as *mut i8; return chan; }
    *chan = chan { list: LIST_HEAD_INIT, free_list: LIST_HEAD_INIT, line, primary: 1, input: 0, output: 0, opened: 0, enabled: 0, fd_in: -1, fd_out: -1, ops, data, dev: core::ptr::null_mut() }; chan
}

pub unsafe fn parse_chan_pair(str_: *mut i8, line: *mut line, device: i32, opts: *const chan_opts, error_out: *mut *mut i8) -> i32 {
    let chans = &mut (*line).chan_list;
    if !list_empty(chans) { (*line).chan_in = core::ptr::null_mut(); (*line).chan_out = core::ptr::null_mut(); free_chan(chans); INIT_LIST_HEAD(chans); }
    if str_.is_null() { return 0; }
    let out = strchr(str_, b',' as i32); if !out.is_null() {
        *out = 0; let input = parse_chan(line, str_, device, opts, error_out); if input.is_null() { return -1; }
        (*input).input = 1; list_add(&mut (*input).list, chans); (*line).chan_in = input;
        let output = parse_chan(line, out.add(1), device, opts, error_out); if output.is_null() { return -1; }
        list_add(&mut (*output).list, chans); (*output).output = 1; (*line).chan_out = output;
    } else { let c = parse_chan(line, str_, device, opts, error_out); if c.is_null() { return -1; } list_add(&mut (*c).list, chans); (*c).input = 1; (*c).output = 1; (*line).chan_in = c; (*line).chan_out = c; }
    0
}

pub unsafe fn chan_interrupt(line: *mut line, _irq: i32) {
    let port = &mut (*line).port; let chan = (*line).chan_in; let mut err; let mut c: u8 = 0;
    if chan.is_null() || (*(*chan).ops).read.is_null() { tty_flip_buffer_push(port); return; }
    loop { if tty_buffer_request_room(port, 1) == 0 { schedule_delayed_work(&mut (*line).task, 1); return; }
        err = ((*(*chan).ops).read.unwrap())((*chan).fd_in, &mut c, (*chan).data); if err > 0 { tty_insert_flip_char(port, c, TTY_NORMAL); } else { break; } }
    if err == -EIO { if (*chan).primary != 0 { tty_port_tty_hangup(&mut (*line).port, false); if (*line).chan_out != chan { close_one_chan((*line).chan_out, 1); } } close_one_chan(chan, 1); if (*chan).primary != 0 { return; } }
    tty_flip_buffer_push(port);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
