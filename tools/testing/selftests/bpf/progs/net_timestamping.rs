// Translated from testing/selftests/bpf/progs/net_timestamping.c
// Original includes:
//   "vmlinux.h"
//   "bpf_tracing_net.h"
//   <bpf/bpf_helpers.h>
//   <bpf/bpf_tracing.h>
//   "bpf_misc.h"
//   "bpf_kfuncs.h"
//   <errno.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type size_t = usize;

const EOPNOTSUPP: i32 = 95;
const SK_BPF_CB_TX_TIMESTAMPING: i32 = 1;
const SK_BPF_CB_FLAGS: i32 = 0;
const SOL_SOCKET: i32 = 1;
const BPF_MAP_TYPE_SK_STORAGE: u32 = 24;
const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_F_NO_PREALLOC: u32 = 1;
const BPF_ANY: u64 = 0;
const BPF_SK_STORAGE_GET_F_CREATE: u64 = 1;
const BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB: i32 = 4;
const BPF_SOCK_OPS_TSTAMP_SENDMSG_CB: i32 = 18;
const BPF_SOCK_OPS_TSTAMP_SCHED_CB: i32 = 19;
const BPF_SOCK_OPS_TSTAMP_SND_SW_CB: i32 = 20;
const BPF_SOCK_OPS_TSTAMP_ACK_CB: i32 = 21;

#[repr(C)]
pub struct sock {
    pub sk_bpf_cb_flags: u32,
}

#[repr(C)]
pub struct msghdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock_ops {
    pub op: i32,
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct bpf_sock_ops_kern {
    pub skb: *mut sk_buff,
}

#[repr(C)]
pub struct sk_buff {
    pub head: *mut u8,
    pub end: u32,
}

#[repr(C)]
pub struct skb_shared_info {
    pub tskey: u32,
}

#[no_mangle]
pub static mut monitored_pid: __u32 = 0;

#[no_mangle]
pub static mut nr_active: i32 = 0;
#[no_mangle]
pub static mut nr_snd: i32 = 0;
#[no_mangle]
pub static mut nr_passive: i32 = 0;
#[no_mangle]
pub static mut nr_sched: i32 = 0;
#[no_mangle]
pub static mut nr_txsw: i32 = 0;
#[no_mangle]
pub static mut nr_ack: i32 = 0;

#[repr(C)]
pub struct sk_stg {
    pub sendmsg_ns: __u64, /* record ts when sendmsg is called */
}

#[repr(C)]
pub struct sk_tskey {
    pub cookie: u64,
    pub tskey: u32,
}

#[repr(C)]
pub struct delay_info {
    pub sendmsg_ns: u64,  /* record ts when sendmsg is called */
    pub sched_delay: u32, /* SCHED_CB - sendmsg_ns */
    pub snd_sw_delay: u32, /* SND_SW_CB - SCHED_CB */
    pub ack_delay: u32,   /* ACK_CB - SND_SW_CB */
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub map_flags: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

// SEC(".maps")
#[no_mangle]
pub static mut sk_stg_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<sk_stg>() as u32,
    max_entries: 0,
};

// SEC(".maps")
#[no_mangle]
pub static mut time_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    map_flags: 0,
    key_size: core::mem::size_of::<sk_tskey>() as u32,
    value_size: core::mem::size_of::<delay_info>() as u32,
    max_entries: 1024,
};

static mut delay_tolerance_nsec: u64 = 10000000000; /* 10 second as an example */

extern "C" {
    #[link_name = "bpf_sock_ops_enable_tx_tstamp"]
    fn bpf_sock_ops_enable_tx_tstamp(skops: *mut bpf_sock_ops_kern, flags: u64) -> i32;

    fn bpf_setsockopt(
        ctx: *mut core::ffi::c_void,
        level: i32,
        optname: i32,
        optval: *const core::ffi::c_void,
        optlen: u32,
    ) -> i32;
    fn bpf_getsockopt(
        ctx: *mut core::ffi::c_void,
        level: i32,
        optname: i32,
        optval: *mut core::ffi::c_void,
        optlen: u32,
    ) -> i32;
    fn bpf_load_hdr_opt(
        skops: *mut bpf_sock_ops,
        searchby_res: *mut core::ffi::c_void,
        len: u32,
        flags: u64,
    ) -> i32;
    fn bpf_sock_ops_cb_flags_set(skops: *mut bpf_sock_ops, argval: i32) -> i32;
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_cast_to_kern_ctx(skops: *mut bpf_sock_ops) -> *mut bpf_sock_ops_kern;
    fn bpf_get_socket_cookie(ctx: *mut core::ffi::c_void) -> u64;
    fn bpf_sk_storage_get(
        map: *mut core::ffi::c_void,
        sk: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i32;
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_map_delete_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> i32;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_skc_to_tcp_sock(sk: *mut bpf_sock) -> *mut core::ffi::c_void;
}

unsafe fn bpf_test_sockopt(ctx: *mut core::ffi::c_void, _sk: *const sock, expected: i32) -> i32 {
    let mut tmp: i32 = 0;
    let mut new: i32 = SK_BPF_CB_TX_TIMESTAMPING;
    let opt: i32 = SK_BPF_CB_FLAGS;
    let level: i32 = SOL_SOCKET;

    if bpf_setsockopt(
        ctx,
        level,
        opt,
        &mut new as *mut _ as *const core::ffi::c_void,
        core::mem::size_of_val(&new) as u32,
    ) != expected
    {
        return 1;
    }

    if bpf_getsockopt(
        ctx,
        level,
        opt,
        &mut tmp as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of_val(&tmp) as u32,
    ) != expected
        || (expected == 0 && tmp != new)
    {
        return 1;
    }

    0
}

unsafe fn bpf_test_access_sockopt(ctx: *mut core::ffi::c_void, sk: *const sock) -> bool {
    if bpf_test_sockopt(ctx, sk, -EOPNOTSUPP) != 0 {
        return true;
    }
    false
}

unsafe fn bpf_test_access_load_hdr_opt(skops: *mut bpf_sock_ops) -> bool {
    let mut opt: [u8; 3] = [0; 3];
    let load_flags: i32 = 0;
    let ret: i32;

    ret = bpf_load_hdr_opt(
        skops,
        opt.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&opt) as u32,
        load_flags as u64,
    );
    if ret != -EOPNOTSUPP {
        return true;
    }

    false
}

unsafe fn bpf_test_access_cb_flags_set(skops: *mut bpf_sock_ops) -> bool {
    let ret: i32;

    ret = bpf_sock_ops_cb_flags_set(skops, 0);
    if ret != -EOPNOTSUPP {
        return true;
    }

    false
}

/* In the timestamping callbacks, we're not allowed to call the following
 * BPF CALLs for the safety concern. Return false if expected.
 */
unsafe fn bpf_test_access_bpf_calls(skops: *mut bpf_sock_ops, sk: *const sock) -> bool {
    if bpf_test_access_sockopt(skops as *mut core::ffi::c_void, sk) {
        return true;
    }

    if bpf_test_access_load_hdr_opt(skops) {
        return true;
    }

    if bpf_test_access_cb_flags_set(skops) {
        return true;
    }

    false
}

unsafe fn bpf_test_delay(skops: *mut bpf_sock_ops, sk: *const sock) -> bool {
    let mut skops_kern: *mut bpf_sock_ops_kern;
    let timestamp: u64 = bpf_ktime_get_ns();
    let mut shinfo: *mut skb_shared_info;
    let mut dinfo: delay_info = delay_info {
        sendmsg_ns: 0,
        sched_delay: 0,
        snd_sw_delay: 0,
        ack_delay: 0,
    };
    let mut key: sk_tskey = sk_tskey { cookie: 0, tskey: 0 };
    let mut val: *mut delay_info;
    let mut skb: *mut sk_buff;
    let mut stg: *mut sk_stg;
    let prior_ts: u64;
    let delay: u64;

    if bpf_test_access_bpf_calls(skops, sk) {
        return false;
    }

    skops_kern = bpf_cast_to_kern_ctx(skops);
    skb = (*skops_kern).skb;
    shinfo = (*skb).head.add((*skb).end as usize) as *mut skb_shared_info;

    key.cookie = bpf_get_socket_cookie(skops as *mut core::ffi::c_void);
    if key.cookie == 0 {
        return false;
    }

    if (*skops).op == BPF_SOCK_OPS_TSTAMP_SENDMSG_CB {
        stg = bpf_sk_storage_get(
            &mut sk_stg_map as *mut _ as *mut core::ffi::c_void,
            sk as *mut core::ffi::c_void,
            core::ptr::null_mut(),
            0,
        ) as *mut sk_stg;
        if stg.is_null() {
            return false;
        }
        dinfo.sendmsg_ns = (*stg).sendmsg_ns;
        bpf_sock_ops_enable_tx_tstamp(skops_kern, 0);
        key.tskey = (*shinfo).tskey;
        if key.tskey == 0 {
            return false;
        }
        bpf_map_update_elem(
            &mut time_map as *mut _ as *mut core::ffi::c_void,
            &key as *const _ as *const core::ffi::c_void,
            &dinfo as *const _ as *const core::ffi::c_void,
            BPF_ANY,
        );
        return true;
    }

    key.tskey = (*shinfo).tskey;
    if key.tskey == 0 {
        return false;
    }

    val = bpf_map_lookup_elem(
        &mut time_map as *mut _ as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut delay_info;
    if val.is_null() {
        return false;
    }

    match (*skops).op {
        BPF_SOCK_OPS_TSTAMP_SCHED_CB => {
            (*val).sched_delay = timestamp.wrapping_sub((*val).sendmsg_ns) as u32;
            delay = (*val).sched_delay as u64;
        }
        BPF_SOCK_OPS_TSTAMP_SND_SW_CB => {
            prior_ts = ((*val).sched_delay as u64).wrapping_add((*val).sendmsg_ns);
            (*val).snd_sw_delay = timestamp.wrapping_sub(prior_ts) as u32;
            delay = (*val).snd_sw_delay as u64;
        }
        BPF_SOCK_OPS_TSTAMP_ACK_CB => {
            prior_ts = ((*val).snd_sw_delay as u64)
                .wrapping_add((*val).sched_delay as u64)
                .wrapping_add((*val).sendmsg_ns);
            (*val).ack_delay = timestamp.wrapping_sub(prior_ts) as u32;
            delay = (*val).ack_delay as u64;
        }
        _ => {
            delay = 0;
        }
    }

    if delay >= delay_tolerance_nsec {
        return false;
    }

    /* Since it's the last one, remove from the map after latency check */
    if (*skops).op == BPF_SOCK_OPS_TSTAMP_ACK_CB {
        bpf_map_delete_elem(
            &mut time_map as *mut _ as *mut core::ffi::c_void,
            &key as *const _ as *const core::ffi::c_void,
        );
    }

    true
}

// SEC("fentry/tcp_sendmsg_locked")
#[no_mangle]
pub unsafe extern "C" fn trace_tcp_sendmsg_locked(
    sk: *mut sock,
    _msg: *mut msghdr,
    _size: size_t,
) -> i32 {
    let pid: __u32 = (bpf_get_current_pid_tgid() >> 32) as __u32;
    let timestamp: u64 = bpf_ktime_get_ns();
    let flag: u32 = (*sk).sk_bpf_cb_flags;
    let mut stg: *mut sk_stg;

    if pid != monitored_pid || flag == 0 {
        return 0;
    }

    stg = bpf_sk_storage_get(
        &mut sk_stg_map as *mut _ as *mut core::ffi::c_void,
        sk as *mut core::ffi::c_void,
        core::ptr::null_mut(),
        BPF_SK_STORAGE_GET_F_CREATE,
    ) as *mut sk_stg;
    if stg.is_null() {
        return 0;
    }

    (*stg).sendmsg_ns = timestamp;
    nr_snd += 1;
    0
}

// SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn skops_sockopt(skops: *mut bpf_sock_ops) -> i32 {
    let bpf_sk: *mut bpf_sock = (*skops).sk;
    let sk: *const sock;

    if bpf_sk.is_null() {
        return 1;
    }

    sk = bpf_skc_to_tcp_sock(bpf_sk) as *const sock;
    if sk.is_null() {
        return 1;
    }

    match (*skops).op {
        BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB => {
            nr_active += (bpf_test_sockopt(skops as *mut core::ffi::c_void, sk, 0) == 0) as i32;
        }
        BPF_SOCK_OPS_TSTAMP_SENDMSG_CB => {
            if bpf_test_delay(skops, sk) {
                nr_snd += 1;
            }
        }
        BPF_SOCK_OPS_TSTAMP_SCHED_CB => {
            if bpf_test_delay(skops, sk) {
                nr_sched += 1;
            }
        }
        BPF_SOCK_OPS_TSTAMP_SND_SW_CB => {
            if bpf_test_delay(skops, sk) {
                nr_txsw += 1;
            }
        }
        BPF_SOCK_OPS_TSTAMP_ACK_CB => {
            if bpf_test_delay(skops, sk) {
                nr_ack += 1;
            }
        }
        _ => {}
    }

    1
}

// SEC("license")
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
