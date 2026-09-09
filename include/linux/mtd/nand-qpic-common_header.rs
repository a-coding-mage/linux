/* SPDX-License-Identifier: GPL-2.0 */
/* QCOM QPIC common APIs header file; translated from C. */

pub const NAND_FLASH_CMD: u32 = 0x00;
pub const NAND_ADDR0: u32 = 0x04;
pub const NAND_ADDR1: u32 = 0x08;
pub const NAND_FLASH_CHIP_SELECT: u32 = 0x0c;
pub const NAND_EXEC_CMD: u32 = 0x10;
pub const NAND_FLASH_STATUS: u32 = 0x14;
pub const NAND_BUFFER_STATUS: u32 = 0x18;
pub const NAND_DEV0_CFG0: u32 = 0x20;
pub const NAND_DEV0_CFG1: u32 = 0x24;
pub const NAND_DEV0_ECC_CFG: u32 = 0x28;
pub const NAND_AUTO_STATUS_EN: u32 = 0x2c;
pub const NAND_DEV1_CFG0: u32 = 0x30;
pub const NAND_DEV1_CFG1: u32 = 0x34;
pub const NAND_READ_ID: u32 = 0x40;
pub const NAND_READ_STATUS: u32 = 0x44;
pub const NAND_DEV_CMD0: u32 = 0xa0;
pub const NAND_DEV_CMD1: u32 = 0xa4;
pub const NAND_DEV_CMD2: u32 = 0xa8;
pub const NAND_DEV_CMD_VLD: u32 = 0xac;
pub const SFLASHC_BURST_CFG: u32 = 0xe0;
pub const NAND_ERASED_CW_DETECT_CFG: u32 = 0xe8;
pub const NAND_ERASED_CW_DETECT_STATUS: u32 = 0xec;
pub const NAND_EBI2_ECC_BUF_CFG: u32 = 0xf0;
pub const FLASH_BUF_ACC: u32 = 0x100;
pub const NAND_CTRL: u32 = 0xf00;
pub const NAND_VERSION: u32 = 0xf08;
pub const NAND_READ_LOCATION_0: u32 = 0xf20;
pub const NAND_READ_LOCATION_1: u32 = 0xf24;
pub const NAND_READ_LOCATION_2: u32 = 0xf28;
pub const NAND_READ_LOCATION_3: u32 = 0xf2c;
pub const NAND_READ_LOCATION_LAST_CW_0: u32 = 0xf40;
pub const NAND_READ_LOCATION_LAST_CW_1: u32 = 0xf44;
pub const NAND_READ_LOCATION_LAST_CW_2: u32 = 0xf48;
pub const NAND_READ_LOCATION_LAST_CW_3: u32 = 0xf4c;

pub const NAND_DEV_CMD1_RESTORE: u32 = 0xdead;
pub const NAND_DEV_CMD_VLD_RESTORE: u32 = 0xbeef;
pub const PAGE_ACC: u32 = 1 << 4;
pub const LAST_PAGE: u32 = 1 << 5;
pub const NAND_DEV_SEL: u32 = 0;
pub const DM_EN: u32 = 1 << 2;
pub const FS_OP_ERR: u32 = 1 << 4;
pub const FS_READY_BSY_N: u32 = 1 << 5;
pub const FS_MPU_ERR: u32 = 1 << 8;
pub const FS_DEVICE_STS_ERR: u32 = 1 << 16;
pub const FS_DEVICE_WP: u32 = 1 << 23;
pub const BS_UNCORRECTABLE_BIT: u32 = 1 << 8;
pub const BS_CORRECTABLE_ERR_MSK: u32 = 0x1f;
pub const DISABLE_STATUS_AFTER_WRITE: u32 = 1 << 4;
pub const CW_PER_PAGE_MASK: u32 = 0x7 << 6;
pub const UD_SIZE_BYTES_MASK: u32 = 0x3ff << 9;
pub const ECC_PARITY_SIZE_BYTES_RS: u32 = 0xf << 19;
pub const SPARE_SIZE_BYTES_MASK: u32 = 0xf << 23;
pub const NUM_ADDR_CYCLES_MASK: u32 = 0x7 << 27;
pub const STATUS_BFR_READ: u32 = 1 << 30;
pub const SET_RD_MODE_AFTER_STATUS: u32 = 1 << 31;
pub const DEV0_CFG1_ECC_DISABLE: u32 = 1;
pub const WIDE_FLASH: u32 = 1 << 1;
pub const NAND_RECOVERY_CYCLES_MASK: u32 = 0x7 << 2;
pub const CS_ACTIVE_BSY: u32 = 1 << 5;
pub const BAD_BLOCK_BYTE_NUM_MASK: u32 = 0x3ff << 6;
pub const BAD_BLOCK_IN_SPARE_AREA: u32 = 1 << 16;
pub const WR_RD_BSY_GAP_MASK: u32 = 0x3f << 17;
pub const ENABLE_BCH_ECC: u32 = 1 << 27;
pub const ECC_CFG_ECC_DISABLE: u32 = 1;
pub const ECC_SW_RESET: u32 = 1 << 1;
pub const ECC_MODE_MASK: u32 = 0x3 << 4;
pub const ECC_MODE_4BIT: u32 = 0;
pub const ECC_MODE_8BIT: u32 = 1;
pub const ECC_PARITY_SIZE_BYTES_BCH_MASK: u32 = 0x1f << 8;
pub const ECC_NUM_DATA_BYTES_MASK: u32 = 0x3ff << 16;
pub const ECC_FORCE_CLK_OPEN: u32 = 1 << 30;
pub const READ_ADDR_MASK: u32 = 0xff;
pub const READ_START_VLD: u32 = 1;
pub const READ_STOP_VLD: u32 = 1 << 1;
pub const WRITE_START_VLD: u32 = 1 << 2;
pub const ERASE_START_VLD: u32 = 1 << 3;
pub const SEQ_READ_START_VLD: u32 = 1 << 4;
pub const NUM_STEPS_MASK: u32 = 0x3ff;
pub const ERASED_CW_ECC_MASK: u32 = 1;
pub const AUTO_DETECT_RES: u32 = 0;
pub const MASK_ECC: u32 = 1 << ERASED_CW_ECC_MASK;
pub const RESET_ERASED_DET: u32 = 1 << AUTO_DETECT_RES;
pub const ACTIVE_ERASED_DET: u32 = 0;
pub const CLR_ERASED_PAGE_DET: u32 = RESET_ERASED_DET | MASK_ECC;
pub const SET_ERASED_PAGE_DET: u32 = ACTIVE_ERASED_DET | MASK_ECC;
pub const PAGE_ALL_ERASED: u32 = 1 << 7;
pub const CODEWORD_ALL_ERASED: u32 = 1 << 6;
pub const PAGE_ERASED: u32 = 1 << 5;
pub const CODEWORD_ERASED: u32 = 1 << 4;
pub const ERASED_PAGE: u32 = PAGE_ALL_ERASED | PAGE_ERASED;
pub const ERASED_CW: u32 = CODEWORD_ALL_ERASED | CODEWORD_ERASED;
pub const READ_LOCATION_OFFSET_MASK: u32 = 0x3ff;
pub const READ_LOCATION_SIZE_MASK: u32 = 0x3ff << 16;
pub const READ_LOCATION_LAST_MASK: u32 = 1 << 31;
pub const NAND_VERSION_MAJOR_MASK: u32 = 0xf0000000;
pub const NAND_VERSION_MAJOR_SHIFT: u32 = 28;
pub const NAND_VERSION_MINOR_MASK: u32 = 0x0fff0000;
pub const NAND_VERSION_MINOR_SHIFT: u32 = 16;
pub const OP_PAGE_READ: u32 = 2;
pub const OP_PAGE_READ_WITH_ECC: u32 = 3;
pub const OP_PAGE_READ_WITH_ECC_SPARE: u32 = 4;
pub const OP_PAGE_READ_ONFI_READ: u32 = 5;
pub const OP_PROGRAM_PAGE: u32 = 6;
pub const OP_PAGE_PROGRAM_WITH_ECC: u32 = 7;
pub const OP_PROGRAM_PAGE_SPARE: u32 = 9;
pub const OP_BLOCK_ERASE: u32 = 10;
pub const OP_CHECK_STATUS: u32 = 12;
pub const OP_FETCH_ID: u32 = 11;
pub const OP_RESET_DEVICE: u32 = 13;
pub const NAND_DEV_CMD_VLD_VAL: u32 = READ_START_VLD | WRITE_START_VLD | ERASE_START_VLD | SEQ_READ_START_VLD;
pub const BAM_MODE_EN: u32 = 1;
pub const NANDC_STEP_SIZE: u32 = 512;
pub const MAX_NUM_STEPS: u32 = 8192 / NANDC_STEP_SIZE;
pub const MAX_REG_RD: u32 = 3 * MAX_NUM_STEPS;
pub const ECC_NONE: u32 = 1;
pub const ECC_RS_4BIT: u32 = 1 << 1;
pub const ECC_BCH_4BIT: u32 = 1 << 2;
pub const ECC_BCH_8BIT: u32 = 1 << 3;
pub const QPIC_PER_CW_CMD_ELEMENTS: usize = 32;
pub const QPIC_PER_CW_CMD_SGL: usize = 32;
pub const QPIC_PER_CW_DATA_SGL: usize = 8;
pub const MAX_ADDRESS_CYCLE: usize = 5;
pub const NAND_BAM_NO_EOT: u32 = 1;
pub const NAND_BAM_NWD: u32 = 1 << 1;
pub const NAND_BAM_NEXT_SGL: u32 = 1 << 2;
pub const NAND_ERASED_CW_SET: u32 = 1 << 4;

/* External kernel types and macros are supplied by dependent translation units. */
#[repr(C)]
pub struct bam_transaction {
    pub bam_ce: *mut bam_cmd_element, pub cmd_sgl: *mut scatterlist, pub data_sgl: *mut scatterlist,
    pub last_data_desc: *mut dma_async_tx_descriptor, pub last_cmd_desc: *mut dma_async_tx_descriptor,
    pub txn_done: completion, pub bam_ce_nitems: u32, pub cmd_sgl_nitems: u32, pub data_sgl_nitems: u32,
    pub bam_ce_pos: u32, pub bam_ce_start: u32, pub cmd_sgl_pos: u32, pub cmd_sgl_start: u32,
    pub tx_sgl_pos: u32, pub tx_sgl_start: u32, pub rx_sgl_pos: u32, pub rx_sgl_start: u32,
}
#[repr(C)] pub union desc_info_union { pub adm_sgl: scatterlist, pub bam: desc_info_bam }
#[repr(C)] pub struct desc_info_bam { pub bam_sgl: *mut scatterlist, pub sgl_cnt: i32 }
#[repr(C)] pub struct desc_info { pub dma_desc: *mut dma_async_tx_descriptor, pub node: list_head, pub data: desc_info_union, pub dir: dma_data_direction }
#[repr(C)] pub struct nandc_regs {
    pub cmd: __le32, pub addr0: __le32, pub addr1: __le32, pub chip_sel: __le32, pub exec: __le32,
    pub cfg0: __le32, pub cfg1: __le32, pub ecc_bch_cfg: __le32, pub clrflashstatus: __le32, pub clrreadstatus: __le32,
    pub cmd1: __le32, pub vld: __le32, pub orig_cmd1: __le32, pub orig_vld: __le32, pub ecc_buf_cfg: __le32,
    pub read_location0: __le32, pub read_location1: __le32, pub read_location2: __le32, pub read_location3: __le32,
    pub read_location_last0: __le32, pub read_location_last1: __le32, pub read_location_last2: __le32, pub read_location_last3: __le32,
    pub spi_cfg: __le32, pub num_addr_cycle: __le32, pub busy_wait_cnt: __le32, pub flash_feature: __le32,
    pub erased_cw_detect_cfg_clr: __le32, pub erased_cw_detect_cfg_set: __le32,
}
#[repr(C)] pub struct qcom_nand_controller {
    pub dev: *mut device, pub base: *mut core::ffi::c_void, pub core_clk: *mut clk, pub aon_clk: *mut clk,
    pub regs: *mut nandc_regs, pub bam_txn: *mut bam_transaction, pub props: *const qcom_nandc_props,
    pub controller: nand_controller, pub qspi: *mut qpic_spi_nand, pub host_list: list_head,
    pub tx_chan: *mut dma_chan, pub rx_chan: *mut dma_chan, pub cmd_chan: *mut dma_chan,
    pub desc_list: list_head, pub data_buffer: *mut u8, pub reg_read_buf: *mut __le32,
    pub base_phys: phys_addr_t, pub base_dma: dma_addr_t, pub reg_read_dma: dma_addr_t,
    pub buf_size: i32, pub buf_count: i32, pub buf_start: i32, pub max_cwperpage: u32,
    pub reg_read_pos: i32, pub cmd1: u32, pub vld: u32, pub exec_opwrite: bool,
}
#[repr(C)] pub struct qcom_nandc_props { pub ecc_modes: u32, pub dev_cmd_reg_start: u32, pub bam_offset: u32, pub supports_bam: bool, pub nandc_part_of_qpic: bool, pub has_onfi_read_op: bool, pub qpic_version2: bool, pub use_codeword_fixup: bool }

pub const NAND_DEV_CMD1_RESTORE_ADDR: u32 = NAND_DEV_CMD1_RESTORE;

extern "C" {
    pub fn qcom_free_bam_transaction(nandc: *mut qcom_nand_controller);
    pub fn qcom_alloc_bam_transaction(nandc: *mut qcom_nand_controller) -> *mut bam_transaction;
    pub fn qcom_clear_bam_transaction(nandc: *mut qcom_nand_controller);
    pub fn qcom_qpic_bam_dma_done(data: *mut core::ffi::c_void);
    pub fn qcom_nandc_dev_to_mem(nandc: *mut qcom_nand_controller, is_cpu: bool);
    pub fn qcom_prepare_bam_async_desc(nandc: *mut qcom_nand_controller, chan: *mut dma_chan, flags: usize) -> i32;
    pub fn qcom_prep_bam_dma_desc_cmd(nandc: *mut qcom_nand_controller, read: bool, reg_off: i32, vaddr: *const core::ffi::c_void, size: i32, flags: u32) -> i32;
    pub fn qcom_prep_bam_dma_desc_data(nandc: *mut qcom_nand_controller, read: bool, vaddr: *const core::ffi::c_void, size: i32, flags: u32) -> i32;
    pub fn qcom_prep_adm_dma_desc(nandc: *mut qcom_nand_controller, read: bool, reg_off: i32, vaddr: *const core::ffi::c_void, size: i32, flow_control: bool) -> i32;
    pub fn qcom_read_reg_dma(nandc: *mut qcom_nand_controller, first: i32, num_regs: i32, flags: u32) -> i32;
    pub fn qcom_write_reg_dma(nandc: *mut qcom_nand_controller, vaddr: *mut __le32, first: i32, num_regs: i32, flags: u32) -> i32;
    pub fn qcom_read_data_dma(nandc: *mut qcom_nand_controller, reg_off: i32, vaddr: *const u8, size: i32, flags: u32) -> i32;
    pub fn qcom_write_data_dma(nandc: *mut qcom_nand_controller, reg_off: i32, vaddr: *const u8, size: i32, flags: u32) -> i32;
    pub fn qcom_submit_descs(nandc: *mut qcom_nand_controller) -> i32;
    pub fn qcom_clear_read_regs(nandc: *mut qcom_nand_controller);
    pub fn qcom_nandc_unalloc(nandc: *mut qcom_nand_controller);
    pub fn qcom_nandc_alloc(nandc: *mut qcom_nand_controller) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
