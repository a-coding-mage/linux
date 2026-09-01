// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/jeq_infer_not_null.c */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

type c_int = i32;

unsafe extern "C" {
    static bpf_sk_fullsock: usize;
    static bpf_map_lookup_elem: usize;
}

// The original C declaration uses BPF helper macros:
//
// struct {
//      __uint(type, BPF_MAP_TYPE_XSKMAP);
//      __uint(max_entries, 1);
//      __type(key, int);
//      __type(value, int);
// } map_xskmap SEC(".maps");
#[repr(C)]
pub struct map_xskmap_def {
    pub type_: *mut u32,
    pub max_entries: *mut u32,
    pub key: *mut c_int,
    pub value: *mut c_int,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut map_xskmap: map_xskmap_def = map_xskmap_def {
    type_: core::ptr::null_mut(),
    max_entries: core::ptr::null_mut(),
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
};

/* This is equivalent to the following program:
 *
 *   r6 = skb->sk;
 *   r7 = sk_fullsock(r6);
 *   r0 = sk_fullsock(r6);
 *   if (r0 == 0) return 0;    (a)
 *   if (r0 != r7) return 0;   (b)
 *   *r7->type;                (c)
 *   return 0;
 *
 * It is safe to dereference r7 at point (c), because of (a) and (b).
 * The test verifies that relation r0 == r7 is propagated from (b) to (c).
 */
// SEC("cgroup/skb")
// __description("jne/jeq infer not null, PTR_TO_SOCKET_OR_NULL -> PTR_TO_SOCKET for JNE false branch")
// __success __failure_unpriv __msg_unpriv("R7 pointer comparison")
// __retval(0)
#[unsafe(link_section = "cgroup/skb")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn socket_for_jne_false_branch() {
    unsafe {
        core::arch::asm!(
            "/* r6 = skb->sk; */",
            "r6 = *(u64*)(r1 + {__sk_buff_sk});",
            "/* if (r6 == 0) return 0; */",
            "if r6 == 0 goto 0f;",
            "/* r7 = sk_fullsock(skb); */",
            "r1 = r6;",
            "call {bpf_sk_fullsock};",
            "r7 = r0;",
            "/* r0 = sk_fullsock(skb); */",
            "r1 = r6;",
            "call {bpf_sk_fullsock};",
            "/* if (r0 == null) return 0; */",
            "if r0 == 0 goto 0f;",
            "/* if (r0 == r7) r0 = *(r7->type); */",
            "if r0 != r7 goto 0f;",
            "r0 = *(u32*)(r7 + {bpf_sock_type});",
            "0:",
            "/* return 0 */",
            "r0 = 0;",
            "exit;",
            bpf_sk_fullsock = sym bpf_sk_fullsock,
            __sk_buff_sk = const 0,
            bpf_sock_type = const 0,
            options(noreturn)
        );
    }
}

/* Same as above, but verify that another branch of JNE still
 * prohibits access to PTR_MAYBE_NULL.
 */
// SEC("cgroup/skb")
// __description("jne/jeq infer not null, PTR_TO_SOCKET_OR_NULL unchanged for JNE true branch")
// __failure __msg("R7 invalid mem access 'sock_or_null'")
// __failure_unpriv __msg_unpriv("R7 pointer comparison")
#[unsafe(link_section = "cgroup/skb")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unchanged_for_jne_true_branch() {
    unsafe {
        core::arch::asm!(
            "/* r6 = skb->sk */",
            "r6 = *(u64*)(r1 + {__sk_buff_sk});",
            "/* if (r6 == 0) return 0; */",
            "if r6 == 0 goto 0f;",
            "/* r7 = sk_fullsock(skb); */",
            "r1 = r6;",
            "call {bpf_sk_fullsock};",
            "r7 = r0;",
            "/* r0 = sk_fullsock(skb); */",
            "r1 = r6;",
            "call {bpf_sk_fullsock};",
            "/* if (r0 == null) return 0; */",
            "if r0 != 0 goto 0f;",
            "/* if (r0 == r7) return 0; */",
            "if r0 != r7 goto 1f;",
            "goto 0f;",
            "1:",
            "/* r0 = *(r7->type); */",
            "r0 = *(u32*)(r7 + {bpf_sock_type});",
            "0:",
            "/* return 0 */",
            "r0 = 0;",
            "exit;",
            bpf_sk_fullsock = sym bpf_sk_fullsock,
            __sk_buff_sk = const 0,
            bpf_sock_type = const 0,
            options(noreturn)
        );
    }
}

/* Same as a first test, but not null should be inferred for JEQ branch */
// SEC("cgroup/skb")
// __description("jne/jeq infer not null, PTR_TO_SOCKET_OR_NULL -> PTR_TO_SOCKET for JEQ true branch")
// __success __failure_unpriv __msg_unpriv("R7 pointer comparison")
// __retval(0)
#[unsafe(link_section = "cgroup/skb")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn socket_for_jeq_true_branch() {
    unsafe {
        core::arch::asm!(
            "/* r6 = skb->sk; */",
            "r6 = *(u64*)(r1 + {__sk_buff_sk});",
            "/* if (r6 == null) return 0; */",
            "if r6 == 0 goto 0f;",
            "/* r7 = sk_fullsock(skb); */",
            "r1 = r6;",
            "call {bpf_sk_fullsock};",
            "r7 = r0;",
            "/* r0 = sk_fullsock(skb); */",
            "r1 = r6;",
            "call {bpf_sk_fullsock};",
            "/* if (r0 == null) return 0; */",
            "if r0 == 0 goto 0f;",
            "/* if (r0 != r7) return 0; */",
            "if r0 == r7 goto 1f;",
            "goto 0f;",
            "1:",
            "/* r0 = *(r7->type); */",
            "r0 = *(u32*)(r7 + {bpf_sock_type});",
            "0:",
            "/* return 0; */",
            "r0 = 0;",
            "exit;",
            bpf_sk_fullsock = sym bpf_sk_fullsock,
            __sk_buff_sk = const 0,
            bpf_sock_type = const 0,
            options(noreturn)
        );
    }
}

/* Same as above, but verify that another branch of JNE still
 * prohibits access to PTR_MAYBE_NULL.
 */
// SEC("cgroup/skb")
// __description("jne/jeq infer not null, PTR_TO_SOCKET_OR_NULL unchanged for JEQ false branch")
// __failure __msg("R7 invalid mem access 'sock_or_null'")
// __failure_unpriv __msg_unpriv("R7 pointer comparison")
#[unsafe(link_section = "cgroup/skb")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unchanged_for_jeq_false_branch() {
    unsafe {
        core::arch::asm!(
            "/* r6 = skb->sk; */",
            "r6 = *(u64*)(r1 + {__sk_buff_sk});",
            "/* if (r6 == null) return 0; */",
            "if r6 == 0 goto 0f;",
            "/* r7 = sk_fullsock(skb); */",
            "r1 = r6;",
            "call {bpf_sk_fullsock};",
            "r7 = r0;",
            "/* r0 = sk_fullsock(skb); */",
            "r1 = r6;",
            "call {bpf_sk_fullsock};",
            "/* if (r0 == null) return 0; */",
            "if r0 == 0 goto 0f;",
            "/* if (r0 != r7) r0 = *(r7->type); */",
            "if r0 == r7 goto 0f;",
            "r0 = *(u32*)(r7 + {bpf_sock_type});",
            "0:",
            "/* return 0; */",
            "r0 = 0;",
            "exit;",
            bpf_sk_fullsock = sym bpf_sk_fullsock,
            __sk_buff_sk = const 0,
            bpf_sock_type = const 0,
            options(noreturn)
        );
    }
}

/* Maps are treated in a different branch of `mark_ptr_not_null_reg`,
 * so separate test for maps case.
 */
// SEC("xdp")
// __description("jne/jeq infer not null, PTR_TO_MAP_VALUE_OR_NULL -> PTR_TO_MAP_VALUE")
// __success __retval(0)
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn null_ptr_to_map_value() {
    unsafe {
        core::arch::asm!(
            "/* r9 = &some stack to use as key */",
            "r1 = 0;",
            "*(u32*)(r10 - 8) = r1;",
            "r9 = r10;",
            "r9 += -8;",
            "/* r8 = process local map */",
            "r8 = {map_xskmap} ll;",
            "/* r6 = map_lookup_elem(r8, r9); */",
            "r1 = r8;",
            "r2 = r9;",
            "call {bpf_map_lookup_elem};",
            "r6 = r0;",
            "/* r7 = map_lookup_elem(r8, r9); */",
            "r1 = r8;",
            "r2 = r9;",
            "call {bpf_map_lookup_elem};",
            "r7 = r0;",
            "/* if (r6 == 0) return 0; */",
            "if r6 == 0 goto 0f;",
            "/* if (r6 != r7) return 0; */",
            "if r6 != r7 goto 0f;",
            "/* read *r7; */",
            "r0 = *(u32*)(r7 + {bpf_xdp_sock_queue_id});",
            "0:",
            "/* return 0; */",
            "r0 = 0;",
            "exit;",
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_xskmap = sym map_xskmap,
            bpf_xdp_sock_queue_id = const 0,
            options(noreturn)
        );
    }
}

/* Verified that we can detect the pointer as non_null when comparing with
 * register with value 0. JEQ test case.
 */
// SEC("xdp")
// __success __log_level(2)
/* to make sure the branch is not falsely predicted*/
// __msg("r0 = *(u32 *)(r0 +0)")
// __msg("from 7 to 9")
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jeq_reg_reg_null_check() {
    unsafe {
        core::arch::asm!(
            "*(u32*)(r10 - 8) = 0;",
            "r1 = {map_xskmap} ll;",
            "r2 = r10;",
            "r2 += -8;",
            "call {bpf_map_lookup_elem};",
            "r1 = 0;",
            "if r0 == r1 goto 1f;",
            "r0 = *(u32*)(r0 +0);",
            "1:",
            "r0 = 0;",
            "exit;",
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_xskmap = sym map_xskmap,
            options(noreturn)
        );
    }
}

/* Same as above but for JNE.
 */
// SEC("xdp")
// __success __log_level(2)
/* to make sure the branch is not falsely predicted*/
// __msg("r0 = *(u32 *)(r0 +0)")
// __msg("from 7 to 9")
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jne_reg_reg_null_check() {
    unsafe {
        core::arch::asm!(
            "*(u32*)(r10 - 8) = 0;",
            "r1 = {map_xskmap} ll;",
            "r2 = r10;",
            "r2 += -8;",
            "call {bpf_map_lookup_elem};",
            "r1 = 0;",
            "if r0 != r1 goto 1f;",
            "goto 2f;",
            "1:",
            "r0 = *(u32*)(r0 +0);",
            "2:",
            "r0 = 0;",
            "exit;",
            bpf_map_lookup_elem = sym bpf_map_lookup_elem,
            map_xskmap = sym map_xskmap,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
