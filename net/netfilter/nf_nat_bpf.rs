// SPDX-License-Identifier: GPL-2.0-only
/* Unstable NAT Helpers for XDP and TC-BPF hook
 *
 * These are called from the XDP and SCHED_CLS BPF programs. Note that it is
 * allowed to break compatibility for these functions since the interface they
 * are exposed through to BPF programs is explicitly unstable.
 */

// Dependencies supplied by the corresponding Linux kernel headers are
// intentionally left as external Rust items.

extern "C" {
    static THIS_MODULE: core::ffi::c_void;
    fn nf_ct_l3num(ct: *mut nf_conn) -> u16;
    fn nf_nat_setup_info(
        ct: *mut nf_conn,
        range: *mut nf_nat_range2,
        manip: nf_nat_manip_type,
    ) -> i32;
    fn register_btf_kfunc_id_set(
        prog_type: u32,
        set: *const btf_kfunc_id_set,
    ) -> i32;
}

#[repr(C)]
pub struct nf_conn___init {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conn {
    _private: [u8; 0],
}

#[repr(C)]
pub union nf_inet_addr {
    pub all: u32,
    pub ip: u32,
    pub ip6: [u32; 4],
}

#[repr(C)]
pub union nf_nat_proto {
    pub all: u16,
}

#[repr(C)]
pub struct nf_nat_range2 {
    pub flags: u32,
    pub min_addr: nf_inet_addr,
    pub max_addr: nf_inet_addr,
    pub min_proto: nf_nat_proto,
    pub max_proto: nf_nat_proto,
}

#[repr(C)]
pub struct btf_kfunc_id_set {
    pub owner: *mut core::ffi::c_void,
    pub set: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nf_nat_manip_type {
    NF_NAT_MANIP_SRC = 0,
    NF_NAT_MANIP_DST = 1,
}

pub const NFPROTO_IPV4: u16 = 2;
pub const NFPROTO_IPV6: u16 = 10;
pub const NF_NAT_RANGE_MAP_IPS: u32 = 1 << 0;
pub const NF_NAT_RANGE_PROTO_SPECIFIED: u32 = 1 << 1;
pub const NF_DROP: i32 = 0;
pub const BPF_PROG_TYPE_XDP: u32 = 6;
pub const BPF_PROG_TYPE_SCHED_CLS: u32 = cls_prog_type();

const fn cls_prog_type() -> u32 {
    3
}

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;

// bpf_ct_set_nat_info - Set source or destination nat address
//
// Set source or destination nat address of the newly allocated
// nf_conn before insertion. This must be invoked for referenced
// PTR_TO_BTF_ID to nf_conn___init.
//
// Parameters:
// @nfct  - Pointer to referenced nf_conn object, obtained using
//          bpf_xdp_ct_alloc or bpf_skb_ct_alloc.
// @addr  - Nat source/destination address
// @port  - Nat source/destination port. Non-positive values are
//          interpreted as select a random port.
// @manip - NF_NAT_MANIP_SRC or NF_NAT_MANIP_DST
pub unsafe fn bpf_ct_set_nat_info(
    nfct: *mut nf_conn___init,
    addr: *mut nf_inet_addr,
    port: i32,
    manip: nf_nat_manip_type,
) -> i32 {
    let ct = nfct as *mut nf_conn;
    let proto: u16 = nf_ct_l3num(ct);
    let mut range: nf_nat_range2 = core::mem::zeroed();

    if proto != NFPROTO_IPV4 && proto != NFPROTO_IPV6 {
        return -EINVAL;
    }

    range.flags = NF_NAT_RANGE_MAP_IPS;
    range.min_addr = *addr;
    range.max_addr = range.min_addr;
    if port > 0 {
        range.flags |= NF_NAT_RANGE_PROTO_SPECIFIED;
        range.min_proto.all = (port as u16).to_be();
        range.max_proto.all = range.min_proto.all;
    }

    if nf_nat_setup_info(ct, &mut range, manip) == NF_DROP {
        -ENOMEM
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn register_nf_nat_bpf() -> i32 {
    let ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_XDP, &nf_bpf_nat_kfunc_set);
    if ret != 0 {
        return ret;
    }

    register_btf_kfunc_id_set(BPF_PROG_TYPE_SCHED_CLS, &nf_bpf_nat_kfunc_set)
}

// BTF_KFUNCS_START(nf_nat_kfunc_set)
// BTF_ID_FLAGS(func, bpf_ct_set_nat_info)
// BTF_KFUNCS_END(nf_nat_kfunc_set)
static nf_nat_kfunc_set: core::ffi::c_void = core::ffi::c_void;

static nf_bpf_nat_kfunc_set: btf_kfunc_id_set = btf_kfunc_id_set {
    owner: unsafe { &THIS_MODULE as *const _ as *mut _ },
    set: &nf_nat_kfunc_set as *const _ as *const core::ffi::c_void,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
