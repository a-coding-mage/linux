/* SPDX-License-Identifier: GPL-2.0 */
/* Interface based on include/net/netlink.h */

/* External kernel types and helpers are supplied by the surrounding translation. */

pub const KVMPPC_GSID_BLANK: u16 = 0x0000;
pub const KVMPPC_GSID_HOST_STATE_SIZE: u16 = 0x0001;
pub const KVMPPC_GSID_RUN_OUTPUT_MIN_SIZE: u16 = 0x0002;
pub const KVMPPC_GSID_LOGICAL_PVR: u16 = 0x0003;
pub const KVMPPC_GSID_TB_OFFSET: u16 = 0x0004;
pub const KVMPPC_GSID_PARTITION_TABLE: u16 = 0x0005;
pub const KVMPPC_GSID_PROCESS_TABLE: u16 = 0x0006;
pub const KVMPPC_GSID_L0_GUEST_HEAP: u16 = 0x0800;
pub const KVMPPC_GSID_L0_GUEST_HEAP_MAX: u16 = 0x0801;
pub const KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE: u16 = 0x0802;
pub const KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE_MAX: u16 = 0x0803;
pub const KVMPPC_GSID_L0_GUEST_PGTABLE_RECLAIM: u16 = 0x0804;
pub const KVMPPC_GSID_RUN_INPUT: u16 = 0x0C00;
pub const KVMPPC_GSID_RUN_OUTPUT: u16 = 0x0C01;
pub const KVMPPC_GSID_VPA: u16 = 0x0C02;

pub const fn KVMPPC_GSID_GPR(x: u16) -> u16 { 0x1000 + x }
pub const KVMPPC_GSID_HDEC_EXPIRY_TB: u16 = 0x1020;
pub const KVMPPC_GSID_NIA: u16 = 0x1021;
pub const KVMPPC_GSID_MSR: u16 = 0x1022;
pub const KVMPPC_GSID_LR: u16 = 0x1023;
pub const KVMPPC_GSID_XER: u16 = 0x1024;
pub const KVMPPC_GSID_CTR: u16 = 0x1025;
pub const KVMPPC_GSID_CFAR: u16 = 0x1026;
pub const KVMPPC_GSID_SRR0: u16 = 0x1027;
pub const KVMPPC_GSID_SRR1: u16 = 0x1028;
pub const KVMPPC_GSID_DAR: u16 = 0x1029;
pub const KVMPPC_GSID_DEC_EXPIRY_TB: u16 = 0x102A;
pub const KVMPPC_GSID_VTB: u16 = 0x102B;
pub const KVMPPC_GSID_LPCR: u16 = 0x102C;
pub const KVMPPC_GSID_HFSCR: u16 = 0x102D;
pub const KVMPPC_GSID_FSCR: u16 = 0x102E;
pub const KVMPPC_GSID_FPSCR: u16 = 0x102F;
pub const KVMPPC_GSID_DAWR0: u16 = 0x1030;
pub const KVMPPC_GSID_DAWR1: u16 = 0x1031;
pub const KVMPPC_GSID_CIABR: u16 = 0x1032;
pub const KVMPPC_GSID_PURR: u16 = 0x1033;
pub const KVMPPC_GSID_SPURR: u16 = 0x1034;
pub const KVMPPC_GSID_IC: u16 = 0x1035;
pub const KVMPPC_GSID_SPRG0: u16 = 0x1036;
pub const KVMPPC_GSID_SPRG1: u16 = 0x1037;
pub const KVMPPC_GSID_SPRG2: u16 = 0x1038;
pub const KVMPPC_GSID_SPRG3: u16 = 0x1039;
pub const KVMPPC_GSID_PPR: u16 = 0x103A;
pub const fn KVMPPC_GSID_MMCR(x: u16) -> u16 { 0x103B + x }
pub const KVMPPC_GSID_MMCRA: u16 = 0x103F;
pub const fn KVMPPC_GSID_SIER(x: u16) -> u16 { 0x1040 + x }
pub const KVMPPC_GSID_BESCR: u16 = 0x1043;
pub const KVMPPC_GSID_EBBHR: u16 = 0x1044;
pub const KVMPPC_GSID_EBBRR: u16 = 0x1045;
pub const KVMPPC_GSID_AMR: u16 = 0x1046;
pub const KVMPPC_GSID_IAMR: u16 = 0x1047;
pub const KVMPPC_GSID_AMOR: u16 = 0x1048;
pub const KVMPPC_GSID_UAMOR: u16 = 0x1049;
pub const KVMPPC_GSID_SDAR: u16 = 0x104A;
pub const KVMPPC_GSID_SIAR: u16 = 0x104B;
pub const KVMPPC_GSID_DSCR: u16 = 0x104C;
pub const KVMPPC_GSID_TAR: u16 = 0x104D;
pub const KVMPPC_GSID_DEXCR: u16 = 0x104E;
pub const KVMPPC_GSID_HDEXCR: u16 = 0x104F;
pub const KVMPPC_GSID_HASHKEYR: u16 = 0x1050;
pub const KVMPPC_GSID_HASHPKEYR: u16 = 0x1051;
pub const KVMPPC_GSID_CTRL: u16 = 0x1052;
pub const KVMPPC_GSID_DPDES: u16 = 0x1053;
pub const KVMPPC_GSID_CR: u16 = 0x2000;
pub const KVMPPC_GSID_PIDR: u16 = 0x2001;
pub const KVMPPC_GSID_DSISR: u16 = 0x2002;
pub const KVMPPC_GSID_VSCR: u16 = 0x2003;
pub const KVMPPC_GSID_VRSAVE: u16 = 0x2004;
pub const KVMPPC_GSID_DAWRX0: u16 = 0x2005;
pub const KVMPPC_GSID_DAWRX1: u16 = 0x2006;
pub const fn KVMPPC_GSID_PMC(x: u16) -> u16 { 0x2007 + x }
pub const KVMPPC_GSID_WORT: u16 = 0x200D;
pub const KVMPPC_GSID_PSPB: u16 = 0x200E;
pub const fn KVMPPC_GSID_VSRS(x: u16) -> u16 { 0x3000 + x }
pub const KVMPPC_GSID_HDAR: u16 = 0xF000;
pub const KVMPPC_GSID_HDSISR: u16 = 0xF001;
pub const KVMPPC_GSID_HEIR: u16 = 0xF002;
pub const KVMPPC_GSID_ASDR: u16 = 0xF003;

pub const KVMPPC_GSE_GUESTWIDE_START: u16 = KVMPPC_GSID_BLANK;
pub const KVMPPC_GSE_GUESTWIDE_END: u16 = KVMPPC_GSID_PROCESS_TABLE;
pub const KVMPPC_GSE_GUESTWIDE_COUNT: usize = (KVMPPC_GSE_GUESTWIDE_END - KVMPPC_GSE_GUESTWIDE_START + 1) as usize;
pub const KVMPPC_GSE_HOSTWIDE_START: u16 = KVMPPC_GSID_L0_GUEST_HEAP;
pub const KVMPPC_GSE_HOSTWIDE_END: u16 = KVMPPC_GSID_L0_GUEST_PGTABLE_RECLAIM;
pub const KVMPPC_GSE_HOSTWIDE_COUNT: usize = (KVMPPC_GSE_HOSTWIDE_END - KVMPPC_GSE_HOSTWIDE_START + 1) as usize;
pub const KVMPPC_GSE_META_START: u16 = KVMPPC_GSID_RUN_INPUT;
pub const KVMPPC_GSE_META_END: u16 = KVMPPC_GSID_VPA;
pub const KVMPPC_GSE_META_COUNT: usize = (KVMPPC_GSE_META_END - KVMPPC_GSE_META_START + 1) as usize;
pub const KVMPPC_GSE_DW_REGS_START: u16 = KVMPPC_GSID_GPR(0);
pub const KVMPPC_GSE_DW_REGS_END: u16 = KVMPPC_GSID_DPDES;
pub const KVMPPC_GSE_DW_REGS_COUNT: usize = (KVMPPC_GSE_DW_REGS_END - KVMPPC_GSE_DW_REGS_START + 1) as usize;
pub const KVMPPC_GSE_W_REGS_START: u16 = KVMPPC_GSID_CR;
pub const KVMPPC_GSE_W_REGS_END: u16 = KVMPPC_GSID_PSPB;
pub const KVMPPC_GSE_W_REGS_COUNT: usize = (KVMPPC_GSE_W_REGS_END - KVMPPC_GSE_W_REGS_START + 1) as usize;
pub const KVMPPC_GSE_VSRS_START: u16 = KVMPPC_GSID_VSRS(0);
pub const KVMPPC_GSE_VSRS_END: u16 = KVMPPC_GSID_VSRS(63);
pub const KVMPPC_GSE_VSRS_COUNT: usize = (KVMPPC_GSE_VSRS_END - KVMPPC_GSE_VSRS_START + 1) as usize;
pub const KVMPPC_GSE_INTR_REGS_START: u16 = KVMPPC_GSID_HDAR;
pub const KVMPPC_GSE_INTR_REGS_END: u16 = KVMPPC_GSID_ASDR;
pub const KVMPPC_GSE_INTR_REGS_COUNT: usize = (KVMPPC_GSE_INTR_REGS_END - KVMPPC_GSE_INTR_REGS_START + 1) as usize;
pub const KVMPPC_GSE_IDEN_COUNT: usize = KVMPPC_GSE_HOSTWIDE_COUNT + KVMPPC_GSE_GUESTWIDE_COUNT + KVMPPC_GSE_META_COUNT + KVMPPC_GSE_DW_REGS_COUNT + KVMPPC_GSE_W_REGS_COUNT + KVMPPC_GSE_VSRS_COUNT + KVMPPC_GSE_INTR_REGS_COUNT;

pub const KVMPPC_GS_CLASS_GUESTWIDE: u32 = 0x01;
pub const KVMPPC_GS_CLASS_HOSTWIDE: u32 = 0x02;
pub const KVMPPC_GS_CLASS_META: u32 = 0x04;
pub const KVMPPC_GS_CLASS_DWORD_REG: u32 = 0x08;
pub const KVMPPC_GS_CLASS_WORD_REG: u32 = 0x10;
pub const KVMPPC_GS_CLASS_VECTOR: u32 = 0x18;
pub const KVMPPC_GS_CLASS_INTR: u32 = 0x20;
pub const KVMPPC_GSE_BE32: u32 = 0;
pub const KVMPPC_GSE_BE64: u32 = 1;
pub const KVMPPC_GSE_VEC128: u32 = 2;
pub const KVMPPC_GSE_PARTITION_TABLE: u32 = 3;
pub const KVMPPC_GSE_PROCESS_TABLE: u32 = 4;
pub const KVMPPC_GSE_BUFFER: u32 = 5;
pub const __KVMPPC_GSE_TYPE_MAX: u32 = 6;
pub const KVMPPC_GS_FLAGS_WIDE: u32 = 0x01;
pub const KVMPPC_GS_FLAGS_HOST_WIDE: u32 = 0x02;

#[repr(C)]
pub struct kvmppc_gs_part_table { pub address: u64, pub ea_bits: u64, pub gpd_size: u64 }
#[repr(C)]
pub struct kvmppc_gs_proc_table { pub address: u64, pub gpd_size: u64 }
#[repr(C)]
pub struct kvmppc_gs_buff_info { pub address: u64, pub size: u64 }
#[repr(C, packed)]
pub struct kvmppc_gs_header { pub nelems: u32, pub data: [u8; 0] }
#[repr(C, packed)]
pub struct kvmppc_gs_elem { pub iden: u16, pub len: u16, pub data: [u8; 0] }
#[repr(C)]
pub struct kvmppc_gs_buff { pub capacity: usize, pub len: usize, pub guest_id: usize, pub vcpu_id: usize, pub hdr: *mut kvmppc_gs_header }
#[repr(C)]
pub struct kvmppc_gs_bitmap { pub bitmap: [usize; (KVMPPC_GSE_IDEN_COUNT + usize::BITS as usize - 1) / usize::BITS as usize] }
#[repr(C)]
pub struct kvmppc_gs_parser { pub iterator: kvmppc_gs_bitmap, pub gses: [*mut kvmppc_gs_elem; KVMPPC_GSE_IDEN_COUNT] }

pub const GSM_GUEST_WIDE: usize = 0x1;
pub const GSM_SEND: usize = 0x2;
pub const GSM_RECEIVE: usize = 0x4;
pub const GSM_GSB_OWNER: usize = 0x8;

pub struct kvmppc_gs_msg;
#[repr(C)]
pub struct kvmppc_gs_msg_ops {
    pub get_size: Option<unsafe extern "C" fn(*mut kvmppc_gs_msg) -> usize>,
    pub fill_info: Option<unsafe extern "C" fn(*mut kvmppc_gs_buff, *mut kvmppc_gs_msg) -> i32>,
    pub refresh_info: Option<unsafe extern "C" fn(*mut kvmppc_gs_msg, *mut kvmppc_gs_buff) -> i32>,
}
#[repr(C)]
pub struct kvmppc_gs_msg { pub bitmap: kvmppc_gs_bitmap, pub ops: *mut kvmppc_gs_msg_ops, pub flags: usize, pub data: *mut core::ffi::c_void }

extern "C" {
    pub fn kvmppc_gsid_size(iden: u16) -> u16;
    pub fn kvmppc_gsid_flags(iden: u16) -> usize;
    pub fn kvmppc_gsid_mask(iden: u16) -> u64;
    pub fn kvmppc_gsb_new(size: usize, guest_id: usize, vcpu_id: usize, flags: usize) -> *mut kvmppc_gs_buff;
    pub fn kvmppc_gsb_free(gsb: *mut kvmppc_gs_buff);
    pub fn kvmppc_gsb_put(gsb: *mut kvmppc_gs_buff, size: usize) -> *mut core::ffi::c_void;
    pub fn kvmppc_gsb_send(gsb: *mut kvmppc_gs_buff, flags: usize) -> i32;
    pub fn kvmppc_gsb_recv(gsb: *mut kvmppc_gs_buff, flags: usize) -> i32;
    pub fn __kvmppc_gse_put(gsb: *mut kvmppc_gs_buff, iden: u16, size: u16, data: *const core::ffi::c_void) -> i32;
    pub fn kvmppc_gse_parse(gsp: *mut kvmppc_gs_parser, gsb: *mut kvmppc_gs_buff) -> i32;
    pub fn kvmppc_gsbm_test(gsbm: *mut kvmppc_gs_bitmap, iden: u16) -> bool;
    pub fn kvmppc_gsbm_set(gsbm: *mut kvmppc_gs_bitmap, iden: u16);
    pub fn kvmppc_gsbm_clear(gsbm: *mut kvmppc_gs_bitmap, iden: u16);
    pub fn kvmppc_gsbm_next(gsbm: *mut kvmppc_gs_bitmap, prev: u16) -> u16;
    pub fn kvmppc_gsp_insert(gsp: *mut kvmppc_gs_parser, iden: u16, gse: *mut kvmppc_gs_elem);
    pub fn kvmppc_gsp_lookup(gsp: *mut kvmppc_gs_parser, iden: u16) -> *mut kvmppc_gs_elem;
    pub fn kvmppc_gsm_init(mgs: *mut kvmppc_gs_msg, ops: *mut kvmppc_gs_msg_ops, data: *mut core::ffi::c_void, flags: usize) -> i32;
    pub fn kvmppc_gsm_new(ops: *mut kvmppc_gs_msg_ops, data: *mut core::ffi::c_void, flags: usize, gfp_flags: usize) -> *mut kvmppc_gs_msg;
    pub fn kvmppc_gsm_free(gsm: *mut kvmppc_gs_msg);
    pub fn kvmppc_gsm_size(gsm: *mut kvmppc_gs_msg) -> usize;
    pub fn kvmppc_gsm_fill_info(gsm: *mut kvmppc_gs_msg, gsb: *mut kvmppc_gs_buff) -> i32;
    pub fn kvmppc_gsm_refresh_info(gsm: *mut kvmppc_gs_msg, gsb: *mut kvmppc_gs_buff) -> i32;
}

#[inline] pub unsafe fn kvmppc_gsb_header(gsb: *mut kvmppc_gs_buff) -> *mut kvmppc_gs_header { (*gsb).hdr }
#[inline] pub unsafe fn kvmppc_gsb_data(gsb: *mut kvmppc_gs_buff) -> *mut kvmppc_gs_elem { (*kvmppc_gsb_header(gsb)).data.as_mut_ptr() as *mut kvmppc_gs_elem }
#[inline] pub unsafe fn kvmppc_gsb_len(gsb: *mut kvmppc_gs_buff) -> usize { (*gsb).len }
#[inline] pub unsafe fn kvmppc_gsb_capacity(gsb: *mut kvmppc_gs_buff) -> usize { (*gsb).capacity }
#[inline] pub unsafe fn kvmppc_gsb_paddress(gsb: *mut kvmppc_gs_buff) -> u64 { kvmppc_gsb_header(gsb) as u64 }
#[inline] pub unsafe fn kvmppc_gsb_nelems(gsb: *mut kvmppc_gs_buff) -> u32 { u32::from_be((*kvmppc_gsb_header(gsb)).nelems) }
#[inline] pub unsafe fn kvmppc_gsb_reset(gsb: *mut kvmppc_gs_buff) { (*kvmppc_gsb_header(gsb)).nelems = 0u32.to_be(); (*gsb).len = core::mem::size_of::<kvmppc_gs_header>(); }
#[inline] pub unsafe fn kvmppc_gsb_data_len(gsb: *mut kvmppc_gs_buff) -> usize { (*gsb).len - core::mem::size_of::<kvmppc_gs_header>() }
#[inline] pub unsafe fn kvmppc_gsb_data_cap(gsb: *mut kvmppc_gs_buff) -> usize { (*gsb).capacity - core::mem::size_of::<kvmppc_gs_header>() }

#[inline] pub unsafe fn kvmppc_gse_iden(gse: *const kvmppc_gs_elem) -> u16 { u16::from_be((*gse).iden) }
#[inline] pub unsafe fn kvmppc_gse_len(gse: *const kvmppc_gs_elem) -> u16 { u16::from_be((*gse).len) }
#[inline] pub unsafe fn kvmppc_gse_total_len(gse: *const kvmppc_gs_elem) -> u16 { kvmppc_gse_len(gse) + core::mem::size_of::<kvmppc_gs_elem>() as u16 }
#[inline] pub const fn kvmppc_gse_total_size(size: u16) -> u16 { core::mem::size_of::<kvmppc_gs_elem>() as u16 + size }
#[inline] pub unsafe fn kvmppc_gse_data(gse: *const kvmppc_gs_elem) -> *mut core::ffi::c_void { (*gse).data.as_ptr() as *mut core::ffi::c_void }
#[inline] pub unsafe fn kvmppc_gse_ok(gse: *const kvmppc_gs_elem, remaining: i32) -> bool { remaining >= kvmppc_gse_total_len(gse) as i32 }
#[inline] pub unsafe fn kvmppc_gse_next(gse: *const kvmppc_gs_elem, remaining: *mut i32) -> *mut kvmppc_gs_elem { let len = core::mem::size_of::<kvmppc_gs_elem>() + kvmppc_gse_len(gse) as usize; *remaining -= len as i32; (gse as *const u8).add(len) as *mut kvmppc_gs_elem }

#[inline] pub unsafe fn kvmppc_gsm_include(gsm: *mut kvmppc_gs_msg, iden: u16) { kvmppc_gsbm_set(&mut (*gsm).bitmap, iden); }
#[inline] pub unsafe fn kvmppc_gsm_includes(gsm: *mut kvmppc_gs_msg, iden: u16) -> bool { kvmppc_gsbm_test(&mut (*gsm).bitmap, iden) }
#[inline] pub unsafe fn kvmppc_gsm_include_all(gsm: *mut kvmppc_gs_msg) { kvmppc_gsbm_fill(&mut (*gsm).bitmap); }
#[inline] pub unsafe fn kvmppc_gsm_reset(gsm: *mut kvmppc_gs_msg) { kvmppc_gsbm_zero(&mut (*gsm).bitmap); }

/* Bitmap operations are supplied by the kernel bitmap implementation. */
#[inline] pub unsafe fn kvmppc_gsbm_zero(gsbm: *mut kvmppc_gs_bitmap) { core::ptr::write_bytes((*gsbm).bitmap.as_mut_ptr(), 0, (*gsbm).bitmap.len()); }
#[inline] pub unsafe fn kvmppc_gsbm_fill(gsbm: *mut kvmppc_gs_bitmap) { for x in (*gsbm).bitmap.iter_mut() { *x = usize::MAX; } (*gsbm).bitmap[0] &= !1usize; }

#[inline] pub unsafe fn kvmppc_gsb_receive_data(gsb: *mut kvmppc_gs_buff, gsm: *mut kvmppc_gs_msg) -> i32 { kvmppc_gsb_reset(gsb); let mut rc = kvmppc_gsm_fill_info(gsm, gsb); if rc < 0 { return rc; } rc = kvmppc_gsb_recv(gsb, (*gsm).flags); if rc < 0 { return rc; } rc = kvmppc_gsm_refresh_info(gsm, gsb); if rc < 0 { return rc; } 0 }
#[inline] pub unsafe fn kvmppc_gsb_receive_datum(gsb: *mut kvmppc_gs_buff, gsm: *mut kvmppc_gs_msg, iden: u16) -> i32 { kvmppc_gsm_include(gsm, iden); let rc = kvmppc_gsb_receive_data(gsb, gsm); if rc < 0 { return rc; } kvmppc_gsm_reset(gsm); 0 }
#[inline] pub unsafe fn kvmppc_gsb_send_data(gsb: *mut kvmppc_gs_buff, gsm: *mut kvmppc_gs_msg) -> i32 { kvmppc_gsb_reset(gsb); let mut rc = kvmppc_gsm_fill_info(gsm, gsb); if rc < 0 { return rc; } rc = kvmppc_gsb_send(gsb, (*gsm).flags); rc }
#[inline] pub unsafe fn kvmppc_gsb_send_datum(gsb: *mut kvmppc_gs_buff, gsm: *mut kvmppc_gs_msg, iden: u16) -> i32 { kvmppc_gsm_include(gsm, iden); let rc = kvmppc_gsb_send_data(gsb, gsm); if rc < 0 { return rc; } kvmppc_gsm_reset(gsm); 0 }

#[inline] pub unsafe fn kvmppc_gse_put_be32(gsb: *mut kvmppc_gs_buff, iden: u16, val: u32) -> i32 { let tmp = val; __kvmppc_gse_put(gsb, iden, 4, &tmp as *const _ as *const _) }
#[inline] pub unsafe fn kvmppc_gse_put_u32(gsb: *mut kvmppc_gs_buff, iden: u16, mut val: u32) -> i32 { val &= kvmppc_gsid_mask(iden) as u32; kvmppc_gse_put_be32(gsb, iden, val.to_be()) }
#[inline] pub unsafe fn kvmppc_gse_put_be64(gsb: *mut kvmppc_gs_buff, iden: u16, val: u64) -> i32 { let tmp = val; __kvmppc_gse_put(gsb, iden, 8, &tmp as *const _ as *const _) }
#[inline] pub unsafe fn kvmppc_gse_put_u64(gsb: *mut kvmppc_gs_buff, iden: u16, mut val: u64) -> i32 { val &= kvmppc_gsid_mask(iden); kvmppc_gse_put_be64(gsb, iden, val.to_be()) }
#[inline] pub unsafe fn __kvmppc_gse_put_reg(gsb: *mut kvmppc_gs_buff, iden: u16, mut val: u64) -> i32 { val &= kvmppc_gsid_mask(iden); if kvmppc_gsid_size(iden) as usize == 8 { return kvmppc_gse_put_u64(gsb, iden, val); } if kvmppc_gsid_size(iden) as usize == 4 { let tmp = val as u32; if tmp as u64 != val { return -22; } return kvmppc_gse_put_u32(gsb, iden, tmp); } -22 }
#[inline] pub unsafe fn kvmppc_gse_put_part_table(gsb: *mut kvmppc_gs_buff, _iden: u16, val: kvmppc_gs_part_table) -> i32 { let tmp = [val.address.to_be(), val.ea_bits.to_be(), val.gpd_size.to_be()]; __kvmppc_gse_put(gsb, KVMPPC_GSID_PARTITION_TABLE, 24, tmp.as_ptr() as *const _) }
#[inline] pub unsafe fn kvmppc_gse_put_proc_table(gsb: *mut kvmppc_gs_buff, _iden: u16, val: kvmppc_gs_proc_table) -> i32 { let tmp = [val.address.to_be(), val.gpd_size.to_be()]; __kvmppc_gse_put(gsb, KVMPPC_GSID_PROCESS_TABLE, 16, tmp.as_ptr() as *const _) }
#[inline] pub unsafe fn kvmppc_gse_put_buff_info(gsb: *mut kvmppc_gs_buff, iden: u16, val: kvmppc_gs_buff_info) -> i32 { let tmp = [val.address.to_be(), val.size.to_be()]; __kvmppc_gse_put(gsb, iden, 16, tmp.as_ptr() as *const _) }
#[inline] pub unsafe fn kvmppc_gse_get_be32(gse: *const kvmppc_gs_elem) -> u32 { if kvmppc_gse_len(gse) != 4 { return 0; } (kvmppc_gse_data(gse) as *const u32).read_unaligned() }
#[inline] pub unsafe fn kvmppc_gse_get_u32(gse: *const kvmppc_gs_elem) -> u32 { u32::from_be(kvmppc_gse_get_be32(gse)) }
#[inline] pub unsafe fn kvmppc_gse_get_be64(gse: *const kvmppc_gs_elem) -> u64 { if kvmppc_gse_len(gse) != 8 { return 0; } (kvmppc_gse_data(gse) as *const u64).read_unaligned() }
#[inline] pub unsafe fn kvmppc_gse_get_u64(gse: *const kvmppc_gs_elem) -> u64 { u64::from_be(kvmppc_gse_get_be64(gse)) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
