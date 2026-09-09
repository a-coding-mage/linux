// SPDX-License-Identifier: GPL-2.0
/* Refer to samples/bpf/tcp_bpf.readme for the instructions on
 * how to run this sample program.
 */

// C includes translated as externally supplied BPF/kernel definitions.

pub const INTERVAL: u64 = 1000000000u64;

#[link_section = "version"]
#[no_mangle]
pub static mut _version: i32 = 1;

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct BpfNextDump {
    pub type_: u32,
    pub map_flags: u32,
    pub key: *mut i32,
    pub value: *mut u64,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut bpf_next_dump: BpfNextDump = BpfNextDump {
    type_: BPF_MAP_TYPE_SK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
};

#[repr(C)]
pub struct bpf_sock_ops {
    pub op: u32,
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_tcp_sock {
    pub dsack_dups: u32,
    pub delivered: u32,
    pub delivered_ce: u32,
    pub icsk_retransmits: u32,
}

extern "C" {
    fn bpf_sock_ops_cb_flags_set(ctx: *mut bpf_sock_ops, flags: u32) -> i32;
    fn bpf_sk_storage_get(
        map: *mut BpfNextDump,
        sk: *mut bpf_sock,
        value: u64,
        flags: u64,
    ) -> *mut u64;
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_tcp_sock(sk: *mut bpf_sock) -> *mut bpf_tcp_sock;
    fn bpf_printk(fmt: *const u8, ...);
}

// External BPF constants supplied by the kernel headers.
extern "C" {
    static BPF_MAP_TYPE_SK_STORAGE: u32;
    static BPF_F_NO_PREALLOC: u32;
    static BPF_SOCK_OPS_TCP_CONNECT_CB: u32;
    static BPF_SOCK_OPS_RTT_CB: u32;
    static BPF_SOCK_OPS_RTT_CB_FLAG: u32;
    static BPF_SK_STORAGE_GET_F_CREATE: u64;
}

#[link_section = "sockops"]
#[no_mangle]
pub unsafe extern "C" fn _sockops(ctx: *mut bpf_sock_ops) -> i32 {
    let mut tcp_sk: *mut bpf_tcp_sock;
    let mut sk: *mut bpf_sock;
    let mut next_dump: *mut u64;
    let mut now: u64;

    match (*ctx).op {
        BPF_SOCK_OPS_TCP_CONNECT_CB => {
            bpf_sock_ops_cb_flags_set(ctx, BPF_SOCK_OPS_RTT_CB_FLAG);
            return 1;
        }
        BPF_SOCK_OPS_RTT_CB => {}
        _ => return 1,
    }

    sk = (*ctx).sk;
    if sk.is_null() {
        return 1;
    }

    next_dump = bpf_sk_storage_get(
        &mut bpf_next_dump,
        sk,
        0,
        BPF_SK_STORAGE_GET_F_CREATE,
    );
    if next_dump.is_null() {
        return 1;
    }

    now = bpf_ktime_get_ns();
    if now < *next_dump {
        return 1;
    }

    tcp_sk = bpf_tcp_sock(sk);
    if tcp_sk.is_null() {
        return 1;
    }

    *next_dump = now.wrapping_add(INTERVAL);

    bpf_printk(
        b"dsack_dups=%u delivered=%u\n\0".as_ptr(),
        (*tcp_sk).dsack_dups,
        (*tcp_sk).delivered,
    );
    bpf_printk(
        b"delivered_ce=%u icsk_retransmits=%u\n\0".as_ptr(),
        (*tcp_sk).delivered_ce,
        (*tcp_sk).icsk_retransmits,
    );

    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
