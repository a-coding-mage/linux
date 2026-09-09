// SPDX-License-Identifier: GPL-2.0

// <linux/efi.h>
// <asm/efi.h>
// "efistub.h"

/**
 * efi_get_memory_map() - get memory map
 * @map:              pointer to memory map pointer to which to assign the
 *                    newly allocated memory map
 * @install_cfg_tbl:  whether or not to install the boot memory map as a
 *                    configuration table
 */
pub unsafe fn efi_get_memory_map(
    map: *mut *mut efi_boot_memmap,
    install_cfg_tbl: bool,
) -> efi_status_t {
    let mut tmp: efi_boot_memmap = core::mem::zeroed();
    let mut m: *mut efi_boot_memmap = core::ptr::null_mut();
    let memtype = if install_cfg_tbl {
        EFI_ACPI_RECLAIM_MEMORY
    } else {
        EFI_LOADER_DATA
    };
    let mut tbl_guid: efi_guid_t = LINUX_EFI_BOOT_MEMMAP_GUID;
    let mut status: efi_status_t;
    let size: usize;

    tmp.map_size = 0;
    status = efi_bs_call!(
        get_memory_map,
        &mut tmp.map_size,
        core::ptr::null_mut(),
        &mut tmp.map_key,
        &mut tmp.desc_size,
        &mut tmp.desc_ver
    );
    if status != EFI_BUFFER_TOO_SMALL {
        return EFI_LOAD_ERROR;
    }

    size = tmp.map_size + tmp.desc_size * EFI_MMAP_NR_SLACK_SLOTS;
    status = efi_bs_call!(
        allocate_pool,
        memtype,
        core::mem::size_of::<*mut efi_boot_memmap>() + size,
        &mut m as *mut *mut efi_boot_memmap as *mut *mut core::ffi::c_void
    );
    if status != EFI_SUCCESS {
        return status;
    }

    if install_cfg_tbl {
        status = efi_bs_call!(install_configuration_table, &mut tbl_guid, m);
        if status != EFI_SUCCESS {
            return status;
        }
    }

    (*m).buff_size = size;
    (*m).map_size = size;
    status = efi_bs_call!(
        get_memory_map,
        &mut (*m).map_size,
        (*m).map,
        &mut (*m).map_key,
        &mut (*m).desc_size,
        &mut (*m).desc_ver
    );
    if status != EFI_SUCCESS {
        if install_cfg_tbl {
            efi_bs_call!(install_configuration_table, &mut tbl_guid, core::ptr::null_mut());
        }
        return status;
    }

    *map = m;
    EFI_SUCCESS
}

/**
 * efi_allocate_pages() - Allocate memory pages
 */
pub unsafe fn efi_allocate_pages(
    size: c_ulong,
    addr: *mut c_ulong,
    mut max: c_ulong,
) -> efi_status_t {
    let mut alloc_addr: efi_physical_addr_t;
    let status: efi_status_t;

    max = min(max, EFI_ALLOC_LIMIT);

    if EFI_ALLOC_ALIGN > EFI_PAGE_SIZE {
        return efi_allocate_pages_aligned(size, addr, max, EFI_ALLOC_ALIGN, EFI_LOADER_DATA);
    }

    alloc_addr = ALIGN_DOWN(max + 1, EFI_ALLOC_ALIGN) - 1;
    status = efi_bs_call!(
        allocate_pages,
        EFI_ALLOCATE_MAX_ADDRESS,
        EFI_LOADER_DATA,
        DIV_ROUND_UP(size, EFI_PAGE_SIZE),
        &mut alloc_addr
    );
    if status != EFI_SUCCESS {
        return status;
    }

    *addr = alloc_addr;
    EFI_SUCCESS
}

/**
 * efi_free() - free memory pages
 */
pub unsafe fn efi_free(size: c_ulong, addr: c_ulong) {
    if size == 0 {
        return;
    }

    let nr_pages = round_up(size, EFI_ALLOC_ALIGN) / EFI_PAGE_SIZE;
    efi_bs_call!(free_pages, addr, nr_pages);
}

/**
 * efi_low_alloc_above() - allocate pages at or above given address
 */
pub unsafe fn efi_low_alloc_above(
    size: c_ulong,
    mut align: c_ulong,
    addr: *mut c_ulong,
    min: c_ulong,
) -> efi_status_t {
    let mut map: *mut efi_boot_memmap = core::ptr::null_mut();
    let status: efi_status_t;
    let nr_pages: c_ulong;
    let mut i: c_int = 0;

    status = efi_get_memory_map(&mut map, false);
    if status != EFI_SUCCESS {
        return status;
    }

    if align < EFI_ALLOC_ALIGN {
        align = EFI_ALLOC_ALIGN;
    }

    let size = round_up(size, EFI_ALLOC_ALIGN);
    nr_pages = size / EFI_PAGE_SIZE;
    while i < (*map).map_size / (*map).desc_size {
        let m = (*map).map as c_ulong;
        let desc: *mut efi_memory_desc_t = efi_memdesc_ptr(m, (*map).desc_size, i);
        let mut start: u64;
        let end: u64;

        if (*desc).type_ != EFI_CONVENTIONAL_MEMORY {
            i += 1;
            continue;
        }
        if (*desc).attribute & EFI_MEMORY_HOT_PLUGGABLE != 0 {
            i += 1;
            continue;
        }
        if efi_soft_reserve_enabled() && (*desc).attribute & EFI_MEMORY_SP != 0 {
            i += 1;
            continue;
        }
        if (*desc).num_pages < nr_pages {
            i += 1;
            continue;
        }

        start = (*desc).phys_addr;
        end = start + (*desc).num_pages * EFI_PAGE_SIZE;
        if start < min {
            start = min;
        }
        start = round_up(start, align);
        if start + size > end {
            i += 1;
            continue;
        }

        status = efi_bs_call!(allocate_pages, EFI_ALLOCATE_ADDRESS, EFI_LOADER_DATA, nr_pages, &mut start);
        if status == EFI_SUCCESS {
            *addr = start;
            break;
        }
        i += 1;
    }

    if i == (*map).map_size / (*map).desc_size {
        return EFI_NOT_FOUND;
    }
    EFI_SUCCESS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
