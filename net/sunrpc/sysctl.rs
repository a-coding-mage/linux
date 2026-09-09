// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/net/sunrpc/sysctl.c
 *
 * Sysctl interface to sunrpc module.
 *
 * I would prefer to register the sunrpc table below sys/net, but that's
 * impossible at the moment.
 */

// C dependencies supplied by the surrounding kernel translation.

/* Declare the debug flags here */
#[no_mangle]
pub static mut rpc_debug: u32 = 0;
#[no_mangle]
pub static mut nfs_debug: u32 = 0;
#[no_mangle]
pub static mut nfsd_debug: u32 = 0;
#[no_mangle]
pub static mut nlm_debug: u32 = 0;

// Preserves: #if IS_ENABLED(CONFIG_SUNRPC_DEBUG)
#[cfg(feature = "CONFIG_SUNRPC_DEBUG")]
static mut sunrpc_table_header: *mut ctl_table_header = core::ptr::null_mut();

#[cfg(feature = "CONFIG_SUNRPC_DEBUG")]
unsafe fn proc_do_xprt(
    _table: *const ctl_table,
    write: i32,
    buffer: *mut core::ffi::c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> i32 {
    let mut tmpbuf = [0i8; 256];
    let mut len: isize;

    if write != 0 || *ppos != 0 {
        *lenp = 0;
        return 0;
    }
    len = svc_print_xprts(tmpbuf.as_mut_ptr(), tmpbuf.len());
    len = memory_read_from_buffer(buffer, *lenp, ppos, tmpbuf.as_ptr(), len);

    if len < 0 {
        *lenp = 0;
        return -22; // -EINVAL
    }
    *lenp = len as usize;
    0
}

#[cfg(feature = "CONFIG_SUNRPC_DEBUG")]
unsafe fn proc_dodebug(
    table: *const ctl_table,
    write: i32,
    buffer: *mut core::ffi::c_void,
    lenp: *mut usize,
    ppos: *mut loff_t,
) -> i32 {
    let mut tmpbuf = [0i8; 20];
    let mut s: *mut i8 = core::ptr::null_mut();
    let mut p: *mut i8;
    let mut value: u32;
    let mut left: usize;
    let mut len: usize;

    if ((*ppos != 0 && write == 0) || *lenp == 0) {
        *lenp = 0;
        return 0;
    }
    left = *lenp;

    if write != 0 {
        p = buffer as *mut i8;
        while left != 0 && isspace(*p as i32) != 0 {
            left -= 1;
            p = p.add(1);
        }
        if left == 0 { goto_done(&mut *lenp, &mut *ppos, left); return 0; }
        if left > tmpbuf.len() - 1 { return -22; }
        core::ptr::copy_nonoverlapping(p, tmpbuf.as_mut_ptr(), left);
        tmpbuf[left] = 0;

        value = simple_strtol(tmpbuf.as_ptr(), &mut s, 0) as u32;
        if !s.is_null() {
            left -= s.offset_from(tmpbuf.as_ptr()) as usize;
            if left != 0 && isspace(*s as i32) == 0 { return -22; }
            while left != 0 && isspace(*s as i32) != 0 {
                left -= 1;
                s = s.add(1);
            }
        } else { left = 0; }
        *( (*table).data as *mut u32) = value;
        if c_str_eq((*table).procname, b"rpc_debug\0".as_ptr() as *const i8) {
            rpc_show_tasks(&init_net);
        }
    } else {
        len = sprintf(tmpbuf.as_mut_ptr(), b"0x%04x\0".as_ptr() as *const i8, *((*table).data as *const u32));
        if len > left { len = left; }
        core::ptr::copy_nonoverlapping(tmpbuf.as_ptr(), buffer as *mut i8, len);
        left -= len;
        if left > 0 {
            *((buffer as *mut i8).add(len)) = b'\n' as i8;
            left -= 1;
        }
    }

    *lenp -= left;
    *ppos += *lenp as loff_t;
    0
}

#[cfg(feature = "CONFIG_SUNRPC_DEBUG")]
unsafe fn goto_done(lenp: &mut usize, ppos: &mut loff_t, left: usize) {
    *lenp -= left;
    *ppos += *lenp as loff_t;
}

// The following declarations are supplied by other translated kernel files.
#[cfg(feature = "CONFIG_SUNRPC_DEBUG")]
extern "C" {
    static init_net: net;
    fn svc_print_xprts(buf: *mut i8, size: usize) -> isize;
    fn memory_read_from_buffer(to: *mut core::ffi::c_void, count: usize, ppos: *mut loff_t, from: *const i8, available: isize) -> isize;
    fn isspace(c: i32) -> i32;
    fn simple_strtol(s: *const i8, end: *mut *mut i8, base: u32) -> isize;
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> usize;
    fn rpc_show_tasks(net: *const net);
    fn register_sysctl(name: *const i8, table: *mut ctl_table) -> *mut ctl_table_header;
    fn unregister_sysctl_table(header: *mut ctl_table_header);
    fn c_str_eq(a: *const i8, b: *const i8) -> bool;
}

#[cfg(feature = "CONFIG_SUNRPC_DEBUG")]
#[repr(C)]
struct ctl_table {
    procname: *const i8,
    data: *mut core::ffi::c_void,
    maxlen: usize,
    mode: u16,
    proc_handler: Option<unsafe fn(*const ctl_table, i32, *mut core::ffi::c_void, *mut usize, *mut loff_t) -> i32>,
}

#[cfg(feature = "CONFIG_SUNRPC_DEBUG")]
#[repr(C)]
struct ctl_table_header {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SUNRPC_DEBUG")]
#[repr(C)]
struct net {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SUNRPC_DEBUG")]
static mut debug_table: [ctl_table; 5] = [
    ctl_table { procname: b"rpc_debug\0".as_ptr() as *const i8, data: core::ptr::addr_of_mut!(rpc_debug) as *mut _, maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dodebug) },
    ctl_table { procname: b"nfs_debug\0".as_ptr() as *const i8, data: core::ptr::addr_of_mut!(nfs_debug) as *mut _, maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dodebug) },
    ctl_table { procname: b"nfsd_debug\0".as_ptr() as *const i8, data: core::ptr::addr_of_mut!(nfsd_debug) as *mut _, maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dodebug) },
    ctl_table { procname: b"nlm_debug\0".as_ptr() as *const i8, data: core::ptr::addr_of_mut!(nlm_debug) as *mut _, maxlen: core::mem::size_of::<i32>(), mode: 0o644, proc_handler: Some(proc_dodebug) },
    ctl_table { procname: b"transports\0".as_ptr() as *const i8, data: core::ptr::null_mut(), maxlen: 256, mode: 0o444, proc_handler: Some(proc_do_xprt) },
];

#[cfg(feature = "CONFIG_SUNRPC_DEBUG")]
pub unsafe fn rpc_register_sysctl() {
    if sunrpc_table_header.is_null() {
        sunrpc_table_header = register_sysctl(b"sunrpc\0".as_ptr() as *const i8, debug_table.as_mut_ptr());
    }
}

#[cfg(feature = "CONFIG_SUNRPC_DEBUG")]
pub unsafe fn rpc_unregister_sysctl() {
    if !sunrpc_table_header.is_null() {
        unregister_sysctl_table(sunrpc_table_header);
        sunrpc_table_header = core::ptr::null_mut();
    }
}

type loff_t = i64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
