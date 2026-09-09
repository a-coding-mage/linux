// SPDX-License-Identifier: GPL-2.0-only
/*
 * Here's a sample kernel module showing the use of fprobe to dump a
 * stack trace and selected registers when kernel_clone() is called.
 *
 * For more information on theory of operation of kprobes, see
 * Documentation/trace/kprobes.rst
 *
 * You will see the trace data in /var/log/messages and on the console
 * whenever kernel_clone() is invoked to create a new process.
 */

// Dependencies supplied by the kernel headers are intentionally external.

const BACKTRACE_DEPTH: usize = 16;
const MAX_SYMBOL_LEN: usize = 4096;

#[repr(C)]
pub struct ftrace_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fprobe {
    pub entry_handler: Option<unsafe extern "C" fn(*mut fprobe, usize, usize, *mut ftrace_regs, *mut core::ffi::c_void) -> i32>,
    pub exit_handler: Option<unsafe extern "C" fn(*mut fprobe, usize, usize, *mut ftrace_regs, *mut core::ffi::c_void)>,
    pub nmissed: usize,
}

unsafe extern "C" {
    fn stack_trace_save(entries: *mut usize, size: usize, skipnr: usize) -> u32;
    fn stack_trace_print(entries: *const usize, nr_entries: u32, spaces: usize);
    fn trace_printk(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn strchr(s: *mut u8, c: i32) -> *mut u8;
    fn kstrdup(s: *const u8, flags: u32) -> *mut u8;
    fn kcalloc(n: usize, size: usize, flags: u32) -> *mut *const u8;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn strsep(stringp: *mut *mut u8, delim: *const u8) -> *mut u8;
    fn register_fprobe(fp: *mut fprobe, symbol: *mut u8, nosymbol: *mut u8) -> i32;
    fn register_fprobe_syms(fp: *mut fprobe, syms: *const *const u8, count: i32) -> i32;
    fn unregister_fprobe(fp: *mut fprobe);
}

const GFP_KERNEL: u32 = 0;

static mut sample_probe: fprobe = fprobe {
    entry_handler: None,
    exit_handler: None,
    nmissed: 0,
};
static mut nhit: usize = 0;

static mut symbol: [u8; MAX_SYMBOL_LEN] = {
    let mut value = [0u8; MAX_SYMBOL_LEN];
    value[0] = b'k';
    value[1] = b'e';
    value[2] = b'r';
    value[3] = b'n';
    value[4] = b'e';
    value[5] = b'l';
    value[6] = b'_' ;
    value[7] = b'c';
    value[8] = b'l';
    value[9] = b'o';
    value[10] = b'n';
    value[11] = b'e';
    value
};
// module_param_string(symbol, symbol, sizeof(symbol), 0644);
// MODULE_PARM_DESC(symbol, "Probed symbol(s), given by comma separated symbols or a wildcard pattern.");

static mut nosymbol: [u8; MAX_SYMBOL_LEN] = [0; MAX_SYMBOL_LEN];
// module_param_string(nosymbol, nosymbol, sizeof(nosymbol), 0644);
// MODULE_PARM_DESC(nosymbol, "Not-probed symbols, given by a wildcard pattern.");

static mut stackdump: bool = true;
// module_param(stackdump, bool, 0644);
// MODULE_PARM_DESC(stackdump, "Enable stackdump.");

static mut use_trace: bool = false;
// module_param(use_trace, bool, 0644);
// MODULE_PARM_DESC(use_trace, "Use trace_printk instead of printk. This is only for debugging.");

unsafe fn show_backtrace() {
    let mut stacks = [0usize; BACKTRACE_DEPTH];
    let len = stack_trace_save(stacks.as_mut_ptr(), BACKTRACE_DEPTH, 2);
    stack_trace_print(stacks.as_ptr(), len, 24);
}

unsafe extern "C" fn sample_entry_handler(
    _fp: *mut fprobe,
    ip: usize,
    _ret_ip: usize,
    _fregs: *mut ftrace_regs,
    _data: *mut core::ffi::c_void,
) -> i32 {
    if use_trace {
        /* This is just an example, no kernel code should call
         * trace_printk() except when actively debugging. */
        trace_printk(b"Enter <%pS> ip = 0x%p\n\0".as_ptr(), ip as *mut core::ffi::c_void, ip as *mut core::ffi::c_void);
    } else {
        pr_info(b"Enter <%pS> ip = 0x%p\n\0".as_ptr(), ip as *mut core::ffi::c_void, ip as *mut core::ffi::c_void);
    }
    nhit += 1;
    if stackdump { show_backtrace(); }
    0
}

unsafe extern "C" fn sample_exit_handler(
    _fp: *mut fprobe, ip: usize, ret_ip: usize, _regs: *mut ftrace_regs,
    _data: *mut core::ffi::c_void,
) {
    let rip = ret_ip;
    if use_trace {
        /* This is just an example, no kernel code should call
         * trace_printk() except when actively debugging. */
        trace_printk(b"Return from <%pS> ip = 0x%p to rip = 0x%p (%pS)\n\0".as_ptr(), ip as *mut _, ip as *mut _, rip as *mut _, rip as *mut _);
    } else {
        pr_info(b"Return from <%pS> ip = 0x%p to rip = 0x%p (%pS)\n\0".as_ptr(), ip as *mut _, ip as *mut _, rip as *mut _, rip as *mut _);
    }
    nhit += 1;
    if stackdump { show_backtrace(); }
}

unsafe extern "C" fn fprobe_init() -> i32 {
    sample_probe.entry_handler = Some(sample_entry_handler);
    sample_probe.exit_handler = Some(sample_exit_handler);
    let star = strchr(symbol.as_mut_ptr(), b'*' as i32);
    let ret;
    if !star.is_null() {
        ret = register_fprobe(&raw mut sample_probe, symbol.as_mut_ptr(), if nosymbol[0] == 0 { core::ptr::null_mut() } else { nosymbol.as_mut_ptr() });
    } else if strchr(symbol.as_mut_ptr(), b',' as i32).is_null() {
        let mut symbuf = symbol.as_mut_ptr();
        ret = register_fprobe_syms(&raw mut sample_probe, &symbuf as *const *mut u8 as *const *const u8, 1);
    } else {
        let mut symbuf = kstrdup(symbol.as_ptr(), GFP_KERNEL);
        if symbuf.is_null() { return -12; }
        let mut p = symbuf;
        let mut count = 1;
        while !strchr({ p = p.add(1); p }, b',' as i32).is_null() { count += 1; }
        pr_info(b"%d symbols found\n\0".as_ptr(), count);
        let syms = kcalloc(count as usize, core::mem::size_of::<*const u8>(), GFP_KERNEL);
        if syms.is_null() { kfree(symbuf as *mut _); return -12; }
        p = symbuf;
        for i in 0..count as usize { *syms.add(i) = strsep(&mut p, b",\0".as_ptr()); }
        ret = register_fprobe_syms(&raw mut sample_probe, syms, count);
        kfree(syms as *mut _); kfree(symbuf as *mut _);
    }
    if ret < 0 { pr_err(b"register_fprobe failed, returned %d\n\0".as_ptr(), ret); }
    else { pr_info(b"Planted fprobe at %s\n\0".as_ptr(), symbol.as_ptr()); }
    ret
}

unsafe extern "C" fn fprobe_exit() {
    unregister_fprobe(&raw mut sample_probe);
    pr_info(b"fprobe at %s unregistered. %ld times hit, %ld times missed\n\0".as_ptr(), symbol.as_ptr(), nhit, sample_probe.nmissed);
}

// module_init(fprobe_init)
// module_exit(fprobe_exit)
// MODULE_DESCRIPTION("sample kernel module showing the use of fprobe");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
