// SPDX-License-Identifier: LGPL-2.1
// C dependencies: string.h, trace/beauty/beauty.h, util/evsel_fprintf.h,
// linux/perf_event.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const PERF_FLAG_FD_NO_GROUP: c_ulong = 1u64 as c_ulong;
const PERF_FLAG_FD_OUTPUT: c_ulong = (1u64 << 1) as c_ulong;
const PERF_FLAG_PID_CGROUP: c_ulong = (1u64 << 2) as c_ulong; /* pid=cgroup id, per-cpu mode only */
const PERF_FLAG_FD_CLOEXEC: c_ulong = (1u64 << 3) as c_ulong; /* O_CLOEXEC */

const PERF_ATTR_SIZE_VER0: usize = 64;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub show_string_prefix: bool,
    pub augmented: syscall_arg_augmented,
    pub trace: *mut trace,
}

#[repr(C)]
pub struct syscall_arg_augmented {
    pub args: *mut augmented_args,
}

#[repr(C)]
pub struct augmented_args {
    pub value: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub size: c_uint,
}

#[repr(C)]
struct attr_fprintf_args {
    size: usize,
    printed: usize,
    bf: *mut c_char,
    first: bool,
}

type AttrFprintf = unsafe extern "C" fn(
    fp: *mut FILE,
    name: *const c_char,
    val: *const c_char,
    priv_: *mut c_void,
) -> c_int;

unsafe extern "C" {
    static mut stdout: *mut FILE;

    fn scnprintf(bf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn perf_event_attr__fprintf(
        fp: *mut FILE,
        attr: *mut perf_event_attr,
        fprintf: AttrFprintf,
        priv_: *mut c_void,
    );
    fn trace__show_zeros(trace: *mut trace) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_perf_flags(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let show_prefix = unsafe { (*arg).show_string_prefix };
    let prefix = c"PERF_".as_ptr();
    let mut printed: c_int = 0;
    let mut flags: c_int = unsafe { (*arg).val as c_int };

    if flags == 0 {
        return 0;
    }

    if flags & PERF_FLAG_FD_NO_GROUP as c_int != 0 {
        printed += unsafe {
            scnprintf(
                bf.add(printed as usize),
                size.wrapping_sub(printed as usize),
                c"%s%s%s".as_ptr(),
                if printed != 0 {
                    c"|".as_ptr()
                } else {
                    c"".as_ptr()
                },
                if show_prefix { prefix } else { c"".as_ptr() },
                c"FD_NO_GROUP".as_ptr(),
            )
        };
        flags &= !(PERF_FLAG_FD_NO_GROUP as c_int);
    }
    if flags & PERF_FLAG_FD_OUTPUT as c_int != 0 {
        printed += unsafe {
            scnprintf(
                bf.add(printed as usize),
                size.wrapping_sub(printed as usize),
                c"%s%s%s".as_ptr(),
                if printed != 0 {
                    c"|".as_ptr()
                } else {
                    c"".as_ptr()
                },
                if show_prefix { prefix } else { c"".as_ptr() },
                c"FD_OUTPUT".as_ptr(),
            )
        };
        flags &= !(PERF_FLAG_FD_OUTPUT as c_int);
    }
    if flags & PERF_FLAG_PID_CGROUP as c_int != 0 {
        printed += unsafe {
            scnprintf(
                bf.add(printed as usize),
                size.wrapping_sub(printed as usize),
                c"%s%s%s".as_ptr(),
                if printed != 0 {
                    c"|".as_ptr()
                } else {
                    c"".as_ptr()
                },
                if show_prefix { prefix } else { c"".as_ptr() },
                c"PID_CGROUP".as_ptr(),
            )
        };
        flags &= !(PERF_FLAG_PID_CGROUP as c_int);
    }
    if flags & PERF_FLAG_FD_CLOEXEC as c_int != 0 {
        printed += unsafe {
            scnprintf(
                bf.add(printed as usize),
                size.wrapping_sub(printed as usize),
                c"%s%s%s".as_ptr(),
                if printed != 0 {
                    c"|".as_ptr()
                } else {
                    c"".as_ptr()
                },
                if show_prefix { prefix } else { c"".as_ptr() },
                c"FD_CLOEXEC".as_ptr(),
            )
        };
        flags &= !(PERF_FLAG_FD_CLOEXEC as c_int);
    }

    if flags != 0 {
        printed += unsafe {
            scnprintf(
                bf.add(printed as usize),
                size.wrapping_sub(printed as usize),
                c"%s%#x".as_ptr(),
                if printed != 0 {
                    c"|".as_ptr()
                } else {
                    c"".as_ptr()
                },
                flags,
            )
        };
    }

    printed as usize
}

unsafe extern "C" fn attr__fprintf(
    _fp: *mut FILE,
    name: *const c_char,
    val: *const c_char,
    priv_: *mut c_void,
) -> c_int {
    let args = priv_ as *mut attr_fprintf_args;
    let printed = unsafe {
        scnprintf(
            (*args).bf.add((*args).printed),
            (*args).size.wrapping_sub((*args).printed),
            c"%s%s: %s".as_ptr(),
            if (*args).first {
                c"".as_ptr()
            } else {
                c", ".as_ptr()
            },
            name,
            val,
        )
    };

    unsafe {
        (*args).first = false;
        (*args).printed = (*args).printed.wrapping_add(printed as usize);
    }
    printed
}

unsafe fn perf_event_attr___scnprintf(
    attr: *mut perf_event_attr,
    bf: *mut c_char,
    size: usize,
    _show_zeros: bool,
) -> usize {
    let mut args = attr_fprintf_args {
        printed: unsafe { scnprintf(bf, size, c"{ ".as_ptr()) as usize },
        size,
        first: true,
        bf,
    };

    unsafe {
        perf_event_attr__fprintf(
            stdout,
            attr,
            attr__fprintf,
            &mut args as *mut attr_fprintf_args as *mut c_void,
        );
    }
    args.printed.wrapping_add(unsafe {
        scnprintf(
            bf.add(args.printed),
            size.wrapping_sub(args.printed),
            c" }".as_ptr(),
        ) as usize
    })
}

unsafe fn syscall_arg__scnprintf_augmented_perf_event_attr(
    arg: *mut syscall_arg,
    bf: *mut c_char,
    size: usize,
) -> usize {
    let mut attr = unsafe { (*(*arg).augmented.args).value.as_mut_ptr() as *mut perf_event_attr };
    let mut local_attr: perf_event_attr = unsafe { core::mem::zeroed() };

    /*
     * augmented_raw_syscalls.bpf.c (shipped with perf) copies
     * PERF_ATTR_SIZE_VER0 bytes when the tracee passes size=0,
     * but leaves the size field as 0.  The payload size is
     * guaranteed by perf's own BPF program, not externally
     * controllable.  Copy to a local so we can fix up size
     * without writing to the potentially read-only augmented
     * args buffer.
     */
    if unsafe { (*attr).size } == 0 {
        unsafe {
            memcpy(
                &mut local_attr as *mut perf_event_attr as *mut c_void,
                attr as *const c_void,
                PERF_ATTR_SIZE_VER0,
            );
            memset(
                (&mut local_attr as *mut perf_event_attr as *mut u8).add(PERF_ATTR_SIZE_VER0)
                    as *mut c_void,
                0,
                core::mem::size_of_val(&local_attr).wrapping_sub(PERF_ATTR_SIZE_VER0),
            );
        }
        local_attr.size = PERF_ATTR_SIZE_VER0 as c_uint;
        attr = &mut local_attr;
    }

    unsafe { perf_event_attr___scnprintf(attr, bf, size, trace__show_zeros((*arg).trace)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_perf_event_attr(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    if unsafe { !(*arg).augmented.args.is_null() } {
        return unsafe { syscall_arg__scnprintf_augmented_perf_event_attr(arg, bf, size) };
    }

    unsafe { scnprintf(bf, size, c"%#lx".as_ptr(), (*arg).val) as usize }
}
