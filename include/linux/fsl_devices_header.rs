/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Rust translation of include/linux/fsl_devices.h.
 */

pub const FSL_UTMI_PHY_DLY: i32 = 10; // Delay for UTMI PHY clock to become stable (10ms).
pub const FSL_USB_PHY_CLK_TIMEOUT: i32 = 10000; // uSec

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fsl_usb2_controller_ver {
    FSL_USB_VER_NONE = -1,
    FSL_USB_VER_OLD = 0,
    FSL_USB_VER_1_6 = 1,
    FSL_USB_VER_2_2 = 2,
    FSL_USB_VER_2_4 = 3,
    FSL_USB_VER_2_5 = 4,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fsl_usb2_operating_modes {
    FSL_USB2_MPH_HOST,
    FSL_USB2_DR_HOST,
    FSL_USB2_DR_DEVICE,
    FSL_USB2_DR_OTG,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fsl_usb2_phy_modes {
    FSL_USB2_PHY_NONE,
    FSL_USB2_PHY_ULPI,
    FSL_USB2_PHY_UTMI,
    FSL_USB2_PHY_UTMI_WIDE,
    FSL_USB2_PHY_SERIAL,
    FSL_USB2_PHY_UTMI_DUAL,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fsl_usb2_platform_data {
    pub controller_ver: fsl_usb2_controller_ver,
    pub operating_mode: fsl_usb2_operating_modes,
    pub phy_mode: fsl_usb2_phy_modes,
    pub port_enables: u32,
    pub workaround: u32,
    pub init: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub exit: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub regs: *mut core::ffi::c_void,
    pub clk: *mut clk,
    pub power_budget: u32,
    // C bit-fields are represented as individual storage words; nonzero means set.
    pub big_endian_mmio: u32,
    pub big_endian_desc: u32,
    pub es: u32,
    pub le_setup_buf: u32,
    pub have_sysif_regs: u32,
    pub invert_drvvbus: u32,
    pub invert_pwr_fault: u32,
    pub suspended: u32,
    pub already_suspended: u32,
    pub has_fsl_erratum_a007792: u32,
    pub has_fsl_erratum_14: u32,
    pub has_fsl_erratum_a005275: u32,
    pub has_fsl_erratum_a005697: u32,
    pub has_fsl_erratum_a006918: u32,
    pub check_phy_clk_valid: u32,
    pub pm_command: u32,
    pub pm_status: u32,
    pub pm_intr_enable: u32,
    pub pm_frame_index: u32,
    pub pm_segment: u32,
    pub pm_frame_list: u32,
    pub pm_async_next: u32,
    pub pm_configured_flag: u32,
    pub pm_portsc: u32,
    pub pm_usbgenctrl: u32,
}

pub const FSL_USB2_PORT0_ENABLED: u32 = 0x00000001;
pub const FSL_USB2_PORT1_ENABLED: u32 = 0x00000002;

#[repr(C)]
pub struct spi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fsl_spi_platform_data {
    pub initial_spmode: u32,
    pub bus_num: i16,
    pub flags: u32,
    pub max_chipselect: u16,
    pub cs_control: Option<unsafe extern "C" fn(*mut spi_device, bool)>,
    pub sysclk: u32,
}

pub const SPI_QE_CPU_MODE: u32 = 1 << 0;
pub const SPI_CPM_MODE: u32 = 1 << 1;
pub const SPI_CPM1: u32 = 1 << 2;
pub const SPI_CPM2: u32 = 1 << 3;
pub const SPI_QE: u32 = 1 << 4;

#[repr(C)]
pub struct mpc8xx_pcmcia_ops {
    pub hw_ctrl: Option<unsafe extern "C" fn(slot: i32, enable: i32)>,
    pub voltage_set: Option<unsafe extern "C" fn(slot: i32, vcc: i32, vpp: i32) -> i32>,
}

// With CONFIG_PPC_83xx and CONFIG_SUSPEND, this is an external function.
#[cfg(all(CONFIG_PPC_83xx, CONFIG_SUSPEND))]
unsafe extern "C" {
    pub fn fsl_deep_sleep() -> i32;
}

// Otherwise the C header provides this inline implementation.
#[cfg(not(all(CONFIG_PPC_83xx, CONFIG_SUSPEND)))]
#[inline]
pub const fn fsl_deep_sleep() -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
