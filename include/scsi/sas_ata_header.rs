/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Support for SATA devices on Serial Attached SCSI (SAS) controllers
 *
 * Copyright (C) 2006 IBM Corporation
 *
 * Written by: Darrick J. Wong <djwong@us.ibm.com>, IBM Corporation
 */

/* Translated from sas_ata.h.  The original include dependencies are supplied
 * by other translation units. */

#[cfg(feature = "CONFIG_SCSI_SAS_ATA")]
#[inline]
pub unsafe fn dev_is_sata(dev: *mut domain_device) -> bool {
    match (*dev).dev_type {
        SAS_SATA_DEV | SAS_SATA_PENDING | SAS_SATA_PM | SAS_SATA_PM_PORT => true,
        _ => false,
    }
}

#[cfg(feature = "CONFIG_SCSI_SAS_ATA")]
extern "C" {
    pub fn sas_ata_schedule_reset(dev: *mut domain_device);
    pub fn sas_ata_device_link_abort(dev: *mut domain_device, force_reset: bool);
    pub fn sas_execute_ata_cmd(device: *mut domain_device, fis: *mut u8, force_phy_id: i32) -> i32;
    pub fn smp_ata_check_ready_type(link: *mut ata_link) -> i32;
    pub static sas_ata_sdev_attr_group: attribute_group;
}

#[cfg(not(feature = "CONFIG_SCSI_SAS_ATA"))]
#[inline]
pub unsafe fn dev_is_sata(_dev: *mut domain_device) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_SCSI_SAS_ATA"))]
#[inline]
pub unsafe fn sas_ata_schedule_reset(_dev: *mut domain_device) {}

#[cfg(not(feature = "CONFIG_SCSI_SAS_ATA"))]
#[inline]
pub unsafe fn sas_ata_device_link_abort(_dev: *mut domain_device, _force_reset: bool) {}

#[cfg(not(feature = "CONFIG_SCSI_SAS_ATA"))]
#[inline]
pub unsafe fn sas_execute_ata_cmd(
    _device: *mut domain_device,
    _fis: *mut u8,
    _force_phy_id: i32,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SCSI_SAS_ATA"))]
#[inline]
pub unsafe fn smp_ata_check_ready_type(_link: *mut ata_link) -> i32 {
    0
}

/* C: #define sas_ata_sdev_attr_group ((struct attribute_group) {}) */
#[cfg(not(feature = "CONFIG_SCSI_SAS_ATA"))]
pub const sas_ata_sdev_attr_group: attribute_group = attribute_group {};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
