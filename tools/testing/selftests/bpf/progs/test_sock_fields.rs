// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

/* Rust translation of testing/selftests/bpf/progs/test_sock_fields.c.
 * Original C dependencies:
 *   <linux/bpf.h>, <netinet/in.h>, <stdbool.h>
 *   <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;

const AF_INET6: __u32 = 10;
const IPPROTO_TCP: __u32 = 6;
const BPF_ANY: __u64 = 0;
const BPF_F_NO_PREALLOC: __u32 = 1;
const BPF_MAP_TYPE_ARRAY: __u32 = 2;
const BPF_MAP_TYPE_SK_STORAGE: __u32 = 24;
const BPF_TCP_LISTEN: __u32 = 10;
const BPF_TCP_SYN_SENT: __u32 = 2;
const BPF_SK_STORAGE_GET_F_CREATE: __u64 = 1;

#[repr(u32)]
enum bpf_linum_array_idx {
    EGRESS_LINUM_IDX,
    INGRESS_LINUM_IDX,
    READ_SK_DST_PORT_LINUM_IDX,
    __NR_BPF_LINUM_ARRAY_IDX,
}

#[repr(C)]
pub struct bpf_spinlock_cnt {
    lock: bpf_spin_lock,
    cnt: __u32,
}

#[repr(C)]
pub struct tcp_sock {
    lsndtime: __u32,
}

/* Original C map metadata:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_ARRAY);
 *     __uint(max_entries, __NR_BPF_LINUM_ARRAY_IDX);
 *     __type(key, __u32);
 *     __type(value, __u32);
 * } linum_map SEC(".maps");
 */
#[no_mangle]
#[link_section = ".maps"]
pub static mut linum_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
    max_entries: bpf_linum_array_idx::__NR_BPF_LINUM_ARRAY_IDX as __u32,
    map_flags: 0,
};

/* Original C map metadata:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
 *     __uint(map_flags, BPF_F_NO_PREALLOC);
 *     __type(key, int);
 *     __type(value, struct bpf_spinlock_cnt);
 * } sk_pkt_out_cnt SEC(".maps");
 */
#[no_mangle]
#[link_section = ".maps"]
pub static mut sk_pkt_out_cnt: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<bpf_spinlock_cnt>() as __u32,
    max_entries: 0,
    map_flags: BPF_F_NO_PREALLOC,
};

/* Original C map metadata:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_SK_STORAGE);
 *     __uint(map_flags, BPF_F_NO_PREALLOC);
 *     __type(key, int);
 *     __type(value, struct bpf_spinlock_cnt);
 * } sk_pkt_out_cnt10 SEC(".maps");
 */
#[no_mangle]
#[link_section = ".maps"]
pub static mut sk_pkt_out_cnt10: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    key_size: core::mem::size_of::<i32>() as __u32,
    value_size: core::mem::size_of::<bpf_spinlock_cnt>() as __u32,
    max_entries: 0,
    map_flags: BPF_F_NO_PREALLOC,
};

#[no_mangle]
pub static mut listen_tp: bpf_tcp_sock = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut srv_sa6: sockaddr_in6 = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut cli_tp: bpf_tcp_sock = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut srv_tp: bpf_tcp_sock = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut listen_sk: bpf_sock = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut srv_sk: bpf_sock = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut cli_sk: bpf_sock = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut parent_cg_id: __u64 = 0;
#[no_mangle]
pub static mut child_cg_id: __u64 = 0;
#[no_mangle]
pub static mut lsndtime: __u64 = 0;

unsafe extern "C" {
    fn bpf_htonl(x: __u32) -> __u32;
    fn bpf_htons(x: __u16) -> __u16;
    fn bpf_ntohs(x: __u16) -> __u16;
    fn bpf_map_update_elem(
        map: *mut bpf_map_def,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
    fn bpf_sk_fullsock(sk: *mut bpf_sock) -> *mut bpf_sock;
    fn bpf_tcp_sock(sk: *mut bpf_sock) -> *mut bpf_tcp_sock;
    fn bpf_skc_to_tcp_sock(sk: *mut bpf_sock) -> *mut tcp_sock;
    fn bpf_sk_cgroup_id(sk: *mut tcp_sock) -> __u64;
    fn bpf_sk_ancestor_cgroup_id(sk: *mut tcp_sock, ancestor_level: i32) -> __u64;
    fn bpf_sk_storage_get(
        map: *mut bpf_map_def,
        sk: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
        flags: __u64,
    ) -> *mut core::ffi::c_void;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
}

unsafe fn is_loopback6(a6: *mut __u32) -> bool {
    unsafe {
        *a6.add(0) == 0 && *a6.add(1) == 0 && *a6.add(2) == 0 && *a6.add(3) == bpf_htonl(1)
    }
}

unsafe fn skcpy(dst: *mut bpf_sock, src: *const bpf_sock) {
    unsafe {
        (*dst).bound_dev_if = (*src).bound_dev_if;
        (*dst).family = (*src).family;
        (*dst).type_ = (*src).type_;
        (*dst).protocol = (*src).protocol;
        (*dst).mark = (*src).mark;
        (*dst).priority = (*src).priority;
        (*dst).src_ip4 = (*src).src_ip4;
        (*dst).src_ip6[0] = (*src).src_ip6[0];
        (*dst).src_ip6[1] = (*src).src_ip6[1];
        (*dst).src_ip6[2] = (*src).src_ip6[2];
        (*dst).src_ip6[3] = (*src).src_ip6[3];
        (*dst).src_port = (*src).src_port;
        (*dst).dst_ip4 = (*src).dst_ip4;
        (*dst).dst_ip6[0] = (*src).dst_ip6[0];
        (*dst).dst_ip6[1] = (*src).dst_ip6[1];
        (*dst).dst_ip6[2] = (*src).dst_ip6[2];
        (*dst).dst_ip6[3] = (*src).dst_ip6[3];
        (*dst).dst_port = (*src).dst_port;
        (*dst).state = (*src).state;
    }
}

unsafe fn tpcpy(dst: *mut bpf_tcp_sock, src: *const bpf_tcp_sock) {
    unsafe {
        (*dst).snd_cwnd = (*src).snd_cwnd;
        (*dst).srtt_us = (*src).srtt_us;
        (*dst).rtt_min = (*src).rtt_min;
        (*dst).snd_ssthresh = (*src).snd_ssthresh;
        (*dst).rcv_nxt = (*src).rcv_nxt;
        (*dst).snd_nxt = (*src).snd_nxt;
        (*dst).snd_una = (*src).snd_una;
        (*dst).mss_cache = (*src).mss_cache;
        (*dst).ecn_flags = (*src).ecn_flags;
        (*dst).rate_delivered = (*src).rate_delivered;
        (*dst).rate_interval_us = (*src).rate_interval_us;
        (*dst).packets_out = (*src).packets_out;
        (*dst).retrans_out = (*src).retrans_out;
        (*dst).total_retrans = (*src).total_retrans;
        (*dst).segs_in = (*src).segs_in;
        (*dst).data_segs_in = (*src).data_segs_in;
        (*dst).segs_out = (*src).segs_out;
        (*dst).data_segs_out = (*src).data_segs_out;
        (*dst).lost_out = (*src).lost_out;
        (*dst).sacked_out = (*src).sacked_out;
        (*dst).bytes_received = (*src).bytes_received;
        (*dst).bytes_acked = (*src).bytes_acked;
    }
}

/* Always return CG_OK so that no pkt will be filtered out */
const CG_OK: i32 = 1;

unsafe fn ret_log(linum_idx: __u32, linum: __u32) -> i32 {
    unsafe {
        bpf_map_update_elem(
            &raw mut linum_map,
            &linum_idx as *const _ as *const core::ffi::c_void,
            &linum as *const _ as *const core::ffi::c_void,
            BPF_ANY,
        );
    }
    CG_OK
}

#[no_mangle]
#[link_section = "cgroup_skb/egress"]
pub unsafe extern "C" fn egress_read_sock_fields(skb: *mut __sk_buff) -> i32 {
    unsafe {
        let mut cli_cnt_init = bpf_spinlock_cnt {
            lock: core::mem::zeroed(),
            cnt: 0xeB9F,
        };
        let mut pkt_out_cnt: *mut bpf_spinlock_cnt;
        let mut pkt_out_cnt10: *mut bpf_spinlock_cnt;
        let mut tp: *mut bpf_tcp_sock;
        let mut tp_ret: *mut bpf_tcp_sock;
        let mut sk: *mut bpf_sock;
        let mut sk_ret: *mut bpf_sock;
        let linum_idx: __u32;
        let mut ktp: *mut tcp_sock;

        linum_idx = bpf_linum_array_idx::EGRESS_LINUM_IDX as __u32;

        sk = (*skb).sk;
        if sk.is_null() {
            return ret_log(linum_idx, line!());
        }

        /* Not testing the egress traffic or the listening socket,
         * which are covered by the cgroup_skb/ingress test program.
         */
        if (*sk).family != AF_INET6 || !is_loopback6((*sk).src_ip6.as_mut_ptr()) || (*sk).state == BPF_TCP_LISTEN {
            return CG_OK;
        }

        if (*sk).src_port == bpf_ntohs(srv_sa6.sin6_port) {
            /* Server socket */
            sk_ret = &raw mut srv_sk;
            tp_ret = &raw mut srv_tp;
        } else if (*sk).dst_port == srv_sa6.sin6_port {
            /* Client socket */
            sk_ret = &raw mut cli_sk;
            tp_ret = &raw mut cli_tp;
        } else {
            /* Not the testing egress traffic */
            return CG_OK;
        }

        /* It must be a fullsock for cgroup_skb/egress prog */
        sk = bpf_sk_fullsock(sk);
        if sk.is_null() {
            return ret_log(linum_idx, line!());
        }

        /* Not the testing egress traffic */
        if (*sk).protocol != IPPROTO_TCP {
            return CG_OK;
        }

        tp = bpf_tcp_sock(sk);
        if tp.is_null() {
            return ret_log(linum_idx, line!());
        }

        skcpy(sk_ret, sk);
        tpcpy(tp_ret, tp);

        if sk_ret == &raw mut srv_sk {
            ktp = bpf_skc_to_tcp_sock(sk);

            if ktp.is_null() {
                return ret_log(linum_idx, line!());
            }

            lsndtime = (*ktp).lsndtime as __u64;

            child_cg_id = bpf_sk_cgroup_id(ktp);
            if child_cg_id == 0 {
                return ret_log(linum_idx, line!());
            }

            parent_cg_id = bpf_sk_ancestor_cgroup_id(ktp, 2);
            if parent_cg_id == 0 {
                return ret_log(linum_idx, line!());
            }

            /* The userspace has created it for srv sk */
            pkt_out_cnt = bpf_sk_storage_get(
                &raw mut sk_pkt_out_cnt,
                ktp as *mut core::ffi::c_void,
                core::ptr::null_mut(),
                0,
            ) as *mut bpf_spinlock_cnt;
            pkt_out_cnt10 = bpf_sk_storage_get(
                &raw mut sk_pkt_out_cnt10,
                ktp as *mut core::ffi::c_void,
                core::ptr::null_mut(),
                0,
            ) as *mut bpf_spinlock_cnt;
        } else {
            pkt_out_cnt = bpf_sk_storage_get(
                &raw mut sk_pkt_out_cnt,
                sk as *mut core::ffi::c_void,
                &mut cli_cnt_init as *mut _ as *mut core::ffi::c_void,
                BPF_SK_STORAGE_GET_F_CREATE,
            ) as *mut bpf_spinlock_cnt;
            pkt_out_cnt10 = bpf_sk_storage_get(
                &raw mut sk_pkt_out_cnt10,
                sk as *mut core::ffi::c_void,
                &mut cli_cnt_init as *mut _ as *mut core::ffi::c_void,
                BPF_SK_STORAGE_GET_F_CREATE,
            ) as *mut bpf_spinlock_cnt;
        }

        if pkt_out_cnt.is_null() || pkt_out_cnt10.is_null() {
            return ret_log(linum_idx, line!());
        }

        /* Even both cnt and cnt10 have lock defined in their BTF,
         * intentionally one cnt takes lock while one does not
         * as a test for the spinlock support in BPF_MAP_TYPE_SK_STORAGE.
         */
        (*pkt_out_cnt).cnt = (*pkt_out_cnt).cnt.wrapping_add(1);
        bpf_spin_lock(&mut (*pkt_out_cnt10).lock);
        (*pkt_out_cnt10).cnt = (*pkt_out_cnt10).cnt.wrapping_add(10);
        bpf_spin_unlock(&mut (*pkt_out_cnt10).lock);

        CG_OK
    }
}

#[no_mangle]
#[link_section = "cgroup_skb/ingress"]
pub unsafe extern "C" fn ingress_read_sock_fields(skb: *mut __sk_buff) -> i32 {
    unsafe {
        let mut tp: *mut bpf_tcp_sock;
        let linum_idx: __u32;
        let mut sk: *mut bpf_sock;

        linum_idx = bpf_linum_array_idx::INGRESS_LINUM_IDX as __u32;

        sk = (*skb).sk;
        if sk.is_null() {
            return ret_log(linum_idx, line!());
        }

        /* Not the testing ingress traffic to the server */
        if (*sk).family != AF_INET6
            || !is_loopback6((*sk).src_ip6.as_mut_ptr())
            || (*sk).src_port != bpf_ntohs(srv_sa6.sin6_port)
        {
            return CG_OK;
        }

        /* Only interested in the listening socket */
        if (*sk).state != BPF_TCP_LISTEN {
            return CG_OK;
        }

        /* It must be a fullsock for cgroup_skb/ingress prog */
        sk = bpf_sk_fullsock(sk);
        if sk.is_null() {
            return ret_log(linum_idx, line!());
        }

        tp = bpf_tcp_sock(sk);
        if tp.is_null() {
            return ret_log(linum_idx, line!());
        }

        skcpy(&raw mut listen_sk, sk);
        tpcpy(&raw mut listen_tp, tp);

        CG_OK
    }
}

/*
 * NOTE: 4-byte load from bpf_sock at dst_port offset is quirky. It
 * gets rewritten by the access converter to a 2-byte load for
 * backward compatibility. Treating the load result as a be16 value
 * makes the code portable across little- and big-endian platforms.
 */
#[inline(never)]
unsafe fn sk_dst_port__load_word(sk: *mut bpf_sock) -> bool {
    unsafe {
        let word = &raw mut (*sk).dst_port as *mut __u16 as *mut __u32;
        *word.add(0) == bpf_htons(0xcafe) as __u32
    }
}

#[inline(never)]
unsafe fn sk_dst_port__load_half(sk: *mut bpf_sock) -> bool {
    unsafe {
        core::arch::asm!("", options(nomem, nostack, preserves_flags));
        let half = &raw mut (*sk).dst_port as *mut __u16;
        *half.add(0) == bpf_htons(0xcafe)
    }
}

#[inline(never)]
unsafe fn sk_dst_port__load_byte(sk: *mut bpf_sock) -> bool {
    unsafe {
        let byte = &raw mut (*sk).dst_port as *mut __u16 as *mut __u8;
        *byte.add(0) == 0xca && *byte.add(1) == 0xfe
    }
}

#[no_mangle]
#[link_section = "cgroup_skb/egress"]
pub unsafe extern "C" fn read_sk_dst_port(skb: *mut __sk_buff) -> i32 {
    unsafe {
        let linum_idx: __u32;
        let mut sk: *mut bpf_sock;

        linum_idx = bpf_linum_array_idx::READ_SK_DST_PORT_LINUM_IDX as __u32;

        sk = (*skb).sk;
        if sk.is_null() {
            return ret_log(linum_idx, line!());
        }

        /* Ignore everything but the SYN from the client socket */
        if (*sk).state != BPF_TCP_SYN_SENT {
            return CG_OK;
        }

        if !sk_dst_port__load_word(sk) {
            return ret_log(linum_idx, line!());
        }
        if !sk_dst_port__load_half(sk) {
            return ret_log(linum_idx, line!());
        }
        if !sk_dst_port__load_byte(sk) {
            return ret_log(linum_idx, line!());
        }

        CG_OK
    }
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
