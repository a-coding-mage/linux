// SPDX-License-Identifier: GPL-2.0

// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#[repr(C)]
pub struct map_array {
    pub type_: u32,
    pub max_entries: u32,
    pub key: u32,
    pub value: u32,
}

// __uint(type, BPF_MAP_TYPE_ARRAY);
// __uint(max_entries, 1);
// __type(key, __u32);
// __type(value, __u32);
#[link_section = ".maps"]
#[no_mangle]
pub static mut map_array: map_array = map_array {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key: 0,
    value: 0,
};

extern "C" {
    static BPF_MAP_TYPE_ARRAY: u32;
    fn bpf_tail_call();
}

// SEC("socket")
// __description("invalid map type for tail call")
// __failure __msg("expected prog array map for tail call")
// __failure_unpriv
#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn invalid_map_for_tail_call() {
    core::arch::asm!(
        "r2 = {map_array} ll",
        "r3 = 0",
        "call {bpf_tail_call}",
        "exit",
        map_array = sym map_array,
        bpf_tail_call = sym bpf_tail_call,
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
