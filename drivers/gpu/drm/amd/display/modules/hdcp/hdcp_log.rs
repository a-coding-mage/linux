/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

use core::ffi::c_char;

#[repr(C)]
pub struct mod_hdcp {
    _private: [u8; 0],
}

extern "C" {
    fn is_hdcp1(hdcp: *mut mod_hdcp) -> bool;
    fn is_hdcp2(hdcp: *mut mod_hdcp) -> bool;
}

pub unsafe fn mod_hdcp_dump_binary_message(
    msg: *mut u8,
    msg_size: u32,
    buf: *mut u8,
    buf_size: u32,
) {
    let bytes_per_line: u8 = 16;
    let byte_size: u32 = 3;
    let newline_size: u32 = 1;
    let terminator_size: u32 = 1;
    let mut buf_pos: u32 = 0;
    let mut i: u32 = 0;

    if buf_size < terminator_size {
        return;
    }

    while i < msg_size {
        let mut needed = byte_size + terminator_size;
        if i % bytes_per_line as u32 == 0 {
            needed += newline_size;
        }
        if buf_pos + needed > buf_size {
            break;
        }
        if i % bytes_per_line as u32 == 0 {
            *buf.add(buf_pos as usize) = b'\n';
            buf_pos += 1;
        }
        // Equivalent to sprintf((char *)&buf[buf_pos], "%02X ", msg[i]).
        let value = *msg.add(i as usize);
        *buf.add(buf_pos as usize) = b"0123456789ABCDEF"[(value >> 4) as usize];
        *buf.add((buf_pos + 1) as usize) = b"0123456789ABCDEF"[(value & 0x0f) as usize];
        *buf.add((buf_pos + 2) as usize) = b' ';
        buf_pos += byte_size;
        i += 1;
    }
    *buf.add(buf_pos as usize) = b'\0';
}

pub unsafe fn mod_hdcp_log_ddc_trace(hdcp: *mut mod_hdcp) {
    if is_hdcp1(hdcp) {
        HDCP_DDC_READ_TRACE!(hdcp, "BKSV", (*hdcp).auth.msg.hdcp1.bksv, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp1.bksv));
        HDCP_DDC_READ_TRACE!(hdcp, "BCAPS", &(*hdcp).auth.msg.hdcp1.bcaps, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp1.bcaps));
        HDCP_DDC_READ_TRACE!(hdcp, "BSTATUS", &(*hdcp).auth.msg.hdcp1.bstatus as *const _ as *mut u8, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp1.bstatus));
        HDCP_DDC_WRITE_TRACE!(hdcp, "AN", (*hdcp).auth.msg.hdcp1.an, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp1.an));
        HDCP_DDC_WRITE_TRACE!(hdcp, "AKSV", (*hdcp).auth.msg.hdcp1.aksv, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp1.aksv));
        HDCP_DDC_WRITE_TRACE!(hdcp, "AINFO", &(*hdcp).auth.msg.hdcp1.ainfo, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp1.ainfo));
        HDCP_DDC_READ_TRACE!(hdcp, "RI' / R0'", &(*hdcp).auth.msg.hdcp1.r0p as *const _ as *mut u8, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp1.r0p));
        HDCP_DDC_READ_TRACE!(hdcp, "BINFO", &(*hdcp).auth.msg.hdcp1.binfo_dp as *const _ as *mut u8, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp1.binfo_dp));
        HDCP_DDC_READ_TRACE!(hdcp, "KSVLIST", (*hdcp).auth.msg.hdcp1.ksvlist, (*hdcp).auth.msg.hdcp1.ksvlist_size);
        HDCP_DDC_READ_TRACE!(hdcp, "V'", (*hdcp).auth.msg.hdcp1.vp, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp1.vp));
    } else if is_hdcp2(hdcp) {
        HDCP_DDC_READ_TRACE!(hdcp, "HDCP2Version", &(*hdcp).auth.msg.hdcp2.hdcp2version_hdmi, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.hdcp2version_hdmi));
        HDCP_DDC_READ_TRACE!(hdcp, "Rx Caps", (*hdcp).auth.msg.hdcp2.rxcaps_dp, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.rxcaps_dp));
        HDCP_DDC_WRITE_TRACE!(hdcp, "AKE Init", (*hdcp).auth.msg.hdcp2.ake_init, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.ake_init));
        HDCP_DDC_READ_TRACE!(hdcp, "AKE Cert", (*hdcp).auth.msg.hdcp2.ake_cert, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.ake_cert));
        HDCP_DDC_WRITE_TRACE!(hdcp, "Stored KM", (*hdcp).auth.msg.hdcp2.ake_stored_km, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.ake_stored_km));
        HDCP_DDC_WRITE_TRACE!(hdcp, "No Stored KM", (*hdcp).auth.msg.hdcp2.ake_no_stored_km, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.ake_no_stored_km));
        HDCP_DDC_READ_TRACE!(hdcp, "H'", (*hdcp).auth.msg.hdcp2.ake_h_prime, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.ake_h_prime));
        HDCP_DDC_READ_TRACE!(hdcp, "Pairing Info", (*hdcp).auth.msg.hdcp2.ake_pairing_info, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.ake_pairing_info));
        HDCP_DDC_WRITE_TRACE!(hdcp, "LC Init", (*hdcp).auth.msg.hdcp2.lc_init, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.lc_init));
        HDCP_DDC_READ_TRACE!(hdcp, "L'", (*hdcp).auth.msg.hdcp2.lc_l_prime, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.lc_l_prime));
        HDCP_DDC_WRITE_TRACE!(hdcp, "Exchange KS", (*hdcp).auth.msg.hdcp2.ske_eks, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.ske_eks));
        HDCP_DDC_READ_TRACE!(hdcp, "Rx Status", &(*hdcp).auth.msg.hdcp2.rxstatus as *const _ as *mut u8, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.rxstatus));
        HDCP_DDC_READ_TRACE!(hdcp, "Rx Id List", (*hdcp).auth.msg.hdcp2.rx_id_list, (*hdcp).auth.msg.hdcp2.rx_id_list_size);
        HDCP_DDC_WRITE_TRACE!(hdcp, "Rx Id List Ack", (*hdcp).auth.msg.hdcp2.repeater_auth_ack, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.repeater_auth_ack));
        HDCP_DDC_WRITE_TRACE!(hdcp, "Content Stream Management", (*hdcp).auth.msg.hdcp2.repeater_auth_stream_manage, (*hdcp).auth.msg.hdcp2.stream_manage_size);
        HDCP_DDC_READ_TRACE!(hdcp, "Stream Ready", (*hdcp).auth.msg.hdcp2.repeater_auth_stream_ready, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.repeater_auth_stream_ready));
        HDCP_DDC_WRITE_TRACE!(hdcp, "Content Stream Type", (*hdcp).auth.msg.hdcp2.content_stream_type_dp, core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.content_stream_type_dp));
    }
}

pub fn mod_hdcp_status_to_str(status: i32) -> *const c_char {
    // MOD_HDCP_STATUS_LIST(CASE_FORMAT) is supplied by the translated dependency.
    unsafe { MOD_HDCP_STATUS_TO_STR!(status) }
}

pub fn mod_hdcp_state_id_to_str(id: i32) -> *const c_char {
    match id {
        HDCP_UNINITIALIZED => b"HDCP_UNINITIALIZED\0".as_ptr() as *const c_char,
        HDCP_INITIALIZED => b"HDCP_INITIALIZED\0".as_ptr() as *const c_char,
        HDCP_CP_NOT_DESIRED => b"HDCP_CP_NOT_DESIRED\0".as_ptr() as *const c_char,
        H1_A0_WAIT_FOR_ACTIVE_RX => b"H1_A0_WAIT_FOR_ACTIVE_RX\0".as_ptr() as *const c_char,
        H1_A1_EXCHANGE_KSVS => b"H1_A1_EXCHANGE_KSVS\0".as_ptr() as *const c_char,
        H1_A2_COMPUTATIONS_A3_VALIDATE_RX_A6_TEST_FOR_REPEATER => b"H1_A2_COMPUTATIONS_A3_VALIDATE_RX_A6_TEST_FOR_REPEATER\0".as_ptr() as *const c_char,
        H1_A45_AUTHENTICATED => b"H1_A45_AUTHENTICATED\0".as_ptr() as *const c_char,
        H1_A8_WAIT_FOR_READY => b"H1_A8_WAIT_FOR_READY\0".as_ptr() as *const c_char,
        H1_A9_READ_KSV_LIST => b"H1_A9_READ_KSV_LIST\0".as_ptr() as *const c_char,
        D1_A0_DETERMINE_RX_HDCP_CAPABLE => b"D1_A0_DETERMINE_RX_HDCP_CAPABLE\0".as_ptr() as *const c_char,
        D1_A1_EXCHANGE_KSVS => b"D1_A1_EXCHANGE_KSVS\0".as_ptr() as *const c_char,
        D1_A23_WAIT_FOR_R0_PRIME => b"D1_A23_WAIT_FOR_R0_PRIME\0".as_ptr() as *const c_char,
        D1_A2_COMPUTATIONS_A3_VALIDATE_RX_A5_TEST_FOR_REPEATER => b"D1_A2_COMPUTATIONS_A3_VALIDATE_RX_A5_TEST_FOR_REPEATER\0".as_ptr() as *const c_char,
        D1_A4_AUTHENTICATED => b"D1_A4_AUTHENTICATED\0".as_ptr() as *const c_char,
        D1_A6_WAIT_FOR_READY => b"D1_A6_WAIT_FOR_READY\0".as_ptr() as *const c_char,
        D1_A7_READ_KSV_LIST => b"D1_A7_READ_KSV_LIST\0".as_ptr() as *const c_char,
        H2_A0_KNOWN_HDCP2_CAPABLE_RX => b"H2_A0_KNOWN_HDCP2_CAPABLE_RX\0".as_ptr() as *const c_char,
        H2_A1_SEND_AKE_INIT => b"H2_A1_SEND_AKE_INIT\0".as_ptr() as *const c_char,
        H2_A1_VALIDATE_AKE_CERT => b"H2_A1_VALIDATE_AKE_CERT\0".as_ptr() as *const c_char,
        H2_A1_SEND_NO_STORED_KM => b"H2_A1_SEND_NO_STORED_KM\0".as_ptr() as *const c_char,
        H2_A1_READ_H_PRIME => b"H2_A1_READ_H_PRIME\0".as_ptr() as *const c_char,
        H2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME => b"H2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME\0".as_ptr() as *const c_char,
        H2_A1_SEND_STORED_KM => b"H2_A1_SEND_STORED_KM\0".as_ptr() as *const c_char,
        H2_A1_VALIDATE_H_PRIME => b"H2_A1_VALIDATE_H_PRIME\0".as_ptr() as *const c_char,
        H2_A2_LOCALITY_CHECK => b"H2_A2_LOCALITY_CHECK\0".as_ptr() as *const c_char,
        H2_A3_EXCHANGE_KS_AND_TEST_FOR_REPEATER => b"H2_A3_EXCHANGE_KS_AND_TEST_FOR_REPEATER\0".as_ptr() as *const c_char,
        H2_ENABLE_ENCRYPTION => b"H2_ENABLE_ENCRYPTION\0".as_ptr() as *const c_char,
        H2_A5_AUTHENTICATED => b"H2_A5_AUTHENTICATED\0".as_ptr() as *const c_char,
        H2_A6_WAIT_FOR_RX_ID_LIST => b"H2_A6_WAIT_FOR_RX_ID_LIST\0".as_ptr() as *const c_char,
        H2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK => b"H2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK\0".as_ptr() as *const c_char,
        H2_A9_SEND_STREAM_MANAGEMENT => b"H2_A9_SEND_STREAM_MANAGEMENT\0".as_ptr() as *const c_char,
        H2_A9_VALIDATE_STREAM_READY => b"H2_A9_VALIDATE_STREAM_READY\0".as_ptr() as *const c_char,
        D2_A0_DETERMINE_RX_HDCP_CAPABLE => b"D2_A0_DETERMINE_RX_HDCP_CAPABLE\0".as_ptr() as *const c_char,
        D2_A1_SEND_AKE_INIT => b"D2_A1_SEND_AKE_INIT\0".as_ptr() as *const c_char,
        D2_A1_VALIDATE_AKE_CERT => b"D2_A1_VALIDATE_AKE_CERT\0".as_ptr() as *const c_char,
        D2_A1_SEND_NO_STORED_KM => b"D2_A1_SEND_NO_STORED_KM\0".as_ptr() as *const c_char,
        D2_A1_READ_H_PRIME => b"D2_A1_READ_H_PRIME\0".as_ptr() as *const c_char,
        D2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME => b"D2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME\0".as_ptr() as *const c_char,
        D2_A1_SEND_STORED_KM => b"D2_A1_SEND_STORED_KM\0".as_ptr() as *const c_char,
        D2_A1_VALIDATE_H_PRIME => b"D2_A1_VALIDATE_H_PRIME\0".as_ptr() as *const c_char,
        D2_A2_LOCALITY_CHECK => b"D2_A2_LOCALITY_CHECK\0".as_ptr() as *const c_char,
        D2_A34_EXCHANGE_KS_AND_TEST_FOR_REPEATER => b"D2_A34_EXCHANGE_KS_AND_TEST_FOR_REPEATER\0".as_ptr() as *const c_char,
        D2_SEND_CONTENT_STREAM_TYPE => b"D2_SEND_CONTENT_STREAM_TYPE\0".as_ptr() as *const c_char,
        D2_ENABLE_ENCRYPTION => b"D2_ENABLE_ENCRYPTION\0".as_ptr() as *const c_char,
        D2_A5_AUTHENTICATED => b"D2_A5_AUTHENTICATED\0".as_ptr() as *const c_char,
        D2_A6_WAIT_FOR_RX_ID_LIST => b"D2_A6_WAIT_FOR_RX_ID_LIST\0".as_ptr() as *const c_char,
        D2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK => b"D2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK\0".as_ptr() as *const c_char,
        D2_A9_SEND_STREAM_MANAGEMENT => b"D2_A9_SEND_STREAM_MANAGEMENT\0".as_ptr() as *const c_char,
        D2_A9_VALIDATE_STREAM_READY => b"D2_A9_VALIDATE_STREAM_READY\0".as_ptr() as *const c_char,
        _ => b"UNKNOWN_STATE_ID\0".as_ptr() as *const c_char,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
