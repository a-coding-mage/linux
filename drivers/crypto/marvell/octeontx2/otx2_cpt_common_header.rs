/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (C) 2020 Marvell.
 */

// Linux and local header dependencies are supplied by the surrounding translation.

pub const OTX2_CPT_MAX_VFS_NUM: u32 = 128;
#[inline]
pub const fn otx2_cpt_rvu_func_addr_s(blk: u64, slot: u64, offs: u64) -> u64 {
    (blk << 20) | (slot << 12) | offs
}

#[inline]
pub fn otx2_cpt_rvu_pffunc(pdev: *mut pci_dev, pf: u64, func: u64) -> u16 {
    rvu_make_pcifunc(pdev, pf, func)
}

pub const OTX2_CPT_INVALID_CRYPTO_ENG_GRP: u8 = 0xff;
pub const OTX2_CPT_NAME_LENGTH: usize = 64;
pub const OTX2_CPT_DMA_MINALIGN: usize = 128;
pub const CN10K_MBOX: usize = 0;
pub const CN10K_LMTST: usize = 1;
pub const BAD_OTX2_CPT_ENG_TYPE: u32 = OTX2_CPT_MAX_ENG_TYPES as u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum otx2_cpt_eng_type {
    OTX2_CPT_AE_TYPES = 1,
    OTX2_CPT_SE_TYPES = 2,
    OTX2_CPT_IE_TYPES = 3,
    OTX2_CPT_MAX_ENG_TYPES,
}

pub const MBOX_MSG_RX_INLINE_IPSEC_LF_CFG: u16 = 0xbfe;
pub const MBOX_MSG_GET_ENG_GRP_NUM: u16 = 0xbff;
pub const MBOX_MSG_GET_CAPS: u16 = 0xbfd;
pub const MBOX_MSG_GET_KVF_LIMITS: u16 = 0xbfc;

#[repr(C)]
pub struct otx2_cpt_rx_inline_lf_cfg {
    pub hdr: mbox_msghdr,
    pub sso_pf_func: u16,
    pub param1: u16,
    pub param2: u16,
    pub opcode: u16,
    pub credit: u32,
    pub credit_th: u32,
    pub bpid: u16,
    pub reserved: u32,
    pub ctx_ilen_valid: u8,
    pub ctx_ilen: u8,
}

#[repr(C)]
pub struct otx2_cpt_egrp_num_msg { pub hdr: mbox_msghdr, pub eng_type: u8 }
#[repr(C)]
pub struct otx2_cpt_egrp_num_rsp { pub hdr: mbox_msghdr, pub eng_type: u8, pub eng_grp_num: u8 }
#[repr(C)]
pub struct otx2_cpt_kvf_limits_msg { pub hdr: mbox_msghdr }
#[repr(C)]
pub struct otx2_cpt_kvf_limits_rsp { pub hdr: mbox_msghdr, pub kvf_limits: u8 }

#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cpt_eng_caps { pub u: u64, pub bits: otx2_cpt_eng_caps_bits }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_eng_caps_bits { pub value: u64 }

#[repr(C)]
pub struct otx2_cpt_caps_msg { pub hdr: mbox_msghdr }
#[repr(C)]
pub struct otx2_cpt_caps_rsp {
    pub hdr: mbox_msghdr,
    pub cpt_pf_drv_version: u16,
    pub cpt_revision: u8,
    pub eng_caps: [otx2_cpt_eng_caps; OTX2_CPT_MAX_ENG_TYPES as usize],
}

#[inline]
pub unsafe fn otx2_cpt_write64(reg_base: *mut u8, blk: u64, slot: u64, offs: u64, val: u64) {
    writeq_relaxed(val, reg_base.add(otx2_cpt_rvu_func_addr_s(blk, slot, offs) as usize));
}
#[inline]
pub unsafe fn otx2_cpt_read64(reg_base: *mut u8, blk: u64, slot: u64, offs: u64) -> u64 {
    readq_relaxed(reg_base.add(otx2_cpt_rvu_func_addr_s(blk, slot, offs) as usize))
}

#[inline] pub unsafe fn is_dev_otx2(pdev: *mut pci_dev) -> bool { (*pdev).device == OTX2_CPT_PCI_PF_DEVICE_ID || (*pdev).device == OTX2_CPT_PCI_VF_DEVICE_ID }
#[inline] pub unsafe fn is_dev_cn10ka(pdev: *mut pci_dev) -> bool { (*pdev).subsystem_device == CPT_PCI_SUBSYS_DEVID_CN10K_A }
#[inline] pub unsafe fn is_dev_cn10ka_ax(pdev: *mut pci_dev) -> bool { is_dev_cn10ka(pdev) && (((*pdev).revision & 0xff) == 4 || ((*pdev).revision & 0xff) == 0x50 || ((*pdev).revision & 0xff) == 0x51) }
#[inline] pub unsafe fn is_dev_cn10kb(pdev: *mut pci_dev) -> bool { (*pdev).subsystem_device == CPT_PCI_SUBSYS_DEVID_CN10K_B }
#[inline] pub unsafe fn is_dev_cn10ka_b0(pdev: *mut pci_dev) -> bool { is_dev_cn10ka(pdev) && ((*pdev).revision & 0xff) == 0x54 }
#[inline] pub unsafe fn otx2_cpt_set_hw_caps(pdev: *mut pci_dev, cap_flag: *mut c_ulong) { if !is_dev_otx2(pdev) { __set_bit(CN10K_MBOX, cap_flag); __set_bit(CN10K_LMTST, cap_flag); } }
#[inline] pub unsafe fn cpt_is_errata_38550_exists(pdev: *mut pci_dev) -> bool { is_dev_otx2(pdev) || is_dev_cn10ka_ax(pdev) }
#[inline] pub unsafe fn cpt_feature_sgv2(pdev: *mut pci_dev) -> bool { !is_dev_otx2(pdev) && !is_dev_cn10ka_ax(pdev) }

extern "C" {
    pub fn otx2_cpt_send_ready_msg(mbox: *mut otx2_mbox, pdev: *mut pci_dev) -> i32;
    pub fn otx2_cpt_send_mbox_msg(mbox: *mut otx2_mbox, pdev: *mut pci_dev) -> i32;
    pub fn otx2_cpt_send_af_reg_requests(mbox: *mut otx2_mbox, pdev: *mut pci_dev) -> i32;
    pub fn otx2_cpt_add_write_af_reg(mbox: *mut otx2_mbox, pdev: *mut pci_dev, reg: u64, val: u64, blkaddr: i32) -> i32;
    pub fn otx2_cpt_read_af_reg(mbox: *mut otx2_mbox, pdev: *mut pci_dev, reg: u64, val: *mut u64, blkaddr: i32) -> i32;
    pub fn otx2_cpt_write_af_reg(mbox: *mut otx2_mbox, pdev: *mut pci_dev, reg: u64, val: u64, blkaddr: i32) -> i32;
    pub fn otx2_cpt_attach_rscrs_msg(lfs: *mut otx2_cptlfs_info) -> i32;
    pub fn otx2_cpt_detach_rsrcs_msg(lfs: *mut otx2_cptlfs_info) -> i32;
    pub fn otx2_cpt_msix_offset_msg(lfs: *mut otx2_cptlfs_info) -> i32;
    pub fn otx2_cpt_sync_mbox_msg(mbox: *mut otx2_mbox) -> i32;
    pub fn otx2_cpt_lf_reset_msg(lfs: *mut otx2_cptlfs_info, slot: i32) -> i32;
    pub fn otx2_cpt_lmtst_tbl_setup_msg(lfs: *mut otx2_cptlfs_info) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
