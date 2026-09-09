/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from fc_frame.h. External kernel/scsi types and functions are supplied elsewhere. */

/* Dependency intent: linux/scatterlist.h, linux/skbuff.h, scsi headers, and linux/if_ether.h. */

pub const FC_FRAME_HEADROOM: usize = 32;
pub const FC_FRAME_TAILROOM: usize = 8;
pub const FCPHF_CRC_UNCHECKED: u8 = 0x01;

#[inline] pub unsafe fn ntohll(x: u64) -> u64 { be64_to_cpu(x) }
#[inline] pub unsafe fn htonll(x: u64) -> u64 { cpu_to_be64(x) }

pub const FC_FRAME_SG_LEN: usize = MAX_SKB_FRAGS - 1;

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;

extern "C" {
    static MAX_SKB_FRAGS: usize;
    fn be64_to_cpu(x: u64) -> u64;
    fn cpu_to_be64(x: u64) -> u64;
    fn htonl(x: u32) -> u32;
    fn kfree_skb(skb: *mut sk_buff);
    fn skb_is_nonlinear(skb: *const sk_buff) -> bool;
    fn WARN_ON(condition: bool) -> bool;
    fn fc_sof_class(sof: u8) -> fc_class;
    fn fc_frame_alloc_fill(dev: *mut fc_lport, payload_len: usize) -> *mut fc_frame;
    fn _fc_frame_alloc(payload_len: usize) -> *mut fc_frame;
    fn fc_frame_crc_check(fp: *mut fc_frame) -> u32;
    fn fc_frame_leak_check();
}

#[repr(C)]
pub struct sk_buff {
    pub data: *mut u8,
    pub len: usize,
    pub cb: [u8; 48],
}

#[repr(C)]
pub struct fc_frame { pub skb: sk_buff }

#[repr(C)]
pub struct fcoe_rcv_info {
    pub fr_dev: *mut fc_lport,
    pub fr_seq: *mut fc_seq,
    pub fr_fsp: *mut fc_fcp_pkt,
    pub fr_crc: u32,
    pub fr_max_payload: u16,
    pub fr_sof: u8,
    pub fr_eof: u8,
    pub fr_flags: u8,
    pub fr_encaps: u8,
    pub granted_mac: [u8; ETH_ALEN],
}

/* External types supplied by the included FC headers. */
pub enum fc_lport {}
pub enum fc_seq {}
pub enum fc_fcp_pkt {}
pub enum fc_class {}
pub enum fc_rctl {}
pub enum fc_fh_type {}
pub enum fc_frame_header {}

pub const ETH_ALEN: usize = 6;

#[inline] pub unsafe fn ntoh24(p: *const u8) -> u32 {
    ((*p as u32) << 16) | ((*p.add(1) as u32) << 8) | (*p.add(2) as u32)
}
#[inline] pub unsafe fn hton24(p: *mut u8, v: u32) {
    *p = (v >> 16) as u8; *p.add(1) = (v >> 8) as u8; *p.add(2) = v as u8;
}

#[inline] pub unsafe fn fp_skb(fp: *mut fc_frame) -> *mut sk_buff { &mut (*fp).skb }
#[inline] pub unsafe fn fr_hdr(fp: *mut fc_frame) -> *mut u8 { (*fp).skb.data }
#[inline] pub unsafe fn fr_len(fp: *const fc_frame) -> usize { (*fp).skb.len }
#[inline] pub unsafe fn fr_cb(fp: *mut fc_frame) -> *mut fcoe_rcv_info { (*fp).skb.cb.as_mut_ptr() as *mut fcoe_rcv_info }
#[inline] pub unsafe fn fr_dev(fp: *mut fc_frame) -> &mut *mut fc_lport { &mut (*fr_cb(fp)).fr_dev }
#[inline] pub unsafe fn fr_seq(fp: *mut fc_frame) -> &mut *mut fc_seq { &mut (*fr_cb(fp)).fr_seq }
#[inline] pub unsafe fn fr_sof(fp: *mut fc_frame) -> &mut u8 { &mut (*fr_cb(fp)).fr_sof }
#[inline] pub unsafe fn fr_eof(fp: *mut fc_frame) -> &mut u8 { &mut (*fr_cb(fp)).fr_eof }
#[inline] pub unsafe fn fr_flags(fp: *mut fc_frame) -> &mut u8 { &mut (*fr_cb(fp)).fr_flags }
#[inline] pub unsafe fn fr_encaps(fp: *mut fc_frame) -> &mut u8 { &mut (*fr_cb(fp)).fr_encaps }
#[inline] pub unsafe fn fr_max_payload(fp: *mut fc_frame) -> &mut u16 { &mut (*fr_cb(fp)).fr_max_payload }
#[inline] pub unsafe fn fr_fsp(fp: *mut fc_frame) -> &mut *mut fc_fcp_pkt { &mut (*fr_cb(fp)).fr_fsp }
#[inline] pub unsafe fn fr_crc(fp: *mut fc_frame) -> &mut u32 { &mut (*fr_cb(fp)).fr_crc }

pub const FC_RCTL_DD_UNSOL_CMD: u8 = 0;

#[inline] pub unsafe fn fcoe_dev_from_skb(skb: *const sk_buff) -> *mut fcoe_rcv_info { (*skb).cb.as_ptr() as *mut fcoe_rcv_info }
#[inline] pub unsafe fn fc_frame_init(fp: *mut fc_frame) { *fr_dev(fp)=core::ptr::null_mut(); *fr_seq(fp)=core::ptr::null_mut(); *fr_flags(fp)=0; *fr_encaps(fp)=0; }
#[inline] pub unsafe fn fc_frame_alloc(dev: *mut fc_lport, len: usize) -> *mut fc_frame { if len != 0 && len % 4 != 0 { fc_frame_alloc_fill(dev,len) } else { _fc_frame_alloc(len) } }
#[inline] pub unsafe fn fc_frame_free(fp: *mut fc_frame) { kfree_skb(fp_skb(fp)); }
#[inline] pub unsafe fn fc_frame_is_linear(fp: *mut fc_frame) -> bool { !skb_is_nonlinear(fp_skb(fp)) }
#[inline] pub unsafe fn __fc_frame_header_get(fp: *const fc_frame) -> *mut fc_frame_header { fr_hdr(fp as *mut fc_frame) as *mut fc_frame_header }
#[inline] pub unsafe fn fc_frame_header_get(fp: *const fc_frame) -> *mut fc_frame_header { WARN_ON(fr_len(fp) < core::mem::size_of::<fc_frame_header>()); __fc_frame_header_get(fp) }
#[inline] pub unsafe fn fc_frame_sid(fp: *const fc_frame) -> u32 { ntoh24(__fc_frame_header_get(fp) as *const u8) }
#[inline] pub unsafe fn fc_frame_did(fp: *const fc_frame) -> u32 { ntoh24(__fc_frame_header_get(fp) as *const u8) }
#[inline] pub unsafe fn fc_frame_payload_get(fp: *const fc_frame, len: usize) -> *mut core::ffi::c_void { if fr_len(fp) >= core::mem::size_of::<fc_frame_header>() + len { (fc_frame_header_get(fp) as *mut u8).add(core::mem::size_of::<fc_frame_header>()) as *mut core::ffi::c_void } else { core::ptr::null_mut() } }
#[inline] pub unsafe fn fc_frame_payload_op(fp: *const fc_frame) -> u8 { let p=fc_frame_payload_get(fp,1) as *const u8; if p.is_null(){0}else{*p} }
#[inline] pub unsafe fn fc_frame_class(fp: *const fc_frame) -> fc_class { fc_sof_class(*fr_sof(fp as *mut fc_frame)) }
#[inline] pub unsafe fn fc_frame_rctl(fp: *const fc_frame) -> u8 { *(fc_frame_header_get(fp) as *mut u8) }
#[inline] pub unsafe fn fc_frame_is_cmd(fp: *const fc_frame) -> bool { fc_frame_rctl(fp) == FC_RCTL_DD_UNSOL_CMD }

#[inline] pub unsafe fn __fc_fill_fc_hdr(_fh: *mut fc_frame_header, _r_ctl: fc_rctl, _did: u32, _sid: u32, _kind: fc_fh_type, _f_ctl: u32, _parm_offset: u32) {
    /* Field assignments correspond to the externally supplied fc_frame_header layout. */
    WARN_ON(false);
}

#[inline] pub unsafe fn fc_fill_fc_hdr(fp: *mut fc_frame, r_ctl: fc_rctl, did: u32, sid: u32, kind: fc_fh_type, f_ctl: u32, parm_offset: u32) {
    let fh = fc_frame_header_get(fp);
    __fc_fill_fc_hdr(fh, r_ctl, did, sid, kind, f_ctl, parm_offset);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
