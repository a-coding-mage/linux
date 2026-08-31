// SPDX-License-Identifier: GPL-2.0
/* Display a menu with individual samples to browse with perf script */

use core::ffi::{c_char, c_int, c_void};

const NSEC_PER_MSEC: u64 = 1_000_000;

static mut context_len: u64 = 10 * NSEC_PER_MSEC;

#[repr(C)]
pub struct perf_event_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel_core {
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct res_sample {
    pub time: u64,
    pub cpu: c_int,
    pub tid: c_int,
}

pub type rstype = c_int;

/* enum rstype values are supplied by the original headers. */
pub const A_ASM: rstype = 0;
pub const A_SOURCE: rstype = 1;

#[repr(C)]
pub struct symbol_conf_t {
    pub inline_name: bool,
}

unsafe extern "C" {
    static mut input_name: *const c_char;
    static symbol_conf: symbol_conf_t;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn sprintf(str: *mut c_char, fmt: *const c_char, ...) -> c_int;

    fn perf_config_u64(var: *mut u64, name: *const c_char, value: *const c_char) -> c_int;
    fn perf_config(
        fn_: Option<unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;
    fn perf_exe(buf: *mut c_char, size: usize) -> *const c_char;
    fn timestamp__scnprintf_nsec(timestamp: u64, buf: *mut c_char, size: usize) -> c_int;
    fn attr_to_script(extra_format: *mut c_char, attr: *const perf_event_attr);
    fn ui__popup_menu(argc: c_int, argv: *mut *mut c_char, help: *mut c_void) -> c_int;
    fn zfree(ptr: *mut *mut c_char);
    fn run_script(cmd: *mut c_char);
}

unsafe extern "C" fn res_sample_config(
    var: *const c_char,
    value: *const c_char,
    _data: *mut c_void,
) -> c_int {
    if strcmp(var, c"samples.context".as_ptr()) == 0 {
        return perf_config_u64(&raw mut context_len, var, value);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn res_sample_init() {
    perf_config(Some(res_sample_config), core::ptr::null_mut());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn res_sample_browse(
    res_samples: *mut res_sample,
    num_res: c_int,
    evsel: *mut evsel,
    rstype: rstype,
) -> c_int {
    let mut names: *mut *mut c_char;
    let mut i: c_int;
    let mut n: c_int;
    let choice: c_int;
    let mut cmd: *mut c_char = core::ptr::null_mut();
    let mut pbuf = [0 as c_char; 256];
    let mut tidbuf = [0 as c_char; 32];
    let mut cpubuf = [0 as c_char; 32];
    let perf = perf_exe(pbuf.as_mut_ptr(), core::mem::size_of_val(&pbuf));
    let mut trange = [0 as c_char; 128];
    let mut tsample = [0 as c_char; 64];
    let r: *mut res_sample;
    let mut extra_format = [0 as c_char; 256];

    names = calloc(num_res as usize, core::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
    if names.is_null() {
        return -1;
    }
    i = 0;
    while i < num_res {
        let mut tbuf = [0 as c_char; 64];

        timestamp__scnprintf_nsec(
            (*res_samples.add(i as usize)).time,
            tbuf.as_mut_ptr(),
            core::mem::size_of_val(&tbuf),
        );
        if asprintf(
            names.add(i as usize),
            c"%s: CPU %d tid %d".as_ptr(),
            tbuf.as_ptr(),
            (*res_samples.add(i as usize)).cpu,
            (*res_samples.add(i as usize)).tid,
        ) < 0
        {
            i -= 1;
            while i >= 0 {
                zfree(names.add(i as usize));
                i -= 1;
            }
            free(names as *mut c_void);
            return -1;
        }
        i += 1;
    }
    choice = ui__popup_menu(num_res, names, core::ptr::null_mut());
    i = 0;
    while i < num_res {
        zfree(names.add(i as usize));
        i += 1;
    }
    free(names as *mut c_void);

    if choice < 0 || choice >= num_res {
        return -1;
    }
    r = res_samples.add(choice as usize);

    n = timestamp__scnprintf_nsec(
        (*r).time.wrapping_sub(context_len),
        trange.as_mut_ptr(),
        core::mem::size_of_val(&trange),
    );
    trange[n as usize] = b',' as c_char;
    n += 1;
    timestamp__scnprintf_nsec(
        (*r).time.wrapping_add(context_len),
        trange.as_mut_ptr().add(n as usize),
        core::mem::size_of_val(&trange) - n as usize,
    );

    timestamp__scnprintf_nsec(
        (*r).time,
        tsample.as_mut_ptr(),
        core::mem::size_of_val(&tsample),
    );

    attr_to_script(extra_format.as_mut_ptr(), &(*evsel).core.attr);

    if asprintf(
        &mut cmd,
        c"%s script %s%s --time %s %s%s %s%s --ns %s %s %s %s %s | less +/%s".as_ptr(),
        perf,
        if !input_name.is_null() {
            c"-i ".as_ptr()
        } else {
            c"".as_ptr()
        },
        if !input_name.is_null() {
            input_name
        } else {
            c"".as_ptr()
        },
        trange.as_ptr(),
        if (*r).cpu >= 0 {
            c"--cpu ".as_ptr()
        } else {
            c"".as_ptr()
        },
        if (*r).cpu >= 0 {
            sprintf(cpubuf.as_mut_ptr(), c"%d".as_ptr(), (*r).cpu);
            cpubuf.as_ptr()
        } else {
            c"".as_ptr()
        },
        if (*r).tid != 0 {
            c"--tid ".as_ptr()
        } else {
            c"".as_ptr()
        },
        if (*r).tid != 0 {
            sprintf(tidbuf.as_mut_ptr(), c"%d".as_ptr(), (*r).tid);
            tidbuf.as_ptr()
        } else {
            c"".as_ptr()
        },
        extra_format.as_ptr(),
        if rstype == A_ASM {
            c"-F +disasm".as_ptr()
        } else if rstype == A_SOURCE {
            c"-F +srcline,+srccode".as_ptr()
        } else {
            c"".as_ptr()
        },
        if symbol_conf.inline_name {
            c"--inline".as_ptr()
        } else {
            c"".as_ptr()
        },
        c"--show-lost-events ".as_ptr(),
        if (*r).tid != 0 {
            c"--show-switch-events --show-task-events ".as_ptr()
        } else {
            c"".as_ptr()
        },
        tsample.as_ptr(),
    ) < 0
    {
        return -1;
    }
    run_script(cmd);
    free(cmd as *mut c_void);
    0
}
