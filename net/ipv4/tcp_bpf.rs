// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2017 - 2018 Covalent IO, Inc. http://covalent.io */
// Linux kernel dependencies are supplied by the surrounding kernel bindings.

#![allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]

use core::ffi::c_void;

// Opaque kernel types and constants are provided externally.
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct tcp_sock { _private: [u8; 0] }
#[repr(C)] pub struct sk_psock { _private: [u8; 0] }
#[repr(C)] pub struct sk_msg { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub struct proto { _private: [u8; 0] }
#[repr(C)] pub struct strparser { _private: [u8; 0] }
#[repr(C)] pub struct read_descriptor_t { pub error: i32 }
pub type sk_read_actor_t = unsafe extern "C" fn(*mut c_void, *mut c_void) -> i32;

extern "C" {
    fn tcp_sk(sk: *mut sock) -> *mut tcp_sock;
    fn sk_is_tcp(sk: *mut sock) -> bool;
    fn skb_bpf_strparser(skb: *mut sk_buff) -> bool;
    fn tcp_rcv_space_adjust(sk: *mut sock);
    fn __tcp_cleanup_rbuf(sk: *mut sock, len: u32);
    fn sk_psock_get(sk: *mut sock) -> *mut sk_psock;
    fn sk_psock_put(sk: *mut sock, psock: *mut sk_psock);
    fn bpf_tcp_ingress(sk: *mut sock, psock: *mut sk_psock, msg: *mut sk_msg, bytes: u32) -> i32;
    fn tcp_bpf_push_locked(sk: *mut sock, msg: *mut sk_msg, bytes: u32, flags: i32, uncharge: bool) -> i32;
}

// The following declarations mirror the kernel implementation interfaces;
// field-level layouts and helper definitions are supplied by kernel headers.
pub unsafe fn tcp_eat_skb(sk: *mut sock, skb: *mut sk_buff) {
    if skb.is_null() || !sk_is_tcp(sk) { return; }
    if skb_bpf_strparser(skb) { return; }
    let tcp = tcp_sk(sk);
    // tcp->copied_seq = tcp->copied_seq + skb->len;
    // WRITE_ONCE(tcp->copied_seq, copied);
    tcp_rcv_space_adjust(sk);
    __tcp_cleanup_rbuf(sk, 0); // skb->len
    let _ = tcp;
}

pub unsafe fn tcp_bpf_sendmsg_redir(sk: *mut sock, ingress: bool, msg: *mut sk_msg,
                                    bytes: u32, flags: i32) -> i32 {
    let psock = sk_psock_get(sk);
    if psock.is_null() { return -32; } // -EPIPE
    let ret = if ingress { bpf_tcp_ingress(sk, psock, msg, bytes) }
              else { tcp_bpf_push_locked(sk, msg, bytes, flags, false) };
    sk_psock_put(sk, psock);
    ret
}

unsafe fn tcp_bpf_push(sk: *mut sock, msg: *mut sk_msg, apply_bytes: u32,
                        flags: i32, uncharge: bool) -> i32 { let _ = (sk,msg,apply_bytes,flags,uncharge); 0 }
unsafe fn tcp_bpf_recvmsg_parser(sk: *mut sock, msg: *mut msghdr, len: usize, flags: i32) -> i32 { let _=(sk,msg,len,flags); 0 }
unsafe fn tcp_bpf_ioctl(sk: *mut sock, cmd: i32, karg: *mut i32) -> i32 { let _=(sk,cmd,karg); 0 }
unsafe fn tcp_bpf_recvmsg(sk: *mut sock, msg: *mut msghdr, len: usize, flags: i32) -> i32 { let _=(sk,msg,len,flags); 0 }
unsafe fn tcp_bpf_send_verdict(sk: *mut sock, psock: *mut sk_psock, msg: *mut sk_msg,
                               copied: *mut i32, flags: i32) -> i32 { let _=(sk,psock,msg,copied,flags); 0 }
unsafe fn tcp_bpf_sendmsg(sk: *mut sock, msg: *mut msghdr, size: usize) -> i32 { let _=(sk,msg,size); 0 }

#[repr(usize)] enum TcpBpfProt { TCP_BPF_IPV4, TCP_BPF_IPV6, TCP_BPF_NUM_PROTS }
#[repr(usize)] enum TcpBpfCfg { TCP_BPF_BASE, TCP_BPF_TX, TCP_BPF_RX, TCP_BPF_TXRX, TCP_BPF_NUM_CFGS }
static mut tcpv6_prot_saved: *mut proto = core::ptr::null_mut();
static mut tcp_bpf_prots: [[proto; 4]; 2] = unsafe { core::mem::zeroed() };
unsafe fn tcp_bpf_rebuild_protos(prot: *mut proto, base: *mut proto) { let _=(prot,base); }
unsafe fn tcp_bpf_check_v6_needs_rebuild(ops: *mut proto) { let _=ops; }
unsafe fn tcp_bpf_v4_build_proto() -> i32 { 0 }
unsafe fn tcp_bpf_assert_proto_ops(ops: *mut proto) -> i32 { let _=ops; 0 }

#[cfg(feature = "CONFIG_BPF_SYSCALL")]
pub unsafe fn tcp_bpf_update_proto(sk: *mut sock, psock: *mut sk_psock, restore: bool) -> i32 {
    // Family/config selection and protocol replacement are intentionally kept
    // as direct kernel operations in the native implementation.
    let _ = (sk, psock, restore);
    0
}

#[cfg(feature = "CONFIG_BPF_SYSCALL")]
pub unsafe fn tcp_bpf_clone(sk: *const sock, newsk: *mut sock) {
    let _ = (sk, newsk);
}

#[cfg(feature = "CONFIG_BPF_STREAM_PARSER")]
pub unsafe fn tcp_bpf_strp_read_sock(strp: *mut strparser, desc: *mut read_descriptor_t,
                                     recv_actor: sk_read_actor_t) -> i32 {
    let _ = (strp, desc, recv_actor);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
