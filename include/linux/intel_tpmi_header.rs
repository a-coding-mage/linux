/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * intel_tpmi.h: Intel TPMI core external interface
 */

// Dependency supplied by the Linux bitfield interface: FIELD_GET(GENMASK(...), val).

// Opaque types supplied by other translation units.
pub enum notifier_block {}
pub enum auxiliary_device {}
pub enum resource {}
pub enum dentry {}
pub enum oobmsm_plat_info {}

pub const TPMI_VERSION_INVALID: u8 = 0xff;

#[inline]
pub const fn TPMI_MINOR_VERSION(val: u8) -> u8 {
    val & 0x1f
}

#[inline]
pub const fn TPMI_MAJOR_VERSION(val: u8) -> u8 {
    (val >> 5) & 0x07
}

/*
 * List of supported TMPI IDs.
 * Some TMPI IDs are not used by Linux, so the numbers are not consecutive.
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_tpmi_id {
    TPMI_ID_RAPL = 0,       /* Running Average Power Limit */
    TPMI_ID_PEM = 1,        /* Power and Perf excursion Monitor */
    TPMI_ID_UNCORE = 2,     /* Uncore Frequency Scaling */
    TPMI_ID_SST = 5,        /* Speed Select Technology */
    TPMI_ID_PLR = 0xc,      /* Performance Limit Reasons */
    TPMI_CONTROL_ID = 0x80, /* Special ID for getting feature status */
    TPMI_INFO_ID = 0x81,    /* Special ID for PCI BDF and Package ID information */
}

pub const TPMI_CORE_INIT: i32 = 0;
pub const TPMI_CORE_EXIT: i32 = 1;

extern "C" {
    pub fn tpmi_register_notifier(nb: *mut notifier_block) -> i32;
    pub fn tpmi_unregister_notifier(nb: *mut notifier_block) -> i32;

    pub fn tpmi_get_platform_data(auxdev: *mut auxiliary_device) -> *mut oobmsm_plat_info;
    pub fn tpmi_get_resource_at_index(
        auxdev: *mut auxiliary_device,
        index: i32,
    ) -> *mut resource;
    pub fn tpmi_get_resource_count(auxdev: *mut auxiliary_device) -> i32;
    pub fn tpmi_get_feature_status(
        auxdev: *mut auxiliary_device,
        feature_id: i32,
        read_blocked: *mut bool,
        write_blocked: *mut bool,
    ) -> i32;
    pub fn tpmi_get_debugfs_dir(auxdev: *mut auxiliary_device) -> *mut dentry;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
