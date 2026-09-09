/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2017-2018, The Linux Foundation. All rights reserved.
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// Dependency supplied by the surrounding kernel translation.

#[repr(C)]
pub enum geni_se_xfer_mode { GENI_SE_INVALID, GENI_SE_FIFO, GENI_SE_DMA, GENI_GPI_DMA }

#[repr(C)]
pub enum geni_se_protocol_type { GENI_SE_NONE, GENI_SE_SPI, GENI_SE_UART, GENI_SE_I2C, GENI_SE_I3C, GENI_SE_SPI_SLAVE, GENI_SE_INVALID_PROTO = 255 }

pub enum geni_wrapper {}
pub enum clk {}

#[repr(C)]
pub enum geni_icc_path_index { GENI_TO_CORE, CPU_TO_GENI, GENI_TO_DDR }

#[repr(C)]
pub struct geni_icc_path { pub path: *mut icc_path, pub avg_bw: u32 }

#[repr(C)]
pub struct geni_se {
    pub base: *mut core::ffi::c_void,
    pub dev: *mut device,
    pub wrapper: *mut geni_wrapper,
    pub clk: *mut clk,
    pub core_clk: *mut clk,
    pub num_clk_levels: u32,
    pub clk_perf_tbl: *mut u64,
    pub icc_paths: [geni_icc_path; 3],
    pub pd_list: *mut dev_pm_domain_list,
    pub has_opp: bool,
}

macro_rules! bit { ($n:expr) => { 1u32 << $n }; }
macro_rules! genmask { ($hi:expr, $lo:expr) => { (((1u32 << ($hi - $lo + 1)) - 1) << $lo) }; }

pub const GENI_GENERAL_CFG: u32 = 0x10; pub const GENI_FORCE_DEFAULT_REG: u32 = 0x20; pub const GENI_OUTPUT_CTRL: u32 = 0x24;
pub const SE_GENI_STATUS: u32 = 0x40; pub const GENI_SER_M_CLK_CFG: u32 = 0x48; pub const GENI_SER_S_CLK_CFG: u32 = 0x4c;
pub const GENI_CLK_CTRL_RO: u32 = 0x60; pub const GENI_IF_DISABLE_RO: u32 = 0x64; pub const GENI_FW_REVISION_RO: u32 = 0x68;
pub const GENI_FW_MULTILOCK_MSA_RO: u32 = 0x74; pub const SE_GENI_CLK_SEL: u32 = 0x7c; pub const SE_GENI_CFG_SEQ_START: u32 = 0x84;
pub const SE_GENI_DMA_MODE_EN: u32 = 0x258; pub const SE_GENI_M_CMD0: u32 = 0x600; pub const SE_GENI_M_CMD_CTRL_REG: u32 = 0x604;
pub const SE_GENI_M_IRQ_STATUS: u32 = 0x610; pub const SE_GENI_M_IRQ_EN: u32 = 0x614; pub const SE_GENI_M_IRQ_CLEAR: u32 = 0x618;
pub const SE_GENI_M_IRQ_EN_SET: u32 = 0x61c; pub const SE_GENI_M_IRQ_EN_CLEAR: u32 = 0x620; pub const M_CMD_ERR_STATUS: u32 = 0x624; pub const M_FW_ERR_STATUS: u32 = 0x628;
pub const SE_GENI_S_CMD0: u32 = 0x630; pub const SE_GENI_S_CMD_CTRL_REG: u32 = 0x634; pub const SE_GENI_S_IRQ_STATUS: u32 = 0x640; pub const SE_GENI_S_IRQ_EN: u32 = 0x644; pub const SE_GENI_S_IRQ_CLEAR: u32 = 0x648; pub const SE_GENI_S_IRQ_EN_SET: u32 = 0x64c; pub const SE_GENI_S_IRQ_EN_CLEAR: u32 = 0x650;
pub const SE_GENI_TX_FIFOn: u32 = 0x700; pub const SE_GENI_RX_FIFOn: u32 = 0x780; pub const SE_GENI_TX_FIFO_STATUS: u32 = 0x800; pub const SE_GENI_RX_FIFO_STATUS: u32 = 0x804; pub const SE_GENI_TX_WATERMARK_REG: u32 = 0x80c; pub const SE_GENI_RX_WATERMARK_REG: u32 = 0x810; pub const SE_GENI_RX_RFR_WATERMARK_REG: u32 = 0x814; pub const SE_GENI_IOS: u32 = 0x908; pub const SE_GENI_M_GP_LENGTH: u32 = 0x910; pub const SE_GENI_S_GP_LENGTH: u32 = 0x914;

pub const SE_DMA_TX_PTR_L:u32=0xc30; pub const SE_DMA_TX_PTR_H:u32=0xc34; pub const SE_DMA_TX_ATTR:u32=0xc38; pub const SE_DMA_TX_LEN:u32=0xc3c; pub const SE_DMA_TX_IRQ_STAT:u32=0xc40; pub const SE_DMA_TX_IRQ_CLR:u32=0xc44; pub const SE_DMA_TX_IRQ_EN:u32=0xc48; pub const SE_DMA_TX_IRQ_EN_SET:u32=0xc4c; pub const SE_DMA_TX_IRQ_EN_CLR:u32=0xc50; pub const SE_DMA_TX_LEN_IN:u32=0xc54; pub const SE_DMA_TX_FSM_RST:u32=0xc58; pub const SE_DMA_TX_MAX_BURST:u32=0xc5c;
pub const SE_DMA_RX_PTR_L:u32=0xd30; pub const SE_DMA_RX_PTR_H:u32=0xd34; pub const SE_DMA_RX_ATTR:u32=0xd38; pub const SE_DMA_RX_LEN:u32=0xd3c; pub const SE_DMA_RX_IRQ_STAT:u32=0xd40; pub const SE_DMA_RX_IRQ_CLR:u32=0xd44; pub const SE_DMA_RX_IRQ_EN:u32=0xd48; pub const SE_DMA_RX_IRQ_EN_SET:u32=0xd4c; pub const SE_DMA_RX_IRQ_EN_CLR:u32=0xd50; pub const SE_DMA_RX_LEN_IN:u32=0xd54; pub const SE_DMA_RX_FSM_RST:u32=0xd58; pub const SE_DMA_RX_MAX_BURST:u32=0xd5c;
pub const SE_GSI_EVENT_EN:u32=0xe18; pub const SE_IRQ_EN:u32=0xe1c; pub const DMA_IF_EN_RO:u32=0xe20; pub const SE_HW_PARAM_0:u32=0xe24; pub const SE_HW_PARAM_1:u32=0xe28; pub const SE_HW_PARAM_2:u32=0xe2c; pub const DMA_GENERAL_CFG:u32=0xe30; pub const SE_DMA_QSB_TRANS_CFG:u32=0xe38; pub const SE_DMA_DEBUG_REG0:u32=0xe40; pub const SE_DMA_IF_EN:u32=0x2004;

pub const FORCE_DEFAULT:u32=bit!(0); pub const GENI_IO_MUX_0_EN:u32=bit!(0); pub const M_GENI_CMD_ACTIVE:u32=bit!(0); pub const S_GENI_CMD_ACTIVE:u32=bit!(12); pub const SER_CLK_EN:u32=bit!(0); pub const CLK_DIV_MSK:u32=genmask!(15,4); pub const CLK_DIV_SHFT:u32=4; pub const FIFO_IF_DISABLE:u32=bit!(0); pub const FW_REV_PROTOCOL_MSK:u32=genmask!(15,8); pub const FW_REV_PROTOCOL_SHFT:u32=8; pub const CLK_SEL_MSK:u32=genmask!(2,0); pub const START_TRIGGER:u32=bit!(0); pub const GENI_DMA_MODE_EN:u32=bit!(0);
pub const M_OPCODE_MSK:u32=genmask!(31,27); pub const M_OPCODE_SHFT:u32=27; pub const M_PARAMS_MSK:u32=genmask!(26,0); pub const M_GENI_CMD_CANCEL:u32=bit!(2); pub const M_GENI_CMD_ABORT:u32=bit!(1); pub const M_GENI_DISABLE:u32=bit!(0); pub const S_OPCODE_MSK:u32=genmask!(31,27); pub const S_OPCODE_SHFT:u32=27; pub const S_PARAMS_MSK:u32=genmask!(26,0); pub const S_GENI_CMD_CANCEL:u32=bit!(2); pub const S_GENI_CMD_ABORT:u32=bit!(1); pub const S_GENI_DISABLE:u32=bit!(0);

pub const M_CMD_DONE_EN:u32=bit!(0); pub const M_CMD_OVERRUN_EN:u32=bit!(1); pub const M_ILLEGAL_CMD_EN:u32=bit!(2); pub const M_CMD_FAILURE_EN:u32=bit!(3); pub const M_CMD_CANCEL_EN:u32=bit!(4); pub const M_CMD_ABORT_EN:u32=bit!(5); pub const M_TIMESTAMP_EN:u32=bit!(6); pub const M_RX_IRQ_EN:u32=bit!(7); pub const M_GP_SYNC_IRQ_0_EN:u32=bit!(8); pub const M_GP_IRQ_0_EN:u32=bit!(9); pub const M_GP_IRQ_1_EN:u32=bit!(10); pub const M_GP_IRQ_2_EN:u32=bit!(11); pub const M_GP_IRQ_3_EN:u32=bit!(12); pub const M_GP_IRQ_4_EN:u32=bit!(13); pub const M_GP_IRQ_5_EN:u32=bit!(14); pub const M_TX_FIFO_NOT_EMPTY_EN:u32=bit!(21); pub const M_IO_DATA_DEASSERT_EN:u32=bit!(22); pub const M_IO_DATA_ASSERT_EN:u32=bit!(23); pub const M_RX_FIFO_RD_ERR_EN:u32=bit!(24); pub const M_RX_FIFO_WR_ERR_EN:u32=bit!(25); pub const M_RX_FIFO_WATERMARK_EN:u32=bit!(26); pub const M_RX_FIFO_LAST_EN:u32=bit!(27); pub const M_TX_FIFO_RD_ERR_EN:u32=bit!(28); pub const M_TX_FIFO_WR_ERR_EN:u32=bit!(29); pub const M_TX_FIFO_WATERMARK_EN:u32=bit!(30); pub const M_SEC_IRQ_EN:u32=bit!(31);
pub const M_COMMON_GENI_M_IRQ_EN:u32=genmask!(6,1)|M_IO_DATA_DEASSERT_EN|M_IO_DATA_ASSERT_EN|M_RX_FIFO_RD_ERR_EN|M_RX_FIFO_WR_ERR_EN|M_TX_FIFO_RD_ERR_EN|M_TX_FIFO_WR_ERR_EN;
pub const S_CMD_DONE_EN:u32=bit!(0); pub const S_CMD_OVERRUN_EN:u32=bit!(1); pub const S_ILLEGAL_CMD_EN:u32=bit!(2); pub const S_CMD_FAILURE_EN:u32=bit!(3); pub const S_CMD_CANCEL_EN:u32=bit!(4); pub const S_CMD_ABORT_EN:u32=bit!(5); pub const S_GP_SYNC_IRQ_0_EN:u32=bit!(8); pub const S_GP_IRQ_0_EN:u32=bit!(9); pub const S_GP_IRQ_1_EN:u32=bit!(10); pub const S_GP_IRQ_2_EN:u32=bit!(11); pub const S_GP_IRQ_3_EN:u32=bit!(12); pub const S_GP_IRQ_4_EN:u32=bit!(13); pub const S_GP_IRQ_5_EN:u32=bit!(14); pub const S_IO_DATA_DEASSERT_EN:u32=bit!(22); pub const S_IO_DATA_ASSERT_EN:u32=bit!(23); pub const S_RX_FIFO_RD_ERR_EN:u32=bit!(24); pub const S_RX_FIFO_WR_ERR_EN:u32=bit!(25); pub const S_RX_FIFO_WATERMARK_EN:u32=bit!(26); pub const S_RX_FIFO_LAST_EN:u32=bit!(27); pub const S_COMMON_GENI_S_IRQ_EN:u32=genmask!(5,1)|genmask!(13,9)|S_RX_FIFO_RD_ERR_EN|S_RX_FIFO_WR_ERR_EN;

pub const WATERMARK_MSK:u32=genmask!(5,0); pub const TX_FIFO_WC:u32=genmask!(27,0); pub const RX_LAST:u32=bit!(31); pub const RX_LAST_BYTE_VALID_MSK:u32=genmask!(30,28); pub const RX_LAST_BYTE_VALID_SHFT:u32=28; pub const RX_FIFO_WC_MSK:u32=genmask!(24,0); pub const IO2_DATA_IN:u32=bit!(1); pub const RX_DATA_IN:u32=bit!(0); pub const GP_LENGTH:u32=genmask!(31,0); pub const TX_DMA_DONE:u32=bit!(0); pub const TX_EOT:u32=bit!(1); pub const TX_SBE:u32=bit!(2); pub const TX_RESET_DONE:u32=bit!(3); pub const RX_DMA_DONE:u32=bit!(0); pub const RX_EOT:u32=bit!(1); pub const RX_SBE:u32=bit!(2); pub const RX_RESET_DONE:u32=bit!(3); pub const RX_FLUSH_DONE:u32=bit!(4); pub const RX_DMA_PARITY_ERR:u32=bit!(5); pub const RX_DMA_BREAK:u32=genmask!(8,7); pub const RX_GENI_GP_IRQ:u32=genmask!(10,5); pub const RX_GENI_GP_IRQ_EXT:u32=genmask!(13,12); pub const RX_GENI_CANCEL_IRQ:u32=bit!(14); pub const DMA_TX_ACTIVE:u32=bit!(0); pub const DMA_RX_ACTIVE:u32=bit!(1); pub const DMA_TX_STATE:u32=genmask!(7,4); pub const DMA_RX_STATE:u32=genmask!(11,8);
pub const TX_FIFO_WIDTH_MSK:u32=genmask!(29,24); pub const TX_FIFO_WIDTH_SHFT:u32=24; pub const TX_FIFO_DEPTH_MSK_256_BYTES:u32=genmask!(23,16); pub const TX_FIFO_DEPTH_MSK:u32=genmask!(21,16); pub const TX_FIFO_DEPTH_SHFT:u32=16; pub const RX_FIFO_WIDTH_MSK:u32=genmask!(29,24); pub const RX_FIFO_WIDTH_SHFT:u32=24; pub const RX_FIFO_DEPTH_MSK_256_BYTES:u32=genmask!(23,16); pub const RX_FIFO_DEPTH_MSK:u32=genmask!(21,16); pub const RX_FIFO_DEPTH_SHFT:u32=16; pub const PROG_RAM_DEPTH_MSK:u32=genmask!(10,0); pub const HW_VER_MAJOR_MASK:u32=genmask!(31,28); pub const HW_VER_MAJOR_SHFT:u32=28; pub const HW_VER_MINOR_MASK:u32=genmask!(27,16); pub const HW_VER_MINOR_SHFT:u32=16; pub const HW_VER_STEP_MASK:u32=genmask!(15,0); pub const QUP_SE_VERSION_2_5:u32=0x20050000;
pub const CORE_2X_19_2_MHZ:u32=960; pub const CORE_2X_50_MHZ:u32=2500; pub const CORE_2X_100_MHZ:u32=5000; pub const CORE_2X_150_MHZ:u32=7500; pub const CORE_2X_200_MHZ:u32=10000; pub const CORE_2X_236_MHZ:u32=16383;
// GENI_DEFAULT_BW is Bps_to_icc(1000), supplied by the interconnect dependency.

pub const fn geni_se_version_major(ver:u32)->u32 {(ver&HW_VER_MAJOR_MASK)>>HW_VER_MAJOR_SHFT}
pub const fn geni_se_version_minor(ver:u32)->u32 {(ver&HW_VER_MINOR_MASK)>>HW_VER_MINOR_SHFT}
pub const fn geni_se_version_step(ver:u32)->u32 {ver&HW_VER_STEP_MASK}

pub unsafe fn geni_se_read_proto(se:*mut geni_se)->u32 { (readl_relaxed((*se).base.add(GENI_FW_REVISION_RO as usize)) & FW_REV_PROTOCOL_MSK) >> FW_REV_PROTOCOL_SHFT }
pub unsafe fn geni_se_setup_m_cmd(se:*mut geni_se,cmd:u32,params:u32) { writel((cmd << M_OPCODE_SHFT) | (params & M_PARAMS_MSK), (*se).base.add(SE_GENI_M_CMD0 as usize)); }
pub unsafe fn geni_se_setup_s_cmd(se:*mut geni_se,cmd:u32,params:u32) { let mut s=readl_relaxed((*se).base.add(SE_GENI_S_CMD0 as usize)); s &= !(S_OPCODE_MSK|S_PARAMS_MSK); s |= cmd << S_OPCODE_SHFT; s |= params & S_PARAMS_MSK; writel(s,(*se).base.add(SE_GENI_S_CMD0 as usize)); }
pub unsafe fn geni_se_cancel_m_cmd(se:*mut geni_se) { writel_relaxed(M_GENI_CMD_CANCEL,(*se).base.add(SE_GENI_M_CMD_CTRL_REG as usize)); }
pub unsafe fn geni_se_cancel_s_cmd(se:*mut geni_se) { writel_relaxed(S_GENI_CMD_CANCEL,(*se).base.add(SE_GENI_S_CMD_CTRL_REG as usize)); }
pub unsafe fn geni_se_abort_m_cmd(se:*mut geni_se) { writel_relaxed(M_GENI_CMD_ABORT,(*se).base.add(SE_GENI_M_CMD_CTRL_REG as usize)); }
pub unsafe fn geni_se_abort_s_cmd(se:*mut geni_se) { writel_relaxed(S_GENI_CMD_ABORT,(*se).base.add(SE_GENI_S_CMD_CTRL_REG as usize)); }
pub unsafe fn geni_se_get_tx_fifo_depth(se:*mut geni_se)->u32 { let v=geni_se_get_qup_hw_version(se); let major=geni_se_version_major(v); let minor=geni_se_version_minor(v); let mask=if (major==3&&minor>=10)||major>3 {TX_FIFO_DEPTH_MSK_256_BYTES} else {TX_FIFO_DEPTH_MSK}; (readl_relaxed((*se).base.add(SE_HW_PARAM_0 as usize))&mask)>>TX_FIFO_DEPTH_SHFT }
pub unsafe fn geni_se_get_tx_fifo_width(se:*mut geni_se)->u32 { (readl_relaxed((*se).base.add(SE_HW_PARAM_0 as usize))&TX_FIFO_WIDTH_MSK)>>TX_FIFO_WIDTH_SHFT }
pub unsafe fn geni_se_get_rx_fifo_depth(se:*mut geni_se)->u32 { let v=geni_se_get_qup_hw_version(se); let major=geni_se_version_major(v); let minor=geni_se_version_minor(v); let mask=if (major==3&&minor>=10)||major>3 {RX_FIFO_DEPTH_MSK_256_BYTES} else {RX_FIFO_DEPTH_MSK}; (readl_relaxed((*se).base.add(SE_HW_PARAM_1 as usize))&mask)>>RX_FIFO_DEPTH_SHFT }

extern "C" {
    pub fn geni_se_get_qup_hw_version(se: *mut geni_se) -> u32;
    pub fn geni_se_init(se:*mut geni_se, rx_wm:u32, rx_rfr:u32);
    pub fn geni_se_select_mode(se:*mut geni_se, mode:geni_se_xfer_mode);
    pub fn geni_se_config_packing(se:*mut geni_se,bpw:i32,pack_words:i32,msb_to_lsb:bool,tx_cfg:bool,rx_cfg:bool);
    pub fn geni_se_resources_off(se:*mut geni_se)->i32; pub fn geni_se_resources_on(se:*mut geni_se)->i32; pub fn geni_se_clk_tbl_get(se:*mut geni_se,tbl:*mut *mut u64)->i32; pub fn geni_se_clk_freq_match(se:*mut geni_se,req_freq:u64,index:*mut u32,res_freq:*mut u64,exact:bool)->i32;
    pub fn geni_se_tx_init_dma(se:*mut geni_se,iova:u64,len:usize); pub fn geni_se_tx_dma_prep(se:*mut geni_se,buf:*mut core::ffi::c_void,len:usize,iova:*mut u64)->i32; pub fn geni_se_rx_init_dma(se:*mut geni_se,iova:u64,len:usize); pub fn geni_se_rx_dma_prep(se:*mut geni_se,buf:*mut core::ffi::c_void,len:usize,iova:*mut u64)->i32; pub fn geni_se_tx_dma_unprep(se:*mut geni_se,iova:u64,len:usize); pub fn geni_se_rx_dma_unprep(se:*mut geni_se,iova:u64,len:usize);
    pub fn geni_icc_get(se:*mut geni_se,icc_ddr:*const i8)->i32; pub fn geni_icc_set_bw(se:*mut geni_se)->i32; pub fn geni_icc_set_bw_ab(se:*mut geni_se,core_ab:u32,cfg_ab:u32,ddr_ab:u32)->i32; pub fn geni_icc_set_tag(se:*mut geni_se,tag:u32); pub fn geni_icc_enable(se:*mut geni_se)->i32; pub fn geni_icc_disable(se:*mut geni_se)->i32; pub fn geni_se_resources_init(se:*mut geni_se)->i32; pub fn geni_se_resources_activate(se:*mut geni_se)->i32; pub fn geni_se_resources_deactivate(se:*mut geni_se)->i32; pub fn geni_load_se_firmware(se:*mut geni_se,protocol:geni_se_protocol_type)->i32; pub fn geni_se_domain_attach(se:*mut geni_se)->i32; pub fn geni_se_set_perf_level(se:*mut geni_se,level:u64)->i32; pub fn geni_se_set_perf_opp(se:*mut geni_se,clk_freq:u64)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
