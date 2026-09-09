/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// linux/scatterlist.h

/*
** HP PCI platforms generally support multiple bus adapters.
**    (workstations 1-~4, servers 2-~32)
**
** Newer platforms number the busses across PCI bus adapters *sparsely*.
** E.g. 0, 8, 16, ...
**
** Under a PCI bus, most HP platforms support PPBs up to two or three
** levels deep. See "Bit3" product line.
*/
pub const PCI_MAX_BUSSES: usize = 256;

/* To be used as: mdelay(pci_post_reset_delay); */
pub const pci_post_reset_delay: u32 = 50;

/* Dependency-supplied types: parisc_device, pci_bus, resource, and ioc. */
#[repr(C)]
pub struct pci_hba_data {
    pub base_addr: *mut core::ffi::c_void,
    pub dev: *const parisc_device,
    pub hba_bus: *mut pci_bus,
    pub hba_num: core::ffi::c_int,
    pub bus_num: resource,
    pub io_space: resource,
    pub lmmio_space: resource,
    pub elmmio_space: resource,
    pub gmmio_space: resource,
    pub lmmio_space_offset: core::ffi::c_ulong,
    pub iommu: *mut ioc,
    pub io_name: [core::ffi::c_char; HBA_NAME_SIZE],
    pub lmmio_name: [core::ffi::c_char; HBA_NAME_SIZE],
    pub elmmio_name: [core::ffi::c_char; HBA_NAME_SIZE],
    pub gmmio_name: [core::ffi::c_char; HBA_NAME_SIZE],
}

pub const DINO_MAX_LMMIO_RESOURCES: usize = 3;
pub const HBA_NAME_SIZE: usize = 16;

pub const HBA_PORT_SPACE_BITS: u32 = 16;

#[inline]
pub const fn HBA_PORT_BASE(h: usize) -> usize {
    h << HBA_PORT_SPACE_BITS
}

#[inline]
pub const fn HBA_PORT_SPACE_SIZE() -> usize {
    1usize << HBA_PORT_SPACE_BITS
}

#[inline]
pub const fn PCI_PORT_HBA(a: usize) -> usize {
    a >> HBA_PORT_SPACE_BITS
}

#[inline]
pub const fn PCI_PORT_ADDR(a: usize) -> usize {
    a & (HBA_PORT_SPACE_SIZE() - 1)
}

// CONFIG_64BIT controls this build-time value in the original header.
#[cfg(target_pointer_width = "64")]
pub const PCI_F_EXTEND: usize = 0xffffffff00000000usize;
#[cfg(not(target_pointer_width = "64"))]
pub const PCI_F_EXTEND: usize = 0usize;

#[repr(C)]
pub struct pci_port_ops {
    pub inb: Option<unsafe extern "C" fn(hba: *mut pci_hba_data, port: u16) -> u8>,
    pub inw: Option<unsafe extern "C" fn(hba: *mut pci_hba_data, port: u16) -> u16>,
    pub inl: Option<unsafe extern "C" fn(hba: *mut pci_hba_data, port: u16) -> u32>,
    pub outb: Option<unsafe extern "C" fn(hba: *mut pci_hba_data, port: u16, data: u8)>,
    pub outw: Option<unsafe extern "C" fn(hba: *mut pci_hba_data, port: u16, data: u16)>,
    pub outl: Option<unsafe extern "C" fn(hba: *mut pci_hba_data, port: u16, data: u32)>,
}

#[repr(C)]
pub struct pci_bios_ops {
    pub init: Option<unsafe extern "C" fn()>,
    pub fixup_bus: Option<unsafe extern "C" fn(bus: *mut pci_bus)>,
}

extern "C" {
    pub static mut pci_port: *mut pci_port_ops;
    pub static mut pci_bios: *mut pci_bios_ops;
}

#[cfg(feature = "CONFIG_PCI")]
extern "C" {
    pub fn pcibios_register_hba(hba: *mut pci_hba_data);
}

#[cfg(not(feature = "CONFIG_PCI"))]
#[inline]
pub unsafe fn pcibios_register_hba(_x: *mut pci_hba_data) {}

extern "C" {
    pub fn pcibios_init_bridge(dev: *mut pci_dev);
}

pub const fn pcibios_assign_all_busses() -> i32 {
    1
}

pub const PCIBIOS_MIN_IO: u32 = 0x10;
pub const PCIBIOS_MIN_MEM: u32 = 0x1000;

// HAVE_PCI_MMAP
// ARCH_GENERIC_PCI_MMAP_RESOURCE

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
