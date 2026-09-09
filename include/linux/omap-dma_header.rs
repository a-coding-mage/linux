/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Legacy OMAP DMA handling defines and functions
 *
 * NOTE: Do not use these any longer.
 * Use the generic dmaengine functions as defined in include/linux/dmaengine.h.
 */

pub const INT_DMA_LCD: u32 = NR_IRQS_LEGACY + 25;

pub const OMAP1_DMA_TOUT_IRQ: u32 = 1 << 0;
pub const OMAP_DMA_DROP_IRQ: u32 = 1 << 1;
pub const OMAP_DMA_HALF_IRQ: u32 = 1 << 2;
pub const OMAP_DMA_FRAME_IRQ: u32 = 1 << 3;
pub const OMAP_DMA_LAST_IRQ: u32 = 1 << 4;
pub const OMAP_DMA_BLOCK_IRQ: u32 = 1 << 5;
pub const OMAP1_DMA_SYNC_IRQ: u32 = 1 << 6;
pub const OMAP2_DMA_PKT_IRQ: u32 = 1 << 7;
pub const OMAP2_DMA_TRANS_ERR_IRQ: u32 = 1 << 8;
pub const OMAP2_DMA_SECURE_ERR_IRQ: u32 = 1 << 9;
pub const OMAP2_DMA_SUPERVISOR_ERR_IRQ: u32 = 1 << 10;
pub const OMAP2_DMA_MISALIGNED_ERR_IRQ: u32 = 1 << 11;

pub const OMAP_DMA_CCR_EN: u32 = 1 << 7;
pub const OMAP_DMA_CCR_RD_ACTIVE: u32 = 1 << 9;
pub const OMAP_DMA_CCR_WR_ACTIVE: u32 = 1 << 10;
pub const OMAP_DMA_CCR_SEL_SRC_DST_SYNC: u32 = 1 << 24;
pub const OMAP_DMA_CCR_BUFFERING_DISABLE: u32 = 1 << 25;

pub const OMAP_DMA_DATA_TYPE_S8: u32 = 0x00;
pub const OMAP_DMA_DATA_TYPE_S16: u32 = 0x01;
pub const OMAP_DMA_DATA_TYPE_S32: u32 = 0x02;
pub const OMAP_DMA_SYNC_ELEMENT: u32 = 0x00;
pub const OMAP_DMA_SYNC_FRAME: u32 = 0x01;
pub const OMAP_DMA_SYNC_BLOCK: u32 = 0x02;
pub const OMAP_DMA_SYNC_PACKET: u32 = 0x03;
pub const OMAP_DMA_DST_SYNC_PREFETCH: u32 = 0x02;
pub const OMAP_DMA_SRC_SYNC: u32 = 0x01;
pub const OMAP_DMA_DST_SYNC: u32 = 0x00;
pub const OMAP_DMA_PORT_EMIFF: u32 = 0x00;
pub const OMAP_DMA_PORT_EMIFS: u32 = 0x01;
pub const OMAP_DMA_PORT_OCP_T1: u32 = 0x02;
pub const OMAP_DMA_PORT_TIPB: u32 = 0x03;
pub const OMAP_DMA_PORT_OCP_T2: u32 = 0x04;
pub const OMAP_DMA_PORT_MPUI: u32 = 0x05;
pub const OMAP_DMA_AMODE_CONSTANT: u32 = 0x00;
pub const OMAP_DMA_AMODE_POST_INC: u32 = 0x01;
pub const OMAP_DMA_AMODE_SINGLE_IDX: u32 = 0x02;
pub const OMAP_DMA_AMODE_DOUBLE_IDX: u32 = 0x03;

pub const DMA_DEFAULT_FIFO_DEPTH: u32 = 0x10;
pub const DMA_DEFAULT_ARB_RATE: u32 = 0x01;
pub const DMA_THREAD_RESERVE_NORM: u32 = 0x00 << 12;
pub const DMA_THREAD_RESERVE_ONET: u32 = 0x01 << 12;
pub const DMA_THREAD_RESERVE_TWOT: u32 = 0x02 << 12;
pub const DMA_THREAD_RESERVE_THREET: u32 = 0x03 << 12;
pub const DMA_THREAD_FIFO_NONE: u32 = 0x00 << 14;
pub const DMA_THREAD_FIFO_75: u32 = 0x01 << 14;
pub const DMA_THREAD_FIFO_25: u32 = 0x02 << 14;
pub const DMA_THREAD_FIFO_50: u32 = 0x03 << 14;

pub const DMA_SYSCONFIG_MIDLEMODE_MASK: u32 = 3 << 12;
pub const DMA_SYSCONFIG_CLOCKACTIVITY_MASK: u32 = 3 << 8;
pub const DMA_SYSCONFIG_EMUFREE: u32 = 1 << 5;
pub const DMA_SYSCONFIG_SIDLEMODE_MASK: u32 = 3 << 3;
pub const DMA_SYSCONFIG_SOFTRESET: u32 = 1 << 2;
pub const DMA_SYSCONFIG_AUTOIDLE: u32 = 1;
pub const DMA_IDLEMODE_SMARTIDLE: u32 = 0x2;
pub const DMA_IDLEMODE_NO_IDLE: u32 = 0x1;
pub const DMA_IDLEMODE_FORCE_IDLE: u32 = 0x0;

#[inline] pub const fn DMA_SYSCONFIG_MIDLEMODE(n: u32) -> u32 { n << 12 }
#[inline] pub const fn DMA_SYSCONFIG_SIDLEMODE(n: u32) -> u32 { n << 3 }

pub const OMAP_DMA_STATIC_CHAIN: u32 = 0x1;
pub const OMAP_DMA_DYNAMIC_CHAIN: u32 = 0x2;
pub const OMAP_DMA_CHAIN_ACTIVE: u32 = 0x1;
pub const OMAP_DMA_CHAIN_INACTIVE: u32 = 0x0;
pub const DMA_CH_PRIO_HIGH: u32 = 0x1;
pub const DMA_CH_PRIO_LOW: u32 = 0x0;

pub const DMA_ERRATA_IFRAME_BUFFERING: u32 = 1 << 0;
pub const DMA_ERRATA_PARALLEL_CHANNELS: u32 = 1 << 1;
pub const DMA_ERRATA_i378: u32 = 1 << 2;
pub const DMA_ERRATA_i541: u32 = 1 << 3;
pub const DMA_ERRATA_i88: u32 = 1 << 4;
pub const DMA_ERRATA_3_3: u32 = 1 << 5;
pub const DMA_ROMCODE_BUG: u32 = 1 << 6;
pub const DMA_LINKED_LCH: u32 = 1 << 0;
pub const GLOBAL_PRIORITY: u32 = 1 << 1;
pub const RESERVE_CHANNEL: u32 = 1 << 2;
pub const IS_CSSA_32: u32 = 1 << 3;
pub const IS_CDSA_32: u32 = 1 << 4;
pub const IS_RW_PRIORITY: u32 = 1 << 5;
pub const ENABLE_1510_MODE: u32 = 1 << 6;
pub const SRC_PORT: u32 = 1 << 7;
pub const DST_PORT: u32 = 1 << 8;
pub const SRC_INDEX: u32 = 1 << 9;
pub const DST_INDEX: u32 = 1 << 10;
pub const IS_BURST_ONLY4: u32 = 1 << 11;
pub const CLEAR_CSR_ON_READ: u32 = 1 << 12;
pub const IS_WORD_16: u32 = 1 << 13;
pub const ENABLE_16XX_MODE: u32 = 1 << 14;
pub const HS_CHANNELS_RESERVED: u32 = 1 << 15;
pub const DMA_HAS_TRANSPARENT_CAPS: u32 = 0x1 << 18;
pub const DMA_HAS_CONSTANT_FILL_CAPS: u32 = 0x1 << 19;
pub const DMA_HAS_DESCRIPTOR_CAPS: u32 = 0x3 << 20;

#[repr(u32)]
pub enum omap_reg_offsets {
    GCR, GSCR, GRST1, HW_ID, PCH2_ID, PCH0_ID, PCH1_ID, PCHG_ID, PCHD_ID,
    CAPS_0, CAPS_1, CAPS_2, CAPS_3, CAPS_4, PCH2_SR, PCH0_SR, PCH1_SR,
    PCHD_SR, REVISION, IRQSTATUS_L0, IRQSTATUS_L1, IRQSTATUS_L2, IRQSTATUS_L3,
    IRQENABLE_L0, IRQENABLE_L1, IRQENABLE_L2, IRQENABLE_L3, SYSSTATUS,
    OCP_SYSCONFIG, CPC, CCR2, LCH_CTRL, CSDP, CCR, CICR, CSR, CEN, CFN,
    CSFI, CSEI, CSAC, CDAC, CDEI, CDFI, CLNK_CTRL, CSSA, CDSA, COLOR, CCEN,
    CCFN, CDP, CNDP, CCDN,
}

#[repr(C)]
pub struct omap_dma_channel_params {
    pub data_type: i32, pub elem_count: i32, pub frame_count: i32,
    pub src_port: i32, pub src_amode: i32, pub src_start: usize, pub src_ei: i32, pub src_fi: i32,
    pub dst_port: i32, pub dst_amode: i32, pub dst_start: usize, pub dst_ei: i32, pub dst_fi: i32,
    pub trigger: i32, pub sync_mode: i32, pub src_or_dst_synch: i32, pub ie: i32,
    pub read_prio: u8, pub write_prio: u8,
    pub burst_mode: omap_dma_burst_mode,
}

#[repr(u32)] pub enum omap_dma_burst_mode { OMAP_DMA_DATA_BURST_DIS = 0, OMAP_DMA_DATA_BURST_4, OMAP_DMA_DATA_BURST_8, OMAP_DMA_DATA_BURST_16 }
#[repr(u32)] pub enum end_type { OMAP_DMA_LITTLE_ENDIAN = 0, OMAP_DMA_BIG_ENDIAN }
#[repr(u32)] pub enum omap_dma_color_mode { OMAP_DMA_COLOR_DIS = 0, OMAP_DMA_CONSTANT_FILL, OMAP_DMA_TRANSPARENT_COPY }
#[repr(u32)] pub enum omap_dma_write_mode { OMAP_DMA_WRITE_NON_POSTED = 0, OMAP_DMA_WRITE_POSTED, OMAP_DMA_WRITE_LAST_NON_POSTED }
#[repr(u32)] pub enum omap_dma_channel_mode { OMAP_DMA_LCH_2D = 0, OMAP_DMA_LCH_G, OMAP_DMA_LCH_P, OMAP_DMA_LCH_PD }

#[repr(C)] pub struct omap_dma_lch {
    pub next_lch: i32, pub dev_id: i32, pub saved_csr: u16, pub enabled_irqs: u16,
    pub dev_name: *const core::ffi::c_char,
    pub callback: Option<unsafe extern "C" fn(i32, u16, *mut core::ffi::c_void)>,
    pub data: *mut core::ffi::c_void, pub flags: isize, pub state: i32, pub chain_id: i32, pub status: i32,
}
#[repr(C)] pub struct omap_dma_dev_attr { pub dev_caps: u32, pub lch_count: u16, pub chan_count: u16 }
pub const OMAP_DMA_REG_NONE: u32 = 0; pub const OMAP_DMA_REG_16BIT: u32 = 1; pub const OMAP_DMA_REG_2X16BIT: u32 = 2; pub const OMAP_DMA_REG_32BIT: u32 = 3;
#[repr(C)] pub struct omap_dma_reg { pub offset: u16, pub stride: u8, pub type_: u8 }
pub fn SDMA_FILTER_PARAM(hw_req: i32) -> [i32; 1] { [hw_req] }
pub struct dma_slave_map;
#[repr(C)] pub struct omap_system_dma_plat_info {
    pub reg_map: *const omap_dma_reg, pub channel_stride: u32, pub dma_attr: *mut omap_dma_dev_attr,
    pub errata: u32, pub show_dma_caps: Option<unsafe extern "C" fn()>, pub clear_lch_regs: Option<unsafe extern "C" fn(i32)>,
    pub clear_dma: Option<unsafe extern "C" fn(i32)>, pub dma_write: Option<unsafe extern "C" fn(u32, i32, i32)>,
    pub dma_read: Option<unsafe extern "C" fn(i32, i32) -> u32>, pub slave_map: *const dma_slave_map, pub slavecnt: i32,
}

extern "C" {
    pub fn omap_get_plat_info() -> *mut omap_system_dma_plat_info;
    pub fn omap_request_dma(dev_id: i32, dev_name: *const core::ffi::c_char, callback: Option<unsafe extern "C" fn(i32, u16, *mut core::ffi::c_void)>, data: *mut core::ffi::c_void, dma_ch: *mut i32) -> i32;
    pub fn omap_free_dma(ch: i32);
    pub fn omap_disable_dma_irq(ch: i32, irq_bits: u16);
    pub fn omap_start_dma(lch: i32);
    pub fn omap_stop_dma(lch: i32);
    pub fn omap_set_dma_transfer_params(lch: i32, data_type: i32, elem_count: i32, frame_count: i32, sync_mode: i32, dma_trigger: i32, src_or_dst_synch: i32);
    pub fn omap_set_dma_channel_mode(lch: i32, mode: omap_dma_channel_mode);
    pub fn omap_set_dma_src_params(lch: i32, src_port: i32, src_amode: i32, src_start: usize, src_ei: i32, src_fi: i32);
    pub fn omap_set_dma_src_data_pack(lch: i32, enable: i32);
    pub fn omap_set_dma_src_burst_mode(lch: i32, burst_mode: omap_dma_burst_mode);
    pub fn omap_set_dma_dest_params(lch: i32, dest_port: i32, dest_amode: i32, dest_start: usize, dst_ei: i32, dst_fi: i32);
    pub fn omap_set_dma_dest_data_pack(lch: i32, enable: i32);
    pub fn omap_set_dma_dest_burst_mode(lch: i32, burst_mode: omap_dma_burst_mode);
    pub fn omap_get_dma_src_pos(lch: i32) -> usize;
    pub fn omap_get_dma_dst_pos(lch: i32) -> usize;
    pub fn omap_get_dma_active_status(lch: i32) -> i32;
    pub fn omap_dma_running() -> i32;
}
#[cfg(target_arch = "unknown")]
extern "C" { pub fn omap_set_dma_priority(lch: i32, dst_port: i32, priority: i32); }
#[inline] pub fn IS_DMA_ERRATA(errata: u32, id: u32) -> u32 { errata & id }
#[inline] pub fn SET_DMA_ERRATA(errata: &mut u32, id: u32) { *errata |= id; }
#[inline] pub const fn dma_omap2plus() -> bool { false }
#[inline] pub const fn dma_omap1() -> bool { !dma_omap2plus() }
#[inline] pub unsafe fn __dma_omap15xx(d: *const omap_dma_dev_attr) -> bool { dma_omap1() && ((*d).dev_caps & ENABLE_1510_MODE) != 0 }
#[inline] pub unsafe fn __dma_omap16xx(d: *const omap_dma_dev_attr) -> bool { dma_omap1() && ((*d).dev_caps & ENABLE_16XX_MODE) != 0 }
#[inline] pub unsafe fn dma_omap15xx(d: *const omap_dma_dev_attr) -> bool { __dma_omap15xx(d) }
#[inline] pub unsafe fn dma_omap16xx(d: *const omap_dma_dev_attr) -> bool { __dma_omap16xx(d) }
#[inline] pub fn omap_lcd_dma_running() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
