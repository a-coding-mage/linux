// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2020
 *
 * Author(s):
 *   Niklas Schnelle <schnelle@linux.ibm.com>
 *
 */

// #define pr_fmt(fmt) "zpci: " fmt
// Dependencies are supplied by the surrounding kernel translation.

use core::ptr;

static mut iov_res: resource = resource {
    name: b"PCI IOV res\0".as_ptr() as *const i8,
    start: 0,
    end: -1isize as resource_size_t,
    flags: IORESOURCE_MEM,
};

pub unsafe fn zpci_iov_map_resources(pdev: *mut pci_dev) {
    let mut len: resource_size_t;
    let mut i: c_int = 0;

    while i < PCI_SRIOV_NUM_BARS {
        let bar = i + PCI_IOV_RESOURCES;

        len = pci_resource_len(pdev, bar);
        if len == 0 {
            i += 1;
            continue;
        }
        (*pdev).resource[bar as usize].parent = &raw mut iov_res;
        i += 1;
    }
}

pub unsafe fn zpci_iov_remove_virtfn(pdev: *mut pci_dev, vfn: c_int) {
    pci_lock_rescan_remove();
    /* Linux' vfid's start at 0 vfn at 1 */
    pci_iov_remove_virtfn((*pdev).physfn, vfn - 1);
    pci_unlock_rescan_remove();
}

unsafe fn zpci_iov_link_virtfn(
    pdev: *mut pci_dev,
    virtfn: *mut pci_dev,
    vfid: c_int,
) -> c_int {
    let rc = pci_iov_sysfs_link(pdev, virtfn, vfid);
    if rc != 0 {
        return rc;
    }

    (*virtfn).is_virtfn = 1;
    (*virtfn).multifunction = 0;
    (*virtfn).physfn = pci_dev_get(pdev);

    0
}

/**
 * zpci_iov_find_parent_pf - Find the parent PF, if any, of the given function
 * @zbus: The bus that the PCI function is on, or would be added on
 * @zdev: The PCI function
 *
 * Finds the parent PF, if it exists and is configured, of the given PCI function
 * and increments its refcount. Th PF is searched for on the provided bus so the
 * caller has to ensure that this is the correct bus to search. This function may
 * be used before adding the PCI function to a zbus.
 *
 * Return: Pointer to the struct pci_dev of the parent PF or NULL if it not
 * found. If the function is not a VF or has no RequesterID information,
 * NULL is returned as well.
 */
pub unsafe fn zpci_iov_find_parent_pf(
    zbus: *mut zpci_bus,
    mut zdev: *mut zpci_dev,
) -> *mut pci_dev {
    let mut i: c_int;
    let vfid: c_int;
    let devfn: c_int;
    let mut cand_devfn: c_int;
    let mut pdev: *mut pci_dev;

    if (*zbus).multifunction == 0 {
        return ptr::null_mut();
    }
    /* Non-VFs and VFs without RID available don't have a parent */
    if (*zdev).vfn == 0 || (*zdev).rid_available == 0 {
        return ptr::null_mut();
    }
    /* Linux vfid starts at 0 vfn at 1 */
    vfid = (*zdev).vfn - 1;
    devfn = (*zdev).rid & ZPCI_RID_MASK_DEVFN;
    /*
     * If the parent PF for the given VF is also configured in the
     * instance, it must be on the same zbus.
     * We can then identify the parent PF by checking what
     * devfn the VF would have if it belonged to that PF using the PF's
     * stride and offset. Only if this candidate devfn matches the
     * actual devfn will we link both functions.
     */
    i = 0;
    while i < ZPCI_FUNCTIONS_PER_BUS {
        zdev = (*zbus).function[i as usize];
        if !zdev.is_null() && (*zdev).is_physfn != 0 {
            pdev = pci_get_slot((*zbus).bus, (*zdev).devfn);
            if pdev.is_null() {
                i += 1;
                continue;
            }
            cand_devfn = pci_iov_virtfn_devfn(pdev, vfid);
            if cand_devfn == devfn {
                return pdev;
            }
            /* balance pci_get_slot() */
            pci_dev_put(pdev);
        }
        i += 1;
    }
    ptr::null_mut()
}

pub unsafe fn zpci_iov_setup_virtfn(
    zbus: *mut zpci_bus,
    virtfn: *mut pci_dev,
    vfn: c_int,
) -> c_int {
    let zdev = to_zpci(virtfn);
    let pdev_pf: *mut pci_dev;
    let mut rc: c_int = 0;

    pdev_pf = zpci_iov_find_parent_pf(zbus, zdev);
    if !pdev_pf.is_null() {
        /* Linux' vfids start at 0 while zdev->vfn starts at 1 */
        rc = zpci_iov_link_virtfn(pdev_pf, virtfn, (*zdev).vfn - 1);
        pci_dev_put(pdev_pf);
    }
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
