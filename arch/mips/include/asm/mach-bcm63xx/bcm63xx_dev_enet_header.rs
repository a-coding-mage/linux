/* SPDX-License-Identifier: GPL-2.0 */
// Translated from bcm63xx_dev_enet.h.
// The Linux headers and bcm63xx_regs.h provide the referenced external types
// and constants.

/*
 * on board ethernet platform data
 */
#[repr(C)]
pub struct bcm63xx_enet_platform_data {
    pub mac_addr: [kernel::ffi::c_char; ETH_ALEN],

    pub has_phy: kernel::ffi::c_int,

    /* if has_phy, then set use_internal_phy */
    pub use_internal_phy: kernel::ffi::c_int,

    /* or fill phy info to use an external one */
    pub phy_id: kernel::ffi::c_int,
    pub has_phy_interrupt: kernel::ffi::c_int,
    pub phy_interrupt: kernel::ffi::c_int,

    /* if has_phy, use autonegotiated pause parameters or force
     * them */
    pub pause_auto: kernel::ffi::c_int,
    pub pause_rx: kernel::ffi::c_int,
    pub pause_tx: kernel::ffi::c_int,

    /* if !has_phy, set desired forced speed/duplex */
    pub force_speed_100: kernel::ffi::c_int,
    pub force_duplex_full: kernel::ffi::c_int,

    /* if !has_phy, set callback to perform mii device
     * init/remove */
    pub mii_config: Option<unsafe extern "C" fn(
        dev: *mut net_device,
        probe: kernel::ffi::c_int,
        mii_read: Option<unsafe extern "C" fn(
            dev: *mut net_device,
            phy_id: kernel::ffi::c_int,
            reg: kernel::ffi::c_int,
        ) -> kernel::ffi::c_int>,
        mii_write: Option<unsafe extern "C" fn(
            dev: *mut net_device,
            phy_id: kernel::ffi::c_int,
            reg: kernel::ffi::c_int,
            val: kernel::ffi::c_int,
        )>,
    ) -> kernel::ffi::c_int>,

    /* DMA channel enable mask */
    pub dma_chan_en_mask: u32,

    /* DMA channel interrupt mask */
    pub dma_chan_int_mask: u32,

    /* DMA engine has internal SRAM */
    pub dma_has_sram: bool,

    /* DMA channel register width */
    pub dma_chan_width: kernel::ffi::c_uint,

    /* DMA descriptor shift */
    pub dma_desc_shift: kernel::ffi::c_uint,

    /* dma channel ids */
    pub rx_chan: kernel::ffi::c_int,
    pub tx_chan: kernel::ffi::c_int,
}

/*
 * on board ethernet switch platform data
 */
pub const ENETSW_MAX_PORT: usize = 8;
pub const ENETSW_PORTS_6328: kernel::ffi::c_int = 5; /* 4 FE PHY + 1 RGMII */
pub const ENETSW_PORTS_6368: kernel::ffi::c_int = 6; /* 4 FE PHY + 2 RGMII */

pub const ENETSW_RGMII_PORT0: kernel::ffi::c_int = 4;

#[repr(C)]
pub struct bcm63xx_enetsw_port {
    pub used: kernel::ffi::c_int,
    pub phy_id: kernel::ffi::c_int,

    pub bypass_link: kernel::ffi::c_int,
    pub force_speed: kernel::ffi::c_int,
    pub force_duplex_full: kernel::ffi::c_int,

    pub name: *const kernel::ffi::c_char,
}

#[repr(C)]
pub struct bcm63xx_enetsw_platform_data {
    pub mac_addr: [kernel::ffi::c_char; ETH_ALEN],
    pub num_ports: kernel::ffi::c_int,
    pub used_ports: [bcm63xx_enetsw_port; ENETSW_MAX_PORT],

    /* DMA channel enable mask */
    pub dma_chan_en_mask: u32,

    /* DMA channel interrupt mask */
    pub dma_chan_int_mask: u32,

    /* DMA channel register width */
    pub dma_chan_width: kernel::ffi::c_uint,

    /* DMA engine has internal SRAM */
    pub dma_has_sram: bool,
}

pub struct net_device;

unsafe extern "C" {
    pub fn bcm63xx_enet_register(
        unit: kernel::ffi::c_int,
        pd: *const bcm63xx_enet_platform_data,
    ) -> kernel::ffi::c_int;

    pub fn bcm63xx_enetsw_register(
        pd: *const bcm63xx_enetsw_platform_data,
    ) -> kernel::ffi::c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bcm63xx_regs_enetdmac {
    ENETDMAC_CHANCFG,
    ENETDMAC_IR,
    ENETDMAC_IRMASK,
    ENETDMAC_MAXBURST,
    ENETDMAC_BUFALLOC,
    ENETDMAC_RSTART,
    ENETDMAC_FC,
    ENETDMAC_LEN,
}

unsafe extern "C" {
    pub static bcm63xx_regs_enetdmac: *const kernel::ffi::c_ulong;
}

#[inline]
pub unsafe fn bcm63xx_enetdmacreg(reg: bcm63xx_regs_enetdmac) -> kernel::ffi::c_ulong {
    *bcm63xx_regs_enetdmac.add(reg as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
