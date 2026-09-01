// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2024. Huawei Technologies Co., Ltd */

// C dependencies:
// #include "test_progs.h"
// #include "read_vsyscall.skel.h"
//
// On x86_64 C includes <asm/vsyscall.h> for VSYSCALL_ADDR.
// On non-x86 architectures C defines VSYSCALL_ADDR as 0UL to prevent build
// failure.

#[cfg(target_arch = "x86_64")]
extern "C" {
    static VSYSCALL_ADDR: libc::c_ulong;
}

#[cfg(not(target_arch = "x86_64"))]
const VSYSCALL_ADDR: libc::c_ulong = 0;

const ERANGE: libc::c_int = 34;
const EFAULT: libc::c_int = 14;

#[repr(C)]
struct read_ret_desc {
    name: *const libc::c_char,
    ret: libc::c_int,
}

static all_read: [read_ret_desc; 10] = [
    read_ret_desc {
        name: b"probe_read_kernel\0".as_ptr() as *const libc::c_char,
        ret: -ERANGE,
    },
    read_ret_desc {
        name: b"probe_read_kernel_str\0".as_ptr() as *const libc::c_char,
        ret: -ERANGE,
    },
    read_ret_desc {
        name: b"probe_read\0".as_ptr() as *const libc::c_char,
        ret: -ERANGE,
    },
    read_ret_desc {
        name: b"probe_read_str\0".as_ptr() as *const libc::c_char,
        ret: -ERANGE,
    },
    read_ret_desc {
        name: b"probe_read_user\0".as_ptr() as *const libc::c_char,
        ret: -EFAULT,
    },
    read_ret_desc {
        name: b"probe_read_user_str\0".as_ptr() as *const libc::c_char,
        ret: -EFAULT,
    },
    read_ret_desc {
        name: b"copy_from_user\0".as_ptr() as *const libc::c_char,
        ret: -EFAULT,
    },
    read_ret_desc {
        name: b"copy_from_user_task\0".as_ptr() as *const libc::c_char,
        ret: -EFAULT,
    },
    read_ret_desc {
        name: b"copy_from_user_str\0".as_ptr() as *const libc::c_char,
        ret: -EFAULT,
    },
    read_ret_desc {
        name: b"copy_from_user_task_str\0".as_ptr() as *const libc::c_char,
        ret: -EFAULT,
    },
];

#[repr(C)]
struct read_vsyscall_bss {
    target_pid: libc::pid_t,
    user_ptr: *mut libc::c_void,
    read_ret: [libc::c_int; 10],
}

#[repr(C)]
struct read_vsyscall {
    bss: *mut read_vsyscall_bss,
}

extern "C" {
    fn test__skip();
    fn read_vsyscall__open_and_load() -> *mut read_vsyscall;
    fn read_vsyscall__attach(skel: *mut read_vsyscall) -> libc::c_int;
    fn read_vsyscall__destroy(skel: *mut read_vsyscall);
    fn ASSERT_OK_PTR(
        ptr: *mut read_vsyscall,
        name: *const libc::c_char,
    ) -> bool;
    fn ASSERT_EQ(
        actual: libc::c_int,
        expected: libc::c_int,
        name: *const libc::c_char,
    ) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn test_read_vsyscall() {
    let skel: *mut read_vsyscall;
    let mut i: libc::c_uint;
    let err: libc::c_int;

    #[cfg(not(target_arch = "x86_64"))]
    {
        test__skip();
        return;
    }

    skel = read_vsyscall__open_and_load();
    if !ASSERT_OK_PTR(skel, b"read_vsyscall open_load\0".as_ptr() as *const libc::c_char) {
        return;
    }

    (*(*skel).bss).target_pid = libc::getpid();
    err = read_vsyscall__attach(skel);
    if !ASSERT_EQ(
        err,
        0,
        b"read_vsyscall attach\0".as_ptr() as *const libc::c_char,
    ) {
        read_vsyscall__destroy(skel);
        return;
    }

    /* userspace may don't have vsyscall page due to LEGACY_VSYSCALL_NONE,
     * but it doesn't affect the returned error codes.
     */
    #[cfg(target_arch = "x86_64")]
    {
        (*(*skel).bss).user_ptr = VSYSCALL_ADDR as *mut libc::c_void;
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        (*(*skel).bss).user_ptr = VSYSCALL_ADDR as *mut libc::c_void;
    }
    libc::usleep(1);

    i = 0;
    while (i as usize) < all_read.len() {
        ASSERT_EQ(
            (*(*skel).bss).read_ret[i as usize],
            all_read[i as usize].ret,
            all_read[i as usize].name,
        );
        i += 1;
    }

    read_vsyscall__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
