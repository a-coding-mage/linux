/* SPDX-License-Identifier: GPL-2.0 */

pub const XEN_IOPORT_MAGIC_VAL: i32 = 0x49d2;
pub const XEN_IOPORT_LINUX_PRODNUM: i32 = 0x0003;
pub const XEN_IOPORT_LINUX_DRVVER: i32 = 0x0001;

pub const XEN_IOPORT_BASE: i32 = 0x10;

pub const XEN_IOPORT_PLATFLAGS: i32 = XEN_IOPORT_BASE + 0; /* 1 byte access (R/W) */
pub const XEN_IOPORT_MAGIC: i32 = XEN_IOPORT_BASE + 0; /* 2 byte access (R) */
pub const XEN_IOPORT_UNPLUG: i32 = XEN_IOPORT_BASE + 0; /* 2 byte access (W) */
pub const XEN_IOPORT_DRVVER: i32 = XEN_IOPORT_BASE + 0; /* 4 byte access (W) */

pub const XEN_IOPORT_SYSLOG: i32 = XEN_IOPORT_BASE + 2; /* 1 byte access (W) */
pub const XEN_IOPORT_PROTOVER: i32 = XEN_IOPORT_BASE + 2; /* 1 byte access (R) */
pub const XEN_IOPORT_PRODNUM: i32 = XEN_IOPORT_BASE + 2; /* 2 byte access (W) */

pub const XEN_UNPLUG_ALL_IDE_DISKS: i32 = 1 << 0;
pub const XEN_UNPLUG_ALL_NICS: i32 = 1 << 1;
pub const XEN_UNPLUG_AUX_IDE_DISKS: i32 = 1 << 2;
pub const XEN_UNPLUG_ALL: i32 =
    XEN_UNPLUG_ALL_IDE_DISKS | XEN_UNPLUG_ALL_NICS | XEN_UNPLUG_AUX_IDE_DISKS;

pub const XEN_UNPLUG_UNNECESSARY: i32 = 1 << 16;
pub const XEN_UNPLUG_NEVER: i32 = 1 << 17;

#[inline]
pub fn xen_must_unplug_nics() -> i32 {
    // The C condition depends on CONFIG_XEN_NETDEV_FRONTEND(_MODULE) and CONFIG_X86.
    #[cfg(all(
        feature = "CONFIG_X86",
        any(feature = "CONFIG_XEN_NETDEV_FRONTEND", feature = "CONFIG_XEN_NETDEV_FRONTEND_MODULE")
    ))]
    {
        return 1;
    }
    0
}

#[inline]
pub fn xen_must_unplug_disks() -> i32 {
    // The C condition depends on CONFIG_XEN_BLKDEV_FRONTEND(_MODULE) and CONFIG_X86.
    #[cfg(all(
        feature = "CONFIG_X86",
        any(feature = "CONFIG_XEN_BLKDEV_FRONTEND", feature = "CONFIG_XEN_BLKDEV_FRONTEND_MODULE")
    ))]
    {
        return 1;
    }
    0
}

#[cfg(feature = "CONFIG_X86")]
unsafe extern "C" {
    pub fn xen_has_pv_devices() -> bool;
    pub fn xen_has_pv_disk_devices() -> bool;
    pub fn xen_has_pv_nic_devices() -> bool;
    pub fn xen_has_pv_and_legacy_disk_devices() -> bool;
}

#[cfg(not(feature = "CONFIG_X86"))]
#[inline]
pub fn xen_has_pv_devices() -> bool {
    cfg!(feature = "CONFIG_XEN")
}

#[cfg(not(feature = "CONFIG_X86"))]
#[inline]
pub fn xen_has_pv_disk_devices() -> bool {
    cfg!(feature = "CONFIG_XEN")
}

#[cfg(not(feature = "CONFIG_X86"))]
#[inline]
pub fn xen_has_pv_nic_devices() -> bool {
    cfg!(feature = "CONFIG_XEN")
}

#[cfg(not(feature = "CONFIG_X86"))]
#[inline]
pub fn xen_has_pv_and_legacy_disk_devices() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
