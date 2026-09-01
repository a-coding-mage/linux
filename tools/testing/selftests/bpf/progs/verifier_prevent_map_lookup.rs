// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/prevent_map_lookup.c */

// C dependencies translated as external requirements:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#[repr(C)]
pub struct MapStacktrace {
    // __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    // __uint(max_entries, 1);
    // __type(key, __u32);
    // __type(value, __u64);
    _private: [u8; 0],
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static map_stacktrace: MapStacktrace = MapStacktrace { _private: [] };

#[repr(C)]
pub struct MapProg2Socket {
    // __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    // __uint(max_entries, 8);
    // __uint(key_size, sizeof(int));
    // __array(values, void (void));
    _private: [u8; 0],
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static map_prog2_socket: MapProg2Socket = MapProg2Socket { _private: [] };

// SEC("perf_event")
// __description("prevent map lookup in stack trace")
// __failure
// __msg("cannot pass map_type 7 into func bpf_map_lookup_elem")
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn map_lookup_in_stack_trace() {
    core::arch::asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_stacktrace} ll",
        "call {bpf_map_lookup_elem}",
        "exit",
        map_stacktrace = sym map_stacktrace,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn),
    );
}

// SEC("socket")
// __description("prevent map lookup in prog array")
// __failure
// __msg("cannot pass map_type 3 into func bpf_map_lookup_elem")
// __failure_unpriv
// __naked
#[unsafe(no_mangle)]
pub unsafe extern "C" fn map_lookup_in_prog_array() {
    core::arch::asm!(
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r2 = r10",
        "r2 += -8",
        "r1 = {map_prog2_socket} ll",
        "call {bpf_map_lookup_elem}",
        "exit",
        map_prog2_socket = sym map_prog2_socket,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        options(noreturn),
    );
}

unsafe extern "C" {
    fn bpf_map_lookup_elem();
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
