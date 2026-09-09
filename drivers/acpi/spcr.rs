// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, Intel Corporation
 * Copyright (c) 2015, Red Hat, Inc.
 * Copyright (c) 2015, 2016 Linaro Ltd.
 */

// Dependency declarations and kernel-provided symbols are supplied by other files.

/*
 * Erratum 44 for QDF2432v1 and QDF2400v1 SoCs describes the BUSY bit as
 * occasionally getting stuck as 1. To avoid the potential for a hang, check
 * TXFE == 0 instead of BUSY == 1. This may not be suitable for all UART
 * implementations, so only do so if an affected platform is detected in
 * acpi_parse_spcr().
 */
pub static mut qdf2400_e44_present: bool = false;

/*
 * Some Qualcomm Datacenter Technologies SoCs have a defective UART BUSY bit.
 * Detect them by examining the OEM fields in the SPCR header, similar to PCI
 * quirk detection in pci_mcfg.c.
 */
unsafe fn qdf2400_erratum_44_present(h: *mut acpi_table_header) -> bool {
    if libc::memcmp((*h).oem_id.as_ptr() as *const libc::c_void, b"QCOM  \0".as_ptr() as *const libc::c_void, ACPI_OEM_ID_SIZE) != 0 {
        return false;
    }

    if libc::memcmp((*h).oem_table_id.as_ptr() as *const libc::c_void, b"QDF2432 \0".as_ptr() as *const libc::c_void, ACPI_OEM_TABLE_ID_SIZE) == 0 {
        return true;
    }

    if libc::memcmp((*h).oem_table_id.as_ptr() as *const libc::c_void, b"QDF2400 \0".as_ptr() as *const libc::c_void, ACPI_OEM_TABLE_ID_SIZE) == 0 && (*h).oem_revision == 1 {
        return true;
    }

    false
}

/*
 * APM X-Gene v1 and v2 UART hardware is an 16550 like device but has its
 * register aligned to 32-bit. In addition, the BIOS also encoded the access
 * width to be 8 bits. This function detects this errata condition.
 */
unsafe fn xgene_8250_erratum_present(tb: *mut acpi_table_spcr) -> bool {
    let mut xgene_8250 = false;

    if (*tb).interface_type != ACPI_DBG2_16550_COMPATIBLE {
        return false;
    }

    if libc::memcmp((*tb).header.oem_id.as_ptr() as *const libc::c_void, b"APMC0D\0".as_ptr() as *const libc::c_void, ACPI_OEM_ID_SIZE) != 0
        && libc::memcmp((*tb).header.oem_id.as_ptr() as *const libc::c_void, b"HPE   \0".as_ptr() as *const libc::c_void, ACPI_OEM_ID_SIZE) != 0
    {
        return false;
    }

    if libc::memcmp((*tb).header.oem_table_id.as_ptr() as *const libc::c_void, b"XGENESPC".as_ptr() as *const libc::c_void, ACPI_OEM_TABLE_ID_SIZE) == 0 && (*tb).header.oem_revision == 0 {
        xgene_8250 = true;
    }

    if libc::memcmp((*tb).header.oem_table_id.as_ptr() as *const libc::c_void, b"ProLiant".as_ptr() as *const libc::c_void, ACPI_OEM_TABLE_ID_SIZE) == 0 && (*tb).header.oem_revision == 1 {
        xgene_8250 = true;
    }

    xgene_8250
}

/// Parse ACPI SPCR table and add preferred console.
pub unsafe fn acpi_parse_spcr(enable_earlycon: bool, enable_console: bool) -> libc::c_int {
    static mut opts: [libc::c_char; 64] = [0; 64];
    let mut table: *mut acpi_table_spcr = core::ptr::null_mut();
    let mut status: acpi_status;
    let mut uart: *const libc::c_char;
    let mut iotype: *const libc::c_char;
    let mut baud_rate: libc::c_int;
    let mut err: libc::c_int;

    if acpi_disabled {
        return -ENODEV;
    }

    status = acpi_get_table(ACPI_SIG_SPCR, 0, &mut table as *mut _ as *mut *mut acpi_table_header);
    if ACPI_FAILURE(status) {
        return -ENOENT;
    }

    if (*table).header.revision < 2 {
        pr_info!("SPCR table version {}\n", (*table).header.revision);
    }

    if (*table).serial_port.space_id == ACPI_ADR_SPACE_SYSTEM_MEMORY {
        let mut bit_width: u32 = (*table).serial_port.access_width;
        if bit_width > ACPI_ACCESS_BIT_MAX {
            pr_err!("Unacceptable wide SPCR Access Width. Defaulting to byte size\n");
            bit_width = ACPI_ACCESS_BIT_DEFAULT;
        }
        match ACPI_ACCESS_BIT_WIDTH(bit_width) {
            8 => iotype = b"mmio\0".as_ptr() as *const libc::c_char,
            16 => iotype = b"mmio16\0".as_ptr() as *const libc::c_char,
            32 => iotype = b"mmio32\0".as_ptr() as *const libc::c_char,
            _ => {
                pr_err!("Unexpected SPCR Access Width. Defaulting to byte size\n");
                iotype = b"mmio\0".as_ptr() as *const libc::c_char;
            }
        }
    } else {
        iotype = b"io\0".as_ptr() as *const libc::c_char;
    }

    match (*table).interface_type {
        ACPI_DBG2_ARM_SBSA_32BIT => {
            iotype = b"mmio32\0".as_ptr() as *const libc::c_char;
            uart = b"pl011\0".as_ptr() as *const libc::c_char;
        }
        ACPI_DBG2_ARM_PL011 | ACPI_DBG2_ARM_SBSA_GENERIC | ACPI_DBG2_BCM2835 => uart = b"pl011\0".as_ptr() as *const libc::c_char,
        ACPI_DBG2_16550_COMPATIBLE | ACPI_DBG2_16550_SUBSET | ACPI_DBG2_16550_WITH_GAS | ACPI_DBG2_16550_NVIDIA => uart = b"uart\0".as_ptr() as *const libc::c_char,
        ACPI_DBG2_RISCV_SBI_CON => uart = b"sbi\0".as_ptr() as *const libc::c_char,
        _ => {
            err = -ENOENT;
            acpi_put_table(&mut (*table).header);
            return err;
        }
    }

    if (*table).header.revision >= 4 && (*table).precise_baudrate != 0 {
        baud_rate = (*table).precise_baudrate as libc::c_int;
    } else {
        baud_rate = match (*table).baud_rate {
            0 => 0,
            3 => 9600,
            4 => 19200,
            6 => 57600,
            7 => 115200,
            _ => {
                acpi_put_table(&mut (*table).header);
                return -ENOENT;
            }
        };
    }

    if qdf2400_erratum_44_present(&mut (*table).header) {
        qdf2400_e44_present = true;
        if enable_earlycon { uart = b"qdf2400_e44\0".as_ptr() as *const libc::c_char; }
    }

    if xgene_8250_erratum_present(table) {
        iotype = b"mmio32\0".as_ptr() as *const libc::c_char;
        baud_rate = 0;
    }

    if baud_rate == 0 {
        snprintf(opts.as_mut_ptr(), opts.len(), b"%s,%s,0x%llx\0".as_ptr() as *const libc::c_char, uart, iotype, (*table).serial_port.address);
    } else {
        snprintf(opts.as_mut_ptr(), opts.len(), b"%s,%s,0x%llx,%d\0".as_ptr() as *const libc::c_char, uart, iotype, (*table).serial_port.address, baud_rate);
    }

    pr_info!("console: {}\n", opts.as_ptr());
    if enable_earlycon { setup_earlycon(opts.as_ptr()); }
    err = if enable_console { add_preferred_console(uart, 0, opts.as_ptr().add(strlen(uart) + 1)) } else { 0 };
    acpi_put_table(&mut (*table).header);
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
