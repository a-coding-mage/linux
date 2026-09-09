/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Copyright(c) 2016 - 2018 Intel Corporation. */

// Translated from ib_hdrs.h.  The Linux/RDMA dependency types and helpers are
// supplied by the surrounding translation unit.

pub const IB_SEQ_NAK: u32 = 3 << 29;
pub const IB_RNR_NAK: u32 = 0x20;
pub const IB_NAK_PSN_ERROR: u32 = 0x60;
pub const IB_NAK_INVALID_REQUEST: u32 = 0x61;
pub const IB_NAK_REMOTE_ACCESS_ERROR: u32 = 0x62;
pub const IB_NAK_REMOTE_OPERATIONAL_ERROR: u32 = 0x63;
pub const IB_NAK_INVALID_RD_REQUEST: u32 = 0x64;

pub const IB_BTH_REQ_ACK: u32 = 1 << 31;
pub const IB_BTH_SOLICITED: u32 = 1 << 23;
pub const IB_BTH_MIG_REQ: u32 = 1 << 22;
pub const IB_GRH_VERSION: u32 = 6;
pub const IB_GRH_VERSION_MASK: u32 = 0xF;
pub const IB_GRH_VERSION_SHIFT: u32 = 28;
pub const IB_GRH_TCLASS_MASK: u32 = 0xFF;
pub const IB_GRH_TCLASS_SHIFT: u32 = 20;
pub const IB_GRH_FLOW_MASK: u32 = 0xFFFFF;
pub const IB_GRH_FLOW_SHIFT: u32 = 0;
pub const IB_GRH_NEXT_HDR: u32 = 0x1B;
pub const IB_FECN_SHIFT: u32 = 31;
pub const IB_FECN_MASK: u32 = 1;
pub const IB_FECN_SMASK: u32 = 1 << IB_FECN_SHIFT;
pub const IB_BECN_SHIFT: u32 = 30;
pub const IB_BECN_MASK: u32 = 1;
pub const IB_BECN_SMASK: u32 = 1 << IB_BECN_SHIFT;
pub const IB_AETH_CREDIT_SHIFT: u32 = 24;
pub const IB_AETH_CREDIT_MASK: u32 = 0x1F;
pub const IB_AETH_CREDIT_INVAL: u32 = 0x1F;
pub const IB_AETH_NAK_SHIFT: u32 = 29;
pub const IB_MSN_MASK: u32 = 0xFFFFFF;

#[repr(C, packed)]
pub struct ib_reth { pub vaddr: __be64, pub rkey: __be32, pub length: __be32 }

#[repr(C, packed)]
pub struct ib_atomic_eth {
    pub vaddr: __be64, pub rkey: __be32, pub swap_data: __be64, pub compare_data: __be64,
}

#[repr(C)]
pub union ib_ehdrs {
    pub ud: ib_ehdrs_ud,
    pub rc: ib_ehdrs_rc,
    pub at: ib_ehdrs_at,
    pub imm_data: __be32,
    pub aeth: __be32,
    pub ieth: __be32,
    pub atomic_eth: ib_atomic_eth,
    pub tid_rdma: ib_ehdrs_tid_rdma,
}
#[repr(C)] pub struct ib_ehdrs_ud { pub deth: [__be32; 2], pub imm_data: __be32 }
#[repr(C)] pub struct ib_ehdrs_rc { pub reth: ib_reth, pub imm_data: __be32 }
#[repr(C, packed)] pub struct ib_ehdrs_at { pub aeth: __be32, pub atomic_ack_eth: __be64 }
#[repr(C)] pub union ib_ehdrs_tid_rdma {
    pub r_req: tid_rdma_read_req, pub r_rsp: tid_rdma_read_resp,
    pub w_req: tid_rdma_write_req, pub w_rsp: tid_rdma_write_resp,
    pub w_data: tid_rdma_write_data, pub resync: tid_rdma_resync, pub ack: tid_rdma_ack,
}

#[repr(C, packed)] pub struct ib_other_headers { pub bth: [__be32; 3], pub u: ib_ehdrs }
#[repr(C, packed)] pub struct ib_header { pub lrh: [__be16; 4], pub u: ib_header_u }
#[repr(C)] pub union ib_header_u { pub l: ib_header_l, pub oth: ib_other_headers }
#[repr(C, packed)] pub struct ib_header_l { pub grh: ib_grh, pub oth: ib_other_headers }

extern "C" {
    fn get_unaligned_be64(p: *const __be64) -> u64;
    fn put_unaligned_be64(val: u64, p: *mut __be64);
    fn be16_to_cpu(v: __be16) -> u16;
    fn be32_to_cpu(v: __be32) -> u32;
    fn cpu_to_be32(v: u32) -> __be32;
}

pub unsafe fn ib_u64_get(p: *const __be64) -> u64 { get_unaligned_be64(p) }
pub unsafe fn ib_u64_put(val: u64, p: *mut __be64) { put_unaligned_be64(val, p) }
pub unsafe fn get_ib_reth_vaddr(reth: *mut ib_reth) -> u64 { ib_u64_get(&raw const (*reth).vaddr) }
pub unsafe fn put_ib_reth_vaddr(val: u64, reth: *mut ib_reth) { ib_u64_put(val, &raw mut (*reth).vaddr) }
pub unsafe fn get_ib_ateth_vaddr(a: *mut ib_atomic_eth) -> u64 { ib_u64_get(&raw const (*a).vaddr) }
pub unsafe fn put_ib_ateth_vaddr(val: u64, a: *mut ib_atomic_eth) { ib_u64_put(val, &raw mut (*a).vaddr) }
pub unsafe fn get_ib_ateth_swap(a: *mut ib_atomic_eth) -> u64 { ib_u64_get(&raw const (*a).swap_data) }
pub unsafe fn put_ib_ateth_swap(val: u64, a: *mut ib_atomic_eth) { ib_u64_put(val, &raw mut (*a).swap_data) }
pub unsafe fn get_ib_ateth_compare(a: *mut ib_atomic_eth) -> u64 { ib_u64_get(&raw const (*a).compare_data) }
pub unsafe fn put_ib_ateth_compare(val: u64, a: *mut ib_atomic_eth) { ib_u64_put(val, &raw mut (*a).compare_data) }

pub const IB_LNH_MASK: u32 = 3; pub const IB_SC_MASK: u32 = 0xf; pub const IB_SC_SHIFT: u32 = 12;
pub const IB_SC5_MASK: u32 = 0x10; pub const IB_SL_MASK: u32 = 0xf; pub const IB_SL_SHIFT: u32 = 4;
pub const IB_LVER_MASK: u32 = 0xf; pub const IB_LVER_SHIFT: u32 = 8;
pub unsafe fn ib_get_lnh(h: *mut ib_header) -> u8 { (be16_to_cpu((*h).lrh[0]) as u32 & IB_LNH_MASK) as u8 }
pub unsafe fn ib_get_sc(h: *mut ib_header) -> u8 { ((be16_to_cpu((*h).lrh[0]) as u32 >> IB_SC_SHIFT) & IB_SC_MASK) as u8 }
pub fn ib_is_sc5(sc5: u16) -> bool { (sc5 & IB_SC5_MASK as u16) != 0 }
pub unsafe fn ib_get_sl(h: *mut ib_header) -> u8 { ((be16_to_cpu((*h).lrh[0]) as u32 >> IB_SL_SHIFT) & IB_SL_MASK) as u8 }
pub unsafe fn ib_get_dlid(h: *mut ib_header) -> u16 { be16_to_cpu((*h).lrh[1]) }
pub unsafe fn ib_get_slid(h: *mut ib_header) -> u16 { be16_to_cpu((*h).lrh[3]) }
pub unsafe fn ib_get_lver(h: *mut ib_header) -> u8 { ((be16_to_cpu((*h).lrh[0]) as u32 >> IB_LVER_SHIFT) & IB_LVER_MASK) as u8 }
pub unsafe fn ib_get_qkey(oh: *mut ib_other_headers) -> u32 { be32_to_cpu((*oh).u.ud.deth[0]) }
pub unsafe fn ib_get_sqpn(oh: *mut ib_other_headers) -> u32 { be32_to_cpu((*oh).u.ud.deth[1]) & IB_QPN_MASK }

pub const IB_BTH_OPCODE_MASK: u32 = 0xff; pub const IB_BTH_OPCODE_SHIFT: u32 = 24; pub const IB_BTH_PAD_MASK: u32 = 3;
pub const IB_BTH_PKEY_MASK: u32 = 0xffff; pub const IB_BTH_PAD_SHIFT: u32 = 20; pub const IB_BTH_A_MASK: u32 = 1;
pub const IB_BTH_A_SHIFT: u32 = 31; pub const IB_BTH_M_MASK: u32 = 1; pub const IB_BTH_M_SHIFT: u32 = 22;
pub const IB_BTH_SE_MASK: u32 = 1; pub const IB_BTH_SE_SHIFT: u32 = 23; pub const IB_BTH_TVER_MASK: u32 = 0xf;
pub const IB_BTH_TVER_SHIFT: u32 = 16; pub const IB_BTH_OPCODE_CNP: u32 = 0x81;
pub unsafe fn ib_bth_get_pad(o: *mut ib_other_headers) -> u8 { ((be32_to_cpu((*o).bth[0]) >> IB_BTH_PAD_SHIFT) & IB_BTH_PAD_MASK) as u8 }
pub unsafe fn ib_bth_get_pkey(o: *mut ib_other_headers) -> u16 { (be32_to_cpu((*o).bth[0]) & IB_BTH_PKEY_MASK) as u16 }
pub unsafe fn ib_bth_get_opcode(o: *mut ib_other_headers) -> u8 { ((be32_to_cpu((*o).bth[0]) >> IB_BTH_OPCODE_SHIFT) & IB_BTH_OPCODE_MASK) as u8 }
pub unsafe fn ib_bth_get_ackreq(o: *mut ib_other_headers) -> u8 { ((be32_to_cpu((*o).bth[2]) >> IB_BTH_A_SHIFT) & IB_BTH_A_MASK) as u8 }
pub unsafe fn ib_bth_get_migreq(o: *mut ib_other_headers) -> u8 { ((be32_to_cpu((*o).bth[0]) >> IB_BTH_M_SHIFT) & IB_BTH_M_MASK) as u8 }
pub unsafe fn ib_bth_get_se(o: *mut ib_other_headers) -> u8 { ((be32_to_cpu((*o).bth[0]) >> IB_BTH_SE_SHIFT) & IB_BTH_SE_MASK) as u8 }
pub unsafe fn ib_bth_get_psn(o: *mut ib_other_headers) -> u32 { be32_to_cpu((*o).bth[2]) }
pub unsafe fn ib_bth_get_qpn(o: *mut ib_other_headers) -> u32 { be32_to_cpu((*o).bth[1]) & IB_QPN_MASK }
pub unsafe fn ib_bth_get_becn(o: *mut ib_other_headers) -> bool { ((*o).bth[1] & cpu_to_be32(IB_BECN_SMASK)) != 0 }
pub unsafe fn ib_bth_get_fecn(o: *mut ib_other_headers) -> bool { ((*o).bth[1] & cpu_to_be32(IB_FECN_SMASK)) != 0 }
pub unsafe fn ib_bth_get_tver(o: *mut ib_other_headers) -> u8 { ((be32_to_cpu((*o).bth[0]) >> IB_BTH_TVER_SHIFT) & IB_BTH_TVER_MASK) as u8 }
pub unsafe fn ib_bth_is_solicited(o: *mut ib_other_headers) -> bool { ((*o).bth[0] & cpu_to_be32(IB_BTH_SOLICITED)) != 0 }
pub unsafe fn ib_bth_is_migration(o: *mut ib_other_headers) -> bool { ((*o).bth[0] & cpu_to_be32(IB_BTH_MIG_REQ)) != 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
