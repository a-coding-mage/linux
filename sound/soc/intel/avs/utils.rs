// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u16 = u16;
type u32 = u32;

const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct guid_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ida {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct avs_module_entry {
    pub uuid: guid_t,
    pub module_id: u32,
    pub instance_max_count: c_int,
}

#[repr(C)]
pub struct avs_mods_info {
    pub count: u32,
    pub entries: [avs_module_entry; 0],
}

#[repr(C)]
pub struct avs_fw_entry {
    pub node: list_head,
    pub fw: *const firmware,
    pub name: *const c_char,
}

#[repr(C)]
pub struct avs_dev {
    pub dev: *mut device,
    pub mods_info: *mut avs_mods_info,
    pub modres_mutex: mutex,
    pub mod_idas: *mut *mut ida,
    pub fw_list: list_head,
}

extern "C" {
    static GFP_KERNEL: c_int;

    fn guid_equal(a: *const guid_t, b: *const guid_t) -> bool_;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn kstrdup_const(s: *const c_char, flags: c_int) -> *const c_char;
    fn kfree_const(ptr: *const c_void);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;

    fn ida_is_empty(ida: *mut ida) -> bool_;
    fn ida_destroy(ida: *mut ida);
    fn ida_init(ida: *mut ida);
    fn ida_alloc_max(ida: *mut ida, max: c_int, gfp: c_int) -> c_int;
    fn ida_free(ida: *mut ida, id: u32);

    fn avs_ipc_get_modules_info(adev: *mut avs_dev, info: *mut *mut avs_mods_info) -> c_int;
    fn AVS_IPC_RET(ret: c_int) -> c_int;

    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn release_firmware(fw: *const firmware);

    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
}

unsafe fn kzalloc_objs<T>(count: u32) -> *mut T {
    kzalloc(size_of::<T>().wrapping_mul(count as usize), GFP_KERNEL) as *mut T
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    kzalloc(size_of::<T>(), GFP_KERNEL) as *mut T
}

/* Caller responsible for holding adev->modres_mutex. */
unsafe fn avs_module_entry_index(adev: *mut avs_dev, uuid: *const guid_t) -> c_int {
    let mut i: c_int = 0;

    while i < (*(*adev).mods_info).count as c_int {
        let module: *mut avs_module_entry;

        module = (*(*adev).mods_info).entries.as_mut_ptr().add(i as usize);
        if guid_equal(&(*module).uuid, uuid) {
            return i;
        }

        i += 1;
    }

    -ENOENT
}

/* Caller responsible for holding adev->modres_mutex. */
unsafe fn avs_module_id_entry_index(adev: *mut avs_dev, module_id: u32) -> c_int {
    let mut i: c_int = 0;

    while i < (*(*adev).mods_info).count as c_int {
        let module: *mut avs_module_entry;

        module = (*(*adev).mods_info).entries.as_mut_ptr().add(i as usize);
        if (*module).module_id == module_id {
            return i;
        }

        i += 1;
    }

    -ENOENT
}

#[no_mangle]
pub unsafe extern "C" fn avs_get_module_entry(
    adev: *mut avs_dev,
    uuid: *const guid_t,
    entry: *mut avs_module_entry,
) -> c_int {
    let idx: c_int;

    guard_mutex!(&mut (*adev).modres_mutex);

    idx = avs_module_entry_index(adev, uuid);
    if idx >= 0 {
        memcpy(
            entry as *mut c_void,
            (*(*adev).mods_info).entries.as_ptr().add(idx as usize) as *const c_void,
            size_of::<avs_module_entry>(),
        );
    }

    if idx < 0 { idx } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn avs_get_module_id_entry(
    adev: *mut avs_dev,
    module_id: u32,
    entry: *mut avs_module_entry,
) -> c_int {
    let idx: c_int;

    guard_mutex!(&mut (*adev).modres_mutex);

    idx = avs_module_id_entry_index(adev, module_id);
    if idx >= 0 {
        memcpy(
            entry as *mut c_void,
            (*(*adev).mods_info).entries.as_ptr().add(idx as usize) as *const c_void,
            size_of::<avs_module_entry>(),
        );
    }

    if idx < 0 { idx } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn avs_get_module_id(adev: *mut avs_dev, uuid: *const guid_t) -> c_int {
    let mut module: avs_module_entry = core::mem::zeroed();
    let ret: c_int;

    ret = avs_get_module_entry(adev, uuid, &mut module);
    if ret == 0 { module.module_id as c_int } else { -ENOENT }
}

#[no_mangle]
pub unsafe extern "C" fn avs_is_module_ida_empty(adev: *mut avs_dev, module_id: u32) -> bool_ {
    let mut ret: bool_ = false;
    let idx: c_int;

    guard_mutex!(&mut (*adev).modres_mutex);

    idx = avs_module_id_entry_index(adev, module_id);
    if idx >= 0 {
        ret = ida_is_empty(*(*adev).mod_idas.add(idx as usize));
    }

    ret
}

/* Caller responsible for holding adev->modres_mutex. */
unsafe fn avs_module_ida_destroy(adev: *mut avs_dev) {
    let mut i: c_int = if !(*adev).mods_info.is_null() {
        (*(*adev).mods_info).count as c_int
    } else {
        0
    };

    while {
        let old = i;
        i -= 1;
        old != 0
    } {
        ida_destroy(*(*adev).mod_idas.add(i as usize));
        kfree(*(*adev).mod_idas.add(i as usize) as *const c_void);
    }
    kfree((*adev).mod_idas as *const c_void);
}

/* Caller responsible for holding adev->modres_mutex. */
unsafe fn avs_module_ida_alloc(
    adev: *mut avs_dev,
    newinfo: *mut avs_mods_info,
    purge: bool_,
) -> c_int {
    let oldinfo: *mut avs_mods_info = (*adev).mods_info;
    let ida_ptrs: *mut *mut ida;
    let mut tocopy_count: u32 = 0;
    let mut i: c_int;

    if !purge && !oldinfo.is_null() {
        if (*oldinfo).count >= (*newinfo).count {
            dev_warn(
                (*adev).dev,
                b"refreshing %d modules info with %d\n\0".as_ptr() as *const c_char,
                (*oldinfo).count,
                (*newinfo).count,
            );
        }
        tocopy_count = (*oldinfo).count;
    }

    ida_ptrs = kzalloc_objs::<*mut ida>((*newinfo).count);
    if ida_ptrs.is_null() {
        return -ENOMEM;
    }

    if tocopy_count != 0 {
        memcpy(
            ida_ptrs as *mut c_void,
            (*adev).mod_idas as *const c_void,
            (tocopy_count as usize).wrapping_mul(size_of::<*mut ida>()),
        );
    }

    i = tocopy_count as c_int;
    while i < (*newinfo).count as c_int {
        *ida_ptrs.add(i as usize) = kzalloc_obj::<ida>();
        if (*ida_ptrs.add(i as usize)).is_null() {
            while {
                let old = i;
                i -= 1;
                old != 0
            } {
                kfree(*ida_ptrs.add(i as usize) as *const c_void);
            }

            kfree(ida_ptrs as *const c_void);
            return -ENOMEM;
        }

        ida_init(*ida_ptrs.add(i as usize));
        i += 1;
    }

    /* If old elements have been reused, don't wipe them. */
    if tocopy_count != 0 {
        kfree((*adev).mod_idas as *const c_void);
    } else {
        avs_module_ida_destroy(adev);
    }

    (*adev).mod_idas = ida_ptrs;
    0
}

#[no_mangle]
pub unsafe extern "C" fn avs_module_info_init(adev: *mut avs_dev, purge: bool_) -> c_int {
    let mut info: *mut avs_mods_info = ptr::null_mut();
    let mut ret: c_int;

    ret = avs_ipc_get_modules_info(adev, &mut info);
    if ret != 0 {
        return AVS_IPC_RET(ret);
    }

    guard_mutex!(&mut (*adev).modres_mutex);

    ret = avs_module_ida_alloc(adev, info, purge);
    if ret < 0 {
        dev_err(
            (*adev).dev,
            b"initialize module idas failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    /* Refresh current information with newly received table. */
    kfree((*adev).mods_info as *const c_void);
    (*adev).mods_info = info;

    ret
}

#[no_mangle]
pub unsafe extern "C" fn avs_module_info_free(adev: *mut avs_dev) {
    guard_mutex!(&mut (*adev).modres_mutex);

    avs_module_ida_destroy(adev);
    kfree((*adev).mods_info as *const c_void);
    (*adev).mods_info = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn avs_module_id_alloc(adev: *mut avs_dev, module_id: u16) -> c_int {
    let idx: c_int;
    let max_id: c_int;

    guard_mutex!(&mut (*adev).modres_mutex);

    idx = avs_module_id_entry_index(adev, module_id as u32);
    if idx == -ENOENT {
        dev_err(
            (*adev).dev,
            b"invalid module id: %d\0".as_ptr() as *const c_char,
            module_id as c_int,
        );
        return -EINVAL;
    }
    max_id = (*(*adev).mods_info).entries.as_ptr().add(idx as usize).read().instance_max_count - 1;

    ida_alloc_max(*(*adev).mod_idas.add(idx as usize), max_id, GFP_KERNEL)
}

#[no_mangle]
pub unsafe extern "C" fn avs_module_id_free(
    adev: *mut avs_dev,
    module_id: u16,
    instance_id: u8,
) {
    let idx: c_int;

    guard_mutex!(&mut (*adev).modres_mutex);

    idx = avs_module_id_entry_index(adev, module_id as u32);
    if idx == -ENOENT {
        dev_err(
            (*adev).dev,
            b"invalid module id: %d\0".as_ptr() as *const c_char,
            module_id as c_int,
        );
    } else {
        ida_free(*(*adev).mod_idas.add(idx as usize), instance_id as u32);
    }
}

/*
 * Once driver loads FW it should keep it in memory, so we are not affected
 * by FW removal from filesystem or even worse by loading different FW at
 * runtime suspend/resume.
 */
#[no_mangle]
pub unsafe extern "C" fn avs_request_firmware(
    adev: *mut avs_dev,
    fw_p: *mut *const firmware,
    name: *const c_char,
) -> c_int {
    let mut entry: *mut avs_fw_entry;
    let ret: c_int;

    /* first check in list if it is not already loaded */
    list_for_each_entry!(entry, &mut (*adev).fw_list, node, {
        if strcmp(name, (*entry).name) == 0 {
            *fw_p = (*entry).fw;
            return 0;
        }
    });

    /* FW is not loaded, let's load it now and add to the list */
    entry = kzalloc_obj::<avs_fw_entry>();
    if entry.is_null() {
        return -ENOMEM;
    }

    (*entry).name = kstrdup_const(name, GFP_KERNEL);
    if (*entry).name.is_null() {
        kfree(entry as *const c_void);
        return -ENOMEM;
    }

    ret = request_firmware(&mut (*entry).fw, name, (*adev).dev);
    if ret < 0 {
        kfree_const((*entry).name as *const c_void);
        kfree(entry as *const c_void);
        return ret;
    }

    *fw_p = (*entry).fw;

    list_add_tail(&mut (*entry).node, &mut (*adev).fw_list);

    0
}

/*
 * Release single FW entry, used to handle errors in functions calling
 * avs_request_firmware()
 */
#[no_mangle]
pub unsafe extern "C" fn avs_release_last_firmware(adev: *mut avs_dev) {
    let entry: *mut avs_fw_entry;

    entry = list_last_entry!(&mut (*adev).fw_list, avs_fw_entry, node);

    list_del(&mut (*entry).node);
    release_firmware((*entry).fw);
    kfree_const((*entry).name as *const c_void);
    kfree(entry as *const c_void);
}

/*
 * Release all FW entries, used on driver removal
 */
#[no_mangle]
pub unsafe extern "C" fn avs_release_firmwares(adev: *mut avs_dev) {
    let mut entry: *mut avs_fw_entry;
    let mut tmp: *mut avs_fw_entry;

    list_for_each_entry_safe!(entry, tmp, &mut (*adev).fw_list, node, {
        list_del(&mut (*entry).node);
        release_firmware((*entry).fw);
        kfree_const((*entry).name as *const c_void);
        kfree(entry as *const c_void);
    });
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
