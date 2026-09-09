/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * spu_csa.h: Definitions for SPU context save area (CSA).
 *
 * (C) Copyright IBM 2005
 *
 * Author: Mark Nutter <mnutter@us.ibm.com>
 */

/* Original header guard: _SPU_CSA_H_.  Original declarations were under
 * __KERNEL__ and, except for assembler use, __ASSEMBLER__. */

/* Total number of 128-bit registers. */
pub const NR_SPU_GPRS: usize = 128;
pub const NR_SPU_SPRS: usize = 9;
pub const NR_SPU_REGS_PAD: usize = 7;
pub const NR_SPU_SPILL_REGS: usize = 144;
pub const SIZEOF_SPU_SPILL_REGS: usize = NR_SPU_SPILL_REGS * 16;

pub const SPU_SAVE_COMPLETE: u32 = 0x3FFB;
pub const SPU_RESTORE_COMPLETE: u32 = 0x3FFC;

/* Definitions for various 'stopped' status conditions. */
pub const SPU_STOPPED_STATUS_P: u32 = 1;
pub const SPU_STOPPED_STATUS_I: u32 = 2;
pub const SPU_STOPPED_STATUS_H: u32 = 3;
pub const SPU_STOPPED_STATUS_S: u32 = 4;
pub const SPU_STOPPED_STATUS_S_I: u32 = 5;
pub const SPU_STOPPED_STATUS_S_P: u32 = 6;
pub const SPU_STOPPED_STATUS_P_H: u32 = 7;
pub const SPU_STOPPED_STATUS_P_I: u32 = 8;
pub const SPU_STOPPED_STATUS_R: u32 = 9;

/* Definitions for software decrementer status flag. */
pub const SPU_DECR_STATUS_RUNNING: u32 = 0x1;
pub const SPU_DECR_STATUS_WRAPPED: u32 = 0x2;

/// spu_reg128 - generic 128-bit register definition.
#[repr(C)]
pub struct spu_reg128 {
    pub slot: [u32; 4],
}

/// struct spu_lscsa - Local Store Context Save Area.
///
/// The `ls` member is required by the C source to be aligned to 65536 bytes.
#[repr(C)]
pub struct spu_lscsa {
    pub gprs: [spu_reg128; 128],
    pub fpcr: spu_reg128,
    pub decr: spu_reg128,
    pub decr_status: spu_reg128,
    pub ppu_mb: spu_reg128,
    pub ppuint_mb: spu_reg128,
    pub tag_mask: spu_reg128,
    pub event_mask: spu_reg128,
    pub srr0: spu_reg128,
    pub stopped_status: spu_reg128,
    /* 'ls' must be page-aligned on all configurations (C aligned(65536)). */
    pub ls: [u8; LS_SIZE],
}

/* Original declaration is excluded when __SPU__ is defined. */
#[repr(C)]
pub struct spu_problem_collapsed {
    pub spc_mssync_RW: u64,
    pub mfc_lsa_W: u32,
    pub unused_pad0: u32,
    pub mfc_ea_W: u64,
    pub mfc_union_W: mfc_tag_size_class_cmd,
    pub dma_qstatus_R: u32,
    pub dma_querytype_RW: u32,
    pub dma_querymask_RW: u32,
    pub dma_tagstatus_R: u32,
    pub pu_mb_R: u32,
    pub spu_mb_W: u32,
    pub mb_stat_R: u32,
    pub spu_runcntl_RW: u32,
    pub spu_status_R: u32,
    pub spu_spc_R: u32,
    pub spu_npc_RW: u32,
    pub signal_notify1: u32,
    pub signal_notify2: u32,
    pub unused_pad1: u32,
}

#[repr(C)]
pub struct spu_priv1_collapsed {
    pub mfc_sr1_RW: u64, pub mfc_lpid_RW: u64, pub spu_idr_RW: u64,
    pub mfc_vr_RO: u64, pub spu_vr_RO: u64,
    pub int_mask_class0_RW: u64, pub int_mask_class1_RW: u64, pub int_mask_class2_RW: u64,
    pub int_stat_class0_RW: u64, pub int_stat_class1_RW: u64, pub int_stat_class2_RW: u64,
    pub int_route_RW: u64, pub mfc_atomic_flush_RW: u64,
    pub resource_allocation_groupID_RW: u64, pub resource_allocation_enable_RW: u64,
    pub mfc_fir_R: u64, pub mfc_fir_status_or_W: u64, pub mfc_fir_status_and_W: u64,
    pub mfc_fir_mask_R: u64, pub mfc_fir_mask_or_W: u64, pub mfc_fir_mask_and_W: u64,
    pub mfc_fir_chkstp_enable_RW: u64, pub smf_sbi_signal_sel: u64, pub smf_ato_signal_sel: u64,
    pub tlb_index_hint_RO: u64, pub tlb_index_W: u64, pub tlb_vpn_RW: u64, pub tlb_rpn_RW: u64,
    pub tlb_invalidate_entry_W: u64, pub tlb_invalidate_all_W: u64, pub smm_hid: u64,
    pub mfc_accr_RW: u64, pub mfc_dsisr_RW: u64, pub mfc_dar_RW: u64, pub rmt_index_RW: u64,
    pub rmt_data1_RW: u64, pub mfc_dsir_R: u64, pub mfc_lsacr_RW: u64, pub mfc_lscrr_R: u64,
    pub mfc_tclass_id_RW: u64, pub mfc_rm_boundary: u64, pub smf_dma_signal_sel: u64,
    pub smm_signal_sel: u64, pub mfc_cer_R: u64, pub pu_ecc_cntl_RW: u64, pub pu_ecc_stat_RW: u64,
    pub spu_ecc_addr_RW: u64, pub spu_err_mask_RW: u64, pub spu_trig0_sel: u64,
    pub spu_trig1_sel: u64, pub spu_trig2_sel: u64, pub spu_trig3_sel: u64, pub spu_trace_sel: u64,
    pub spu_event0_sel: u64, pub spu_event1_sel: u64, pub spu_event2_sel: u64, pub spu_event3_sel: u64,
    pub spu_trace_cntl: u64,
}

#[repr(C)]
pub struct spu_priv2_collapsed {
    pub slb_index_W: u64, pub slb_esid_RW: u64, pub slb_vsid_RW: u64,
    pub slb_invalidate_entry_W: u64, pub slb_invalidate_all_W: u64,
    pub spuq: [mfc_cq_sr; 16], pub puq: [mfc_cq_sr; 8],
    pub mfc_control_RW: u64, pub puint_mb_R: u64, pub spu_privcntl_RW: u64,
    pub spu_lslr_RW: u64, pub spu_chnlcntptr_RW: u64, pub spu_chnlcnt_RW: u64,
    pub spu_chnldata_RW: u64, pub spu_cfg_RW: u64, pub spu_tag_status_query_RW: u64,
    pub spu_cmd_buf1_RW: u64, pub spu_cmd_buf2_RW: u64, pub spu_atomic_status_RW: u64,
}

#[repr(C)]
pub struct spu_state {
    pub lscsa: *mut spu_lscsa,
    pub prob: spu_problem_collapsed,
    pub priv1: spu_priv1_collapsed,
    pub priv2: spu_priv2_collapsed,
    pub spu_chnlcnt_RW: [u64; 32],
    pub spu_chnldata_RW: [u64; 32],
    pub spu_mailbox_data: [u32; 4],
    pub pu_mailbox_data: [u32; 1],
    pub class_0_dar: u64,
    pub class_0_pending: u64,
    pub class_1_dar: u64,
    pub class_1_dsisr: u64,
    pub suspend_time: usize,
    pub register_lock: spinlock_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
