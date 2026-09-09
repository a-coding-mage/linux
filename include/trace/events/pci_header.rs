/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of trace/events/pci.h. */

/* The C preprocessor tracepoint include files are represented here as
 * declarations and data descriptions; their framework implementation is an
 * external dependency. */

use core::ffi::c_char;

/* PCI hotplug event values are supplied by the PCI tracepoint definitions. */
pub const PCI_HOTPLUG_LINK_UP: i32 = 0;
pub const PCI_HOTPLUG_LINK_DOWN: i32 = 1;
pub const PCI_HOTPLUG_CARD_PRESENT: i32 = 2;
pub const PCI_HOTPLUG_CARD_NOT_PRESENT: i32 = 3;

pub const PCI_HOTPLUG_EVENT: &[(&str, i32)] = &[
    ("LINK_UP", PCI_HOTPLUG_LINK_UP),
    ("LINK_DOWN", PCI_HOTPLUG_LINK_DOWN),
    ("CARD_PRESENT", PCI_HOTPLUG_CARD_PRESENT),
    ("CARD_NOT_PRESENT", PCI_HOTPLUG_CARD_NOT_PRESENT),
];

/* These constants are defined by <uapi/linux/pci_regs.h>. */
unsafe extern "C" {
    pub static PCI_EXP_LNKSTA_LBMS: u32;
    pub static PCI_EXP_LNKSTA_LABS: u32;
    pub static PCI_EXP_LNKSTA_LT: u32;
    pub static PCI_EXP_LNKSTA_DLLLA: u32;
}

pub const PCI_EXP_LNKSTA_LINK_STATUS_MASK: u32 =
    PCI_EXP_LNKSTA_LBMS | PCI_EXP_LNKSTA_LABS | PCI_EXP_LNKSTA_LT | PCI_EXP_LNKSTA_DLLLA;

pub const LNKSTA_FLAGS: &[(&str, u32)] = &[
    ("LT", PCI_EXP_LNKSTA_LT),
    ("DLLLA", PCI_EXP_LNKSTA_DLLLA),
    ("LBMS", PCI_EXP_LNKSTA_LBMS),
    ("LABS", PCI_EXP_LNKSTA_LABS),
];

#[repr(C)]
pub struct PciBus {
    pub self_: *mut core::ffi::c_void,
    pub cur_bus_speed: u32,
    pub max_bus_speed: u32,
    pub flit_mode: u32,
}

unsafe extern "C" {
    pub fn pci_name(dev: *const core::ffi::c_void) -> *const c_char;
    pub fn pci_pcie_type(dev: *const core::ffi::c_void) -> u32;
}

#[repr(C)]
pub struct PciHpEventEntry {
    pub port_name: *const c_char,
    pub slot: *const c_char,
    pub event: i32,
}

#[repr(C)]
pub struct PcieLinkEventEntry {
    pub port_name: *const c_char,
    pub r#type: u32,
    pub reason: u32,
    pub cur_bus_speed: u32,
    pub max_bus_speed: u32,
    pub width: u32,
    pub flit_mode: u32,
    pub link_status: u32,
}

/* TRACE_EVENT(pci_hp_event) */
#[inline]
pub unsafe fn pci_hp_event_entry(
    port_name: *const c_char,
    slot: *const c_char,
    event: i32,
) -> PciHpEventEntry {
    PciHpEventEntry { port_name, slot, event }
}

/* TRACE_EVENT(pcie_link_event) */
#[inline]
pub unsafe fn pcie_link_event_entry(
    bus: *mut PciBus,
    reason: u32,
    width: u32,
    status: u32,
) -> PcieLinkEventEntry {
    PcieLinkEventEntry {
        port_name: pci_name((*bus).self_),
        r#type: pci_pcie_type((*bus).self_),
        reason,
        cur_bus_speed: (*bus).cur_bus_speed,
        max_bus_speed: (*bus).max_bus_speed,
        width,
        flit_mode: (*bus).flit_mode,
        link_status: status,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
