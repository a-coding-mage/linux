// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2014-2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// External kernel/KFD declarations and constants are supplied by surrounding dependencies.

pub unsafe fn kfd_doorbell_process_slice(kfd: *mut kfd_dev) -> usize {
    if !(*kfd).shared_resources.enable_mes {
        roundup((*kfd).device_info.doorbell_size * KFD_MAX_NUM_OF_QUEUES_PER_PROCESS, PAGE_SIZE)
    } else {
        amdgpu_mes_doorbell_process_slice((*kfd).adev as *mut amdgpu_device)
    }
}

pub unsafe fn kfd_doorbell_init(kfd: *mut kfd_dev) -> i32 {
    let size: i32 = PAGE_SIZE as i32;
    let mut r: i32;

    (*kfd).doorbell_bitmap = bitmap_zalloc(size as usize / core::mem::size_of::<u32>(), GFP_KERNEL);
    if (*kfd).doorbell_bitmap.is_null() {
        DRM_ERROR("Failed to allocate kernel doorbell bitmap\n");
        return -ENOMEM;
    }

    r = amdgpu_bo_create_kernel(
        (*kfd).adev, size as usize, PAGE_SIZE, AMDGPU_GEM_DOMAIN_DOORBELL,
        &mut (*kfd).doorbells, core::ptr::null_mut(),
        &mut (*kfd).doorbell_kernel_ptr as *mut _ as *mut *mut core::ffi::c_void,
    );
    if r != 0 {
        pr_err("failed to allocate kernel doorbells\n");
        bitmap_free((*kfd).doorbell_bitmap);
        return r;
    }
    pr_debug("Doorbell kernel address == %p\n", (*kfd).doorbell_kernel_ptr);
    0
}

pub unsafe fn kfd_doorbell_fini(kfd: *mut kfd_dev) {
    bitmap_free((*kfd).doorbell_bitmap);
    amdgpu_bo_free_kernel(&mut (*kfd).doorbells, core::ptr::null_mut(),
                          &mut (*kfd).doorbell_kernel_ptr as *mut _ as *mut *mut core::ffi::c_void);
}

pub unsafe fn kfd_doorbell_mmap(dev: *mut kfd_node, process: *mut kfd_process,
                                vma: *mut vm_area_struct) -> i32 {
    if (*vma).vm_end - (*vma).vm_start != kfd_doorbell_process_slice((*dev).kfd) {
        return -EINVAL;
    }
    let pdd = kfd_get_process_device_data(dev, process);
    if pdd.is_null() { return -EINVAL; }
    let address = kfd_get_process_doorbells(pdd);
    if address == 0 { return -ENOMEM; }
    vm_flags_set(vma, VM_IO | VM_DONTCOPY | VM_DONTEXPAND | VM_NORESERVE | VM_DONTDUMP | VM_PFNMAP);
    (*vma).vm_page_prot = pgprot_noncached((*vma).vm_page_prot);
    pr_debug("Mapping doorbell page\n     target user address == 0x%08llX\n     physical address    == 0x%08llX\n     vm_flags            == 0x%04lX\n     size                == 0x%04lX\n",
             (*vma).vm_start as u64, address, (*vma).vm_flags,
             kfd_doorbell_process_slice((*dev).kfd));
    io_remap_pfn_range(vma, (*vma).vm_start, address >> PAGE_SHIFT,
                       kfd_doorbell_process_slice((*dev).kfd), (*vma).vm_page_prot)
}

pub unsafe fn kfd_get_kernel_doorbell(kfd: *mut kfd_dev, doorbell_off: *mut u32) -> *mut core::ffi::c_void {
    mutex_lock(&mut (*kfd).doorbell_mutex);
    let mut inx = find_first_zero_bit((*kfd).doorbell_bitmap, PAGE_SIZE / core::mem::size_of::<u32>());
    if inx >= KFD_MAX_NUM_OF_QUEUES_PER_PROCESS { mutex_unlock(&mut (*kfd).doorbell_mutex); return core::ptr::null_mut(); }
    __set_bit(inx, (*kfd).doorbell_bitmap);
    mutex_unlock(&mut (*kfd).doorbell_mutex);
    *doorbell_off = amdgpu_doorbell_index_on_bar((*kfd).adev, (*kfd).doorbells, inx, (*kfd).device_info.doorbell_size);
    inx *= 2;
    pr_debug("Get kernel queue doorbell\n     doorbell offset   == 0x%08X\n     doorbell index    == 0x%x\n", *doorbell_off, inx);
    (*kfd).doorbell_kernel_ptr.add(inx as usize) as *mut core::ffi::c_void
}

pub unsafe fn kfd_release_kernel_doorbell(kfd: *mut kfd_dev, db_addr: *mut u32) {
    let mut inx = db_addr.offset_from((*kfd).doorbell_kernel_ptr) as u32;
    inx /= 2;
    mutex_lock(&mut (*kfd).doorbell_mutex);
    __clear_bit(inx, (*kfd).doorbell_bitmap);
    mutex_unlock(&mut (*kfd).doorbell_mutex);
}

pub unsafe fn write_kernel_doorbell(db: *mut core::ffi::c_void, value: u32) {
    if !db.is_null() { writel(value, db); pr_debug("Writing %d to doorbell address %p\n", value, db); }
}

pub unsafe fn write_kernel_doorbell64(db: *mut core::ffi::c_void, value: u64) {
    if !db.is_null() { WARN((db as usize & 7) != 0, "Unaligned 64-bit doorbell"); writeq(value, db as *mut u64); pr_debug("writing %llu to doorbell address %p\n", value, db); }
}

unsafe fn init_doorbell_bitmap(qpd: *mut qcm_process_device, dev: *mut kfd_dev) -> i32 {
    let range_start = (*dev).shared_resources.non_cp_doorbells_start;
    let range_end = (*dev).shared_resources.non_cp_doorbells_end;
    if !KFD_IS_SOC15(dev) { return 0; }
    pr_debug("reserved doorbell 0x%03x - 0x%03x\n", range_start, range_end);
    pr_debug("reserved doorbell 0x%03x - 0x%03x\n", range_start + KFD_QUEUE_DOORBELL_MIRROR_OFFSET, range_end + KFD_QUEUE_DOORBELL_MIRROR_OFFSET);
    for i in 0..(KFD_MAX_NUM_OF_QUEUES_PER_PROCESS / 2) { if i >= range_start && i <= range_end { __set_bit(i, (*qpd).doorbell_bitmap); __set_bit(i + KFD_QUEUE_DOORBELL_MIRROR_OFFSET, (*qpd).doorbell_bitmap); } }
    0
}

pub unsafe fn kfd_get_process_doorbells(pdd: *mut kfd_process_device) -> phys_addr_t {
    let adev = (*(*pdd).dev).adev;
    if (*pdd).qpd.proc_doorbells.is_null() && kfd_alloc_process_doorbells((*pdd).dev.kfd, pdd) != 0 { return 0; }
    let first_db_index = amdgpu_doorbell_index_on_bar(adev, (*pdd).qpd.proc_doorbells, 0, (*pdd).dev.kfd.device_info.doorbell_size);
    (*adev).doorbell.base + first_db_index * core::mem::size_of::<u32>()
}

pub unsafe fn kfd_alloc_process_doorbells(kfd: *mut kfd_dev, pdd: *mut kfd_process_device) -> i32 {
    let qpd = &mut (*pdd).qpd;
    qpd.doorbell_bitmap = bitmap_zalloc(KFD_MAX_NUM_OF_QUEUES_PER_PROCESS, GFP_KERNEL);
    if qpd.doorbell_bitmap.is_null() { DRM_ERROR("Failed to allocate process doorbell bitmap\n"); return -ENOMEM; }
    let mut r = init_doorbell_bitmap(qpd, kfd);
    if r != 0 { DRM_ERROR("Failed to initialize process doorbells\n"); r = -ENOMEM; goto_err(qpd); return r; }
    r = amdgpu_bo_create_kernel((*kfd).adev, kfd_doorbell_process_slice(kfd), PAGE_SIZE, AMDGPU_GEM_DOMAIN_DOORBELL, &mut qpd.proc_doorbells, core::ptr::null_mut(), core::ptr::null_mut());
    if r == 0 { return 0; }
    DRM_ERROR("Failed to allocate process doorbells\n"); goto_err(qpd); r
}

unsafe fn goto_err(qpd: &mut qcm_process_device) { bitmap_free(qpd.doorbell_bitmap); qpd.doorbell_bitmap = core::ptr::null_mut(); }

pub unsafe fn kfd_free_process_doorbells(_kfd: *mut kfd_dev, pdd: *mut kfd_process_device) {
    let qpd = &mut (*pdd).qpd;
    if !qpd.doorbell_bitmap.is_null() { bitmap_free(qpd.doorbell_bitmap); qpd.doorbell_bitmap = core::ptr::null_mut(); }
    amdgpu_bo_free_kernel(&mut qpd.proc_doorbells, core::ptr::null_mut(), core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
