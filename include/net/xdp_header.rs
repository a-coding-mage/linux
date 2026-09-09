/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of include/net/xdp.h. */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum xdp_mem_type {
    MEM_TYPE_PAGE_SHARED = 0,
    MEM_TYPE_PAGE_ORDER0,
    MEM_TYPE_PAGE_POOL,
    MEM_TYPE_XSK_BUFF_POOL,
    MEM_TYPE_MAX,
}

extern "C" {
    pub fn __xdp_rxq_info_reg(xdp_rxq: *mut xdp_rxq_info, dev: *mut net_device, queue_index: u32, napi_id: u32, frag_size: u32) -> i32;
    pub fn xdp_rxq_info_reg_mem_model(xdp_rxq: *mut xdp_rxq_info, type_: xdp_mem_type, allocator: *mut core::ffi::c_void) -> i32;
    pub fn xdp_rxq_info_unreg_mem_model(xdp_rxq: *mut xdp_rxq_info);
    pub fn xdp_reg_mem_model(mem: *mut xdp_mem_info, type_: xdp_mem_type, allocator: *mut core::ffi::c_void) -> i32;
    pub fn xdp_unreg_mem_model(mem: *mut xdp_mem_info);
    pub fn xdp_reg_page_pool(pool: *mut page_pool) -> i32;
    pub fn xdp_unreg_page_pool(pool: *const page_pool);
    pub fn xdp_rxq_info_attach_page_pool(xdp_rxq: *mut xdp_rxq_info, pool: *const page_pool);
    pub fn __xdp_return(netmem: netmem_ref, mem_type: xdp_mem_type, napi_direct: bool, xdp: *mut xdp_buff);
    pub fn xdp_return_frame_bulk(xdpf: *mut xdp_frame, bq: *mut xdp_frame_bulk);
    pub fn __xdp_build_skb_from_frame(xdpf: *mut xdp_frame, skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff;
    pub fn xdp_build_skb_from_frame(xdpf: *mut xdp_frame, dev: *mut net_device) -> *mut sk_buff;
    pub fn xdpf_clone(xdpf: *mut xdp_frame) -> *mut xdp_frame;
    pub fn bpf_xdp_metadata_kfunc_id(id: i32) -> u32;
    pub fn bpf_dev_bound_kfunc_id(btf_id: u32) -> bool;
    pub fn xdp_set_features_flag(dev: *mut net_device, val: u32);
    pub fn xdp_set_features_flag_locked(dev: *mut net_device, val: u32);
    pub fn xdp_features_set_redirect_target(dev: *mut net_device, support_sg: bool);
    pub fn xdp_features_set_redirect_target_locked(dev: *mut net_device, support_sg: bool);
    pub fn xdp_features_clear_redirect_target(dev: *mut net_device);
    pub fn xdp_features_clear_redirect_target_locked(dev: *mut net_device);
}

#[inline] pub unsafe fn xdp_rxq_info_reg(xdp_rxq: *mut xdp_rxq_info, dev: *mut net_device, queue_index: u32, napi_id: u32) -> i32 { __xdp_rxq_info_reg(xdp_rxq, dev, queue_index, napi_id, 0) }
#[inline] pub unsafe fn xdp_rxq_info_attach_mem_model(xdp_rxq: *mut xdp_rxq_info, mem: *const xdp_mem_info) { (*xdp_rxq).mem = *mem; }
#[inline] pub unsafe fn xdp_rxq_info_detach_mem_model(xdp_rxq: *mut xdp_rxq_info) { (*xdp_rxq).mem = xdp_mem_info { type_: 0, id: 0 }; }
#[inline(always)] pub unsafe fn xdp_set_data_meta_invalid(xdp: *mut xdp_buff) { (*xdp).data_meta = (*xdp).data.cast::<u8>().add(1).cast(); }
#[inline(always)] pub unsafe fn xdp_data_meta_unsupported(xdp: *const xdp_buff) -> bool { (*xdp).data_meta as usize > (*xdp).data as usize }
#[inline] pub fn xdp_metalen_invalid(metalen: usize) -> bool { metalen % core::mem::size_of::<u32>() != 0 || metalen > u8::MAX as usize }
#[inline] pub unsafe fn xdp_clear_features_flag(dev: *mut net_device) { xdp_set_features_flag(dev, 0); }

#[inline] pub unsafe fn xdp_convert_frame_to_buff(frame: *const xdp_frame, xdp: *mut xdp_buff) {
    (*xdp).data_hard_start = (*frame).data.sub((*frame).headroom as usize + core::mem::size_of::<xdp_frame>()).cast();
    (*xdp).data = (*frame).data; (*xdp).data_end = (*frame).data.add((*frame).len as usize);
    (*xdp).data_meta = (*frame).data.sub((*frame).metasize as usize);
    (*xdp).flags_union.fields = xdp_buff_fields { frame_sz: (*frame).frame_sz, flags: (*frame).flags };
}

#[inline] pub unsafe fn xdp_update_frame_from_buff(xdp: *const xdp_buff, frame: *mut xdp_frame) -> i32 {
    let headroom = (*xdp).data as usize - (*xdp).data_hard_start as usize;
    let meta = ((*xdp).data as usize).saturating_sub((*xdp).data_meta as usize);
    if headroom.saturating_sub(meta) < core::mem::size_of::<xdp_frame>() { return -28; }
    (*frame).data = (*xdp).data; (*frame).len = ((*xdp).data_end as usize - (*xdp).data as usize) as u32;
    (*frame).headroom = (headroom - core::mem::size_of::<xdp_frame>()) as u32; (*frame).metasize = meta as u32;
    (*frame).frame_sz = (*xdp).flags_union.fields.frame_sz; (*frame).flags = (*xdp).flags_union.fields.flags; 0
}

#[inline] pub unsafe fn xdp_convert_buff_to_frame(xdp: *mut xdp_buff) -> *mut xdp_frame {
    let frame = (*xdp).data_hard_start as *mut xdp_frame;
    if xdp_update_frame_from_buff(xdp, frame) < 0 { return core::ptr::null_mut(); }
    (*frame).mem_type = (*(*xdp).rxq).mem.type_ as xdp_mem_type; frame
}

pub const XDP_XMIT_FLUSH: u32 = 1u32 << 0;
pub const XDP_XMIT_FLAGS_MASK: u32 = XDP_XMIT_FLUSH;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_mem_info { pub type_: u32, pub id: u32 }

#[repr(C)]
pub struct page_pool;
#[repr(C)]
pub struct net_device;
#[repr(C)]
pub struct bpf_prog;
#[repr(C)]
pub struct netdev_bpf;
#[repr(C)]
pub struct xdp_md;
#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct skb_frag_t;
#[repr(C)]
pub struct netmem_ref;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_rxq_info {
    pub dev: *mut net_device,
    pub queue_index: u32,
    pub reg_state: u32,
    pub mem: xdp_mem_info,
    pub frag_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_txq_info { pub dev: *mut net_device }

pub const XDP_FLAGS_HAS_FRAGS: u32 = 1 << 0;
pub const XDP_FLAGS_FRAGS_PF_MEMALLOC: u32 = 1 << 1;
pub const XDP_FLAGS_FRAGS_UNREADABLE: u32 = 1 << 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub union xdp_buff_flags_union { pub frame_sz_flags_init: u64, pub fields: xdp_buff_fields }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_buff_fields { pub frame_sz: u32, pub flags: u32 }

#[repr(C)]
pub struct xdp_buff {
    pub data: *mut core::ffi::c_void,
    pub data_end: *mut core::ffi::c_void,
    pub data_meta: *mut core::ffi::c_void,
    pub data_hard_start: *mut core::ffi::c_void,
    pub rxq: *mut xdp_rxq_info,
    pub txq: *mut xdp_txq_info,
    pub flags_union: xdp_buff_flags_union,
}

#[inline(always)] pub unsafe fn xdp_buff_has_frags(xdp: *const xdp_buff) -> bool { ((*xdp).flags_union.fields.flags & XDP_FLAGS_HAS_FRAGS) != 0 }
#[inline(always)] pub unsafe fn xdp_buff_set_frags_flag(xdp: *mut xdp_buff) { (*xdp).flags_union.fields.flags |= XDP_FLAGS_HAS_FRAGS; }
#[inline(always)] pub unsafe fn xdp_buff_clear_frags_flag(xdp: *mut xdp_buff) { (*xdp).flags_union.fields.flags &= !XDP_FLAGS_HAS_FRAGS; }
#[inline(always)] pub unsafe fn xdp_buff_set_frag_pfmemalloc(xdp: *mut xdp_buff) { (*xdp).flags_union.fields.flags |= XDP_FLAGS_FRAGS_PF_MEMALLOC; }
#[inline(always)] pub unsafe fn xdp_buff_set_frag_unreadable(xdp: *mut xdp_buff) { (*xdp).flags_union.fields.flags |= XDP_FLAGS_FRAGS_UNREADABLE; }
#[inline(always)] pub unsafe fn xdp_buff_get_skb_flags(xdp: *const xdp_buff) -> u32 { (*xdp).flags_union.fields.flags }
#[inline(always)] pub unsafe fn xdp_buff_clear_frag_pfmemalloc(xdp: *mut xdp_buff) { (*xdp).flags_union.fields.flags &= !XDP_FLAGS_FRAGS_PF_MEMALLOC; }

#[inline(always)] pub unsafe fn xdp_init_buff(xdp: *mut xdp_buff, frame_sz: u32, rxq: *mut xdp_rxq_info) { (*xdp).rxq = rxq; (*xdp).flags_union.fields = xdp_buff_fields { frame_sz, flags: 0 }; }
#[inline(always)] pub unsafe fn xdp_prepare_buff(xdp: *mut xdp_buff, hard_start: *mut u8, headroom: isize, data_len: isize, meta_valid: bool) { let data = hard_start.offset(headroom); (*xdp).data_hard_start = hard_start.cast(); (*xdp).data = data.cast(); (*xdp).data_end = data.offset(data_len).cast(); (*xdp).data_meta = if meta_valid { data } else { data.offset(1) }.cast(); }

extern "C" {
    pub fn xdp_return_frag(netmem: netmem_ref, xdp: *const xdp_buff);
    pub fn xdp_warn(msg: *const core::ffi::c_char, func: *const core::ffi::c_char, line: i32);
    pub fn xdp_build_skb_from_buff(xdp: *const xdp_buff) -> *mut sk_buff;
    pub fn xdp_build_skb_from_zc(xdp: *mut xdp_buff) -> *mut sk_buff;
    pub fn xdp_convert_zc_to_xdp_frame(xdp: *mut xdp_buff) -> *mut xdp_frame;
    pub fn xdp_return_frame(xdpf: *mut xdp_frame);
    pub fn xdp_return_frame_rx_napi(xdpf: *mut xdp_frame);
    pub fn xdp_return_buff(xdp: *mut xdp_buff);
    pub fn xdp_rxq_info_unreg(xdp_rxq: *mut xdp_rxq_info);
    pub fn xdp_rxq_info_unused(xdp_rxq: *mut xdp_rxq_info);
    pub fn xdp_rxq_info_is_reg(xdp_rxq: *mut xdp_rxq_info) -> bool;
    pub fn xdp_clear_features_flag(dev: *mut net_device);
}

#[repr(C)]
pub struct xdp_frame { pub data: *mut core::ffi::c_void, pub len: u32, pub headroom: u32, pub metasize: u32, pub mem_type: xdp_mem_type, pub dev_rx: *mut net_device, pub frame_sz: u32, pub flags: u32 }
#[inline(always)] pub unsafe fn xdp_frame_has_frags(frame: *const xdp_frame) -> bool { ((*frame).flags & XDP_FLAGS_HAS_FRAGS) != 0 }
#[inline(always)] pub unsafe fn xdp_frame_get_skb_flags(frame: *const xdp_frame) -> u32 { (*frame).flags }
pub const XDP_BULK_QUEUE_SIZE: usize = 16;
#[repr(C)] pub struct xdp_frame_bulk { pub count: i32, pub q: [netmem_ref; XDP_BULK_QUEUE_SIZE] }
#[inline(always)] pub unsafe fn xdp_frame_bulk_init(bq: *mut xdp_frame_bulk) { (*bq).count = 0; }
#[repr(C)] pub struct xdp_cpumap_stats { pub redirect: u32, pub pass: u32, pub drop: u32 }
#[inline] pub unsafe fn xdp_scrub_frame(frame: *mut xdp_frame) { (*frame).data = core::ptr::null_mut(); (*frame).dev_rx = core::ptr::null_mut(); }

#[repr(C)] pub struct xdp_attachment_info { pub prog: *mut bpf_prog, pub flags: u32 }
extern "C" { pub fn xdp_attachment_setup(info: *mut xdp_attachment_info, bpf: *mut netdev_bpf); }
pub const DEV_MAP_BULK_SIZE: usize = XDP_BULK_QUEUE_SIZE;

#[repr(C)] pub struct xdp_metadata_ops {
    pub xmo_rx_timestamp: Option<unsafe extern "C" fn(*const xdp_md, *mut u64) -> i32>,
    pub xmo_rx_hash: Option<unsafe extern "C" fn(*const xdp_md, *mut u32, *mut xdp_rss_hash_type) -> i32>,
    pub xmo_rx_vlan_tag: Option<unsafe extern "C" fn(*const xdp_md, *mut u16, *mut u16) -> i32>,
}
#[repr(C)] #[derive(Copy, Clone)] pub enum xdp_rx_metadata { XDP_METADATA_KFUNC_RX_TIMESTAMP, XDP_METADATA_KFUNC_RX_HASH, XDP_METADATA_KFUNC_RX_VLAN_TAG, MAX_XDP_METADATA_KFUNC }
#[repr(C)] #[derive(Copy, Clone)] pub enum xdp_rss_hash_type {
    XDP_RSS_L3_IPV4=1<<0, XDP_RSS_L3_IPV6=1<<1, XDP_RSS_L3_DYNHDR=1<<2, XDP_RSS_L4=1<<3, XDP_RSS_L4_TCP=1<<4, XDP_RSS_L4_UDP=1<<5, XDP_RSS_L4_SCTP=1<<6, XDP_RSS_L4_IPSEC=1<<7, XDP_RSS_L4_ICMP=1<<8,
    XDP_RSS_TYPE_NONE=0, XDP_RSS_TYPE_L2=0, XDP_RSS_TYPE_L3_IPV4=1, XDP_RSS_TYPE_L3_IPV6=2, XDP_RSS_TYPE_L3_IPV4_OPT=5, XDP_RSS_TYPE_L3_IPV6_EX=6,
    XDP_RSS_TYPE_L4_ANY=8, XDP_RSS_TYPE_L4_IPV4_TCP=25, XDP_RSS_TYPE_L4_IPV4_UDP=41, XDP_RSS_TYPE_L4_IPV4_SCTP=73, XDP_RSS_TYPE_L4_IPV4_IPSEC=137, XDP_RSS_TYPE_L4_IPV4_ICMP=265,
    XDP_RSS_TYPE_L4_IPV6_TCP=26, XDP_RSS_TYPE_L4_IPV6_UDP=42, XDP_RSS_TYPE_L4_IPV6_SCTP=74, XDP_RSS_TYPE_L4_IPV6_IPSEC=138, XDP_RSS_TYPE_L4_IPV6_ICMP=266,
    XDP_RSS_TYPE_L4_IPV6_TCP_EX=30, XDP_RSS_TYPE_L4_IPV6_UDP_EX=46, XDP_RSS_TYPE_L4_IPV6_SCTP_EX=78,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
