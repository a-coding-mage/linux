// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: oslinuxtbl - Linux OSL for obtaining ACPI tables
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* Original C dependency: acpidump.h */
/* Original C module metadata:
 * #define _COMPONENT          ACPI_OS_SERVICES
 * ACPI_MODULE_NAME("oslinuxtbl")
 */

use core::ffi::{c_char, c_int, c_uchar, c_ulonglong, c_void};
use core::mem;
use core::ptr;

const PATH_MAX: usize = 256;

/* List of information about obtained ACPI tables */
#[repr(C)]
pub struct osl_table_info {
    pub next: *mut osl_table_info,
    pub instance: u32,
    pub signature: [c_char; ACPI_NAMESEG_SIZE],
}

/* File locations */

const DYNAMIC_TABLE_DIR: &[u8] = b"/sys/firmware/acpi/tables/dynamic\0";
const STATIC_TABLE_DIR: &[u8] = b"/sys/firmware/acpi/tables\0";
const EFI_SYSTAB: &[u8] = b"/sys/firmware/efi/systab\0";

/* Should we get dynamically loaded SSDTs from DYNAMIC_TABLE_DIR? */

pub static mut gbl_dump_dynamic_tables: u8 = TRUE;

/* Initialization flags */

pub static mut gbl_table_list_initialized: u8 = FALSE;

/* Local copies of main ACPI tables */

pub static mut gbl_rsdp: acpi_table_rsdp = unsafe { mem::zeroed() };
pub static mut gbl_fadt: *mut acpi_table_fadt = ptr::null_mut();
pub static mut gbl_rsdt: *mut acpi_table_rsdt = ptr::null_mut();
pub static mut gbl_xsdt: *mut acpi_table_xsdt = ptr::null_mut();

/* Table addresses */

pub static mut gbl_fadt_address: acpi_physical_address = 0;
pub static mut gbl_rsdp_address: acpi_physical_address = 0;

/* Revision of RSD PTR */

pub static mut gbl_revision: u8 = 0;

pub static mut gbl_table_list_head: *mut osl_table_info = ptr::null_mut();
pub static mut gbl_table_count: u32 = 0;

extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut FILE;
    static mut gbl_dump_customized_tables: u8;
    static mut gbl_rsdp_base: acpi_physical_address;
    static mut gbl_do_not_dump_xsdt: u8;
    static mut acpi_gbl_do_not_use_xsdt: u8;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn isdigit(c: c_int) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;

    fn ap_get_table_length(table: *mut acpi_table_header) -> u32;
    fn ap_is_valid_checksum(table: *mut acpi_table_header) -> u8;
    fn acpi_os_map_memory(
        where_: acpi_physical_address,
        length: acpi_size,
    ) -> *mut c_void;
    fn acpi_os_unmap_memory(where_: *mut c_void, length: acpi_size);
    fn acpi_tb_scan_memory_for_rsdp(start_address: *mut u8, length: acpi_size) -> *mut u8;
    fn acpi_os_open_directory(
        pathname: *const c_char,
        wildcard_spec: *const c_char,
        requested_file_type: c_char,
    ) -> *mut c_void;
    fn acpi_os_get_next_filename(dir_handle: *mut c_void) -> *mut c_char;
    fn acpi_os_close_directory(dir_handle: *mut c_void);
}

/* Types, constants, and macros supplied by acpidump.h/ACPICA headers. */
pub type acpi_status = u32;
pub type acpi_size = usize;
pub type acpi_physical_address = u64;
pub type c_long = i64;
pub enum FILE {}

extern "C" {
    static ACPI_SIG_XSDT: *mut c_char;
    static ACPI_SIG_RSDT: *mut c_char;
    static ACPI_SIG_DSDT: *mut c_char;
    static ACPI_SIG_FACS: *mut c_char;
    static ACPI_SIG_RSDP: *mut c_char;
    static ACPI_SIG_FADT: *mut c_char;
    static ACPI_RSDP_NAME: *mut c_char;
}

const TRUE: u8 = 1;
const FALSE: u8 = 0;
const SEEK_SET: c_int = 0;
const EACCES: c_int = 13;
const EPERM: c_int = 1;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const REQUEST_FILE_ONLY: c_char = 0;
const ACPI_NAMESEG_SIZE: usize = 4;
const AE_OK: acpi_status = 0;
const AE_ACCESS: acpi_status = 0x1;
const AE_NOT_FOUND: acpi_status = 0x2;
const AE_NO_MEMORY: acpi_status = 0x3;
const AE_BAD_HEADER: acpi_status = 0x4;
const AE_LIMIT: acpi_status = 0x5;
const AE_BAD_SIGNATURE: acpi_status = 0x6;
const AE_BAD_ADDRESS: acpi_status = 0x7;
const AE_INVALID_TABLE_LENGTH: acpi_status = 0x8;
const ACPI_HI_RSDP_WINDOW_BASE: acpi_physical_address = 0;
const ACPI_HI_RSDP_WINDOW_SIZE: acpi_size = 0;
const MIN_FADT_FOR_XDSDT: u32 = 0;
const MIN_FADT_FOR_DSDT: u32 = 0;
const MIN_FADT_FOR_XFACS: u32 = 0;
const MIN_FADT_FOR_FACS: u32 = 0;

#[repr(C)]
pub struct acpi_table_header {
    pub signature: [c_char; ACPI_NAMESEG_SIZE],
    pub length: u32,
}

#[repr(C)]
pub struct acpi_table_rsdp {
    pub signature: [c_char; 8],
    pub checksum: u8,
    pub oem_id: [c_char; 6],
    pub revision: u8,
    pub rsdt_physical_address: u32,
    pub length: u32,
    pub xsdt_physical_address: u64,
}

#[repr(C)]
pub struct acpi_table_rsdt {
    pub header: acpi_table_header,
}

#[repr(C)]
pub struct acpi_table_xsdt {
    pub header: acpi_table_header,
}

#[repr(C)]
pub struct acpi_table_fadt {
    pub header: acpi_table_header,
    pub facs: u32,
    pub dsdt: u32,
    pub Xfacs: u64,
    pub Xdsdt: u64,
}

unsafe fn ACPI_FAILURE(status: acpi_status) -> bool {
    status != AE_OK
}

unsafe fn ACPI_SUCCESS(status: acpi_status) -> bool {
    status == AE_OK
}

unsafe fn ACPI_COMPARE_NAMESEG(a: *const c_char, b: *const c_char) -> bool {
    ptr::read_unaligned(a as *const u32) == ptr::read_unaligned(b as *const u32)
}

unsafe fn ACPI_COPY_NAMESEG(dest: *mut c_char, src: *const c_char) {
    ptr::copy_nonoverlapping(src, dest, ACPI_NAMESEG_SIZE);
}

unsafe fn ACPI_VALIDATE_RSDP_SIG(sig: *const c_char) -> bool {
    ptr::read_unaligned(sig as *const u64) == ptr::read_unaligned(ACPI_SIG_RSDP as *const u64)
}

unsafe fn ACPI_FORMAT_UINT64(value: u64) -> (u32, u32) {
    ((value >> 32) as u32, value as u32)
}

/******************************************************************************
 *
 * FUNCTION:    osl_get_last_status
 *
 * PARAMETERS:  default_status  - Default error status to return
 *
 * RETURN:      Status; Converted from errno.
 *
 * DESCRIPTION: Get last errno and convert it to acpi_status.
 *
 *****************************************************************************/

unsafe fn osl_get_last_status(default_status: acpi_status) -> acpi_status {
    match errno {
        EACCES | EPERM => AE_ACCESS,
        ENOENT => AE_NOT_FOUND,
        ENOMEM => AE_NO_MEMORY,
        _ => default_status,
    }
}

/******************************************************************************
 *
 * FUNCTION:    acpi_os_get_table_by_address
 *
 * PARAMETERS:  address         - Physical address of the ACPI table
 *              table           - Where a pointer to the table is returned
 *
 * RETURN:      Status; Table buffer is returned if AE_OK.
 *              AE_NOT_FOUND: A valid table was not found at the address
 *
 * DESCRIPTION: Get an ACPI table via a physical memory address.
 *
 *****************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn acpi_os_get_table_by_address(
    address: acpi_physical_address,
    table: *mut *mut acpi_table_header,
) -> acpi_status {
    let table_length: u32;
    let mut mapped_table: *mut acpi_table_header = ptr::null_mut();
    let mut local_table: *mut acpi_table_header = ptr::null_mut();
    let mut status: acpi_status = AE_OK;

    /* Get main ACPI tables from memory on first invocation of this function */

    status = osl_table_initialize();
    if ACPI_FAILURE(status) {
        return status;
    }

    /* Map the table and validate it */

    status = osl_map_table(address as acpi_size, ptr::null_mut(), &mut mapped_table);
    if ACPI_FAILURE(status) {
        return status;
    }

    /* Copy table to local buffer and return it */

    table_length = ap_get_table_length(mapped_table);
    if table_length == 0 {
        status = AE_BAD_HEADER;
        osl_unmap_table(mapped_table);
        *table = local_table;
        return status;
    }

    local_table = calloc(1, table_length as usize) as *mut acpi_table_header;
    if local_table.is_null() {
        status = AE_NO_MEMORY;
        osl_unmap_table(mapped_table);
        *table = local_table;
        return status;
    }

    memcpy(
        local_table as *mut c_void,
        mapped_table as *const c_void,
        table_length as usize,
    );

    osl_unmap_table(mapped_table);
    *table = local_table;
    status
}

/******************************************************************************
 *
 * FUNCTION:    acpi_os_get_table_by_name
 *
 * PARAMETERS:  signature       - ACPI Signature for desired table. Must be
 *                                a null terminated 4-character string.
 *              instance        - Multiple table support for SSDT/UEFI (0...n)
 *                                Must be 0 for other tables.
 *              table           - Where a pointer to the table is returned
 *              address         - Where the table physical address is returned
 *
 * RETURN:      Status; Table buffer and physical address returned if AE_OK.
 *              AE_LIMIT: Instance is beyond valid limit
 *              AE_NOT_FOUND: A table with the signature was not found
 *
 * NOTE:        Assumes the input signature is uppercase.
 *
 *****************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn acpi_os_get_table_by_name(
    signature: *mut c_char,
    instance: u32,
    table: *mut *mut acpi_table_header,
    address: *mut acpi_physical_address,
) -> acpi_status {
    let mut status: acpi_status;

    /* Get main ACPI tables from memory on first invocation of this function */

    status = osl_table_initialize();
    if ACPI_FAILURE(status) {
        return status;
    }

    /* Not a main ACPI table, attempt to extract it from the RSDT/XSDT */

    if gbl_dump_customized_tables == 0 {
        /* Attempt to get the table from the memory */

        status = osl_get_bios_table(signature, instance, table, address);
    } else {
        /* Attempt to get the table from the static directory */

        status = osl_get_customized_table(
            STATIC_TABLE_DIR.as_ptr() as *mut c_char,
            signature,
            instance,
            table,
            address,
        );
    }

    if ACPI_FAILURE(status) && status == AE_LIMIT {
        if gbl_dump_dynamic_tables != 0 {
            /* Attempt to get a dynamic table */

            status = osl_get_customized_table(
                DYNAMIC_TABLE_DIR.as_ptr() as *mut c_char,
                signature,
                instance,
                table,
                address,
            );
        }
    }

    status
}

/******************************************************************************
 *
 * FUNCTION:    osl_add_table_to_list
 *
 *****************************************************************************/

unsafe fn osl_add_table_to_list(signature: *mut c_char, mut instance: u32) -> acpi_status {
    let new_info: *mut osl_table_info;
    let mut next: *mut osl_table_info;
    let mut next_instance: u32 = 0;
    let mut found: u8 = FALSE;

    new_info = calloc(1, mem::size_of::<osl_table_info>()) as *mut osl_table_info;
    if new_info.is_null() {
        return AE_NO_MEMORY;
    }

    ACPI_COPY_NAMESEG((*new_info).signature.as_mut_ptr(), signature);

    if gbl_table_list_head.is_null() {
        gbl_table_list_head = new_info;
    } else {
        next = gbl_table_list_head;
        loop {
            if ACPI_COMPARE_NAMESEG((*next).signature.as_ptr(), signature) {
                if (*next).instance == instance {
                    found = TRUE;
                }
                if (*next).instance >= next_instance {
                    next_instance = (*next).instance + 1;
                }
            }

            if (*next).next.is_null() {
                break;
            }
            next = (*next).next;
        }
        (*next).next = new_info;
    }

    if found != 0 {
        if instance != 0 {
            fprintf(
                stderr,
                b"%4.4s: Warning unmatched table instance %d, expected %d\n\0".as_ptr()
                    as *const c_char,
                signature,
                instance,
                next_instance,
            );
        }
        instance = next_instance;
    }

    (*new_info).instance = instance;
    gbl_table_count += 1;

    AE_OK
}

/******************************************************************************
 *
 * FUNCTION:    acpi_os_get_table_by_index
 *
 *****************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn acpi_os_get_table_by_index(
    index: u32,
    table: *mut *mut acpi_table_header,
    instance: *mut u32,
    address: *mut acpi_physical_address,
) -> acpi_status {
    let mut info: *mut osl_table_info;
    let status: acpi_status;
    let mut i: u32;

    /* Get main ACPI tables from memory on first invocation of this function */

    let init_status = osl_table_initialize();
    if ACPI_FAILURE(init_status) {
        return init_status;
    }

    /* Validate Index */

    if index >= gbl_table_count {
        return AE_LIMIT;
    }

    /* Point to the table list entry specified by the Index argument */

    info = gbl_table_list_head;
    i = 0;
    while i < index {
        info = (*info).next;
        i += 1;
    }

    /* Now we can just get the table via the signature */

    status = acpi_os_get_table_by_name(
        (*info).signature.as_mut_ptr(),
        (*info).instance,
        table,
        address,
    );

    if ACPI_SUCCESS(status) {
        *instance = (*info).instance;
    }
    status
}

/******************************************************************************
 *
 * FUNCTION:    osl_find_rsdp_via_efi_by_keyword
 *
 *****************************************************************************/

unsafe fn osl_find_rsdp_via_efi_by_keyword(
    file: *mut FILE,
    keyword: *const c_char,
) -> acpi_physical_address {
    let mut buffer: [c_char; 80] = [0; 80];
    let mut address: c_ulonglong = 0;
    let mut format: [c_char; 32] = [0; 32];

    snprintf(
        format.as_mut_ptr(),
        32,
        b"%s=%s\0".as_ptr() as *const c_char,
        keyword,
        b"%llx\0".as_ptr() as *const c_char,
    );
    fseek(file, 0, SEEK_SET);
    while !fgets(buffer.as_mut_ptr(), 80, file).is_null() {
        if sscanf(buffer.as_ptr(), format.as_ptr(), &mut address) == 1 {
            break;
        }
    }

    address as acpi_physical_address
}

/******************************************************************************
 *
 * FUNCTION:    osl_find_rsdp_via_efi
 *
 *****************************************************************************/

unsafe fn osl_find_rsdp_via_efi() -> acpi_physical_address {
    let file: *mut FILE;
    let mut address: acpi_physical_address = 0;

    file = fopen(EFI_SYSTAB.as_ptr() as *const c_char, b"r\0".as_ptr() as *const c_char);
    if !file.is_null() {
        address = osl_find_rsdp_via_efi_by_keyword(file, b"ACPI20\0".as_ptr() as *const c_char);
        if address == 0 {
            address = osl_find_rsdp_via_efi_by_keyword(file, b"ACPI\0".as_ptr() as *const c_char);
        }
        fclose(file);
    }

    address
}

/******************************************************************************
 *
 * FUNCTION:    osl_load_rsdp
 *
 *****************************************************************************/

unsafe fn osl_load_rsdp() -> acpi_status {
    let mapped_table: *mut acpi_table_header;
    let rsdp_address: *mut u8;
    let mut rsdp_base: acpi_physical_address;
    let mut rsdp_size: acpi_size;

    /* Get RSDP from memory */

    rsdp_size = mem::size_of::<acpi_table_rsdp>();
    if gbl_rsdp_base != 0 {
        rsdp_base = gbl_rsdp_base;
    } else {
        rsdp_base = osl_find_rsdp_via_efi();
    }

    if rsdp_base == 0 {
        rsdp_base = ACPI_HI_RSDP_WINDOW_BASE;
        rsdp_size = ACPI_HI_RSDP_WINDOW_SIZE;
    }

    rsdp_address = acpi_os_map_memory(rsdp_base, rsdp_size) as *mut u8;
    if rsdp_address.is_null() {
        return osl_get_last_status(AE_BAD_ADDRESS);
    }

    /* Search low memory for the RSDP */

    mapped_table =
        acpi_tb_scan_memory_for_rsdp(rsdp_address, rsdp_size) as *mut acpi_table_header;
    if mapped_table.is_null() {
        acpi_os_unmap_memory(rsdp_address as *mut c_void, rsdp_size);
        return AE_NOT_FOUND;
    }

    gbl_rsdp_address = rsdp_base + ((mapped_table as *mut u8).offset_from(rsdp_address) as u64);

    memcpy(
        &mut gbl_rsdp as *mut _ as *mut c_void,
        mapped_table as *const c_void,
        mem::size_of::<acpi_table_rsdp>(),
    );
    acpi_os_unmap_memory(rsdp_address as *mut c_void, rsdp_size);

    AE_OK
}

/******************************************************************************
 *
 * FUNCTION:    osl_can_use_xsdt
 *
 *****************************************************************************/

unsafe fn osl_can_use_xsdt() -> u8 {
    if gbl_revision != 0 && acpi_gbl_do_not_use_xsdt == 0 {
        TRUE
    } else {
        FALSE
    }
}

/******************************************************************************
 *
 * FUNCTION:    osl_table_initialize
 *
 *****************************************************************************/

unsafe fn osl_table_initialize() -> acpi_status {
    let mut status: acpi_status;
    let mut address: acpi_physical_address = 0;

    if gbl_table_list_initialized != 0 {
        return AE_OK;
    }

    if gbl_dump_customized_tables == 0 {
        /* Get RSDP from memory */

        status = osl_load_rsdp();
        if ACPI_FAILURE(status) {
            return status;
        }

        /* Get XSDT from memory */

        if gbl_rsdp.revision != 0 && gbl_do_not_dump_xsdt == 0 {
            if !gbl_xsdt.is_null() {
                free(gbl_xsdt as *mut c_void);
                gbl_xsdt = ptr::null_mut();
            }

            gbl_revision = 2;
            status = osl_get_bios_table(
                ACPI_SIG_XSDT,
                0,
                &mut gbl_xsdt as *mut _ as *mut *mut acpi_table_header,
                &mut address,
            );
            if ACPI_FAILURE(status) {
                return status;
            }
        }

        /* Get RSDT from memory */

        if gbl_rsdp.rsdt_physical_address != 0 {
            if !gbl_rsdt.is_null() {
                free(gbl_rsdt as *mut c_void);
                gbl_rsdt = ptr::null_mut();
            }

            status = osl_get_bios_table(
                ACPI_SIG_RSDT,
                0,
                &mut gbl_rsdt as *mut _ as *mut *mut acpi_table_header,
                &mut address,
            );
            if ACPI_FAILURE(status) {
                return status;
            }
        }

        /* Get FADT from memory */

        if !gbl_fadt.is_null() {
            free(gbl_fadt as *mut c_void);
            gbl_fadt = ptr::null_mut();
        }

        status = osl_get_bios_table(
            ACPI_SIG_FADT,
            0,
            &mut gbl_fadt as *mut _ as *mut *mut acpi_table_header,
            &mut gbl_fadt_address,
        );
        if ACPI_FAILURE(status) {
            return status;
        }

        /* Add mandatory tables to global table list first */

        status = osl_add_table_to_list(ACPI_RSDP_NAME, 0);
        if ACPI_FAILURE(status) {
            return status;
        }

        status = osl_add_table_to_list(ACPI_SIG_RSDT, 0);
        if ACPI_FAILURE(status) {
            return status;
        }

        if gbl_revision == 2 {
            status = osl_add_table_to_list(ACPI_SIG_XSDT, 0);
            if ACPI_FAILURE(status) {
                return status;
            }
        }

        status = osl_add_table_to_list(ACPI_SIG_DSDT, 0);
        if ACPI_FAILURE(status) {
            return status;
        }

        status = osl_add_table_to_list(ACPI_SIG_FACS, 0);
        if ACPI_FAILURE(status) {
            return status;
        }

        /* Add all tables found in the memory */

        status = osl_list_bios_tables();
        if ACPI_FAILURE(status) {
            return status;
        }
    } else {
        /* Add all tables found in the static directory */

        status = osl_list_customized_tables(STATIC_TABLE_DIR.as_ptr() as *mut c_char);
        if ACPI_FAILURE(status) {
            return status;
        }
    }

    if gbl_dump_dynamic_tables != 0 {
        /* Add all dynamically loaded tables in the dynamic directory */

        status = osl_list_customized_tables(DYNAMIC_TABLE_DIR.as_ptr() as *mut c_char);
        if ACPI_FAILURE(status) {
            return status;
        }
    }

    gbl_table_list_initialized = TRUE;
    AE_OK
}

/******************************************************************************
 *
 * FUNCTION:    osl_list_bios_tables
 *
 *****************************************************************************/

unsafe fn osl_list_bios_tables() -> acpi_status {
    let mut mapped_table: *mut acpi_table_header = ptr::null_mut();
    let mut table_data: *mut u8;
    let number_of_tables: u8;
    let item_size: u8;
    let mut table_address: acpi_physical_address;
    let mut status: acpi_status = AE_OK;
    let mut i: u32;

    if osl_can_use_xsdt() != 0 {
        item_size = mem::size_of::<u64>() as u8;
        table_data = (gbl_xsdt as *mut u8).add(mem::size_of::<acpi_table_header>());
        number_of_tables =
            (((*gbl_xsdt).header.length - mem::size_of::<acpi_table_header>() as u32)
                / item_size as u32) as u8;
    } else {
        /* Use RSDT if XSDT is not available */

        item_size = mem::size_of::<u32>() as u8;
        table_data = (gbl_rsdt as *mut u8).add(mem::size_of::<acpi_table_header>());
        number_of_tables =
            (((*gbl_rsdt).header.length - mem::size_of::<acpi_table_header>() as u32)
                / item_size as u32) as u8;
    }

    /* Search RSDT/XSDT for the requested table */

    i = 0;
    while i < number_of_tables as u32 {
        if osl_can_use_xsdt() != 0 {
            table_address = ptr::read_unaligned(table_data as *const u64) as acpi_physical_address;
        } else {
            table_address = ptr::read_unaligned(table_data as *const u32) as acpi_physical_address;
        }

        /* Skip NULL entries in RSDT/XSDT */

        if table_address != 0 {
            status = osl_map_table(table_address as acpi_size, ptr::null_mut(), &mut mapped_table);
            if ACPI_FAILURE(status) {
                return status;
            }

            osl_add_table_to_list((*mapped_table).signature.as_mut_ptr(), 0);
            osl_unmap_table(mapped_table);
        }

        i += 1;
        table_data = table_data.add(item_size as usize);
    }

    AE_OK
}

/******************************************************************************
 *
 * FUNCTION:    osl_get_bios_table
 *
 *****************************************************************************/

unsafe fn osl_get_bios_table(
    mut signature: *mut c_char,
    instance: u32,
    table: *mut *mut acpi_table_header,
    address: *mut acpi_physical_address,
) -> acpi_status {
    let mut local_table: *mut acpi_table_header = ptr::null_mut();
    let mut mapped_table: *mut acpi_table_header = ptr::null_mut();
    let mut table_data: *mut u8;
    let number_of_tables: u8;
    let item_size: u8;
    let mut current_instance: u32 = 0;
    let mut table_address: acpi_physical_address = 0;
    let mut first_table_address: acpi_physical_address = 0;
    let mut table_length: u32 = 0;
    let mut status: acpi_status = AE_OK;
    let mut i: u32;

    /* Handle special tables whose addresses are not in RSDT/XSDT */

    if ACPI_COMPARE_NAMESEG(signature, ACPI_RSDP_NAME)
        || ACPI_COMPARE_NAMESEG(signature, ACPI_SIG_RSDT)
        || ACPI_COMPARE_NAMESEG(signature, ACPI_SIG_XSDT)
        || ACPI_COMPARE_NAMESEG(signature, ACPI_SIG_DSDT)
        || ACPI_COMPARE_NAMESEG(signature, ACPI_SIG_FACS)
    {
        loop {
            table_address = 0;

            /*
             * Get the appropriate address, either 32-bit or 64-bit. Be very
             * careful about the FADT length and validate table addresses.
             * Note: The 64-bit addresses have priority.
             */
            if ACPI_COMPARE_NAMESEG(signature, ACPI_SIG_DSDT) {
                if current_instance < 2 {
                    if (*gbl_fadt).header.length >= MIN_FADT_FOR_XDSDT
                        && (*gbl_fadt).Xdsdt != 0
                        && current_instance == 0
                    {
                        table_address = (*gbl_fadt).Xdsdt as acpi_physical_address;
                    } else if (*gbl_fadt).header.length >= MIN_FADT_FOR_DSDT
                        && (*gbl_fadt).dsdt as acpi_physical_address != first_table_address
                    {
                        table_address = (*gbl_fadt).dsdt as acpi_physical_address;
                    }
                }
            } else if ACPI_COMPARE_NAMESEG(signature, ACPI_SIG_FACS) {
                if current_instance < 2 {
                    if (*gbl_fadt).header.length >= MIN_FADT_FOR_XFACS
                        && (*gbl_fadt).Xfacs != 0
                        && current_instance == 0
                    {
                        table_address = (*gbl_fadt).Xfacs as acpi_physical_address;
                    } else if (*gbl_fadt).header.length >= MIN_FADT_FOR_FACS
                        && (*gbl_fadt).facs as acpi_physical_address != first_table_address
                    {
                        table_address = (*gbl_fadt).facs as acpi_physical_address;
                    }
                }
            } else if ACPI_COMPARE_NAMESEG(signature, ACPI_SIG_XSDT) {
                if gbl_revision == 0 {
                    return AE_BAD_SIGNATURE;
                }
                if current_instance == 0 {
                    table_address = gbl_rsdp.xsdt_physical_address as acpi_physical_address;
                }
            } else if ACPI_COMPARE_NAMESEG(signature, ACPI_SIG_RSDT) {
                if current_instance == 0 {
                    table_address = gbl_rsdp.rsdt_physical_address as acpi_physical_address;
                }
            } else if current_instance == 0 {
                table_address = gbl_rsdp_address as acpi_physical_address;
                signature = ACPI_SIG_RSDP;
            }

            if table_address == 0 {
                break;
            }

            /* Now we can get the requested special table */

            status = osl_map_table(table_address as acpi_size, signature, &mut mapped_table);
            if ACPI_FAILURE(status) {
                return status;
            }

            table_length = ap_get_table_length(mapped_table);
            if first_table_address == 0 {
                first_table_address = table_address;
            }

            /* Match table instance */

            if current_instance != instance {
                osl_unmap_table(mapped_table);
                mapped_table = ptr::null_mut();
                current_instance += 1;
                continue;
            }
            break;
        }
    } else {
        /* Case for a normal ACPI table */

        if osl_can_use_xsdt() != 0 {
            item_size = mem::size_of::<u64>() as u8;
            table_data = (gbl_xsdt as *mut u8).add(mem::size_of::<acpi_table_header>());
            number_of_tables =
                (((*gbl_xsdt).header.length - mem::size_of::<acpi_table_header>() as u32)
                    / item_size as u32) as u8;
        } else {
            /* Use RSDT if XSDT is not available */

            item_size = mem::size_of::<u32>() as u8;
            table_data = (gbl_rsdt as *mut u8).add(mem::size_of::<acpi_table_header>());
            number_of_tables =
                (((*gbl_rsdt).header.length - mem::size_of::<acpi_table_header>() as u32)
                    / item_size as u32) as u8;
        }

        /* Search RSDT/XSDT for the requested table */

        i = 0;
        while i < number_of_tables as u32 {
            if osl_can_use_xsdt() != 0 {
                table_address =
                    ptr::read_unaligned(table_data as *const u64) as acpi_physical_address;
            } else {
                table_address =
                    ptr::read_unaligned(table_data as *const u32) as acpi_physical_address;
            }

            /* Skip NULL entries in RSDT/XSDT */

            if table_address != 0 {
                status =
                    osl_map_table(table_address as acpi_size, ptr::null_mut(), &mut mapped_table);
                if ACPI_FAILURE(status) {
                    return status;
                }
                table_length = (*mapped_table).length;

                /* Does this table match the requested signature? */

                if !ACPI_COMPARE_NAMESEG((*mapped_table).signature.as_ptr(), signature) {
                    osl_unmap_table(mapped_table);
                    mapped_table = ptr::null_mut();
                    i += 1;
                    table_data = table_data.add(item_size as usize);
                    continue;
                }

                /* Match table instance (for SSDT/UEFI tables) */

                if current_instance != instance {
                    osl_unmap_table(mapped_table);
                    mapped_table = ptr::null_mut();
                    current_instance += 1;
                    i += 1;
                    table_data = table_data.add(item_size as usize);
                    continue;
                }

                break;
            }

            i += 1;
            table_data = table_data.add(item_size as usize);
        }
    }

    if mapped_table.is_null() {
        return AE_LIMIT;
    }

    if table_length == 0 {
        status = AE_BAD_HEADER;
        osl_unmap_table(mapped_table);
        return status;
    }

    /* Copy table to local buffer and return it */

    local_table = calloc(1, table_length as usize) as *mut acpi_table_header;
    if local_table.is_null() {
        status = AE_NO_MEMORY;
        osl_unmap_table(mapped_table);
        return status;
    }

    memcpy(
        local_table as *mut c_void,
        mapped_table as *const c_void,
        table_length as usize,
    );
    *address = table_address;
    *table = local_table;

    osl_unmap_table(mapped_table);
    status
}

/******************************************************************************
 *
 * FUNCTION:    osl_list_customized_tables
 *
 *****************************************************************************/

unsafe fn osl_list_customized_tables(directory: *mut c_char) -> acpi_status {
    let table_dir: *mut c_void;
    let mut instance: u32 = 0;
    let mut temp_name: [c_char; ACPI_NAMESEG_SIZE] = [0; ACPI_NAMESEG_SIZE];
    let mut filename: *mut c_char;
    let mut status: acpi_status = AE_OK;

    /* Open the requested directory */

    table_dir =
        acpi_os_open_directory(directory, b"*\0".as_ptr() as *const c_char, REQUEST_FILE_ONLY);
    if table_dir.is_null() {
        return osl_get_last_status(AE_NOT_FOUND);
    }

    /* Examine all entries in this directory */

    loop {
        filename = acpi_os_get_next_filename(table_dir);
        if filename.is_null() {
            break;
        }

        /* Extract table name and instance number */

        status = osl_table_name_from_file(filename, temp_name.as_mut_ptr(), &mut instance);

        /* Ignore meaningless files */

        if ACPI_FAILURE(status) {
            continue;
        }

        /* Add new info node to global table list */

        status = osl_add_table_to_list(temp_name.as_mut_ptr(), instance);
        if ACPI_FAILURE(status) {
            break;
        }
    }

    acpi_os_close_directory(table_dir);
    status
}

/******************************************************************************
 *
 * FUNCTION:    osl_map_table
 *
 *****************************************************************************/

unsafe fn osl_map_table(
    address: acpi_size,
    signature: *mut c_char,
    table: *mut *mut acpi_table_header,
) -> acpi_status {
    let mut mapped_table: *mut acpi_table_header;
    let length: u32;

    if address == 0 {
        return AE_BAD_ADDRESS;
    }

    /*
     * Map the header so we can get the table length.
     * Use sizeof (struct acpi_table_header) as:
     * 1. it is bigger than 24 to include RSDP->Length
     * 2. it is smaller than sizeof (struct acpi_table_rsdp)
     */
    mapped_table =
        acpi_os_map_memory(address as acpi_physical_address, mem::size_of::<acpi_table_header>())
            as *mut acpi_table_header;
    if mapped_table.is_null() {
        let (hi, lo) = ACPI_FORMAT_UINT64(address as u64);
        fprintf(
            stderr,
            b"Could not map table header at 0x%8.8X%8.8X\n\0".as_ptr() as *const c_char,
            hi,
            lo,
        );
        return osl_get_last_status(AE_BAD_ADDRESS);
    }

    /* If specified, signature must match */

    if !signature.is_null() {
        if ACPI_VALIDATE_RSDP_SIG(signature) {
            if !ACPI_VALIDATE_RSDP_SIG((*mapped_table).signature.as_ptr()) {
                acpi_os_unmap_memory(
                    mapped_table as *mut c_void,
                    mem::size_of::<acpi_table_header>(),
                );
                return AE_BAD_SIGNATURE;
            }
        } else if !ACPI_COMPARE_NAMESEG(signature, (*mapped_table).signature.as_ptr()) {
            acpi_os_unmap_memory(
                mapped_table as *mut c_void,
                mem::size_of::<acpi_table_header>(),
            );
            return AE_BAD_SIGNATURE;
        }
    }

    /* Map the entire table */

    length = ap_get_table_length(mapped_table);
    acpi_os_unmap_memory(
        mapped_table as *mut c_void,
        mem::size_of::<acpi_table_header>(),
    );
    if length == 0 {
        return AE_BAD_HEADER;
    }

    mapped_table =
        acpi_os_map_memory(address as acpi_physical_address, length as acpi_size)
            as *mut acpi_table_header;
    if mapped_table.is_null() {
        let (hi, lo) = ACPI_FORMAT_UINT64(address as u64);
        fprintf(
            stderr,
            b"Could not map table at 0x%8.8X%8.8X length %8.8X\n\0".as_ptr() as *const c_char,
            hi,
            lo,
            length,
        );
        return osl_get_last_status(AE_INVALID_TABLE_LENGTH);
    }

    ap_is_valid_checksum(mapped_table);

    *table = mapped_table;
    AE_OK
}

/******************************************************************************
 *
 * FUNCTION:    osl_unmap_table
 *
 *****************************************************************************/

unsafe fn osl_unmap_table(table: *mut acpi_table_header) {
    if !table.is_null() {
        acpi_os_unmap_memory(table as *mut c_void, ap_get_table_length(table) as acpi_size);
    }
}

/******************************************************************************
 *
 * FUNCTION:    osl_table_name_from_file
 *
 *****************************************************************************/

unsafe fn osl_table_name_from_file(
    filename: *mut c_char,
    signature: *mut c_char,
    instance: *mut u32,
) -> acpi_status {
    /* Ignore meaningless files */

    if strlen(filename) < ACPI_NAMESEG_SIZE {
        return AE_BAD_SIGNATURE;
    }

    /* Extract instance number */

    if isdigit(*filename.add(ACPI_NAMESEG_SIZE) as c_int) != 0 {
        sscanf(
            filename.add(ACPI_NAMESEG_SIZE),
            b"%u\0".as_ptr() as *const c_char,
            instance,
        );
    } else if strlen(filename) != ACPI_NAMESEG_SIZE {
        return AE_BAD_SIGNATURE;
    } else {
        *instance = 0;
    }

    /* Extract signature */

    ACPI_COPY_NAMESEG(signature, filename);
    AE_OK
}

/******************************************************************************
 *
 * FUNCTION:    osl_read_table_from_file
 *
 *****************************************************************************/

unsafe fn osl_read_table_from_file(
    filename: *mut c_char,
    file_offset: acpi_size,
    table: *mut *mut acpi_table_header,
) -> acpi_status {
    let table_file: *mut FILE;
    let mut header: acpi_table_header = mem::zeroed();
    let mut local_table: *mut acpi_table_header = ptr::null_mut();
    let table_length: u32;
    let mut count: usize;
    let mut status: acpi_status = AE_OK;

    /* Open the file */

    table_file = fopen(filename, b"rb\0".as_ptr() as *const c_char);
    if table_file.is_null() {
        fprintf(
            stderr,
            b"Could not open table file: %s\n\0".as_ptr() as *const c_char,
            filename,
        );
        return osl_get_last_status(AE_NOT_FOUND);
    }

    fseek(table_file, file_offset as c_long, SEEK_SET);

    /* Read the Table header to get the table length */

    count = fread(
        &mut header as *mut _ as *mut c_void,
        1,
        mem::size_of::<acpi_table_header>(),
        table_file,
    );
    if count != mem::size_of::<acpi_table_header>() {
        fprintf(
            stderr,
            b"Could not read table header: %s\n\0".as_ptr() as *const c_char,
            filename,
        );
        status = AE_BAD_HEADER;
        fclose(table_file);
        *table = local_table;
        return status;
    }

    /*
     * Original conditional code under ACPI_OBSOLETE_FUNCTIONS checked an
     * optional signature argument. The active C function has no such parameter,
     * so the disabled block is preserved here as conditional intent only.
     */

    table_length = ap_get_table_length(&mut header);
    if table_length == 0 {
        status = AE_BAD_HEADER;
        fclose(table_file);
        *table = local_table;
        return status;
    }

    /* Read the entire table into a local buffer */

    local_table = calloc(1, table_length as usize) as *mut acpi_table_header;
    if local_table.is_null() {
        fprintf(
            stderr,
            b"%4.4s: Could not allocate buffer for table of length %X\n\0".as_ptr()
                as *const c_char,
            header.signature.as_ptr(),
            table_length,
        );
        status = AE_NO_MEMORY;
        fclose(table_file);
        *table = local_table;
        return status;
    }

    fseek(table_file, file_offset as c_long, SEEK_SET);

    count = fread(
        local_table as *mut c_void,
        1,
        table_length as usize,
        table_file,
    );
    if count != table_length as usize {
        fprintf(
            stderr,
            b"%4.4s: Could not read table content\n\0".as_ptr() as *const c_char,
            header.signature.as_ptr(),
        );
        status = AE_INVALID_TABLE_LENGTH;
        fclose(table_file);
        *table = local_table;
        return status;
    }

    /* Validate checksum */

    ap_is_valid_checksum(local_table);

    fclose(table_file);
    *table = local_table;
    status
}

/******************************************************************************
 *
 * FUNCTION:    osl_get_customized_table
 *
 *****************************************************************************/

unsafe fn osl_get_customized_table(
    pathname: *mut c_char,
    signature: *mut c_char,
    instance: u32,
    table: *mut *mut acpi_table_header,
    address: *mut acpi_physical_address,
) -> acpi_status {
    let table_dir: *mut c_void;
    let mut current_instance: u32 = 0;
    let mut temp_name: [c_char; ACPI_NAMESEG_SIZE] = [0; ACPI_NAMESEG_SIZE];
    let mut table_filename: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut filename: *mut c_char;
    let mut status: acpi_status;

    /* Open the directory for customized tables */

    table_dir =
        acpi_os_open_directory(pathname, b"*\0".as_ptr() as *const c_char, REQUEST_FILE_ONLY);
    if table_dir.is_null() {
        return osl_get_last_status(AE_NOT_FOUND);
    }

    /* Attempt to find the table in the directory */

    loop {
        filename = acpi_os_get_next_filename(table_dir);
        if filename.is_null() {
            break;
        }

        /* Ignore meaningless files */

        if !ACPI_COMPARE_NAMESEG(filename, signature) {
            continue;
        }

        /* Extract table name and instance number */

        status = osl_table_name_from_file(filename, temp_name.as_mut_ptr(), &mut current_instance);

        /* Ignore meaningless files */

        if ACPI_FAILURE(status) || current_instance != instance {
            continue;
        }

        /* Create the table pathname */

        if instance != 0 {
            sprintf(
                table_filename.as_mut_ptr(),
                b"%s/%4.4s%d\0".as_ptr() as *const c_char,
                pathname,
                temp_name.as_mut_ptr(),
                instance,
            );
        } else {
            sprintf(
                table_filename.as_mut_ptr(),
                b"%s/%4.4s\0".as_ptr() as *const c_char,
                pathname,
                temp_name.as_mut_ptr(),
            );
        }
        break;
    }

    acpi_os_close_directory(table_dir);

    if filename.is_null() {
        return AE_LIMIT;
    }

    /* There is no physical address saved for customized tables, use zero */

    *address = 0;
    status = osl_read_table_from_file(table_filename.as_mut_ptr(), 0, table);

    status
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
