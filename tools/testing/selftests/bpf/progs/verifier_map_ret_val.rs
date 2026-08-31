// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/map_ret_val.c */

/* C dependencies removed from executable Rust code:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 * #include "../../../include/linux/filter.h"
 * #include "bpf_misc.h"
 */

#[repr(C)]
pub struct MapHash8b {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

/* Original BPF map declaration:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_HASH);
 *     __uint(max_entries, 1);
 *     __type(key, long long);
 *     __type(value, long long);
 * } map_hash_8b SEC(".maps");
 */
#[link_section = ".maps"]
#[no_mangle]
pub static mut map_hash_8b: MapHash8b = MapHash8b {
    type_: BPF_MAP_TYPE_HASH as u32,
    max_entries: 1,
    key_size: core::mem::size_of::<i64>() as u32,
    value_size: core::mem::size_of::<i64>() as u32,
};

extern "C" {
    static BPF_MAP_TYPE_HASH: i32;
    fn bpf_map_delete_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> i64;
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
}

/* SEC("socket")
 * __description("invalid map_fd for function call")
 * __failure __msg("fd 0 is not pointing to valid bpf_map")
 * __failure_unpriv
 * __naked
 */
#[no_mangle]
pub unsafe extern "C" fn map_fd_for_function_call() {
    core::arch::asm!(
        r#"
	r2 = 0;
	*(u64*)(r10 - 8) = r2;
	r2 = r10;
	r2 += -8;
	.8byte {ld_map_fd};
	.8byte 0;
	call {bpf_map_delete_elem};
	exit;
"#,
        bpf_map_delete_elem = sym bpf_map_delete_elem,
        ld_map_fd = const 0,
        options(noreturn)
    );
}

/* SEC("socket")
 * __description("don't check return value before access")
 * __failure __msg("R0 invalid mem access 'map_value_or_null'")
 * __failure_unpriv
 * __naked
 */
#[no_mangle]
pub unsafe extern "C" fn check_return_value_before_access() {
    core::arch::asm!(
        r#"
	r1 = 0;
	*(u64*)(r10 - 8) = r1;
	r2 = r10;
	r2 += -8;
	r1 = {map_hash_8b} ll;
	call {bpf_map_lookup_elem};
	r1 = 0;
	*(u64*)(r0 + 0) = r1;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

/* SEC("socket")
 * __description("access memory with incorrect alignment")
 * __failure __msg("misaligned value access")
 * __failure_unpriv
 * __flag(BPF_F_STRICT_ALIGNMENT)
 * __naked
 */
#[no_mangle]
pub unsafe extern "C" fn access_memory_with_incorrect_alignment_1() {
    core::arch::asm!(
        r#"
	r1 = 0;
	*(u64*)(r10 - 8) = r1;
	r2 = r10;
	r2 += -8;
	r1 = {map_hash_8b} ll;
	call {bpf_map_lookup_elem};
	if r0 == 0 goto 0f;
	r1 = 0;
	*(u64*)(r0 + 4) = r1;
0:	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

/* SEC("socket")
 * __description("sometimes access memory with incorrect alignment")
 * __failure __msg("R0 invalid mem access")
 * __msg_unpriv("R0 leaks addr")
 * __flag(BPF_F_STRICT_ALIGNMENT)
 * __naked
 */
#[no_mangle]
pub unsafe extern "C" fn access_memory_with_incorrect_alignment_2() {
    core::arch::asm!(
        r#"
	r1 = 0;
	*(u64*)(r10 - 8) = r1;
	r2 = r10;
	r2 += -8;
	r1 = {map_hash_8b} ll;
	call {bpf_map_lookup_elem};
	if r0 == 0 goto 0f;
	r1 = 0;
	*(u64*)(r0 + 0) = r1;
	exit;
0:	r1 = 1;
	*(u64*)(r0 + 0) = r1;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        map_hash_8b = sym map_hash_8b,
        options(noreturn)
    );
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
