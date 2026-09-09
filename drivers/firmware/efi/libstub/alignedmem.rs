// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by <linux/efi.h>, <asm/efi.h>, and "efistub.h".

/**
 * efi_allocate_pages_aligned() - Allocate memory pages
 * @size: minimum number of bytes to allocate
 * @addr: On return the address of the first allocated page. The first
 *        allocated page has alignment EFI_ALLOC_ALIGN which is an
 *        architecture dependent multiple of the page size.
 * @max: the address that the last allocated memory page shall not exceed
 * @align: minimum alignment of the base of the allocation
 * @memory_type: the type of memory to allocate
 *
 * Allocate pages as EFI_LOADER_DATA. The allocated pages are aligned according
 * to @align, which should be >= EFI_ALLOC_ALIGN. The last allocated page will
 * not exceed the address given by @max.
 *
 * Return: status code
 */
pub unsafe fn efi_allocate_pages_aligned(
    mut size: ::core::ffi::c_ulong,
    addr: *mut ::core::ffi::c_ulong,
    mut max: ::core::ffi::c_ulong,
    mut align: ::core::ffi::c_ulong,
    memory_type: ::core::ffi::c_int,
) -> efi_status_t {
    let mut alloc_addr: efi_physical_addr_t;
    let status: efi_status_t;
    let mut slack: ::core::ffi::c_int;

    max = core::cmp::min(max, EFI_ALLOC_LIMIT);

    if align < EFI_ALLOC_ALIGN {
        align = EFI_ALLOC_ALIGN;
    }

    alloc_addr = (max + 1) & !(align - 1);
    alloc_addr -= 1;
    size = (size + EFI_ALLOC_ALIGN - 1) & !(EFI_ALLOC_ALIGN - 1);
    slack = (align / EFI_PAGE_SIZE - 1) as ::core::ffi::c_int;

    status = efi_bs_call!(
        allocate_pages,
        EFI_ALLOCATE_MAX_ADDRESS,
        memory_type,
        size / EFI_PAGE_SIZE + slack as ::core::ffi::c_ulong,
        &mut alloc_addr
    );
    if status != EFI_SUCCESS {
        return status;
    }

    *addr = (alloc_addr as ::core::ffi::c_ulong + align - 1) & !(align - 1);

    if slack > 0 {
        let l = ((alloc_addr & (align - 1)) / EFI_PAGE_SIZE)
            as ::core::ffi::c_int;

        if l != 0 {
            efi_bs_call!(free_pages, alloc_addr, slack - l + 1);
            slack = l - 1;
        }
        if slack != 0 {
            efi_bs_call!(free_pages, *addr + size, slack);
        }
    }
    EFI_SUCCESS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
