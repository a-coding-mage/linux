// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2005 IBM Corporation
 *
 * Authors:
 *	Seiji Munetoh <munetoh@jp.ibm.com>
 *	Stefan Berger <stefanb@us.ibm.com>
 *	Reiner Sailer <sailer@watson.ibm.com>
 *	Kylene Hall <kjhall@us.ibm.com>
 *	Nayna Jain <nayna@linux.vnet.ibm.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * Access to the event log extended by the TCG BIOS of PC platform
 */

#[repr(C)]
pub struct AcpiTcpa {
    pub hdr: acpi_table_header,
    pub platform_class: u16,
    pub _u: AcpiTcpaUnion,
}

#[repr(C)]
pub union AcpiTcpaUnion {
    pub client: AcpiTcpaClientHdr,
    pub server: AcpiTcpaServerHdr,
}

#[repr(C, packed)]
pub struct AcpiTcpaClientHdr {
    pub log_max_len: u32,
    pub log_start_addr: u64,
}

#[repr(C, packed)]
pub struct AcpiTcpaServerHdr {
    pub reserved: u16,
    pub log_max_len: u64,
    pub log_start_addr: u64,
}

extern "C" {
    fn memcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, n: usize) -> i32;
    fn kvfree(address: *mut core::ffi::c_void);
    fn acpi_get_table(
        signature: *const core::ffi::c_char,
        instance: u32,
        table: *mut *mut acpi_table_header,
    ) -> acpi_status;
    fn acpi_put_table(table: *mut acpi_table_header);
    fn acpi_os_map_iomem(phys: u64, size: usize) -> *mut core::ffi::c_void;
    fn acpi_os_unmap_iomem(virt: *mut core::ffi::c_void, size: usize);
    fn memcpy_fromio(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize);
    fn dev_warn(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn devm_add_action(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut core::ffi::c_void),
        data: *mut core::ffi::c_void,
    ) -> i32;
    fn kvmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
}

// Types and constants below are supplied by the surrounding kernel headers.
#[allow(non_camel_case_types)]
pub type acpi_status = u32;
#[allow(non_camel_case_types)]
pub type acpi_table_header = AcpiTableHeader;

#[repr(C)]
pub struct AcpiTableHeader {
    pub signature: [u8; 4],
    pub length: u32,
}

extern "C" {
    static TCG_SPECID_SIG: [u8; 16];
}

#[repr(C)]
pub struct tcg_efi_specid_event_head {
    pub signature: [u8; 16],
}

#[repr(C)]
pub struct tcg_pcr_event {
    pub event: [u8; 0],
}

#[repr(C)]
pub struct acpi_table_tpm2 {
    pub header: AcpiTableHeader,
}

#[repr(C)]
pub struct acpi_tpm2_phy {
    pub log_area_minimum_length: u64,
    pub log_area_start_address: u64,
}

#[repr(C)]
pub struct tpm_bios_log {
    pub bios_event_log: *mut u8,
    pub bios_event_log_end: *mut u8,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tpm_chip {
    pub dev: device,
    pub log: tpm_bios_log,
    pub flags: u32,
    pub acpi_dev_handle: *mut core::ffi::c_void,
}

const ENODEV: i32 = 19;
const EIO: i32 = 5;
const ENOMEM: i32 = 12;
const TPM_CHIP_FLAG_TPM2: u32 = 1 << 1;
const BIOS_SERVER: u16 = 0;
const BIOS_CLIENT: u16 = 1;
const EFI_TCG2_EVENT_LOG_FORMAT_TCG_2: i32 = 2;
const EFI_TCG2_EVENT_LOG_FORMAT_TCG_1_2: i32 = 1;
const GFP_KERNEL: u32 = 0x400cc0;

unsafe fn tpm_is_tpm2_log(bios_event_log: *mut core::ffi::c_void, mut len: u64) -> bool {
    let event_header: *mut tcg_pcr_event;
    let efispecid: *mut tcg_efi_specid_event_head;
    let n: i32;

    if len < core::mem::size_of::<tcg_pcr_event>() as u64 {
        return false;
    }
    len -= core::mem::size_of::<tcg_pcr_event>() as u64;
    event_header = bios_event_log as *mut tcg_pcr_event;

    if len < core::mem::size_of::<tcg_efi_specid_event_head>() as u64 {
        return false;
    }
    efispecid = (*event_header).event.as_mut_ptr() as *mut tcg_efi_specid_event_head;

    n = memcmp(
        (*efispecid).signature.as_ptr() as *const core::ffi::c_void,
        TCG_SPECID_SIG.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&TCG_SPECID_SIG),
    );
    n == 0
}

unsafe extern "C" fn tpm_bios_log_free(data: *mut core::ffi::c_void) {
    kvfree(data);
}

pub unsafe fn tpm_read_log_acpi(chip: *mut tpm_chip) -> i32 {
    let mut buff: *mut AcpiTcpa;
    let mut status: acpi_status;
    let mut virt: *mut core::ffi::c_void;
    let mut len: u64;
    let mut start: u64;
    let log: *mut tpm_bios_log;
    let mut tbl: *mut acpi_table_tpm2;
    let mut tpm2_phy: *mut acpi_tpm2_phy;
    let mut format: i32;
    let mut ret: i32;

    log = &mut (*chip).log;

    if (*chip).acpi_dev_handle.is_null() {
        return -ENODEV;
    }

    if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 {
        status = acpi_get_table(b"TPM2\0".as_ptr() as *const i8, 1, &mut tbl as *mut _ as *mut *mut acpi_table_header);
        if status != 0 {
            return -ENODEV;
        }

        if (*tbl).header.length as usize < core::mem::size_of::<acpi_table_tpm2>() + core::mem::size_of::<acpi_tpm2_phy>() {
            acpi_put_table(tbl as *mut acpi_table_header);
            return -ENODEV;
        }

        tpm2_phy = (tbl as *mut u8).add(core::mem::size_of::<acpi_table_tpm2>()) as *mut acpi_tpm2_phy;
        len = (*tpm2_phy).log_area_minimum_length;
        start = (*tpm2_phy).log_area_start_address;
        if start == 0 || len == 0 {
            acpi_put_table(tbl as *mut acpi_table_header);
            return -ENODEV;
        }
        acpi_put_table(tbl as *mut acpi_table_header);
        format = EFI_TCG2_EVENT_LOG_FORMAT_TCG_2;
    } else {
        status = acpi_get_table(b"TCPA".as_ptr() as *const i8, 1, &mut buff as *mut _ as *mut *mut acpi_table_header);
        if status != 0 {
            return -ENODEV;
        }

        match (*buff).platform_class {
            BIOS_SERVER => {
                len = (*buff)._u.server.log_max_len;
                start = (*buff)._u.server.log_start_addr;
            }
            _ => {
                len = (*buff)._u.client.log_max_len as u64;
                start = (*buff)._u.client.log_start_addr;
            }
        }
        acpi_put_table(buff as *mut acpi_table_header);
        format = EFI_TCG2_EVENT_LOG_FORMAT_TCG_1_2;
    }

    if len == 0 {
        return -EIO;
    }

    (*log).bios_event_log = kvmalloc(len as usize, GFP_KERNEL) as *mut u8;
    if (*log).bios_event_log.is_null() {
        return -ENOMEM;
    }
    (*log).bios_event_log_end = (*log).bios_event_log.add(len as usize);

    virt = acpi_os_map_iomem(start, len as usize);
    if virt.is_null() {
        ret = -ENODEV;
        goto_err!(log, ret);
    }
    memcpy_fromio((*log).bios_event_log as *mut core::ffi::c_void, virt, len as usize);
    acpi_os_unmap_iomem(virt, len as usize);

    if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 && !tpm_is_tpm2_log((*log).bios_event_log as *mut core::ffi::c_void, len) {
        ret = -ENODEV;
        goto_err!(log, ret);
    }

    ret = devm_add_action(&mut (*chip).dev, tpm_bios_log_free, (*log).bios_event_log as *mut core::ffi::c_void);
    if ret != 0 {
        (*log).bios_event_log = core::ptr::null_mut();
        goto_err!(log, ret);
    }
    return format;
}

macro_rules! goto_err {
    ($log:expr, $ret:expr) => {{
        tpm_bios_log_free((*$log).bios_event_log as *mut core::ffi::c_void);
        (*$log).bios_event_log = core::ptr::null_mut();
        return $ret;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
