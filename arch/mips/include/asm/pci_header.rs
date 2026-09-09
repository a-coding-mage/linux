/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// C header guard: _ASM_PCI_H
// Includes: linux/mm.h, linux/ioport.h, linux/list.h, linux/of.h,
// linux/types.h, linux/slab.h, linux/scatterlist.h, linux/string.h, asm/io.h
// The declarations below refer to types and functions supplied by those
// dependencies.

// The following declarations are kernel-only in the C source (__KERNEL__).

#[cfg(feature = "CONFIG_PCI_DRIVERS_LEGACY")]
#[repr(C)]
pub struct pci_controller {
    pub list: list_head,
    pub bus: *mut pci_bus,
    pub of_node: *mut device_node,

    pub pci_ops: *mut pci_ops,
    pub mem_resource: *mut resource,
    pub mem_offset: c_ulong,
    pub io_resource: *mut resource,
    pub io_offset: c_ulong,
    pub io_map_base: c_ulong,

    #[cfg(not(feature = "CONFIG_PCI_DOMAINS_GENERIC"))]
    pub index: c_uint,
    #[cfg(not(feature = "CONFIG_PCI_DOMAINS_GENERIC"))]
    pub need_domain_info: c_uint,

    pub get_busno: Option<unsafe extern "C" fn() -> c_int>,
    pub set_busno: Option<unsafe extern "C" fn(busno: c_int)>,
}

#[cfg(feature = "CONFIG_PCI_DRIVERS_LEGACY")]
extern "C" {
    pub fn register_pci_controller(hose: *mut pci_controller);
    pub fn pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> c_int;
    pub fn pcibios_plat_dev_init(dev: *mut pci_dev) -> c_int;
    pub static mut pcibios_plat_setup:
        Option<unsafe extern "C" fn(str_: *mut c_char) -> *mut c_char>;
}

#[cfg(all(feature = "CONFIG_PCI_DRIVERS_LEGACY", feature = "CONFIG_OF"))]
extern "C" {
    pub fn pci_load_of_ranges(hose: *mut pci_controller, node: *mut device_node);
}

#[cfg(all(feature = "CONFIG_PCI_DRIVERS_LEGACY", not(feature = "CONFIG_OF")))]
#[inline]
pub unsafe fn pci_load_of_ranges(_hose: *mut pci_controller, _node: *mut device_node) {}

#[cfg(all(feature = "CONFIG_PCI_DRIVERS_LEGACY", feature = "CONFIG_PCI_DOMAINS_GENERIC"))]
#[inline]
pub unsafe fn set_pci_need_domain_info(
    _hose: *mut pci_controller,
    _need_domain_info: c_int,
) {
    // nothing to do
}

#[cfg(all(
    feature = "CONFIG_PCI_DRIVERS_LEGACY",
    not(feature = "CONFIG_PCI_DOMAINS_GENERIC"),
    feature = "CONFIG_PCI_DOMAINS"
))]
#[inline]
pub unsafe fn set_pci_need_domain_info(hose: *mut pci_controller, need_domain_info: c_int) {
    (*hose).need_domain_info = need_domain_info as c_uint;
}

#[inline]
pub const fn pcibios_assign_all_busses() -> c_uint {
    1
}

extern "C" {
    pub static mut PCIBIOS_MIN_IO: c_ulong;
    pub static mut PCIBIOS_MIN_MEM: c_ulong;
}

pub const PCIBIOS_MIN_CARDBUS_IO: c_ulong = 0x4000;

// #define HAVE_PCI_MMAP
// #define ARCH_GENERIC_PCI_MMAP_RESOURCE

// Dynamic DMA mapping stuff. MIPS has everything mapped statically.

#[cfg(feature = "CONFIG_PCI_DOMAINS_GENERIC")]
#[inline]
pub unsafe fn pci_proc_domain(bus: *mut pci_bus) -> c_int {
    pci_domain_nr(bus)
}

#[cfg(feature = "CONFIG_PCI_DOMAINS")]
#[inline]
pub unsafe fn pci_proc_domain(bus: *mut pci_bus) -> c_int {
    let hose = (*bus).sysdata as *mut pci_controller;
    (*hose).need_domain_info as c_int
}

// CONFIG_PCI_DOMAINS: #define pci_domain_nr(bus) \
//     ((struct pci_controller *)(bus)->sysdata)->index

// Do platform specific device initialization at pci_enable_device() time.
extern "C" {
    pub fn pcibios_plat_dev_init(dev: *mut pci_dev) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
