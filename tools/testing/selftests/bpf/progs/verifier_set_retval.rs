// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;

#[repr(C)]
pub struct bpf_cgroup_dev_ctx {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_set_retval(retval: i32) -> i64;
    fn bpf_get_prandom_u32() -> u32;
}

// SEC("lsm_cgroup/socket_create")
// __description("lsm_cgroup bpf_set_retval success")
// __success
#[no_mangle]
pub unsafe extern "C" fn lsm_cgroup_set_retval_zero_valid(
    family: i32,
    type_: i32,
    protocol: i32,
    kern: i32,
) -> i32 {
    let _ = (family, type_, protocol, kern);
    bpf_set_retval(0);
    return 0;
}

// SEC("lsm_cgroup/socket_create")
// __description("lsm_cgroup bpf_set_retval valid errno")
// __success
#[no_mangle]
pub unsafe extern "C" fn lsm_cgroup_set_retval_negative_valid(
    family: i32,
    type_: i32,
    protocol: i32,
    kern: i32,
) -> i32 {
    let _ = (family, type_, protocol, kern);
    bpf_set_retval(-12);
    return 0;
}

// SEC("lsm_cgroup/socket_create")
// __description("lsm_cgroup bpf_set_retval invalid negative value")
// __failure __msg("should have been in [-4095, 0]")
#[no_mangle]
pub unsafe extern "C" fn lsm_cgroup_set_retval_negative_invalid(
    family: i32,
    type_: i32,
    protocol: i32,
    kern: i32,
) -> i32 {
    let _ = (family, type_, protocol, kern);
    bpf_set_retval(-4096);
    return 0;
}

// SEC("lsm_cgroup/socket_create")
// __description("lsm_cgroup bpf_set_retval invalid positive value")
// __failure __msg("should have been in [-4095, 0]")
#[no_mangle]
pub unsafe extern "C" fn lsm_cgroup_set_retval_positive_invalid(
    family: i32,
    type_: i32,
    protocol: i32,
    kern: i32,
) -> i32 {
    let _ = (family, type_, protocol, kern);
    bpf_set_retval(1);
    return 0;
}

// SEC("cgroup/dev")
// __description("cgroup_device bpf_set_retval success")
// __success
#[no_mangle]
pub unsafe extern "C" fn cgroup_dev_set_retval_0(ctx: *mut bpf_cgroup_dev_ctx) -> i32 {
    let _ = ctx;
    bpf_set_retval(0);
    return 1;
}

// SEC("cgroup/dev")
// __description("cgroup_device bpf_set_retval valid errno")
// __success
#[no_mangle]
pub unsafe extern "C" fn cgroup_dev_set_retval_neg_maxerrno(
    ctx: *mut bpf_cgroup_dev_ctx,
) -> i32 {
    let _ = ctx;
    bpf_set_retval(-4095);
    return 1;
}

// SEC("cgroup/dev")
// __description("cgroup_device bpf_set_retval invalid positive value")
// __failure __msg("should have been in [-4095, 0]")
#[no_mangle]
pub unsafe extern "C" fn cgroup_dev_set_retval_1(ctx: *mut bpf_cgroup_dev_ctx) -> i32 {
    let _ = ctx;
    bpf_set_retval(1);
    return 1;
}

// SEC("cgroup/dev")
// __description("cgroup_device bpf_set_retval invalid negative value")
// __failure __msg("should have been in [-4095, 0]")
#[no_mangle]
pub unsafe extern "C" fn cgroup_dev_set_retval_neg_4096(
    ctx: *mut bpf_cgroup_dev_ctx,
) -> i32 {
    let _ = ctx;
    bpf_set_retval(-4096);
    return 1;
}

// SEC("cgroup/dev")
// __description("bpf_set_retval bounds check survives state pruning")
// __failure __msg("should have been in [-4095, 0]")
// __naked
#[no_mangle]
pub unsafe extern "C" fn cgroup_dev_set_retval_pruning_bypass(
    ctx: *mut bpf_cgroup_dev_ctx,
) -> i32 {
    let _ = ctx;
    asm!(
        "call {bpf_get_prandom_u32}",
        "if r0 != 0 goto 1f",
        "r0 = r0",
        "r0 = r0",
        "r0 = r0",
        "r0 = r0",
        "goto 2f",
        "1:",
        "call {bpf_get_prandom_u32}",
        "2:",
        "r1 = r0",
        "call {bpf_set_retval}",
        "r0 = 1",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        bpf_set_retval = sym bpf_set_retval,
        options(noreturn)
    );
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
