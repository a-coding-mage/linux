/*
 * Copyright 2018 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.
 */

// Declarations supplied by hdcp.h are intentionally external to this file.

pub unsafe fn mod_hdcp_hdcp2_transition(
    hdcp: *mut mod_hdcp,
    event_ctx: *mut mod_hdcp_event_context,
    input: *mut mod_hdcp_transition_input_hdcp2,
    output: *mut mod_hdcp_output,
) -> mod_hdcp_status {
    let mut status = MOD_HDCP_STATUS_SUCCESS;
    let conn = &mut (*hdcp).connection;
    let adjust = &mut (*hdcp).connection.link.adjust;

    match current_state(hdcp) {
        H2_A0_KNOWN_HDCP2_CAPABLE_RX => {
            if (*input).hdcp2version_read != PASS || (*input).hdcp2_capable_check != PASS { adjust.hdcp2.disable = 1; callback_in_ms(0, output); set_state_id(hdcp, output, HDCP_INITIALIZED); }
            else { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A1_SEND_AKE_INIT); }
        }
        H2_A1_SEND_AKE_INIT => {
            if (*input).create_session != PASS || (*input).ake_init_prepare != PASS { adjust.hdcp2.disable = 1; fail_and_restart_in_ms(0, &mut status, output); }
            else if (*input).ake_init_write != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else { set_watchdog_in_ms(hdcp, 100, output); callback_in_ms(0, output); set_state_id(hdcp, output, H2_A1_VALIDATE_AKE_CERT); }
        }
        H2_A1_VALIDATE_AKE_CERT => {
            if (*input).ake_cert_available != PASS {
                if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { fail_and_restart_in_ms(1000, &mut status, output); }
                else { callback_in_ms(10, output); increment_stay_counter(hdcp); }
            } else if (*input).ake_cert_read != PASS || (*input).ake_cert_validation != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else if conn.is_km_stored && !adjust.hdcp2.force_no_stored_km { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A1_SEND_STORED_KM); }
            else { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A1_SEND_NO_STORED_KM); }
        }
        H2_A1_SEND_NO_STORED_KM => {
            if (*input).no_stored_km_write != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else { if adjust.hdcp2.increase_h_prime_timeout { set_watchdog_in_ms(hdcp, 2000, output); } else { set_watchdog_in_ms(hdcp, 1000, output); } callback_in_ms(0, output); set_state_id(hdcp, output, H2_A1_READ_H_PRIME); }
        }
        H2_A1_READ_H_PRIME => {
            if (*input).h_prime_available != PASS { if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { fail_and_restart_in_ms(1000, &mut status, output); } else { callback_in_ms(100, output); increment_stay_counter(hdcp); } }
            else if (*input).h_prime_read != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else { set_watchdog_in_ms(hdcp, 200, output); callback_in_ms(0, output); set_state_id(hdcp, output, H2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME); }
        }
        H2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME => {
            if (*input).pairing_available != PASS { if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { fail_and_restart_in_ms(0, &mut status, output); } else { callback_in_ms(20, output); increment_stay_counter(hdcp); } }
            else if (*input).pairing_info_read != PASS || (*input).h_prime_validation != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A2_LOCALITY_CHECK); }
        }
        H2_A1_SEND_STORED_KM => {
            if (*input).stored_km_write != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else { set_watchdog_in_ms(hdcp, 200, output); callback_in_ms(0, output); set_state_id(hdcp, output, H2_A1_VALIDATE_H_PRIME); }
        }
        H2_A1_VALIDATE_H_PRIME => {
            if (*input).h_prime_available != PASS { if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { fail_and_restart_in_ms(1000, &mut status, output); } else { callback_in_ms(20, output); increment_stay_counter(hdcp); } }
            else if (*input).h_prime_read != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else if (*input).h_prime_validation != PASS { adjust.hdcp2.force_no_stored_km = 1; fail_and_restart_in_ms(0, &mut status, output); }
            else { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A2_LOCALITY_CHECK); }
        }
        H2_A2_LOCALITY_CHECK => {
            if (*hdcp).state.stay_count > 10 || (*input).lc_init_prepare != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else if adjust.hdcp2.use_fw_locality_check && (*input).l_prime_combo_read != PASS {
                if adjust.hdcp2.use_sw_locality_fallback { adjust.hdcp2.use_fw_locality_check = 0; callback_in_ms(0, output); increment_stay_counter(hdcp); }
                else { fail_and_restart_in_ms(0, &mut status, output); }
            } else if !adjust.hdcp2.use_fw_locality_check && ((*input).lc_init_write != PASS || (*input).l_prime_available_poll != PASS || (*input).l_prime_read != PASS) { fail_and_restart_in_ms(0, &mut status, output); }
            else if (*input).l_prime_validation != PASS { callback_in_ms(0, output); increment_stay_counter(hdcp); }
            else { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A3_EXCHANGE_KS_AND_TEST_FOR_REPEATER); }
        }
        H2_A3_EXCHANGE_KS_AND_TEST_FOR_REPEATER => {
            if (*input).eks_prepare != PASS || (*input).eks_write != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else if conn.is_repeater { set_watchdog_in_ms(hdcp, 3000, output); callback_in_ms(0, output); set_state_id(hdcp, output, H2_A6_WAIT_FOR_RX_ID_LIST); }
            else { callback_in_ms(210, output); set_state_id(hdcp, output, H2_ENABLE_ENCRYPTION); }
        }
        H2_ENABLE_ENCRYPTION => {
            if (*input).rxstatus_read != PASS || (*input).reauth_request_check != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else if (*event_ctx).rx_id_list_ready && conn.is_repeater { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK); }
            else if (*input).enable_encryption != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A5_AUTHENTICATED); set_auth_complete(hdcp, output); }
        }
        H2_A5_AUTHENTICATED => {
            if (*input).rxstatus_read == FAIL || (*input).reauth_request_check == FAIL { fail_and_restart_in_ms(0, &mut status, output); }
            else if (*event_ctx).rx_id_list_ready && conn.is_repeater { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK); }
            else { callback_in_ms(500, output); increment_stay_counter(hdcp); }
        }
        H2_A6_WAIT_FOR_RX_ID_LIST => {
            if (*input).rxstatus_read != PASS || (*input).reauth_request_check != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else if !(*event_ctx).rx_id_list_ready { if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { fail_and_restart_in_ms(100, &mut status, output); } else { callback_in_ms(300, output); increment_stay_counter(hdcp); } }
            else { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK); }
        }
        H2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK => {
            if (*input).rxstatus_read != PASS || (*input).reauth_request_check != PASS || (*input).rx_id_list_read != PASS || (*input).device_count_check != PASS || (*input).rx_id_list_validation != PASS || (*input).repeater_auth_ack_write != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A9_SEND_STREAM_MANAGEMENT); }
        }
        H2_A9_SEND_STREAM_MANAGEMENT => {
            if (*input).rxstatus_read != PASS || (*input).reauth_request_check != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else if (*event_ctx).rx_id_list_ready && conn.is_repeater { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK); }
            else if (*input).prepare_stream_manage != PASS || (*input).stream_manage_write != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else { set_watchdog_in_ms(hdcp, 100, output); callback_in_ms(0, output); set_state_id(hdcp, output, H2_A9_VALIDATE_STREAM_READY); }
        }
        H2_A9_VALIDATE_STREAM_READY => {
            if (*input).rxstatus_read != PASS || (*input).reauth_request_check != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else if (*event_ctx).rx_id_list_ready && conn.is_repeater { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK); }
            else if (*input).stream_ready_available != PASS { if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { (*hdcp).auth.count.stream_management_retry_count += 1; callback_in_ms(0, output); set_state_id(hdcp, output, H2_A9_SEND_STREAM_MANAGEMENT); } else { callback_in_ms(10, output); increment_stay_counter(hdcp); } }
            else if (*input).stream_ready_read != PASS || (*input).stream_ready_validation != PASS { if (*hdcp).auth.count.stream_management_retry_count > 10 { fail_and_restart_in_ms(0, &mut status, output); } else { (*hdcp).auth.count.stream_management_retry_count += 1; callback_in_ms(0, output); set_state_id(hdcp, output, H2_A9_SEND_STREAM_MANAGEMENT); } }
            else { callback_in_ms(200, output); set_state_id(hdcp, output, H2_ENABLE_ENCRYPTION); }
        }
        _ => { status = MOD_HDCP_STATUS_INVALID_STATE; fail_and_restart_in_ms(0, &mut status, output); }
    }
    status
}

// The DP transition has the same state-machine structure; DP-only checks and
// states are retained explicitly below.
pub unsafe fn mod_hdcp_hdcp2_dp_transition(hdcp: *mut mod_hdcp, event_ctx: *mut mod_hdcp_event_context, input: *mut mod_hdcp_transition_input_hdcp2, output: *mut mod_hdcp_output) -> mod_hdcp_status {
    let mut status = MOD_HDCP_STATUS_SUCCESS;
    let conn = &mut (*hdcp).connection;
    let adjust = &mut (*hdcp).connection.link.adjust;
    match current_state(hdcp) {
        D2_A0_DETERMINE_RX_HDCP_CAPABLE => { if (*input).rx_caps_read_dp != PASS || (*input).hdcp2_capable_check != PASS { adjust.hdcp2.disable=1; callback_in_ms(0,output); set_state_id(hdcp,output,HDCP_INITIALIZED); } else { callback_in_ms(0,output); set_state_id(hdcp,output,D2_A1_SEND_AKE_INIT); } }
        D2_A1_SEND_AKE_INIT => { if (*input).create_session != PASS || (*input).ake_init_prepare != PASS { adjust.hdcp2.disable=1; fail_and_restart_in_ms(0,&mut status,output); } else if (*input).ake_init_write != PASS { fail_and_restart_in_ms(0,&mut status,output); } else { callback_in_ms(100,output); set_state_id(hdcp,output,D2_A1_VALIDATE_AKE_CERT); } }
        D2_A1_VALIDATE_AKE_CERT => { if (*input).ake_cert_read != PASS || (*input).ake_cert_validation != PASS { fail_and_restart_in_ms(0,&mut status,output); } else if conn.is_km_stored && !adjust.hdcp2.force_no_stored_km { callback_in_ms(0,output); set_state_id(hdcp,output,D2_A1_SEND_STORED_KM); } else { callback_in_ms(0,output); set_state_id(hdcp,output,D2_A1_SEND_NO_STORED_KM); } }
        D2_A1_SEND_NO_STORED_KM => { if (*input).no_stored_km_write != PASS { fail_and_restart_in_ms(0,&mut status,output); } else { if adjust.hdcp2.increase_h_prime_timeout { set_watchdog_in_ms(hdcp,2000,output); } else { set_watchdog_in_ms(hdcp,1000,output); } set_state_id(hdcp,output,D2_A1_READ_H_PRIME); } }
        D2_A1_READ_H_PRIME => { if (*input).h_prime_available != PASS { if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { fail_and_restart_in_ms(1000,&mut status,output); } else { increment_stay_counter(hdcp); } } else if (*input).h_prime_read != PASS { fail_and_restart_in_ms(0,&mut status,output); } else { set_watchdog_in_ms(hdcp,200,output); set_state_id(hdcp,output,D2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME); } }
        D2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME => { if (*input).pairing_available != PASS { if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { fail_and_restart_in_ms(0,&mut status,output); } else { increment_stay_counter(hdcp); } } else if (*input).pairing_info_read != PASS || (*input).h_prime_validation != PASS { fail_and_restart_in_ms(0,&mut status,output); } else { callback_in_ms(0,output); set_state_id(hdcp,output,D2_A2_LOCALITY_CHECK); } }
        D2_A1_SEND_STORED_KM => { if (*input).stored_km_write != PASS { fail_and_restart_in_ms(0,&mut status,output); } else { set_watchdog_in_ms(hdcp,200,output); set_state_id(hdcp,output,D2_A1_VALIDATE_H_PRIME); } }
        D2_A1_VALIDATE_H_PRIME => { if (*input).h_prime_available != PASS { if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { fail_and_restart_in_ms(1000,&mut status,output); } else { increment_stay_counter(hdcp); } } else if (*input).h_prime_read != PASS { fail_and_restart_in_ms(0,&mut status,output); } else if (*input).h_prime_validation != PASS { adjust.hdcp2.force_no_stored_km=1; fail_and_restart_in_ms(0,&mut status,output); } else { callback_in_ms(0,output); set_state_id(hdcp,output,D2_A2_LOCALITY_CHECK); } }
        D2_A2_LOCALITY_CHECK => { if (*hdcp).state.stay_count > 10 || (*input).lc_init_prepare != PASS { fail_and_restart_in_ms(0,&mut status,output); } else if adjust.hdcp2.use_fw_locality_check && (*input).l_prime_combo_read != PASS { if adjust.hdcp2.use_sw_locality_fallback { adjust.hdcp2.use_fw_locality_check=0; callback_in_ms(0,output); increment_stay_counter(hdcp); } else { fail_and_restart_in_ms(0,&mut status,output); } } else if !adjust.hdcp2.use_fw_locality_check && ((*input).lc_init_write != PASS || (*input).l_prime_read != PASS) { fail_and_restart_in_ms(0,&mut status,output); } else if (*input).l_prime_validation != PASS { callback_in_ms(0,output); increment_stay_counter(hdcp); } else { callback_in_ms(0,output); set_state_id(hdcp,output,D2_A34_EXCHANGE_KS_AND_TEST_FOR_REPEATER); } }
        D2_A34_EXCHANGE_KS_AND_TEST_FOR_REPEATER => { if (*input).eks_prepare != PASS || (*input).eks_write != PASS { fail_and_restart_in_ms(0,&mut status,output); } else if conn.is_repeater { set_watchdog_in_ms(hdcp,3000,output); set_state_id(hdcp,output,D2_A6_WAIT_FOR_RX_ID_LIST); } else { callback_in_ms(1,output); set_state_id(hdcp,output,D2_SEND_CONTENT_STREAM_TYPE); } }
        D2_SEND_CONTENT_STREAM_TYPE => { if (*input).rxstatus_read != PASS || (*input).reauth_request_check != PASS || (*input).link_integrity_check_dp != PASS || (*input).content_stream_type_write != PASS { fail_and_restart_in_ms(0,&mut status,output); } else { callback_in_ms(210,output); set_state_id(hdcp,output,D2_ENABLE_ENCRYPTION); } }
        D2_ENABLE_ENCRYPTION => { if (*input).rxstatus_read != PASS || (*input).reauth_request_check != PASS || (*input).link_integrity_check_dp != PASS { fail_and_restart_in_ms(0,&mut status,output); } else if (*event_ctx).rx_id_list_ready && conn.is_repeater { callback_in_ms(0,output); set_state_id(hdcp,output,D2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK); } else if (*input).enable_encryption != PASS || (is_dp_mst_hdcp(hdcp) && (*input).stream_encryption_dp != PASS) { fail_and_restart_in_ms(0,&mut status,output); } else { set_state_id(hdcp,output,D2_A5_AUTHENTICATED); set_auth_complete(hdcp,output); } }
        D2_A5_AUTHENTICATED => { if (*input).rxstatus_read == FAIL || (*input).reauth_request_check == FAIL { fail_and_restart_in_ms(100,&mut status,output); } else if (*input).link_integrity_check_dp == FAIL { if (*hdcp).connection.hdcp2_retry_count >= 1 { adjust.hdcp2.force_type=MOD_HDCP_FORCE_TYPE_0; } fail_and_restart_in_ms(0,&mut status,output); } else if (*event_ctx).rx_id_list_ready && conn.is_repeater { callback_in_ms(0,output); set_state_id(hdcp,output,D2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK); } else { increment_stay_counter(hdcp); } }
        D2_A6_WAIT_FOR_RX_ID_LIST => { if (*input).rxstatus_read != PASS || (*input).reauth_request_check != PASS || (*input).link_integrity_check_dp != PASS { fail_and_restart_in_ms(0,&mut status,output); } else if !(*event_ctx).rx_id_list_ready { if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { fail_and_restart_in_ms(0,&mut status,output); } else { increment_stay_counter(hdcp); } } else { callback_in_ms(0,output); set_state_id(hdcp,output,D2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK); } }
        D2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK => { if (*input).rxstatus_read != PASS || (*input).reauth_request_check != PASS || (*input).link_integrity_check_dp != PASS || (*input).rx_id_list_read != PASS || (*input).device_count_check != PASS || (*input).rx_id_list_validation != PASS || (*input).repeater_auth_ack_write != PASS { fail_and_restart_in_ms(0,&mut status,output); } else { callback_in_ms(0,output); set_state_id(hdcp,output,D2_A9_SEND_STREAM_MANAGEMENT); } }
        D2_A9_SEND_STREAM_MANAGEMENT => { if (*input).rxstatus_read != PASS || (*input).reauth_request_check != PASS || (*input).link_integrity_check_dp != PASS { fail_and_restart_in_ms(0,&mut status,output); } else if (*event_ctx).rx_id_list_ready { callback_in_ms(0,output); set_state_id(hdcp,output,D2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK); } else if (*input).prepare_stream_manage != PASS || (*input).stream_manage_write != PASS { if (*event_ctx).event == MOD_HDCP_EVENT_CALLBACK { fail_and_restart_in_ms(0,&mut status,output); } else { increment_stay_counter(hdcp); } } else { callback_in_ms(100,output); set_state_id(hdcp,output,D2_A9_VALIDATE_STREAM_READY); } }
        D2_A9_VALIDATE_STREAM_READY => { if (*input).rxstatus_read != PASS || (*input).reauth_request_check != PASS || (*input).link_integrity_check_dp != PASS { fail_and_restart_in_ms(0,&mut status,output); } else if (*event_ctx).rx_id_list_ready { callback_in_ms(0,output); set_state_id(hdcp,output,D2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK); } else if (*input).stream_ready_read != PASS || (*input).stream_ready_validation != PASS { if (*hdcp).auth.count.stream_management_retry_count > 10 { fail_and_restart_in_ms(0,&mut status,output); } else if (*event_ctx).event == MOD_HDCP_EVENT_CALLBACK { (*hdcp).auth.count.stream_management_retry_count += 1; callback_in_ms(0,output); set_state_id(hdcp,output,D2_A9_SEND_STREAM_MANAGEMENT); } else { increment_stay_counter(hdcp); } } else { callback_in_ms(200,output); set_state_id(hdcp,output,D2_ENABLE_ENCRYPTION); } }
        _ => { status=MOD_HDCP_STATUS_INVALID_STATE; fail_and_restart_in_ms(0,&mut status,output); }
    }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
