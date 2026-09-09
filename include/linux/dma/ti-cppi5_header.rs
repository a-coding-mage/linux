/* SPDX-License-Identifier: GPL-2.0 */
/* CPPI5 descriptors interface */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C, packed)]
pub struct cppi5_desc_hdr_t { pub pkt_info0: u32, pub pkt_info1: u32, pub pkt_info2: u32, pub src_dst_tag: u32 }
#[repr(C, packed)]
pub struct cppi5_host_desc_t { pub hdr: cppi5_desc_hdr_t, pub next_desc: u64, pub buf_ptr: u64, pub buf_info1: u32, pub org_buf_len: u32, pub org_buf_ptr: u64, pub epib: [u32; 0] }
#[repr(C)]
pub struct cppi5_desc_epib_t { pub timestamp: u32, pub sw_info0: u32, pub sw_info1: u32, pub sw_info2: u32 }
#[repr(C)]
pub struct cppi5_monolithic_desc_t { pub hdr: cppi5_desc_hdr_t, pub epib: [u32; 0] }

macro_rules! BIT { ($n:expr) => { 1u32 << ($n) }; }
macro_rules! GENMASK { ($h:expr, $l:expr) => { (!0u32 >> (31 - ($h))) & (!0u32 << ($l)) }; }
macro_rules! ALIGN { ($x:expr, $a:expr) => { (($x + $a - 1) / $a) * $a }; }

pub const CPPI5_DESC_MIN_ALIGN: u32 = 16;
pub const CPPI5_INFO0_HDESC_EPIB_SIZE: u32 = 16;
pub const CPPI5_INFO0_HDESC_PSDATA_MAX_SIZE: u32 = 128;
pub const CPPI5_INFO0_HDESC_TYPE_SHIFT: u32 = 30;
pub const CPPI5_INFO0_HDESC_TYPE_MASK: u32 = GENMASK!(31,30);
pub const CPPI5_INFO0_DESC_TYPE_VAL_HOST: u32 = 1;
pub const CPPI5_INFO0_DESC_TYPE_VAL_MONO: u32 = 2;
pub const CPPI5_INFO0_DESC_TYPE_VAL_TR: u32 = 3;
pub const CPPI5_INFO0_HDESC_EPIB_PRESENT: u32 = BIT!(29);
pub const CPPI5_INFO0_HDESC_PSINFO_LOCATION: u32 = BIT!(28);
pub const CPPI5_INFO0_HDESC_PSINFO_SIZE_SHIFT: u32 = 22;
pub const CPPI5_INFO0_HDESC_PSINFO_SIZE_MASK: u32 = GENMASK!(27,22);
pub const CPPI5_INFO0_HDESC_PKTLEN_SHIFT: u32 = 0;
pub const CPPI5_INFO0_HDESC_PKTLEN_MASK: u32 = GENMASK!(21,0);
pub const CPPI5_INFO1_DESC_PKTERROR_SHIFT: u32 = 28;
pub const CPPI5_INFO1_DESC_PKTERROR_MASK: u32 = GENMASK!(31,28);
pub const CPPI5_INFO1_HDESC_PSFLGS_SHIFT: u32 = 24;
pub const CPPI5_INFO1_HDESC_PSFLGS_MASK: u32 = GENMASK!(27,24);
pub const CPPI5_INFO1_DESC_PKTID_SHIFT: u32 = 14;
pub const CPPI5_INFO1_DESC_PKTID_MASK: u32 = GENMASK!(23,14);
pub const CPPI5_INFO1_DESC_FLOWID_SHIFT: u32 = 0;
pub const CPPI5_INFO1_DESC_FLOWID_MASK: u32 = GENMASK!(13,0);
pub const CPPI5_INFO1_DESC_FLOWID_DEFAULT: u32 = CPPI5_INFO1_DESC_FLOWID_MASK;
pub const CPPI5_INFO2_HDESC_PKTTYPE_SHIFT: u32 = 27;
pub const CPPI5_INFO2_HDESC_PKTTYPE_MASK: u32 = GENMASK!(31,27);
pub const CPPI5_INFO2_HDESC_RETPOLICY: u32 = BIT!(18);
pub const CPPI5_INFO2_HDESC_EARLYRET: u32 = BIT!(17);
pub const CPPI5_INFO2_DESC_RETPUSHPOLICY: u32 = BIT!(16);
pub const CPPI5_INFO2_DESC_RETP_MASK: u32 = GENMASK!(18,16);
pub const CPPI5_INFO2_DESC_RETQ_MASK: u32 = GENMASK!(15,0);
pub const CPPI5_INFO3_DESC_SRCTAG_SHIFT: u32 = 16;
pub const CPPI5_INFO3_DESC_SRCTAG_MASK: u32 = GENMASK!(31,16);
pub const CPPI5_INFO3_DESC_DSTTAG_MASK: u32 = GENMASK!(15,0);
pub const CPPI5_BUFINFO1_HDESC_DATA_LEN_MASK: u32 = GENMASK!(27,0);
pub const CPPI5_OBUFINFO0_HDESC_BUF_LEN_MASK: u32 = GENMASK!(27,0);

pub unsafe fn cppi5_desc_is_tdcm(paddr: u64) -> bool { (paddr & 1) != 0 }
pub unsafe fn cppi5_desc_get_type(h: *mut cppi5_desc_hdr_t) -> u32 { ((*h).pkt_info0 & CPPI5_INFO0_HDESC_TYPE_MASK) >> 30 }
pub unsafe fn cppi5_desc_get_errflags(h: *mut cppi5_desc_hdr_t) -> u32 { ((*h).pkt_info1 & CPPI5_INFO1_DESC_PKTERROR_MASK) >> 28 }
pub unsafe fn cppi5_desc_get_pktids(h: *mut cppi5_desc_hdr_t, p: *mut u32, f: *mut u32) { *p = ((*h).pkt_info1 & CPPI5_INFO1_DESC_PKTID_MASK) >> 14; *f = (*h).pkt_info1 & CPPI5_INFO1_DESC_FLOWID_MASK; }
pub unsafe fn cppi5_desc_set_pktids(h: *mut cppi5_desc_hdr_t, p: u32, f: u32) { (*h).pkt_info1 &= !(CPPI5_INFO1_DESC_PKTID_MASK | CPPI5_INFO1_DESC_FLOWID_MASK); (*h).pkt_info1 |= (p << 14) & CPPI5_INFO1_DESC_PKTID_MASK; (*h).pkt_info1 |= f & CPPI5_INFO1_DESC_FLOWID_MASK; }
pub unsafe fn cppi5_desc_set_retpolicy(h: *mut cppi5_desc_hdr_t, flags: u32, ring: u32) { (*h).pkt_info2 &= !(CPPI5_INFO2_DESC_RETP_MASK | CPPI5_INFO2_DESC_RETQ_MASK); (*h).pkt_info2 |= flags & CPPI5_INFO2_DESC_RETP_MASK; (*h).pkt_info2 |= ring & CPPI5_INFO2_DESC_RETQ_MASK; }
pub unsafe fn cppi5_desc_get_tags_ids(h: *mut cppi5_desc_hdr_t, s: *mut u32, d: *mut u32) { if !s.is_null() { *s = ((*h).src_dst_tag & CPPI5_INFO3_DESC_SRCTAG_MASK) >> 16; } if !d.is_null() { *d = (*h).src_dst_tag & CPPI5_INFO3_DESC_DSTTAG_MASK; } }
pub unsafe fn cppi5_desc_set_tags_ids(h: *mut cppi5_desc_hdr_t, s: u32, d: u32) { (*h).src_dst_tag = (s << 16) & CPPI5_INFO3_DESC_SRCTAG_MASK; (*h).src_dst_tag |= d & CPPI5_INFO3_DESC_DSTTAG_MASK; }
pub unsafe fn cppi5_hdesc_calc_size(epib: bool, ps: u32, sw: u32) -> u32 { if ps > 128 { return 0; } ALIGN!(core::mem::size_of::<cppi5_host_desc_t>() as u32 + ps + sw + if epib {16} else {0}, 16) }
pub unsafe fn cppi5_hdesc_init(d: *mut cppi5_host_desc_t, flags: u32, ps: u32) { (*d).hdr.pkt_info0 = (1 << 30) | flags | (((ps >> 2) << 22) & GENMASK!(27,22)); (*d).next_desc = 0; }
pub unsafe fn cppi5_hdesc_update_flags(d: *mut cppi5_host_desc_t, f: u32) { (*d).hdr.pkt_info0 &= !(BIT!(29)|BIT!(28)); (*d).hdr.pkt_info0 |= f; }
pub unsafe fn cppi5_hdesc_update_psdata_size(d: *mut cppi5_host_desc_t, ps: u32) { (*d).hdr.pkt_info0 &= !GENMASK!(27,22); (*d).hdr.pkt_info0 |= ((ps >> 2) << 22) & GENMASK!(27,22); }
pub unsafe fn cppi5_hdesc_get_psdata_size(d: *mut cppi5_host_desc_t) -> u32 { if (*d).hdr.pkt_info0 & BIT!(28) == 0 { (((*d).hdr.pkt_info0 & GENMASK!(27,22)) >> 22) << 2 } else { 0 } }
pub unsafe fn cppi5_hdesc_get_pktlen(d: *mut cppi5_host_desc_t) -> u32 { (*d).hdr.pkt_info0 & GENMASK!(21,0) }
pub unsafe fn cppi5_hdesc_set_pktlen(d: *mut cppi5_host_desc_t, p: u32) { (*d).hdr.pkt_info0 = ((*d).hdr.pkt_info0 & !GENMASK!(21,0)) | (p & GENMASK!(21,0)); }
pub unsafe fn cppi5_hdesc_get_psflags(d: *mut cppi5_host_desc_t) -> u32 { ((*d).hdr.pkt_info1 & GENMASK!(27,24)) >> 24 }
pub unsafe fn cppi5_hdesc_set_psflags(d: *mut cppi5_host_desc_t, p: u32) { (*d).hdr.pkt_info1 = ((*d).hdr.pkt_info1 & !GENMASK!(27,24)) | ((p << 24) & GENMASK!(27,24)); }
pub unsafe fn cppi5_hdesc_get_pkttype(d: *mut cppi5_host_desc_t) -> u32 { ((*d).hdr.pkt_info2 & GENMASK!(31,27)) >> 27 }
pub unsafe fn cppi5_hdesc_set_pkttype(d: *mut cppi5_host_desc_t, p: u32) { (*d).hdr.pkt_info2 = ((*d).hdr.pkt_info2 & !GENMASK!(31,27)) | ((p << 27) & GENMASK!(31,27)); }
pub unsafe fn cppi5_hdesc_attach_buf(d: *mut cppi5_host_desc_t, b: u64, bl: u32, ob: u64, obl: u32) { (*d).buf_ptr=b; (*d).buf_info1=bl&GENMASK!(27,0); (*d).org_buf_ptr=ob; (*d).org_buf_len=obl&GENMASK!(27,0); }
pub unsafe fn cppi5_hdesc_get_obuf(d: *mut cppi5_host_desc_t, b: *mut u64, l: *mut u32) { *b=(*d).org_buf_ptr; *l=(*d).org_buf_len&GENMASK!(27,0); }
pub unsafe fn cppi5_hdesc_reset_to_original(d: *mut cppi5_host_desc_t) { (*d).buf_ptr=(*d).org_buf_ptr; (*d).buf_info1=(*d).org_buf_len; }
pub unsafe fn cppi5_hdesc_link_hbdesc(d: *mut cppi5_host_desc_t, b: u64) { (*d).next_desc=b; }
pub unsafe fn cppi5_hdesc_get_next_hbdesc(d: *mut cppi5_host_desc_t) -> u64 { (*d).next_desc }
pub unsafe fn cppi5_hdesc_reset_hbdesc(d: *mut cppi5_host_desc_t) { (*d).hdr=core::mem::zeroed(); (*d).next_desc=0; }
pub unsafe fn cppi5_hdesc_epib_present(h: *mut cppi5_desc_hdr_t) -> bool { (*h).pkt_info0 & BIT!(29) != 0 }
pub unsafe fn cppi5_hdesc_get_psdata(d: *mut cppi5_host_desc_t) -> *mut u8 { if (*d).hdr.pkt_info0 & BIT!(28)!=0 {return core::ptr::null_mut();} let n=((*d).hdr.pkt_info0&GENMASK!(27,22))>>22; if n==0{return core::ptr::null_mut();} let mut p=(*d).epib.as_mut_ptr() as *mut u8; if cppi5_hdesc_epib_present(&mut (*d).hdr){p=p.add(16);} p }
pub unsafe fn cppi5_hdesc_get_swdata(d: *mut cppi5_host_desc_t) -> *mut u8 { let n=if (*d).hdr.pkt_info0&BIT!(28)==0 {((*d).hdr.pkt_info0&GENMASK!(27,22))>>22} else {0}; let mut p=(*d).epib.as_mut_ptr() as *mut u8; if cppi5_hdesc_epib_present(&mut (*d).hdr){p=p.add(16);} p.add((n<<2) as usize) }

pub type cppi5_tr_flags_t = u32;
#[repr(u32)] pub enum cppi5_tr_types { CPPI5_TR_TYPE0=0, CPPI5_TR_TYPE1, CPPI5_TR_TYPE2, CPPI5_TR_TYPE3, CPPI5_TR_TYPE4, CPPI5_TR_TYPE5, CPPI5_TR_TYPE8=8, CPPI5_TR_TYPE9, CPPI5_TR_TYPE10, CPPI5_TR_TYPE11, CPPI5_TR_TYPE15=15, CPPI5_TR_TYPE_MAX }
#[repr(u32)] pub enum cppi5_tr_event_size { CPPI5_TR_EVENT_SIZE_COMPLETION, CPPI5_TR_EVENT_SIZE_ICNT1_DEC, CPPI5_TR_EVENT_SIZE_ICNT2_DEC, CPPI5_TR_EVENT_SIZE_ICNT3_DEC, CPPI5_TR_EVENT_SIZE_MAX }
#[repr(u32)] pub enum cppi5_tr_trigger { CPPI5_TR_TRIGGER_NONE, CPPI5_TR_TRIGGER_GLOBAL0, CPPI5_TR_TRIGGER_GLOBAL1, CPPI5_TR_TRIGGER_LOCAL_EVENT, CPPI5_TR_TRIGGER_MAX }
#[repr(u32)] pub enum cppi5_tr_trigger_type { CPPI5_TR_TRIGGER_TYPE_ICNT1_DEC, CPPI5_TR_TRIGGER_TYPE_ICNT2_DEC, CPPI5_TR_TRIGGER_TYPE_ICNT3_DEC, CPPI5_TR_TRIGGER_TYPE_ALL, CPPI5_TR_TRIGGER_TYPE_MAX }
#[repr(C, packed)] pub struct cppi5_tr_type0_t { pub flags:u32,pub icnt0:u16,pub _reserved:u16,pub addr:u64 }
#[repr(C, packed)] pub struct cppi5_tr_type1_t { pub flags:u32,pub icnt0:u16,pub icnt1:u16,pub addr:u64,pub dim1:i32 }
#[repr(C, packed)] pub struct cppi5_tr_type2_t { pub flags:u32,pub icnt0:u16,pub icnt1:u16,pub addr:u64,pub dim1:i32,pub icnt2:u16,pub _reserved:u16,pub dim2:i32 }
#[repr(C, packed)] pub struct cppi5_tr_type3_t { pub flags:u32,pub icnt0:u16,pub icnt1:u16,pub addr:u64,pub dim1:i32,pub icnt2:u16,pub icnt3:u16,pub dim2:i32,pub dim3:i32 }
#[repr(C, packed)] pub struct cppi5_tr_type15_t { pub flags:u32,pub icnt0:u16,pub icnt1:u16,pub addr:u64,pub dim1:i32,pub icnt2:u16,pub icnt3:u16,pub dim2:i32,pub dim3:i32,pub _reserved:u32,pub ddim1:i32,pub daddr:u64,pub ddim2:i32,pub ddim3:i32,pub dicnt0:u16,pub dicnt1:u16,pub dicnt2:u16,pub dicnt3:u16 }
#[repr(C, packed)] pub struct cppi5_tr_resp_t { pub status:u8,pub _reserved:u8,pub cmd_id:u8,pub flags:u8 }
#[repr(u32)] pub enum cppi5_tr_resp_status_type { CPPI5_TR_RESPONSE_STATUS_NONE, CPPI5_TR_RESPONSE_STATUS_TRANSFER_ERR, CPPI5_TR_RESPONSE_STATUS_ABORTED_ERR, CPPI5_TR_RESPONSE_STATUS_SUBMISSION_ERR, CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_ERR, CPPI5_TR_RESPONSE_STATUS_TRANSFER_EXCEPTION, CPPI5_TR_RESPONSE_STATUS__TEARDOWN_FLUSH, CPPI5_TR_RESPONSE_STATUS_MAX }
#[repr(u32)] pub enum cppi5_tr_resp_status_submission { CPPI5_TR_RESPONSE_STATUS_SUBMISSION_ICNT0, CPPI5_TR_RESPONSE_STATUS_SUBMISSION_FIFO_FULL, CPPI5_TR_RESPONSE_STATUS_SUBMISSION_OWN, CPPI5_TR_RESPONSE_STATUS_SUBMISSION_MAX }
#[repr(u32)] pub enum cppi5_tr_resp_status_unsupported { CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_TR_TYPE, CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_STATIC, CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_EOL, CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_CFG_SPECIFIC, CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_AMODE, CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_ELTYPE, CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_DFMT, CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_SECTR, CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_AMODE_SPECIFIC, CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_MAX }
pub unsafe fn cppi5_trdesc_calc_size(n:u32,s:u32)->usize{s as usize*(n as usize+1)+core::mem::size_of::<cppi5_tr_resp_t>()*n as usize}
pub unsafe fn cppi5_trdesc_init(h:*mut cppi5_desc_hdr_t,n:u32,s:u32,ri:u32,rc:u32){(*h).pkt_info0=(3<<30)|((rc<<20)&GENMASK!(28,20))|((ri<<14)&GENMASK!(19,14))|((n-1)&GENMASK!(13,0)); (*h).pkt_info1|=(((s>>4).trailing_zeros())<<24)&GENMASK!(26,24);}
pub unsafe fn cppi5_tr_init(f:*mut u32,t:cppi5_tr_types,st:bool,w:bool,e:cppi5_tr_event_size,id:u32){*f=t as u32|(((e as u32)<<6)&GENMASK!(7,6))|((id<<16)&GENMASK!(23,16)); if st&&matches!(t,cppi5_tr_types::CPPI5_TR_TYPE8|cppi5_tr_types::CPPI5_TR_TYPE9){*f|=BIT!(4);} if w{*f|=BIT!(5);}}
pub unsafe fn cppi5_tr_set_trigger(f:*mut u32,a:cppi5_tr_trigger,at:cppi5_tr_trigger_type,b:cppi5_tr_trigger,bt:cppi5_tr_trigger_type){*f&=!(GENMASK!(9,8)|GENMASK!(11,10)|GENMASK!(13,12)|GENMASK!(15,14));*f|=((a as u32)<<8)&GENMASK!(9,8);*f|=((at as u32)<<10)&GENMASK!(11,10);*f|=((b as u32)<<12)&GENMASK!(13,12);*f|=((bt as u32)<<14)&GENMASK!(15,14);}
pub unsafe fn cppi5_tr_csf_set(f:*mut u32,c:u32){*f=(*f&!GENMASK!(31,24))|((c<<24)&GENMASK!(31,24));}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
