/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Author(s)......: Holger Smolinski <Holger.Smolinski@de.ibm.com>
 *                  Martin Schwidefsky <schwidefsky@de.ibm.com>
 * Bugreports.to..: <Linux390@de.ibm.com>
 * Copyright IBM Corp. 2000
 *
 * History of changes
 * 07/24/00 new file
 * 05/04/02 code restructuring.
 */

pub const IDA_SIZE_SHIFT: usize = 12;
pub const IDA_BLOCK_SIZE: usize = 1usize << IDA_SIZE_SHIFT;
pub const IDA_2K_SIZE_SHIFT: usize = 11;
pub const IDA_2K_BLOCK_SIZE: usize = 1usize << IDA_2K_SIZE_SHIFT;

#[inline]
pub unsafe fn idal_is_needed(vaddr: *mut core::ffi::c_void, length: u32) -> bool {
    let paddr: dma64_t = virt_to_dma64(vaddr);
    (((paddr as usize).wrapping_add(length as usize).wrapping_sub(1)) >> 31) != 0
}

#[inline]
pub unsafe fn idal_nr_words(vaddr: *mut core::ffi::c_void, length: u32) -> u32 {
    let mut cidaw = (vaddr as usize & (IDA_BLOCK_SIZE - 1)) as u32;
    cidaw = cidaw.wrapping_add(length).wrapping_add((IDA_BLOCK_SIZE - 1) as u32);
    cidaw >> IDA_SIZE_SHIFT
}

#[inline]
pub unsafe fn idal_2k_nr_words(vaddr: *mut core::ffi::c_void, length: u32) -> u32 {
    let mut cidaw = (vaddr as usize & (IDA_2K_BLOCK_SIZE - 1)) as u32;
    cidaw = cidaw.wrapping_add(length).wrapping_add((IDA_2K_BLOCK_SIZE - 1) as u32);
    cidaw >> IDA_2K_SIZE_SHIFT
}

#[inline]
pub unsafe fn idal_create_words(mut idaws: *mut dma64_t, vaddr: *mut core::ffi::c_void, length: u32) -> *mut dma64_t {
    let mut paddr = virt_to_dma64(vaddr);
    *idaws = paddr;
    idaws = idaws.add(1);
    let mut cidaw = idal_nr_words(vaddr, length);
    paddr = dma64_and(paddr, -(IDA_BLOCK_SIZE as i64) as dma64_t);
    while { cidaw = cidaw.wrapping_sub(1); cidaw > 0 } {
        paddr = dma64_add(paddr, IDA_BLOCK_SIZE as dma64_t);
        *idaws = paddr;
        idaws = idaws.add(1);
    }
    idaws
}

#[inline]
pub unsafe fn set_normalized_cda(ccw: *mut ccw1, mut vaddr: *mut core::ffi::c_void) -> i32 {
    if (*ccw).flags & CCW_FLAG_IDA != 0 { return -EINVAL; }
    let nridaws = idal_nr_words(vaddr, (*ccw).count);
    if nridaws > 0 {
        let idal = kzalloc_objs::<dma64_t>(nridaws, GFP_ATOMIC | GFP_DMA);
        if idal.is_null() { return -ENOMEM; }
        idal_create_words(idal, vaddr, (*ccw).count);
        (*ccw).flags |= CCW_FLAG_IDA;
        vaddr = idal.cast();
    }
    (*ccw).cda = virt_to_dma32(vaddr);
    0
}

#[inline]
pub unsafe fn clear_normalized_cda(ccw: *mut ccw1) {
    if (*ccw).flags & CCW_FLAG_IDA != 0 {
        kfree(dma32_to_virt((*ccw).cda));
        (*ccw).flags &= !CCW_FLAG_IDA;
    }
    (*ccw).cda = 0;
}

#[repr(C)]
pub struct idal_buffer {
    pub size: usize,
    pub page_order: usize,
    pub data: [dma64_t; 0],
}

#[inline]
pub unsafe fn idal_buffer_alloc(size: usize, page_order: i32) -> *mut idal_buffer {
    let nr_ptrs = (size + IDA_BLOCK_SIZE - 1) >> IDA_SIZE_SHIFT;
    let nr_chunks = (PAGE_SIZE << page_order) >> IDA_SIZE_SHIFT;
    let ib = kmalloc_flex::<idal_buffer, dma64_t>(nr_ptrs, GFP_DMA | GFP_KERNEL);
    if ib.is_null() { return ERR_PTR(-ENOMEM); }
    (*ib).size = size;
    (*ib).page_order = page_order as usize;
    let mut i = 0usize;
    while i < nr_ptrs {
        if i & (nr_chunks - 1) != 0 {
            (*ib).data.as_mut_ptr().add(i).write(dma64_add((*ib).data.as_ptr().add(i - 1).read(), IDA_BLOCK_SIZE as dma64_t));
        } else {
            let vaddr = __get_free_pages(GFP_KERNEL, page_order);
            if vaddr.is_null() { break; }
            (*ib).data.as_mut_ptr().add(i).write(virt_to_dma64(vaddr));
        }
        i += 1;
    }
    if i == nr_ptrs { return ib; }
    while i >= nr_chunks {
        i -= nr_chunks;
        let vaddr = dma64_to_virt((*ib).data.as_ptr().add(i).read());
        free_pages(vaddr as usize, (*ib).page_order);
    }
    kfree(ib.cast());
    ERR_PTR(-ENOMEM)
}

#[inline]
pub unsafe fn idal_buffer_free(ib: *mut idal_buffer) {
    let nr_ptrs = ((*ib).size + IDA_BLOCK_SIZE - 1) >> IDA_SIZE_SHIFT;
    let nr_chunks = (PAGE_SIZE << (*ib).page_order) >> IDA_SIZE_SHIFT;
    let mut i = 0usize;
    while i < nr_ptrs {
        free_pages(dma64_to_virt((*ib).data.as_ptr().add(i).read()) as usize, (*ib).page_order);
        i += nr_chunks;
    }
    kfree(ib.cast());
}

#[inline]
pub unsafe fn idal_buffer_array_alloc(mut size: usize, page_order: i32) -> *mut *mut idal_buffer {
    let count = (size + CCW_MAX_BYTE_COUNT - 1) / CCW_MAX_BYTE_COUNT;
    let ibs = kmalloc_objs::<*mut idal_buffer>(count + 1);
    let mut i = 0usize;
    while i < count {
        let ib_size = core::cmp::min(size, CCW_MAX_BYTE_COUNT);
        size -= ib_size;
        *ibs.add(i) = idal_buffer_alloc(ib_size, page_order);
        if IS_ERR(*ibs.add(i)) {
            while i > 0 { i -= 1; idal_buffer_free(*ibs.add(i)); }
            kfree(ibs.cast());
            return ERR_PTR(-ENOMEM);
        }
        i += 1;
    }
    *ibs.add(i) = core::ptr::null_mut();
    ibs
}

#[inline]
pub unsafe fn idal_buffer_array_free(ibs: *mut *mut *mut idal_buffer) {
    if ibs.is_null() || (*ibs).is_null() { return; }
    let mut p = *ibs;
    while !(*p).is_null() { idal_buffer_free(*p); p = p.add(1); }
    kfree((*ibs).cast());
    *ibs = core::ptr::null_mut();
}

#[inline]
pub unsafe fn idal_buffer_array_size(mut ibs: *mut *mut idal_buffer) -> i32 {
    let mut size = 0;
    while !ibs.is_null() && !(*ibs).is_null() { size += 1; ibs = ibs.add(1); }
    size
}

#[inline]
pub unsafe fn idal_buffer_array_datasize(mut ibs: *mut *mut idal_buffer) -> usize {
    let mut size = 0;
    while !ibs.is_null() && !(*ibs).is_null() { size += (**ibs).size; ibs = ibs.add(1); }
    size
}

#[inline]
pub unsafe fn __idal_buffer_is_needed(ib: *mut idal_buffer) -> bool {
    (*ib).size > (PAGE_SIZE << (*ib).page_order) || idal_is_needed(dma64_to_virt((*ib).data[0]), (*ib).size as u32)
}

#[inline]
pub unsafe fn idal_buffer_set_cda(ib: *mut idal_buffer, ccw: *mut ccw1) {
    if __idal_buffer_is_needed(ib) {
        (*ccw).cda = virt_to_dma32((*ib).data.as_mut_ptr().cast());
        (*ccw).flags |= CCW_FLAG_IDA;
    } else {
        (*ccw).cda = virt_to_dma32(dma64_to_virt((*ib).data[0]));
    }
    (*ccw).count = (*ib).size as u32;
}

#[inline]
pub unsafe fn idal_buffer_to_user(ib: *mut idal_buffer, mut to: *mut core::ffi::c_void, mut count: usize) -> usize {
    BUG_ON(count > (*ib).size);
    let mut i = 0usize;
    while count > IDA_BLOCK_SIZE {
        let left = copy_to_user(to, dma64_to_virt((*ib).data[i]), IDA_BLOCK_SIZE);
        if left != 0 { return left + count - IDA_BLOCK_SIZE; }
        to = (to as *mut u8).add(IDA_BLOCK_SIZE).cast(); count -= IDA_BLOCK_SIZE; i += 1;
    }
    copy_to_user(to, dma64_to_virt((*ib).data[i]), count)
}

#[inline]
pub unsafe fn idal_buffer_from_user(ib: *mut idal_buffer, mut from: *const core::ffi::c_void, mut count: usize) -> usize {
    BUG_ON(count > (*ib).size);
    let mut i = 0usize;
    while count > IDA_BLOCK_SIZE {
        let left = copy_from_user(dma64_to_virt((*ib).data[i]), from, IDA_BLOCK_SIZE);
        if left != 0 { return left + count - IDA_BLOCK_SIZE; }
        from = (from as *const u8).add(IDA_BLOCK_SIZE).cast(); count -= IDA_BLOCK_SIZE; i += 1;
    }
    copy_from_user(dma64_to_virt((*ib).data[i]), from, count)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
