// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/cgroup_storage.c */

// C includes translated as dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "../../../include/linux/filter.h"
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::asm;

extern "C" {
    static bpf_get_local_storage: u64;
}

// Map definitions preserve the original BPF map metadata:
// __uint(type, BPF_MAP_TYPE_CGROUP_STORAGE);
// __uint(max_entries, 0);
// __type(key, struct bpf_cgroup_storage_key);
// __type(value, char[TEST_DATA_LEN]);
#[link_section = ".maps"]
#[no_mangle]
pub static mut cgroup_storage: [u8; 0] = [];

// __uint(type, BPF_MAP_TYPE_HASH);
// __uint(max_entries, 1);
// __type(key, long long);
// __type(value, long long);
#[link_section = ".maps"]
#[no_mangle]
pub static mut map_hash_8b: [u8; 0] = [];

// __uint(type, BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE);
// __uint(max_entries, 0);
// __type(key, struct bpf_cgroup_storage_key);
// __type(value, char[64]);
#[link_section = ".maps"]
#[no_mangle]
pub static mut percpu_cgroup_storage: [u8; 0] = [];

#[link_section = "cgroup/skb"]
// __description("valid cgroup storage access")
// __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn valid_cgroup_storage_access() {
    asm!(
        "r2 = 0",
        "r1 = {cgroup_storage} ll",
        "call {bpf_get_local_storage}",
        "r1 = *(u32*)(r0 + 0)",
        "r0 = r1",
        "r0 &= 1",
        "exit",
        cgroup_storage = sym cgroup_storage,
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("invalid cgroup storage access 1")
// __failure __msg("cannot pass map_type 1 into func bpf_get_local_storage")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn invalid_cgroup_storage_access_1() {
    asm!(
        "r2 = 0",
        "r1 = {map_hash_8b} ll",
        "call {bpf_get_local_storage}",
        "r1 = *(u32*)(r0 + 0)",
        "r0 = r1",
        "r0 &= 1",
        "exit",
        map_hash_8b = sym map_hash_8b,
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("invalid cgroup storage access 2")
// __failure __msg("fd 1 is not pointing to valid bpf_map")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn invalid_cgroup_storage_access_2() {
    asm!(
        "r2 = 0",
        ".8byte {ld_map_fd}",
        ".8byte 0",
        "call {bpf_get_local_storage}",
        "r0 &= 1",
        "exit",
        ld_map_fd = const BPF_RAW_INSN(BPF_LD | BPF_DW | BPF_IMM, BPF_REG_1, BPF_PSEUDO_MAP_FD, 0, 1),
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("invalid cgroup storage access 3")
// __failure __msg("invalid access to map value, value_size=64 off=256 size=4")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn invalid_cgroup_storage_access_3() {
    asm!(
        "r2 = 0",
        "r1 = {cgroup_storage} ll",
        "call {bpf_get_local_storage}",
        "r1 = *(u32*)(r0 + 256)",
        "r1 += 1",
        "r0 = 0",
        "exit",
        cgroup_storage = sym cgroup_storage,
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("invalid cgroup storage access 4")
// __failure __msg("invalid access to map value, value_size=64 off=-2 size=4")
// __failure_unpriv
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn invalid_cgroup_storage_access_4() {
    asm!(
        "r2 = 0",
        "r1 = {cgroup_storage} ll",
        "call {bpf_get_local_storage}",
        "r1 = *(u32*)(r0 - 2)",
        "r0 = r1",
        "r1 += 1",
        "exit",
        cgroup_storage = sym cgroup_storage,
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("invalid cgroup storage access 5")
// __failure __msg("get_local_storage() doesn't support non-zero flags")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn invalid_cgroup_storage_access_5() {
    asm!(
        "r2 = 7",
        "r1 = {cgroup_storage} ll",
        "call {bpf_get_local_storage}",
        "r1 = *(u32*)(r0 + 0)",
        "r0 = r1",
        "r0 &= 1",
        "exit",
        cgroup_storage = sym cgroup_storage,
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("invalid cgroup storage access 6")
// __failure __msg("get_local_storage() doesn't support non-zero flags")
// __msg_unpriv("R2 leaks addr into helper function")
#[no_mangle]
pub unsafe extern "C" fn invalid_cgroup_storage_access_6() {
    asm!(
        "r2 = r1",
        "r1 = {cgroup_storage} ll",
        "call {bpf_get_local_storage}",
        "r1 = *(u32*)(r0 + 0)",
        "r0 = r1",
        "r0 &= 1",
        "exit",
        cgroup_storage = sym cgroup_storage,
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("valid per-cpu cgroup storage access")
// __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn per_cpu_cgroup_storage_access() {
    asm!(
        "r2 = 0",
        "r1 = {percpu_cgroup_storage} ll",
        "call {bpf_get_local_storage}",
        "r1 = *(u32*)(r0 + 0)",
        "r0 = r1",
        "r0 &= 1",
        "exit",
        percpu_cgroup_storage = sym percpu_cgroup_storage,
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("invalid per-cpu cgroup storage access 1")
// __failure __msg("cannot pass map_type 1 into func bpf_get_local_storage")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_storage_access_1() {
    asm!(
        "r2 = 0",
        "r1 = {map_hash_8b} ll",
        "call {bpf_get_local_storage}",
        "r1 = *(u32*)(r0 + 0)",
        "r0 = r1",
        "r0 &= 1",
        "exit",
        map_hash_8b = sym map_hash_8b,
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("invalid per-cpu cgroup storage access 2")
// __failure __msg("fd 1 is not pointing to valid bpf_map")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_storage_access_2() {
    asm!(
        "r2 = 0",
        ".8byte {ld_map_fd}",
        ".8byte 0",
        "call {bpf_get_local_storage}",
        "r0 &= 1",
        "exit",
        ld_map_fd = const BPF_RAW_INSN(BPF_LD | BPF_DW | BPF_IMM, BPF_REG_1, BPF_PSEUDO_MAP_FD, 0, 1),
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("invalid per-cpu cgroup storage access 3")
// __failure __msg("invalid access to map value, value_size=64 off=256 size=4")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_storage_access_3() {
    asm!(
        "r2 = 0",
        "r1 = {percpu_cgroup_storage} ll",
        "call {bpf_get_local_storage}",
        "r1 = *(u32*)(r0 + 256)",
        "r1 += 1",
        "r0 = 0",
        "exit",
        percpu_cgroup_storage = sym percpu_cgroup_storage,
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("invalid per-cpu cgroup storage access 4")
// __failure __msg("invalid access to map value, value_size=64 off=-2 size=4")
// __failure_unpriv
// __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_storage_access_4() {
    asm!(
        "r2 = 0",
        "r1 = {cgroup_storage} ll",
        "call {bpf_get_local_storage}",
        "r1 = *(u32*)(r0 - 2)",
        "r0 = r1",
        "r1 += 1",
        "exit",
        cgroup_storage = sym cgroup_storage,
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("invalid per-cpu cgroup storage access 5")
// __failure __msg("get_local_storage() doesn't support non-zero flags")
// __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_storage_access_5() {
    asm!(
        "r2 = 7",
        "r1 = {percpu_cgroup_storage} ll",
        "call {bpf_get_local_storage}",
        "r1 = *(u32*)(r0 + 0)",
        "r0 = r1",
        "r0 &= 1",
        "exit",
        percpu_cgroup_storage = sym percpu_cgroup_storage,
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "cgroup/skb"]
// __description("invalid per-cpu cgroup storage access 6")
// __failure __msg("get_local_storage() doesn't support non-zero flags")
// __msg_unpriv("R2 leaks addr into helper function")
#[no_mangle]
pub unsafe extern "C" fn cpu_cgroup_storage_access_6() {
    asm!(
        "r2 = r1",
        "r1 = {percpu_cgroup_storage} ll",
        "call {bpf_get_local_storage}",
        "r1 = *(u32*)(r0 + 0)",
        "r0 = r1",
        "r0 &= 1",
        "exit",
        percpu_cgroup_storage = sym percpu_cgroup_storage,
        bpf_get_local_storage = sym bpf_get_local_storage,
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
