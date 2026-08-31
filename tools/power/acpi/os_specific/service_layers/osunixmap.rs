// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: osunixmap - Unix OSL for file mappings
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// C dependencies in the source: "acpidump.h", <unistd.h>, <sys/mman.h>.
// On _free_BSD the C source also includes <sys/param.h>.

use core::ffi::{c_char, c_int, c_long, c_void};

const _COMPONENT: u32 = ACPI_OS_SERVICES;
// ACPI_MODULE_NAME("osunixmap")

// C fallback when O_BINARY is not defined.
const O_BINARY: c_int = 0;

// C condition:
// #if defined(_dragon_fly) || defined(_free_BSD) || defined(_QNX)
// #define MMAP_FLAGS MAP_SHARED
// #else
// #define MMAP_FLAGS MAP_PRIVATE
// #endif
#[cfg(any(_dragon_fly, _free_BSD, _QNX))]
const MMAP_FLAGS: c_int = MAP_SHARED;
#[cfg(not(any(_dragon_fly, _free_BSD, _QNX)))]
const MMAP_FLAGS: c_int = MAP_PRIVATE;

const SYSTEM_MEMORY: *const c_char = b"/dev/mem\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static ACPI_OS_SERVICES: u32;

    static mut stderr: *mut FILE;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn mmap(
        addr: *mut c_void,
        length: acpi_size,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: acpi_physical_address,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: acpi_size) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
}

unsafe extern "C" {
    static O_RDONLY: c_int;
    static PROT_READ: c_int;
    static MAP_SHARED: c_int;
    static MAP_PRIVATE: c_int;
    static MAP_FAILED: *mut c_void;
    static _SC_PAGESIZE: c_int;
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_os_get_page_size
 *
 * PARAMETERS:  None
 *
 * RETURN:      Page size of the platform.
 *
 * DESCRIPTION: Obtain page size of the platform.
 *
 ******************************************************************************/
unsafe fn acpi_os_get_page_size() -> acpi_size {
    // C condition:
    // #ifdef PAGE_SIZE
    //     return PAGE_SIZE;
    // #else
    //     return sysconf(_SC_PAGESIZE);
    // #endif
    #[cfg(PAGE_SIZE)]
    {
        PAGE_SIZE as acpi_size
    }
    #[cfg(not(PAGE_SIZE))]
    {
        sysconf(_SC_PAGESIZE) as acpi_size
    }
}

/******************************************************************************
 *
 * FUNCTION:    acpi_os_map_memory
 *
 * PARAMETERS:  where               - Physical address of memory to be mapped
 *              length              - How much memory to map
 *
 * RETURN:      Pointer to mapped memory. Null on error.
 *
 * DESCRIPTION: Map physical memory into local address space.
 *
 *****************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn acpi_os_map_memory(
    where_: acpi_physical_address,
    length: acpi_size,
) -> *mut c_void {
    let mapped_memory: *mut u8;
    let offset: acpi_physical_address;
    let page_size: acpi_size;
    let fd: c_int;

    fd = open(SYSTEM_MEMORY, O_RDONLY | O_BINARY);
    if fd < 0 {
        fprintf(
            stderr,
            b"Cannot open %s\n\0".as_ptr() as *const c_char,
            SYSTEM_MEMORY,
        );
        return core::ptr::null_mut();
    }

    /* Align the offset to use mmap */

    page_size = acpi_os_get_page_size();
    offset = where_ % page_size as acpi_physical_address;

    /* Map the table header to get the length of the full table */

    mapped_memory = mmap(
        core::ptr::null_mut(),
        length + offset as acpi_size,
        PROT_READ,
        MMAP_FLAGS,
        fd,
        where_ - offset,
    ) as *mut u8;
    if mapped_memory as *mut c_void == MAP_FAILED {
        fprintf(
            stderr,
            b"Cannot map %s\n\0".as_ptr() as *const c_char,
            SYSTEM_MEMORY,
        );
        close(fd);
        return core::ptr::null_mut();
    }

    close(fd);
    mapped_memory.add(offset as usize) as *mut c_void
}

/******************************************************************************
 *
 * FUNCTION:    acpi_os_unmap_memory
 *
 * PARAMETERS:  where               - Logical address of memory to be unmapped
 *              length              - How much memory to unmap
 *
 * RETURN:      None.
 *
 * DESCRIPTION: Delete a previously created mapping. Where and Length must
 *              correspond to a previous mapping exactly.
 *
 *****************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn acpi_os_unmap_memory(where_: *mut c_void, length: acpi_size) {
    let offset: acpi_physical_address;
    let page_size: acpi_size;

    page_size = acpi_os_get_page_size();
    offset = (where_ as usize as acpi_physical_address) % page_size as acpi_physical_address;
    munmap(
        (where_ as *mut u8).sub(offset as usize) as *mut c_void,
        length + offset as acpi_size,
    );
}
