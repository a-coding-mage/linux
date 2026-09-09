// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: hwvalid - I/O request validation

// Dependency declarations supplied by the ACPICA headers are intentionally
// left external to this translation.

#[repr(C)]
struct acpi_port_info {
    name: *const u8,
    start: u16,
    end: u16,
    osi_dependency: u32,
}

const ACPI_PORT_INFO_ENTRIES: usize = 17;

static acpi_protected_ports: [acpi_port_info; ACPI_PORT_INFO_ENTRIES] = [
    acpi_port_info { name: b"DMA\0".as_ptr(), start: 0x0000, end: 0x000F, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"PIC0\0".as_ptr(), start: 0x0020, end: 0x0021, osi_dependency: ACPI_ALWAYS_ILLEGAL },
    acpi_port_info { name: b"PIT1\0".as_ptr(), start: 0x0040, end: 0x0043, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"PIT2\0".as_ptr(), start: 0x0048, end: 0x004B, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"RTC\0".as_ptr(), start: 0x0070, end: 0x0071, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"CMOS\0".as_ptr(), start: 0x0074, end: 0x0076, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"DMA1\0".as_ptr(), start: 0x0081, end: 0x0083, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"DMA1L\0".as_ptr(), start: 0x0087, end: 0x0087, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"DMA2\0".as_ptr(), start: 0x0089, end: 0x008B, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"DMA2L\0".as_ptr(), start: 0x008F, end: 0x008F, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"ARBC\0".as_ptr(), start: 0x0090, end: 0x0091, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"SETUP\0".as_ptr(), start: 0x0093, end: 0x0094, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"POS\0".as_ptr(), start: 0x0096, end: 0x0097, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"PIC1\0".as_ptr(), start: 0x00A0, end: 0x00A1, osi_dependency: ACPI_ALWAYS_ILLEGAL },
    acpi_port_info { name: b"IDMA\0".as_ptr(), start: 0x00C0, end: 0x00DF, osi_dependency: ACPI_OSI_WIN_XP },
    acpi_port_info { name: b"ELCR\0".as_ptr(), start: 0x04D0, end: 0x04D1, osi_dependency: ACPI_ALWAYS_ILLEGAL },
    acpi_port_info { name: b"PCI\0".as_ptr(), start: 0x0CF8, end: 0x0CFF, osi_dependency: ACPI_OSI_WIN_XP },
];

extern "C" {
    static mut acpi_gbl_osi_data: u32;
    static mut acpi_gbl_truncate_io_addresses: bool;
    fn acpi_os_read_port(address: acpi_io_address, value: *mut u32, width: u32) -> acpi_status;
    fn acpi_os_write_port(address: acpi_io_address, value: u32, width: u32) -> acpi_status;
}

unsafe fn acpi_hw_validate_io_request(address: acpi_io_address, bit_width: u32) -> acpi_status {
    if bit_width != 8 && bit_width != 16 && bit_width != 32 { return AE_BAD_PARAMETER; }
    let byte_width = bit_width / 8;
    let last_address = address + byte_width as acpi_io_address - 1;
    if last_address > ACPI_UINT16_MAX as acpi_io_address { return AE_LIMIT; }
    if address > acpi_protected_ports[ACPI_PORT_INFO_ENTRIES - 1].end as acpi_io_address { return AE_OK; }
    let mut i = 0;
    while i < ACPI_PORT_INFO_ENTRIES {
        let port_info = &acpi_protected_ports[i];
        if address <= port_info.end as acpi_io_address && last_address >= port_info.start as acpi_io_address {
            if port_info.osi_dependency == ACPI_ALWAYS_ILLEGAL || acpi_gbl_osi_data == port_info.osi_dependency { return AE_AML_ILLEGAL_ADDRESS; }
        }
        if last_address <= port_info.end as acpi_io_address { break; }
        i += 1;
    }
    AE_OK
}

pub unsafe fn acpi_hw_read_port(mut address: acpi_io_address, value: *mut u32, width: u32) -> acpi_status {
    if acpi_gbl_truncate_io_addresses { address &= ACPI_UINT16_MAX as acpi_io_address; }
    let mut status = acpi_hw_validate_io_request(address, width);
    if ACPI_SUCCESS(status) { return acpi_os_read_port(address, value, width); }
    if status != AE_AML_ILLEGAL_ADDRESS { return status; }
    let mut one_byte = 0u32;
    let mut i = 0;
    *value = 0;
    while i < width {
        if acpi_hw_validate_io_request(address, 8) == AE_OK {
            status = acpi_os_read_port(address, &mut one_byte, 8);
            if ACPI_FAILURE(status) { return status; }
            *value |= one_byte << i;
        }
        address += 1;
        i += 8;
    }
    AE_OK
}

pub unsafe fn acpi_hw_write_port(mut address: acpi_io_address, value: u32, width: u32) -> acpi_status {
    if acpi_gbl_truncate_io_addresses { address &= ACPI_UINT16_MAX as acpi_io_address; }
    let mut status = acpi_hw_validate_io_request(address, width);
    if ACPI_SUCCESS(status) { return acpi_os_write_port(address, value, width); }
    if status != AE_AML_ILLEGAL_ADDRESS { return status; }
    let mut i = 0;
    while i < width {
        if acpi_hw_validate_io_request(address, 8) == AE_OK {
            status = acpi_os_write_port(address, (value >> i) & 0xFF, 8);
            if ACPI_FAILURE(status) { return status; }
        }
        address += 1;
        i += 8;
    }
    AE_OK
}

pub unsafe fn acpi_hw_validate_io_block(mut address: u64, bit_width: u32, mut count: u32) -> acpi_status {
    while count != 0 {
        let status = acpi_hw_validate_io_request(address as acpi_io_address, bit_width);
        if ACPI_FAILURE(status) { return status; }
        address += (bit_width / 8) as u64;
        count -= 1;
    }
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
