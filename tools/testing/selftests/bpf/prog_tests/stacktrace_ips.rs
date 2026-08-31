// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, "stacktrace_ips.skel.h"

#[cfg(target_arch = "x86_64")]
use core::ffi::{c_char, c_int, c_ulong, c_void};

#[cfg(target_arch = "x86_64")]
type __u32 = u32;
#[cfg(target_arch = "x86_64")]
type __u64 = u64;

#[cfg(target_arch = "x86_64")]
const PERF_MAX_STACK_DEPTH: usize = 127;

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct ksym {
    addr: c_ulong,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct ksyms {
    _private: [u8; 0],
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct bpf_kprobe_multi_opts {
    retprobe: bool,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct bpf_kprobe_opts {
    retprobe: bool,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct bpf_test_run_opts {
    _private: [u8; 0],
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct bpf_prog_info {
    jited_ksyms: __u64,
    nr_jited_ksyms: __u32,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct stacktrace_ips {
    kconfig: *mut stacktrace_ips_kconfig,
    links: stacktrace_ips_links,
    progs: stacktrace_ips_progs,
    maps: stacktrace_ips_maps,
    bss: *mut stacktrace_ips_bss,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct stacktrace_ips_kconfig {
    CONFIG_UNWINDER_ORC: bool,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct stacktrace_ips_links {
    kprobe_multi_test: *mut c_void,
    rawtp_test: *mut c_void,
    kprobe_test: *mut c_void,
    fexit_test: *mut c_void,
    fentry_test: *mut c_void,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct stacktrace_ips_progs {
    kprobe_multi_test: *mut c_void,
    rawtp_test: *mut c_void,
    kprobe_test: *mut c_void,
    fexit_test: *mut c_void,
    fentry_test: *mut c_void,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct stacktrace_ips_maps {
    stackmap: *mut c_void,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
struct stacktrace_ips_bss {
    stack_key: __u32,
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" {
    fn load_kallsyms_local() -> *mut ksyms;
    fn free_kallsyms_local(ksyms: *mut ksyms);
    fn ksym_search_local(ksyms: *mut ksyms, addr: __u64) -> *mut ksym;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn stacktrace_ips__open_and_load() -> *mut stacktrace_ips;
    fn stacktrace_ips__destroy(skel: *mut stacktrace_ips);
    fn bpf_program__attach_kprobe_multi_opts(
        prog: *mut c_void,
        name: *const c_char,
        opts: *const bpf_kprobe_multi_opts,
    ) -> *mut c_void;
    fn bpf_program__attach_raw_tracepoint(prog: *mut c_void, name: *const c_char) -> *mut c_void;
    fn bpf_program__attach_kprobe_opts(
        prog: *mut c_void,
        name: *const c_char,
        opts: *const bpf_kprobe_opts,
    ) -> *mut c_void;
    fn bpf_program__attach_trace(prog: *mut c_void) -> *mut c_void;
    fn trigger_module_test_read(arg: c_int);
    fn load_kallsyms();
    fn ksym_get_addr(name: *const c_char) -> c_ulong;
    fn bpf_map__fd(map: *mut c_void) -> c_int;
    fn bpf_program__fd(prog: *mut c_void) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_LT(cnt: c_int, max: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: c_ulong, right: c_ulong, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
}

#[cfg(target_arch = "x86_64")]
fn ptr_to_u64<T>(ptr: *const T) -> __u64 {
    ptr as __u64
}

#[cfg(target_arch = "x86_64")]
unsafe fn check_stacktrace_ips(fd: c_int, key: __u32, cnt: c_int, vals: &[c_ulong]) -> c_int {
    let mut ips = [0 as __u64; PERF_MAX_STACK_DEPTH];
    let mut ksyms: *mut ksyms = core::ptr::null_mut();
    let mut err: c_int = 0;

    /* sorted by addr */
    ksyms = unsafe { load_kallsyms_local() };
    if !unsafe { ASSERT_OK_PTR(ksyms as *mut c_void, c"load_kallsyms_local".as_ptr()) } {
        return -1;
    }

    /* unlikely, but... */
    if !unsafe { ASSERT_LT(cnt, PERF_MAX_STACK_DEPTH as c_int, c"check_max".as_ptr()) } {
        return -1;
    }

    err = unsafe {
        bpf_map_lookup_elem(
            fd,
            &key as *const __u32 as *const c_void,
            ips.as_mut_ptr() as *mut c_void,
        )
    };
    if err != 0 {
        unsafe { free_kallsyms_local(ksyms) };
        return err;
    }

    /*
     * Compare all symbols provided via arguments with stacktrace ips,
     * and their related symbol addresses.t
     */
    for i in 0..cnt {
        let val: c_ulong = vals[i as usize];
        let ksym = unsafe { ksym_search_local(ksyms, ips[i as usize]) };
        if !unsafe { ASSERT_OK_PTR(ksym as *mut c_void, c"ksym_search_local".as_ptr()) } {
            break;
        }
        unsafe {
            ASSERT_EQ((*ksym).addr, val, c"stack_cmp".as_ptr());
        }
    }

    unsafe { free_kallsyms_local(ksyms) };
    err
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_stacktrace_ips_kprobe_multi(retprobe: bool) {
    let opts = bpf_kprobe_multi_opts { retprobe };
    let _topts = bpf_test_run_opts { _private: [] };
    let skel: *mut stacktrace_ips;

    skel = unsafe { stacktrace_ips__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel as *mut c_void, c"stacktrace_ips__open_and_load".as_ptr()) } {
        return;
    }

    if !unsafe { (*(*skel).kconfig).CONFIG_UNWINDER_ORC } {
        unsafe { test__skip() };
        unsafe { stacktrace_ips__destroy(skel) };
        return;
    }

    unsafe {
        (*skel).links.kprobe_multi_test = bpf_program__attach_kprobe_multi_opts(
            (*skel).progs.kprobe_multi_test,
            c"bpf_testmod_stacktrace_test".as_ptr(),
            &opts,
        );
    }
    if !unsafe {
        ASSERT_OK_PTR(
            (*skel).links.kprobe_multi_test,
            c"bpf_program__attach_kprobe_multi_opts".as_ptr(),
        )
    } {
        unsafe { stacktrace_ips__destroy(skel) };
        return;
    }

    unsafe { trigger_module_test_read(1) };

    unsafe { load_kallsyms() };

    if retprobe {
        unsafe {
            check_stacktrace_ips(
                bpf_map__fd((*skel).maps.stackmap),
                (*(*skel).bss).stack_key,
                4,
                &[
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_3".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_2".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_1".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_test_read".as_ptr()),
                ],
            );
        }
    } else {
        unsafe {
            check_stacktrace_ips(
                bpf_map__fd((*skel).maps.stackmap),
                (*(*skel).bss).stack_key,
                5,
                &[
                    ksym_get_addr(c"bpf_testmod_stacktrace_test".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_3".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_2".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_1".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_test_read".as_ptr()),
                ],
            );
        }
    }

    unsafe { stacktrace_ips__destroy(skel) };
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_stacktrace_ips_raw_tp() {
    let mut info_len: __u32 = core::mem::size_of::<bpf_prog_info>() as __u32;
    let _topts = bpf_test_run_opts { _private: [] };
    let mut info = bpf_prog_info {
        jited_ksyms: 0,
        nr_jited_ksyms: 0,
    };
    let skel: *mut stacktrace_ips;
    let mut bpf_prog_ksym: __u64 = 0;
    let err: c_int;

    skel = unsafe { stacktrace_ips__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel as *mut c_void, c"stacktrace_ips__open_and_load".as_ptr()) } {
        return;
    }

    if !unsafe { (*(*skel).kconfig).CONFIG_UNWINDER_ORC } {
        unsafe { test__skip() };
        unsafe { stacktrace_ips__destroy(skel) };
        return;
    }

    unsafe {
        (*skel).links.rawtp_test = bpf_program__attach_raw_tracepoint(
            (*skel).progs.rawtp_test,
            c"bpf_testmod_test_read".as_ptr(),
        );
    }
    if !unsafe {
        ASSERT_OK_PTR(
            (*skel).links.rawtp_test,
            c"bpf_program__attach_raw_tracepoint".as_ptr(),
        )
    } {
        unsafe { stacktrace_ips__destroy(skel) };
        return;
    }

    /* get bpf program address */
    info.jited_ksyms = ptr_to_u64(&bpf_prog_ksym);
    info.nr_jited_ksyms = 1;
    err = unsafe {
        bpf_prog_get_info_by_fd(
            bpf_program__fd((*skel).progs.rawtp_test),
            &mut info,
            &mut info_len,
        )
    };
    if !unsafe { ASSERT_OK(err, c"bpf_prog_get_info_by_fd".as_ptr()) } {
        unsafe { stacktrace_ips__destroy(skel) };
        return;
    }

    unsafe { trigger_module_test_read(1) };

    unsafe { load_kallsyms() };

    unsafe {
        check_stacktrace_ips(
            bpf_map__fd((*skel).maps.stackmap),
            (*(*skel).bss).stack_key,
            2,
            &[
                bpf_prog_ksym as c_ulong,
                ksym_get_addr(c"bpf_trace_run2".as_ptr()),
            ],
        );
    }

    unsafe { stacktrace_ips__destroy(skel) };
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_stacktrace_ips_kprobe(retprobe: bool) {
    let opts = bpf_kprobe_opts { retprobe };
    let _topts = bpf_test_run_opts { _private: [] };
    let skel: *mut stacktrace_ips;

    skel = unsafe { stacktrace_ips__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel as *mut c_void, c"stacktrace_ips__open_and_load".as_ptr()) } {
        return;
    }

    if !unsafe { (*(*skel).kconfig).CONFIG_UNWINDER_ORC } {
        unsafe { test__skip() };
        unsafe { stacktrace_ips__destroy(skel) };
        return;
    }

    unsafe {
        (*skel).links.kprobe_test = bpf_program__attach_kprobe_opts(
            (*skel).progs.kprobe_test,
            c"bpf_testmod_stacktrace_test".as_ptr(),
            &opts,
        );
    }
    if !unsafe {
        ASSERT_OK_PTR(
            (*skel).links.kprobe_test,
            c"bpf_program__attach_kprobe_opts".as_ptr(),
        )
    } {
        unsafe { stacktrace_ips__destroy(skel) };
        return;
    }

    unsafe { trigger_module_test_read(1) };

    unsafe { load_kallsyms() };

    if retprobe {
        unsafe {
            check_stacktrace_ips(
                bpf_map__fd((*skel).maps.stackmap),
                (*(*skel).bss).stack_key,
                4,
                &[
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_3".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_2".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_1".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_test_read".as_ptr()),
                ],
            );
        }
    } else {
        unsafe {
            check_stacktrace_ips(
                bpf_map__fd((*skel).maps.stackmap),
                (*(*skel).bss).stack_key,
                5,
                &[
                    ksym_get_addr(c"bpf_testmod_stacktrace_test".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_3".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_2".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_1".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_test_read".as_ptr()),
                ],
            );
        }
    }

    unsafe { stacktrace_ips__destroy(skel) };
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_stacktrace_ips_trampoline(retprobe: bool) {
    let _topts = bpf_test_run_opts { _private: [] };
    let skel: *mut stacktrace_ips;

    skel = unsafe { stacktrace_ips__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel as *mut c_void, c"stacktrace_ips__open_and_load".as_ptr()) } {
        return;
    }

    if !unsafe { (*(*skel).kconfig).CONFIG_UNWINDER_ORC } {
        unsafe { test__skip() };
        unsafe { stacktrace_ips__destroy(skel) };
        return;
    }

    if retprobe {
        unsafe {
            (*skel).links.fexit_test = bpf_program__attach_trace((*skel).progs.fexit_test);
        }
        if !unsafe {
            ASSERT_OK_PTR(
                (*skel).links.fexit_test,
                c"bpf_program__attach_trace".as_ptr(),
            )
        } {
            unsafe { stacktrace_ips__destroy(skel) };
            return;
        }
    } else {
        unsafe {
            (*skel).links.fentry_test = bpf_program__attach_trace((*skel).progs.fentry_test);
        }
        if !unsafe {
            ASSERT_OK_PTR(
                (*skel).links.fentry_test,
                c"bpf_program__attach_trace".as_ptr(),
            )
        } {
            unsafe { stacktrace_ips__destroy(skel) };
            return;
        }
    }

    unsafe { trigger_module_test_read(1) };

    unsafe { load_kallsyms() };

    if retprobe {
        unsafe {
            check_stacktrace_ips(
                bpf_map__fd((*skel).maps.stackmap),
                (*(*skel).bss).stack_key,
                4,
                &[
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_3".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_2".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_1".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_test_read".as_ptr()),
                ],
            );
        }
    } else {
        unsafe {
            check_stacktrace_ips(
                bpf_map__fd((*skel).maps.stackmap),
                (*(*skel).bss).stack_key,
                5,
                &[
                    ksym_get_addr(c"bpf_testmod_stacktrace_test".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_3".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_2".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_stacktrace_test_1".as_ptr()),
                    ksym_get_addr(c"bpf_testmod_test_read".as_ptr()),
                ],
            );
        }
    }

    unsafe { stacktrace_ips__destroy(skel) };
}

#[cfg(target_arch = "x86_64")]
unsafe fn __test_stacktrace_ips() {
    if unsafe { test__start_subtest(c"kprobe_multi".as_ptr()) } {
        unsafe { test_stacktrace_ips_kprobe_multi(false) };
    }
    if unsafe { test__start_subtest(c"kretprobe_multi".as_ptr()) } {
        unsafe { test_stacktrace_ips_kprobe_multi(true) };
    }
    if unsafe { test__start_subtest(c"raw_tp".as_ptr()) } {
        unsafe { test_stacktrace_ips_raw_tp() };
    }
    if unsafe { test__start_subtest(c"kprobe".as_ptr()) } {
        unsafe { test_stacktrace_ips_kprobe(false) };
    }
    if unsafe { test__start_subtest(c"kretprobe".as_ptr()) } {
        unsafe { test_stacktrace_ips_kprobe(true) };
    }
    if unsafe { test__start_subtest(c"fentry".as_ptr()) } {
        unsafe { test_stacktrace_ips_trampoline(false) };
    }
    if unsafe { test__start_subtest(c"fexit".as_ptr()) } {
        unsafe { test_stacktrace_ips_trampoline(true) };
    }
}

#[cfg(not(target_arch = "x86_64"))]
unsafe extern "C" {
    fn test__skip();
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn __test_stacktrace_ips() {
    unsafe { test__skip() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_stacktrace_ips() {
    unsafe { __test_stacktrace_ips() };
}
