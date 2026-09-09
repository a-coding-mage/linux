/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations originate from the Linux kernel environment.

/* System Clock Control Register */
pub const MPC83XX_SCCR_OFFS: u32 = 0xA08;
pub const MPC83XX_SCCR_USB_MASK: u32 = 0x00f00000;
pub const MPC83XX_SCCR_USB_MPHCM_11: u32 = 0x00c00000;
pub const MPC83XX_SCCR_USB_MPHCM_01: u32 = 0x00400000;
pub const MPC83XX_SCCR_USB_MPHCM_10: u32 = 0x00800000;
pub const MPC83XX_SCCR_USB_DRCM_11: u32 = 0x00300000;
pub const MPC83XX_SCCR_USB_DRCM_01: u32 = 0x00100000;
pub const MPC83XX_SCCR_USB_DRCM_10: u32 = 0x00200000;
pub const MPC8315_SCCR_USB_MASK: u32 = 0x00c00000;
pub const MPC8315_SCCR_USB_DRCM_11: u32 = 0x00c00000;
pub const MPC8315_SCCR_USB_DRCM_01: u32 = 0x00400000;
pub const MPC837X_SCCR_USB_DRCM_11: u32 = 0x00c00000;

/* system i/o configuration register low */
pub const MPC83XX_SICRL_OFFS: u32 = 0x114;
pub const MPC834X_SICRL_USB_MASK: u32 = 0x60000000;
pub const MPC834X_SICRL_USB0: u32 = 0x20000000;
pub const MPC834X_SICRL_USB1: u32 = 0x40000000;
pub const MPC831X_SICRL_USB_MASK: u32 = 0x00000c00;
pub const MPC831X_SICRL_USB_ULPI: u32 = 0x00000800;
pub const MPC8315_SICRL_USB_MASK: u32 = 0x000000fc;
pub const MPC8315_SICRL_USB_ULPI: u32 = 0x00000054;
pub const MPC837X_SICRL_USB_MASK: u32 = 0xf0000000;
pub const MPC837X_SICRL_USB_ULPI: u32 = 0x50000000;
pub const MPC837X_SICRL_USBB_MASK: u32 = 0x30000000;
pub const MPC837X_SICRL_SD: u32 = 0x20000000;

/* system i/o configuration register high */
pub const MPC83XX_SICRH_OFFS: u32 = 0x118;
pub const MPC8308_SICRH_USB_MASK: u32 = 0x000c0000;
pub const MPC8308_SICRH_USB_ULPI: u32 = 0x00040000;
pub const MPC834X_SICRH_USB_UTMI: u32 = 0x00020000;
pub const MPC831X_SICRH_USB_MASK: u32 = 0x000000e0;
pub const MPC831X_SICRH_USB_ULPI: u32 = 0x000000a0;
pub const MPC8315_SICRH_USB_MASK: u32 = 0x0000ff00;
pub const MPC8315_SICRH_USB_ULPI: u32 = 0x00000000;
pub const MPC837X_SICRH_SPI_MASK: u32 = 0x00000003;
pub const MPC837X_SICRH_SD: u32 = 0x00000001;

/* USB Control Register */
pub const FSL_USB2_CONTROL_OFFS: u32 = 0x500;
pub const CONTROL_UTMI_PHY_EN: u32 = 0x00000200;
pub const CONTROL_REFSEL_24MHZ: u32 = 0x00000040;
pub const CONTROL_REFSEL_48MHZ: u32 = 0x00000080;
pub const CONTROL_PHY_CLK_SEL_ULPI: u32 = 0x00000400;
pub const CONTROL_OTG_PORT: u32 = 0x00000020;

/* USB PORTSC Registers */
pub const FSL_USB2_PORTSC1_OFFS: u32 = 0x184;
pub const FSL_USB2_PORTSC2_OFFS: u32 = 0x188;
pub const PORTSCX_PTW_16BIT: u32 = 0x10000000;
pub const PORTSCX_PTS_UTMI: u32 = 0x00000000;
pub const PORTSCX_PTS_ULPI: u32 = 0x80000000;

/*
 * Declaration for the various functions exported by the
 * mpc83xx_* files. Mostly for use by mpc83xx_setup
 */
extern "C" {
    pub fn mpc83xx_restart(cmd: *mut core::ffi::c_char) -> !;
    pub fn mpc83xx_time_init() -> isize;
    pub fn mpc837x_usb_cfg() -> i32;
    pub fn mpc834x_usb_cfg() -> i32;
    pub fn mpc831x_usb_cfg() -> i32;
    pub fn mpc83xx_ipic_init_IRQ();

    // When CONFIG_PCI is enabled, this is an external function; otherwise it is NULL.
    #[cfg(CONFIG_PCI)]
    pub fn mpc83xx_setup_pci();

    #[cfg(not(CONFIG_PCI))]
    pub const mpc83xx_setup_pci: Option<unsafe extern "C" fn()> = None;

    pub fn mpc83xx_declare_of_platform_devices() -> i32;
    pub fn mpc83xx_setup_arch();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
