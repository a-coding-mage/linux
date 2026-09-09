/*
 * Defines, structures, APIs for edac_mc module
 *
 * (C) 2007 Linux Networx (http://lnxi.com)
 * This file may be distributed under the terms of the
 * GNU General Public License.
 *
 * Written by Thayne Harbaugh
 * Based on work by Dan Hollis <goemon at anime dot net> and others.
 *	http://www.anime.net/~goemon/linux-ecc/
 *
 * NMI handling support added by
 *     Dave Peterson <dsp@llnl.gov> <dave_peterson@pobox.com>
 *
 * Refactored for multi-source files:
 *	Doug Thompson <norsk5@xmission.com>
 *
 * Please look at Documentation/driver-api/edac.rst for more info about
 * EDAC core structs and functions.
 */

// C header dependencies are supplied by other translated units.

// The PAGE_SHIFT build-time condition is preserved from the C header.
#[cfg(any())]
pub const PAGE_SHIFT: usize = 0;

#[inline]
pub const fn pages_to_mib(pages: usize, page_shift: usize) -> usize {
    if page_shift < 20 {
        pages >> (20 - page_shift)
    } else {
        pages << (page_shift - 20)
    }
}

#[inline]
pub const fn mib_to_pages(mb: usize, page_shift: usize) -> usize {
    if page_shift < 20 {
        mb << (20 - page_shift)
    } else {
        mb >> (page_shift - 20)
    }
}

// C variadic printk macros; the underlying printk and dependent structures
// are supplied by other translated units.
#[macro_export]
macro_rules! edac_printk {
    ($level:expr, $prefix:expr, $fmt:expr $(, $arg:expr)*) => {
        printk!(concat!($level, "EDAC ", $prefix, ": ", $fmt) $(, $arg)*)
    };
}

#[macro_export]
macro_rules! edac_mc_printk {
    ($mci:expr, $level:expr, $fmt:expr $(, $arg:expr)*) => {
        printk!(concat!($level, "EDAC MC%d: ", $fmt), $mci.mc_idx $(, $arg)*)
    };
}

#[macro_export]
macro_rules! edac_mc_chipset_printk {
    ($mci:expr, $level:expr, $prefix:expr, $fmt:expr $(, $arg:expr)*) => {
        printk!(concat!($level, "EDAC ", $prefix, " MC%d: ", $fmt), $mci.mc_idx $(, $arg)*)
    };
}

#[macro_export]
macro_rules! edac_device_printk {
    ($ctl:expr, $level:expr, $fmt:expr $(, $arg:expr)*) => {
        printk!(concat!($level, "EDAC DEVICE%d: ", $fmt), $ctl.dev_idx $(, $arg)*)
    };
}

#[macro_export]
macro_rules! edac_pci_printk {
    ($ctl:expr, $level:expr, $fmt:expr $(, $arg:expr)*) => {
        printk!(concat!($level, "EDAC PCI%d: ", $fmt), $ctl.pci_idx $(, $arg)*)
    };
}

/* prefixes for edac_printk() and edac_mc_printk() */
pub const EDAC_MC: &str = "MC";
pub const EDAC_PCI: &str = "PCI";
pub const EDAC_DEBUG: &str = "DEBUG";

extern "C" {
    pub static edac_mem_types: *const *const std::os::raw::c_char;
    #[cfg(feature = "CONFIG_EDAC_DEBUG")]
    pub static mut edac_debug_level: std::os::raw::c_int;
}

// CONFIG_EDAC_DEBUG is a build-time condition from the original header.
#[macro_export]
macro_rules! edac_dbg {
    ($level:expr, $fmt:expr $(, $arg:expr)*) => {{
        #[cfg(feature = "CONFIG_EDAC_DEBUG")]
        {
            if $level <= unsafe { edac_debug_level } {
                edac_printk!(KERN_DEBUG, EDAC_DEBUG, concat!("%s: ", $fmt), module_path!() $(, $arg)*);
            }
        }
    }};
}

// PCI_VEND_DEV expands to the corresponding PCI_DEVICE vendor/device IDs.
#[macro_export]
macro_rules! PCI_VEND_DEV {
    ($vend:ident, $dev:ident) => { PCI_DEVICE!(PCI_VENDOR_ID_$vend, PCI_DEVICE_ID_$vend ## _ ## $dev) };
}

#[macro_export]
macro_rules! edac_dev_name {
    ($dev:expr) => { $dev.dev_name };
}

// Equivalent of container_of(k, struct mem_ctl_info, dev); requires the
// translated mem_ctl_info layout and is intentionally left as a raw-pointer
// operation at call sites.
#[macro_export]
macro_rules! to_mci {
    ($k:expr) => { container_of!($k, mem_ctl_info, dev) };
}

/**
 * edac_mc_alloc() - Allocate and partially fill a struct &mem_ctl_info.
 */
extern "C" {
    pub fn edac_mc_alloc(
        mc_num: u32,
        n_layers: u32,
        layers: *mut edac_mc_layer,
        sz_pvt: u32,
    ) -> *mut mem_ctl_info;

    pub fn edac_get_owner() -> *const std::os::raw::c_char;

    pub fn edac_mc_add_mc_with_groups(
        mci: *mut mem_ctl_info,
        groups: *const *const attribute_group,
    ) -> std::os::raw::c_int;

    pub fn edac_mc_free(mci: *mut mem_ctl_info);
    pub fn edac_has_mcs() -> bool;
    pub fn edac_mc_find(idx: std::os::raw::c_int) -> *mut mem_ctl_info;
    pub fn find_mci_by_dev(dev: *mut device) -> *mut mem_ctl_info;
    pub fn edac_mc_del_mc(dev: *mut device) -> *mut mem_ctl_info;
    pub fn edac_mc_find_csrow_by_page(mci: *mut mem_ctl_info, page: usize) -> std::os::raw::c_int;
    pub fn edac_raw_mc_handle_error(e: *mut edac_raw_error_desc);
    pub fn edac_mc_handle_error(
        type_: hw_event_mc_err_type,
        mci: *mut mem_ctl_info,
        error_count: u16,
        page_frame_number: usize,
        offset_in_page: usize,
        syndrome: usize,
        top_layer: std::os::raw::c_int,
        mid_layer: std::os::raw::c_int,
        low_layer: std::os::raw::c_int,
        msg: *const std::os::raw::c_char,
        other_detail: *const std::os::raw::c_char,
    );
    pub fn edac_op_state_to_string(op_state: std::os::raw::c_int) -> *mut std::os::raw::c_char;
}

// Declaration-only types supplied by the Linux EDAC dependencies.
pub enum edac_mc_layer {}
pub enum mem_ctl_info {}
pub enum attribute_group {}
pub enum device {}
pub enum edac_raw_error_desc {}
pub enum hw_event_mc_err_type {}

#[macro_export]
macro_rules! edac_mc_add_mc {
    ($mci:expr) => { edac_mc_add_mc_with_groups!($mci, core::ptr::null()) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
