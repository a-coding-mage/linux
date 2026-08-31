// SPDX-License-Identifier: GPL-2.0
// Converted from tools/testing/selftests/bpf/verifier/value_ptr_arith.c

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_attributes)]

// Dependencies originally supplied by <linux/bpf.h>, <bpf/bpf_helpers.h>, <errno.h>, and "bpf_misc.h".
// Test metadata macros such as SEC, __description, __success, __failure, and __retval are preserved as Rust comments.

const MAX_ENTRIES: usize = 11;
const EINVAL: i64 = 22;
const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_F_ANY_ALIGNMENT: u32 = 2;

#[repr(C)]
pub struct test_val {
    pub index: u32,
    pub foo: [i32; MAX_ENTRIES],
}

#[repr(C)]
pub struct other_val {
    pub foo: i64,
    pub bar: i64,
}

#[repr(C)]
pub struct bpf_map_def<K, V> {
    pub type_: u32,
    pub max_entries: u32,
    pub key: core::marker::PhantomData<K>,
    pub value: core::marker::PhantomData<V>,
}

#[link_section = ".maps"]
#[no_mangle]
pub static map_array_48b: bpf_map_def<i32, test_val> = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key: core::marker::PhantomData,
    value: core::marker::PhantomData,
};

#[link_section = ".maps"]
#[no_mangle]
pub static map_hash_16b: bpf_map_def<i64, other_val> = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key: core::marker::PhantomData,
    value: core::marker::PhantomData,
};

#[link_section = ".maps"]
#[no_mangle]
pub static map_hash_48b: bpf_map_def<i64, test_val> = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key: core::marker::PhantomData,
    value: core::marker::PhantomData,
};

extern "C" {
    fn bpf_map_lookup_elem(map: *const core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_delete_elem(map: *const core::ffi::c_void, key: *const core::ffi::c_void) -> i64;
    fn bpf_get_prandom_u32() -> u32;
}

// SPDX-License-Identifier: GPL-2.0




#[link_section = "socket"]
#[no_mangle]
// __description("map access: known scalar += value_ptr unknown vs const")
// __success __failure_unpriv
// __msg_unpriv("R1 tried to add from different maps, paths or scalars")
// __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: known scalar += value_ptr const vs unknown")
// __success __failure_unpriv
// __msg_unpriv("R1 tried to add from different maps, paths or scalars")
// __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: known scalar += value_ptr const vs const (ne)")
// __success __failure_unpriv
// __msg_unpriv("R1 tried to add from different maps, paths or scalars")
// __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: known scalar += value_ptr const vs const (eq)")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: known scalar += value_ptr unknown vs unknown (eq)")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: known scalar += value_ptr unknown vs unknown (lt)")
// __success __failure_unpriv
// __msg_unpriv("R1 tried to add from different maps, paths or scalars")
// __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: known scalar += value_ptr unknown vs unknown (gt)")
// __success __failure_unpriv
// __msg_unpriv("R1 tried to add from different maps, paths or scalars")
// __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: known scalar += value_ptr from different maps")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr -= known scalar from different maps")
// __success __failure_unpriv
// __msg_unpriv("R0 min value is negative")
// __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: known scalar += value_ptr from different maps, but same value properties")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: mixing value pointer and scalar, 1")
// __success __failure_unpriv
// __msg_unpriv("R2 tried to add from different maps, paths or scalars, pointer arithmetic with it prohibited for !root")
// __retval(0)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: mixing value pointer and scalar, 2")
// __success __failure_unpriv
// __msg_unpriv("R2 tried to add from different maps, paths or scalars, pointer arithmetic with it prohibited for !root")
// __retval(0)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("sanitation: alu with different scalars 1")
// __success __success_unpriv __retval(0x100000)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("sanitation: alu with different scalars 2")
// __success __success_unpriv __retval(0)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("sanitation: alu with different scalars 3")
// __success __success_unpriv __retval(0)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += known scalar, upper oob arith, test 1")
// __success __failure_unpriv
// __msg_unpriv("R0 pointer arithmetic of map value goes out of range")
// __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += known scalar, upper oob arith, test 2")
// __success __failure_unpriv
// __msg_unpriv("R0 pointer arithmetic of map value goes out of range")
// __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += known scalar, upper oob arith, test 3")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr -= known scalar, lower oob arith, test 1")
// __failure __msg("R0 min value is negative")
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr -= known scalar, lower oob arith, test 2")
// __success __failure_unpriv
// __msg_unpriv("R0 pointer arithmetic of map value goes out of range")
// __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr -= known scalar, lower oob arith, test 3")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: known scalar += value_ptr")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += known scalar, 1")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += known scalar, 2")
// __failure __msg("invalid access to map value")
// __failure_unpriv
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += known scalar, 3")
// __failure __msg("R0 min value is negative")
// __failure_unpriv
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += known scalar, 4")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += known scalar, 5")
// __success __success_unpriv __retval(0xabcdef12)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += known scalar, 6")
// __success __success_unpriv __retval(0xabcdef12)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += N, value_ptr -= N known scalar")
// __success __success_unpriv __retval(0x12345678)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: unknown scalar += value_ptr, 1")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: unknown scalar += value_ptr, 2")
// __success __success_unpriv __retval(0xabcdef12) __flag(BPF_F_ANY_ALIGNMENT)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: unknown scalar += value_ptr, 3")
// __success __failure_unpriv
// __msg_unpriv("R0 pointer arithmetic of map value goes out of range")
// __retval(0xabcdef12) __flag(BPF_F_ANY_ALIGNMENT)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: unknown scalar += value_ptr, 4")
// __failure __msg("R1 max value is outside of the allowed memory range")
// __msg_unpriv("R1 pointer arithmetic of map value goes out of range")
// __flag(BPF_F_ANY_ALIGNMENT)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += unknown scalar, 1")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += unknown scalar, 2")
// __success __success_unpriv __retval(0xabcdef12) __flag(BPF_F_ANY_ALIGNMENT)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += unknown scalar, 3")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr += value_ptr")
// __failure __msg("R0 pointer += pointer prohibited")
// __failure_unpriv
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: known scalar -= value_ptr")
// __failure __msg("R1 tried to subtract pointer from scalar")
// __failure_unpriv
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr -= known scalar")
// __failure __msg("R0 min value is negative")
// __failure_unpriv
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr -= known scalar, 2")
// __success __success_unpriv __retval(1)
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: unknown scalar -= value_ptr")
// __failure __msg("R1 tried to subtract pointer from scalar")
// __failure_unpriv
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr -= unknown scalar")
// __failure __msg("R0 min value is negative")
// __failure_unpriv
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr -= unknown scalar, 2")
// __success __success_unpriv
// __retval(1)
// #ifdef SPEC_V1
// __xlated_unpriv("r1 &= 7")
// __xlated_unpriv("nospec") /* inserted to prevent `R0 pointer arithmetic of map value goes out of range` */
// __xlated_unpriv("r0 -= r1")
// #endif
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: value_ptr -= value_ptr")
// __failure __msg("R0 invalid mem access 'scalar'")
// __msg_unpriv("R0 pointer -= pointer prohibited")
// 

#[link_section = "socket"]
#[no_mangle]
// __description("map access: trying to leak tainted dst reg")
// __failure __msg("math between map_value pointer and 4294967295 is not allowed")
// __failure_unpriv
// 

#[link_section = "tc"]
#[no_mangle]
// __description("32bit pkt_ptr -= scalar")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
// 

#[link_section = "tc"]
#[no_mangle]
// __description("32bit scalar -= pkt_ptr")
// __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
// 

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
