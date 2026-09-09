// SPDX-License-Identifier: GPL-2.0
/*
 *  IBM System z PNET ID Support
 *
 *    Copyright IBM Corp. 2018
 */

// Kernel headers and symbols used by this translation are supplied by the
// surrounding kernel crate.

const PNETIDS_LEN: usize = 64; // Total utility string length in bytes.
const MAX_PNETID_LEN: usize = 16; // Max. length of a single port PNETID.
const MAX_PNETID_PORTS: usize = PNETIDS_LEN / MAX_PNETID_LEN;

const ENOMEM: i32 = 12;
const EOPNOTSUPP: i32 = 95;
const ENOENT: i32 = 2;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ccw_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ccwgroup_device {
    pub cdev: [*mut ccw_device; 1],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct zpci_dev {
    pub util_str: [u8; PNETIDS_LEN],
}

extern "C" {
    fn memset(dest: *mut core::ffi::c_void, value: i32, count: usize);
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize);
    fn memcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, count: usize) -> i32;
    fn kfree(ptr: *mut u8);
    fn dev_is_ccwgroup(dev: *mut device) -> bool;
    fn to_ccwgroupdev(dev: *mut device) -> *mut ccwgroup_device;
    fn ccw_device_get_util_str(cdev: *mut ccw_device, length: u32) -> *mut u8;
    fn dev_is_pci(dev: *mut device) -> bool;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn to_zpci(dev: *mut pci_dev) -> *mut zpci_dev;
    fn EBCASC(data: *mut u8, length: usize);
}

/*
 * Get the PNETIDs from a device.
 * s390 hardware supports the definition of a so-called Physical Network
 * Identifier (short PNETID) per network device port. These PNETIDs can be
 * used to identify network devices that are attached to the same physical
 * network (broadcast domain).
 */
unsafe fn pnet_ids_by_device(dev: *mut device, pnetids: *mut u8) -> i32 {
    memset(pnetids.cast(), 0, PNETIDS_LEN);
    if dev_is_ccwgroup(dev) {
        let gdev = to_ccwgroupdev(dev);
        let util_str = ccw_device_get_util_str((*gdev).cdev[0], 0);
        if util_str.is_null() {
            return -ENOMEM;
        }
        memcpy(pnetids.cast(), util_str.cast(), PNETIDS_LEN);
        EBCASC(pnetids, PNETIDS_LEN);
        kfree(util_str);
        return 0;
    }
    if dev_is_pci(dev) {
        let zdev = to_zpci(to_pci_dev(dev));
        memcpy(
            pnetids.cast(),
            (*zdev).util_str.as_ptr().cast(),
            core::mem::size_of_val(&(*zdev).util_str),
        );
        EBCASC(pnetids, core::mem::size_of_val(&(*zdev).util_str));
        return 0;
    }
    -EOPNOTSUPP
}

/* Extract the pnetid for a device port. */
#[no_mangle]
pub unsafe extern "C" fn pnet_id_by_dev_port(
    dev: *mut device,
    port: u16,
    pnetid: *mut u8,
) -> i32 {
    let mut pnetids = [[0u8; MAX_PNETID_LEN]; MAX_PNETID_PORTS];
    let zero = [0u8; MAX_PNETID_LEN];
    let mut rc = 0;

    if dev.is_null() || (port as usize) >= MAX_PNETID_PORTS {
        return -ENOENT;
    }

    if pnet_ids_by_device(dev, pnetids.as_mut_ptr().cast()) == 0
        && memcmp(
            pnetids[port as usize].as_ptr().cast(),
            zero.as_ptr().cast(),
            MAX_PNETID_LEN,
        ) != 0
    {
        memcpy(
            pnetid.cast(),
            pnetids[port as usize].as_ptr().cast(),
            MAX_PNETID_LEN,
        );
    } else {
        rc = -ENOENT;
    }

    rc
}

// EXPORT_SYMBOL_GPL(pnet_id_by_dev_port);
// MODULE_DESCRIPTION("pnetid determination from utility strings");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
