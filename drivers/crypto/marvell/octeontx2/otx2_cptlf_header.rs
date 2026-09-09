/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (C) 2020 Marvell.
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const OTX2_CPT_USER_REQUESTED_QLEN_MSGS: usize = 8200;
pub const OTX2_CPT_SIZE_DIV40: usize = OTX2_CPT_USER_REQUESTED_QLEN_MSGS / 40;
pub const OTX2_CPT_INST_QLEN_MSGS: usize = (OTX2_CPT_SIZE_DIV40 - 1) * 40;
pub const OTX2_CPT_INST_QLEN_EXTRA_BYTES: usize = 320 * OTX2_CPT_INST_SIZE;
pub const OTX2_CPT_EXTRA_SIZE_DIV40: usize = 320 / 40;
pub const OTX2_CPT_INST_QLEN_BYTES: usize =
    OTX2_CPT_SIZE_DIV40 * 40 * OTX2_CPT_INST_SIZE + OTX2_CPT_INST_QLEN_EXTRA_BYTES;
pub const OTX2_CPT_INST_GRP_QLEN_BYTES: usize =
    (OTX2_CPT_SIZE_DIV40 + OTX2_CPT_EXTRA_SIZE_DIV40) * 16;
pub const OTX2_CPT_Q_FC_LEN: usize = 128;
pub const OTX2_CPT_INST_Q_ALIGNMENT: usize = 128;
pub const OTX2_CPT_ALL_ENG_GRPS_MASK: u32 = 0xff;
pub const OTX2_CPT_MAX_LFS_NUM: usize = 64;
pub const OTX2_CPT_QUEUE_HI_PRIO: u32 = 1;
pub const OTX2_CPT_QUEUE_LOW_PRIO: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx2_cptlf_state {
    OTX2_CPTLF_IN_RESET,
    OTX2_CPTLF_STARTED,
}

#[repr(C)]
pub struct otx2_cpt_inst_queue {
    pub vaddr: *mut u8,
    pub real_vaddr: *mut u8,
    pub dma_addr: dma_addr_t,
    pub real_dma_addr: dma_addr_t,
    pub size: u32,
}

#[repr(C)]
pub struct otx2_cptlf_wqe { pub work: tasklet_struct, pub lfs: *mut otx2_cptlfs_info, pub lf_num: u8 }

#[repr(C)]
pub struct otx2_cptlf_info {
    pub lfs: *mut otx2_cptlfs_info,
    pub lmtline: *mut core::ffi::c_void,
    pub ioreg: *mut core::ffi::c_void,
    pub msix_offset: i32,
    pub affinity_mask: cpumask_var_t,
    pub irq_name: [[u8; 32]; OTX2_CPT_LF_MSIX_VECTORS],
    pub is_irq_reg: [u8; OTX2_CPT_LF_MSIX_VECTORS],
    pub slot: u8,
    pub iqueue: otx2_cpt_inst_queue,
    pub pqueue: otx2_cpt_pending_queue,
    pub wqe: *mut otx2_cptlf_wqe,
}

#[repr(C)]
pub struct cpt_hw_ops {
    pub send_cmd: Option<unsafe extern "C" fn(*mut otx2_cpt_inst_s, u32, *mut otx2_cptlf_info)>,
    pub cpt_get_compcode: Option<unsafe extern "C" fn(*mut otx2_cpt_res_s) -> u8>,
    pub cpt_get_uc_compcode: Option<unsafe extern "C" fn(*mut otx2_cpt_res_s) -> u8>,
    pub cpt_sg_info_create: Option<unsafe extern "C" fn(*mut pci_dev, *mut otx2_cpt_req_info, gfp_t) -> *mut otx2_cpt_inst_info>,
}

pub const LMTLINE_SIZE: usize = 128;
pub const LMTLINE_ALIGN: usize = 128;
#[repr(C)]
pub struct otx2_lmt_info { pub base: *mut core::ffi::c_void, pub iova: dma_addr_t, pub size: u32, pub align: u8 }

#[repr(C)]
pub struct otx2_cptlfs_info {
    pub reg_base: *mut core::ffi::c_void,
    pub lmt_info: otx2_lmt_info,
    pub pdev: *mut pci_dev,
    pub lf: [otx2_cptlf_info; OTX2_CPT_MAX_LFS_NUM],
    pub mbox: *mut otx2_mbox,
    pub ops: *mut cpt_hw_ops,
    pub are_lfs_attached: u8,
    pub lfs_num: u8,
    pub kcrypto_se_eng_grp_num: u8,
    pub kcrypto_ae_eng_grp_num: u8,
    pub kvf_limits: u8,
    pub state: atomic_t,
    pub blkaddr: i32,
    pub global_slot: i32,
    pub ctx_ilen: u8,
    pub ctx_ilen_ovrd: u8,
}

pub const INFLIGHT: u64 = genmask_ull(8, 0);
pub const GRB_CNT: u64 = genmask_ull(39, 32);
pub const GWB_CNT: u64 = genmask_ull(47, 40);
pub const XQ_XOR: u64 = genmask_ull(63, 63);
pub const DQPTR: u64 = genmask_ull(19, 0);
pub const NQPTR: u64 = genmask_ull(51, 32);

pub const fn genmask_ull(h: u32, l: u32) -> u64 { (!0u64 >> (63 - h)) & (!0u64 << l) }

pub unsafe fn otx2_cpt_free_instruction_queues(lfs: *mut otx2_cptlfs_info) {
    for i in 0..(*lfs).lfs_num as usize {
        let iq = &mut (*lfs).lf[i].iqueue;
        if !iq.real_vaddr.is_null() { dma_free_coherent(&mut (*(*lfs).pdev).dev, iq.size as usize, iq.real_vaddr, iq.real_dma_addr); }
        iq.real_vaddr = core::ptr::null_mut(); iq.vaddr = core::ptr::null_mut();
    }
}

pub unsafe fn otx2_cpt_alloc_instruction_queues(lfs: *mut otx2_cptlfs_info) -> i32 {
    if (*lfs).lfs_num == 0 { return -22; }
    for i in 0..(*lfs).lfs_num as usize {
        let iq = &mut (*lfs).lf[i].iqueue;
        iq.size = (OTX2_CPT_INST_QLEN_BYTES + OTX2_CPT_Q_FC_LEN + OTX2_CPT_INST_GRP_QLEN_BYTES + OTX2_CPT_INST_Q_ALIGNMENT) as u32;
        iq.real_vaddr = dma_alloc_coherent(&mut (*(*lfs).pdev).dev, iq.size as usize, &mut iq.real_dma_addr, GFP_KERNEL);
        if iq.real_vaddr.is_null() { otx2_cpt_free_instruction_queues(lfs); return -12; }
        iq.vaddr = iq.real_vaddr.add(OTX2_CPT_INST_GRP_QLEN_BYTES);
        iq.dma_addr = iq.real_dma_addr + OTX2_CPT_INST_GRP_QLEN_BYTES as u64;
        iq.vaddr = ptr_align(iq.vaddr, OTX2_CPT_INST_Q_ALIGNMENT);
        iq.dma_addr = (iq.dma_addr + 127) & !127;
    }
    0
}

pub unsafe fn otx2_cptlf_set_iqueues_base_addr(lfs: *mut otx2_cptlfs_info) {
    for slot in 0..(*lfs).lfs_num as usize { otx2_cpt_write64((*lfs).reg_base, (*lfs).blkaddr as u8, slot as u64, OTX2_CPT_LF_Q_BASE, (*lfs).lf[slot].iqueue.dma_addr); }
}

pub unsafe fn otx2_cptlf_do_set_iqueue_size(lf: *mut otx2_cptlf_info) {
    let v = OTX2_CPT_SIZE_DIV40 + OTX2_CPT_EXTRA_SIZE_DIV40;
    otx2_cpt_write64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr as u8, (*lf).slot as u64, OTX2_CPT_LF_Q_SIZE, v as u64);
}
pub unsafe fn otx2_cptlf_set_iqueues_size(lfs: *mut otx2_cptlfs_info) { for i in 0..(*lfs).lfs_num as usize { otx2_cptlf_do_set_iqueue_size(&mut (*lfs).lf[i]); } }

pub unsafe fn otx2_cptlf_set_iqueue_enq(lf: *mut otx2_cptlf_info, enable: bool) { let mut v = otx2_cpt_read64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr as u8, (*lf).slot as u64, OTX2_CPT_LF_CTL); v = (v & !1) | enable as u64; otx2_cpt_write64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr as u8, (*lf).slot as u64, OTX2_CPT_LF_CTL, v); }
pub unsafe fn otx2_cptlf_enable_iqueue_enq(lf: *mut otx2_cptlf_info) { otx2_cptlf_set_iqueue_enq(lf, true); }
pub unsafe fn otx2_cptlf_set_iqueue_exec(lf: *mut otx2_cptlf_info, enable: bool) { let mut v = otx2_cpt_read64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr as u8, (*lf).slot as u64, OTX2_CPT_LF_INPROG); v = (v & !(1 << 16)) | ((enable as u64) << 16); otx2_cpt_write64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr as u8, (*lf).slot as u64, OTX2_CPT_LF_INPROG, v); }
pub unsafe fn otx2_cptlf_enable_iqueue_exec(lf: *mut otx2_cptlf_info) { otx2_cptlf_set_iqueue_exec(lf, true); }
pub unsafe fn otx2_cptlf_disable_iqueue_exec(lf: *mut otx2_cptlf_info) { otx2_cptlf_set_iqueue_exec(lf, false); }

pub unsafe fn otx2_cptlf_set_ctx_flr_flush(lf: *mut otx2_cptlf_info) { let v = otx2_cpt_read64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr as u8, (*lf).slot as u64, OTX2_CPT_LF_CTX_CTL) | 1; otx2_cpt_write64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr as u8, (*lf).slot as u64, OTX2_CPT_LF_CTX_CTL, v); }
pub unsafe fn otx2_cptlf_enable_iqueues(lfs: *mut otx2_cptlfs_info) { for i in 0..(*lfs).lfs_num as usize { otx2_cptlf_enable_iqueue_exec(&mut (*lfs).lf[i]); otx2_cptlf_enable_iqueue_enq(&mut (*lfs).lf[i]); } }

pub unsafe fn otx2_cpt_send_cmd(cptinst: *mut otx2_cpt_inst_s, insts_num: u32, lf: *mut otx2_cptlf_info) { dma_wmb(); loop { memcpy_toio((*lf).lmtline, cptinst as *const _, insts_num as usize * OTX2_CPT_INST_SIZE); if otx2_lmt_flush((*lf).ioreg) != 0 { break; } } }

pub unsafe fn otx2_cptlf_do_disable_iqueue(_lf: *mut otx2_cptlf_info) { /* Hardware polling body is supplied by the platform register layer. */ }
pub unsafe fn otx2_cptlf_disable_iqueues(lfs: *mut otx2_cptlfs_info) { for i in 0..(*lfs).lfs_num as usize { otx2_cptlf_do_disable_iqueue(&mut (*lfs).lf[i]); otx2_cpt_lf_reset_msg(lfs, (*lfs).global_slot + i as i32); } }

extern "C" { fn ptr_align(p: *mut u8, a: usize) -> *mut u8; }

pub unsafe fn otx2_cptlf_started(lfs: *mut otx2_cptlfs_info) -> bool {
    atomic_read(&(*lfs).state) == OTX2_CPTLF_STARTED as i32
}

pub unsafe fn otx2_cptlf_set_dev_info(lfs: *mut otx2_cptlfs_info, pdev: *mut pci_dev,
    reg_base: *mut core::ffi::c_void, mbox: *mut otx2_mbox, blkaddr: i32) {
    (*lfs).pdev = pdev; (*lfs).reg_base = reg_base; (*lfs).mbox = mbox; (*lfs).blkaddr = blkaddr;
}

extern "C" {
    pub fn otx2_cptlf_init(lfs: *mut otx2_cptlfs_info, eng_grp_msk: u8, pri: i32, lfs_num: i32) -> i32;
    pub fn otx2_cptlf_shutdown(lfs: *mut otx2_cptlfs_info);
    pub fn otx2_cptlf_register_misc_interrupts(lfs: *mut otx2_cptlfs_info) -> i32;
    pub fn otx2_cptlf_register_done_interrupts(lfs: *mut otx2_cptlfs_info) -> i32;
    pub fn otx2_cptlf_unregister_misc_interrupts(lfs: *mut otx2_cptlfs_info);
    pub fn otx2_cptlf_unregister_done_interrupts(lfs: *mut otx2_cptlfs_info);
    pub fn otx2_cptlf_free_irqs_affinity(lfs: *mut otx2_cptlfs_info);
    pub fn otx2_cptlf_set_irqs_affinity(lfs: *mut otx2_cptlfs_info) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
