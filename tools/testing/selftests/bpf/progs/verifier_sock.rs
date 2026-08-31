// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/sock.c */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, "bpf_misc.h".
// The SEC(), __description(), __success, __failure, __msg(), __retval(),
// __failure_unpriv, __success_unpriv, __naked, __noinline, __uint(), __type(),
// __imm(), __imm_const(), __imm_addr(), __clobber_all, offsetof(),
// offsetofend(), barrier_var(), and __sink() facilities are supplied by the
// surrounding BPF selftest build.

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct __sk_buff {
    pub data: u32,
    pub data_end: u32,
}

#[repr(C)]
pub struct xdp_md {
    pub data: u32,
    pub data_end: u32,
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct val {
    pub cnt: i32,
    pub l: bpf_spin_lock,
}

unsafe extern "C" {
    fn bpf_skb_pull_data(sk: *mut __sk_buff, len: __u32) -> i64;
    fn bpf_xdp_pull_data(x: *mut xdp_md, len: __u32) -> i64;
    fn bpf_tail_call_static(ctx: *mut __sk_buff, prog_array_map: *const core::ffi::c_void, index: __u32);
    fn barrier_var(ret: i32);
    fn __sink(ret: i32);
}

const TCX_DROP: i32 = 0;
const TCX_PASS: i32 = 1;
const XDP_DROP: i32 = 1;
const XDP_PASS: i32 = 2;

// Map definitions translated from anonymous C map declaration structs.
// BPF map metadata macros are preserved as comments because their Rust
// expansion is supplied outside this isolated file.
#[link_section = ".maps"]
#[no_mangle]
pub static mut map_reuseport_array: [u8; 0] = []; // type=BPF_MAP_TYPE_REUSEPORT_SOCKARRAY, max_entries=1, key=__u32, value=__u64
#[link_section = ".maps"]
#[no_mangle]
pub static mut map_sockhash: [u8; 0] = []; // type=BPF_MAP_TYPE_SOCKHASH, max_entries=1, key=int, value=int
#[link_section = ".maps"]
#[no_mangle]
pub static mut map_sockmap: [u8; 0] = []; // type=BPF_MAP_TYPE_SOCKMAP, max_entries=1, key=int, value=int
#[link_section = ".maps"]
#[no_mangle]
pub static mut map_xskmap: [u8; 0] = []; // type=BPF_MAP_TYPE_XSKMAP, max_entries=1, key=int, value=int
#[link_section = ".maps"]
#[no_mangle]
pub static mut sk_storage_map: [u8; 0] = []; // type=BPF_MAP_TYPE_SK_STORAGE, max_entries=0, key=int, value=struct val, map_flags=BPF_F_NO_PREALLOC
#[link_section = ".maps"]
#[no_mangle]
pub static mut jmp_table: [u8; 0] = []; // type=BPF_MAP_TYPE_PROG_ARRAY, max_entries=1, key_size=sizeof(__u32), value_size=sizeof(__u32)

macro_rules! bpf_asm_prog {
    ($section:literal, $description:literal, $verdict:literal, fn $name:ident, $body:literal $(,)?) => {
        #[unsafe(link_section = $section)]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name() {
            // description: $description
            // verifier expectation: $verdict
            core::arch::asm!($body, options(noreturn));
        }
    };
}

bpf_asm_prog!("cgroup/skb", "skb->sk: no NULL check", "failure: invalid mem access 'sock_common_or_null'; failure_unpriv", fn skb_sk_no_null_check,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
r0 = *(u32*)(r1 + 0);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "skb->sk: sk->family [non fullsock field]", "success; success_unpriv; retval=0", fn sk_family_non_fullsock_field_1,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: r0 = *(u32*)(r1 + %[bpf_sock_family]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "skb->sk: sk->type [fullsock field]", "failure: invalid sock_common access; failure_unpriv", fn sk_sk_type_fullsock_field_1,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: r0 = *(u32*)(r1 + %[bpf_sock_type]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "bpf_sk_fullsock(skb->sk): no !skb->sk check", "failure: type=sock_common_or_null expected=sock_common; failure_unpriv", fn sk_no_skb_sk_check_1,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
call %[bpf_sk_fullsock];
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): no NULL check on ret", "failure: invalid mem access 'sock_or_null'; failure_unpriv", fn no_null_check_on_ret_1,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
r0 = *(u32*)(r0 + %[bpf_sock_type]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): sk->type [fullsock field]", "success; success_unpriv; retval=0", fn sk_sk_type_fullsock_field_2,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r0 = *(u32*)(r0 + %[bpf_sock_type]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): sk->family [non fullsock field]", "success; success_unpriv; retval=0", fn sk_family_non_fullsock_field_2,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
exit;
l1_%=: r0 = *(u32*)(r0 + %[bpf_sock_family]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): sk->state [narrow load]", "success; success_unpriv; retval=0", fn sk_sk_state_narrow_load,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r0 = *(u8*)(r0 + %[bpf_sock_state]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): sk->dst_port [word load] (backward compatibility)", "success; success_unpriv; retval=0", fn port_word_load_backward_compatibility,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r0 = *(u32*)(r0 + %[bpf_sock_dst_port]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): sk->dst_port [half load]", "success; success_unpriv; retval=0", fn sk_dst_port_half_load,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r0 = *(u16*)(r0 + %[bpf_sock_dst_port]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): sk->dst_port [half load] (invalid)", "failure: invalid sock access; failure_unpriv", fn dst_port_half_load_invalid_1,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r0 = *(u16*)(r0 + %[__imm_0]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): sk->dst_port [byte load]", "success; success_unpriv; retval=0", fn sk_dst_port_byte_load,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r2 = *(u8*)(r0 + %[bpf_sock_dst_port]);
r2 = *(u8*)(r0 + %[__imm_0]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): sk->dst_port [byte load] (invalid)", "failure: invalid sock access; failure_unpriv", fn dst_port_byte_load_invalid,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r0 = *(u8*)(r0 + %[__imm_0]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): past sk->dst_port [half load] (invalid)", "failure: invalid sock access; failure_unpriv", fn dst_port_half_load_invalid_2,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r0 = *(u16*)(r0 + %[bpf_sock_dst_port__end]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): sk->dst_ip6 [load 2nd byte]", "success; success_unpriv; retval=0", fn dst_ip6_load_2nd_byte,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r0 = *(u8*)(r0 + %[__imm_0]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): sk->type [narrow load]", "success; success_unpriv; retval=0", fn sk_sk_type_narrow_load,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r0 = *(u8*)(r0 + %[bpf_sock_type]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): sk->protocol [narrow load]", "success; success_unpriv; retval=0", fn sk_sk_protocol_narrow_load,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r0 = *(u8*)(r0 + %[bpf_sock_protocol]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "sk_fullsock(skb->sk): beyond last field", "failure: invalid sock access; failure_unpriv", fn skb_sk_beyond_last_field_1,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r0 = *(u32*)(r0 + %[bpf_sock_rx_queue_mapping__end]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "bpf_tcp_sock(skb->sk): no !skb->sk check", "failure: type=sock_common_or_null expected=sock_common; failure_unpriv", fn sk_no_skb_sk_check_2,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
call %[bpf_tcp_sock];
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "bpf_tcp_sock(skb->sk): no NULL check on ret", "failure: invalid mem access 'tcp_sock_or_null'; failure_unpriv", fn no_null_check_on_ret_2,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_tcp_sock];
r0 = *(u32*)(r0 + %[bpf_tcp_sock_snd_cwnd]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "bpf_tcp_sock(skb->sk): tp->snd_cwnd", "success; success_unpriv; retval=0", fn skb_sk_tp_snd_cwnd_1,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_tcp_sock];
if r0 != 0 goto l1_%=;
exit;
l1_%=: r0 = *(u32*)(r0 + %[bpf_tcp_sock_snd_cwnd]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "bpf_tcp_sock(skb->sk): tp->bytes_acked", "success; success_unpriv; retval=0", fn skb_sk_tp_bytes_acked,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_tcp_sock];
if r0 != 0 goto l1_%=;
exit;
l1_%=: r0 = *(u64*)(r0 + %[bpf_tcp_sock_bytes_acked]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "bpf_tcp_sock(skb->sk): beyond last field", "failure: invalid tcp_sock access; failure_unpriv", fn skb_sk_beyond_last_field_2,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_tcp_sock];
if r0 != 0 goto l1_%=;
exit;
l1_%=: r0 = *(u64*)(r0 + %[bpf_tcp_sock_bytes_acked__end]);
r0 = 0;
exit;");

bpf_asm_prog!("cgroup/skb", "bpf_tcp_sock(bpf_sk_fullsock(skb->sk)): tp->snd_cwnd", "success; success_unpriv; retval=0", fn skb_sk_tp_snd_cwnd_2,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
exit;
l1_%=: r1 = r0;
call %[bpf_tcp_sock];
if r0 != 0 goto l2_%=;
exit;
l2_%=: r0 = *(u32*)(r0 + %[bpf_tcp_sock_snd_cwnd]);
r0 = 0;
exit;");

bpf_asm_prog!("tc", "bpf_sk_release(skb->sk)", "failure: release helper bpf_sk_release expects referenced PTR_TO_BTF_ID passed to R1", fn bpf_sk_release_skb_sk,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 == 0 goto l0_%=;
call %[bpf_sk_release];
l0_%=: r0 = 0;
exit;");

bpf_asm_prog!("tc", "bpf_sk_release(bpf_sk_fullsock(skb->sk))", "failure: release helper bpf_sk_release expects referenced PTR_TO_BTF_ID passed to R1", fn bpf_sk_fullsock_skb_sk,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
exit;
l1_%=: r1 = r0;
call %[bpf_sk_release];
r0 = 1;
exit;");

bpf_asm_prog!("tc", "bpf_sk_release(bpf_tcp_sock(skb->sk))", "failure: release helper bpf_sk_release expects referenced PTR_TO_BTF_ID passed to R1", fn bpf_tcp_sock_skb_sk,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_tcp_sock];
if r0 != 0 goto l1_%=;
exit;
l1_%=: r1 = r0;
call %[bpf_sk_release];
r0 = 1;
exit;");

bpf_asm_prog!("tc", "sk_storage_get(map, skb->sk, NULL, 0): value == NULL", "success; retval=0", fn sk_null_0_value_null,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r4 = 0;
r3 = 0;
r2 = r0;
r1 = %[sk_storage_map] ll;
call %[bpf_sk_storage_get];
r0 = 0;
exit;");

bpf_asm_prog!("tc", "sk_storage_get(map, skb->sk, 1, 1): value == 1", "failure: R3 type=scalar expected=fp", fn sk_1_1_value_1,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r4 = 1;
r3 = 1;
r2 = r0;
r1 = %[sk_storage_map] ll;
call %[bpf_sk_storage_get];
r0 = 0;
exit;");

bpf_asm_prog!("tc", "sk_storage_get(map, skb->sk, &stack_value, 1): stack_value", "success; retval=0", fn stack_value_1_stack_value,
"r2 = 0;
*(u64*)(r10 - 8) = r2;
r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: call %[bpf_sk_fullsock];
if r0 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r4 = 1;
r3 = r10;
r3 += -8;
r2 = r0;
r1 = %[sk_storage_map] ll;
call %[bpf_sk_storage_get];
r0 = 0;
exit;");

bpf_asm_prog!("tc", "bpf_map_lookup_elem(smap, &key)", "failure: cannot pass map_type 24 into func bpf_map_lookup_elem", fn map_lookup_elem_smap_key,
"r1 = 0;
*(u32*)(r10 - 4) = r1;
r2 = r10;
r2 += -4;
r1 = %[sk_storage_map] ll;
call %[bpf_map_lookup_elem];
r0 = 0;
exit;");

bpf_asm_prog!("xdp", "bpf_map_lookup_elem(xskmap, &key); xs->queue_id", "success; retval=0", fn xskmap_key_xs_queue_id,
"r1 = 0;
*(u32*)(r10 - 8) = r1;
r2 = r10;
r2 += -8;
r1 = %[map_xskmap] ll;
call %[bpf_map_lookup_elem];
if r0 != 0 goto l0_%=;
exit;
l0_%=: r0 = *(u32*)(r0 + %[bpf_xdp_sock_queue_id]);
r0 = 0;
exit;");

bpf_asm_prog!("sk_skb", "bpf_map_lookup_elem(sockmap, &key)", "failure: Unreleased reference id=2 alloc_insn=6", fn map_lookup_elem_sockmap_key,
"r1 = 0;
*(u32*)(r10 - 4) = r1;
r2 = r10;
r2 += -4;
r1 = %[map_sockmap] ll;
call %[bpf_map_lookup_elem];
r0 = 0;
exit;");

bpf_asm_prog!("sk_skb", "bpf_map_lookup_elem(sockhash, &key)", "failure: Unreleased reference id=2 alloc_insn=6", fn map_lookup_elem_sockhash_key,
"r1 = 0;
*(u32*)(r10 - 4) = r1;
r2 = r10;
r2 += -4;
r1 = %[map_sockhash] ll;
call %[bpf_map_lookup_elem];
r0 = 0;
exit;");

bpf_asm_prog!("sk_skb", "bpf_map_lookup_elem(sockmap, &key); sk->type [fullsock field]; bpf_sk_release(sk)", "success", fn field_bpf_sk_release_sk_1,
"r1 = 0;
*(u32*)(r10 - 4) = r1;
r2 = r10;
r2 += -4;
r1 = %[map_sockmap] ll;
call %[bpf_map_lookup_elem];
if r0 != 0 goto l0_%=;
exit;
l0_%=: r1 = r0;
r0 = *(u32*)(r0 + %[bpf_sock_type]);
call %[bpf_sk_release];
exit;");

bpf_asm_prog!("sk_skb", "bpf_map_lookup_elem(sockhash, &key); sk->type [fullsock field]; bpf_sk_release(sk)", "success", fn field_bpf_sk_release_sk_2,
"r1 = 0;
*(u32*)(r10 - 4) = r1;
r2 = r10;
r2 += -4;
r1 = %[map_sockhash] ll;
call %[bpf_map_lookup_elem];
if r0 != 0 goto l0_%=;
exit;
l0_%=: r1 = r0;
r0 = *(u32*)(r0 + %[bpf_sock_type]);
call %[bpf_sk_release];
exit;");

bpf_asm_prog!("sk_reuseport", "bpf_sk_select_reuseport(ctx, reuseport_array, &key, flags)", "success", fn ctx_reuseport_array_key_flags,
"r4 = 0;
r2 = 0;
*(u32*)(r10 - 4) = r2;
r3 = r10;
r3 += -4;
r2 = %[map_reuseport_array] ll;
call %[bpf_sk_select_reuseport];
exit;");

bpf_asm_prog!("sk_reuseport", "bpf_sk_select_reuseport(ctx, sockmap, &key, flags)", "success", fn reuseport_ctx_sockmap_key_flags,
"r4 = 0;
r2 = 0;
*(u32*)(r10 - 4) = r2;
r3 = r10;
r3 += -4;
r2 = %[map_sockmap] ll;
call %[bpf_sk_select_reuseport];
exit;");

bpf_asm_prog!("sk_reuseport", "bpf_sk_select_reuseport(ctx, sockhash, &key, flags)", "success", fn reuseport_ctx_sockhash_key_flags,
"r4 = 0;
r2 = 0;
*(u32*)(r10 - 4) = r2;
r3 = r10;
r3 += -4;
r2 = %[map_sockmap] ll;
call %[bpf_sk_select_reuseport];
exit;");

bpf_asm_prog!("tc", "mark null check on return value of bpf_skc_to helpers", "failure: invalid mem access", fn of_bpf_skc_to_helpers,
"r1 = *(u64*)(r1 + %[__sk_buff_sk]);
if r1 != 0 goto l0_%=;
r0 = 0;
exit;
l0_%=: r6 = r1;
call %[bpf_skc_to_tcp_sock];
r7 = r0;
r1 = r6;
call %[bpf_skc_to_tcp_request_sock];
r8 = r0;
if r8 != 0 goto l1_%=;
r0 = 0;
exit;
l1_%=: r0 = *(u8*)(r7 + 0);
exit;");

bpf_asm_prog!("cgroup/post_bind4", "sk->src_ip6[0] [load 1st byte]", "failure: invalid bpf_context access off=28 size=2", fn post_bind4_read_src_ip6,
"r6 = r1;
r7 = *(u16*)(r6 + %[bpf_sock_src_ip6_0]);
r0 = 1;
exit;");

bpf_asm_prog!("cgroup/post_bind4", "sk->mark [load mark]", "failure: invalid bpf_context access off=16 size=2", fn post_bind4_read_mark,
"r6 = r1;
r7 = *(u16*)(r6 + %[bpf_sock_mark]);
r0 = 1;
exit;");

bpf_asm_prog!("cgroup/post_bind6", "sk->src_ip4 [load src_ip4]", "failure: invalid bpf_context access off=24 size=2", fn post_bind6_read_src_ip4,
"r6 = r1;
r7 = *(u16*)(r6 + %[bpf_sock_src_ip4]);
r0 = 1;
exit;");

bpf_asm_prog!("cgroup/sock_create", "sk->src_port [word load]", "failure: invalid bpf_context access off=44 size=2", fn sock_create_read_src_port,
"r6 = r1;
r7 = *(u16*)(r6 + %[bpf_sock_src_port]);
r0 = 1;
exit;");

#[inline(never)]
pub unsafe extern "C" fn skb_pull_data2(sk: *mut __sk_buff, len: __u32) -> i64 {
    unsafe { bpf_skb_pull_data(sk, len) }
}

#[inline(never)]
pub unsafe extern "C" fn skb_pull_data1(sk: *mut __sk_buff, len: __u32) -> i64 {
    unsafe { skb_pull_data2(sk, len) }
}

/* global function calls bpf_skb_pull_data(), which invalidates packet
 * pointers established before global function call.
 */
#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn invalidate_pkt_pointers_from_global_func(sk: *mut __sk_buff) -> i32 {
    // failure: invalid mem access
    let p = unsafe { (*sk).data as usize as *mut i32 };

    if unsafe { p.add(1) as *mut core::ffi::c_void > (*sk).data_end as usize as *mut core::ffi::c_void } {
        return TCX_DROP;
    }
    unsafe { skb_pull_data1(sk, 0) };
    unsafe { *p = 42 }; /* this is unsafe */
    TCX_PASS
}

#[inline(never)]
pub unsafe extern "C" fn xdp_pull_data2(x: *mut xdp_md, len: __u32) -> i64 {
    unsafe { bpf_xdp_pull_data(x, len) }
}

#[inline(never)]
pub unsafe extern "C" fn xdp_pull_data1(x: *mut xdp_md, len: __u32) -> i64 {
    unsafe { xdp_pull_data2(x, len) }
}

/* global function calls bpf_xdp_pull_data(), which invalidates packet
 * pointers established before global function call.
 */
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn invalidate_xdp_pkt_pointers_from_global_func(x: *mut xdp_md) -> i32 {
    // failure: invalid mem access
    let p = unsafe { (*x).data as usize as *mut i32 };

    if unsafe { p.add(1) as *mut core::ffi::c_void > (*x).data_end as usize as *mut core::ffi::c_void } {
        return XDP_DROP;
    }
    unsafe { xdp_pull_data1(x, 0) };
    unsafe { *p = 42 }; /* this is unsafe */
    XDP_PASS
}

/* XDP packet changing kfunc calls invalidate packet pointers */
#[unsafe(link_section = "xdp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn invalidate_xdp_pkt_pointers(x: *mut xdp_md) -> i32 {
    // failure: invalid mem access
    let p = unsafe { (*x).data as usize as *mut i32 };

    if unsafe { p.add(1) as *mut core::ffi::c_void > (*x).data_end as usize as *mut core::ffi::c_void } {
        return XDP_DROP;
    }
    unsafe { bpf_xdp_pull_data(x, 0) };
    unsafe { *p = 42 }; /* this is unsafe */
    XDP_PASS
}

#[inline(never)]
pub unsafe extern "C" fn tail_call(sk: *mut __sk_buff) -> i32 {
    unsafe { bpf_tail_call_static(sk, core::ptr::addr_of!(jmp_table).cast(), 0) };
    0
}

#[inline(never)]
unsafe extern "C" fn static_tail_call(sk: *mut __sk_buff) -> i32 {
    let ret = 0;

    unsafe { bpf_tail_call_static(sk, core::ptr::addr_of!(jmp_table).cast(), 0) };
    unsafe { barrier_var(ret) };
    ret
}

/* Tail calls in sub-programs invalidate packet pointers. */
#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn invalidate_pkt_pointers_by_global_tail_call(sk: *mut __sk_buff) -> i32 {
    // failure: invalid mem access
    let p = unsafe { (*sk).data as usize as *mut i32 };

    if unsafe { p.add(1) as *mut core::ffi::c_void > (*sk).data_end as usize as *mut core::ffi::c_void } {
        return TCX_DROP;
    }
    unsafe { tail_call(sk) };
    unsafe { *p = 42 }; /* this is unsafe */
    TCX_PASS
}

/* Tail calls in static sub-programs invalidate packet pointers. */
#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn invalidate_pkt_pointers_by_static_tail_call(sk: *mut __sk_buff) -> i32 {
    // failure: invalid mem access
    let p = unsafe { (*sk).data as usize as *mut i32 };
    let ret: i32;

    if unsafe { p.add(1) as *mut core::ffi::c_void > (*sk).data_end as usize as *mut core::ffi::c_void } {
        return TCX_DROP;
    }
    ret = unsafe { static_tail_call(sk) };
    unsafe { __sink(ret) };
    unsafe { *p = 42 }; /* this is unsafe */
    TCX_PASS
}

/* Direct tail calls do not invalidate packet pointers. */
#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn invalidate_pkt_pointers_by_tail_call(sk: *mut __sk_buff) -> i32 {
    // success
    let p = unsafe { (*sk).data as usize as *mut i32 };

    if unsafe { p.add(1) as *mut core::ffi::c_void > (*sk).data_end as usize as *mut core::ffi::c_void } {
        return TCX_DROP;
    }
    unsafe { bpf_tail_call_static(sk, core::ptr::addr_of!(jmp_table).cast(), 0) };
    unsafe { *p = 42 }; /* this is NOT unsafe: tail calls don't return */
    TCX_PASS
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
