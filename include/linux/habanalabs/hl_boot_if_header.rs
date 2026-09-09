/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2018-2023 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

pub const LKD_HARD_RESET_MAGIC: u32 = 0xED7BD694; /* deprecated - do not use */
pub const HL_POWER9_HOST_MAGIC: u32 = 0x1DA30009;
pub const BOOT_FIT_SRAM_OFFSET: u32 = 0x200000;
pub const VERSION_MAX_LEN: usize = 128;

#[repr(i32)]
pub enum cpu_boot_err {
    CPU_BOOT_ERR_DRAM_INIT_FAIL = 0,
    CPU_BOOT_ERR_FIT_CORRUPTED = 1,
    CPU_BOOT_ERR_TS_INIT_FAIL = 2,
    CPU_BOOT_ERR_DRAM_SKIPPED = 3,
    CPU_BOOT_ERR_BMC_WAIT_SKIPPED = 4,
    CPU_BOOT_ERR_NIC_DATA_NOT_RDY = 5,
    CPU_BOOT_ERR_NIC_FW_FAIL = 6,
    CPU_BOOT_ERR_SECURITY_NOT_RDY = 7,
    CPU_BOOT_ERR_SECURITY_FAIL = 8,
    CPU_BOOT_ERR_EFUSE_FAIL = 9,
    CPU_BOOT_ERR_PRI_IMG_VER_FAIL = 10,
    CPU_BOOT_ERR_SEC_IMG_VER_FAIL = 11,
    CPU_BOOT_ERR_PLL_FAIL = 12,
    CPU_BOOT_ERR_DEVICE_UNUSABLE_FAIL = 13,
    CPU_BOOT_ERR_BOOT_FW_CRIT_ERR = 18,
    CPU_BOOT_ERR_BINNING_FAIL = 19,
    CPU_BOOT_ERR_TPM_FAIL = 20,
    CPU_BOOT_ERR_TMP_THRESH_INIT_FAIL = 21,
    CPU_BOOT_ERR_EEPROM_FAIL = 22,
    CPU_BOOT_ERR_ENG_ARC_MEM_SCRUB_FAIL = 23,
    CPU_BOOT_ERR_ENABLED = 31,
    CPU_BOOT_ERR_SCND_EN = 63,
    CPU_BOOT_ERR_LAST = 64,
}

pub const CPU_BOOT_ERR_FATAL_MASK: u32 = (1 << 0) | (1 << 12) | (1 << 19) | (1 << 3) | (1 << 23) | (1 << 22);
pub const CPU_BOOT_ERR0_DRAM_INIT_FAIL: u32 = 1 << 0;
pub const CPU_BOOT_ERR0_FIT_CORRUPTED: u32 = 1 << 1;
pub const CPU_BOOT_ERR0_TS_INIT_FAIL: u32 = 1 << 2;
pub const CPU_BOOT_ERR0_DRAM_SKIPPED: u32 = 1 << 3;
pub const CPU_BOOT_ERR0_BMC_WAIT_SKIPPED: u32 = 1 << 4;
pub const CPU_BOOT_ERR0_NIC_DATA_NOT_RDY: u32 = 1 << 5;
pub const CPU_BOOT_ERR0_NIC_FW_FAIL: u32 = 1 << 6;
pub const CPU_BOOT_ERR0_SECURITY_NOT_RDY: u32 = 1 << 7;
pub const CPU_BOOT_ERR0_SECURITY_FAIL: u32 = 1 << 8;
pub const CPU_BOOT_ERR0_EFUSE_FAIL: u32 = 1 << 9;
pub const CPU_BOOT_ERR0_PRI_IMG_VER_FAIL: u32 = 1 << 10;
pub const CPU_BOOT_ERR0_SEC_IMG_VER_FAIL: u32 = 1 << 11;
pub const CPU_BOOT_ERR0_PLL_FAIL: u32 = 1 << 12;
pub const CPU_BOOT_ERR0_DEVICE_UNUSABLE_FAIL: u32 = 1 << 13;
pub const CPU_BOOT_ERR0_BOOT_FW_CRIT_ERR: u32 = 1 << 18;
pub const CPU_BOOT_ERR0_BINNING_FAIL: u32 = 1 << 19;
pub const CPU_BOOT_ERR0_TPM_FAIL: u32 = 1 << 20;
pub const CPU_BOOT_ERR0_TMP_THRESH_INIT_FAIL: u32 = 1 << 21;
pub const CPU_BOOT_ERR0_EEPROM_FAIL: u32 = 1 << 22;
pub const CPU_BOOT_ERR0_ENG_ARC_MEM_SCRUB_FAIL: u32 = 1 << 23;
pub const CPU_BOOT_ERR0_ENABLED: u32 = 1 << 31;
pub const CPU_BOOT_ERR1_ENABLED: u32 = 1 << 31;

#[repr(i32)]
pub enum cpu_boot_dev_sts {
    CPU_BOOT_DEV_STS_SECURITY_EN = 0, CPU_BOOT_DEV_STS_DEBUG_EN, CPU_BOOT_DEV_STS_WATCHDOG_EN,
    CPU_BOOT_DEV_STS_DRAM_INIT_EN, CPU_BOOT_DEV_STS_BMC_WAIT_EN, CPU_BOOT_DEV_STS_E2E_CRED_EN,
    CPU_BOOT_DEV_STS_HBM_CRED_EN, CPU_BOOT_DEV_STS_RL_EN, CPU_BOOT_DEV_STS_SRAM_SCR_EN,
    CPU_BOOT_DEV_STS_DRAM_SCR_EN, CPU_BOOT_DEV_STS_FW_HARD_RST_EN, CPU_BOOT_DEV_STS_PLL_INFO_EN,
    CPU_BOOT_DEV_STS_SP_SRAM_EN, CPU_BOOT_DEV_STS_CLK_GATE_EN, CPU_BOOT_DEV_STS_HBM_ECC_EN,
    CPU_BOOT_DEV_STS_PKT_PI_ACK_EN, CPU_BOOT_DEV_STS_FW_LD_COM_EN, CPU_BOOT_DEV_STS_FW_IATU_CONF_EN,
    CPU_BOOT_DEV_STS_FW_NIC_MAC_EN, CPU_BOOT_DEV_STS_DYN_PLL_EN, CPU_BOOT_DEV_STS_GIC_PRIVILEGED_EN,
    CPU_BOOT_DEV_STS_EQ_INDEX_EN, CPU_BOOT_DEV_STS_MULTI_IRQ_POLL_EN,
    CPU_BOOT_DEV_STS_FW_NIC_STAT_XPCS91_EN, CPU_BOOT_DEV_STS_FW_NIC_STAT_EXT_EN,
    CPU_BOOT_DEV_STS_IS_IDLE_CHECK_EN, CPU_BOOT_DEV_STS_MAP_HWMON_EN, CPU_BOOT_DEV_STS_NIC_MEM_CLEAR_EN,
    CPU_BOOT_DEV_STS_MMU_PGTBL_DRAM_EN, CPU_BOOT_DEV_STS_ENABLED = 31,
    CPU_BOOT_DEV_STS_SCND_EN = 63, CPU_BOOT_DEV_STS_LAST = 64,
}

pub const CPU_BOOT_DEV_STS0_SECURITY_EN: u32 = 1 << 0;
pub const CPU_BOOT_DEV_STS0_DEBUG_EN: u32 = 1 << 1;
pub const CPU_BOOT_DEV_STS0_WATCHDOG_EN: u32 = 1 << 2;
pub const CPU_BOOT_DEV_STS0_DRAM_INIT_EN: u32 = 1 << 3;
pub const CPU_BOOT_DEV_STS0_BMC_WAIT_EN: u32 = 1 << 4;
pub const CPU_BOOT_DEV_STS0_E2E_CRED_EN: u32 = 1 << 5;
pub const CPU_BOOT_DEV_STS0_HBM_CRED_EN: u32 = 1 << 6;
pub const CPU_BOOT_DEV_STS0_RL_EN: u32 = 1 << 7;
pub const CPU_BOOT_DEV_STS0_SRAM_SCR_EN: u32 = 1 << 8;
pub const CPU_BOOT_DEV_STS0_DRAM_SCR_EN: u32 = 1 << 9;
pub const CPU_BOOT_DEV_STS0_FW_HARD_RST_EN: u32 = 1 << 10;
pub const CPU_BOOT_DEV_STS0_PLL_INFO_EN: u32 = 1 << 11;
pub const CPU_BOOT_DEV_STS0_SP_SRAM_EN: u32 = 1 << 12;
pub const CPU_BOOT_DEV_STS0_CLK_GATE_EN: u32 = 1 << 13;
pub const CPU_BOOT_DEV_STS0_HBM_ECC_EN: u32 = 1 << 14;
pub const CPU_BOOT_DEV_STS0_PKT_PI_ACK_EN: u32 = 1 << 15;
pub const CPU_BOOT_DEV_STS0_FW_LD_COM_EN: u32 = 1 << 16;
pub const CPU_BOOT_DEV_STS0_FW_IATU_CONF_EN: u32 = 1 << 17;
pub const CPU_BOOT_DEV_STS0_FW_NIC_MAC_EN: u32 = 1 << 18;
pub const CPU_BOOT_DEV_STS0_DYN_PLL_EN: u32 = 1 << 19;
pub const CPU_BOOT_DEV_STS0_GIC_PRIVILEGED_EN: u32 = 1 << 20;
pub const CPU_BOOT_DEV_STS0_EQ_INDEX_EN: u32 = 1 << 21;
pub const CPU_BOOT_DEV_STS0_MULTI_IRQ_POLL_EN: u32 = 1 << 22;
pub const CPU_BOOT_DEV_STS0_FW_NIC_STAT_XPCS91_EN: u32 = 1 << 23;
pub const CPU_BOOT_DEV_STS0_FW_NIC_STAT_EXT_EN: u32 = 1 << 24;
pub const CPU_BOOT_DEV_STS0_IS_IDLE_CHECK_EN: u32 = 1 << 25;
pub const CPU_BOOT_DEV_STS0_MAP_HWMON_EN: u32 = 1 << 26;
pub const CPU_BOOT_DEV_STS0_NIC_MEM_CLEAR_EN: u32 = 1 << 27;
pub const CPU_BOOT_DEV_STS0_MMU_PGTBL_DRAM_EN: u32 = 1 << 28;
pub const CPU_BOOT_DEV_STS0_ENABLED: u32 = 1 << 31;
pub const CPU_BOOT_DEV_STS1_ENABLED: u32 = 1 << 31;

#[repr(i32)]
pub enum cpu_boot_status {
    CPU_BOOT_STATUS_NA = 0, CPU_BOOT_STATUS_IN_WFE = 1, CPU_BOOT_STATUS_DRAM_RDY = 2,
    CPU_BOOT_STATUS_SRAM_AVAIL = 3, CPU_BOOT_STATUS_IN_BTL = 4, CPU_BOOT_STATUS_IN_PREBOOT = 5,
    CPU_BOOT_STATUS_IN_SPL = 6, CPU_BOOT_STATUS_IN_UBOOT = 7, CPU_BOOT_STATUS_DRAM_INIT_FAIL = 8,
    CPU_BOOT_STATUS_FIT_CORRUPTED = 9, CPU_BOOT_STATUS_UBOOT_NOT_READY = 10,
    CPU_BOOT_STATUS_NIC_FW_RDY = 11, CPU_BOOT_STATUS_TS_INIT_FAIL = 12,
    CPU_BOOT_STATUS_DRAM_SKIPPED = 13, CPU_BOOT_STATUS_BMC_WAITING_SKIPPED = 14,
    CPU_BOOT_STATUS_READY_TO_BOOT = 15, CPU_BOOT_STATUS_WAITING_FOR_BOOT_FIT = 16,
    CPU_BOOT_STATUS_SECURITY_READY = 17, CPU_BOOT_STATUS_FW_SHUTDOWN_PREP = 18,
}

#[repr(i32)]
pub enum kmd_msg { KMD_MSG_NA = 0, KMD_MSG_GOTO_WFE, KMD_MSG_FIT_RDY, KMD_MSG_SKIP_BMC, RESERVED, KMD_MSG_RST_DEV, KMD_MSG_LAST }
#[repr(i32)]
pub enum cpu_msg_status { CPU_MSG_CLR = 0, CPU_MSG_OK, CPU_MSG_ERR }

#[repr(C)]
pub struct cpu_dyn_regs {
    pub cpu_pq_base_addr_low: u32, pub cpu_pq_base_addr_high: u32, pub cpu_pq_length: u32, pub cpu_pq_init_status: u32,
    pub cpu_eq_base_addr_low: u32, pub cpu_eq_base_addr_high: u32, pub cpu_eq_length: u32, pub cpu_eq_ci: u32,
    pub cpu_cq_base_addr_low: u32, pub cpu_cq_base_addr_high: u32, pub cpu_cq_length: u32, pub cpu_pf_pq_pi: u32,
    pub cpu_boot_dev_sts0: u32, pub cpu_boot_dev_sts1: u32, pub cpu_boot_err0: u32, pub cpu_boot_err1: u32,
    pub cpu_boot_status: u32, pub fw_upd_sts: u32, pub fw_upd_cmd: u32, pub fw_upd_pending_sts: u32,
    pub fuse_ver_offset: u32, pub preboot_ver_offset: u32, pub uboot_ver_offset: u32, pub hw_state: u32,
    pub kmd_msg_to_cpu: u32, pub cpu_cmd_status_to_host: u32, pub gic_host_pi_upd_irq: u32,
    pub gic_tpc_qm_irq_ctrl: u32, pub gic_mme_qm_irq_ctrl: u32, pub gic_dma_qm_irq_ctrl: u32,
    pub gic_nic_qm_irq_ctrl: u32, pub gic_dma_core_irq_ctrl: u32, pub gic_host_halt_irq: u32,
    pub gic_host_ints_irq: u32, pub reserved0: u32, pub gic_rot_qm_irq_ctrl: u32, pub reserved1: u32,
    pub eng_arc_irq_ctrl: u32, pub reserved2: [u32; 20],
}

pub const HL_COMMS_DESC_MAGIC: u32 = 0x4843444D;
pub const HL_COMMS_DESC_VER: u32 = 3;
pub const HL_COMMS_MSG_MAGIC_VALUE: u32 = 0x48434D00;
pub const HL_COMMS_MSG_MAGIC_MASK: u32 = 0xFFFFFF00;
pub const HL_COMMS_MSG_MAGIC_VER_MASK: u32 = 0xFF;
#[inline] pub const fn HL_COMMS_MSG_MAGIC_VER(ver: u32) -> u32 { HL_COMMS_MSG_MAGIC_VALUE | (ver & HL_COMMS_MSG_MAGIC_VER_MASK) }
pub const HL_COMMS_MSG_MAGIC_V0: u32 = HL_COMMS_DESC_MAGIC;
pub const HL_COMMS_MSG_MAGIC_V1: u32 = HL_COMMS_MSG_MAGIC_VER(1);
pub const HL_COMMS_MSG_MAGIC_V2: u32 = HL_COMMS_MSG_MAGIC_VER(2);
pub const HL_COMMS_MSG_MAGIC_V3: u32 = HL_COMMS_MSG_MAGIC_VER(3);
pub const HL_COMMS_MSG_MAGIC: u32 = HL_COMMS_MSG_MAGIC_V3;
#[inline] pub const fn HL_COMMS_MSG_MAGIC_VALIDATE_MAGIC(magic: u32) -> bool { (magic & HL_COMMS_MSG_MAGIC_MASK) == HL_COMMS_MSG_MAGIC_VALUE }
#[inline] pub const fn HL_COMMS_MSG_MAGIC_VALIDATE_VERSION(magic: u32, ver: u32) -> bool { (magic & HL_COMMS_MSG_MAGIC_VER_MASK) >= (ver & HL_COMMS_MSG_MAGIC_VER_MASK) }
#[inline] pub const fn HL_COMMS_MSG_MAGIC_VALIDATE(magic: u32, ver: u32) -> bool { HL_COMMS_MSG_MAGIC_VALIDATE_MAGIC(magic) && HL_COMMS_MSG_MAGIC_VALIDATE_VERSION(magic, ver) }

#[repr(i32)]
pub enum comms_msg_type { HL_COMMS_DESC_TYPE = 0, HL_COMMS_RESET_CAUSE_TYPE = 1, HL_COMMS_FW_CFG_SKIP_TYPE = 2, HL_COMMS_BINNING_CONF_TYPE = 3 }

#[repr(C)]
pub struct lkd_fw_binning_info {
    pub tpc_mask_l: u64, pub dec_mask: u32, pub dram_mask: u32, pub edma_mask: u32, pub mme_mask_l: u32,
    pub mme_mask_h: u32, pub rot_mask: u32, pub xbar_mask: u32, pub reserved0: u32, pub tpc_mask_h: u64,
    pub nic_mask: u64, pub reserved1: [u32; 8],
}
#[repr(C)] pub struct comms_desc_header { pub magic: u32, pub crc32: u32, pub size: u16, pub version: u8, pub reserved: [u8; 5] }
#[repr(C)] pub struct comms_msg_header { pub magic: u32, pub crc32: u32, pub size: u16, pub version: u8, pub msg_type: u8, pub reserved: [u8; 4] }
#[repr(i32)] pub enum lkd_fw_ascii_msg_lvls { LKD_FW_ASCII_MSG_ERR = 0, LKD_FW_ASCII_MSG_WRN = 1, LKD_FW_ASCII_MSG_INF = 2, LKD_FW_ASCII_MSG_DBG = 3 }
pub const LKD_FW_ASCII_MSG_MAX_LEN: usize = 128;
pub const LKD_FW_ASCII_MSG_MAX: usize = 4;
pub const LKD_FW_ASCII_MSG_MIN_DESC_VERSION: u32 = 3;
#[repr(C)] pub struct lkd_fw_ascii_msg { pub valid: u8, pub msg_lvl: u8, pub reserved: [u8; 6], pub msg: [u8; LKD_FW_ASCII_MSG_MAX_LEN] }
#[repr(C)] pub struct lkd_fw_comms_desc { pub header: comms_desc_header, pub cpu_dyn_regs: cpu_dyn_regs, pub fuse_ver: [u8; VERSION_MAX_LEN], pub cur_fw_ver: [u8; VERSION_MAX_LEN], pub reserved0: [u8; VERSION_MAX_LEN], pub img_addr: u64, pub binning_info: lkd_fw_binning_info, pub ascii_msg: [lkd_fw_ascii_msg; LKD_FW_ASCII_MSG_MAX], pub rsvd_mem_size_mb: u32, pub reserved1: [u8; 4] }

#[repr(i32)] pub enum comms_reset_cause { HL_RESET_CAUSE_UNKNOWN = 0, HL_RESET_CAUSE_HEARTBEAT = 1, HL_RESET_CAUSE_TDR = 2 }
pub type lkd_msg_comms = lkd_fw_comms_msg;
#[repr(C)] pub struct lkd_fw_comms_msg_payload { pub cpu_dyn_regs: cpu_dyn_regs, pub fuse_ver: [u8; VERSION_MAX_LEN], pub cur_fw_ver: [u8; VERSION_MAX_LEN], pub reserved0: [u8; VERSION_MAX_LEN], pub img_addr: u64, pub binning_info: lkd_fw_binning_info, pub ascii_msg: [lkd_fw_ascii_msg; LKD_FW_ASCII_MSG_MAX], pub rsvd_mem_size_mb: u32, pub reserved1: [u8; 4] }
#[repr(C)] pub union lkd_fw_comms_msg_union { pub fw_comms: lkd_fw_comms_msg_payload, pub reset_cause: u8, pub fw_cfg_skip: u8, pub binning_conf: lkd_fw_binning_info }
#[repr(C)] pub struct lkd_fw_comms_msg { pub header: comms_msg_header, pub payload: lkd_fw_comms_msg_union }

#[repr(i32)] pub enum comms_cmd { COMMS_NOOP = 0, COMMS_CLR_STS = 1, COMMS_RST_STATE = 2, COMMS_PREP_DESC = 3, COMMS_DATA_RDY = 4, COMMS_EXEC = 5, COMMS_RST_DEV = 6, COMMS_GOTO_WFE = 7, COMMS_SKIP_BMC = 8, COMMS_PREP_DESC_ELBI = 10, COMMS_INVLD_LAST }
pub const COMMS_COMMAND_SIZE_SHIFT: u32 = 0; pub const COMMS_COMMAND_SIZE_MASK: u32 = 0x1FFFFFF; pub const COMMS_COMMAND_CMD_SHIFT: u32 = 27; pub const COMMS_COMMAND_CMD_MASK: u32 = 0xF8000000;
#[repr(C)] pub union comms_command { pub val: u32 }

#[repr(i32)] pub enum comms_sts { COMMS_STS_NOOP = 0, COMMS_STS_ACK = 1, COMMS_STS_OK = 2, COMMS_STS_ERR = 3, COMMS_STS_VALID_ERR = 4, COMMS_STS_TIMEOUT_ERR = 5, COMMS_STS_INVLD_LAST }
#[repr(i32)] pub enum comms_ram_types { COMMS_SRAM = 0, COMMS_DRAM = 1 }
pub const COMMS_STATUS_OFFSET_SHIFT: u32 = 0; pub const COMMS_STATUS_OFFSET_MASK: u32 = 0x03FFFFFF; pub const COMMS_STATUS_OFFSET_ALIGN_SHIFT: u32 = 2; pub const COMMS_STATUS_RAM_TYPE_SHIFT: u32 = 26; pub const COMMS_STATUS_RAM_TYPE_MASK: u32 = 0x0C000000; pub const COMMS_STATUS_STATUS_SHIFT: u32 = 28; pub const COMMS_STATUS_STATUS_MASK: u32 = 0xF0000000;
#[repr(C)] pub union comms_status { pub val: u32 }

pub const NAME_MAX_LEN: usize = 32;
#[repr(C)] pub struct hl_module_data { pub name: [u8; NAME_MAX_LEN], pub version: [u8; VERSION_MAX_LEN] }
#[repr(C)] pub struct hl_component_versions { pub struct_size: u16, pub modules_offset: u16, pub component: [u8; VERSION_MAX_LEN], pub fw_os: [u8; VERSION_MAX_LEN], pub comp_name: [u8; NAME_MAX_LEN], pub modules_counter: u8, pub reserved: [u8; 3], pub modules: [hl_module_data; 0] }
pub const HL_FW_VERSIONS_FIT_SIZE: usize = 4096;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
