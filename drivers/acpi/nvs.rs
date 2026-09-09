// SPDX-License-Identifier: GPL-2.0-only
/*
 * nvs.c - Routines for saving and restoring ACPI NVS memory region
 *
 * Copyright (C) 2008-2011 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct NvsRegion {
    pub phys_start: u64,
    pub size: u64,
    pub node: ListHead,
}

#[repr(C)]
pub struct NvsPage {
    pub phys_start: usize,
    pub size: u32,
    pub kaddr: *mut core::ffi::c_void,
    pub data: *mut core::ffi::c_void,
    pub unmap: bool,
    pub node: ListHead,
}

#[repr(C)]
pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

unsafe extern "C" {
    static mut nvs_region_list: ListHead;
    fn kmalloc_obj<T>() -> *mut T;
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn list_add_tail(new: *mut ListHead, head: *mut ListHead);
    fn list_del(entry: *mut ListHead);
    fn acpi_os_unmap_iomem(addr: *mut core::ffi::c_void, size: u32);
    fn iounmap(addr: *mut core::ffi::c_void);
    fn acpi_os_get_iomem(phys: usize, size: u32) -> *mut core::ffi::c_void;
    fn acpi_os_ioremap(phys: usize, size: u32) -> *mut core::ffi::c_void;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

// CONFIG_ACPI_SLEEP supplies the implementation below; without it the
// registration hook is an inline no-op returning zero.
static mut nvs_list: ListHead = ListHead {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};

#[cfg(feature = "CONFIG_ACPI_SLEEP")]
unsafe fn suspend_nvs_register(start: usize, mut size: usize) -> i32 {
    while size > 0 {
        let entry = kzalloc_obj::<NvsPage>();
        if entry.is_null() {
            let mut node = nvs_list.next;
            while node != &mut nvs_list {
                let next = (*node).next;
                list_del(node);
                kfree(node as *mut core::ffi::c_void);
                node = next;
            }
            return -ENOMEM;
        }
        list_add_tail(&mut (*entry).node, &mut nvs_list);
        (*entry).phys_start = start;
        let nr_bytes = PAGE_SIZE - (start & !PAGE_MASK);
        (*entry).size = if size < nr_bytes { size } else { nr_bytes } as u32;
        start += (*entry).size as usize;
        size -= (*entry).size as usize;
    }
    0
}

#[cfg(not(feature = "CONFIG_ACPI_SLEEP"))]
unsafe fn suspend_nvs_register(_start: usize, _size: usize) -> i32 {
    0
}

const ENOMEM: i32 = 12;
const GFP_KERNEL: u32 = 0;
const PAGE_SIZE: usize = 4096;
const PAGE_MASK: usize = !(PAGE_SIZE - 1);

pub unsafe fn acpi_nvs_register(start: u64, size: u64) -> i32 {
    let region = kmalloc_obj::<NvsRegion>();
    if region.is_null() {
        return -ENOMEM;
    }
    (*region).phys_start = start;
    (*region).size = size;
    list_add_tail(&mut (*region).node, &mut nvs_region_list);

    suspend_nvs_register(start as usize, size as usize)
}

pub unsafe fn acpi_nvs_for_each_region(
    func: unsafe extern "C" fn(u64, u64, *mut core::ffi::c_void) -> i32,
    data: *mut core::ffi::c_void,
) -> i32 {
    let mut node = nvs_region_list.next;
    while node != &mut nvs_region_list {
        let region = node as *mut NvsRegion;
        let rc = func((*region).phys_start, (*region).size, data);
        if rc != 0 {
            return rc;
        }
        node = (*node).next;
    }
    0
}

pub unsafe fn suspend_nvs_free(nvs_list: *mut ListHead) {
    let mut node = (*nvs_list).next;
    while node != nvs_list {
        let entry = node as *mut NvsPage;
        node = (*node).next;
        if !(*entry).data.is_null() {
            kfree((*entry).data);
            (*entry).data = core::ptr::null_mut();
            if !(*entry).kaddr.is_null() {
                if (*entry).unmap {
                    iounmap((*entry).kaddr);
                    (*entry).unmap = false;
                } else {
                    acpi_os_unmap_iomem((*entry).kaddr, (*entry).size);
                }
                (*entry).kaddr = core::ptr::null_mut();
            }
        }
    }
}

pub unsafe fn suspend_nvs_alloc(nvs_list: *mut ListHead) -> i32 {
    let mut node = (*nvs_list).next;
    while node != nvs_list {
        let entry = node as *mut NvsPage;
        (*entry).data = kmalloc(PAGE_SIZE, GFP_KERNEL);
        if (*entry).data.is_null() {
            suspend_nvs_free(nvs_list);
            return -ENOMEM;
        }
        node = (*node).next;
    }
    0
}

pub unsafe fn suspend_nvs_save(nvs_list: *mut ListHead) -> i32 {
    let mut node = (*nvs_list).next;
    while node != nvs_list {
        let entry = node as *mut NvsPage;
        if !(*entry).data.is_null() {
            let phys = (*entry).phys_start;
            let size = (*entry).size;
            (*entry).kaddr = acpi_os_get_iomem(phys, size);
            if (*entry).kaddr.is_null() {
                (*entry).kaddr = acpi_os_ioremap(phys, size);
                (*entry).unmap = !(*entry).kaddr.is_null();
            }
            if (*entry).kaddr.is_null() {
                suspend_nvs_free(nvs_list);
                return -ENOMEM;
            }
            memcpy((*entry).data, (*entry).kaddr, (*entry).size as usize);
        }
        node = (*node).next;
    }
    0
}

pub unsafe fn suspend_nvs_restore(nvs_list: *mut ListHead) {
    let mut node = (*nvs_list).next;
    while node != nvs_list {
        let entry = node as *mut NvsPage;
        if !(*entry).data.is_null() {
            memcpy((*entry).kaddr, (*entry).data, (*entry).size as usize);
        }
        node = (*node).next;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
