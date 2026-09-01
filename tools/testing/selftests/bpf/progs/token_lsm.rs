// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/* Dependencies from the original C source:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 */

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut my_pid: i32 = 0;
#[no_mangle]
pub static mut reject_capable: i32 = 0;
#[no_mangle]
pub static mut reject_cmd: i32 = 0;

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[no_mangle]
#[link_section = "lsm/bpf_token_capable"]
pub unsafe extern "C" fn token_capable(token: *mut bpf_token, cap: i32) -> i32 {
    if my_pid == 0 || my_pid != (bpf_get_current_pid_tgid() >> 32) as i32 {
        return 0;
    }
    if reject_capable != 0 {
        return -1;
    }
    return 0;
}

#[no_mangle]
#[link_section = "lsm/bpf_token_cmd"]
pub unsafe extern "C" fn token_cmd(token: *mut bpf_token, cmd: bpf_cmd) -> i32 {
    if my_pid == 0 || my_pid != (bpf_get_current_pid_tgid() >> 32) as i32 {
        return 0;
    }
    if reject_cmd != 0 {
        return -1;
    }
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
