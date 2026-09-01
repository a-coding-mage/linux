// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/spin_lock.c */

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::naked_asm;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_F_ANY_ALIGNMENT: u32 = 2;
const BPF_F_TEST_STATE_FREQ: u32 = 8;

#[repr(C)]
pub struct bpf_spin_lock {
    pub val: u32,
}

#[repr(C)]
pub struct val {
    pub cnt: i32,
    pub l: bpf_spin_lock,
}

#[repr(C)]
pub struct bpf_map_def_val {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_spin_lock: bpf_map_def_val = bpf_map_def_val {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<val>() as u32,
};

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_ktime_get_ns() -> u64;
}

// SEC("cgroup/skb")
// __description("spin_lock: test1 success")
// __success __failure_unpriv __msg_unpriv("")
// __retval(0)
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spin_lock_test1_success() {
    naked_asm!(
        r#"
	r1 = 0;
	*(u32*)(r10 - 4) = r1;
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 0f;
	exit;
0:	r6 = r0;
	r1 = r0;
	r1 += 4;
	call {bpf_spin_lock};
	r1 = r6;
	r1 += 4;
	r0 = *(u32*)(r6 + 0);
	call {bpf_spin_unlock};
	r0 = 0;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_lock = sym bpf_spin_lock,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
    );
}

// SEC("cgroup/skb")
// __description("spin_lock: test2 direct ld/st")
// __failure __msg("cannot be accessed directly")
// __failure_unpriv __msg_unpriv("")
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_test2_direct_ld_st() {
    naked_asm!(
        r#"
	r1 = 0;
	*(u32*)(r10 - 4) = r1;
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 0f;
	exit;
0:	r6 = r0;
	r1 = r0;
	r1 += 4;
	call {bpf_spin_lock};
	r1 = r6;
	r1 += 4;
	r0 = *(u32*)(r1 + 0);
	call {bpf_spin_unlock};
	r0 = 0;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_lock = sym bpf_spin_lock,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
    );
}

// SEC("cgroup/skb")
// __description("spin_lock: test3 direct ld/st")
// __failure __msg("cannot be accessed directly")
// __failure_unpriv __msg_unpriv("")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_test3_direct_ld_st() {
    naked_asm!(
        r#"
	r1 = 0;
	*(u32*)(r10 - 4) = r1;
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 0f;
	exit;
0:	r6 = r0;
	r1 = r0;
	r1 += 4;
	call {bpf_spin_lock};
	r1 = r6;
	r1 += 4;
	r0 = *(u32*)(r6 + 1);
	call {bpf_spin_unlock};
	r0 = 0;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_lock = sym bpf_spin_lock,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
    );
}

// SEC("cgroup/skb")
// __description("spin_lock: test4 direct ld/st")
// __failure __msg("cannot be accessed directly")
// __failure_unpriv __msg_unpriv("")
// __flag(BPF_F_ANY_ALIGNMENT)
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_test4_direct_ld_st() {
    naked_asm!(
        r#"
	r1 = 0;
	*(u32*)(r10 - 4) = r1;
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 0f;
	exit;
0:	r6 = r0;
	r1 = r0;
	r1 += 4;
	call {bpf_spin_lock};
	r1 = r6;
	r1 += 4;
	r0 = *(u16*)(r6 + 3);
	call {bpf_spin_unlock};
	r0 = 0;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_lock = sym bpf_spin_lock,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
    );
}

// SEC("cgroup/skb")
// __description("spin_lock: test5 call within a locked region")
// __failure __msg("calls are not allowed")
// __failure_unpriv __msg_unpriv("")
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_within_a_locked_region() {
    naked_asm!(
        r#"
	r1 = 0;
	*(u32*)(r10 - 4) = r1;
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 0f;
	exit;
0:	r6 = r0;
	r1 = r0;
	r1 += 4;
	call {bpf_spin_lock};
	call {bpf_get_prandom_u32};
	r1 = r6;
	r1 += 4;
	call {bpf_spin_unlock};
	r0 = 0;
	exit;
"#,
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_lock = sym bpf_spin_lock,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
    );
}

// SEC("cgroup/skb")
// __description("spin_lock: test6 missing unlock")
// __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_spin_lock-ed region")
// __failure_unpriv __msg_unpriv("")
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spin_lock_test6_missing_unlock() {
    naked_asm!(
        r#"
	r1 = 0;
	*(u32*)(r10 - 4) = r1;
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 0f;
	exit;
0:	r6 = r0;
	r1 = r0;
	r1 += 4;
	call {bpf_spin_lock};
	r1 = r6;
	r1 += 4;
	r0 = *(u32*)(r6 + 0);
	if r0 != 0 goto 1f;
	call {bpf_spin_unlock};
1:	r0 = 0;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_lock = sym bpf_spin_lock,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
    );
}

// SEC("cgroup/skb")
// __description("spin_lock: test7 unlock without lock")
// __failure __msg("without taking a lock")
// __failure_unpriv __msg_unpriv("")
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_test7_unlock_without_lock() {
    naked_asm!(
        r#"
	r1 = 0;
	*(u32*)(r10 - 4) = r1;
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 0f;
	exit;
0:	r6 = r0;
	r1 = r0;
	r1 += 4;
	if r1 != 0 goto 1f;
	call {bpf_spin_lock};
1:	r1 = r6;
	r1 += 4;
	r0 = *(u32*)(r6 + 0);
	call {bpf_spin_unlock};
	r0 = 0;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_lock = sym bpf_spin_lock,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
    );
}

// SEC("cgroup/skb")
// __description("spin_lock: test8 double lock")
// __failure __msg("calls are not allowed")
// __failure_unpriv __msg_unpriv("")
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spin_lock_test8_double_lock() {
    naked_asm!(
        r#"
	r1 = 0;
	*(u32*)(r10 - 4) = r1;
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 0f;
	exit;
0:	r6 = r0;
	r1 = r0;
	r1 += 4;
	call {bpf_spin_lock};
	r1 = r6;
	r1 += 4;
	call {bpf_spin_lock};
	r1 = r6;
	r1 += 4;
	r0 = *(u32*)(r6 + 0);
	call {bpf_spin_unlock};
	r0 = 0;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_lock = sym bpf_spin_lock,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
    );
}

// SEC("cgroup/skb")
// __description("spin_lock: test9 different lock")
// __failure __msg("unlock of different lock")
// __failure_unpriv __msg_unpriv("")
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn spin_lock_test9_different_lock() {
    naked_asm!(
        r#"
	r1 = 0;
	*(u32*)(r10 - 4) = r1;
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 0f;
	exit;
0:	r6 = r0;
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 1f;
	exit;
1:	r7 = r0;
	r1 = r6;
	r1 += 4;
	call {bpf_spin_lock};
	r1 = r7;
	r1 += 4;
	call {bpf_spin_unlock};
	r0 = 0;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_lock = sym bpf_spin_lock,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
    );
}

// SEC("cgroup/skb")
// __description("spin_lock: test10 lock in subprog without unlock")
// __success
// __failure_unpriv __msg_unpriv("")
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lock_in_subprog_without_unlock() {
    naked_asm!(
        r#"
	r1 = 0;
	*(u32*)(r10 - 4) = r1;
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 0f;
	exit;
0:	r6 = r0;
	r1 = r0;
	r1 += 4;
	call lock_in_subprog_without_unlock__1;
	r1 = r6;
	r1 += 4;
	call {bpf_spin_unlock};
	r0 = 1;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
    );
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn lock_in_subprog_without_unlock__1() {
    naked_asm!(
        r#"
	call {bpf_spin_lock};
	r0 = 0;
	exit;
"#,
        bpf_spin_lock = sym bpf_spin_lock,
    );
}

// SEC("tc")
// __description("spin_lock: test11 ld_abs under lock")
// __failure __msg("inside bpf_spin_lock")
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test11_ld_abs_under_lock() {
    naked_asm!(
        r#"
	r6 = r1;
	r1 = 0;
	*(u32*)(r10 - 4) = r1;
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 0f;
	exit;
0:	r7 = r0;
	r1 = r0;
	r1 += 4;
	call {bpf_spin_lock};
	r0 = *(u8*)skb[0];
	r1 = r7;
	r1 += 4;
	call {bpf_spin_unlock};
	r0 = 0;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_lock = sym bpf_spin_lock,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
    );
}

// SEC("tc")
// __description("spin_lock: regsafe compare reg->id for map value")
// __failure __msg("bpf_spin_unlock of different lock")
// __flag(BPF_F_TEST_STATE_FREQ)
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reg_id_for_map_value() {
    naked_asm!(
        r#"
	r6 = r1;
	r6 = *(u32*)(r6 + {__sk_buff_mark});
	r1 = {map_spin_lock} ll;
	r9 = r1;
	r2 = 0;
	*(u32*)(r10 - 4) = r2;
	r2 = r10;
	r2 += -4;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 0f;
	exit;
0:	r7 = r0;
	r1 = r9;
	r2 = r10;
	r2 += -4;
	call {bpf_map_lookup_elem};
	if r0 != 0 goto 1f;
	exit;
1:	r8 = r0;
	r1 = r7;
	r1 += 4;
	call {bpf_spin_lock};
	if r6 == 0 goto 2f;
	goto 3f;
2:	r7 = r8;
3:	r1 = r7;
	r1 += 4;
	call {bpf_spin_unlock};
	r0 = 0;
	exit;
"#,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_lock = sym bpf_spin_lock,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
        __sk_buff_mark = const 8,
    );
}

/* Make sure that regsafe() compares ids for spin lock records using
 * check_ids():
 *  1: r9 = map_lookup_elem(...)  ; r9.id == 1
 *  2: r8 = map_lookup_elem(...)  ; r8.id == 2
 *  3: r7 = ktime_get_ns()
 *  4: r6 = ktime_get_ns()
 *  5: if r6 > r7 goto <9>
 *  6: spin_lock(r8)
 *  7: r9 = r8
 *  8: goto <10>
 *  9: spin_lock(r9)
 * 10: spin_unlock(r9)             ; r9.id == 1 || r9.id == 2 and lock is active,
 *                                 ; second visit to (10) should be considered safe
 *                                 ; if check_ids() is used.
 * 11: exit(0)
 */

// SEC("cgroup/skb")
// __description("spin_lock: regsafe() check_ids() similar id mappings")
// __success __msg("29: safe")
// __failure_unpriv __msg_unpriv("")
// __log_level(2) __retval(0) __flag(BPF_F_TEST_STATE_FREQ)
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_ids_similar_id_mappings() {
    naked_asm!(
        r#"
	r1 = 0;
	*(u32*)(r10 - 4) = r1;
	/* r9 = map_lookup_elem(...) */
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 == 0 goto 0f;
	r9 = r0;
	/* r8 = map_lookup_elem(...) */
	r2 = r10;
	r2 += -4;
	r1 = {map_spin_lock} ll;
	call {bpf_map_lookup_elem};
	if r0 == 0 goto 1f;
	r8 = r0;
	/* r7 = ktime_get_ns() */
	call {bpf_ktime_get_ns};
	r7 = r0;
	/* r6 = ktime_get_ns() */
	call {bpf_ktime_get_ns};
	r6 = r0;
	/* if r6 > r7 goto +5      ; no new information about the state is derived from
	 *                         ; this check, thus produced verifier states differ
	 *                         ; only in 'insn_idx'
	 * spin_lock(r8)
	 * r9 = r8
	 * goto unlock
	 */
	if r6 > r7 goto 2f;
	r1 = r8;
	r1 += 4;
	call {bpf_spin_lock};
	r9 = r8;
	goto 3f;
2:	/* spin_lock(r9) */
	r1 = r9;
	r1 += 4;
	call {bpf_spin_lock};
3:	/* spin_unlock(r9) */
	r1 = r9;
	r1 += 4;
	call {bpf_spin_unlock};
0:	/* exit(0) */
	r0 = 0;
1:	exit;
"#,
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        bpf_map_lookup_elem = sym bpf_map_lookup_elem,
        bpf_spin_lock = sym bpf_spin_lock,
        bpf_spin_unlock = sym bpf_spin_unlock,
        map_spin_lock = sym map_spin_lock,
    );
}

// SEC("tc")
// __description("spin_lock: loop within a locked region")
// __success __failure_unpriv __msg_unpriv("")
// __retval(0)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_loop_inside_locked_region() -> i32 {
    let zero: i32 = 0;
    let mut val: *mut val;
    let mut i: i32;
    let mut j: i32 = 0;

    val = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(map_spin_lock).cast::<core::ffi::c_void>(),
        core::ptr::addr_of!(zero).cast::<core::ffi::c_void>(),
    )
    .cast::<val>();
    if val.is_null() {
        return -1;
    }

    bpf_spin_lock(core::ptr::addr_of_mut!((*val).l));
    i = 0;
    while i < 10 {
        j += 1;
        /* Silence "unused variable" warnings. */
        if j == 10 {
            break;
        }
        i += 1;
    }
    bpf_spin_unlock(core::ptr::addr_of_mut!((*val).l));

    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
