// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the EFI and kernel translation units.

use core::ptr;

extern "C" {
    fn get_efi_config_table(guid: efi_guid_t) -> *mut efi_unaccepted_memory;
    fn efi_memdesc_ptr(map: *mut u8, desc_size: usize, index: i32) -> *mut efi_memory_desc_t;
    fn efi_bs_call_allocate_pool(ty: u32, size: usize, buffer: *mut *mut core::ffi::c_void) -> efi_status_t;
    fn efi_bs_call_install_configuration_table(
        guid: *const efi_guid_t,
        table: *mut efi_unaccepted_memory,
    ) -> efi_status_t;
    fn efi_bs_call_free_pool(buffer: *mut efi_unaccepted_memory) -> efi_status_t;
    fn efi_err(message: *const u8);
    fn arch_accept_memory(start: u64, end: u64);
    fn bitmap_set(bitmap: *mut u8, start: usize, bits: usize);
    fn bitmap_clear(bitmap: *mut u8, start: usize, bits: usize);
    fn find_next_bit(bitmap: *const u8, size: usize, offset: usize) -> usize;
}

#[no_mangle]
pub static mut unaccepted_table: *mut efi_unaccepted_memory = core::ptr::null_mut();

// These types and constants are provided by the included EFI/kernel headers.
#[repr(C)]
pub struct efi_unaccepted_memory {
    pub version: u32,
    pub unit_size: u32,
    pub phys_base: u64,
    pub size: usize,
    pub bitmap: [u8; 0],
}

#[allow(non_camel_case_types)]
type efi_status_t = usize;
type phys_addr_t = u64;
type u64_ = u64;
type efi_guid_t = [u8; 16];

#[repr(C)]
pub struct efi_memory_desc_t {
    pub ty: u32,
    pub phys_addr: u64,
    pub num_pages: u64,
}

#[repr(C)]
pub struct efi_boot_memmap {
    pub map: *mut u8,
    pub desc_size: usize,
}

extern "C" {
    static LINUX_EFI_UNACCEPTED_MEM_TABLE_GUID: efi_guid_t;
}

const EFI_SUCCESS: efi_status_t = 0;
const EFI_UNSUPPORTED: efi_status_t = 3;
const EFI_ACPI_RECLAIM_MEMORY: u32 = 9;
const EFI_UNACCEPTED_MEMORY: u32 = 0x8000_0000;
const EFI_UNACCEPTED_UNIT_SIZE: u64 = 2 * 1024 * 1024;
const PAGE_SIZE: u64 = 4096;
const BITS_PER_BYTE: u64 = 8;

#[inline]
const fn round_down(value: u64, align: u64) -> u64 {
    value & !(align - 1)
}

#[inline]
const fn round_up(value: u64, align: u64) -> u64 {
    (value + align - 1) & !(align - 1)
}

#[inline]
const fn div_round_up(value: u64, divisor: u64) -> u64 {
    (value + divisor - 1) / divisor
}

pub unsafe fn allocate_unaccepted_bitmap(nr_desc: u32, map: *mut efi_boot_memmap) -> efi_status_t {
    let unaccepted_table_guid = LINUX_EFI_UNACCEPTED_MEM_TABLE_GUID;
    let mut unaccepted_start = u64::MAX;
    let mut unaccepted_end = 0u64;
    let bitmap_size: u64;
    let status: efi_status_t;

    // Check if the table is already installed
    unaccepted_table = get_efi_config_table(unaccepted_table_guid);
    if !unaccepted_table.is_null() {
        if (*unaccepted_table).version != 1 {
            efi_err(b"Unknown version of unaccepted memory table\0".as_ptr());
            return EFI_UNSUPPORTED;
        }
        return EFI_SUCCESS;
    }

    // Check if there's any unaccepted memory and find the max address
    for i in 0..nr_desc {
        let d = efi_memdesc_ptr((*map).map, (*map).desc_size, i as i32);
        if (*d).ty != EFI_UNACCEPTED_MEMORY {
            continue;
        }
        unaccepted_start = core::cmp::min(unaccepted_start, (*d).phys_addr);
        unaccepted_end = core::cmp::max(
            unaccepted_end,
            (*d).phys_addr + (*d).num_pages * PAGE_SIZE,
        );
    }

    if unaccepted_start == u64::MAX {
        return EFI_SUCCESS;
    }

    unaccepted_start = round_down(unaccepted_start, EFI_UNACCEPTED_UNIT_SIZE);
    unaccepted_end = round_up(unaccepted_end, EFI_UNACCEPTED_UNIT_SIZE);
    bitmap_size = div_round_up(
        unaccepted_end - unaccepted_start,
        EFI_UNACCEPTED_UNIT_SIZE * BITS_PER_BYTE,
    );

    status = efi_bs_call_allocate_pool(
        EFI_ACPI_RECLAIM_MEMORY,
        core::mem::size_of::<efi_unaccepted_memory>() + bitmap_size as usize,
        &mut unaccepted_table as *mut _ as *mut *mut core::ffi::c_void,
    );
    if status != EFI_SUCCESS {
        efi_err(b"Failed to allocate unaccepted memory config table\0".as_ptr());
        return status;
    }

    (*unaccepted_table).version = 1;
    (*unaccepted_table).unit_size = EFI_UNACCEPTED_UNIT_SIZE as u32;
    (*unaccepted_table).phys_base = unaccepted_start;
    (*unaccepted_table).size = bitmap_size as usize;
    ptr::write_bytes((*unaccepted_table).bitmap.as_mut_ptr(), 0, bitmap_size as usize);

    status = efi_bs_call_install_configuration_table(&unaccepted_table_guid, unaccepted_table);
    if status != EFI_SUCCESS {
        efi_bs_call_free_pool(unaccepted_table);
        efi_err(b"Failed to install unaccepted memory config table!\0".as_ptr());
    }
    status
}

pub unsafe fn process_unaccepted_memory(mut start: u64, mut end: u64) {
    let unit_size = (*unaccepted_table).unit_size as u64;
    let unit_mask = unit_size - 1;
    let bitmap_size = (*unaccepted_table).size as u64;

    if end - start < 2 * unit_size {
        arch_accept_memory(start, end);
        return;
    }
    if start & unit_mask != 0 {
        arch_accept_memory(start, round_up(start, unit_size));
        start = round_up(start, unit_size);
    }
    if end & unit_mask != 0 {
        arch_accept_memory(round_down(end, unit_size), end);
        end = round_down(end, unit_size);
    }
    if start < (*unaccepted_table).phys_base {
        arch_accept_memory(start, core::cmp::min((*unaccepted_table).phys_base, end));
        start = (*unaccepted_table).phys_base;
    }
    if end < (*unaccepted_table).phys_base {
        return;
    }
    start -= (*unaccepted_table).phys_base;
    end -= (*unaccepted_table).phys_base;
    if end > bitmap_size * unit_size * BITS_PER_BYTE {
        let phys_start = bitmap_size * unit_size * BITS_PER_BYTE + (*unaccepted_table).phys_base;
        let phys_end = end + (*unaccepted_table).phys_base;
        arch_accept_memory(phys_start, phys_end);
        end = bitmap_size * unit_size * BITS_PER_BYTE;
    }
    bitmap_set((*unaccepted_table).bitmap.as_mut_ptr(), (start / unit_size) as usize, ((end - start) / unit_size) as usize);
}

pub unsafe fn accept_memory(mut start: phys_addr_t, size: usize) {
    if unaccepted_table.is_null() {
        return;
    }
    let unit_size = (*unaccepted_table).unit_size as u64;
    let mut end = start + size as u64;
    if start < (*unaccepted_table).phys_base { start = (*unaccepted_table).phys_base; }
    if end < (*unaccepted_table).phys_base { return; }
    start -= (*unaccepted_table).phys_base;
    end -= (*unaccepted_table).phys_base;
    if end > (*unaccepted_table).size as u64 * unit_size * BITS_PER_BYTE {
        end = (*unaccepted_table).size as u64 * unit_size * BITS_PER_BYTE;
    }
    let range_start = (start / unit_size) as usize;
    let bitmap_size = div_round_up(end, unit_size) as usize;
    // C's for_each_set_bitrange_from() is provided by the kernel bitmap helpers.
    let mut range_start = range_start;
    while range_start < bitmap_size {
        range_start = find_next_bit((*unaccepted_table).bitmap.as_ptr(), bitmap_size, range_start);
        if range_start >= bitmap_size {
            break;
        }
        let range_end = find_next_bit((*unaccepted_table).bitmap.as_ptr(), bitmap_size, range_start + 1);
        let phys_start = range_start as u64 * unit_size + (*unaccepted_table).phys_base;
        let phys_end = range_end as u64 * unit_size + (*unaccepted_table).phys_base;
        arch_accept_memory(phys_start, phys_end);
        bitmap_clear(
            (*unaccepted_table).bitmap.as_mut_ptr(),
            range_start,
            range_end - range_start,
        );
        range_start = range_end;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
