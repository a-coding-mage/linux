// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  fw_tables.c - Parsing support for ACPI and ACPI-like tables provided by
 *                platform or device firmware
 *
 *  Copyright (C) 2001 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
 *  Copyright (C) 2023 Intel Corp.
 */
// Dependencies supplied by the surrounding firmware-table implementation.

#[repr(C)]
#[derive(Copy, Clone)]
enum acpi_subtable_type {
    ACPI_SUBTABLE_COMMON,
    ACPI_SUBTABLE_HMAT,
    ACPI_SUBTABLE_PRMT,
    ACPI_SUBTABLE_CEDT,
    CDAT_SUBTABLE,
}

#[repr(C)]
struct acpi_subtable_entry {
    hdr: *mut acpi_subtable_headers,
    r#type: acpi_subtable_type,
}

unsafe fn acpi_get_entry_type(entry: *mut acpi_subtable_entry) -> c_ulong {
    match (*entry).r#type {
        acpi_subtable_type::ACPI_SUBTABLE_COMMON => (*(*entry).hdr).common.r#type as c_ulong,
        acpi_subtable_type::ACPI_SUBTABLE_HMAT => (*(*entry).hdr).hmat.r#type as c_ulong,
        acpi_subtable_type::ACPI_SUBTABLE_PRMT => 0,
        acpi_subtable_type::ACPI_SUBTABLE_CEDT => (*(*entry).hdr).cedt.r#type as c_ulong,
        acpi_subtable_type::CDAT_SUBTABLE => (*(*entry).hdr).cdat.r#type as c_ulong,
    }
}

unsafe fn acpi_get_entry_length(entry: *mut acpi_subtable_entry) -> c_ulong {
    match (*entry).r#type {
        acpi_subtable_type::ACPI_SUBTABLE_COMMON => (*(*entry).hdr).common.length as c_ulong,
        acpi_subtable_type::ACPI_SUBTABLE_HMAT => (*(*entry).hdr).hmat.length as c_ulong,
        acpi_subtable_type::ACPI_SUBTABLE_PRMT => (*(*entry).hdr).prmt.length as c_ulong,
        acpi_subtable_type::ACPI_SUBTABLE_CEDT => (*(*entry).hdr).cedt.length as c_ulong,
        acpi_subtable_type::CDAT_SUBTABLE => u16::from_le((*(*entry).hdr).cdat.length) as c_ulong,
    }
}

unsafe fn acpi_get_subtable_header_length(entry: *mut acpi_subtable_entry) -> c_ulong {
    match (*entry).r#type {
        acpi_subtable_type::ACPI_SUBTABLE_COMMON => core::mem::size_of_val(&(*(*entry).hdr).common) as c_ulong,
        acpi_subtable_type::ACPI_SUBTABLE_HMAT => core::mem::size_of_val(&(*(*entry).hdr).hmat) as c_ulong,
        acpi_subtable_type::ACPI_SUBTABLE_PRMT => core::mem::size_of_val(&(*(*entry).hdr).prmt) as c_ulong,
        acpi_subtable_type::ACPI_SUBTABLE_CEDT => core::mem::size_of_val(&(*(*entry).hdr).cedt) as c_ulong,
        acpi_subtable_type::CDAT_SUBTABLE => core::mem::size_of_val(&(*(*entry).hdr).cdat) as c_ulong,
    }
}

unsafe fn acpi_get_subtable_type(id: *mut c_char) -> acpi_subtable_type {
    if strncmp(id, ACPI_SIG_HMAT, 4) == 0 { return acpi_subtable_type::ACPI_SUBTABLE_HMAT; }
    if strncmp(id, ACPI_SIG_PRMT, 4) == 0 { return acpi_subtable_type::ACPI_SUBTABLE_PRMT; }
    if strncmp(id, ACPI_SIG_CEDT, 4) == 0 { return acpi_subtable_type::ACPI_SUBTABLE_CEDT; }
    if strncmp(id, ACPI_SIG_CDAT, 4) == 0 { return acpi_subtable_type::CDAT_SUBTABLE; }
    acpi_subtable_type::ACPI_SUBTABLE_COMMON
}

unsafe fn acpi_table_get_length(r#type: acpi_subtable_type, header: *mut fw_table_header) -> c_ulong {
    if let acpi_subtable_type::CDAT_SUBTABLE = r#type {
        return u32::from_le((*header).cdat.length) as c_ulong;
    }
    (*header).acpi.length as c_ulong
}

unsafe fn call_handler(proc: *mut acpi_subtable_proc, hdr: *mut acpi_subtable_headers, end: c_ulong) -> c_int {
    if let Some(handler) = (*proc).handler { return handler(hdr, end); }
    if let Some(handler_arg) = (*proc).handler_arg { return handler_arg(hdr, (*proc).arg, end); }
    -EINVAL
}

pub unsafe fn acpi_parse_entries_array(
    id: *mut c_char, table_size: c_ulong, table_header: *mut fw_table_header,
    max_length: c_ulong, proc: *mut acpi_subtable_proc, proc_num: c_int,
    max_entries: c_uint,
) -> c_int {
    let mut table_len = acpi_table_get_length(acpi_get_subtable_type(id), table_header);
    let mut entry = acpi_subtable_entry { hdr: core::ptr::null_mut(), r#type: acpi_get_subtable_type(id) };
    let mut count: c_int = 0;
    if max_length != 0 && max_length < table_len { table_len = max_length; }
    let table_end = table_header as c_ulong + table_len;
    entry.hdr = (table_header as c_ulong + table_size) as *mut acpi_subtable_headers;
    let subtable_len = acpi_get_subtable_header_length(&mut entry);

    while entry.hdr as c_ulong + subtable_len < table_end {
        for i in 0..proc_num {
            if acpi_get_entry_type(&mut entry) != (*proc.add(i as usize)).id as c_ulong { continue; }
            if max_entries == 0 || count < max_entries as c_int {
                if call_handler(proc.add(i as usize), entry.hdr, table_end) != 0 { return -EINVAL; }
            }
            (*proc.add(i as usize)).count += 1;
            count += 1;
            break;
        }
        let entry_len = acpi_get_entry_length(&mut entry);
        if entry_len == 0 {
            pr_err!("[%4.4s:0x%02x] Invalid zero length\n", id, (*proc).id);
            return -EINVAL;
        }
        entry.hdr = (entry.hdr as c_ulong + entry_len) as *mut acpi_subtable_headers;
    }
    if max_entries != 0 && count > max_entries as c_int {
        pr_warn!("[%4.4s:0x%02x] ignored %i entries of %i found\n", id, (*proc).id, count - max_entries as c_int, count);
    }
    count
}

pub unsafe fn cdat_table_parse(
    r#type: acpi_cdat_type, handler_arg: acpi_tbl_entry_handler_arg, arg: *mut c_void,
    table_header: *mut acpi_table_cdat, length: c_ulong,
) -> c_int {
    let mut proc = acpi_subtable_proc { id: r#type, handler_arg: Some(handler_arg), arg, ..core::mem::zeroed() };
    if table_header.is_null() { return -EINVAL; }
    acpi_parse_entries_array(ACPI_SIG_CDAT as *mut c_char, core::mem::size_of::<acpi_table_cdat>() as c_ulong,
        table_header as *mut fw_table_header, length, &mut proc, 1, 0)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
