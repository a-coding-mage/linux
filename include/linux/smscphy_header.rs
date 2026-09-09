/* SPDX-License-Identifier: GPL-2.0 */

// Declarations use types supplied by the surrounding kernel translation.

pub const MII_LAN83C185_ISF: i32 = 29; // Interrupt Source Flags
pub const MII_LAN83C185_IM: i32 = 30; // Interrupt Mask
pub const MII_LAN83C185_CTRL_STATUS: i32 = 17; // Mode/Status Register
pub const MII_LAN83C185_SPECIAL_MODES: i32 = 18; // Special Modes Register

pub const MII_LAN83C185_ISF_INT1: i32 = 1 << 1; // Auto-Negotiation Page Received
pub const MII_LAN83C185_ISF_INT2: i32 = 1 << 2; // Parallel Detection Fault
pub const MII_LAN83C185_ISF_INT3: i32 = 1 << 3; // Auto-Negotiation LP Ack
pub const MII_LAN83C185_ISF_INT4: i32 = 1 << 4; // Link Down
pub const MII_LAN83C185_ISF_INT5: i32 = 1 << 5; // Remote Fault Detected
pub const MII_LAN83C185_ISF_INT6: i32 = 1 << 6; // Auto-Negotiation complete
pub const MII_LAN83C185_ISF_INT7: i32 = 1 << 7; // ENERGYON

pub const MII_LAN83C185_ISF_INT_ALL: i32 = 0x0e;
pub const MII_LAN83C185_ISF_INT_PHYLIB_EVENTS: i32 =
    MII_LAN83C185_ISF_INT6 | MII_LAN83C185_ISF_INT4 | MII_LAN83C185_ISF_INT7;

pub const MII_LAN83C185_EDPWRDOWN: i32 = 1 << 13; // EDPWRDOWN
pub const MII_LAN83C185_ENERGYON: i32 = 1 << 1; // ENERGYON

pub const MII_LAN83C185_MODE_MASK: i32 = 0xE0;
pub const MII_LAN83C185_MODE_POWERDOWN: i32 = 0xC0; // Power Down mode
pub const MII_LAN83C185_MODE_ALL: i32 = 0xE0; // All capable mode

unsafe extern "C" {
    pub fn smsc_phy_config_intr(phydev: *mut phy_device) -> i32;
    pub fn smsc_phy_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t;
    pub fn smsc_phy_config_init(phydev: *mut phy_device) -> i32;
    pub fn lan87xx_read_status(phydev: *mut phy_device) -> i32;
    pub fn smsc_phy_get_tunable(
        phydev: *mut phy_device,
        tuna: *mut ethtool_tunable,
        data: *mut core::ffi::c_void,
    ) -> i32;
    pub fn smsc_phy_set_tunable(
        phydev: *mut phy_device,
        tuna: *mut ethtool_tunable,
        data: *const core::ffi::c_void,
    ) -> i32;
    pub fn smsc_phy_probe(phydev: *mut phy_device) -> i32;
}

pub const MII_LAN874X_PHY_MMD_WOL_WUCSR: i32 = 0x8010;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_CFGA: i32 = 0x8011;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_CFGB: i32 = 0x8012;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK0: i32 = 0x8021;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK1: i32 = 0x8022;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK2: i32 = 0x8023;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK3: i32 = 0x8024;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK4: i32 = 0x8025;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK5: i32 = 0x8026;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK6: i32 = 0x8027;
pub const MII_LAN874X_PHY_MMD_WOL_WUF_MASK7: i32 = 0x8028;
pub const MII_LAN874X_PHY_MMD_WOL_RX_ADDRA: i32 = 0x8061;
pub const MII_LAN874X_PHY_MMD_WOL_RX_ADDRB: i32 = 0x8062;
pub const MII_LAN874X_PHY_MMD_WOL_RX_ADDRC: i32 = 0x8063;
pub const MII_LAN874X_PHY_MMD_MCFGR: i32 = 0x8064;

pub const MII_LAN874X_PHY_PME1_SET: i32 = 2 << 13;
pub const MII_LAN874X_PHY_PME2_SET: i32 = 2 << 11;
pub const MII_LAN874X_PHY_PME_SELF_CLEAR: i32 = 1 << 9;
pub const MII_LAN874X_PHY_WOL_PFDA_FR: i32 = 1 << 7;
pub const MII_LAN874X_PHY_WOL_WUFR: i32 = 1 << 6;
pub const MII_LAN874X_PHY_WOL_MPR: i32 = 1 << 5;
pub const MII_LAN874X_PHY_WOL_BCAST_FR: i32 = 1 << 4;
pub const MII_LAN874X_PHY_WOL_PFDAEN: i32 = 1 << 3;
pub const MII_LAN874X_PHY_WOL_WUEN: i32 = 1 << 2;
pub const MII_LAN874X_PHY_WOL_MPEN: i32 = 1 << 1;
pub const MII_LAN874X_PHY_WOL_BCSTEN: i32 = 1 << 0;

pub const MII_LAN874X_PHY_WOL_FILTER_EN: i32 = 1 << 15;
pub const MII_LAN874X_PHY_WOL_FILTER_MCASTTEN: i32 = 1 << 9;
pub const MII_LAN874X_PHY_WOL_FILTER_BCSTEN: i32 = 1 << 8;

pub const MII_LAN874X_PHY_PME_SELF_CLEAR_DELAY: i32 = 0x1000; // 81 milliseconds


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
