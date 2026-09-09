// SPDX-License-Identifier: GPL-2.0

// *****************************************************************************
//  platform-pci-unplug.c
//
//  Xen platform PCI device driver
//  Copyright (c) 2010, Citrix
// *****************************************************************************

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt

// The following symbols are supplied by the surrounding kernel/Xen sources.
use core::ffi::{c_char, c_int, c_short};

extern "C" {
    fn inb(port: u16) -> u8;
    fn inw(port: u16) -> u16;
    fn outw(value: u16, port: u16);
    fn outl(value: u32, port: u16);
    fn xen_domain() -> bool;
    fn xen_pv_domain() -> bool;
    fn xen_pvh_domain() -> bool;
    fn xen_hvm_domain() -> bool;
    fn xen_must_unplug_nics() -> bool;
    fn xen_must_unplug_disks() -> bool;
    fn strchr(s: *mut c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
}

// Values supplied by <xen/platform_pci.h>.
const XEN_PLATFORM_ERR_MAGIC: c_int = -1;
const XEN_PLATFORM_ERR_PROTOCOL: c_int = -2;
const XEN_PLATFORM_ERR_BLACKLIST: c_int = -3;

/* store the value of xen_emul_unplug after the unplug is done */
static mut xen_platform_pci_unplug: c_int = 0;
static mut xen_emul_unplug: c_int = 0;

unsafe fn check_platform_magic() -> c_int {
    let magic: c_short = inw(XEN_IOPORT_MAGIC) as c_short;
    if magic != XEN_IOPORT_MAGIC_VAL {
        pr_err!("Xen Platform PCI: unrecognised magic value\n");
        return XEN_PLATFORM_ERR_MAGIC;
    }

    let protocol: i8 = inb(XEN_IOPORT_PROTOVER) as i8;
    pr_debug!("Xen Platform PCI: I/O protocol version %d\n", protocol);

    match protocol {
        1 => {
            outw(XEN_IOPORT_LINUX_PRODNUM, XEN_IOPORT_PRODNUM);
            outl(XEN_IOPORT_LINUX_DRVVER, XEN_IOPORT_DRVVER);
            if inw(XEN_IOPORT_MAGIC) != XEN_IOPORT_MAGIC_VAL {
                pr_err!("Xen Platform: blacklisted by host\n");
                return XEN_PLATFORM_ERR_BLACKLIST;
            }
        }
        _ => {
            pr_warn!("Xen Platform PCI: unknown I/O protocol version\n");
            return XEN_PLATFORM_ERR_PROTOCOL;
        }
    }

    0
}

pub unsafe extern "C" fn xen_has_pv_devices() -> bool {
    if !xen_domain() {
        return false;
    }

    /* PV and PVH domains always have them. */
    if xen_pv_domain() || xen_pvh_domain() {
        return true;
    }

    /* And user has xen_platform_pci=0 set in guest config as
     * driver did not modify the value. */
    if xen_platform_pci_unplug == 0 {
        return false;
    }
    if xen_platform_pci_unplug & XEN_UNPLUG_NEVER != 0 {
        return false;
    }
    if xen_platform_pci_unplug & XEN_UNPLUG_ALL != 0 {
        return true;
    }

    /* This is an odd one - we are going to run legacy
     * and PV drivers at the same time. */
    if xen_platform_pci_unplug & XEN_UNPLUG_UNNECESSARY != 0 {
        return true;
    }

    /* And the caller has to follow with xen_pv_{disk,nic}_devices
     * to be certain which driver can load. */
    false
}

unsafe fn __xen_has_pv_device(state: c_int) -> bool {
    /* HVM domains might or might not */
    if xen_hvm_domain() && (xen_platform_pci_unplug & state != 0) {
        return true;
    }
    xen_has_pv_devices()
}

pub unsafe extern "C" fn xen_has_pv_nic_devices() -> bool {
    __xen_has_pv_device(XEN_UNPLUG_ALL_NICS | XEN_UNPLUG_ALL)
}

pub unsafe extern "C" fn xen_has_pv_disk_devices() -> bool {
    __xen_has_pv_device(XEN_UNPLUG_ALL_IDE_DISKS | XEN_UNPLUG_AUX_IDE_DISKS | XEN_UNPLUG_ALL)
}

/*
 * This one is odd - it determines whether you want to run PV _and_
 * legacy (IDE) drivers together. This combination is only possible
 * under HVM.
 */
pub unsafe extern "C" fn xen_has_pv_and_legacy_disk_devices() -> bool {
    if !xen_domain() {
        return false;
    }
    /* N.B. This is only ever used in HVM mode */
    if xen_pv_domain() {
        return false;
    }
    xen_platform_pci_unplug & XEN_UNPLUG_UNNECESSARY != 0
}

pub unsafe extern "C" fn xen_unplug_emulated_devices() {
    let mut r: c_int;

    /* PVH guests don't have emulated devices. */
    if xen_pvh_domain() {
        return;
    }
    /* user explicitly requested no unplug */
    if xen_emul_unplug & XEN_UNPLUG_NEVER != 0 {
        return;
    }
    /* check the version of the xen platform PCI device */
    r = check_platform_magic();
    /* If the version matches enable the Xen platform PCI driver.
     * Also enable the Xen platform PCI driver if the host does
     * not support the unplug protocol (XEN_PLATFORM_ERR_MAGIC)
     * but the user told us that unplugging is unnecessary. */
    if r != 0 && !(r == XEN_PLATFORM_ERR_MAGIC &&
                   (xen_emul_unplug & XEN_UNPLUG_UNNECESSARY != 0)) {
        return;
    }
    /* Set the default value of xen_emul_unplug depending on whether or
     * not the Xen PV frontends and the Xen platform PCI driver have
     * been compiled for this kernel (modules or built-in are both OK). */
    if xen_emul_unplug == 0 {
        if xen_must_unplug_nics() {
            pr_info!("Netfront and the Xen platform PCI driver have \
                     been compiled for this kernel: unplug emulated NICs.\n");
            xen_emul_unplug |= XEN_UNPLUG_ALL_NICS;
        }
        if xen_must_unplug_disks() {
            pr_info!("Blkfront and the Xen platform PCI driver have \
                     been compiled for this kernel: unplug emulated disks.\n\
                     You might have to change the root device\n\
                     from /dev/hd[a-d] to /dev/xvd[a-d]\n\
                     in your root= kernel command line option\n");
            xen_emul_unplug |= XEN_UNPLUG_ALL_IDE_DISKS;
        }
    }
    /* Now unplug the emulated devices */
    if xen_emul_unplug & XEN_UNPLUG_UNNECESSARY == 0 {
        outw(xen_emul_unplug as u16, XEN_IOPORT_UNPLUG);
    }
    xen_platform_pci_unplug = xen_emul_unplug;
}

unsafe extern "C" fn parse_xen_emul_unplug(arg: *mut c_char) -> c_int {
    let mut p = arg;
    let mut q: *mut c_char;
    let mut l: usize;

    while !p.is_null() {
        q = strchr(p, ',' as c_int);
        if !q.is_null() {
            l = q.offset_from(p) as usize;
            q = q.add(1);
        } else {
            l = strlen(p);
        }
        if strncmp(p, c"all".as_ptr(), l) == 0 {
            xen_emul_unplug |= XEN_UNPLUG_ALL;
        } else if strncmp(p, c"ide-disks".as_ptr(), l) == 0 {
            xen_emul_unplug |= XEN_UNPLUG_ALL_IDE_DISKS;
        } else if strncmp(p, c"aux-ide-disks".as_ptr(), l) == 0 {
            xen_emul_unplug |= XEN_UNPLUG_AUX_IDE_DISKS;
        } else if strncmp(p, c"nics".as_ptr(), l) == 0 {
            xen_emul_unplug |= XEN_UNPLUG_ALL_NICS;
        } else if strncmp(p, c"unnecessary".as_ptr(), l) == 0 {
            xen_emul_unplug |= XEN_UNPLUG_UNNECESSARY;
        } else if strncmp(p, c"never".as_ptr(), l) == 0 {
            xen_emul_unplug |= XEN_UNPLUG_NEVER;
        } else {
            pr_warn!("unrecognised option '%s' in parameter 'xen_emul_unplug'\n", p);
        }
        p = q;
    }
    0
}

// early_param("xen_emul_unplug", parse_xen_emul_unplug);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
