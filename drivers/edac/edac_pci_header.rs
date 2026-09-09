/*
 * Defines, structures, APIs for edac_pci and edac_pci_sysfs
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

/* C dependencies are supplied by the surrounding kernel translation unit. */

#[cfg(feature = "CONFIG_PCI")]
#[repr(C)]
pub struct edac_pci_counter {
    pub pe_count: atomic_t,
    pub npe_count: atomic_t,
}

/*
 * Abstract edac_pci control info structure
 */
#[cfg(feature = "CONFIG_PCI")]
#[repr(C)]
pub struct edac_pci_ctl_info {
    /* for global list of edac_pci_ctl_info structs */
    pub link: list_head,
    pub pci_idx: core::ffi::c_int,
    /* the internal state of this controller instance */
    pub op_state: core::ffi::c_int,
    /* work struct for this instance */
    pub work: delayed_work,
    /* pointer to edac polling checking routine */
    pub edac_check: Option<unsafe extern "C" fn(*mut edac_pci_ctl_info)>,
    pub dev: *mut device,
    pub mod_name: *const core::ffi::c_char,
    pub ctl_name: *const core::ffi::c_char,
    pub dev_name: *const core::ffi::c_char,
    pub pvt_info: *mut core::ffi::c_void,
    pub start_time: core::ffi::c_ulong,
    pub name: [core::ffi::c_char; (EDAC_DEVICE_NAME_LEN + 1) as usize],
    pub counters: edac_pci_counter,
    pub kobj: kobject,
}

#[cfg(feature = "CONFIG_PCI")]
#[macro_export]
macro_rules! to_edac_pci_ctl_work {
    ($w:expr) => { container_of!($w, edac_pci_ctl_info, work) };
}

/* write all or some bits in a byte-register */
#[cfg(feature = "CONFIG_PCI")]
#[inline]
pub unsafe fn pci_write_bits8(pdev: *mut pci_dev, offset: core::ffi::c_int,
                              mut value: u8, mask: u8) {
    if mask != 0xff {
        let mut buf: u8 = 0;
        pci_read_config_byte(pdev, offset, &mut buf);
        value &= mask;
        buf &= !mask;
        value |= buf;
    }
    pci_write_config_byte(pdev, offset, value);
}

/* write all or some bits in a word-register */
#[cfg(feature = "CONFIG_PCI")]
#[inline]
pub unsafe fn pci_write_bits16(pdev: *mut pci_dev, offset: core::ffi::c_int,
                               mut value: u16, mask: u16) {
    if mask != 0xffff {
        let mut buf: u16 = 0;
        pci_read_config_word(pdev, offset, &mut buf);
        value &= mask;
        buf &= !mask;
        value |= buf;
    }
    pci_write_config_word(pdev, offset, value);
}

/* write all or some bits in a dword-register */
#[cfg(feature = "CONFIG_PCI")]
#[inline]
pub unsafe fn pci_write_bits32(pdev: *mut pci_dev, offset: core::ffi::c_int,
                               mut value: u32, mask: u32) {
    if mask != 0xffff_ffff {
        let mut buf: u32 = 0;
        pci_read_config_dword(pdev, offset, &mut buf);
        value &= mask;
        buf &= !mask;
        value |= buf;
    }
    pci_write_config_dword(pdev, offset, value);
}

extern "C" {
    pub fn edac_pci_alloc_ctl_info(sz_pvt: core::ffi::c_uint,
                                   edac_pci_name: *const core::ffi::c_char)
        -> *mut edac_pci_ctl_info;
    pub fn edac_pci_free_ctl_info(pci: *mut edac_pci_ctl_info);
    pub fn edac_pci_alloc_index() -> core::ffi::c_int;
    pub fn edac_pci_add_device(pci: *mut edac_pci_ctl_info,
                               edac_idx: core::ffi::c_int) -> core::ffi::c_int;
    pub fn edac_pci_del_device(dev: *mut device) -> *mut edac_pci_ctl_info;
    pub fn edac_pci_create_generic_ctl(dev: *mut device,
                                       mod_name: *const core::ffi::c_char)
        -> *mut edac_pci_ctl_info;
    pub fn edac_pci_release_generic_ctl(pci: *mut edac_pci_ctl_info);
    pub fn edac_pci_create_sysfs(pci: *mut edac_pci_ctl_info) -> core::ffi::c_int;
    pub fn edac_pci_remove_sysfs(pci: *mut edac_pci_ctl_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
