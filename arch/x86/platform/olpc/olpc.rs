// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Support for the OLPC DCON and OLPC EC access
 *
 * Copyright © 2006  Advanced Micro Devices, Inc.
 * Copyright © 2007-2008  Andres Salomon <dilinger@debian.org>
 */

// Linux kernel dependencies and build-time configuration are supplied by the
// surrounding Rust translation.

#[repr(C)]
pub struct olpc_platform_t {
    pub boardrev: u32,
    pub flags: u32,
}

extern "C" {
    pub static mut olpc_platform_info: olpc_platform_t;
    fn get_option(str_: *mut *mut core::ffi::c_char, value: *mut i32) -> i32;
    fn printk(level: *const core::ffi::c_char, ...) -> i32;
    fn inb(port: u16) -> u8;
    fn outb(value: u8, port: u16);
    fn mdelay(ms: u32);
    fn pr_devel(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn of_get_property(root: *mut device_node, name: *const core::ffi::c_char,
                       lenp: *mut i32) -> *const u8;
    fn of_find_node_by_path(path: *const core::ffi::c_char) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn be32_to_cpu(value: u32) -> u32;
    fn platform_device_register_simple(name: *const core::ffi::c_char, id: i32,
                                       data: *const core::ffi::c_void, size: u32)
        -> *mut platform_device;
    fn olpc_ec_cmd(cmd: u8, inbuf: *const u8, inlen: usize,
                   outbuf: *mut u8, outlen: usize) -> i32;
    fn olpc_ofw_present() -> bool;
    fn olpc_ec_driver_register(driver: *mut olpc_ec_driver, arg: *const core::ffi::c_void);
    fn olpc_board_pre(value: u32) -> u32;
    fn olpc_board(value: u32) -> u32;
    fn olpc_board_at_least(value: u32) -> bool;
    fn cs5535_has_vsa2() -> bool;
}

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }

const EC_BASE_TIMEOUT: i32 = 20;
static mut ec_timeout: i32 = EC_BASE_TIMEOUT;

unsafe fn olpc_ec_timeout_set(str_: *mut core::ffi::c_char) -> i32 {
    if get_option(&mut (str_ as *mut _), &mut ec_timeout) != 1 { ec_timeout = EC_BASE_TIMEOUT; }
    1
}

unsafe fn ibf_status(port: u32) -> u32 { ((inb(port as u16) & 0x02) != 0) as u32 }
unsafe fn obf_status(port: u32) -> u32 { (inb(port as u16) & 0x01) as u32 }

unsafe fn __wait_on_ibf(line: u32, port: u32, desired: i32) -> i32 {
    let mut timeo = ec_timeout as u32;
    let mut state = ibf_status(port) as i32;
    while state != desired && timeo != 0 { mdelay(1); timeo -= 1; state = ibf_status(port) as i32; }
    if state == desired && ec_timeout > EC_BASE_TIMEOUT && (timeo as i32) < ec_timeout - EC_BASE_TIMEOUT { let _ = line; }
    (state != desired) as i32
}
unsafe fn __wait_on_obf(line: u32, port: u32, desired: i32) -> i32 {
    let mut timeo = ec_timeout as u32;
    let mut state = obf_status(port) as i32;
    while state != desired && timeo != 0 { mdelay(1); timeo -= 1; state = obf_status(port) as i32; }
    if state == desired && ec_timeout > EC_BASE_TIMEOUT && (timeo as i32) < ec_timeout - EC_BASE_TIMEOUT { let _ = line; }
    (state != desired) as i32
}

unsafe fn olpc_xo1_ec_cmd(cmd: u8, inbuf: *const u8, inlen: usize, outbuf: *mut u8, outlen: usize, _arg: *mut core::ffi::c_void) -> i32 {
    let mut ret = -5i32;
    let mut i = 0usize;
    let mut restarts = 0;
    while i < 10 && obf_status(0x6c) == 1 { inb(0x68); i += 1; }
    if i == 10 { return ret; }
    if __wait_on_ibf(0, 0x6c, 0) != 0 { return ret; }
    'restart: loop {
        outb(cmd, 0x6c);
        if __wait_on_ibf(0, 0x6c, 0) != 0 { break; }
        if !inbuf.is_null() { for i in 0..inlen { outb(*inbuf.add(i), 0x68); if __wait_on_ibf(0, 0x6c, 0) != 0 { break 'restart; } } }
        if !outbuf.is_null() {
            for i in 0..outlen { if __wait_on_obf(0, 0x6c, 1) != 0 { restarts += 1; if restarts <= 10 { continue 'restart; } break 'restart; } *outbuf.add(i) = inb(0x68); }
        }
        ret = 0;
        break;
    }
    ret
}

unsafe fn check_ofw_architecture(root: *mut device_node) -> bool {
    let mut size = 0; let p = of_get_property(root, b"architecture\0".as_ptr() as _, &mut size);
    size == 5 && !p.is_null() && core::slice::from_raw_parts(p, 5) == b"OLPC\0"
}
unsafe fn get_board_revision(root: *mut device_node) -> u32 { let mut size = 0; let p = of_get_property(root, b"board-revision-int\0".as_ptr() as _, &mut size); if size != 4 || p.is_null() { 0 } else { be32_to_cpu((*(p as *const u32))) } }
unsafe fn platform_detect() -> bool { let root = of_find_node_by_path(b"/\0".as_ptr() as _); if root.is_null() { return false; } let ok = check_ofw_architecture(root); if ok { olpc_platform_info.boardrev = get_board_revision(root); } of_node_put(root); ok }

unsafe fn add_xo1_platform_devices() -> i32 {
    let p = platform_device_register_simple(b"xo1-rfkill\0".as_ptr() as _, -1, core::ptr::null(), 0);
    if p.is_null() { return -1; }
    let p = platform_device_register_simple(b"olpc-xo1\0".as_ptr() as _, -1, core::ptr::null(), 0);
    if p.is_null() { -1 } else { 0 }
}

unsafe extern "C" fn olpc_xo1_ec_suspend(_pdev: *mut platform_device) -> i32 {
    olpc_ec_cmd(0x1f, core::ptr::null(), 0, core::ptr::null_mut(), 0)
}
unsafe extern "C" fn olpc_xo1_ec_resume(_pdev: *mut platform_device) -> i32 {
    olpc_ec_cmd(0x20, core::ptr::null(), 0, core::ptr::null_mut(), 0);
    olpc_ec_cmd(0x24, core::ptr::null(), 0, core::ptr::null_mut(), 0);
    olpc_ec_cmd(0x24, core::ptr::null(), 0, core::ptr::null_mut(), 0);
    0
}

#[repr(C)] struct olpc_ec_driver { suspend: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, resume: Option<unsafe extern "C" fn(*mut platform_device) -> i32>, ec_cmd: Option<unsafe extern "C" fn(u8,*const u8,usize,*mut u8,usize,*mut core::ffi::c_void)->i32>, wakeup_available: bool }

static mut ec_xo1_driver: olpc_ec_driver = olpc_ec_driver { suspend: Some(olpc_xo1_ec_suspend), resume: Some(olpc_xo1_ec_resume), ec_cmd: Some(olpc_xo1_ec_cmd), wakeup_available: false };
static mut ec_xo1_5_driver: olpc_ec_driver = olpc_ec_driver { suspend: None, resume: None, ec_cmd: Some(olpc_xo1_ec_cmd), wakeup_available: false };

unsafe fn olpc_init() -> i32 {
    if !olpc_ofw_present() || !platform_detect() { return 0; }
    if olpc_platform_info.boardrev < olpc_board_pre(0xd0) { olpc_ec_driver_register(&mut ec_xo1_driver, core::ptr::null()); } else { olpc_ec_driver_register(&mut ec_xo1_5_driver, core::ptr::null()); }
    platform_device_register_simple(b"olpc-ec\0".as_ptr() as _, -1, core::ptr::null(), 0);
    if olpc_platform_info.boardrev < olpc_board_pre(0xd0) { add_xo1_platform_devices() } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
