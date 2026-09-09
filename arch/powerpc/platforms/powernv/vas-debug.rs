// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016-17 IBM Corp.
 */

// #define pr_fmt(fmt) "vas: " fmt
// Dependencies supplied by the Linux kernel and the surrounding VAS sources
// are intentionally left as external Rust symbols.

static mut vas_debugfs: *mut dentry = core::ptr::null_mut();

unsafe fn cop_to_str(cop: core::ffi::c_int) -> *const core::ffi::c_char {
    match cop {
        VAS_COP_TYPE_FAULT => c"Fault".as_ptr(),
        VAS_COP_TYPE_842 => c"NX-842 Normal Priority".as_ptr(),
        VAS_COP_TYPE_842_HIPRI => c"NX-842 High Priority".as_ptr(),
        VAS_COP_TYPE_GZIP => c"NX-GZIP Normal Priority".as_ptr(),
        VAS_COP_TYPE_GZIP_HIPRI => c"NX-GZIP High Priority".as_ptr(),
        VAS_COP_TYPE_FTW => c"Fast Thread-wakeup".as_ptr(),
        _ => c"Unknown".as_ptr(),
    }
}

unsafe fn info_show(s: *mut seq_file, _private: *mut core::ffi::c_void) -> core::ffi::c_int {
    let window = (*s).private as *mut pnv_vas_window;

    mutex_lock(&raw mut vas_mutex);

    // ensure window is not unmapped
    if (*window).hvwc_map.is_null() {
        mutex_unlock(&raw mut vas_mutex);
        return 0;
    }

    seq_printf(s, c"Type: %s, %s\n".as_ptr(), cop_to_str((*window).vas_win.cop),
        if (*window).tx_win { c"Send".as_ptr() } else { c"Receive".as_ptr() });
    seq_printf(s, c"Pid : %d\n".as_ptr(), vas_window_pid(&raw mut (*window).vas_win));

    mutex_unlock(&raw mut vas_mutex);
    0
}

// DEFINE_SHOW_ATTRIBUTE(info);

#[inline]
unsafe fn print_reg(s: *mut seq_file, win: *mut pnv_vas_window,
                    name: *const core::ffi::c_char, reg: u32) {
    seq_printf(s, c"0x%016llx %s\n".as_ptr(), read_hvwc_reg(win, name, reg), name);
}

unsafe fn hvwc_show(s: *mut seq_file, _private: *mut core::ffi::c_void) -> core::ffi::c_int {
    let window = (*s).private as *mut pnv_vas_window;

    mutex_lock(&raw mut vas_mutex);
    // ensure window is not unmapped
    if (*window).hvwc_map.is_null() {
        mutex_unlock(&raw mut vas_mutex);
        return 0;
    }

    print_reg(s, window, c"LPID".as_ptr(), VREG_LPID);
    print_reg(s, window, c"PID".as_ptr(), VREG_PID);
    print_reg(s, window, c"XLATE_MSR".as_ptr(), VREG_XLATE_MSR);
    print_reg(s, window, c"XLATE_LPCR".as_ptr(), VREG_XLATE_LPCR);
    print_reg(s, window, c"XLATE_CTL".as_ptr(), VREG_XLATE_CTL);
    print_reg(s, window, c"AMR".as_ptr(), VREG_AMR);
    print_reg(s, window, c"SEIDR".as_ptr(), VREG_SEIDR);
    print_reg(s, window, c"FAULT_TX_WIN".as_ptr(), VREG_FAULT_TX_WIN);
    print_reg(s, window, c"OSU_INTR_SRC_RA".as_ptr(), VREG_OSU_INTR_SRC_RA);
    print_reg(s, window, c"HV_INTR_SRC_RA".as_ptr(), VREG_HV_INTR_SRC_RA);
    print_reg(s, window, c"PSWID".as_ptr(), VREG_PSWID);
    print_reg(s, window, c"LFIFO_BAR".as_ptr(), VREG_LFIFO_BAR);
    print_reg(s, window, c"LDATA_STAMP_CTL".as_ptr(), VREG_LDATA_STAMP_CTL);
    print_reg(s, window, c"LDMA_CACHE_CTL".as_ptr(), VREG_LDMA_CACHE_CTL);
    print_reg(s, window, c"LRFIFO_PUSH".as_ptr(), VREG_LRFIFO_PUSH);
    print_reg(s, window, c"CURR_MSG_COUNT".as_ptr(), VREG_CURR_MSG_COUNT);
    print_reg(s, window, c"LNOTIFY_AFTER_COUNT".as_ptr(), VREG_LNOTIFY_AFTER_COUNT);
    print_reg(s, window, c"LRX_WCRED".as_ptr(), VREG_LRX_WCRED);
    print_reg(s, window, c"LRX_WCRED_ADDER".as_ptr(), VREG_LRX_WCRED_ADDER);
    print_reg(s, window, c"TX_WCRED".as_ptr(), VREG_TX_WCRED);
    print_reg(s, window, c"TX_WCRED_ADDER".as_ptr(), VREG_TX_WCRED_ADDER);
    print_reg(s, window, c"LFIFO_SIZE".as_ptr(), VREG_LFIFO_SIZE);
    print_reg(s, window, c"WINCTL".as_ptr(), VREG_WINCTL);
    print_reg(s, window, c"WIN_STATUS".as_ptr(), VREG_WIN_STATUS);
    print_reg(s, window, c"WIN_CTX_CACHING_CTL".as_ptr(), VREG_WIN_CTX_CACHING_CTL);
    print_reg(s, window, c"TX_RSVD_BUF_COUNT".as_ptr(), VREG_TX_RSVD_BUF_COUNT);
    print_reg(s, window, c"LRFIFO_WIN_PTR".as_ptr(), VREG_LRFIFO_WIN_PTR);
    print_reg(s, window, c"LNOTIFY_CTL".as_ptr(), VREG_LNOTIFY_CTL);
    print_reg(s, window, c"LNOTIFY_PID".as_ptr(), VREG_LNOTIFY_PID);
    print_reg(s, window, c"LNOTIFY_LPID".as_ptr(), VREG_LNOTIFY_LPID);
    print_reg(s, window, c"LNOTIFY_TID".as_ptr(), VREG_LNOTIFY_TID);
    print_reg(s, window, c"LNOTIFY_SCOPE".as_ptr(), VREG_LNOTIFY_SCOPE);
    print_reg(s, window, c"NX_UTIL_ADDER".as_ptr(), VREG_NX_UTIL_ADDER);

    mutex_unlock(&raw mut vas_mutex);
    0
}

// DEFINE_SHOW_ATTRIBUTE(hvwc);

pub unsafe fn vas_window_free_dbgdir(pnv_win: *mut pnv_vas_window) {
    let window = &mut (*pnv_win).vas_win;
    if !window.dbgdir.is_null() {
        debugfs_remove_recursive(window.dbgdir);
        kfree(window.dbgname);
        window.dbgdir = core::ptr::null_mut();
        window.dbgname = core::ptr::null_mut();
    }
}

pub unsafe fn vas_window_init_dbgdir(window: *mut pnv_vas_window) {
    if (*window).vinst.dbgdir.is_null() { return; }
    (*window).vas_win.dbgname = kzalloc(16, GFP_KERNEL);
    if (*window).vas_win.dbgname.is_null() { return; }
    snprintf((*window).vas_win.dbgname, 16, c"w%d".as_ptr(), (*window).vas_win.winid);
    let d = debugfs_create_dir((*window).vas_win.dbgname, (*window).vinst.dbgdir);
    (*window).vas_win.dbgdir = d;
    debugfs_create_file(c"info".as_ptr(), 0o444, d, window, &info_fops);
    debugfs_create_file(c"hvwc".as_ptr(), 0o444, d, window, &hvwc_fops);
}

pub unsafe fn vas_instance_init_dbgdir(vinst: *mut vas_instance) {
    vas_init_dbgdir();
    (*vinst).dbgname = kzalloc(16, GFP_KERNEL);
    if (*vinst).dbgname.is_null() { return; }
    snprintf((*vinst).dbgname, 16, c"v%d".as_ptr(), (*vinst).vas_id);
    (*vinst).dbgdir = debugfs_create_dir((*vinst).dbgname, vas_debugfs);
}

// Set up the "root" VAS debugfs dir. Return if already set up (or failed).
pub unsafe fn vas_init_dbgdir() {
    static mut first_time: bool = true;
    if !first_time { return; }
    first_time = false;
    vas_debugfs = debugfs_create_dir(c"vas".as_ptr(), core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
