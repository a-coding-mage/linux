// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2020 Google LLC.
 */

// Dependencies from the original C includes:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, and <bpf/bpf_tracing.h>.

type __s32 = i32;
type __u32 = u32;
type __u64 = u64;

extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

static mut sequence: i32 = 0;
#[no_mangle]
pub static mut input_retval: __s32 = 0;
#[no_mangle]
pub static mut test_pid: __u32 = 0;

#[no_mangle]
pub static mut fentry_result: __u64 = 0;

#[no_mangle]
#[link_section = "fentry/bpf_modify_return_test"]
pub unsafe extern "C" fn fentry_test(a: i32, b: __u64) -> i32 {
    let _ = a;
    let _ = b;

    if (bpf_get_current_pid_tgid() >> 32) != test_pid as __u64 {
        return 0;
    }
    sequence += 1;
    fentry_result = (sequence == 1) as __u64;
    return 0;
}

#[no_mangle]
pub static mut fmod_ret_result: __u64 = 0;

#[no_mangle]
#[link_section = "fmod_ret/bpf_modify_return_test"]
pub unsafe extern "C" fn fmod_ret_test(a: i32, b: *mut i32, ret: i32) -> i32 {
    let _ = a;
    let _ = b;

    if (bpf_get_current_pid_tgid() >> 32) != test_pid as __u64 {
        return ret;
    }
    sequence += 1;
    /* This is the first fmod_ret program, the ret passed should be 0 */
    fmod_ret_result = (sequence == 2 && ret == 0) as __u64;
    return input_retval;
}

#[no_mangle]
pub static mut fexit_result: __u64 = 0;

#[no_mangle]
#[link_section = "fexit/bpf_modify_return_test"]
pub unsafe extern "C" fn fexit_test(a: i32, b: __u64, ret: i32) -> i32 {
    let _ = a;
    let _ = b;

    if (bpf_get_current_pid_tgid() >> 32) != test_pid as __u64 {
        return 0;
    }
    sequence += 1;
    /* If the input_reval is non-zero a successful modification should have
     * occurred.
     */
    if input_retval != 0 {
        fexit_result = (sequence == 3 && ret == input_retval) as __u64;
    } else {
        fexit_result = (sequence == 3 && ret == 4) as __u64;
    }

    return 0;
}

static mut sequence2: i32 = 0;

#[no_mangle]
pub static mut fentry_result2: __u64 = 0;

#[no_mangle]
#[link_section = "fentry/bpf_modify_return_test2"]
pub unsafe extern "C" fn fentry_test2(
    a: i32,
    b: *mut i32,
    c: i16,
    d: i32,
    e: *mut core::ffi::c_void,
    f: i8,
    g: i32,
) -> i32 {
    let _ = a;
    let _ = b;
    let _ = c;
    let _ = d;
    let _ = e;
    let _ = f;
    let _ = g;

    if (bpf_get_current_pid_tgid() >> 32) != test_pid as __u64 {
        return 0;
    }
    sequence2 += 1;
    fentry_result2 = (sequence2 == 1) as __u64;
    return 0;
}

#[no_mangle]
pub static mut fmod_ret_result2: __u64 = 0;

#[no_mangle]
#[link_section = "fmod_ret/bpf_modify_return_test2"]
pub unsafe extern "C" fn fmod_ret_test2(
    a: i32,
    b: *mut i32,
    c: i16,
    d: i32,
    e: *mut core::ffi::c_void,
    f: i8,
    g: i32,
    ret: i32,
) -> i32 {
    let _ = a;
    let _ = b;
    let _ = c;
    let _ = d;
    let _ = e;
    let _ = f;
    let _ = g;

    if (bpf_get_current_pid_tgid() >> 32) != test_pid as __u64 {
        return ret;
    }
    sequence2 += 1;
    /* This is the first fmod_ret program, the ret passed should be 0 */
    fmod_ret_result2 = (sequence2 == 2 && ret == 0) as __u64;
    return input_retval;
}

#[no_mangle]
pub static mut fexit_result2: __u64 = 0;

#[no_mangle]
#[link_section = "fexit/bpf_modify_return_test2"]
pub unsafe extern "C" fn fexit_test2(
    a: i32,
    b: *mut i32,
    c: i16,
    d: i32,
    e: *mut core::ffi::c_void,
    f: i8,
    g: i32,
    ret: i32,
) -> i32 {
    let _ = a;
    let _ = b;
    let _ = c;
    let _ = d;
    let _ = e;
    let _ = f;
    let _ = g;

    if (bpf_get_current_pid_tgid() >> 32) != test_pid as __u64 {
        return 0;
    }
    sequence2 += 1;
    /* If the input_reval is non-zero a successful modification should have
     * occurred.
     */
    if input_retval != 0 {
        fexit_result2 = (sequence2 == 3 && ret == input_retval) as __u64;
    } else {
        fexit_result2 = (sequence2 == 3 && ret == 29) as __u64;
    }

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
