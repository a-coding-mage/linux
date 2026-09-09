// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 IBM Corporation
 *
 * Authors:
 *      Nayna Jain <nayna@linux.vnet.ibm.com>
 *
 * Access to TPM 2.0 event log as written by Firmware.
 * It assumes that writer of event log has followed TCG Specification
 * for Family "2.0" and written the event data in little endian.
 * With that, it doesn't need any endian conversion for structure
 * content.
 */

// The types, constants, macros, and functions referenced below are supplied
// by the corresponding kernel TPM and seq_file interfaces.

extern "C" {
    fn __calc_tpm2_event_size(
        event: *mut tcg_pcr_event2_head,
        event_header: *mut tcg_pcr_event,
        check_event: bool,
    ) -> usize;
}

unsafe fn event_header_size(event_header: *mut tcg_pcr_event) -> usize {
    core::mem::offset_of!(tcg_pcr_event, event) + (*event_header).event_size as usize
}

/*
 * calc_tpm2_event_size() - calculate the event size, where event
 * is an entry in the TPM 2.0 event log. The event is of type Crypto
 * Agile Log Entry Format as defined in TCG EFI Protocol Specification
 * Family "2.0".
 *
 * @event: event whose size is to be calculated.
 * @event_header: the first event in the event log.
 *
 * Returns size of the event. If it is an invalid event, returns 0.
 */
unsafe fn calc_tpm2_event_size(
    event: *mut tcg_pcr_event2_head,
    event_header: *mut tcg_pcr_event,
) -> usize {
    __calc_tpm2_event_size(event, event_header, false)
}

unsafe fn tpm2_bios_measurements_start(m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let chip = (*m).private as *mut tpm_chip;
    let log = &mut (*chip).log;
    let mut addr = log.bios_event_log as *mut u8;
    let limit = log.bios_event_log_end as *mut u8;
    let event_header = addr as *mut tcg_pcr_event;
    let mut size = event_header_size(event_header);
    let mut event: *mut tcg_pcr_event2_head;
    let mut i: i64;

    if *pos == 0 {
        if addr.add(size) < limit {
            if (*event_header).event_type == 0 && (*event_header).event_size == 0 {
                return core::ptr::null_mut();
            }
            return SEQ_START_TOKEN as *mut core::ffi::c_void;
        }
    }

    if *pos > 0 {
        addr = addr.add(size);
        event = addr as *mut tcg_pcr_event2_head;
        size = calc_tpm2_event_size(event, event_header);
        if addr.add(size) >= limit || size == 0 {
            return core::ptr::null_mut();
        }
    }

    i = 0;
    while i < *pos - 1 {
        event = addr as *mut tcg_pcr_event2_head;
        size = calc_tpm2_event_size(event, event_header);
        if addr.add(size) >= limit || size == 0 {
            return core::ptr::null_mut();
        }
        addr = addr.add(size);
        i += 1;
    }

    addr as *mut core::ffi::c_void
}

unsafe fn tpm2_bios_measurements_next(
    m: *mut seq_file,
    v: *mut core::ffi::c_void,
    pos: *mut loff_t,
) -> *mut core::ffi::c_void {
    let chip = (*m).private as *mut tpm_chip;
    let log = &mut (*chip).log;
    let limit = log.bios_event_log_end as *mut u8;
    let event_header = log.bios_event_log as *mut tcg_pcr_event;
    let mut event_size: usize;
    let mut marker: *mut u8;

    *pos += 1;
    if v == SEQ_START_TOKEN as *mut core::ffi::c_void {
        event_size = event_header_size(event_header);
        marker = event_header as *mut u8;
    } else {
        let event = v as *mut tcg_pcr_event2_head;
        event_size = calc_tpm2_event_size(event, event_header);
        if event_size == 0 { return core::ptr::null_mut(); }
        marker = event as *mut u8;
    }

    marker = marker.add(event_size);
    if marker >= limit { return core::ptr::null_mut(); }
    let v = marker as *mut tcg_pcr_event2_head;
    event_size = calc_tpm2_event_size(v, event_header);
    if v.cast::<u8>().add(event_size) >= limit || event_size == 0 {
        return core::ptr::null_mut();
    }
    v as *mut core::ffi::c_void
}

unsafe fn tpm2_bios_measurements_stop(_m: *mut seq_file, _v: *mut core::ffi::c_void) {}

unsafe fn tpm2_binary_bios_measurements_show(
    m: *mut seq_file,
    v: *mut core::ffi::c_void,
) -> i32 {
    let chip = (*m).private as *mut tpm_chip;
    let log = &mut (*chip).log;
    let event_header = log.bios_event_log as *mut tcg_pcr_event;
    let mut size: usize;
    let temp_ptr: *mut core::ffi::c_void;

    if v == SEQ_START_TOKEN as *mut core::ffi::c_void {
        size = event_header_size(event_header);
        temp_ptr = event_header as *mut core::ffi::c_void;
    } else {
        size = calc_tpm2_event_size(v as *mut tcg_pcr_event2_head, event_header);
        temp_ptr = v;
    }
    if size > 0 { seq_write(m, temp_ptr, size); }
    0
}

pub static tpm2_binary_b_measurements_seqops: seq_operations = seq_operations {
    start: Some(tpm2_bios_measurements_start),
    next: Some(tpm2_bios_measurements_next),
    stop: Some(tpm2_bios_measurements_stop),
    show: Some(tpm2_binary_bios_measurements_show),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
