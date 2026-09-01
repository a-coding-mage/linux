// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <test_progs.h>
// #include "bpf/libbpf_internal.h"
// #include "get_smp_processor_id.skel.h"

use core::ffi::{c_char, c_int, c_void};

const BPF_F_TEST_RUN_ON_CPU: u32 = 1;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub flags: u32,
    pub cpu: u32,
}

#[repr(C)]
pub struct get_smp_processor_id {
    pub progs: get_smp_processor_id__progs,
    pub bss: *mut get_smp_processor_id__bss,
}

#[repr(C)]
pub struct get_smp_processor_id__progs {
    pub call_bpf_get_smp_processor_id: *mut bpf_program,
}

#[repr(C)]
pub struct get_smp_processor_id__bss {
    pub cpu_nr_result: c_int,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

extern "C" {
    fn parse_cpu_mask_file(
        path: *const c_char,
        mask: *mut *mut bool,
        mask_sz: *mut c_int,
    ) -> c_int;
    fn get_smp_processor_id__open_and_load() -> *mut get_smp_processor_id;
    fn get_smp_processor_id__destroy(obj: *mut get_smp_processor_id);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn free(ptr: *mut c_void);

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

pub unsafe fn test_get_smp_processor_id() {
    let mut opts = bpf_test_run_opts {
        flags: BPF_F_TEST_RUN_ON_CPU,
        cpu: 0,
    };
    let mut skel: *mut get_smp_processor_id;
    let mut prog_fd: c_int;
    let mut err: c_int;
    let mut online_cpu_nr: c_int = 0;
    let mut i: c_int;
    let mut online: *mut bool = core::ptr::null_mut();

    err = parse_cpu_mask_file(
        b"/sys/devices/system/cpu/online\0".as_ptr() as *const c_char,
        &mut online,
        &mut online_cpu_nr,
    );
    if !ASSERT_OK(err, b"parse_cpu_mask_file\0".as_ptr() as *const c_char) {
        return;
    }

    skel = get_smp_processor_id__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        b"get_smp_processor_id__open_and_load\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(online, skel);
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.call_bpf_get_smp_processor_id);

    i = 0;
    while i < online_cpu_nr {
        if !*online.offset(i as isize) {
            i += 1;
            continue;
        }

        opts.cpu = i as u32;
        (*(*skel).bss).cpu_nr_result = -1;

        err = bpf_prog_test_run_opts(prog_fd, &mut opts);
        if !ASSERT_OK(err, b"bpf_prog_test_run_opts\0".as_ptr() as *const c_char) {
            goto_cleanup(online, skel);
            return;
        }

        ASSERT_EQ(
            (*(*skel).bss).cpu_nr_result,
            opts.cpu as c_int,
            b"cpu_nr_result\0".as_ptr() as *const c_char,
        );

        i += 1;
    }

    goto_cleanup(online, skel);
}

unsafe fn goto_cleanup(online: *mut bool, skel: *mut get_smp_processor_id) {
    free(online as *mut c_void);
    get_smp_processor_id__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
