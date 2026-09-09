/* SPDX-License-Identifier: GPL-2.0 */

/* Forward declarations supplied by other translation units. */
#[repr(C)]
pub struct mv643xx_eth_platform_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mv_sata_platform_data {
    _private: [u8; 0],
}

pub const ORION_MBUS_PCIE_MEM_TARGET: u32 = 0x04;
pub const ORION_MBUS_PCIE_MEM_ATTR: u32 = 0x59;
pub const ORION_MBUS_PCIE_IO_TARGET: u32 = 0x04;
pub const ORION_MBUS_PCIE_IO_ATTR: u32 = 0x51;
pub const ORION_MBUS_PCIE_WA_TARGET: u32 = 0x04;
pub const ORION_MBUS_PCIE_WA_ATTR: u32 = 0x79;
pub const ORION_MBUS_PCI_MEM_TARGET: u32 = 0x03;
pub const ORION_MBUS_PCI_MEM_ATTR: u32 = 0x59;
pub const ORION_MBUS_PCI_IO_TARGET: u32 = 0x03;
pub const ORION_MBUS_PCI_IO_ATTR: u32 = 0x51;
pub const ORION_MBUS_DEVBUS_BOOT_TARGET: u32 = 0x01;
pub const ORION_MBUS_DEVBUS_BOOT_ATTR: u32 = 0x0f;
#[inline]
pub const fn ORION_MBUS_DEVBUS_TARGET(_cs: u32) -> u32 { 0x01 }
#[inline]
pub const fn ORION_MBUS_DEVBUS_ATTR(cs: u32) -> u32 { !(1u32 << cs) }
pub const ORION_MBUS_SRAM_TARGET: u32 = 0x09;
pub const ORION_MBUS_SRAM_ATTR: u32 = 0x00;

/* Basic Orion init functions used early by machine-setup. */
extern "C" {
    pub fn orion5x_map_io();
    pub fn orion5x_init_early();
    pub fn orion5x_init_irq();
    pub fn orion5x_init();
    pub fn orion5x_id(dev: *mut u32, rev: *mut u32, dev_name: *mut *mut core::ffi::c_char);
    pub fn clk_init();
    pub static mut orion5x_tclk: i32;
    pub fn orion5x_timer_init();
    pub fn orion5x_setup_wins();
    pub fn orion5x_ehci0_init();
    pub fn orion5x_ehci1_init();
    pub fn orion5x_eth_init(eth_data: *mut mv643xx_eth_platform_data);
    pub fn orion5x_i2c_init();
    pub fn orion5x_sata_init(sata_data: *mut mv_sata_platform_data);
    pub fn orion5x_spi_init();
    pub fn orion5x_uart0_init();
    pub fn orion5x_uart1_init();
    pub fn orion5x_xor_init();
    pub fn orion5x_restart(mode: reboot_mode, cmd: *const core::ffi::c_char);
}

/* PCIe/PCI functions. */
#[repr(C)] pub struct pci_bus { _private: [u8; 0] }
#[repr(C)] pub struct pci_host_bridge { _private: [u8; 0] }
#[repr(C)] pub struct pci_sys_data { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct tag { _private: [u8; 0] }
pub type reboot_mode = u32;

extern "C" {
    pub fn orion5x_pcie_id(dev: *mut u32, rev: *mut u32);
    pub fn orion5x_pci_disable();
    pub fn orion5x_pci_set_cardbus_mode();
    pub fn orion5x_pci_sys_setup(nr: i32, sys: *mut pci_sys_data) -> i32;
    pub fn orion5x_pci_sys_scan_bus(nr: i32, bridge: *mut pci_host_bridge) -> i32;
    pub fn orion5x_pci_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> i32;
    pub fn tag_fixup_mem32(tag: *mut tag, cmdline: *mut *mut core::ffi::c_char);
}

/* Build-time configuration selects the external implementation. */
#[cfg(CONFIG_MACH_MSS2_DT)]
extern "C" { pub fn mss2_init(); }
#[cfg(not(CONFIG_MACH_MSS2_DT))]
#[inline] pub fn mss2_init() {}

#[cfg(CONFIG_MACH_D2NET_DT)]
extern "C" { pub fn d2net_init(); }
#[cfg(not(CONFIG_MACH_D2NET_DT))]
#[inline] pub fn d2net_init() {}

/* These are not preempt-safe. Locks, if needed, must be taken by the caller. */
extern "C" {
    pub fn readl(addr: *const core::ffi::c_void) -> u32;
    pub fn writel(value: u32, addr: *mut core::ffi::c_void);
}
#[inline]
pub unsafe fn orion5x_setbits(r: *mut core::ffi::c_void, mask: u32) {
    writel(readl(r) | mask, r);
}
#[inline]
pub unsafe fn orion5x_clrbits(r: *mut core::ffi::c_void, mask: u32) {
    writel(readl(r) & !mask, r);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
