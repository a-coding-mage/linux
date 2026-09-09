/*
 * AGPGART driver backend routines.
 * Copyright (C) 2004 Silicon Graphics, Inc.
 * Copyright (C) 2002-2003 Dave Jones.
 * Copyright (C) 1999 Jeff Hartmann.
 * Copyright (C) 1999 Precision Insight, Inc.
 * Copyright (C) 1999 Xi Graphics, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * JEFF HARTMANN, DAVE JONES, OR ANY OTHER CONTRIBUTORS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
 * OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * TODO:
 * - Allocate more than order 0 pages to avoid too much linear map splitting.
 */

// Linux kernel dependencies supplied by other translation units.

pub const AGPGART_VERSION_MAJOR: u32 = 0;
pub const AGPGART_VERSION_MINOR: u32 = 103;

pub static AGP_CURRENT_VERSION: agp_version = agp_version {
    major: AGPGART_VERSION_MAJOR,
    minor: AGPGART_VERSION_MINOR,
};

pub static mut agp_find_bridge: Option<unsafe extern "C" fn(*mut pci_dev) -> *mut agp_bridge_data> =
    Some(agp_generic_find_bridge);

pub static mut agp_bridge: *mut agp_bridge_data = core::ptr::null_mut();
// Corresponds to LIST_HEAD(agp_bridges).
pub static mut agp_bridges: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

pub unsafe fn agp_backend_acquire(pdev: *mut pci_dev) -> *mut agp_bridge_data {
    let bridge = (agp_find_bridge.expect("agp_find_bridge"))(pdev);

    if bridge.is_null() {
        return core::ptr::null_mut();
    }

    if atomic_read(&(*bridge).agp_in_use) != 0 {
        return core::ptr::null_mut();
    }
    atomic_inc(&mut (*bridge).agp_in_use);
    bridge
}

pub unsafe fn agp_backend_release(bridge: *mut agp_bridge_data) {
    if !bridge.is_null() {
        atomic_dec(&mut (*bridge).agp_in_use);
    }
}

#[repr(C)]
struct maxes_entry { mem: i32, agp: i32 }

static MAXES_TABLE: [maxes_entry; 9] = [
    maxes_entry { mem: 0, agp: 0 },
    maxes_entry { mem: 32, agp: 4 },
    maxes_entry { mem: 64, agp: 28 },
    maxes_entry { mem: 128, agp: 96 },
    maxes_entry { mem: 256, agp: 204 },
    maxes_entry { mem: 512, agp: 440 },
    maxes_entry { mem: 1024, agp: 942 },
    maxes_entry { mem: 2048, agp: 1920 },
    maxes_entry { mem: 4096, agp: 3932 },
];

unsafe fn agp_find_max() -> i32 {
    let mut memory: isize;
    #[cfg(target_pointer_width = "64")]
    { memory = (totalram_pages() as isize) >> (20 - PAGE_SHIFT); }
    #[cfg(not(target_pointer_width = "64"))]
    { memory = (totalram_pages() as isize) << (PAGE_SHIFT - 20); }
    let mut index: usize = 1;

    while memory > MAXES_TABLE[index].mem as isize && index < 8 {
        index += 1;
    }

    let result = MAXES_TABLE[index - 1].agp as isize
        + ((memory - MAXES_TABLE[index - 1].mem as isize)
            * (MAXES_TABLE[index].agp - MAXES_TABLE[index - 1].agp) as isize)
            / (MAXES_TABLE[index].mem - MAXES_TABLE[index - 1].mem) as isize;
    (result << (20 - PAGE_SHIFT)) as i32
}

unsafe fn agp_backend_initialize(bridge: *mut agp_bridge_data) -> i32 {
    let mut size_value: i32;
    let rc: i32;
    let mut got_gatt = false;
    let mut got_keylist = false;

    (*bridge).max_memory_agp = agp_find_max();
    (*bridge).version = &AGP_CURRENT_VERSION;

    if (*(*bridge).driver).needs_scratch_page {
        let page = ((*(*bridge).driver).agp_alloc_page.expect("agp_alloc_page"))(bridge);
        if page.is_null() { return -ENOMEM; }
        (*bridge).scratch_page_page = page;
        (*bridge).scratch_page_dma = page_to_phys(page);
        (*bridge).scratch_page = ((*(*bridge).driver).mask_memory.expect("mask_memory"))(bridge, (*bridge).scratch_page_dma, 0);
    }

    size_value = ((*(*bridge).driver).fetch_size.expect("fetch_size"))();
    if size_value == 0 { return -EINVAL; }
    if ((*(*bridge).driver).create_gatt_table.expect("create_gatt_table"))(bridge) != 0 { return -ENOMEM; }
    got_gatt = true;

    (*bridge).key_list = vzalloc(PAGE_SIZE * 4);
    if (*bridge).key_list.is_null() { return -ENOMEM; }
    got_keylist = true;

    if ((*(*bridge).driver).configure.expect("configure"))() != 0 { return -EINVAL; }
    INIT_LIST_HEAD(&mut (*bridge).mapped_list);
    spin_lock_init(&mut (*bridge).mapped_lock);
    return 0;

    // C's err_out cleanup is retained below for source-level control flow.
    #[allow(unreachable_code)]
    {
        if (*(*bridge).driver).needs_scratch_page {
            let page = (*bridge).scratch_page_page;
            ((*(*bridge).driver).agp_destroy_page.expect("agp_destroy_page"))(page, AGP_PAGE_DESTROY_UNMAP);
            ((*(*bridge).driver).agp_destroy_page.expect("agp_destroy_page"))(page, AGP_PAGE_DESTROY_FREE);
        }
        if got_gatt { ((*(*bridge).driver).free_gatt_table.expect("free_gatt_table"))(bridge); }
        if got_keylist { vfree((*bridge).key_list); (*bridge).key_list = core::ptr::null_mut(); }
        rc
    }
}

unsafe fn agp_backend_cleanup(bridge: *mut agp_bridge_data) {
    if let Some(cleanup) = (*(*bridge).driver).cleanup { cleanup(); }
    if let Some(free) = (*(*bridge).driver).free_gatt_table { free(bridge); }
    vfree((*bridge).key_list);
    (*bridge).key_list = core::ptr::null_mut();
    if let Some(destroy) = (*(*bridge).driver).agp_destroy_page {
        if (*(*bridge).driver).needs_scratch_page {
            destroy((*bridge).scratch_page_page, AGP_PAGE_DESTROY_UNMAP);
            destroy((*bridge).scratch_page_page, AGP_PAGE_DESTROY_FREE);
        }
    }
}

pub unsafe fn agp_alloc_bridge() -> *mut agp_bridge_data {
    let bridge = kzalloc_obj::<agp_bridge_data>();
    if bridge.is_null() { return core::ptr::null_mut(); }
    atomic_set(&mut (*bridge).agp_in_use, 0);
    atomic_set(&mut (*bridge).current_memory_agp, 0);
    if list_empty(&agp_bridges) { agp_bridge = bridge; }
    bridge
}

pub unsafe fn agp_put_bridge(bridge: *mut agp_bridge_data) {
    kfree(bridge);
    if list_empty(&agp_bridges) { agp_bridge = core::ptr::null_mut(); }
}

pub unsafe fn agp_add_bridge(bridge: *mut agp_bridge_data) -> i32 {
    if agp_off != 0 { agp_put_bridge(bridge); return -ENODEV; }
    if (*bridge).dev.is_null() { agp_put_bridge(bridge); return -EINVAL; }
    if !try_module_get((*(*bridge).driver).owner) { agp_put_bridge(bridge); return -EINVAL; }
    let error = agp_backend_initialize(bridge);
    if error != 0 { module_put((*(*bridge).driver).owner); agp_put_bridge(bridge); return error; }
    list_add(&mut (*bridge).list, &mut agp_bridges);
    0
}

pub unsafe fn agp_remove_bridge(bridge: *mut agp_bridge_data) {
    agp_backend_cleanup(bridge);
    list_del(&mut (*bridge).list);
    module_put((*(*bridge).driver).owner);
}

pub static mut agp_off: i32 = 0;
pub static mut agp_try_unsupported_boot: i32 = 0;

#[cfg(not(feature = "module"))]
pub unsafe fn agp_setup(s: *const i8) -> i32 {
    if strcmp(s, b"off\0".as_ptr() as *const i8) == 0 { agp_off = 1; }
    if strcmp(s, b"try_unsupported\0".as_ptr() as *const i8) == 0 { agp_try_unsupported_boot = 1; }
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
