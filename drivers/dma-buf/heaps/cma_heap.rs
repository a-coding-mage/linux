// SPDX-License-Identifier: GPL-2.0
/*
 * DMABUF CMA heap exporter
 *
 * Copyright (C) 2012, 2019, 2020 Linaro Ltd.
 * Author: <benjamin.gaignard@linaro.org> for ST-Ericsson.
 *
 * Also utilizing parts of Andrew Davis' SRAM heap:
 * Copyright (C) 2019 Texas Instruments Incorporated - http://www.ti.com/
 * Andrew F. Davis <afd@ti.com>
 */

// C dependencies: linux/cma.h, dma-buf.h, dma-heap.h, dma-map-ops.h, err.h,
// highmem.h, io.h, mm.h, module.h, of.h, of_reserved_mem.h, scatterlist.h,
// slab.h, vmalloc.h.

pub const DEFAULT_CMA_NAME: &[u8] = b"default_cma_region\0";

#[repr(C)]
pub struct cma_heap {
    pub heap: *mut dma_heap,
    pub cma: *mut cma,
}

#[repr(C)]
pub struct cma_heap_buffer {
    pub heap: *mut cma_heap,
    pub attachments: list_head,
    pub lock: mutex,
    pub len: c_ulong,
    pub cma_pages: *mut page,
    pub pages: *mut *mut page,
    pub pagecount: pgoff_t,
    pub vmap_cnt: c_int,
    pub vaddr: *mut c_void,
}

#[repr(C)]
pub struct dma_heap_attachment {
    pub dev: *mut device,
    pub table: sg_table,
    pub list: list_head,
    pub mapped: bool,
}

unsafe fn cma_heap_attach(dmabuf: *mut dma_buf, attachment: *mut dma_buf_attachment) -> c_int {
    let buffer = (*dmabuf).priv_ as *mut cma_heap_buffer;
    let a = kzalloc_obj::<dma_heap_attachment>();
    if a.is_null() { return -ENOMEM; }

    let ret = sg_alloc_table_from_pages(&mut (*a).table, (*buffer).pages, (*buffer).pagecount,
        0, (*buffer).pagecount << PAGE_SHIFT, GFP_KERNEL);
    if ret != 0 { kfree(a as *mut c_void); return ret; }

    (*a).dev = (*attachment).dev;
    INIT_LIST_HEAD(&mut (*a).list);
    (*a).mapped = false;
    (*attachment).priv_ = a as *mut c_void;
    mutex_lock(&mut (*buffer).lock);
    list_add(&mut (*a).list, &mut (*buffer).attachments);
    mutex_unlock(&mut (*buffer).lock);
    0
}

unsafe fn cma_heap_detach(dmabuf: *mut dma_buf, attachment: *mut dma_buf_attachment) {
    let buffer = (*dmabuf).priv_ as *mut cma_heap_buffer;
    let a = (*attachment).priv_ as *mut dma_heap_attachment;
    mutex_lock(&mut (*buffer).lock);
    list_del(&mut (*a).list);
    mutex_unlock(&mut (*buffer).lock);
    sg_free_table(&mut (*a).table);
    kfree(a as *mut c_void);
}

unsafe fn cma_heap_map_dma_buf(attachment: *mut dma_buf_attachment, direction: dma_data_direction) -> *mut sg_table {
    let a = (*attachment).priv_ as *mut dma_heap_attachment;
    let table = &mut (*a).table;
    let ret = dma_map_sgtable((*attachment).dev, table, direction, 0);
    if ret != 0 { return ERR_PTR(-ENOMEM); }
    (*a).mapped = true;
    table
}

unsafe fn cma_heap_unmap_dma_buf(attachment: *mut dma_buf_attachment, table: *mut sg_table, direction: dma_data_direction) {
    let a = (*attachment).priv_ as *mut dma_heap_attachment;
    (*a).mapped = false;
    dma_unmap_sgtable((*attachment).dev, table, direction, 0);
}

unsafe fn cma_heap_dma_buf_begin_cpu_access(dmabuf: *mut dma_buf, direction: dma_data_direction) -> c_int {
    let buffer = (*dmabuf).priv_ as *mut cma_heap_buffer;
    mutex_lock(&mut (*buffer).lock);
    if (*buffer).vmap_cnt != 0 { invalidate_kernel_vmap_range((*buffer).vaddr, (*buffer).len); }
    let mut a: *mut dma_heap_attachment = core::ptr::null_mut();
    list_for_each_entry!(a, &mut (*buffer).attachments, list, dma_heap_attachment {
        if !(*a).mapped { continue; }
        dma_sync_sgtable_for_cpu((*a).dev, &mut (*a).table, direction);
    });
    mutex_unlock(&mut (*buffer).lock);
    0
}

unsafe fn cma_heap_dma_buf_end_cpu_access(dmabuf: *mut dma_buf, direction: dma_data_direction) -> c_int {
    let buffer = (*dmabuf).priv_ as *mut cma_heap_buffer;
    mutex_lock(&mut (*buffer).lock);
    if (*buffer).vmap_cnt != 0 { flush_kernel_vmap_range((*buffer).vaddr, (*buffer).len); }
    let mut a: *mut dma_heap_attachment = core::ptr::null_mut();
    list_for_each_entry!(a, &mut (*buffer).attachments, list, dma_heap_attachment {
        if !(*a).mapped { continue; }
        dma_sync_sgtable_for_device((*a).dev, &mut (*a).table, direction);
    });
    mutex_unlock(&mut (*buffer).lock);
    0
}

unsafe fn cma_heap_vm_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let vma = (*vmf).vma;
    let buffer = (*vma).vm_private_data as *mut cma_heap_buffer;
    if (*vmf).pgoff >= (*buffer).pagecount { return VM_FAULT_SIGBUS; }
    vmf_insert_pfn(vma, (*vmf).address, page_to_pfn(*(*buffer).pages.add((*vmf).pgoff as usize)))
}

pub static dma_heap_vm_ops: vm_operations_struct = vm_operations_struct { fault: Some(cma_heap_vm_fault) };

unsafe fn cma_heap_mmap(dmabuf: *mut dma_buf, vma: *mut vm_area_struct) -> c_int {
    let buffer = (*dmabuf).priv_ as *mut cma_heap_buffer;
    if ((*vma).vm_flags & (VM_SHARED | VM_MAYSHARE)) == 0 { return -EINVAL; }
    vm_flags_set(vma, VM_IO | VM_PFNMAP | VM_DONTEXPAND | VM_DONTDUMP);
    (*vma).vm_ops = &dma_heap_vm_ops;
    (*vma).vm_private_data = buffer as *mut c_void;
    0
}

unsafe fn cma_heap_do_vmap(buffer: *mut cma_heap_buffer) -> *mut c_void {
    let vaddr = vmap((*buffer).pages, (*buffer).pagecount, VM_MAP, PAGE_KERNEL);
    if vaddr.is_null() { return ERR_PTR(-ENOMEM); }
    vaddr
}

unsafe fn cma_heap_vmap(dmabuf: *mut dma_buf, map: *mut iosys_map) -> c_int {
    let buffer = (*dmabuf).priv_ as *mut cma_heap_buffer;
    let mut ret = 0;
    mutex_lock(&mut (*buffer).lock);
    if (*buffer).vmap_cnt != 0 {
        (*buffer).vmap_cnt += 1;
        iosys_map_set_vaddr(map, (*buffer).vaddr);
    } else {
        let vaddr = cma_heap_do_vmap(buffer);
        if IS_ERR(vaddr) { ret = PTR_ERR(vaddr); } else {
            (*buffer).vaddr = vaddr;
            (*buffer).vmap_cnt += 1;
            iosys_map_set_vaddr(map, (*buffer).vaddr);
        }
    }
    mutex_unlock(&mut (*buffer).lock);
    ret
}

unsafe fn cma_heap_vunmap(dmabuf: *mut dma_buf, map: *mut iosys_map) {
    let buffer = (*dmabuf).priv_ as *mut cma_heap_buffer;
    mutex_lock(&mut (*buffer).lock);
    (*buffer).vmap_cnt -= 1;
    if (*buffer).vmap_cnt == 0 { vunmap((*buffer).vaddr); (*buffer).vaddr = core::ptr::null_mut(); }
    mutex_unlock(&mut (*buffer).lock);
    iosys_map_clear(map);
}

unsafe fn cma_heap_dma_buf_release(dmabuf: *mut dma_buf) {
    let buffer = (*dmabuf).priv_ as *mut cma_heap_buffer;
    let cma_heap = (*buffer).heap;
    if (*buffer).vmap_cnt > 0 { WARN!(1, "%s: buffer still mapped in the kernel\n", __func__); vunmap((*buffer).vaddr); (*buffer).vaddr = core::ptr::null_mut(); }
    kfree((*buffer).pages as *mut c_void);
    cma_release((*cma_heap).cma, (*buffer).cma_pages, (*buffer).pagecount);
    kfree(buffer as *mut c_void);
}

pub static cma_heap_buf_ops: dma_buf_ops = dma_buf_ops {
    attach: Some(cma_heap_attach), detach: Some(cma_heap_detach), map_dma_buf: Some(cma_heap_map_dma_buf),
    unmap_dma_buf: Some(cma_heap_unmap_dma_buf), begin_cpu_access: Some(cma_heap_dma_buf_begin_cpu_access),
    end_cpu_access: Some(cma_heap_dma_buf_end_cpu_access), mmap: Some(cma_heap_mmap), vmap: Some(cma_heap_vmap),
    vunmap: Some(cma_heap_vunmap), release: Some(cma_heap_dma_buf_release),
};

unsafe fn cma_heap_allocate(heap: *mut dma_heap, len: c_ulong, fd_flags: u32, heap_flags: u64) -> *mut dma_buf {
    let cma_heap = dma_heap_get_drvdata(heap) as *mut cma_heap;
    let buffer = kzalloc_obj::<cma_heap_buffer>();
    if buffer.is_null() { return ERR_PTR(-ENOMEM); }
    INIT_LIST_HEAD(&mut (*buffer).attachments);
    mutex_init(&mut (*buffer).lock);
    let size = PAGE_ALIGN(len as usize);
    let pagecount = (size >> PAGE_SHIFT) as pgoff_t;
    let mut align = get_order(size);
    (*buffer).len = size as c_ulong;
    if align > CONFIG_CMA_ALIGNMENT { align = CONFIG_CMA_ALIGNMENT; }
    let cma_pages = cma_alloc((*cma_heap).cma, pagecount, align, false);
    if cma_pages.is_null() { kfree(buffer as *mut c_void); return ERR_PTR(-ENOMEM); }
    if PageHighMem(cma_pages) {
        let mut nr_clear_pages = pagecount;
        let mut page = cma_pages;
        while nr_clear_pages > 0 {
            clear_highpage(page);
            if fatal_signal_pending(current) { cma_release((*cma_heap).cma, cma_pages, pagecount); kfree(buffer as *mut c_void); return ERR_PTR(-ENOMEM); }
            page = page.add(1); nr_clear_pages -= 1;
        }
    } else { clear_pages(page_address(cma_pages), pagecount); }
    (*buffer).pages = kmalloc_objs::<*mut page>(pagecount);
    if (*buffer).pages.is_null() { cma_release((*cma_heap).cma, cma_pages, pagecount); kfree(buffer as *mut c_void); return ERR_PTR(-ENOMEM); }
    for pg in 0..pagecount as usize { *(*buffer).pages.add(pg) = cma_pages.add(pg); }
    (*buffer).cma_pages = cma_pages; (*buffer).heap = cma_heap; (*buffer).pagecount = pagecount;
    let mut exp_info = dma_buf_export_info::default();
    exp_info.exp_name = dma_heap_get_name(heap); exp_info.ops = &cma_heap_buf_ops; exp_info.size = (*buffer).len as usize;
    exp_info.flags = fd_flags; exp_info.priv_ = buffer as *mut c_void;
    let dmabuf = dma_buf_export(&mut exp_info);
    if IS_ERR(dmabuf) { kfree((*buffer).pages as *mut c_void); cma_release((*cma_heap).cma, cma_pages, pagecount); kfree(buffer as *mut c_void); }
    dmabuf
}

pub static cma_heap_ops: dma_heap_ops = dma_heap_ops { allocate: Some(cma_heap_allocate) };

unsafe fn __add_cma_heap(cma: *mut cma, name: *const c_char) -> c_int {
    let cma_heap = kzalloc_obj::<cma_heap>();
    if cma_heap.is_null() { return -ENOMEM; }
    (*cma_heap).cma = cma;
    let mut exp_info = dma_heap_export_info::default();
    exp_info.name = name; exp_info.ops = &cma_heap_ops; exp_info.priv_ = cma_heap as *mut c_void;
    (*cma_heap).heap = dma_heap_add(&mut exp_info);
    if IS_ERR((*cma_heap).heap) { let ret = PTR_ERR((*cma_heap).heap); kfree(cma_heap as *mut c_void); return ret; }
    0
}

unsafe fn add_cma_heaps() -> c_int {
    let default_cma = dev_get_cma_area(core::ptr::null_mut());
    if !default_cma.is_null() { let ret = __add_cma_heap(default_cma, DEFAULT_CMA_NAME.as_ptr() as *const c_char); if ret != 0 { return ret; } }
    let mut i = 0;
    loop {
        let cma = dma_contiguous_get_area_by_idx(i); if cma.is_null() { break; }
        let ret = __add_cma_heap(cma, cma_get_name(cma));
        if ret != 0 { pr_warn!("Failed to add CMA heap %s", cma_get_name(cma)); }
        i += 1;
    }
    0
}

module_init!(add_cma_heaps);
MODULE_DESCRIPTION!("DMA-BUF CMA Heap");
MODULE_LICENSE!("GPL");
MODULE_IMPORT_NS!("DMA_BUF");
MODULE_IMPORT_NS!("DMA_BUF_HEAP");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
