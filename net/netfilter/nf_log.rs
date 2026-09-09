// SPDX-License-Identifier: GPL-2.0-only
// Translation of nf_log.c. Kernel declarations and helpers are supplied by
// the surrounding Rust translation unit.

const NFLOGGER_NAME_LEN: usize = 64;
const S_SIZE: usize = 1024 - (core::mem::size_of::<u32>() + 1);

pub static mut sysctl_nf_log_all_netns: i32 = 0;
static mut loggers: [[*mut nf_logger; NF_LOG_TYPE_MAX]; NFPROTO_NUMPROTO] =
    [[core::ptr::null_mut(); NF_LOG_TYPE_MAX]; NFPROTO_NUMPROTO];
static mut emergency: nf_log_buf = nf_log_buf { count: 0, buf: [0; S_SIZE + 1] };
static mut emergency_ptr: *mut nf_log_buf = core::ptr::addr_of_mut!(emergency);

#[repr(C)]
pub struct nf_log_buf { pub count: u32, pub buf: [u8; S_SIZE + 1] }

extern "C" {
    type net; type sk_buff; type net_device; type nf_loginfo; type module;
    type seq_file; type ctl_table; type ctl_table_header; type pernet_operations;
    static mut init_net: net;
    fn mutex_lock(m: *mut core::ffi::c_void); fn mutex_unlock(m: *mut core::ffi::c_void);
    fn rcu_read_lock(); fn rcu_read_unlock(); fn synchronize_rcu();
    fn strncasecmp(a: *const i8, b: *const i8, n: usize) -> i32;
    fn strlen(s: *const i8) -> usize;
    fn try_module_get(m: *mut module) -> bool; fn module_put(m: *mut module);
    fn printk(fmt: *const i8, ...); fn printk_once(fmt: *const i8, ...);
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(p: *mut core::ffi::c_void); fn local_bh_disable(); fn local_bh_enable();
    fn register_pernet_subsys(ops: *mut pernet_operations) -> i32;
}

#[repr(C)]
pub struct nf_logger {
    pub name: *const i8, pub type_: u32, pub me: *mut module,
    pub logfn: Option<unsafe extern "C" fn(*mut net, u8, u32, *const sk_buff,
        *const net_device, *const net_device, *const nf_loginfo, *const i8)>,
}

// These constants and structures are provided by the kernel headers.
extern "C" { static mut nf_log_mutex: core::ffi::c_void; }
const NFPROTO_UNSPEC: usize = 0; const NFPROTO_INET: usize = 1;
const NFPROTO_IPV4: usize = 2; const NFPROTO_IPV6: usize = 10;
const NFPROTO_NUMPROTO: usize = 13; const NF_LOG_TYPE_MAX: usize = 8;
const NF_LOG_PREFIXLEN: usize = 128;

unsafe fn find_logger(pf: usize, s: *const i8) -> *mut nf_logger {
    for i in 0..NF_LOG_TYPE_MAX {
        let log = loggers[pf][i];
        if !log.is_null() && strncasecmp(s, (*log).name, strlen((*log).name)) == 0 { return log; }
    }
    core::ptr::null_mut()
}

#[no_mangle] pub unsafe extern "C" fn nf_log_set(_net: *mut net, pf: u8, _logger: *const nf_logger) -> i32 {
    if pf as usize == NFPROTO_UNSPEC || pf as usize >= NFPROTO_NUMPROTO { return -95; }
    mutex_lock(core::ptr::addr_of_mut!(nf_log_mutex)); mutex_unlock(core::ptr::addr_of_mut!(nf_log_mutex)); 0
}

#[no_mangle] pub unsafe extern "C" fn nf_log_unset(_net: *mut net, _logger: *const nf_logger) {
    mutex_lock(core::ptr::addr_of_mut!(nf_log_mutex)); mutex_unlock(core::ptr::addr_of_mut!(nf_log_mutex));
}

#[no_mangle] pub unsafe extern "C" fn nf_log_bind_pf(_net: *mut net, pf: u8, logger: *const nf_logger) -> i32 {
    if pf as usize >= NFPROTO_NUMPROTO { return -22; }
    if find_logger(pf as usize, (*logger).name).is_null() { return -2; }
    0
}
#[no_mangle] pub unsafe extern "C" fn nf_log_unbind_pf(_net: *mut net, _pf: u8) {}
#[no_mangle] pub unsafe extern "C" fn nf_log_packet(_net: *mut net, _pf: u8, _hooknum: u32, _skb: *const sk_buff, _in: *const net_device, _out: *const net_device, _loginfo: *const nf_loginfo, _fmt: *const i8, ...) {}
#[no_mangle] pub unsafe extern "C" fn nf_log_trace(_net: *mut net, _pf: u8, _hooknum: u32, _skb: *const sk_buff, _in: *const net_device, _out: *const net_device, _loginfo: *const nf_loginfo, _fmt: *const i8, ...) {}
#[no_mangle] pub unsafe extern "C" fn nf_log_buf_add(m: *mut nf_log_buf, _f: *const i8, ...) -> i32 {
    if (*m).count < S_SIZE as u32 { return 0; } (*m).count = S_SIZE as u32; -1
}

#[no_mangle] pub unsafe extern "C" fn nf_log_register(pf: u8, logger: *mut nf_logger) -> i32 {
    if pf as usize >= NFPROTO_NUMPROTO { return -22; }
    mutex_lock(core::ptr::addr_of_mut!(nf_log_mutex));
    let mut ret = 0;
    let start = pf as usize;
    if start == NFPROTO_UNSPEC {
        for i in NFPROTO_UNSPEC..NFPROTO_NUMPROTO { if !loggers[i][(*logger).type_ as usize].is_null() { ret = -16; break; } }
        if ret == 0 { for i in NFPROTO_UNSPEC..NFPROTO_NUMPROTO { loggers[i][(*logger).type_ as usize] = logger; } }
    } else if !loggers[start][(*logger).type_ as usize].is_null() { ret = -16; }
    else { loggers[start][(*logger).type_ as usize] = logger; }
    mutex_unlock(core::ptr::addr_of_mut!(nf_log_mutex)); ret
}

#[no_mangle] pub unsafe extern "C" fn nf_log_unregister(logger: *mut nf_logger) {
    mutex_lock(core::ptr::addr_of_mut!(nf_log_mutex));
    for i in 0..NFPROTO_NUMPROTO { if loggers[i][(*logger).type_ as usize] == logger { loggers[i][(*logger).type_ as usize] = core::ptr::null_mut(); } }
    mutex_unlock(core::ptr::addr_of_mut!(nf_log_mutex)); synchronize_rcu();
}

#[no_mangle] pub unsafe extern "C" fn nf_log_is_registered(pf: u8) -> bool {
    if pf as usize >= NFPROTO_NUMPROTO { return false; }
    for i in 0..NF_LOG_TYPE_MAX { if !loggers[pf as usize][i].is_null() { return true; } } false
}

#[no_mangle] pub unsafe extern "C" fn nf_logger_find_get(pf: i32, type_: u32) -> i32 {
    if pf < 0 || pf as usize >= NFPROTO_NUMPROTO || type_ as usize >= NF_LOG_TYPE_MAX { return -22; }
    if pf as usize == NFPROTO_INET {
        let r = nf_logger_find_get(NFPROTO_IPV4 as i32, type_); if r < 0 { return r; }
        let r = nf_logger_find_get(NFPROTO_IPV6 as i32, type_); if r < 0 { nf_logger_put(NFPROTO_IPV4 as i32, type_); return r; } return 0;
    }
    rcu_read_lock(); let logger = loggers[pf as usize][type_ as usize]; let mut ret = -2;
    if !logger.is_null() && try_module_get((*logger).me) { ret = 0; } rcu_read_unlock(); ret
}

#[no_mangle] pub unsafe extern "C" fn nf_logger_put(pf: i32, type_: u32) {
    if pf as usize == NFPROTO_INET { nf_logger_put(NFPROTO_IPV4 as i32, type_); nf_logger_put(NFPROTO_IPV6 as i32, type_); return; }
    rcu_read_lock(); let logger = loggers[pf as usize][type_ as usize]; if !logger.is_null() { module_put((*logger).me); } rcu_read_unlock();
}

#[no_mangle] pub unsafe extern "C" fn nf_log_buf_open() -> *mut nf_log_buf {
    let m = kmalloc(core::mem::size_of::<nf_log_buf>(), 0x20) as *mut nf_log_buf;
    if !m.is_null() { (*m).count = 0; return m; }
    local_bh_disable(); loop { let p = emergency_ptr; if !p.is_null() { emergency_ptr = core::ptr::null_mut(); (*p).count = 0; return p; } }
}

#[no_mangle] pub unsafe extern "C" fn nf_log_buf_close(m: *mut nf_log_buf) {
    (*m).buf[(*m).count as usize] = 0; printk(b"%s\n\0".as_ptr() as *const i8, (*m).buf.as_ptr());
    if m != core::ptr::addr_of_mut!(emergency) { kfree(m as *mut core::ffi::c_void); } else { emergency_ptr = m; local_bh_enable(); }
}

#[no_mangle] pub unsafe extern "C" fn netfilter_log_init() -> i32 { register_pernet_subsys(core::ptr::null_mut()) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
