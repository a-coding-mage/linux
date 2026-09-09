// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Kernel headers and symbols supplied by other translation units.
extern "C" {
    static SYS_INFO_TASKS: c_ulong;
    static SYS_INFO_MEM: c_ulong;
    static SYS_INFO_TIMERS: c_ulong;
    static SYS_INFO_LOCKS: c_ulong;
    static SYS_INFO_FTRACE: c_ulong;
    static SYS_INFO_PANIC_CONSOLE_REPLAY: c_ulong;
    static SYS_INFO_ALL_BT: c_ulong;
    static SYS_INFO_BLOCKED_TASKS: c_ulong;

    fn match_string(arr: *const *const c_char, n: usize, s: *const c_char) -> c_int;
    fn __set_bit(nr: c_int, addr: *mut c_ulong);
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn proc_dostring(
        table: *const ctl_table,
        write: c_int,
        buffer: *mut c_void,
        lenp: *mut usize,
        ppos: *mut i64,
    ) -> c_int;
    fn show_state();
    fn show_mem();
    fn sysrq_timer_list_show();
    fn debug_show_all_locks();
    fn ftrace_dump(flags: c_int);
    fn trigger_all_cpu_backtrace();
    fn show_state_filter(state: c_ulong);
    fn register_sysctl_init(name: *const c_char, table: *const ctl_table);
}

#[repr(C)]
pub struct ctl_table {
    pub procname: *const c_char,
    pub data: *mut c_void,
    pub maxlen: usize,
    pub mode: u16,
    pub proc_handler: Option<unsafe extern "C" fn(*const ctl_table, c_int, *mut c_void, *mut usize, *mut i64) -> c_int>,
}

static SI_NAMES: [*const c_char; 8] = [
    b"tasks\0".as_ptr() as *const c_char,
    b"mem\0".as_ptr() as *const c_char,
    b"timers\0".as_ptr() as *const c_char,
    b"locks\0".as_ptr() as *const c_char,
    b"ftrace\0".as_ptr() as *const c_char,
    b"\0".as_ptr() as *const c_char,
    b"all_bt\0".as_ptr() as *const c_char,
    b"blocked_tasks\0".as_ptr() as *const c_char,
];

/* Default kernel sys_info mask. */
static mut KERNEL_SI_MASK: c_ulong = 0;

/* Expecting string like "xxx_sys_info=tasks,mem,timers,locks,ftrace,..." */
#[no_mangle]
pub unsafe extern "C" fn sys_info_parse_param(mut str_: *mut c_char) -> c_ulong {
    let mut si_bits: c_ulong = 0;
    let mut s = str_;
    let comma = b",\0".as_ptr() as *const c_char;
    loop {
        let name = strsep(&mut s, comma);
        if name.is_null() || *name == 0 {
            break;
        }
        let i = match_string(SI_NAMES.as_ptr(), SI_NAMES.len(), name);
        if i >= 0 {
            __set_bit(i, &mut si_bits);
        }
    }
    si_bits
}

#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn sys_info_write_handler(table: *const ctl_table, buffer: *mut c_void, lenp: *mut usize, ppos: *mut i64, si_bits_global: *mut c_ulong) -> c_int {
    let ret = proc_dostring(table, 1, buffer, lenp, ppos);
    if ret != 0 { return ret; }
    let si_bits = sys_info_parse_param((*table).data as *mut c_char);
    core::ptr::write_volatile(si_bits_global, si_bits);
    0
}

#[cfg(CONFIG_SYSCTL)]
unsafe extern "C" fn sys_info_read_handler(table: *const ctl_table, buffer: *mut c_void, lenp: *mut usize, ppos: *mut i64, si_bits_global: *mut c_ulong) -> c_int {
    let si_bits = core::ptr::read_volatile(si_bits_global);
    let mut len = 0usize;
    let mut delim = b"\0".as_ptr() as *const c_char;
    for i in 0..SI_NAMES.len() {
        if (si_bits & (1 << i)) != 0 && *SI_NAMES[i] != 0 {
            let fmt = b"%s%s\0".as_ptr() as *const c_char;
            len += scnprintf(((*table).data as *mut c_char).add(len), (*table).maxlen - len, fmt, delim, SI_NAMES[i]) as usize;
            delim = b",\0".as_ptr() as *const c_char;
        }
    }
    proc_dostring(table, 0, buffer, lenp, ppos)
}

#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
pub unsafe extern "C" fn sysctl_sys_info_handler(ro_table: *const ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut usize, ppos: *mut i64) -> c_int {
    let mut maxlen = 0usize;
    for name in SI_NAMES { maxlen += strlen(name) + 1; }
    let mut names = vec![0u8; maxlen];
    let mut table = core::ptr::read(ro_table);
    table.data = names.as_mut_ptr() as *mut c_void;
    table.maxlen = maxlen;
    if write != 0 { sys_info_write_handler(&table, buffer, lenp, ppos, (*ro_table).data as *mut c_ulong) }
    else { sys_info_read_handler(&table, buffer, lenp, ppos, (*ro_table).data as *mut c_ulong) }
}

unsafe fn __sys_info(si_mask: c_ulong) {
    if si_mask & SYS_INFO_TASKS != 0 { show_state(); }
    if si_mask & SYS_INFO_MEM != 0 { show_mem(); }
    if si_mask & SYS_INFO_TIMERS != 0 { sysrq_timer_list_show(); }
    if si_mask & SYS_INFO_LOCKS != 0 { debug_show_all_locks(); }
    if si_mask & SYS_INFO_FTRACE != 0 { ftrace_dump(0); }
    if si_mask & SYS_INFO_ALL_BT != 0 { trigger_all_cpu_backtrace(); }
    if si_mask & SYS_INFO_BLOCKED_TASKS != 0 { show_state_filter(2); }
}

#[no_mangle]
pub unsafe extern "C" fn sys_info(si_mask: c_ulong) {
    __sys_info(if si_mask != 0 { si_mask } else { KERNEL_SI_MASK });
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
