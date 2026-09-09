// SPDX-License-Identifier: GPL-2.0-only
/* Unstable XFRM Helpers for TC-BPF hook
 *
 * These are called from SCHED_CLS BPF programs. Note that it is
 * allowed to break compatibility for these functions since the interface they
 * are exposed through to BPF programs is explicitly unstable.
 */

// Dependency declarations supplied by the Linux kernel environment.
use core::ffi::c_void;

#[repr(C)]
pub struct __sk_buff {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct dst_entry {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct metadata_dst {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct xfrm_md_info {
    pub if_id: u32,
    pub link: i32,
    pub dst_orig: *mut dst_entry,
}

#[repr(C)]
pub struct btf_kfunc_id_set {
    pub owner: *mut c_void,
    pub set: *const c_void,
}

extern "C" {
    fn skb_xfrm_md_info(skb: *mut sk_buff) -> *mut xfrm_md_info;
    fn skb_metadata_dst(skb: *mut sk_buff) -> *mut metadata_dst;
    fn metadata_dst_alloc_percpu(flags: u32, metadata_type: u32, gfp_mask: u32)
        -> *mut metadata_dst;
    fn metadata_dst_free_percpu(dst: *mut metadata_dst);
    fn this_cpu_ptr(ptr: *mut *mut metadata_dst) -> *mut metadata_dst;
    fn skb_dst_force(skb: *mut sk_buff);
    fn skb_dst(skb: *mut sk_buff) -> *mut dst_entry;
    fn dst_hold(dst: *mut dst_entry);
    fn skb_dst_set(skb: *mut sk_buff, dst: *mut dst_entry);
    fn register_btf_kfunc_id_set(prog_type: u32, set: *const btf_kfunc_id_set) -> i32;
    static mut xfrm_bpf_md_dst: *mut metadata_dst;
    static mut THIS_MODULE: *mut c_void;
}

pub const EINVAL: i32 = 22;
pub const ENOMEM: i32 = 12;
pub const METADATA_XFRM: u32 = 0;
pub const GFP_ATOMIC: u32 = 0;
pub const BPF_PROG_TYPE_SCHED_CLS: u32 = 3;

/* bpf_xfrm_info - XFRM metadata information
 *
 * Members:
 * @if_id - XFRM if_id:
 *          Transmit: if_id to be used in policy and state lookups
 *          Receive: if_id of the state matched for the incoming packet
 * @link - Underlying device ifindex:
 *         Transmit: used as the underlying device in VRF routing
 *         Receive: the device on which the packet had been received
 */
#[repr(C)]
pub struct bpf_xfrm_info {
    pub if_id: u32,
    pub link: i32,
}

/* bpf_skb_get_xfrm_info - Get XFRM metadata */
pub unsafe extern "C" fn bpf_skb_get_xfrm_info(
    skb_ctx: *mut __sk_buff,
    to: *mut bpf_xfrm_info,
) -> i32 {
    let skb = skb_ctx as *mut sk_buff;
    let info = skb_xfrm_md_info(skb);
    if info.is_null() {
        return -EINVAL;
    }

    (*to).if_id = (*info).if_id;
    (*to).link = (*info).link;
    0
}

/* bpf_skb_set_xfrm_info - Set XFRM metadata */
pub unsafe extern "C" fn bpf_skb_set_xfrm_info(
    skb_ctx: *mut __sk_buff,
    from: *const bpf_xfrm_info,
) -> i32 {
    let skb = skb_ctx as *mut sk_buff;
    let md_dst: *mut metadata_dst;
    let info: *mut xfrm_md_info;

    if !skb_metadata_dst(skb).is_null() {
        return -EINVAL;
    }

    if xfrm_bpf_md_dst.is_null() {
        let tmp = metadata_dst_alloc_percpu(0, METADATA_XFRM, GFP_ATOMIC);
        if tmp.is_null() {
            return -ENOMEM;
        }
        if !xfrm_bpf_md_dst.is_null() {
            metadata_dst_free_percpu(tmp);
        } else {
            xfrm_bpf_md_dst = tmp;
        }
    }
    md_dst = this_cpu_ptr(&raw mut xfrm_bpf_md_dst);

    // The kernel metadata_dst contains the xfrm_info union member.
    info = md_dst as *mut xfrm_md_info;

    (*info).if_id = (*from).if_id;
    (*info).link = (*from).link;
    skb_dst_force(skb);
    (*info).dst_orig = skb_dst(skb);

    dst_hold(md_dst as *mut dst_entry);
    skb_dst_set(skb, md_dst as *mut dst_entry);
    0
}

// BTF_KFUNCS_START(xfrm_ifc_kfunc_set)
// BTF_ID_FLAGS(func, bpf_skb_get_xfrm_info)
// BTF_ID_FLAGS(func, bpf_skb_set_xfrm_info)
// BTF_KFUNCS_END(xfrm_ifc_kfunc_set)
static xfrm_ifc_kfunc_set: c_void = c_void { };

static xfrm_interface_kfunc_set: btf_kfunc_id_set = btf_kfunc_id_set {
    owner: core::ptr::null_mut(),
    set: &raw const xfrm_ifc_kfunc_set as *const c_void,
};

pub unsafe extern "C" fn register_xfrm_interface_bpf() -> i32 {
    register_btf_kfunc_id_set(BPF_PROG_TYPE_SCHED_CLS, &raw const xfrm_interface_kfunc_set)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
