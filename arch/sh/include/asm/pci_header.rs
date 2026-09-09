/* SPDX-License-Identifier: GPL-2.0 */

/* Can be used to override the logic in pci_scan_bus for skipping
 * already-configured bus numbers - to be used for buggy BIOSes
 * or architectures with incomplete PCI setup by the loader.
 */
pub const PCIBIOS_ASSIGN_ALL_BUSSES: i32 = 1;

/* A board can define one or more PCI channels that represent built-in (or
 * external) PCI controllers. */
#[repr(C)]
pub struct pci_channel {
    pub next: *mut pci_channel,
    pub bus: *mut pci_bus,

    pub pci_ops: *mut pci_ops,

    pub resources: *mut resource,
    pub nr_resources: ::core::ffi::c_uint,

    pub io_offset: ::core::ffi::c_ulong,
    pub mem_offset: ::core::ffi::c_ulong,

    pub reg_base: ::core::ffi::c_ulong,
    pub io_map_base: ::core::ffi::c_ulong,

    pub index: ::core::ffi::c_uint,
    pub need_domain_info: ::core::ffi::c_uint,

    /* Optional error handling */
    pub err_timer: timer_list,
    pub serr_timer: timer_list,
    pub err_irq: ::core::ffi::c_uint,
    pub serr_irq: ::core::ffi::c_uint,
}

extern "C" {
    /* arch/sh/drivers/pci/pci.c */
    pub static mut pci_config_lock: raw_spinlock_t;

    pub fn register_pci_controller(hose: *mut pci_channel) -> ::core::ffi::c_int;
    pub fn pcibios_report_status(status_mask: ::core::ffi::c_uint, warn: ::core::ffi::c_int);

    /* arch/sh/drivers/pci/common.c */
    pub fn early_read_config_byte(
        hose: *mut pci_channel,
        top_bus: ::core::ffi::c_int,
        bus: ::core::ffi::c_int,
        devfn: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        value: *mut u8,
    ) -> ::core::ffi::c_int;
    pub fn early_read_config_word(
        hose: *mut pci_channel,
        top_bus: ::core::ffi::c_int,
        bus: ::core::ffi::c_int,
        devfn: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        value: *mut u16,
    ) -> ::core::ffi::c_int;
    pub fn early_read_config_dword(
        hose: *mut pci_channel,
        top_bus: ::core::ffi::c_int,
        bus: ::core::ffi::c_int,
        devfn: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        value: *mut u32,
    ) -> ::core::ffi::c_int;
    pub fn early_write_config_byte(
        hose: *mut pci_channel,
        top_bus: ::core::ffi::c_int,
        bus: ::core::ffi::c_int,
        devfn: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        value: u8,
    ) -> ::core::ffi::c_int;
    pub fn early_write_config_word(
        hose: *mut pci_channel,
        top_bus: ::core::ffi::c_int,
        bus: ::core::ffi::c_int,
        devfn: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        value: u16,
    ) -> ::core::ffi::c_int;
    pub fn early_write_config_dword(
        hose: *mut pci_channel,
        top_bus: ::core::ffi::c_int,
        bus: ::core::ffi::c_int,
        devfn: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
        value: u32,
    ) -> ::core::ffi::c_int;
    pub fn pcibios_enable_timers(hose: *mut pci_channel);
    pub fn pcibios_handle_status_errors(
        addr: ::core::ffi::c_ulong,
        status: ::core::ffi::c_uint,
        hose: *mut pci_channel,
    ) -> ::core::ffi::c_uint;
    pub fn pci_is_66mhz_capable(
        hose: *mut pci_channel,
        top_bus: ::core::ffi::c_int,
        current_bus: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub static mut PCIBIOS_MIN_IO: ::core::ffi::c_ulong;
    pub static mut PCIBIOS_MIN_MEM: ::core::ffi::c_ulong;

    /* Board-specific fixup routines. */
    pub fn pcibios_map_platform_irq(
        dev: *const pci_dev,
        slot: u8,
        pin: u8,
    ) -> ::core::ffi::c_int;
}

/* These configuration macros are enabled by the architecture. */
pub const HAVE_PCI_MMAP: bool = true;
pub const ARCH_GENERIC_PCI_MMAP_RESOURCE: bool = true;

/* CONFIG_PCI: None of the SH PCI controllers support MWI; it is always
 * treated as a direct memory write. */
// #define PCI_DISABLE_MWI when CONFIG_PCI is enabled.

#[inline]
pub unsafe fn pci_domain_nr(bus: *mut pci_bus) -> ::core::ffi::c_uint {
    (*(*(bus)).sysdata.cast::<pci_channel>()).index
}

#[inline]
pub unsafe fn pci_proc_domain(bus: *mut pci_bus) -> ::core::ffi::c_uint {
    let hose = (*(bus)).sysdata.cast::<pci_channel>();
    (*hose).need_domain_info
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
