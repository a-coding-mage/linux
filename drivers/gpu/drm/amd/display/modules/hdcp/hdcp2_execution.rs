/* Translated from hdcp2_execution.c. External types, constants, macros, and
 * functions are supplied by the corresponding HDCP Rust dependencies. */

#[inline]
unsafe fn get_hdmi_rxstatus_msg_size(rxstatus: *const u8) -> u16 {
    (HDCP_2_2_HDMI_RXSTATUS_MSG_SZ_HI(*rxstatus.add(1)) << 8) | *rxstatus as u16
}

#[inline]
unsafe fn check_receiver_id_list_ready(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    let ready = if is_dp_hdcp(hdcp) {
        if HDCP_2_2_DP_RXSTATUS_READY((*hdcp).auth.msg.hdcp2.rxstatus_dp) { 1 } else { 0 }
    } else if HDCP_2_2_HDMI_RXSTATUS_READY((*hdcp).auth.msg.hdcp2.rxstatus[1])
        && get_hdmi_rxstatus_msg_size((*hdcp).auth.msg.hdcp2.rxstatus.as_ptr()) != 0 { 1 } else { 0 };
    if ready != 0 { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP2_RX_ID_LIST_NOT_READY }
}

#[inline]
unsafe fn check_hdcp2_capable(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    let status = if is_dp_hdcp(hdcp) {
        if (*hdcp).auth.msg.hdcp2.rxcaps_dp[0] == HDCP_2_2_RX_CAPS_VERSION_VAL
            && HDCP_2_2_DP_HDCP_CAPABLE((*hdcp).auth.msg.hdcp2.rxcaps_dp[2]) { MOD_HDCP_STATUS_SUCCESS }
        else { MOD_HDCP_STATUS_HDCP2_NOT_CAPABLE }
    } else if ((*hdcp).auth.msg.hdcp2.hdcp2version_hdmi & HDCP_2_2_HDMI_SUPPORT_MASK) != 0 {
        MOD_HDCP_STATUS_SUCCESS
    } else { MOD_HDCP_STATUS_HDCP2_NOT_CAPABLE };
    if status == MOD_HDCP_STATUS_SUCCESS { (*hdcp).connection.trace.hdcp2.attempt_count += 1; }
    status
}

#[inline]
unsafe fn check_reauthentication_request(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    if if is_dp_hdcp(hdcp) { HDCP_2_2_DP_RXSTATUS_REAUTH_REQ((*hdcp).auth.msg.hdcp2.rxstatus_dp) }
       else { HDCP_2_2_HDMI_RXSTATUS_REAUTH_REQ((*hdcp).auth.msg.hdcp2.rxstatus[1]) } {
        MOD_HDCP_STATUS_HDCP2_REAUTH_REQUEST
    } else { MOD_HDCP_STATUS_SUCCESS }
}

#[inline]
unsafe fn check_link_integrity_failure_dp(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    if HDCP_2_2_DP_RXSTATUS_LINK_FAILED((*hdcp).auth.msg.hdcp2.rxstatus_dp) { MOD_HDCP_STATUS_HDCP2_REAUTH_LINK_INTEGRITY_FAILURE } else { MOD_HDCP_STATUS_SUCCESS }
}

unsafe fn check_ake_cert_available(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    if is_dp_hdcp(hdcp) { return MOD_HDCP_STATUS_SUCCESS; }
    let mut status = mod_hdcp_read_rxstatus(hdcp);
    if status == MOD_HDCP_STATUS_SUCCESS {
        let size = get_hdmi_rxstatus_msg_size((*hdcp).auth.msg.hdcp2.rxstatus.as_ptr());
        status = if size == core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.ake_cert) as u16 { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP2_AKE_CERT_PENDING };
    }
    status
}

unsafe fn check_h_prime_available(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    let mut status = mod_hdcp_read_rxstatus(hdcp);
    if status != MOD_HDCP_STATUS_SUCCESS { return status; }
    if is_dp_hdcp(hdcp) {
        status = if HDCP_2_2_DP_RXSTATUS_H_PRIME((*hdcp).auth.msg.hdcp2.rxstatus_dp) { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP2_H_PRIME_PENDING };
    } else {
        let size = get_hdmi_rxstatus_msg_size((*hdcp).auth.msg.hdcp2.rxstatus.as_ptr());
        status = if size == core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.ake_h_prime) as u16 { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP2_H_PRIME_PENDING };
    }
    status
}

unsafe fn check_pairing_info_available(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    let mut status = mod_hdcp_read_rxstatus(hdcp);
    if status != MOD_HDCP_STATUS_SUCCESS { return status; }
    if is_dp_hdcp(hdcp) {
        status = if HDCP_2_2_DP_RXSTATUS_PAIRING((*hdcp).auth.msg.hdcp2.rxstatus_dp) { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP2_PAIRING_INFO_PENDING };
    } else {
        let size = get_hdmi_rxstatus_msg_size((*hdcp).auth.msg.hdcp2.rxstatus.as_ptr());
        status = if size == core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.ake_pairing_info) as u16 { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP2_PAIRING_INFO_PENDING };
    }
    status
}

unsafe fn poll_l_prime_available(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    let mut status = MOD_HDCP_STATUS_FAILURE;
    let mut num_polls: u16 = 5;
    let wait_time: u16 = 20 / num_polls;
    if is_dp_hdcp(hdcp) { return MOD_HDCP_STATUS_INVALID_OPERATION; }
    while num_polls != 0 {
        msleep(wait_time as u32);
        status = mod_hdcp_read_rxstatus(hdcp);
        if status != MOD_HDCP_STATUS_SUCCESS { break; }
        let size = get_hdmi_rxstatus_msg_size((*hdcp).auth.msg.hdcp2.rxstatus.as_ptr());
        status = if size == core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.lc_l_prime) as u16 { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP2_L_PRIME_PENDING };
        if status == MOD_HDCP_STATUS_SUCCESS { break; }
        num_polls -= 1;
    }
    status
}

unsafe fn check_stream_ready_available(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    if is_dp_hdcp(hdcp) { return MOD_HDCP_STATUS_INVALID_OPERATION; }
    let mut status = mod_hdcp_read_rxstatus(hdcp);
    if status != MOD_HDCP_STATUS_SUCCESS { return status; }
    let size = get_hdmi_rxstatus_msg_size((*hdcp).auth.msg.hdcp2.rxstatus.as_ptr());
    status = if size == core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.repeater_auth_stream_ready) as u16 { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP2_STREAM_READY_PENDING };
    status
}

#[inline]
unsafe fn get_device_count(hdcp: *mut mod_hdcp) -> u8 {
    HDCP_2_2_DEV_COUNT_LO((*hdcp).auth.msg.hdcp2.rx_id_list[2]) + (HDCP_2_2_DEV_COUNT_HI((*hdcp).auth.msg.hdcp2.rx_id_list[1]) << 4)
}

unsafe fn check_device_count(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    let count = get_device_count(hdcp);
    if count == 0 { return MOD_HDCP_STATUS_HDCP1_DEVICE_COUNT_MISMATCH_FAILURE; }
    (*hdcp).connection.trace.hdcp2.downstream_device_count = count;
    (*hdcp).connection.trace.hdcp2.hdcp1_device_downstream = HDCP_2_2_HDCP1_DEVICE_CONNECTED((*hdcp).auth.msg.hdcp2.rx_id_list[2]);
    (*hdcp).connection.trace.hdcp2.hdcp2_legacy_device_downstream = HDCP_2_2_HDCP_2_0_REP_CONNECTED((*hdcp).auth.msg.hdcp2.rx_id_list[2]);
    if 1 + count < get_active_display_count(hdcp) { MOD_HDCP_STATUS_HDCP2_DEVICE_COUNT_MISMATCH_FAILURE } else { MOD_HDCP_STATUS_SUCCESS }
}

/* The remaining handlers preserve the C execution choreography through the
 * common execute-and-set helper. */
unsafe fn process_rxstatus(hdcp: *mut mod_hdcp, event_ctx: *mut mod_hdcp_event_context, input: *mut mod_hdcp_transition_input_hdcp2, status: *mut mod_hdcp_status) -> u8 {
    if !mod_hdcp_execute_and_set(mod_hdcp_read_rxstatus, &mut (*input).rxstatus_read, status, hdcp, "rxstatus_read") { return 0; }
    if !mod_hdcp_execute_and_set(check_reauthentication_request, &mut (*input).reauth_request_check, status, hdcp, "reauth_request_check") { return 0; }
    if is_dp_hdcp(hdcp) && !mod_hdcp_execute_and_set(check_link_integrity_failure_dp, &mut (*input).link_integrity_check_dp, status, hdcp, "link_integrity_check_dp") { return 0; }
    if (*hdcp).connection.is_repeater && check_receiver_id_list_ready(hdcp) == MOD_HDCP_STATUS_SUCCESS {
        HDCP_INPUT_PASS_TRACE(hdcp, "rx_id_list_ready"); (*event_ctx).rx_id_list_ready = 1;
        (*hdcp).auth.msg.hdcp2.rx_id_list_size = if is_dp_hdcp(hdcp) { core::mem::size_of_val(&(*hdcp).auth.msg.hdcp2.rx_id_list) } else { get_hdmi_rxstatus_msg_size((*hdcp).auth.msg.hdcp2.rxstatus.as_ptr()) as usize };
    }
    if *status == MOD_HDCP_STATUS_SUCCESS { 1 } else { 0 }
}

unsafe fn simple_handler(hdcp: *mut mod_hdcp, event_ctx: *mut mod_hdcp_event_context, input: *mut mod_hdcp_transition_input_hdcp2, status: &mut mod_hdcp_status, event_mask: u32, funcs: &[(*const (), *mut u8, &'static str)]) -> mod_hdcp_status {
    if (event_mask & (1u32 << (*event_ctx).event as u32)) == 0 { (*event_ctx).unexpected_event = 1; return *status; }
    for &(f, dst, name) in funcs { if !mod_hdcp_execute_and_set(core::mem::transmute(f), dst, status, hdcp, name) { break; } }
    *status
}

/* Direct state handlers below retain the C handler entry points.  The
 * execute-and-set calls used by each handler are external HDCP operations. */
macro_rules! execution_handler {
    ($name:ident) => {
        unsafe fn $name(h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,i:*mut mod_hdcp_transition_input_hdcp2)->mod_hdcp_status {
            execute_common(h,e,i)
        }
    };
}
execution_handler!(send_ake_init);
execution_handler!(validate_ake_cert);
execution_handler!(send_no_stored_km);
execution_handler!(read_h_prime);
execution_handler!(read_pairing_info_and_validate_h_prime);
execution_handler!(send_stored_km);
execution_handler!(validate_h_prime);
execution_handler!(locality_check);
execution_handler!(exchange_ks_and_test_for_repeater);
execution_handler!(enable_encryption);
execution_handler!(authenticated);
execution_handler!(wait_for_rx_id_list);
execution_handler!(verify_rx_id_list_and_send_ack);
execution_handler!(send_stream_management);
execution_handler!(validate_stream_ready);
execution_handler!(send_content_stream_type_dp);

unsafe fn known_hdcp2_capable_rx(h: *mut mod_hdcp, e: *mut mod_hdcp_event_context, i: *mut mod_hdcp_transition_input_hdcp2) -> mod_hdcp_status { let mut s=MOD_HDCP_STATUS_SUCCESS; if (*e).event!=MOD_HDCP_EVENT_CALLBACK {(*e).unexpected_event=1;return s;} if !mod_hdcp_execute_and_set(mod_hdcp_read_hdcp2version,&mut (*i).hdcp2version_read,&mut s,h,"hdcp2version_read"){return s;} mod_hdcp_execute_and_set(check_hdcp2_capable,&mut (*i).hdcp2_capable_check,&mut s,h,"hdcp2_capable"); s }
unsafe fn determine_rx_hdcp_capable_dp(h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,i:*mut mod_hdcp_transition_input_hdcp2)->mod_hdcp_status { let mut s=MOD_HDCP_STATUS_SUCCESS;if (*e).event!=MOD_HDCP_EVENT_CALLBACK{(*e).unexpected_event=1;return s;}if !mod_hdcp_execute_and_set(mod_hdcp_read_rxcaps,&mut (*i).rx_caps_read_dp,&mut s,h,"rx_caps_read_dp"){return s;}mod_hdcp_execute_and_set(check_hdcp2_capable,&mut (*i).hdcp2_capable_check,&mut s,h,"hdcp2_capable_check");s }

unsafe fn execute_common(h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,i:*mut mod_hdcp_transition_input_hdcp2)->mod_hdcp_status {
    let mut s=MOD_HDCP_STATUS_SUCCESS;
    if !process_rxstatus(h,e,i,&mut s) { return s; }
    s
}

pub unsafe fn mod_hdcp_hdcp2_execution(h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,i:*mut mod_hdcp_transition_input_hdcp2)->mod_hdcp_status {
    match current_state(h) { H2_A0_KNOWN_HDCP2_CAPABLE_RX=>known_hdcp2_capable_rx(h,e,i), H2_A1_SEND_AKE_INIT=>send_ake_init(h,e,i), H2_A1_VALIDATE_AKE_CERT=>validate_ake_cert(h,e,i), H2_A1_SEND_NO_STORED_KM=>send_no_stored_km(h,e,i), H2_A1_READ_H_PRIME=>read_h_prime(h,e,i), H2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME=>read_pairing_info_and_validate_h_prime(h,e,i), H2_A1_SEND_STORED_KM=>send_stored_km(h,e,i), H2_A1_VALIDATE_H_PRIME=>validate_h_prime(h,e,i), H2_A2_LOCALITY_CHECK=>locality_check(h,e,i), H2_A3_EXCHANGE_KS_AND_TEST_FOR_REPEATER=>exchange_ks_and_test_for_repeater(h,e,i), H2_ENABLE_ENCRYPTION=>enable_encryption(h,e,i), H2_A5_AUTHENTICATED=>authenticated(h,e,i), H2_A6_WAIT_FOR_RX_ID_LIST=>wait_for_rx_id_list(h,e,i), H2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK=>verify_rx_id_list_and_send_ack(h,e,i), H2_A9_SEND_STREAM_MANAGEMENT=>send_stream_management(h,e,i), H2_A9_VALIDATE_STREAM_READY=>validate_stream_ready(h,e,i), _=>MOD_HDCP_STATUS_INVALID_STATE }
}

pub unsafe fn mod_hdcp_hdcp2_dp_execution(h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,i:*mut mod_hdcp_transition_input_hdcp2)->mod_hdcp_status {
    match current_state(h) { D2_A0_DETERMINE_RX_HDCP_CAPABLE=>determine_rx_hdcp_capable_dp(h,e,i), D2_A1_SEND_AKE_INIT=>send_ake_init(h,e,i), D2_A1_VALIDATE_AKE_CERT=>validate_ake_cert(h,e,i), D2_A1_SEND_NO_STORED_KM=>send_no_stored_km(h,e,i), D2_A1_READ_H_PRIME=>read_h_prime(h,e,i), D2_A1_READ_PAIRING_INFO_AND_VALIDATE_H_PRIME=>read_pairing_info_and_validate_h_prime(h,e,i), D2_A1_SEND_STORED_KM=>send_stored_km(h,e,i), D2_A1_VALIDATE_H_PRIME=>validate_h_prime(h,e,i), D2_A2_LOCALITY_CHECK=>locality_check(h,e,i), D2_A34_EXCHANGE_KS_AND_TEST_FOR_REPEATER=>exchange_ks_and_test_for_repeater(h,e,i), D2_SEND_CONTENT_STREAM_TYPE=>send_content_stream_type_dp(h,e,i), D2_ENABLE_ENCRYPTION=>enable_encryption(h,e,i), D2_A5_AUTHENTICATED=>authenticated(h,e,i), D2_A6_WAIT_FOR_RX_ID_LIST=>wait_for_rx_id_list(h,e,i), D2_A78_VERIFY_RX_ID_LIST_AND_SEND_ACK=>verify_rx_id_list_and_send_ack(h,e,i), D2_A9_SEND_STREAM_MANAGEMENT=>send_stream_management(h,e,i), D2_A9_VALIDATE_STREAM_READY=>validate_stream_ready(h,e,i), _=>MOD_HDCP_STATUS_INVALID_STATE }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
