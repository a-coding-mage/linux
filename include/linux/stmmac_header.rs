/* SPDX-License-Identifier: GPL-2.0-only */
/*******************************************************************************

  Header file for stmmac platform data

  Copyright (C) 2009  STMicroelectronics Ltd


  Author: Giuseppe Cavallaro <peppe.cavallaro@st.com>
*******************************************************************************/

// Translated from the C header. Kernel-provided types and symbols are external dependencies.

pub const MTL_MAX_RX_QUEUES: usize = 8;
pub const MTL_MAX_TX_QUEUES: usize = 8;
pub const STMMAC_CH_MAX: usize = 8;

pub const STMMAC_RX_COE_NONE: i32 = 0;
pub const STMMAC_RX_COE_TYPE1: i32 = 1;
pub const STMMAC_RX_COE_TYPE2: i32 = 2;

/* Define the macros for CSR clock range parameters to be passed by
 * platform code.
 * This could also be configured at run time using CPU freq framework. */

/* MDC Clock Selection define */
pub const STMMAC_CSR_60_100M: u32 = 0x0;
pub const STMMAC_CSR_100_150M: u32 = 0x1;
pub const STMMAC_CSR_20_35M: u32 = 0x2;
pub const STMMAC_CSR_35_60M: u32 = 0x3;
pub const STMMAC_CSR_150_250M: u32 = 0x4;
pub const STMMAC_CSR_250_300M: u32 = 0x5;
pub const STMMAC_CSR_300_500M: u32 = 0x6;
pub const STMMAC_CSR_500_800M: u32 = 0x7;

/* MTL algorithms identifiers */
pub const MTL_TX_ALGORITHM_WRR: u32 = 0x0;
pub const MTL_TX_ALGORITHM_WFQ: u32 = 0x1;
pub const MTL_TX_ALGORITHM_DWRR: u32 = 0x2;
pub const MTL_TX_ALGORITHM_SP: u32 = 0x3;
pub const MTL_RX_ALGORITHM_SP: u32 = 0x4;
pub const MTL_RX_ALGORITHM_WSP: u32 = 0x5;

/* RX/TX Queue Mode */
pub const MTL_QUEUE_AVB: u32 = 0x0;
pub const MTL_QUEUE_DCB: u32 = 0x1;

/* The MDC clock could be set higher than the IEEE 802.3
 * specified frequency limit 0f 2.5 MHz, by programming a clock divider
 * of value different than the above defined values. The resultant MDIO
 * clock frequency of 12.5 MHz is applicable for the interfacing chips
 * supporting higher MDC clocks.
 * The MDC clock selection macros need to be defined for MDC clock rate
 * of 12.5 MHz, corresponding to the following selection.
 */
pub const STMMAC_CSR_I_4: u32 = 0x8;
pub const STMMAC_CSR_I_6: u32 = 0x9;
pub const STMMAC_CSR_I_8: u32 = 0xA;
pub const STMMAC_CSR_I_10: u32 = 0xB;
pub const STMMAC_CSR_I_12: u32 = 0xC;
pub const STMMAC_CSR_I_14: u32 = 0xD;
pub const STMMAC_CSR_I_16: u32 = 0xE;
pub const STMMAC_CSR_I_18: u32 = 0xF;

/* AXI DMA Burst length supported */
pub const DMA_AXI_BLEN_4: u32 = 1 << 1;
pub const DMA_AXI_BLEN_8: u32 = 1 << 2;
pub const DMA_AXI_BLEN_16: u32 = 1 << 3;
pub const DMA_AXI_BLEN_32: u32 = 1 << 4;
pub const DMA_AXI_BLEN_64: u32 = 1 << 5;
pub const DMA_AXI_BLEN_128: u32 = 1 << 6;
pub const DMA_AXI_BLEN_256: u32 = 1 << 7;
pub const DMA_AXI_BLEN_ALL: u32 = DMA_AXI_BLEN_4 | DMA_AXI_BLEN_8 | DMA_AXI_BLEN_16 |
    DMA_AXI_BLEN_32 | DMA_AXI_BLEN_64 | DMA_AXI_BLEN_128 | DMA_AXI_BLEN_256;

#[repr(C)]
pub struct stmmac_mdio_bus_data {
    pub phy_mask: u32,
    pub pcs_mask: u32,
    pub irqs: *mut i32,
    pub probed_phy_irq: i32,
    pub needs_reset: bool,
}

#[repr(C)]
pub struct stmmac_dma_cfg {
    pub pbl: i32,
    pub txpbl: i32,
    pub rxpbl: i32,
    pub pblx8: bool,
    pub fixed_burst: bool,
    pub mixed_burst: bool,
    pub aal: bool,
    pub dche: bool,
    pub eame: bool,
    pub multi_msi_en: bool,
    pub atds: bool,
}

pub const AXI_BLEN: u32 = 7;

#[repr(C)]
pub struct stmmac_axi {
    pub axi_wr_osr_lmt: u32,
    pub axi_rd_osr_lmt: u32,
    pub axi_blen_regval: u32,
    pub axi_lpi_en: bool,
    pub axi_xit_frm: bool,
    pub axi_fb: bool,
}

#[repr(C)]
pub struct stmmac_rxq_cfg {
    pub chan: u32,
    pub prio: u32,
    pub mode_to_use: u8,
    pub pkt_route: u8,
    pub use_prio: bool,
}

#[repr(C)]
pub struct stmmac_txq_cfg {
    pub weight: u32,
    pub send_slope: u32,
    pub idle_slope: u32,
    pub high_credit: u32,
    pub low_credit: u32,
    pub prio: u32,
    pub tbs_en: i32,
    pub use_prio: bool,
    pub coe_unsupported: bool,
    pub mode_to_use: u8,
}

#[repr(C)]
pub struct stmmac_safety_feature_cfg {
    pub tsoee: u32,
    pub mrxpee: u32,
    pub mestee: u32,
    pub mrxee: u32,
    pub mtxee: u32,
    pub epsi: u32,
    pub edpp: u32,
    pub prtyen: u32,
    pub tmouten: u32,
}

#[repr(C)]
pub struct dwmac4_addrs {
    pub dma_chan: u32,
    pub dma_chan_offset: u32,
    pub mtl_chan: u32,
    pub mtl_chan_offset: u32,
    pub mtl_ets_ctrl: u32,
    pub mtl_ets_ctrl_offset: u32,
    pub mtl_txq_weight: u32,
    pub mtl_txq_weight_offset: u32,
    pub mtl_send_slp_cred: u32,
    pub mtl_send_slp_cred_offset: u32,
    pub mtl_high_cred: u32,
    pub mtl_high_cred_offset: u32,
    pub mtl_low_cred: u32,
    pub mtl_low_cred_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dwmac_core_type {
    DWMAC_CORE_MAC100,
    DWMAC_CORE_GMAC,
    DWMAC_CORE_GMAC4,
    DWMAC_CORE_XGMAC,
}

pub const STMMAC_FLAG_SPH_DISABLE: u32 = 1 << 1;
pub const STMMAC_FLAG_USE_PHY_WOL: u32 = 1 << 2;
pub const STMMAC_FLAG_HAS_SUN8I: u32 = 1 << 3;
pub const STMMAC_FLAG_TSO_EN: u32 = 1 << 4;
pub const STMMAC_FLAG_SERDES_UP_AFTER_PHY_LINKUP: u32 = 1 << 5;
pub const STMMAC_FLAG_VLAN_FAIL_Q_EN: u32 = 1 << 6;
pub const STMMAC_FLAG_MULTI_MSI_EN: u32 = 1 << 7;
pub const STMMAC_FLAG_EXT_SNAPSHOT_EN: u32 = 1 << 8;
pub const STMMAC_FLAG_INT_SNAPSHOT_EN: u32 = 1 << 9;
pub const STMMAC_FLAG_EEE_DISABLE: u32 = 1 << 10;
pub const STMMAC_FLAG_RX_CLK_RUNS_IN_LPI: u32 = 1 << 11;
pub const STMMAC_FLAG_EN_TX_LPI_CLOCKGATING: u32 = 1 << 12;
pub const STMMAC_FLAG_EN_TX_LPI_CLK_PHY_CAP: u32 = 1 << 13;
pub const STMMAC_FLAG_HWTSTAMP_CORRECT_LATENCY: u32 = 1 << 14;
pub const STMMAC_FLAG_KEEP_PREAMBLE_BEFORE_SFD: u32 = 1 << 15;
pub const STMMAC_FLAG_SERDES_SUPPORTS_2500M: u32 = 1 << 16;

#[repr(C)]
pub struct plat_stmmacenet_data {
    pub core_type: dwmac_core_type,
    pub bus_id: i32,
    pub phy_addr: i32,
    pub phy_interface: phy_interface_t,
    pub mdio_bus_data: *mut stmmac_mdio_bus_data,
    pub phy_node: *mut device_node,
    pub mdio_node: *mut device_node,
    pub dma_cfg: *mut stmmac_dma_cfg,
    pub safety_feat_cfg: *mut stmmac_safety_feature_cfg,
    pub clk_csr: i32,
    pub default_an_inband: bool,
    pub enh_desc: bool,
    pub tx_coe: bool,
    pub bugged_jumbo: bool,
    pub pmt: bool,
    pub force_sf_dma_mode: bool,
    pub force_thresh_dma_mode: bool,
    pub riwt_off: bool,
    pub rx_coe: i32,
    pub max_speed: i32,
    pub maxmtu: i32,
    pub multicast_filter_bins: i32,
    pub unicast_filter_entries: i32,
    pub tx_fifo_size: i32,
    pub rx_fifo_size: i32,
    pub host_dma_width: u8,
    pub rx_queues_to_use: u8,
    pub tx_queues_to_use: u8,
    pub rx_sched_algorithm: u8,
    pub tx_sched_algorithm: u8,
    pub rx_queues_cfg: [stmmac_rxq_cfg; MTL_MAX_RX_QUEUES],
    pub tx_queues_cfg: [stmmac_txq_cfg; MTL_MAX_TX_QUEUES],
    pub get_interfaces: Option<unsafe extern "C" fn(*mut stmmac_priv, *mut core::ffi::c_void, *mut c_ulong)>,
    pub set_phy_intf_sel: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u8) -> i32>,
    pub set_clk_tx_rate: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut clk, phy_interface_t, i32) -> i32>,
    pub fix_mac_speed: Option<unsafe extern "C" fn(*mut core::ffi::c_void, phy_interface_t, i32, u32)>,
    pub fix_soc_reset: Option<unsafe extern "C" fn(*mut stmmac_priv) -> i32>,
    pub serdes_powerup: Option<unsafe extern "C" fn(*mut net_device, *mut core::ffi::c_void) -> i32>,
    pub serdes_powerdown: Option<unsafe extern "C" fn(*mut net_device, *mut core::ffi::c_void)>,
    pub mac_finish: Option<unsafe extern "C" fn(*mut net_device, *mut core::ffi::c_void, u32, phy_interface_t) -> i32>,
    pub ptp_clk_freq_config: Option<unsafe extern "C" fn(*mut stmmac_priv)>,
    pub init: Option<unsafe extern "C" fn(*mut device, *mut core::ffi::c_void) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut device, *mut core::ffi::c_void)>,
    pub suspend: Option<unsafe extern "C" fn(*mut device, *mut core::ffi::c_void) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut device, *mut core::ffi::c_void) -> i32>,
    pub mac_setup: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut mac_device_info) -> i32>,
    pub clks_config: Option<unsafe extern "C" fn(*mut core::ffi::c_void, bool) -> i32>,
    pub crosststamp: Option<unsafe extern "C" fn(*mut ktime_t, *mut system_counterval_t, *mut core::ffi::c_void) -> i32>,
    pub dump_debug_regs: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub pcs_init: Option<unsafe extern "C" fn(*mut stmmac_priv) -> i32>,
    pub pcs_exit: Option<unsafe extern "C" fn(*mut stmmac_priv)>,
    pub select_pcs: Option<unsafe extern "C" fn(*mut stmmac_priv, phy_interface_t) -> *mut phylink_pcs>,
    pub bsp_priv: *mut core::ffi::c_void,
    pub stmmac_clk: *mut clk,
    pub pclk: *mut clk,
    pub clk_ptp_ref: *mut clk,
    pub clk_tx_i: *mut clk,
    pub clk_ptp_rate: c_ulong,
    pub clk_ref_rate: c_ulong,
    pub clks: *mut clk_bulk_data,
    pub num_clks: i32,
    pub mult_fact_100ns: u32,
    pub ptp_max_adj: i32,
    pub cdc_error_adj: u32,
    pub stmmac_rst: *mut reset_control,
    pub stmmac_ahb_rst: *mut reset_control,
    pub axi: *mut stmmac_axi,
    pub rss_en: i32,
    pub mac_port_sel_speed: i32,
    pub vlan_fail_q: u8,
    pub provide_bus_info: bool,
    pub int_snapshot_num: i32,
    pub msi_mac_vec: i32,
    pub msi_wol_vec: i32,
    pub msi_sfty_ce_vec: i32,
    pub msi_sfty_ue_vec: i32,
    pub msi_rx_base_vec: i32,
    pub msi_tx_base_vec: i32,
    pub dwmac4_addrs: *const dwmac4_addrs,
    pub flags: u32,
    pub __dma_cfg: stmmac_dma_cfg,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
