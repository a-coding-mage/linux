// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2005, 2012 IBM Corporation
 *
 * Authors:
 *	Kent Yoder <key@linux.vnet.ibm.com>
 *	Seiji Munetoh <munetoh@jp.ibm.com>
 *	Stefan Berger <stefanb@us.ibm.com>
 *	Reiner Sailer <sailer@watson.ibm.com>
 *	Kylene Hall <kjhall@us.ibm.com>
 *	Nayna Jain <nayna@linux.vnet.ibm.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * Access to the event log created by a system's firmware / BIOS
 */

// Dependencies supplied by the surrounding kernel translation.

static TCpa_EVENT_TYPE_STRINGS: [&'static [u8]; 18] = [
    b"PREBOOT\0", b"POST CODE\0", b"\0", b"NO ACTION\0", b"SEPARATOR\0",
    b"ACTION\0", b"EVENT TAG\0", b"S-CRTM Contents\0", b"S-CRTM Version\0",
    b"CPU Microcode\0", b"Platform Config Flags\0", b"Table of Devices\0",
    b"Compact Hash\0", b"IPL\0", b"IPL Partition Data\0", b"Non-Host Code\0",
    b"Non-Host Config\0", b"Non-Host Info\0",
];

static TCPa_PC_EVENT_ID_STRINGS: [&'static [u8]; 15] = [
    b"\0", b"SMBIOS\0", b"BIS Certificate\0", b"POST BIOS \0", b"ESCD \0",
    b"CMOS\0", b"NVRAM\0", b"Option ROM\0", b"Option ROM config\0", b"\0",
    b"Option ROM microcode \0", b"S-CRTM Version\0", b"S-CRTM Contents \0",
    b"POST Contents \0", b"Table of Devices\0",
];

/* returns pointer to start of pos. entry of tcg log */
unsafe fn tpm1_bios_measurements_start(m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let mut i: loff_t = 0;
    let chip: *mut tpm_chip = (*m).private as *mut tpm_chip;
    let log: *mut tpm_bios_log = &mut (*chip).log;
    let mut addr = (*log).bios_event_log;
    let limit = (*log).bios_event_log_end;
    loop {
        let event = addr as *mut tcpa_event;
        if (addr as usize + core::mem::size_of::<tcpa_event>()) > limit as usize { return core::ptr::null_mut(); }
        let converted_event_size = do_endian_conversion((*event).event_size);
        let converted_event_type = do_endian_conversion((*event).event_type);
        if ((converted_event_type == 0 && converted_event_size == 0)
            || addr as usize + core::mem::size_of::<tcpa_event>() + converted_event_size as usize > limit as usize) {
            return core::ptr::null_mut();
        }
        if i == *pos { return addr; }
        i += 1;
        addr = (addr as *mut u8).add(core::mem::size_of::<tcpa_event>() + converted_event_size as usize) as *mut core::ffi::c_void;
    }
}

unsafe fn tpm1_bios_measurements_next(m: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let event = v as *mut tcpa_event;
    let chip: *mut tpm_chip = (*m).private as *mut tpm_chip;
    let limit = (*(&mut (*chip).log)).bios_event_log_end;
    *pos += 1;
    let size = do_endian_conversion((*event).event_size) as usize;
    let v = (v as *mut u8).add(core::mem::size_of::<tcpa_event>() + size) as *mut core::ffi::c_void;
    if v as usize + core::mem::size_of::<tcpa_event>() > limit as usize { return core::ptr::null_mut(); }
    let event = v as *mut tcpa_event;
    let size = do_endian_conversion((*event).event_size);
    let typ = do_endian_conversion((*event).event_type);
    if (typ == 0 && size == 0) || v as usize + core::mem::size_of::<tcpa_event>() + size as usize > limit as usize { return core::ptr::null_mut(); }
    v
}

unsafe fn tpm1_bios_measurements_stop(_m: *mut seq_file, _v: *mut core::ffi::c_void) {}

unsafe fn get_event_name(dest: *mut i8, event: *mut tcpa_event, event_entry: *mut u8) -> i32 {
    let mut name: *const i8 = b"\0".as_ptr() as *const i8;
    let mut data = [0i8; 41];
    let mut n_len: i32 = 0;
    let mut d_len: i32 = 0;
    let typ = do_endian_conversion((*event).event_type);
    match typ {
        PREBOOT | POST_CODE | UNUSED | NO_ACTION | SCRTM_CONTENTS | SCRTM_VERSION | CPU_MICROCODE |
        PLATFORM_CONFIG_FLAGS | TABLE_OF_DEVICES | COMPACT_HASH | IPL | IPL_PARTITION_DATA | NONHOST_CODE |
        NONHOST_CONFIG | NONHOST_INFO => { name = TCpa_EVENT_TYPE_STRINGS[typ as usize].as_ptr() as *const i8; n_len = strlen(name); }
        SEPARATOR | ACTION => { let size = do_endian_conversion((*event).event_size); if MAX_TEXT_EVENT > size { name = event_entry as *const i8; n_len = size as i32; } }
        EVENT_TAG => {
            let pc_event = event_entry as *mut tcpa_pc_event;
            match do_endian_conversion((*pc_event).event_id) {
                SMBIOS | BIS_CERT | CMOS | NVRAM | OPTION_ROM_EXEC | OPTION_ROM_CONFIG | S_CRTM_VERSION => { name = TCPa_PC_EVENT_ID_STRINGS[do_endian_conversion((*pc_event).event_id) as usize].as_ptr() as *const i8; n_len = strlen(name); }
                POST_BIOS_ROM | ESCD | OPTION_ROM_MICROCODE | S_CRTM_CONTENTS | POST_CONTENTS => {
                    name = TCPa_PC_EVENT_ID_STRINGS[do_endian_conversion((*pc_event).event_id) as usize].as_ptr() as *const i8; n_len = strlen(name);
                    for i in 0..20 { d_len += sprintf(data.as_mut_ptr().add(2 * i), b"%02x\0".as_ptr() as *const i8, (*pc_event).event_data[i] as core::ffi::c_uint); }
                }
                _ => {}
            }
        }
        _ => {}
    }
    snprintf(dest, MAX_TEXT_EVENT, b"[%.*s%.*s]\0".as_ptr() as *const i8, n_len, name, d_len, data.as_ptr())
}

unsafe fn tpm1_binary_bios_measurements_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let event = v as *mut tcpa_event;
    let mut temp_event = core::ptr::read(event);
    temp_event.pcr_index = do_endian_conversion((*event).pcr_index);
    temp_event.event_type = do_endian_conversion((*event).event_type);
    temp_event.event_size = do_endian_conversion((*event).event_size);
    let temp_ptr = &temp_event as *const _ as *const i8;
    for i in 0..core::mem::size_of::<tcpa_event>() { seq_putc(m, *temp_ptr.add(i)); }
    let ptr = v as *const i8;
    for i in core::mem::size_of::<tcpa_event>()..core::mem::size_of::<tcpa_event>() + temp_event.event_size as usize { seq_putc(m, *ptr.add(i)); }
    0
}

unsafe fn tpm1_ascii_bios_measurements_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let event = v as *mut tcpa_event;
    let event_entry = (v as *mut u8).add(core::mem::size_of::<tcpa_event>());
    let eventname = kmalloc(MAX_TEXT_EVENT as usize, GFP_KERNEL) as *mut i8;
    if eventname.is_null() { return -ENOMEM; }
    seq_printf(m, b"%2d \0".as_ptr() as *const i8, do_endian_conversion((*event).pcr_index));
    seq_printf(m, b"%20phN\0".as_ptr() as *const i8, (*event).pcr_value.as_ptr());
    seq_printf(m, b" %02x\0".as_ptr() as *const i8, do_endian_conversion((*event).event_type));
    get_event_name(eventname, event, event_entry);
    seq_printf(m, b" %s\n\0".as_ptr() as *const i8, eventname);
    kfree(eventname as *mut core::ffi::c_void);
    0
}

#[no_mangle]
pub static tpm1_ascii_b_measurements_seqops: seq_operations = seq_operations { start: Some(tpm1_bios_measurements_start), next: Some(tpm1_bios_measurements_next), stop: Some(tpm1_bios_measurements_stop), show: Some(tpm1_ascii_bios_measurements_show) };

#[no_mangle]
pub static tpm1_binary_b_measurements_seqops: seq_operations = seq_operations { start: Some(tpm1_bios_measurements_start), next: Some(tpm1_bios_measurements_next), stop: Some(tpm1_bios_measurements_stop), show: Some(tpm1_binary_bios_measurements_show) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
