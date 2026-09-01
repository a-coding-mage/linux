// SPDX-License-Identifier: GPL-2.0
//
// C dependencies from:
// <test_progs.h>
// "kprobe_multi.skel.h"
// "trace_helpers.h"
// "bpf/libbpf_internal.h"

use std::ffi::{c_char, c_int, c_ulong};
use std::ptr;

#[repr(C)]
pub struct ksyms {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_kprobe_multi_opts {
    pub sz: usize,
    pub syms: *const *const c_char,
    pub addrs: *const c_ulong,
    pub cnt: usize,
    pub retprobe: bool,
}

impl Default for bpf_kprobe_multi_opts {
    fn default() -> Self {
        Self {
            sz: std::mem::size_of::<bpf_kprobe_multi_opts>(),
            syms: ptr::null(),
            addrs: ptr::null(),
            cnt: 0,
            retprobe: false,
        }
    }
}

#[repr(C)]
pub struct kprobe_multi_bss {
    pub kprobe_testmod_test1_result: c_int,
    pub kprobe_testmod_test2_result: c_int,
    pub kprobe_testmod_test3_result: c_int,
    pub kretprobe_testmod_test1_result: c_int,
    pub kretprobe_testmod_test2_result: c_int,
    pub kretprobe_testmod_test3_result: c_int,
    pub pid: c_int,
}

#[repr(C)]
pub struct kprobe_multi_links {
    pub test_kprobe_testmod: *mut bpf_link,
    pub test_kretprobe_testmod: *mut bpf_link,
}

#[repr(C)]
pub struct kprobe_multi_progs {
    pub test_kprobe_testmod: *mut bpf_program,
    pub test_kretprobe_testmod: *mut bpf_program,
}

#[repr(C)]
pub struct kprobe_multi {
    pub bss: *mut kprobe_multi_bss,
    pub links: kprobe_multi_links,
    pub progs: kprobe_multi_progs,
}

unsafe extern "C" {
    static mut ksyms: *mut ksyms;

    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: u64, expected: u64, name: *const c_char) -> bool;
    fn ASSERT_OK(actual: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const std::ffi::c_void, name: *const c_char) -> bool;

    fn getpid() -> c_int;
    fn kprobe_multi__open_and_load() -> *mut kprobe_multi;
    fn kprobe_multi__destroy(skel: *mut kprobe_multi);
    fn bpf_program__attach_kprobe_multi_opts(
        prog: *mut bpf_program,
        pattern: *const c_char,
        opts: *mut bpf_kprobe_multi_opts,
    ) -> *mut bpf_link;
    fn trigger_module_test_read(arg: c_int) -> c_int;
    fn ksym_get_addr_local(ksyms: *mut ksyms, name: *const c_char) -> u64;
    fn load_kallsyms_local() -> *mut ksyms;
    fn free_kallsyms_local(ksyms: *mut ksyms);
    fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn kprobe_multi_testmod_check(skel: *mut kprobe_multi) {
    unsafe {
        ASSERT_EQ(
            (*(*skel).bss).kprobe_testmod_test1_result,
            1,
            c"kprobe_test1_result".as_ptr(),
        );
        ASSERT_EQ(
            (*(*skel).bss).kprobe_testmod_test2_result,
            1,
            c"kprobe_test2_result".as_ptr(),
        );
        ASSERT_EQ(
            (*(*skel).bss).kprobe_testmod_test3_result,
            1,
            c"kprobe_test3_result".as_ptr(),
        );

        ASSERT_EQ(
            (*(*skel).bss).kretprobe_testmod_test1_result,
            1,
            c"kretprobe_test1_result".as_ptr(),
        );
        ASSERT_EQ(
            (*(*skel).bss).kretprobe_testmod_test2_result,
            1,
            c"kretprobe_test2_result".as_ptr(),
        );
        ASSERT_EQ(
            (*(*skel).bss).kretprobe_testmod_test3_result,
            1,
            c"kretprobe_test3_result".as_ptr(),
        );
    }
}

unsafe fn test_testmod_attach_api(opts: *mut bpf_kprobe_multi_opts) {
    let mut skel: *mut kprobe_multi = ptr::null_mut();

    unsafe {
        skel = kprobe_multi__open_and_load();
        if !ASSERT_OK_PTR(skel.cast(), c"fentry_raw_skel_load".as_ptr()) {
            return;
        }

        (*(*skel).bss).pid = getpid();

        (*skel).links.test_kprobe_testmod = bpf_program__attach_kprobe_multi_opts(
            (*skel).progs.test_kprobe_testmod,
            ptr::null(),
            opts,
        );
        if (*skel).links.test_kprobe_testmod.is_null() {
            kprobe_multi__destroy(skel);
            return;
        }

        (*opts).retprobe = true;
        (*skel).links.test_kretprobe_testmod = bpf_program__attach_kprobe_multi_opts(
            (*skel).progs.test_kretprobe_testmod,
            ptr::null(),
            opts,
        );
        if (*skel).links.test_kretprobe_testmod.is_null() {
            kprobe_multi__destroy(skel);
            return;
        }

        ASSERT_OK(trigger_module_test_read(1), c"trigger_read".as_ptr());
        kprobe_multi_testmod_check(skel);

        kprobe_multi__destroy(skel);
    }
}

unsafe fn test_testmod_attach_api_addrs() {
    let mut opts = bpf_kprobe_multi_opts::default();
    let mut addrs: [u64; 3] = [0; 3];

    unsafe {
        addrs[0] = ksym_get_addr_local(ksyms, c"bpf_testmod_fentry_test1".as_ptr());
        ASSERT_NEQ(addrs[0], 0, c"ksym_get_addr_local".as_ptr());
        addrs[1] = ksym_get_addr_local(ksyms, c"bpf_testmod_fentry_test2".as_ptr());
        ASSERT_NEQ(addrs[1], 0, c"ksym_get_addr_local".as_ptr());
        addrs[2] = ksym_get_addr_local(ksyms, c"bpf_testmod_fentry_test3".as_ptr());
        ASSERT_NEQ(addrs[2], 0, c"ksym_get_addr_local".as_ptr());

        opts.addrs = addrs.as_ptr().cast::<c_ulong>();
        opts.cnt = addrs.len();

        test_testmod_attach_api(&mut opts);
    }
}

unsafe fn test_testmod_attach_api_syms() {
    let mut opts = bpf_kprobe_multi_opts::default();
    let syms: [*const c_char; 3] = [
        c"bpf_testmod_fentry_test1".as_ptr(),
        c"bpf_testmod_fentry_test2".as_ptr(),
        c"bpf_testmod_fentry_test3".as_ptr(),
    ];

    opts.syms = syms.as_ptr();
    opts.cnt = syms.len();
    unsafe {
        test_testmod_attach_api(&mut opts);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_kprobe_multi_testmod_test() {
    unsafe {
        ksyms = load_kallsyms_local();
        if !ASSERT_OK_PTR(ksyms.cast(), c"load_kallsyms_local".as_ptr()) {
            return;
        }

        if test__start_subtest(c"testmod_attach_api_syms".as_ptr()) {
            test_testmod_attach_api_syms();
        }

        if test__start_subtest(c"testmod_attach_api_addrs".as_ptr()) {
            test_testmod_attach_api_addrs();
        }

        free_kallsyms_local(ksyms);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
