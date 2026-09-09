// SPDX-License-Identifier: GPL-2.0
/*
 * Common EFI memory map functions.
 */

// Dependency intent preserved from the C implementation:
// linux/init.h, linux/kernel.h, linux/efi.h, linux/io.h, asm/early_ioremap.h,
// asm/efi.h, linux/memblock.h, and linux/slab.h provide the external symbols
// and types referenced below.

unsafe fn __efi_memmap_alloc_early(size: c_ulong) -> phys_addr_t {
    memblock_phys_alloc(size, SMP_CACHE_BYTES)
}

unsafe fn __efi_memmap_alloc_late(size: c_ulong) -> phys_addr_t {
    let order: c_uint = get_order(size);
    let p: *mut page = alloc_pages(GFP_KERNEL, order);

    if p.is_null() {
        return 0;
    }

    PFN_PHYS(page_to_pfn(p))
}

unsafe fn __efi_memmap_free(phys: u64, size: c_ulong, flags: c_ulong) {
    if flags & EFI_MEMMAP_MEMBLOCK != 0 {
        memblock_phys_free(phys, size);
    } else if flags & EFI_MEMMAP_SLAB != 0 {
        let p: *mut page = pfn_to_page(PHYS_PFN(phys));
        let order: c_uint = get_order(size);

        __free_pages(p, order);
    }
}

/**
 * efi_memmap_alloc - Allocate memory for the EFI memory map
 * @num_entries: Number of entries in the allocated map.
 * @data: efi memmap installation parameters
 *
 * Depending on whether mm_init() has already been invoked or not,
 * either memblock or "normal" page allocation is used.
 *
 * Returns zero on success, a negative error code on failure.
 */
pub unsafe fn efi_memmap_alloc(
    num_entries: c_uint,
    data: *mut efi_memory_map_data,
) -> c_int {
    // Expect allocation parameters are zero initialized
    WARN_ON((*data).phys_map != 0 || (*data).size != 0);

    (*data).size = num_entries as c_ulong * efi.memmap.desc_size;
    (*data).desc_version = efi.memmap.desc_version;
    (*data).desc_size = efi.memmap.desc_size;
    (*data).flags &= !(EFI_MEMMAP_SLAB | EFI_MEMMAP_MEMBLOCK);
    (*data).flags |= efi.memmap.flags & EFI_MEMMAP_LATE;

    if slab_is_available() {
        (*data).flags |= EFI_MEMMAP_SLAB;
        (*data).phys_map = __efi_memmap_alloc_late((*data).size);
    } else {
        (*data).flags |= EFI_MEMMAP_MEMBLOCK;
        (*data).phys_map = __efi_memmap_alloc_early((*data).size);
    }

    if (*data).phys_map == 0 {
        return -ENOMEM;
    }
    0
}

/**
 * efi_memmap_install - Install a new EFI memory map in efi.memmap
 * @data: efi memmap installation parameters
 *
 * Unlike efi_memmap_init_*(), this function does not allow the caller
 * to switch from early to late mappings. It simply uses the existing
 * mapping function and installs the new memmap.
 *
 * Returns zero on success, a negative error code on failure.
 */
pub unsafe fn efi_memmap_install(data: *mut efi_memory_map_data) -> c_int {
    let size: c_ulong = efi.memmap.desc_size * efi.memmap.nr_map;
    let flags: c_ulong = efi.memmap.flags;
    let phys: u64 = efi.memmap.phys_map;

    efi_memmap_unmap();

    if efi_enabled(EFI_PARAVIRT) {
        return 0;
    }

    let ret: c_int = __efi_memmap_init(data);
    if ret != 0 {
        return ret;
    }

    __efi_memmap_free(phys, size, flags);
    0
}

/**
 * efi_memmap_split_count - Count number of additional EFI memmap entries
 * @md: EFI memory descriptor to split
 * @range: Address range (start, end) to split around
 *
 * Returns the number of additional EFI memmap entries required to
 * accommodate @range.
 */
pub unsafe fn efi_memmap_split_count(
    md: *mut efi_memory_desc_t,
    range: *mut range,
) -> c_int {
    let start: u64 = (*md).phys_addr;
    let end: u64 = start + ((*md).num_pages << EFI_PAGE_SHIFT) - 1;
    let m_start: u64 = (*range).start;
    let m_end: u64 = (*range).end;
    let mut count: c_int = 0;

    if m_start <= start {
        // split into 2 parts
        if start < m_end && m_end < end {
            count += 1;
        }
    }

    if start < m_start && m_start < end {
        // split into 3 parts
        if m_end < end {
            count += 2;
        }
        // split into 2 parts
        if end <= m_end {
            count += 1;
        }
    }

    count
}

/**
 * efi_memmap_insert - Insert a memory region in an EFI memmap
 * @old_memmap: The existing EFI memory map structure
 * @buf: Address of buffer to store new map
 * @mem: Memory map entry to insert
 *
 * It is suggested that you call efi_memmap_split_count() first
 * to see how large @buf needs to be.
 */
pub unsafe fn efi_memmap_insert(
    old_memmap: *mut efi_memory_map,
    buf: *mut c_void,
    mem: *mut efi_mem_range,
) {
    let m_start: u64 = (*mem).range.start;
    let m_end: u64 = (*mem).range.end;
    let m_attr: u64 = (*mem).attribute;
    let mut old: *mut u8 = (*old_memmap).map as *mut u8;
    let mut new: *mut u8 = buf as *mut u8;

    // The EFI memory map deals with regions in EFI_PAGE_SIZE units. Ensure
    // that the region described by 'mem' is aligned correctly.
    if !IS_ALIGNED(m_start, EFI_PAGE_SIZE) || !IS_ALIGNED(m_end + 1, EFI_PAGE_SIZE) {
        WARN_ON(1);
        return;
    }

    while old < (*old_memmap).map_end as *mut u8 {
        // copy original EFI memory descriptor
        core::ptr::copy_nonoverlapping(
            old,
            new,
            (*old_memmap).desc_size as usize,
        );
        let mut md: *mut efi_memory_desc_t = new as *mut efi_memory_desc_t;
        let start: u64 = (*md).phys_addr;
        let end: u64 = (*md).phys_addr + ((*md).num_pages << EFI_PAGE_SHIFT) - 1;

        if m_start <= start && end <= m_end {
            (*md).attribute |= m_attr;
        }

        if m_start <= start && start < m_end && m_end < end {
            // first part
            (*md).attribute |= m_attr;
            (*md).num_pages = (m_end - (*md).phys_addr + 1) >> EFI_PAGE_SHIFT;
            // latter part
            new = new.add((*old_memmap).desc_size as usize);
            core::ptr::copy_nonoverlapping(old, new, (*old_memmap).desc_size as usize);
            md = new as *mut efi_memory_desc_t;
            (*md).phys_addr = m_end + 1;
            (*md).num_pages = (end - (*md).phys_addr + 1) >> EFI_PAGE_SHIFT;
        }

        if start < m_start && m_start < end && m_end < end {
            // first part
            (*md).num_pages = (m_start - (*md).phys_addr) >> EFI_PAGE_SHIFT;
            // middle part
            new = new.add((*old_memmap).desc_size as usize);
            core::ptr::copy_nonoverlapping(old, new, (*old_memmap).desc_size as usize);
            md = new as *mut efi_memory_desc_t;
            (*md).attribute |= m_attr;
            (*md).phys_addr = m_start;
            (*md).num_pages = (m_end - m_start + 1) >> EFI_PAGE_SHIFT;
            // last part
            new = new.add((*old_memmap).desc_size as usize);
            core::ptr::copy_nonoverlapping(old, new, (*old_memmap).desc_size as usize);
            md = new as *mut efi_memory_desc_t;
            (*md).phys_addr = m_end + 1;
            (*md).num_pages = (end - m_end) >> EFI_PAGE_SHIFT;
        }

        if start < m_start && m_start < end && end <= m_end {
            // first part
            (*md).num_pages = (m_start - (*md).phys_addr) >> EFI_PAGE_SHIFT;
            // latter part
            new = new.add((*old_memmap).desc_size as usize);
            core::ptr::copy_nonoverlapping(old, new, (*old_memmap).desc_size as usize);
            md = new as *mut efi_memory_desc_t;
            (*md).phys_addr = m_start;
            (*md).num_pages = (end - (*md).phys_addr + 1) >> EFI_PAGE_SHIFT;
            (*md).attribute |= m_attr;
        }

        old = old.add((*old_memmap).desc_size as usize);
        new = new.add((*old_memmap).desc_size as usize);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
