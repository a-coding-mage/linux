/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from altera_edac.h. External kernel types are supplied elsewhere. */

// linux/arm-smccc.h, linux/edac.h, and linux/types.h dependencies are external.

pub const CV_CTLCFG_OFST: u32 = 0x00;
pub const CV_CTLCFG_ECC_EN: u32 = 0x400;
pub const CV_CTLCFG_ECC_CORR_EN: u32 = 0x800;
pub const CV_CTLCFG_GEN_SB_ERR: u32 = 0x2000;
pub const CV_CTLCFG_GEN_DB_ERR: u32 = 0x4000;
pub const CV_CTLCFG_ECC_AUTO_EN: u32 = CV_CTLCFG_ECC_EN;
pub const CV_DRAMADDRW_OFST: u32 = 0x2C;
pub const DRAMADDRW_COLBIT_MASK: u32 = 0x001F;
pub const DRAMADDRW_COLBIT_SHIFT: u32 = 0;
pub const DRAMADDRW_ROWBIT_MASK: u32 = 0x03E0;
pub const DRAMADDRW_ROWBIT_SHIFT: u32 = 5;
pub const CV_DRAMADDRW_BANKBIT_MASK: u32 = 0x1C00;
pub const CV_DRAMADDRW_BANKBIT_SHIFT: u32 = 10;
pub const CV_DRAMADDRW_CSBIT_MASK: u32 = 0xE000;
pub const CV_DRAMADDRW_CSBIT_SHIFT: u32 = 13;
pub const CV_DRAMIFWIDTH_OFST: u32 = 0x30;
pub const CV_DRAMIFWIDTH_16B_ECC: u32 = 24;
pub const CV_DRAMIFWIDTH_32B_ECC: u32 = 40;
pub const CV_DRAMSTS_OFST: u32 = 0x38;
pub const CV_DRAMSTS_SBEERR: u32 = 0x04;
pub const CV_DRAMSTS_DBEERR: u32 = 0x08;
pub const CV_DRAMSTS_CORR_DROP: u32 = 0x10;
pub const CV_DRAMINTR_OFST: u32 = 0x3C;
pub const CV_DRAMINTR_INTREN: u32 = 0x01;
pub const CV_DRAMINTR_SBEMASK: u32 = 0x02;
pub const CV_DRAMINTR_DBEMASK: u32 = 0x04;
pub const CV_DRAMINTR_CORRDROPMASK: u32 = 0x08;
pub const CV_DRAMINTR_INTRCLR: u32 = 0x10;
pub const CV_SBECOUNT_OFST: u32 = 0x40;
pub const CV_DBECOUNT_OFST: u32 = 0x44;
pub const CV_ERRADDR_OFST: u32 = 0x48;

pub const A10_ECCCTRL1_OFST: u32 = 0x00;
pub const A10_ECCCTRL1_ECC_EN: u32 = 0x001;
pub const A10_ECCCTRL1_CNT_RST: u32 = 0x010;
pub const A10_ECCCTRL1_AWB_CNT_RST: u32 = 0x100;
pub const A10_ECC_CNT_RESET_MASK: u32 = A10_ECCCTRL1_CNT_RST | A10_ECCCTRL1_AWB_CNT_RST;
pub const CV_DRAMADDRW: u32 = 0xFFC2502C;
pub const A10_DRAMADDRW: u32 = 0xFFCFA0A8;
pub const S10_DRAMADDRW: u32 = 0xF80110E0;
pub const A10_DRAMADDRW_BANKBIT_MASK: u32 = 0x3C00;
pub const A10_DRAMADDRW_BANKBIT_SHIFT: u32 = 10;
pub const A10_DRAMADDRW_GRPBIT_MASK: u32 = 0xC000;
pub const A10_DRAMADDRW_GRPBIT_SHIFT: u32 = 14;
pub const A10_DRAMADDRW_CSBIT_MASK: u32 = 0x70000;
pub const A10_DRAMADDRW_CSBIT_SHIFT: u32 = 16;
pub const CV_DRAMIFWIDTH: u32 = 0xFFC25030;
pub const A10_DRAMIFWIDTH: u32 = 0xFFCFB008;
pub const S10_DRAMIFWIDTH: u32 = 0xF8011008;
pub const A10_DRAMIFWIDTH_16B: u32 = 0x0;
pub const A10_DRAMIFWIDTH_32B: u32 = 0x1;
pub const A10_DRAMIFWIDTH_64B: u32 = 0x2;
pub const A10_ERRINTEN_OFST: u32 = 0x10;
pub const A10_ERRINTEN_SERRINTEN: u32 = 0x01;
pub const A10_ERRINTEN_DERRINTEN: u32 = 0x02;
pub const A10_ECC_IRQ_EN_MASK: u32 = A10_ERRINTEN_SERRINTEN | A10_ERRINTEN_DERRINTEN;
pub const A10_INTMODE_OFST: u32 = 0x1C;
pub const A10_INTMODE_SB_INT: u32 = 1;
pub const A10_INTSTAT_OFST: u32 = 0x20;
pub const A10_INTSTAT_SBEERR: u32 = 0x01;
pub const A10_INTSTAT_DBEERR: u32 = 0x02;
pub const A10_DERRADDR_OFST: u32 = 0x2C;
pub const A10_SERRADDR_OFST: u32 = 0x30;
pub const A10_DIAGINTTEST_OFST: u32 = 0x24;
pub const A10_DIAGINT_TSERRA_MASK: u32 = 0x0001;
pub const A10_DIAGINT_TDERRA_MASK: u32 = 0x0100;
pub const A10_SBERR_IRQ: i32 = 34;
pub const A10_DBERR_IRQ: i32 = 32;
pub const A10_SERRCNTREG_OFST: u32 = 0x3C;
pub const A10_SYMAN_INTMASK_CLR: u32 = 0xFFD06098;
pub const A10_INTMASK_CLR_OFST: u32 = 0x10;
pub const A10_DDR0_IRQ_MASK: u32 = 1 << 17;

#[repr(C)]
pub struct altr_sdram_prv_data {
    pub ecc_ctrl_offset: i32, pub ecc_ctl_en_mask: i32, pub ecc_cecnt_offset: i32,
    pub ecc_uecnt_offset: i32, pub ecc_stat_offset: i32, pub ecc_stat_ce_mask: i32,
    pub ecc_stat_ue_mask: i32, pub ecc_saddr_offset: i32, pub ecc_daddr_offset: i32,
    pub ecc_irq_en_offset: i32, pub ecc_irq_en_mask: i32, pub ecc_irq_clr_offset: i32,
    pub ecc_irq_clr_mask: i32, pub ecc_cnt_rst_offset: i32, pub ecc_cnt_rst_mask: i32,
    pub ecc_enable_mask: i32, pub ce_set_mask: i32, pub ue_set_mask: i32,
    pub ce_ue_trgr_offset: i32,
}

#[repr(C)]
pub struct altr_sdram_mc_data {
    pub mc_vbase: *mut regmap, pub sb_irq: i32, pub db_irq: i32,
    pub data: *const altr_sdram_prv_data,
}

pub const ALTR_UE_TRIGGER_CHAR: u8 = b'U';
pub const ALTR_TRIGGER_READ_WRD_CNT: usize = 32;
pub const ALTR_TRIG_OCRAM_BYTE_SIZE: usize = 128;
pub const ALTR_TRIG_L2C_BYTE_SIZE: usize = 4096;
pub const ALTR_MAN_GRP_OCRAM_ECC_OFFSET: u32 = 0x04;
pub const ALTR_OCR_ECC_REG_OFFSET: u32 = 0x00;
pub const ALTR_OCR_ECC_EN: u32 = 1 << 0;
pub const ALTR_OCR_ECC_INJS: u32 = 1 << 1;
pub const ALTR_OCR_ECC_INJD: u32 = 1 << 2;
pub const ALTR_OCR_ECC_SERR: u32 = 1 << 3;
pub const ALTR_OCR_ECC_DERR: u32 = 1 << 4;
pub const ALTR_MAN_GRP_L2_ECC_OFFSET: u32 = 0x00;
pub const ALTR_L2_ECC_REG_OFFSET: u32 = 0x00;
pub const ALTR_L2_ECC_EN: u32 = 1 << 0;
pub const ALTR_L2_ECC_INJS: u32 = 1 << 1;
pub const ALTR_L2_ECC_INJD: u32 = 1 << 2;
pub const ALTR_A10_ECC_CTRL_OFST: u32 = 0x08;
pub const ALTR_A10_ECC_EN: u32 = 1 << 0;
pub const ALTR_A10_ECC_INITA: u32 = 1 << 16;
pub const ALTR_A10_ECC_INITB: u32 = 1 << 24;
pub const ALTR_A10_ECC_INITSTAT_OFST: u32 = 0x0C;
pub const ALTR_A10_ECC_INITCOMPLETEA: u32 = 1 << 0;
pub const ALTR_A10_ECC_INITCOMPLETEB: u32 = 1 << 8;
pub const ALTR_A10_ECC_ERRINTEN_OFST: u32 = 0x10;
pub const ALTR_A10_ECC_ERRINTENS_OFST: u32 = 0x14;
pub const ALTR_A10_ECC_ERRINTENR_OFST: u32 = 0x18;
pub const ALTR_A10_ECC_SERRINTEN: u32 = 1 << 0;
pub const ALTR_A10_ECC_INTMODE_OFST: u32 = 0x1C;
pub const ALTR_A10_ECC_INTMODE: u32 = 1 << 0;
pub const ALTR_A10_ECC_INTSTAT_OFST: u32 = 0x20;
pub const ALTR_A10_ECC_SERRPENA: u32 = 1 << 0;
pub const ALTR_A10_ECC_DERRPENA: u32 = 1 << 8;
pub const ALTR_A10_ECC_ERRPENA_MASK: u32 = ALTR_A10_ECC_SERRPENA | ALTR_A10_ECC_DERRPENA;
pub const ALTR_A10_ECC_SERRPENB: u32 = 1 << 16;
pub const ALTR_A10_ECC_DERRPENB: u32 = 1 << 24;
pub const ALTR_A10_ECC_ERRPENB_MASK: u32 = ALTR_A10_ECC_SERRPENB | ALTR_A10_ECC_DERRPENB;
pub const ALTR_A10_ECC_INTTEST_OFST: u32 = 0x24;
pub const ALTR_A10_ECC_TSERRA: u32 = 1 << 0;
pub const ALTR_A10_ECC_TDERRA: u32 = 1 << 8;
pub const ALTR_A10_ECC_TSERRB: u32 = 1 << 16;
pub const ALTR_A10_ECC_TDERRB: u32 = 1 << 24;
pub const A10_SYSMGR_ECC_INTMASK_SET_OFST: u32 = 0x94;
pub const A10_SYSMGR_ECC_INTMASK_CLR_OFST: u32 = 0x98;
pub const A10_SYSMGR_ECC_INTMASK_OCRAM: u32 = 1 << 1;
pub const A10_SYSMGR_ECC_INTMASK_SDMMCB: u32 = 1 << 16;
pub const A10_SYSMGR_ECC_INTMASK_DDR0: u32 = 1 << 17;
pub const A10_SYSMGR_ECC_INTSTAT_SERR_OFST: u32 = 0x9C;
pub const A10_SYSMGR_ECC_INTSTAT_DERR_OFST: u32 = 0xA0;
pub const A10_SYSMGR_ECC_INTSTAT_L2: u32 = 1 << 0;
pub const A10_SYSMGR_ECC_INTSTAT_OCRAM: u32 = 1 << 1;
pub const A10_SYSGMR_MPU_CLEAR_L2_ECC_OFST: u32 = 0xA8;
pub const A10_SYSGMR_MPU_CLEAR_L2_ECC_SB: u32 = 1 << 15;
pub const A10_SYSGMR_MPU_CLEAR_L2_ECC_MB: u32 = 1 << 31;
pub const ALTR_A10_L2_ECC_CTL_OFST: u32 = 0;
pub const ALTR_A10_L2_ECC_EN_CTL: u32 = 1 << 0;
pub const ALTR_A10_L2_ECC_STATUS: u32 = 0xFFD060A4;
pub const ALTR_A10_L2_ECC_STAT_OFST: u32 = 0xA4;
pub const ALTR_A10_L2_ECC_SERR_PEND: u32 = 1 << 0;
pub const ALTR_A10_L2_ECC_MERR_PEND: u32 = 1 << 0;
pub const ALTR_A10_L2_ECC_CLR_OFST: u32 = 0x4;
pub const ALTR_A10_L2_ECC_SERR_CLR: u32 = 1 << 15;
pub const ALTR_A10_L2_ECC_MERR_CLR: u32 = 1 << 31;
pub const ALTR_A10_L2_ECC_INJ_OFST: u32 = ALTR_A10_L2_ECC_CTL_OFST;
pub const ALTR_A10_L2_ECC_CE_INJ_MASK: u32 = 0x00000101;
pub const ALTR_A10_L2_ECC_UE_INJ_MASK: u32 = 0x00010101;
pub const ALTR_A10_OCRAM_ECC_EN_CTL: u32 = (1 << 1) | (1 << 0);
pub const ALTR_A10_COMMON_ECC_EN_CTL: u32 = 1 << 0;
pub const ALTR_A10_SDMMC_IRQ_MASK: u32 = (1 << 16) | (1 << 15);
pub const ALTR_A10_ECC_INIT_WATCHDOG_10US: u32 = 10000;

pub const ALTR_S10_ECC_CTRL_SDRAM_OFST: u32 = 0x00;
pub const ALTR_S10_ECC_EN: u32 = 1 << 0;
pub const ALTR_S10_ECC_ERRINTEN_OFST: u32 = 0x10;
pub const ALTR_S10_ECC_ERRINTENS_OFST: u32 = 0x14;
pub const ALTR_S10_ECC_ERRINTENR_OFST: u32 = 0x18;
pub const ALTR_S10_ECC_SERRINTEN: u32 = 1 << 0;
pub const ALTR_S10_ECC_INTMODE_OFST: u32 = 0x1C;
pub const ALTR_S10_ECC_INTMODE: u32 = 1 << 0;
pub const ALTR_S10_ECC_INTSTAT_OFST: u32 = 0x20;
pub const ALTR_S10_ECC_SERRPENA: u32 = 1 << 0;
pub const ALTR_S10_ECC_DERRPENA: u32 = 1 << 8;
pub const ALTR_S10_ECC_ERRPENA_MASK: u32 = ALTR_S10_ECC_SERRPENA | ALTR_S10_ECC_DERRPENA;
pub const ALTR_S10_ECC_INTTEST_OFST: u32 = 0x24;
pub const ALTR_S10_ECC_TSERRA: u32 = 1 << 0;
pub const ALTR_S10_ECC_TDERRA: u32 = 1 << 8;
pub const ALTR_S10_ECC_TSERRB: u32 = 1 << 16;
pub const ALTR_S10_ECC_TDERRB: u32 = 1 << 24;
pub const ALTR_S10_DERR_ADDRA_OFST: u32 = 0x2C;
pub const S10_SYSMGR_ECC_INTMASK_CLR_OFST: u32 = 0x98;
pub const S10_SYSMGR_ECC_INTSTAT_DERR_OFST: u32 = 0xA0;
pub const S10_SYSMGR_UE_VAL_OFST: u32 = 0x220;
pub const S10_SYSMGR_UE_ADDR_OFST: u32 = 0x224;
pub const S10_DDR0_IRQ_MASK: u32 = 1 << 16;
pub const S10_DBE_IRQ_MASK: u32 = 0x3FFFE;
pub const ECC_BLK_ADDRESS_OFST: u32 = 0x40;
pub const ECC_BLK_RDATA0_OFST: u32 = 0x44;
pub const ECC_BLK_RDATA1_OFST: u32 = 0x48;
pub const ECC_BLK_RDATA2_OFST: u32 = 0x4C;
pub const ECC_BLK_RDATA3_OFST: u32 = 0x50;
pub const ECC_BLK_WDATA0_OFST: u32 = 0x54;
pub const ECC_BLK_WDATA1_OFST: u32 = 0x58;
pub const ECC_BLK_WDATA2_OFST: u32 = 0x5C;
pub const ECC_BLK_WDATA3_OFST: u32 = 0x60;
pub const ECC_BLK_RECC0_OFST: u32 = 0x64;
pub const ECC_BLK_RECC1_OFST: u32 = 0x68;
pub const ECC_BLK_WECC0_OFST: u32 = 0x6C;
pub const ECC_BLK_WECC1_OFST: u32 = 0x70;
pub const ECC_BLK_DBYTECTRL_OFST: u32 = 0x74;
pub const ECC_BLK_ACCCTRL_OFST: u32 = 0x78;
pub const ECC_BLK_STARTACC_OFST: u32 = 0x7C;
pub const ECC_XACT_KICK: u32 = 0x10000;
pub const ECC_WORD_WRITE: u32 = 0xFF;
pub const ECC_WRITE_DOVR: u32 = 0x101;
pub const ECC_WRITE_EDOVR: u32 = 0x103;
pub const ECC_READ_EOVR: u32 = 0x2;
pub const ECC_READ_EDOVR: u32 = 0x3;

pub struct altr_edac_device_dev;

#[repr(C)]
pub struct edac_device_prv_data {
    pub setup: Option<unsafe extern "C" fn(*mut altr_edac_device_dev) -> i32>,
    pub ce_clear_mask: i32, pub ue_clear_mask: i32, pub irq_status_mask: i32,
    pub alloc_mem: Option<unsafe extern "C" fn(usize, *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void>,
    pub free_mem: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize, *mut core::ffi::c_void)>,
    pub ecc_enable_mask: i32, pub ecc_en_ofst: i32, pub ce_set_mask: i32,
    pub ue_set_mask: i32, pub set_err_ofst: i32,
    pub ecc_irq_handler: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t>,
    pub trig_alloc_sz: i32, pub inject_fops: *const file_operations, pub panic: bool,
}

#[repr(C)]
pub struct altr_edac_device_dev {
    pub next: list_head, pub base: *mut core::ffi::c_void, pub sb_irq: i32, pub db_irq: i32,
    pub data: *const edac_device_prv_data, pub debugfs_dir: *mut dentry,
    pub edac_dev_name: *mut i8, pub edac: *mut altr_arria10_edac,
    pub edac_dev: *mut edac_device_ctl_info, pub ddev: device, pub edac_idx: i32,
}

#[repr(C)]
pub struct altr_arria10_edac {
    pub dev: *mut device, pub ecc_mgr_map: *mut regmap, pub sb_irq: i32, pub db_irq: i32,
    pub domain: *mut irq_domain, pub irq_chip: irq_chip, pub a10_ecc_devices: list_head,
    pub panic_notifier: notifier_block,
}

// External kernel types referenced above.
extern "C" {
    type regmap; type file_operations; type dentry; type edac_device_ctl_info;
    type device; type irq_domain; type irq_chip; type list_head; type notifier_block;
    type irqreturn_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
