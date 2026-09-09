/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/ssb/ssb_driver_gige.h.
// The CONFIG_SSB_DRIVER_GIGE build-time condition is preserved below.

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_PCIIO: u32 = 0x0000; // PCI I/O Registers (1024 bytes)
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_RESERVED: u32 = 0x0400; // Reserved (1024 bytes)
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_PCICFG: u32 = 0x0800; // PCI config space (256 bytes)
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_SHIM_FLUSHSTAT: u32 = 0x0C00; // PCI to OCP: Flush status control (32bit)
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_SHIM_FLUSHRDA: u32 = 0x0C04; // PCI to OCP: Flush read address (32bit)
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_SHIM_FLUSHTO: u32 = 0x0C08; // PCI to OCP: Flush timeout counter (32bit)
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_SHIM_BARRIER: u32 = 0x0C0C; // PCI to OCP: Barrier register (32bit)
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_SHIM_MAOCPSI: u32 = 0x0C10; // PCI to OCP: MaocpSI Control (32bit)
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_SHIM_SIOCPMA: u32 = 0x0C14; // PCI to OCP: SiocpMa Control (32bit)

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_TMSHIGH_RGMII: u32 = 0x0001_0000; // Have an RGMII PHY-bus
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_TMSLOW_TXBYPASS: u32 = 0x0008_0000; // TX bypass (no delay)
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_TMSLOW_RXBYPASS: u32 = 0x0010_0000; // RX bypass (no delay)
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_TMSLOW_DLLEN: u32 = 0x0100_0000; // Enable DLL controls
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_BFL_ROBOSWITCH: u32 = 0x0010;

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_MEM_RES_NAME: &str = "SSB Broadcom 47xx GigE memory";
#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub const SSB_GIGE_IO_RES_NAME: &str = "SSB Broadcom 47xx GigE I/O";

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
#[repr(C)]
pub struct ssb_gige {
    pub dev: *mut ssb_device,
    pub lock: spinlock_t,
    pub has_rgmii: bool,
    pub pci_controller: pci_controller,
    pub pci_ops: pci_ops,
    pub mem_resource: resource,
    pub io_resource: resource,
}

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
extern "C" {
    pub fn pdev_is_ssb_gige_core(pdev: *mut pci_dev) -> bool;
    pub fn ssb_gige_pcibios_plat_dev_init(sdev: *mut ssb_device, pdev: *mut pci_dev) -> i32;
    pub fn ssb_gige_map_irq(sdev: *mut ssb_device, pdev: *const pci_dev) -> i32;
    pub fn ssb_gige_init() -> i32;
}

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub unsafe fn pdev_to_ssb_gige(pdev: *mut pci_dev) -> *mut ssb_gige {
    if !pdev_is_ssb_gige_core(pdev) {
        return core::ptr::null_mut();
    }
    container_of!((*(*pdev).bus).ops, ssb_gige, pci_ops)
}

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub unsafe fn ssb_gige_is_rgmii(pdev: *mut pci_dev) -> bool {
    let dev = pdev_to_ssb_gige(pdev);
    if !dev.is_null() { (*dev).has_rgmii } else { false }
}

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub unsafe fn ssb_gige_have_roboswitch(pdev: *mut pci_dev) -> bool {
    let dev = pdev_to_ssb_gige(pdev);
    !dev.is_null() && ((*(*dev).dev).bus.sprom.boardflags_lo & SSB_GIGE_BFL_ROBOSWITCH) != 0
}

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub unsafe fn ssb_gige_one_dma_at_once(pdev: *mut pci_dev) -> bool {
    let dev = pdev_to_ssb_gige(pdev);
    !dev.is_null() && (*(*dev).dev).bus.chip_id == 0x4785 && (*(*dev).dev).bus.chip_rev < 2
}

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub unsafe fn ssb_gige_must_flush_posted_writes(pdev: *mut pci_dev) -> bool {
    let dev = pdev_to_ssb_gige(pdev);
    !dev.is_null() && (*(*dev).dev).bus.chip_id == 0x4785
}

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub unsafe fn ssb_gige_get_macaddr(pdev: *mut pci_dev, macaddr: *mut u8) -> i32 {
    let dev = pdev_to_ssb_gige(pdev);
    if dev.is_null() { return -ENODEV; }
    core::ptr::copy_nonoverlapping((*(*dev).dev).bus.sprom.et0mac.as_ptr(), macaddr, 6);
    0
}

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub unsafe fn ssb_gige_get_phyaddr(pdev: *mut pci_dev) -> i32 {
    let dev = pdev_to_ssb_gige(pdev);
    if dev.is_null() { return -ENODEV; }
    (*(*dev).dev).bus.sprom.et0phyaddr as i32
}

#[cfg(CONFIG_SSB_DRIVER_GIGE)]
pub unsafe fn ssb_gige_exit() {
    // Currently we can not unregister the GigE driver, because we can not unregister the PCI bridge.
    BUG!();
}

#[cfg(not(CONFIG_SSB_DRIVER_GIGE))]
pub unsafe fn ssb_gige_pcibios_plat_dev_init(_: *mut ssb_device, _: *mut pci_dev) -> i32 { -ENOSYS }
#[cfg(not(CONFIG_SSB_DRIVER_GIGE))]
pub unsafe fn ssb_gige_map_irq(_: *mut ssb_device, _: *const pci_dev) -> i32 { -ENOSYS }
#[cfg(not(CONFIG_SSB_DRIVER_GIGE))]
pub unsafe fn ssb_gige_init() -> i32 { 0 }
#[cfg(not(CONFIG_SSB_DRIVER_GIGE))]
pub unsafe fn ssb_gige_exit() {}
#[cfg(not(CONFIG_SSB_DRIVER_GIGE))]
pub unsafe fn pdev_is_ssb_gige_core(_: *mut pci_dev) -> bool { false }
#[cfg(not(CONFIG_SSB_DRIVER_GIGE))]
pub unsafe fn pdev_to_ssb_gige(_: *mut pci_dev) -> *mut ssb_gige { core::ptr::null_mut() }
#[cfg(not(CONFIG_SSB_DRIVER_GIGE))]
pub unsafe fn ssb_gige_is_rgmii(_: *mut pci_dev) -> bool { false }
#[cfg(not(CONFIG_SSB_DRIVER_GIGE))]
pub unsafe fn ssb_gige_have_roboswitch(_: *mut pci_dev) -> bool { false }
#[cfg(not(CONFIG_SSB_DRIVER_GIGE))]
pub unsafe fn ssb_gige_one_dma_at_once(_: *mut pci_dev) -> bool { false }
#[cfg(not(CONFIG_SSB_DRIVER_GIGE))]
pub unsafe fn ssb_gige_must_flush_posted_writes(_: *mut pci_dev) -> bool { false }
#[cfg(not(CONFIG_SSB_DRIVER_GIGE))]
pub unsafe fn ssb_gige_get_macaddr(_: *mut pci_dev, _: *mut u8) -> i32 { -ENODEV }
#[cfg(not(CONFIG_SSB_DRIVER_GIGE))]
pub unsafe fn ssb_gige_get_phyaddr(_: *mut pci_dev) -> i32 { -ENODEV }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
