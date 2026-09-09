// SPDX-License-Identifier: GPL-2.0-only
/* Unstable Fou Helpers for TC-BPF hook
 *
 * These are called from SCHED_CLS BPF programs. Note that it is
 * allowed to break compatibility for these functions since the interface they
 * are exposed through to BPF programs is explicitly unstable.
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left as external Rust declarations.

#[repr(C)]
pub struct bpf_fou_encap {
    pub sport: u16,
    pub dport: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bpf_fou_encap_type {
    FOU_BPF_ENCAP_FOU,
    FOU_BPF_ENCAP_GUE,
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_tunnel_info {
    pub mode: u16,
    pub encap: ip_tunnel_encap,
    pub key: ip_tunnel_key,
}

#[repr(C)]
pub struct ip_tunnel_encap {
    pub type_: u16,
    pub flags: u16,
    pub sport: u16,
    pub dport: u16,
}

#[repr(C)]
pub struct ip_tunnel_key {
    pub tun_flags: u64,
}

#[repr(C)]
pub struct btf_kfunc_id_set {
    pub owner: *mut core::ffi::c_void,
    pub set: *const core::ffi::c_void,
}

unsafe extern "C" {
    fn skb_tunnel_info(skb: *mut sk_buff) -> *mut ip_tunnel_info;
    fn test_bit(nr: usize, addr: *const u64) -> bool;
    fn register_btf_kfunc_id_set(prog_type: u32, set: *const btf_kfunc_id_set) -> i32;
    static THIS_MODULE: core::ffi::c_void;
}

pub const EINVAL: i32 = 22;
pub const IP_TUNNEL_INFO_TX: u16 = 1 << 0;
pub const IP_TUNNEL_CSUM_BIT: usize = 0;
pub const TUNNEL_ENCAP_FOU: u16 = 1;
pub const TUNNEL_ENCAP_GUE: u16 = 2;
pub const TUNNEL_ENCAP_NONE: u16 = 0;
pub const TUNNEL_ENCAP_FLAG_CSUM: u16 = 1 << 0;
pub const BPF_PROG_TYPE_SCHED_CLS: u32 = 3;

// bpf_skb_set_fou_encap - Set FOU encap parameters
//
// This function allows for using GUE or FOU encapsulation together with an
// ipip device in collect-metadata mode.
//
// It is meant to be used in BPF tc-hooks and after a call to the
// bpf_skb_set_tunnel_key helper, responsible for setting IP addresses.
pub unsafe extern "C" fn bpf_skb_set_fou_encap(
    skb_ctx: *mut __sk_buff,
    encap: *mut bpf_fou_encap,
    type_: i32,
) -> i32 {
    let skb = skb_ctx as *mut sk_buff;
    let info = skb_tunnel_info(skb);

    if encap.is_null() {
        return -EINVAL;
    }

    if info.is_null() || ((*info).mode & IP_TUNNEL_INFO_TX) == 0 {
        return -EINVAL;
    }

    match type_ {
        0 => (*info).encap.type_ = TUNNEL_ENCAP_FOU,
        1 => (*info).encap.type_ = TUNNEL_ENCAP_GUE,
        _ => (*info).encap.type_ = TUNNEL_ENCAP_NONE,
    }

    if test_bit(IP_TUNNEL_CSUM_BIT, &(*info).key.tun_flags) {
        (*info).encap.flags |= TUNNEL_ENCAP_FLAG_CSUM;
    }

    (*info).encap.sport = (*encap).sport;
    (*info).encap.dport = (*encap).dport;

    0
}

// bpf_skb_get_fou_encap - Get FOU encap parameters
//
// This function allows for reading encap metadata from a packet received
// on an ipip device in collect-metadata mode.
pub unsafe extern "C" fn bpf_skb_get_fou_encap(
    skb_ctx: *mut __sk_buff,
    encap: *mut bpf_fou_encap,
) -> i32 {
    let skb = skb_ctx as *mut sk_buff;
    let info = skb_tunnel_info(skb);

    if info.is_null() {
        return -EINVAL;
    }

    (*encap).sport = (*info).encap.sport;
    (*encap).dport = (*info).encap.dport;

    0
}

// BTF_KFUNCS_START(fou_kfunc_set)
// BTF_ID_FLAGS(func, bpf_skb_set_fou_encap)
// BTF_ID_FLAGS(func, bpf_skb_get_fou_encap)
// BTF_KFUNCS_END(fou_kfunc_set)
static fou_kfunc_set: [u8; 0] = [];

static fou_bpf_kfunc_set: btf_kfunc_id_set = btf_kfunc_id_set {
    owner: unsafe { &THIS_MODULE as *const _ as *mut _ },
    set: &fou_kfunc_set as *const _ as *const core::ffi::c_void,
};

pub unsafe extern "C" fn register_fou_bpf() -> i32 {
    register_btf_kfunc_id_set(BPF_PROG_TYPE_SCHED_CLS, &fou_bpf_kfunc_set)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
