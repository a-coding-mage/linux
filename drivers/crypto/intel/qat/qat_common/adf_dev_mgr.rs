// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Translated from adf_dev_mgr.c. Kernel/project-provided declarations and
// macros are intentionally referenced but not reimplemented here.

static mut accel_table: list_head = LIST_HEAD_INIT;
static mut vfs_table: list_head = LIST_HEAD_INIT;
static mut table_lock: mutex = DEFINE_MUTEX_INIT;
static mut num_devices: u32 = 0;
static mut id_map: [u8; ADF_MAX_DEVICES as usize] = [0; ADF_MAX_DEVICES as usize];

#[repr(C)]
struct vf_id_map {
    bdf: u32,
    id: u32,
    fake_id: u32,
    attached: bool,
    list: list_head,
}

unsafe fn adf_get_vf_id(vf: *mut adf_accel_dev) -> i32 {
    (7 * (PCI_SLOT((*accel_to_pci_dev(vf)).devfn) - 1))
        + PCI_FUNC((*accel_to_pci_dev(vf)).devfn)
        + (PCI_SLOT((*accel_to_pci_dev(vf)).devfn) - 1)
}

unsafe fn adf_get_vf_num(vf: *mut adf_accel_dev) -> u32 {
    (((*(*accel_to_pci_dev(vf)).bus).number as u32) << 8) | adf_get_vf_id(vf) as u32
}

unsafe fn adf_find_vf(bdf: u32) -> *mut vf_id_map {
    let mut itr: *mut list_head = (&raw mut vfs_table);
    list_for_each(itr, &raw mut vfs_table) {
        let ptr: *mut vf_id_map = list_entry(itr, vf_id_map, list);
        if (*ptr).bdf == bdf {
            return ptr;
        }
    }
    core::ptr::null_mut()
}

/**
 * adf_clean_vf_map() - Cleans VF id mappings
 * @vf: flag indicating whether mappings is cleaned
 *\tfor vfs only or for vfs and pfs
 *
 * Function cleans internal ids for virtual functions.
 */
#[no_mangle]
pub unsafe extern "C" fn adf_clean_vf_map(vf: bool) {
    let mut map: *mut vf_id_map;
    let mut ptr: *mut list_head;
    let mut tmp: *mut list_head;

    mutex_lock(&raw mut table_lock);
    list_for_each_safe(ptr, tmp, &raw mut vfs_table) {
        map = list_entry(ptr, vf_id_map, list);
        if (*map).bdf != u32::MAX {
            id_map[(*map).id as usize] = 0;
            num_devices -= 1;
        }
        if vf && (*map).bdf == u32::MAX {
            continue;
        }
        list_del(ptr);
        kfree(map);
    }
    mutex_unlock(&raw mut table_lock);
}

/** Update internal index for VFs. */
#[no_mangle]
pub unsafe extern "C" fn adf_devmgr_update_class_index(hw_data: *mut adf_hw_device_data) {
    let class = (*hw_data).dev_class;
    let mut i = 0;
    let mut itr: *mut list_head;

    list_for_each(itr, &raw mut accel_table) {
        let ptr: *mut adf_accel_dev = list_entry(itr, adf_accel_dev, list);
        if (*ptr).hw_device.dev_class == class {
            (*ptr).hw_device.instance_id = i;
            i += 1;
        }
        if i == (*class).instances {
            break;
        }
    }
}

unsafe fn adf_find_free_id() -> u32 {
    for i in 0..ADF_MAX_DEVICES as usize {
        if id_map[i] == 0 {
            id_map[i] = 1;
            return i as u32;
        }
    }
    ADF_MAX_DEVICES + 1
}

/** Add accel_dev to the acceleration framework. */
#[no_mangle]
pub unsafe extern "C" fn adf_devmgr_add_dev(
    accel_dev: *mut adf_accel_dev,
    pf: *mut adf_accel_dev,
) -> i32 {
    let mut ret = 0;
    let mut itr: *mut list_head;

    if num_devices == ADF_MAX_DEVICES {
        dev_err(&GET_DEV(accel_dev), "Only support up to %d devices\n", ADF_MAX_DEVICES);
        return -EFAULT;
    }

    mutex_lock(&raw mut table_lock);
    atomic_set(&raw mut (*accel_dev).ref_count, 0);

    if !(*accel_dev).is_vf || pf.is_null() {
        list_for_each(itr, &raw mut accel_table) {
            let ptr: *mut adf_accel_dev = list_entry(itr, adf_accel_dev, list);
            if ptr == accel_dev {
                ret = -EEXIST;
                goto unlock;
            }
        }
        list_add_tail(&raw mut (*accel_dev).list, &raw mut accel_table);
        (*accel_dev).accel_id = adf_find_free_id();
        if (*accel_dev).accel_id > ADF_MAX_DEVICES {
            ret = -EFAULT;
            goto unlock;
        }
        num_devices += 1;
        let map: *mut vf_id_map = kzalloc_obj();
        if map.is_null() {
            ret = -ENOMEM;
            goto unlock;
        }
        (*map).bdf = u32::MAX;
        (*map).id = (*accel_dev).accel_id;
        (*map).fake_id = (*map).id;
        (*map).attached = true;
        list_add_tail(&raw mut (*map).list, &raw mut vfs_table);
    } else {
        let mut map = adf_find_vf(adf_get_vf_num(accel_dev));
        if !map.is_null() {
            (*accel_dev).accel_id = (*map).id;
            list_add_tail(&raw mut (*accel_dev).list, &raw mut accel_table);
            (*map).fake_id += 1;
            (*map).attached = true;
            let mut next = list_next_entry(map, list);
            while !next.is_null() && (*next).list.next != (&raw mut vfs_table) {
                (*next).fake_id += 1;
                next = list_next_entry(next, list);
            }
            ret = 0;
            goto unlock;
        }
        map = kzalloc_obj();
        if map.is_null() {
            ret = -ENOMEM;
            goto unlock;
        }
        (*accel_dev).accel_id = adf_find_free_id();
        if (*accel_dev).accel_id > ADF_MAX_DEVICES {
            kfree(map);
            ret = -EFAULT;
            goto unlock;
        }
        num_devices += 1;
        list_add_tail(&raw mut (*accel_dev).list, &raw mut accel_table);
        (*map).bdf = adf_get_vf_num(accel_dev);
        (*map).id = (*accel_dev).accel_id;
        (*map).fake_id = (*map).id;
        (*map).attached = true;
        list_add_tail(&raw mut (*map).list, &raw mut vfs_table);
    }
    mutex_init(&raw mut (*accel_dev).state_lock);
unlock:
    mutex_unlock(&raw mut table_lock);
    ret
}

unsafe fn adf_devmgr_get_head() -> *mut list_head { &raw mut accel_table }

/** Remove accel_dev from the acceleration framework. */
#[no_mangle]
pub unsafe extern "C" fn adf_devmgr_rm_dev(accel_dev: *mut adf_accel_dev, pf: *mut adf_accel_dev) {
    mutex_lock(&raw mut table_lock);
    if !(*accel_dev).is_vf || pf.is_null() {
        id_map[(*accel_dev).accel_id as usize] = 0;
        num_devices -= 1;
    } else {
        let map = adf_find_vf(adf_get_vf_num(accel_dev));
        if map.is_null() {
            dev_err(&GET_DEV(accel_dev), "Failed to find VF map\n");
            goto unlock_rm;
        }
        (*map).fake_id -= 1;
        (*map).attached = false;
        let mut next = list_next_entry(map, list);
        while !next.is_null() && (*next).list.next != (&raw mut vfs_table) {
            (*next).fake_id -= 1;
            next = list_next_entry(next, list);
        }
    }
unlock_rm:
    mutex_destroy(&raw mut (*accel_dev).state_lock);
    list_del(&raw mut (*accel_dev).list);
    mutex_unlock(&raw mut table_lock);
}

/** Get accel_dev associated with the pci_dev. */
#[no_mangle]
pub unsafe extern "C" fn adf_devmgr_pci_to_accel_dev(pci_dev: *mut pci_dev) -> *mut adf_accel_dev {
    let mut itr: *mut list_head;
    mutex_lock(&raw mut table_lock);
    list_for_each(itr, &raw mut accel_table) {
        let ptr: *mut adf_accel_dev = list_entry(itr, adf_accel_dev, list);
        if (*ptr).accel_pci_dev.pci_dev == pci_dev {
            mutex_unlock(&raw mut table_lock);
            return ptr;
        }
    }
    mutex_unlock(&raw mut table_lock);
    core::ptr::null_mut()
}

/** Check whether accel_dev is currently in use. */
#[no_mangle]
pub unsafe extern "C" fn adf_dev_in_use(accel_dev: *mut adf_accel_dev) -> i32 {
    (atomic_read(&raw mut (*accel_dev).ref_count) != 0) as i32
}

/** Increment accel_dev reference count. */
#[no_mangle]
pub unsafe extern "C" fn adf_dev_get(accel_dev: *mut adf_accel_dev) -> i32 {
    if atomic_add_return(1, &raw mut (*accel_dev).ref_count) == 1
        && !try_module_get((*accel_dev).owner)
    {
        return -EFAULT;
    }
    0
}

/** Decrement accel_dev reference count. */
#[no_mangle]
pub unsafe extern "C" fn adf_dev_put(accel_dev: *mut adf_accel_dev) {
    if atomic_sub_return(1, &raw mut (*accel_dev).ref_count) == 0 {
        module_put((*accel_dev).owner);
    }
}

/** Check whether device is in reset. */
#[no_mangle]
pub unsafe extern "C" fn adf_devmgr_in_reset(accel_dev: *mut adf_accel_dev) -> i32 {
    test_bit(ADF_STATUS_RESTARTING, &raw mut (*accel_dev).status)
}

/** Check whether device has started. */
#[no_mangle]
pub unsafe extern "C" fn adf_dev_started(accel_dev: *mut adf_accel_dev) -> i32 {
    test_bit(ADF_STATUS_STARTED, &raw mut (*accel_dev).status)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
