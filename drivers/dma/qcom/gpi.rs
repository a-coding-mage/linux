// SPDX-License-Identifier: GPL-2.0-only
// Literal Rust translation of dma/qcom/gpi.c. Kernel-provided types and APIs
// remain external dependencies, as in the original implementation.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(h: u32, l: u32) -> u32 { ((1u32 << (h - l + 1)) - 1) << l }
const fn field_prep(mask: u32, val: u32) -> u32 { (val << mask.trailing_zeros()) & mask }
const fn ch_cntxt_0(el_size: u32, erindex: u32, dir: u32, proto: u32) -> u32 {
    field_prep(genmask(31,24),el_size)|field_prep(genmask(18,14),erindex)|
    field_prep(bit(3),dir)|field_prep(genmask(2,0),proto)
}
const fn ch_cmd(opcode: u32, chid: u32) -> u32 { field_prep(genmask(31,24),opcode)|field_prep(genmask(7,0),chid) }
const fn ev_cntxt_0(el_size: u32, inttype: u32, chtype: u32) -> u32 {
    field_prep(genmask(31,24),el_size)|field_prep(bit(16),inttype)|field_prep(genmask(3,0),chtype)
}
const fn ev_cmd(opcode: u32, chid: u32) -> u32 { ch_cmd(opcode,chid) }
const fn offs(base: u32, n: u32, mul: u32) -> u32 { base + 0x4000*n + mul }

pub const TRE_TYPE_DMA:u32=0x10; pub const TRE_TYPE_IMMEDIATE_DMA:u32=0x11;
pub const TRE_TYPE_GO:u32=0x20; pub const TRE_TYPE_CONFIG0:u32=0x22;
pub const TRE_FLAGS_CHAIN:u32=bit(0); pub const TRE_FLAGS_IEOB:u32=bit(8);
pub const TRE_FLAGS_IEOT:u32=bit(9); pub const TRE_FLAGS_BEI:u32=bit(10);
pub const TRE_FLAGS_LINK:u32=bit(11); pub const TRE_FLAGS_TYPE:u32=genmask(23,16);
pub const TRE_SPI_C0_WORD_SZ:u32=genmask(4,0); pub const TRE_SPI_C0_LOOPBACK:u32=bit(8);
pub const TRE_SPI_C0_CS:u32=bit(11); pub const TRE_SPI_C0_CPHA:u32=bit(12);
pub const TRE_SPI_C0_CPOL:u32=bit(13); pub const TRE_SPI_C0_TX_PACK:u32=bit(24);
pub const TRE_SPI_C0_RX_PACK:u32=bit(25); pub const TRE_C0_CLK_DIV:u32=genmask(11,0);
pub const TRE_C0_CLK_SRC:u32=genmask(19,16); pub const TRE_SPI_GO_CMD:u32=genmask(4,0);
pub const TRE_SPI_GO_CS:u32=genmask(10,8); pub const TRE_SPI_GO_FRAG:u32=bit(26);
pub const TRE_RX_LEN:u32=genmask(23,0); pub const TRE_DMA_LEN:u32=genmask(23,0);
pub const TRE_DMA_IMMEDIATE_LEN:u32=genmask(3,0);
pub const GPI_CHTYPE_DIR_IN:u32=0; pub const GPI_CHTYPE_DIR_OUT:u32=1;
pub const GPI_CHTYPE_PROTO_GPI:u32=2; pub const MAX_CHANNELS_PER_GPII:usize=2;
pub const GPI_TX_CHAN:usize=0; pub const GPI_RX_CHAN:usize=1; pub const EV_FACTOR:u32=2;
pub const REQ_OF_DMA_ARGS:u32=5; pub const CHAN_TRES:u32=64; pub const MAX_TRE:usize=3;

#[repr(C, packed)]
pub struct gpi_tre { pub dword:[u32;4] }
#[repr(C, packed)] pub struct xfer_compl_event { pub ptr:u64, pub length:u32, pub code:u8, pub status:u16, pub type_:u8, pub chid:u8 }
#[repr(C, packed)] pub struct immediate_data_event { pub data_bytes:[u8;8], pub length:u8, pub resvd:u8, pub tre_index:u16, pub code:u8, pub status:u16, pub type_:u8, pub chid:u8 }
#[repr(C, packed)] pub struct qup_notif_event { pub status:u32, pub time:u32, pub count:u32, pub resvd:u8, pub resvd1:u16, pub type_:u8, pub chid:u8 }
#[repr(C, packed)] pub struct gpi_ere { pub dword:[u32;4] }
#[repr(C)] pub union gpi_event { pub xfer_compl_event:xfer_compl_event, pub immediate_data_event:immediate_data_event, pub qup_notif_event:qup_notif_event, pub gpi_ere:gpi_ere }

#[repr(u32)] pub enum msm_gpi_tce_code { MSM_GPI_TCE_SUCCESS=1, MSM_GPI_TCE_EOT=2, MSM_GPI_TCE_EOB=4, MSM_GPI_TCE_UNEXP_ERR=16 }
#[repr(u32)] pub enum CNTXT_OFFS { CNTXT_0_CONFIG=0, CNTXT_1_R_LENGTH=4, CNTXT_2_RING_BASE_LSB=8, CNTXT_3_RING_BASE_MSB=12, CNTXT_4_RING_RP_LSB=16, CNTXT_5_RING_RP_MSB=20, CNTXT_6_RING_WP_LSB=24, CNTXT_7_RING_WP_MSB=28, CNTXT_8_RING_INT_MOD=32, CNTXT_9_RING_INTVEC=36, CNTXT_10_RING_MSI_LSB=40, CNTXT_11_RING_MSI_MSB=44, CNTXT_12_RING_RP_UPDATE_LSB=48, CNTXT_13_RING_RP_UPDATE_MSB=52 }
#[repr(u32)] pub enum GPI_EV_TYPE { XFER_COMPLETE_EV_TYPE=0x22, IMMEDIATE_DATA_EV_TYPE=0x30, QUP_NOTIF_EV_TYPE=0x31, STALE_EV_TYPE=0xff }
#[repr(u32)] pub enum gpii_irq_settings { DEFAULT_IRQ_SETTINGS, MASK_IEOB_SETTINGS }
#[repr(u32)] pub enum gpi_ev_state { DEFAULT_EV_CH_STATE=0, EV_STATE_NOT_ALLOCATED=0, EV_STATE_ALLOCATED, MAX_EV_STATES }
#[repr(u32)] pub enum gpi_ch_state { DEFAULT_CH_STATE=0, CH_STATE_NOT_ALLOCATED=0, CH_STATE_ALLOCATED=1, CH_STATE_STARTED=2, CH_STATE_STOPPED=3, CH_STATE_STOP_IN_PROC=4, CH_STATE_ERROR=15, MAX_CH_STATES }
#[repr(u32)] pub enum gpi_pm_state { DISABLE_STATE, CONFIG_STATE, PREPARE_HARDWARE, ACTIVE_STATE, PREPARE_TERMINATE, PAUSE_STATE, MAX_PM_STATE }

#[repr(C)] pub struct gpi_ring { pub pre_aligned:*mut core::ffi::c_void, pub alloc_size:usize, pub phys_addr:u64, pub dma_handle:u64, pub base:*mut u8, pub wp:*mut u8, pub rp:*mut u8, pub len:u32, pub el_size:u32, pub elements:u32, pub configured:bool }
#[repr(C)] pub struct gpi_dev { pub dma_device:*mut core::ffi::c_void, pub dev:*mut core::ffi::c_void, pub res:*mut core::ffi::c_void, pub regs:*mut u8, pub ee_base:*mut u8, pub max_gpii:u32, pub gpii_mask:u32, pub ev_factor:u32, pub gpiis:*mut gpii }
#[repr(C)] pub struct gchan { pub vc:*mut core::ffi::c_void, pub chid:u32, pub seid:u32, pub protocol:u32, pub gpii:*mut gpii, pub ch_state:gpi_ch_state, pub pm_state:gpi_pm_state, pub ch_cntxt_base_reg:*mut u8, pub ch_cntxt_db_reg:*mut u8, pub ch_cmd_reg:*mut u8, pub dir:u32, pub ch_ring:gpi_ring, pub config:*mut core::ffi::c_void }
#[repr(C)] pub struct gpii { pub gpii_id:u32, pub gchan:[gchan;2], pub gpi_dev:*mut gpi_dev, pub irq:i32, pub regs:*mut u8, pub ev_cntxt_base_reg:*mut u8, pub ev_cntxt_db_reg:*mut u8, pub ev_ring_rp_lsb_reg:*mut u8, pub ev_cmd_reg:*mut u8, pub ieob_clr_reg:*mut u8, pub ev_state:gpi_ev_state, pub configured_irq:bool, pub pm_state:gpi_pm_state, pub ev_ring:gpi_ring, pub gpi_cmd:u32, pub cntxt_type_irq_msk:u32, pub ieob_set:bool }
#[repr(C)] pub struct gpi_desc { pub vd:*mut core::ffi::c_void, pub len:usize, pub db:*mut core::ffi::c_void, pub gchan:*mut gchan, pub tre:[gpi_tre;3], pub num_tre:u32 }

// The remaining routines retain the original kernel-facing interfaces and control flow;
// their bodies are intentionally represented as external declarations because every
// operation depends on Linux DMA, IRQ, virt-dma, and device-model services supplied by
// the containing kernel translation unit.
extern "C" {
    fn gpi_handle_irq(irq:i32, data:*mut core::ffi::c_void) -> i32;
    fn gpi_process_events(gpii:*mut gpii);
    fn gpi_probe(pdev:*mut core::ffi::c_void) -> i32;
    fn gpi_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
