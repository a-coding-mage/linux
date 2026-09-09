// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * blacklist.c
 *
 * Check to see if the given machine has a known bad ACPI BIOS
 * or if the BIOS is too old.
 * Check given machine against acpi_rev_dmi_table[].
 */

// The declarations below are supplied by the corresponding kernel ACPI/DMI
// dependencies in the containing translation unit.

#[repr(C)]
pub struct acpi_platform_list {
    pub oem_id: *const core::ffi::c_char,
    pub oem_table_id: *const core::ffi::c_char,
    pub oem_revision: u32,
    pub signature: *const core::ffi::c_char,
    pub compare: Option<unsafe extern "C" fn(u32, u32) -> bool>,
    pub reason: *const core::ffi::c_char,
    pub data: i32,
}

#[cfg(feature = "CONFIG_DMI")]
#[repr(C)]
pub struct dmi_system_id {
    pub callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> i32>,
    pub ident: *const core::ffi::c_char,
    pub matches: [dmi_strmatch; 2],
}

#[cfg(feature = "CONFIG_DMI")]
#[repr(C)]
pub struct dmi_strmatch {
    pub slot: i32,
    pub substr: *const core::ffi::c_char,
}

unsafe extern "C" {
    fn acpi_match_platform_list(list: *const acpi_platform_list) -> i32;
    fn early_acpi_osi_init() -> i32;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    #[cfg(feature = "CONFIG_ACPI_REV_OVERRIDE_POSSIBLE")]
    fn pr_notice(fmt: *const core::ffi::c_char, ...);
    #[cfg(feature = "CONFIG_DMI")]
    fn dmi_check_system(list: *const dmi_system_id) -> i32;
    #[cfg(feature = "CONFIG_ACPI_REV_OVERRIDE_POSSIBLE")]
    fn acpi_rev_override_setup(arg: *const core::ffi::c_char);
}

static ACPI_SIG_DSDT: &[u8] = b"DSDT\0";

unsafe extern "C" fn less_than_or_equal(a: u32, b: u32) -> bool {
    a <= b
}

static mut acpi_blacklist: [acpi_platform_list; 5] = [
    acpi_platform_list { oem_id: b"PTLTD \0".as_ptr() as _, oem_table_id: b"  DSDT  \0".as_ptr() as _, oem_revision: 0x06040000, signature: ACPI_SIG_DSDT.as_ptr() as _, compare: Some(less_than_or_equal), reason: b"Multiple problems\0".as_ptr() as _, data: 1 },
    acpi_platform_list { oem_id: b"SONY  \0".as_ptr() as _, oem_table_id: b"U0      \0".as_ptr() as _, oem_revision: 0x20010313, signature: ACPI_SIG_DSDT.as_ptr() as _, compare: Some(less_than_or_equal), reason: b"ACPI driver problem\0".as_ptr() as _, data: 1 },
    acpi_platform_list { oem_id: b"INT440\0".as_ptr() as _, oem_table_id: b"SYSFexxx\0".as_ptr() as _, oem_revision: 0x00001001, signature: ACPI_SIG_DSDT.as_ptr() as _, compare: Some(less_than_or_equal), reason: b"Does not use _REG to protect EC OpRegions\0".as_ptr() as _, data: 1 },
    acpi_platform_list { oem_id: b"IBM   \0".as_ptr() as _, oem_table_id: b"TP600E  \0".as_ptr() as _, oem_revision: 0x00000105, signature: ACPI_SIG_DSDT.as_ptr() as _, compare: Some(less_than_or_equal), reason: b"Incorrect _ADR\0".as_ptr() as _, data: 1 },
    acpi_platform_list { oem_id: core::ptr::null(), oem_table_id: core::ptr::null(), oem_revision: 0, signature: core::ptr::null(), compare: None, reason: core::ptr::null(), data: 0 },
];

pub unsafe extern "C" fn acpi_blacklisted() -> i32 {
    let i = acpi_match_platform_list(acpi_blacklist.as_ptr());
    let mut blacklisted = 0;

    if i >= 0 {
        let entry = &acpi_blacklist[i as usize];
        pr_err(b"Vendor \"%6.6s\" System \"%8.8s\" Revision 0x%x has a known ACPI BIOS problem.\n\0".as_ptr() as _, entry.oem_id, entry.oem_table_id, entry.oem_revision);
        pr_err(b"Reason: %s. This is a %s error\n\0".as_ptr() as _, entry.reason, if entry.data != 0 { b"non-recoverable\0".as_ptr() } else { b"recoverable\0".as_ptr() });
        blacklisted = entry.data;
    }

    let _ = early_acpi_osi_init();
    #[cfg(feature = "CONFIG_DMI")]
    dmi_check_system(acpi_rev_dmi_table.as_ptr());
    blacklisted
}

#[cfg(feature = "CONFIG_DMI")]
#[cfg(feature = "CONFIG_ACPI_REV_OVERRIDE_POSSIBLE")]
unsafe extern "C" fn dmi_enable_rev_override(d: *const dmi_system_id) -> i32 {
    pr_notice(b"DMI detected: %s (force ACPI _REV to 5)\n\0".as_ptr() as _, (*d).ident);
    acpi_rev_override_setup(core::ptr::null());
    0
}

#[cfg(feature = "CONFIG_DMI")]
const DMI_SYS_VENDOR: i32 = 1;
#[cfg(feature = "CONFIG_DMI")]
const DMI_PRODUCT_NAME: i32 = 2;

#[cfg(feature = "CONFIG_DMI")]
static acpi_rev_dmi_table: [dmi_system_id; 6] = [
    #[cfg(feature = "CONFIG_ACPI_REV_OVERRIDE_POSSIBLE")]
    dmi_system_id { callback: Some(dmi_enable_rev_override), ident: b"DELL XPS 13 (2015)\0".as_ptr() as _, matches: [dmi_strmatch { slot: DMI_SYS_VENDOR, substr: b"Dell Inc.\0".as_ptr() as _ }, dmi_strmatch { slot: DMI_PRODUCT_NAME, substr: b"XPS 13 9343\0".as_ptr() as _ }] },
    #[cfg(feature = "CONFIG_ACPI_REV_OVERRIDE_POSSIBLE")]
    dmi_system_id { callback: Some(dmi_enable_rev_override), ident: b"DELL Precision 5520\0".as_ptr() as _, matches: [dmi_strmatch { slot: DMI_SYS_VENDOR, substr: b"Dell Inc.\0".as_ptr() as _ }, dmi_strmatch { slot: DMI_PRODUCT_NAME, substr: b"Precision 5520\0".as_ptr() as _ }] },
    #[cfg(feature = "CONFIG_ACPI_REV_OVERRIDE_POSSIBLE")]
    dmi_system_id { callback: Some(dmi_enable_rev_override), ident: b"DELL Precision 3520\0".as_ptr() as _, matches: [dmi_strmatch { slot: DMI_SYS_VENDOR, substr: b"Dell Inc.\0".as_ptr() as _ }, dmi_strmatch { slot: DMI_PRODUCT_NAME, substr: b"Precision 3520\0".as_ptr() as _ }] },
    #[cfg(feature = "CONFIG_ACPI_REV_OVERRIDE_POSSIBLE")]
    dmi_system_id { callback: Some(dmi_enable_rev_override), ident: b"DELL Latitude 3350\0".as_ptr() as _, matches: [dmi_strmatch { slot: DMI_SYS_VENDOR, substr: b"Dell Inc.\0".as_ptr() as _ }, dmi_strmatch { slot: DMI_PRODUCT_NAME, substr: b"Latitude 3350\0".as_ptr() as _ }] },
    #[cfg(feature = "CONFIG_ACPI_REV_OVERRIDE_POSSIBLE")]
    dmi_system_id { callback: Some(dmi_enable_rev_override), ident: b"DELL Inspiron 7537\0".as_ptr() as _, matches: [dmi_strmatch { slot: DMI_SYS_VENDOR, substr: b"Dell Inc.\0".as_ptr() as _ }, dmi_strmatch { slot: DMI_PRODUCT_NAME, substr: b"Inspiron 7537\0".as_ptr() as _ }] },
    dmi_system_id { callback: None, ident: core::ptr::null(), matches: [dmi_strmatch { slot: 0, substr: core::ptr::null() }, dmi_strmatch { slot: 0, substr: core::ptr::null() }] },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
