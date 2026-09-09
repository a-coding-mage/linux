// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of clk-mt7988-topckgen.c.  Kernel dependencies and
 * the macro data constructors are supplied by the surrounding crate. */

use core::ffi::c_char;

static mut MT7988_CLK_LOCK: SpinLock = DEFINE_SPINLOCK!();

static TOP_FIXED_CLKS: [MtkFixedClk; 1] = [
    FIXED_CLK!(CLK_TOP_XTAL, "top_xtal", "clkxtal", 40000000),
];

static TOP_DIVS: [MtkFixedFactor; 28] = [
    FACTOR!(CLK_TOP_XTAL_D2, "top_xtal_d2", "top_xtal", 1, 2),
    FACTOR!(CLK_TOP_RTC_32K, "top_rtc_32k", "top_xtal", 1, 1250),
    FACTOR!(CLK_TOP_RTC_32P7K, "top_rtc_32p7k", "top_xtal", 1, 1220),
    FACTOR!(CLK_TOP_MPLL_D2, "mpll_d2", "mpll", 1, 2),
    FACTOR!(CLK_TOP_MPLL_D3_D2, "mpll_d3_d2", "mpll", 1, 2),
    FACTOR!(CLK_TOP_MPLL_D4, "mpll_d4", "mpll", 1, 4),
    FACTOR!(CLK_TOP_MPLL_D8, "mpll_d8", "mpll", 1, 8),
    FACTOR!(CLK_TOP_MPLL_D8_D2, "mpll_d8_d2", "mpll", 1, 16),
    FACTOR!(CLK_TOP_MMPLL_D2, "mmpll_d2", "mmpll", 1, 2),
    FACTOR!(CLK_TOP_MMPLL_D3_D5, "mmpll_d3_d5", "mmpll", 1, 15),
    FACTOR!(CLK_TOP_MMPLL_D4, "mmpll_d4", "mmpll", 1, 4),
    FACTOR!(CLK_TOP_MMPLL_D6_D2, "mmpll_d6_d2", "mmpll", 1, 12),
    FACTOR!(CLK_TOP_MMPLL_D8, "mmpll_d8", "mmpll", 1, 8),
    FACTOR!(CLK_TOP_APLL2_D4, "apll2_d4", "apll2", 1, 4),
    FACTOR!(CLK_TOP_NET1PLL_D4, "net1pll_d4", "net1pll", 1, 4),
    FACTOR!(CLK_TOP_NET1PLL_D5, "net1pll_d5", "net1pll", 1, 5),
    FACTOR!(CLK_TOP_NET1PLL_D5_D2, "net1pll_d5_d2", "net1pll", 1, 10),
    FACTOR!(CLK_TOP_NET1PLL_D5_D4, "net1pll_d5_d4", "net1pll", 1, 20),
    FACTOR!(CLK_TOP_NET1PLL_D8, "net1pll_d8", "net1pll", 1, 8),
    FACTOR!(CLK_TOP_NET1PLL_D8_D2, "net1pll_d8_d2", "net1pll", 1, 16),
    FACTOR!(CLK_TOP_NET1PLL_D8_D4, "net1pll_d8_d4", "net1pll", 1, 32),
    FACTOR!(CLK_TOP_NET1PLL_D8_D8, "net1pll_d8_d8", "net1pll", 1, 64),
    FACTOR!(CLK_TOP_NET1PLL_D8_D16, "net1pll_d8_d16", "net1pll", 1, 128),
    FACTOR!(CLK_TOP_NET2PLL_D2, "net2pll_d2", "net2pll", 1, 2),
    FACTOR!(CLK_TOP_NET2PLL_D4, "net2pll_d4", "net2pll", 1, 4),
    FACTOR!(CLK_TOP_NET2PLL_D4_D4, "net2pll_d4_d4", "net2pll", 1, 16),
    FACTOR!(CLK_TOP_NET2PLL_D4_D8, "net2pll_d4_d8", "net2pll", 1, 32),
    FACTOR!(CLK_TOP_NET2PLL_D6, "net2pll_d6", "net2pll", 1, 6),
    FACTOR!(CLK_TOP_NET2PLL_D8, "net2pll_d8", "net2pll", 1, 8),
];

macro_rules! parents { ($($x:literal),* $(,)?) => { &[$($x as *const c_char),*] }; }
static NETSYS_PARENTS: &[&str] = &["top_xtal", "net2pll_d2", "mmpll_d2"];
static NETSYS_500M_PARENTS: &[&str] = &["top_xtal", "net1pll_d5", "net1pll_d5_d2"];
static NETSYS_2X_PARENTS: &[&str] = &["top_xtal", "net2pll", "mmpll"];
static NETSYS_GSW_PARENTS: &[&str] = &["top_xtal", "net1pll_d4", "net1pll_d5"];
static ETH_GMII_PARENTS: &[&str] = &["top_xtal", "net1pll_d5_d4"];
static NETSYS_MCU_PARENTS: &[&str] = &["top_xtal", "net2pll", "mmpll", "net1pll_d4", "net1pll_d5", "mpll"];
static EIP197_PARENTS: &[&str] = &["top_xtal", "netsyspll", "net2pll", "mmpll", "net1pll_d4", "net1pll_d5"];
static AXI_INFRA_PARENTS: &[&str] = &["top_xtal", "net1pll_d8_d2"];
static UART_PARENTS: &[&str] = &["top_xtal", "mpll_d8", "mpll_d8_d2"];
static EMMC_250M_PARENTS: &[&str] = &["top_xtal", "net1pll_d5_d2", "mmpll_d4"];
static EMMC_400M_PARENTS: &[&str] = &["top_xtal", "msdcpll", "mmpll_d2", "mpll_d2", "mmpll_d4", "net1pll_d8_d2"];
static SPI_PARENTS: &[&str] = &["top_xtal", "mpll_d2", "mmpll_d4", "net1pll_d8_d2", "net2pll_d6", "net1pll_d5_d4", "mpll_d4", "net1pll_d8_d4"];
static NFI1X_PARENTS: &[&str] = &["top_xtal", "mmpll_d4", "net1pll_d8_d2", "net2pll_d6", "mpll_d4", "mmpll_d8", "net1pll_d8_d4", "mpll_d8"];
static SSPXTP_PARENTS: &[&str] = &["top_xtal_d2", "top_xtal", "net1pll_d5_d4", "mpll_d4", "mmpll_d8", "net1pll_d8_d4", "mmpll_d6_d2", "mpll_d8"];
static PWM_PARENTS: &[&str] = &["top_xtal", "net1pll_d8_d2", "net1pll_d5_d4", "mpll_d4", "mpll_d8_d2", "top_rtc_32k"];
static I2C_PARENTS: &[&str] = &["top_xtal", "net1pll_d5_d4", "mpll_d4", "net1pll_d8_d4"];
static PCIE_MBIST_250M_PARENTS: &[&str] = &["top_xtal", "net1pll_d5_d2"];
static PEXTP_TL_CK_PARENTS: &[&str] = &["top_xtal", "net2pll_d6", "mmpll_d8", "mpll_d8_d2", "top_rtc_32k"];
static USB_FRMCNT_PARENTS: &[&str] = &["top_xtal", "mmpll_d3_d5"];
static AUD_PARENTS: &[&str] = &["top_xtal", "apll2"];
static A1SYS_PARENTS: &[&str] = &["top_xtal", "apll2_d4"];
static AUD_L_PARENTS: &[&str] = &["top_xtal", "apll2", "mpll_d8_d2"];
static SSPXTP2_PARENTS: &[&str] = &["top_xtal_d2", "mpll_d8_d2"];
static USXGMII_SBUS_0_PARENTS: &[&str] = &["top_xtal", "net1pll_d8_d4"];
static SGM_0_PARENTS: &[&str] = &["top_xtal", "sgmpll"];
static SYSAPB_PARENTS: &[&str] = &["top_xtal", "mpll_d3_d2"];
static ETH_REFCK_50M_PARENTS: &[&str] = &["top_xtal", "net2pll_d4_d4"];
static ETH_SYS_200M_PARENTS: &[&str] = &["top_xtal", "net2pll_d4"];
static ETH_XGMII_PARENTS: &[&str] = &["top_xtal_d2", "net1pll_d8_d8", "net1pll_d8_d16"];
static BUS_TOPS_PARENTS: &[&str] = &["top_xtal", "net1pll_d5", "net2pll_d2"];
static NPU_TOPS_PARENTS: &[&str] = &["top_xtal", "net2pll"];
static DRAMC_MD32_PARENTS: &[&str] = &["top_xtal", "mpll_d2", "wedmcupll"];
static DA_XTP_GLB_P0_PARENTS: &[&str] = &["top_xtal", "net2pll_d8"];
static MCUSYS_BACKUP_625M_PARENTS: &[&str] = &["top_xtal", "net1pll_d4"];
static MACSEC_PARENTS: &[&str] = &["top_xtal", "sgmpll", "net1pll_d8"];
static NETSYS_TOPS_400M_PARENTS: &[&str] = &["top_xtal", "net2pll_d2"];
static ETH_MII_PARENTS: &[&str] = &["top_xtal_d2", "net2pll_d4_d8"];

// The following table is intentionally expressed with the crate's direct
// equivalents of the original C initializer macros.
static TOP_MUXES: &[MtkMux] = &[
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_SEL, "netsys_sel", NETSYS_PARENTS, 0x000,0x004,0x008,0,2,7,0x1c0,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_500M_SEL,"netsys_500m_sel",NETSYS_500M_PARENTS,0,4,8,2,15,0x1c0,1),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_2X_SEL,"netsys_2x_sel",NETSYS_2X_PARENTS,0,4,16,2,23,0x1c0,2),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_GSW_SEL,"netsys_gsw_sel",NETSYS_GSW_PARENTS,0,4,24,2,31,0x1c0,3),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_ETH_GMII_SEL,"eth_gmii_sel",ETH_GMII_PARENTS,0x10,0x14,0x18,0,1,7,0x1c0,4),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_MCU_SEL,"netsys_mcu_sel",NETSYS_MCU_PARENTS,0x10,0x14,0x18,8,3,15,0x1c0,5),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_PAO_2X_SEL,"netsys_pao_2x_sel",NETSYS_MCU_PARENTS,0x10,0x14,0x18,16,3,23,0x1c0,6),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_EIP197_SEL,"eip197_sel",EIP197_PARENTS,0x10,0x14,0x18,24,3,31,0x1c0,7),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_AXI_INFRA_SEL,"axi_infra_sel",AXI_INFRA_PARENTS,0x20,0x24,0x28,0,1,7,0x1c0,8,CLK_IS_CRITICAL),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_UART_SEL,"uart_sel",UART_PARENTS,0x20,0x24,0x28,8,2,15,0x1c0,9),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_EMMC_250M_SEL,"emmc_250m_sel",EMMC_250M_PARENTS,0x20,0x24,0x28,16,2,23,0x1c0,10),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_EMMC_400M_SEL,"emmc_400m_sel",EMMC_400M_PARENTS,0x20,0x24,0x28,24,3,31,0x1c0,11),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SPI_SEL,"spi_sel",SPI_PARENTS,0x30,0x34,0x38,0,3,7,0x1c0,12),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SPIM_MST_SEL,"spim_mst_sel",SPI_PARENTS,0x30,0x34,0x38,8,3,15,0x1c0,13),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NFI1X_SEL,"nfi1x_sel",NFI1X_PARENTS,0x30,0x34,0x38,16,3,23,0x1c0,14),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SPINFI_SEL,"spinfi_sel",SSPXTP_PARENTS,0x30,0x34,0x38,24,3,31,0x1c0,15),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PWM_SEL,"pwm_sel",PWM_PARENTS,0x40,0x44,0x48,0,3,7,0x1c0,16),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_I2C_SEL,"i2c_sel",I2C_PARENTS,0x40,0x44,0x48,8,2,15,0x1c0,17),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PCIE_MBIST_250M_SEL,"pcie_mbist_250m_sel",PCIE_MBIST_250M_PARENTS,0x40,0x44,0x48,16,1,23,0x1c0,18),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PEXTP_TL_SEL,"pextp_tl_sel",PEXTP_TL_CK_PARENTS,0x40,0x44,0x48,24,3,31,0x1c0,19),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PEXTP_TL_P1_SEL,"pextp_tl_p1_sel",PEXTP_TL_CK_PARENTS,0x50,0x54,0x58,0,3,7,0x1c0,20),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PEXTP_TL_P2_SEL,"pextp_tl_p2_sel",PEXTP_TL_CK_PARENTS,0x50,0x54,0x58,8,3,15,0x1c0,21),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PEXTP_TL_P3_SEL,"pextp_tl_p3_sel",PEXTP_TL_CK_PARENTS,0x50,0x54,0x58,16,3,23,0x1c0,22),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_USB_SYS_SEL,"usb_sys_sel",ETH_GMII_PARENTS,0x50,0x54,0x58,24,1,31,0x1c0,23),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_USB_SYS_P1_SEL,"usb_sys_p1_sel",ETH_GMII_PARENTS,0x60,0x64,0x68,0,1,7,0x1c0,24),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_USB_XHCI_SEL,"usb_xhci_sel",ETH_GMII_PARENTS,0x60,0x64,0x68,8,1,15,0x1c0,25),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_USB_XHCI_P1_SEL,"usb_xhci_p1_sel",ETH_GMII_PARENTS,0x60,0x64,0x68,16,1,23,0x1c0,26),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_USB_FRMCNT_SEL,"usb_frmcnt_sel",USB_FRMCNT_PARENTS,0x60,0x64,0x68,24,1,31,0x1c0,27),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_USB_FRMCNT_P1_SEL,"usb_frmcnt_p1_sel",USB_FRMCNT_PARENTS,0x70,0x74,0x78,0,1,7,0x1c0,28),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUD_SEL,"aud_sel",AUD_PARENTS,0x70,0x74,0x78,8,1,15,0x1c0,29),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_A1SYS_SEL,"a1sys_sel",A1SYS_PARENTS,0x70,0x74,0x78,16,1,23,0x1c0,30),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_AUD_L_SEL,"aud_l_sel",AUD_L_PARENTS,0x70,0x74,0x78,24,2,31,0x1c4,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_A_TUNER_SEL,"a_tuner_sel",A1SYS_PARENTS,0x80,0x84,0x88,0,1,7,0x1c4,1),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SSPXTP_SEL,"sspxtp_sel",SSPXTP2_PARENTS,0x80,0x84,0x88,8,1,15,0x1c4,2),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_USB_PHY_SEL,"usb_phy_sel",SSPXTP2_PARENTS,0x80,0x84,0x88,16,1,23,0x1c4,3),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_USXGMII_SBUS_0_SEL,"usxgmii_sbus_0_sel",USXGMII_SBUS_0_PARENTS,0x80,0x84,0x88,24,1,31,0x1c4,4),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_USXGMII_SBUS_1_SEL,"usxgmii_sbus_1_sel",USXGMII_SBUS_0_PARENTS,0x90,0x94,0x98,0,1,7,0x1c4,5),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SGM_0_SEL,"sgm_0_sel",SGM_0_PARENTS,0x90,0x94,0x98,8,1,15,0x1c4,6),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_SGM_SBUS_0_SEL,"sgm_sbus_0_sel",USXGMII_SBUS_0_PARENTS,0x90,0x94,0x98,16,1,23,0x1c4,7,CLK_IS_CRITICAL),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_SGM_1_SEL,"sgm_1_sel",SGM_0_PARENTS,0x90,0x94,0x98,24,1,31,0x1c4,8),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_SGM_SBUS_1_SEL,"sgm_sbus_1_sel",USXGMII_SBUS_0_PARENTS,0xa0,0xa4,0xa8,0,1,7,0x1c4,9,CLK_IS_CRITICAL),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_XFI_PHY_0_XTAL_SEL,"xfi_phy_0_xtal_sel",SSPXTP2_PARENTS,0xa0,0xa4,0xa8,8,1,15,0x1c4,10),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_XFI_PHY_1_XTAL_SEL,"xfi_phy_1_xtal_sel",SSPXTP2_PARENTS,0xa0,0xa4,0xa8,16,1,23,0x1c4,11),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_SYSAXI_SEL,"sysaxi_sel",AXI_INFRA_PARENTS,0xa0,0xa4,0xa8,24,1,31,0x1c4,12,CLK_IS_CRITICAL),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_SYSAPB_SEL,"sysapb_sel",SYSAPB_PARENTS,0xb0,0xb4,0xb8,0,1,7,0x1c4,13,CLK_IS_CRITICAL),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_ETH_REFCK_50M_SEL,"eth_refck_50m_sel",ETH_REFCK_50M_PARENTS,0xb0,0xb4,0xb8,8,1,15,0x1c4,14),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_ETH_SYS_200M_SEL,"eth_sys_200m_sel",ETH_SYS_200M_PARENTS,0xb0,0xb4,0xb8,16,1,23,0x1c4,15),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_ETH_SYS_SEL,"eth_sys_sel",PCIE_MBIST_250M_PARENTS,0xb0,0xb4,0xb8,24,1,31,0x1c4,16),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_ETH_XGMII_SEL,"eth_xgmii_sel",ETH_XGMII_PARENTS,0xc0,0xc4,0xc8,0,2,7,0x1c4,17),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_BUS_TOPS_SEL,"bus_tops_sel",BUS_TOPS_PARENTS,0xc0,0xc4,0xc8,8,2,15,0x1c4,18),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NPU_TOPS_SEL,"npu_tops_sel",NPU_TOPS_PARENTS,0xc0,0xc4,0xc8,16,1,23,0x1c4,19),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_DRAMC_SEL,"dramc_sel",SSPXTP2_PARENTS,0xc0,0xc4,0xc8,24,1,31,0x1c4,20,CLK_IS_CRITICAL),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_DRAMC_MD32_SEL,"dramc_md32_sel",DRAMC_MD32_PARENTS,0xd0,0xd4,0xd8,0,2,7,0x1c4,21,CLK_IS_CRITICAL),
    MUX_GATE_CLR_SET_UPD_FLAGS!(CLK_TOP_INFRA_F26M_SEL,"csw_infra_f26m_sel",SSPXTP2_PARENTS,0xd0,0xd4,0xd8,8,1,15,0x1c4,22,CLK_IS_CRITICAL),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PEXTP_P0_SEL,"pextp_p0_sel",SSPXTP2_PARENTS,0xd0,0xd4,0xd8,16,1,23,0x1c4,23),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PEXTP_P1_SEL,"pextp_p1_sel",SSPXTP2_PARENTS,0xd0,0xd4,0xd8,24,1,31,0x1c4,24),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PEXTP_P2_SEL,"pextp_p2_sel",SSPXTP2_PARENTS,0xe0,0xe4,0xe8,0,1,7,0x1c4,25),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PEXTP_P3_SEL,"pextp_p3_sel",SSPXTP2_PARENTS,0xe0,0xe4,0xe8,8,1,15,0x1c4,26),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_DA_XTP_GLB_P0_SEL,"da_xtp_glb_p0_sel",DA_XTP_GLB_P0_PARENTS,0xe0,0xe4,0xe8,16,1,23,0x1c4,27),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_DA_XTP_GLB_P1_SEL,"da_xtp_glb_p1_sel",DA_XTP_GLB_P0_PARENTS,0xe0,0xe4,0xe8,24,1,31,0x1c4,28),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_DA_XTP_GLB_P2_SEL,"da_xtp_glb_p2_sel",DA_XTP_GLB_P0_PARENTS,0xf0,0xf4,0xf8,0,1,7,0x1c4,29),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_DA_XTP_GLB_P3_SEL,"da_xtp_glb_p3_sel",DA_XTP_GLB_P0_PARENTS,0xf0,0xf4,0xf8,8,1,15,0x1c4,30),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_CKM_SEL,"ckm_sel",SSPXTP2_PARENTS,0xf0,0xf4,0xf8,16,1,23,0x1c8,0),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_DA_SEL,"da_sel",SSPXTP2_PARENTS,0xf0,0xf4,0xf8,24,1,31,0x1c8,1),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_PEXTP_SEL,"pextp_sel",SSPXTP2_PARENTS,0x100,0x104,0x108,0,1,7,0x1c8,2),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_TOPS_P2_26M_SEL,"tops_p2_26m_sel",SSPXTP2_PARENTS,0x100,0x104,0x108,8,1,15,0x1c8,3),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_MCUSYS_BACKUP_625M_SEL,"mcusys_backup_625m_sel",MCUSYS_BACKUP_625M_PARENTS,0x100,0x104,0x108,16,1,23,0x1c8,4),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_SYNC_250M_SEL,"netsys_sync_250m_sel",PCIE_MBIST_250M_PARENTS,0x100,0x104,0x108,24,1,31,0x1c8,5),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_MACSEC_SEL,"macsec_sel",MACSEC_PARENTS,0x110,0x114,0x118,0,2,7,0x1c8,6),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_TOPS_400M_SEL,"netsys_tops_400m_sel",NETSYS_TOPS_400M_PARENTS,0x110,0x114,0x118,8,1,15,0x1c8,7),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_PPEFB_250M_SEL,"netsys_ppefb_250m_sel",PCIE_MBIST_250M_PARENTS,0x110,0x114,0x118,16,1,23,0x1c8,8),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NETSYS_WARP_SEL,"netsys_warp_sel",NETSYS_PARENTS,0x110,0x114,0x118,24,2,31,0x1c8,9),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_ETH_MII_SEL,"eth_mii_sel",ETH_MII_PARENTS,0x120,0x124,0x128,0,1,7,0x1c8,10),
    MUX_GATE_CLR_SET_UPD!(CLK_TOP_NPU_SEL,"ck_npu_sel",NETSYS_2X_PARENTS,0x120,0x124,0x128,8,2,15,0x1c8,11),
];

static TOP_AUD_DIVS: [MtkComposite; 1] = [DIV_GATE!(CLK_TOP_AUD_I2S_M, "aud_i2s_m", "aud_sel", 0x0420, 0, 0x0420, 8, 8)];
static TOPCK_DESC: MtkClkDesc = MtkClkDesc { fixed_clks: TOP_FIXED_CLKS.as_ptr(), num_fixed_clks: TOP_FIXED_CLKS.len(), factor_clks: TOP_DIVS.as_ptr(), num_factor_clks: TOP_DIVS.len(), mux_clks: TOP_MUXES.as_ptr(), num_mux_clks: TOP_MUXES.len(), composite_clks: TOP_AUD_DIVS.as_ptr(), num_composite_clks: TOP_AUD_DIVS.len(), clk_lock: unsafe { &raw mut MT7988_CLK_LOCK } };

static MCU_BUS_DIV_PARENTS: &[&str] = &["top_xtal", "ccipll2_b", "net1pll_d4"];
static MCU_ARM_DIV_PARENTS: &[&str] = &["top_xtal", "arm_b", "net1pll_d4"];
static MCU_MUXES: [MtkComposite; 2] = [
    MUX_GATE_FLAGS!(CLK_MCU_BUS_DIV_SEL, "mcu_bus_div_sel", MCU_BUS_DIV_PARENTS, 0x7c0, 9, 2, -1, CLK_IS_CRITICAL),
    MUX_GATE_FLAGS!(CLK_MCU_ARM_DIV_SEL, "mcu_arm_div_sel", MCU_ARM_DIV_PARENTS, 0x7a8, 9, 2, -1, CLK_IS_CRITICAL),
];
static MCUSYS_DESC: MtkClkDesc = MtkClkDesc { composite_clks: MCU_MUXES.as_ptr(), num_composite_clks: MCU_MUXES.len(), ..MtkClkDesc::empty() };

#[no_mangle]
pub static OF_MATCH_CLK_MT7988_TOPCKGEN: &[OfDeviceId] = &[
    OfDeviceId { compatible: "mediatek,mt7988-topckgen", data: &TOPCK_DESC },
    OfDeviceId { compatible: "mediatek,mt7988-mcusys", data: &MCUSYS_DESC },
    OfDeviceId::sentinel(),
];

static mut CLK_MT7988_TOPCKGEN_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe), remove: Some(mtk_clk_simple_remove),
    driver: Driver { name: "clk-mt7988-topckgen", of_match_table: OF_MATCH_CLK_MT7988_TOPCKGEN.as_ptr() },
};

module_platform_driver!(CLK_MT7988_TOPCKGEN_DRV);
module_description!("MediaTek MT7988 top clock generators driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
