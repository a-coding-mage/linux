// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Facebook */

/*
 * Translated from C source that included:
 * <test_progs.h>
 * <network_helpers.h>
 * "dynptr_fail.skel.h"
 * "dynptr_success.skel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, size_of_val};

const PAGE_SIZE_64K: c_int = 65536;

#[repr(C)]
#[derive(Copy, Clone)]
enum test_setup_type {
    SETUP_SYSCALL_SLEEP,
    SETUP_SKB_PROG,
    SETUP_SKB_PROG_TP,
    SETUP_XDP_PROG,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct success_test {
    prog_name: *const c_char,
    type_: test_setup_type,
}

#[repr(C)]
struct dynptr_success {
    obj: *mut bpf_object,
    bss: *mut dynptr_success_bss,
    data: *mut dynptr_success_data,
}

#[repr(C)]
struct dynptr_success_bss {
    pid: c_int,
    user_ptr: *mut c_char,
    expected_str: [c_char; 384],
    err: c_int,
}

#[repr(C)]
struct dynptr_success_data {
    test_len: [usize; 1],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_test_run_opts {
    data_in: *const c_void,
    data_size_in: c_uint,
    data_out: *mut c_void,
    data_size_out: c_uint,
    ctx_in: *mut c_void,
    ctx_size_in: c_uint,
    repeat: c_uint,
}

const BPF_PROG_TYPE_SCHED_CLS: c_int = 3;

unsafe extern "C" {
    static pkt_v4: [u8; 0];

    fn dynptr_success__open() -> *mut dynptr_success;
    fn dynptr_success__load(skel: *mut dynptr_success) -> c_int;
    fn dynptr_success__destroy(skel: *mut dynptr_success);

    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        pobj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn RUN_TESTS_dynptr_fail();

    fn getpid() -> c_int;
    fn getpagesize() -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

static mut success_tests: [success_test; 34] = [
    success_test {
        prog_name: c"test_read_write".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_dynptr_data".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_dynptr_copy".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_dynptr_copy_xdp".as_ptr(),
        type_: test_setup_type::SETUP_XDP_PROG,
    },
    success_test {
        prog_name: c"test_dynptr_memset_zero".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_dynptr_memset_notzero".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_dynptr_memset_zero_offset".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_dynptr_memset_zero_adjusted".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_dynptr_memset_overflow".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_dynptr_memset_overflow_offset".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_dynptr_memset_readonly".as_ptr(),
        type_: test_setup_type::SETUP_SKB_PROG,
    },
    success_test {
        prog_name: c"test_dynptr_memset_xdp_chunks".as_ptr(),
        type_: test_setup_type::SETUP_XDP_PROG,
    },
    success_test {
        prog_name: c"test_ringbuf".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_skb_readonly".as_ptr(),
        type_: test_setup_type::SETUP_SKB_PROG,
    },
    success_test {
        prog_name: c"test_dynptr_skb_data".as_ptr(),
        type_: test_setup_type::SETUP_SKB_PROG,
    },
    success_test {
        prog_name: c"test_dynptr_skb_meta_data".as_ptr(),
        type_: test_setup_type::SETUP_SKB_PROG,
    },
    success_test {
        prog_name: c"test_dynptr_skb_meta_flags".as_ptr(),
        type_: test_setup_type::SETUP_SKB_PROG,
    },
    success_test {
        prog_name: c"test_adjust".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_adjust_err".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_zero_size_dynptr".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_dynptr_is_null".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_dynptr_is_rdonly".as_ptr(),
        type_: test_setup_type::SETUP_SKB_PROG,
    },
    success_test {
        prog_name: c"test_dynptr_clone".as_ptr(),
        type_: test_setup_type::SETUP_SKB_PROG,
    },
    success_test {
        prog_name: c"test_dynptr_skb_no_buff".as_ptr(),
        type_: test_setup_type::SETUP_SKB_PROG,
    },
    success_test {
        prog_name: c"test_dynptr_skb_strcmp".as_ptr(),
        type_: test_setup_type::SETUP_SKB_PROG,
    },
    success_test {
        prog_name: c"test_dynptr_skb_tp_btf".as_ptr(),
        type_: test_setup_type::SETUP_SKB_PROG_TP,
    },
    success_test {
        prog_name: c"test_probe_read_user_dynptr".as_ptr(),
        type_: test_setup_type::SETUP_XDP_PROG,
    },
    success_test {
        prog_name: c"test_probe_read_kernel_dynptr".as_ptr(),
        type_: test_setup_type::SETUP_XDP_PROG,
    },
    success_test {
        prog_name: c"test_probe_read_user_str_dynptr".as_ptr(),
        type_: test_setup_type::SETUP_XDP_PROG,
    },
    success_test {
        prog_name: c"test_probe_read_kernel_str_dynptr".as_ptr(),
        type_: test_setup_type::SETUP_XDP_PROG,
    },
    success_test {
        prog_name: c"test_copy_from_user_dynptr".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_copy_from_user_str_dynptr".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_copy_from_user_task_dynptr".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
    success_test {
        prog_name: c"test_copy_from_user_task_str_dynptr".as_ptr(),
        type_: test_setup_type::SETUP_SYSCALL_SLEEP,
    },
];

unsafe fn verify_success(prog_name: *const c_char, setup_type: test_setup_type) {
    let mut user_data: [c_char; 384] = [b'a' as c_char; 384];
    user_data[383] = 0;
    let skel: *mut dynptr_success;
    let prog: *mut bpf_program;
    let mut link: *mut bpf_link;
    let mut err: c_int;

    skel = dynptr_success__open();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"dynptr_success__open".as_ptr()) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    prog = bpf_object__find_program_by_name((*skel).obj, prog_name);
    if !ASSERT_OK_PTR(
        prog as *mut c_void,
        c"bpf_object__find_program_by_name".as_ptr(),
    ) {
        dynptr_success__destroy(skel);
        return;
    }

    bpf_program__set_autoload(prog, true);

    err = dynptr_success__load(skel);
    if !ASSERT_OK(err, c"dynptr_success__load".as_ptr()) {
        dynptr_success__destroy(skel);
        return;
    }

    (*(*skel).bss).user_ptr = user_data.as_mut_ptr();
    (*(*skel).data).test_len[0] = size_of::<[c_char; 384]>();
    memcpy(
        (*(*skel).bss).expected_str.as_mut_ptr() as *mut c_void,
        user_data.as_ptr() as *const c_void,
        size_of::<[c_char; 384]>(),
    );

    match setup_type {
        test_setup_type::SETUP_SYSCALL_SLEEP => {
            link = bpf_program__attach(prog);
            if !ASSERT_OK_PTR(link as *mut c_void, c"bpf_program__attach".as_ptr()) {
                dynptr_success__destroy(skel);
                return;
            }

            usleep(1);

            bpf_link__destroy(link);
        }
        test_setup_type::SETUP_SKB_PROG => {
            let prog_fd: c_int;
            let mut buf: [c_char; 64] = [0; 64];

            let mut topts = bpf_test_run_opts {
                data_in: &raw const pkt_v4 as *const c_void,
                data_size_in: size_of_val(&pkt_v4) as c_uint,
                data_out: buf.as_mut_ptr() as *mut c_void,
                data_size_out: size_of::<[c_char; 64]>() as c_uint,
                ctx_in: core::ptr::null_mut(),
                ctx_size_in: 0,
                repeat: 1,
            };

            prog_fd = bpf_program__fd(prog);
            if !ASSERT_GE(prog_fd, 0, c"prog_fd".as_ptr()) {
                dynptr_success__destroy(skel);
                return;
            }

            err = bpf_prog_test_run_opts(prog_fd, &mut topts);

            if !ASSERT_OK(err, c"test_run".as_ptr()) {
                dynptr_success__destroy(skel);
                return;
            }
        }
        test_setup_type::SETUP_SKB_PROG_TP => {
            let mut skb: __sk_buff = core::mem::zeroed();
            let mut obj: *mut bpf_object = core::ptr::null_mut();
            let mut aux_prog_fd: c_int = 0;

            /* Just use its test_run to trigger kfree_skb tracepoint */
            err = bpf_prog_test_load(
                c"./test_pkt_access.bpf.o".as_ptr(),
                BPF_PROG_TYPE_SCHED_CLS,
                &mut obj,
                &mut aux_prog_fd,
            );
            if !ASSERT_OK(err, c"prog_load sched cls".as_ptr()) {
                dynptr_success__destroy(skel);
                return;
            }

            let mut topts = bpf_test_run_opts {
                data_in: &raw const pkt_v4 as *const c_void,
                data_size_in: size_of_val(&pkt_v4) as c_uint,
                data_out: core::ptr::null_mut(),
                data_size_out: 0,
                ctx_in: &mut skb as *mut __sk_buff as *mut c_void,
                ctx_size_in: size_of::<__sk_buff>() as c_uint,
                repeat: 0,
            };

            link = bpf_program__attach(prog);
            if !ASSERT_OK_PTR(link as *mut c_void, c"bpf_program__attach".as_ptr()) {
                bpf_object__close(obj);
                dynptr_success__destroy(skel);
                return;
            }

            err = bpf_prog_test_run_opts(aux_prog_fd, &mut topts);
            bpf_link__destroy(link);
            bpf_object__close(obj);

            if !ASSERT_OK(err, c"test_run".as_ptr()) {
                dynptr_success__destroy(skel);
                return;
            }
        }
        test_setup_type::SETUP_XDP_PROG => {
            let mut data: [c_char; 90000] = [0; 90000];
            let prog_fd: c_int;
            let mut opts = bpf_test_run_opts {
                data_in: data.as_mut_ptr() as *const c_void,
                data_size_in: 0,
                data_out: core::ptr::null_mut(),
                data_size_out: 0,
                ctx_in: core::ptr::null_mut(),
                ctx_size_in: 0,
                repeat: 1,
            };

            if getpagesize() == PAGE_SIZE_64K {
                opts.data_size_in = size_of::<[c_char; 90000]>() as c_uint;
            } else {
                opts.data_size_in = 5000;
            }

            prog_fd = bpf_program__fd(prog);
            err = bpf_prog_test_run_opts(prog_fd, &mut opts);

            if !ASSERT_OK(err, c"test_run".as_ptr()) {
                dynptr_success__destroy(skel);
                return;
            }
        }
    }

    ASSERT_EQ((*(*skel).bss).err, 0, c"err".as_ptr());

    dynptr_success__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_dynptr() {
    let mut i: c_int;

    i = 0;
    while (i as usize) < success_tests.len() {
        if !test__start_subtest(success_tests[i as usize].prog_name) {
            i += 1;
            continue;
        }

        verify_success(
            success_tests[i as usize].prog_name,
            success_tests[i as usize].type_,
        );
        i += 1;
    }

    RUN_TESTS_dynptr_fail();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
