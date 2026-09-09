/* Translated from hdcp.c. External types, constants, and helpers are supplied by hdcp.h. */

unsafe fn push_error_status(hdcp: *mut mod_hdcp, status: mod_hdcp_status) {
    let trace = &mut (*hdcp).connection.trace;
    let retry_limit: u8 = (*hdcp).connection.link.adjust.retry_limit;
    if trace.error_count < MAX_NUM_OF_ERROR_TRACE {
        trace.errors[trace.error_count as usize].status = status;
        trace.errors[trace.error_count as usize].state_id = (*hdcp).state.id;
        trace.error_count += 1;
        HDCP_ERROR_TRACE(hdcp, status);
    }
    if is_hdcp1(hdcp) {
        (*hdcp).connection.hdcp1_retry_count += 1;
        if (*hdcp).connection.hdcp1_retry_count == retry_limit { (*hdcp).connection.link.adjust.hdcp1.disable = 1; }
    } else if is_hdcp2(hdcp) {
        (*hdcp).connection.hdcp2_retry_count += 1;
        if (*hdcp).connection.hdcp2_retry_count == retry_limit { (*hdcp).connection.link.adjust.hdcp2.disable = 1; }
    }
}

unsafe fn is_cp_desired_hdcp1(hdcp: *mut mod_hdcp) -> u8 {
    let mut is_auth_needed = 0;
    for i in 0..MAX_NUM_OF_DISPLAYS {
        if (*hdcp).displays[i].state != MOD_HDCP_DISPLAY_INACTIVE && (*hdcp).displays[i].adjust.disable != MOD_HDCP_DISPLAY_DISABLE_AUTHENTICATION { is_auth_needed = 1; break; }
    }
    (is_auth_needed != 0 && !(*hdcp).connection.link.adjust.hdcp1.disable && !is_frl_hdcp(hdcp) && !(*hdcp).connection.is_hdcp1_revoked) as u8
}

unsafe fn is_cp_desired_hdcp2(hdcp: *mut mod_hdcp) -> u8 {
    let mut is_auth_needed = 0;
    for i in 0..MAX_NUM_OF_DISPLAYS {
        if (*hdcp).displays[i].state != MOD_HDCP_DISPLAY_INACTIVE && (*hdcp).displays[i].adjust.disable != MOD_HDCP_DISPLAY_DISABLE_AUTHENTICATION { is_auth_needed = 1; break; }
    }
    (is_auth_needed != 0 && !(*hdcp).connection.link.adjust.hdcp2.disable && !(*hdcp).connection.is_hdcp2_revoked) as u8
}

unsafe fn execution(hdcp: *mut mod_hdcp, event_ctx: *mut mod_hdcp_event_context, input: *mut mod_hdcp_transition_input) -> mod_hdcp_status {
    let mut status = MOD_HDCP_STATUS_SUCCESS;
    if is_in_initialized_state(hdcp) {
        if (*event_ctx).event != MOD_HDCP_EVENT_CALLBACK { (*event_ctx).unexpected_event = 1; return status; }
        core::ptr::write_bytes(input as *mut u8, 0, core::mem::size_of::<mod_hdcp_transition_input>());
    } else if is_in_cp_not_desired_state(hdcp) {
        if (*event_ctx).event != MOD_HDCP_EVENT_CALLBACK { (*event_ctx).unexpected_event = 1; return status; }
    } else if is_in_hdcp1_states(hdcp) { status = mod_hdcp_hdcp1_execution(hdcp, event_ctx, &mut (*input).hdcp1); }
    else if is_in_hdcp1_dp_states(hdcp) { status = mod_hdcp_hdcp1_dp_execution(hdcp, event_ctx, &mut (*input).hdcp1); }
    else if is_in_hdcp2_states(hdcp) { status = mod_hdcp_hdcp2_execution(hdcp, event_ctx, &mut (*input).hdcp2); }
    else if is_in_hdcp2_dp_states(hdcp) { status = mod_hdcp_hdcp2_dp_execution(hdcp, event_ctx, &mut (*input).hdcp2); }
    else { (*event_ctx).unexpected_event = 1; }
    status
}

unsafe fn transition(hdcp: *mut mod_hdcp, event_ctx: *mut mod_hdcp_event_context, input: *mut mod_hdcp_transition_input, output: *mut mod_hdcp_output) -> mod_hdcp_status {
    let mut status = MOD_HDCP_STATUS_SUCCESS;
    if (*event_ctx).unexpected_event { return status; }
    if is_in_initialized_state(hdcp) {
        if is_dp_hdcp(hdcp) {
            if is_cp_desired_hdcp2(hdcp) != 0 { callback_in_ms(0, output); set_state_id(hdcp, output, D2_A0_DETERMINE_RX_HDCP_CAPABLE); }
            else if is_cp_desired_hdcp1(hdcp) != 0 { callback_in_ms(0, output); set_state_id(hdcp, output, D1_A0_DETERMINE_RX_HDCP_CAPABLE); }
            else { callback_in_ms(0, output); set_state_id(hdcp, output, HDCP_CP_NOT_DESIRED); set_auth_complete(hdcp, output); }
        } else if is_hdmi_dvi_sl_hdcp(hdcp) {
            if is_cp_desired_hdcp2(hdcp) != 0 { callback_in_ms(0, output); set_state_id(hdcp, output, H2_A0_KNOWN_HDCP2_CAPABLE_RX); }
            else if is_cp_desired_hdcp1(hdcp) != 0 { callback_in_ms(0, output); set_state_id(hdcp, output, H1_A0_WAIT_FOR_ACTIVE_RX); }
            else { callback_in_ms(0, output); set_state_id(hdcp, output, HDCP_CP_NOT_DESIRED); set_auth_complete(hdcp, output); }
        } else { callback_in_ms(0, output); set_state_id(hdcp, output, HDCP_CP_NOT_DESIRED); set_auth_complete(hdcp, output); }
    } else if is_in_cp_not_desired_state(hdcp) { increment_stay_counter(hdcp); }
    else if is_in_hdcp1_states(hdcp) { status = mod_hdcp_hdcp1_transition(hdcp, event_ctx, &mut (*input).hdcp1, output); }
    else if is_in_hdcp1_dp_states(hdcp) { status = mod_hdcp_hdcp1_dp_transition(hdcp, event_ctx, &mut (*input).hdcp1, output); }
    else if is_in_hdcp2_states(hdcp) { status = mod_hdcp_hdcp2_transition(hdcp, event_ctx, &mut (*input).hdcp2, output); }
    else if is_in_hdcp2_dp_states(hdcp) { status = mod_hdcp_hdcp2_dp_transition(hdcp, event_ctx, &mut (*input).hdcp2, output); }
    else { status = MOD_HDCP_STATUS_INVALID_STATE; }
    status
}

unsafe fn reset_authentication(hdcp: *mut mod_hdcp, output: *mut mod_hdcp_output) -> mod_hdcp_status {
    let mut status = MOD_HDCP_STATUS_SUCCESS;
    if is_hdcp1(hdcp) {
        if (*hdcp).auth.trans_input.hdcp1.create_session != UNKNOWN { mod_hdcp_hdcp1_destroy_session(hdcp); }
        HDCP_TOP_RESET_AUTH_TRACE(hdcp); core::ptr::write_bytes(&mut (*hdcp).auth as *mut _ as *mut u8, 0, core::mem::size_of::<mod_hdcp_authentication>()); core::ptr::write_bytes(&mut (*hdcp).state as *mut _ as *mut u8, 0, core::mem::size_of::<mod_hdcp_state>()); set_state_id(hdcp, output, HDCP_INITIALIZED);
    } else if is_hdcp2(hdcp) {
        if (*hdcp).auth.trans_input.hdcp2.create_session == PASS { status = mod_hdcp_hdcp2_destroy_session(hdcp); if status != MOD_HDCP_STATUS_SUCCESS { (*output).callback_needed = 0; (*output).watchdog_timer_needed = 0; } }
        if status == MOD_HDCP_STATUS_SUCCESS { HDCP_TOP_RESET_AUTH_TRACE(hdcp); core::ptr::write_bytes(&mut (*hdcp).auth as *mut _ as *mut u8, 0, core::mem::size_of::<mod_hdcp_authentication>()); core::ptr::write_bytes(&mut (*hdcp).state as *mut _ as *mut u8, 0, core::mem::size_of::<mod_hdcp_state>()); set_state_id(hdcp, output, HDCP_INITIALIZED); }
    } else if is_in_cp_not_desired_state(hdcp) { HDCP_TOP_RESET_AUTH_TRACE(hdcp); core::ptr::write_bytes(&mut (*hdcp).auth as *mut _ as *mut u8, 0, core::mem::size_of::<mod_hdcp_authentication>()); core::ptr::write_bytes(&mut (*hdcp).state as *mut _ as *mut u8, 0, core::mem::size_of::<mod_hdcp_state>()); set_state_id(hdcp, output, HDCP_INITIALIZED); }
    (*output).watchdog_timer_stop = 1; (*output).callback_stop = 1; status
}

unsafe fn reset_connection(hdcp: *mut mod_hdcp, output: *mut mod_hdcp_output) -> mod_hdcp_status {
    core::ptr::write_bytes(output as *mut u8, 0, core::mem::size_of::<mod_hdcp_output>());
    let status = reset_authentication(hdcp, output); if status != MOD_HDCP_STATUS_SUCCESS { return status; }
    if current_state(hdcp) != HDCP_UNINITIALIZED { HDCP_TOP_RESET_CONN_TRACE(hdcp); set_state_id(hdcp, output, HDCP_UNINITIALIZED); }
    core::ptr::write_bytes(&mut (*hdcp).connection as *mut _ as *mut u8, 0, core::mem::size_of_val(&(*hdcp).connection)); status
}

unsafe fn update_display_adjustments(hdcp: *mut mod_hdcp, display: *mut mod_hdcp_display, adj: *mut mod_hdcp_display_adjustment) -> mod_hdcp_status {
    let mut status = MOD_HDCP_STATUS_NOT_IMPLEMENTED;
    if is_in_authenticated_states(hdcp) && is_dp_mst_hdcp(hdcp) && (*display).adjust.disable == true && (*adj).disable == false { (*display).adjust.disable = false; status = if is_hdcp1(hdcp) { mod_hdcp_hdcp1_enable_dp_stream_encryption(hdcp) } else if is_hdcp2(hdcp) { mod_hdcp_hdcp2_enable_dp_stream_encryption(hdcp) } else { status }; if status != MOD_HDCP_STATUS_SUCCESS { (*display).adjust.disable = true; } }
    if status == MOD_HDCP_STATUS_SUCCESS && core::slice::from_raw_parts(adj as *const u8, core::mem::size_of::<mod_hdcp_display_adjustment>()) != core::slice::from_raw_parts(&(*display).adjust as *const _ as *const u8, core::mem::size_of::<mod_hdcp_display_adjustment>()) { status = MOD_HDCP_STATUS_NOT_IMPLEMENTED; } status
}

pub unsafe fn mod_hdcp_get_memory_size() -> usize { core::mem::size_of::<mod_hdcp>() }

pub unsafe fn mod_hdcp_setup(hdcp: *mut mod_hdcp, config: *mut mod_hdcp_config) -> mod_hdcp_status { let mut output = core::mem::zeroed::<mod_hdcp_output>(); (*hdcp).config = *config; HDCP_TOP_INTERFACE_TRACE(hdcp); let status = reset_connection(hdcp, &mut output); if status != MOD_HDCP_STATUS_SUCCESS { push_error_status(hdcp, status); } status }

pub unsafe fn mod_hdcp_teardown(hdcp: *mut mod_hdcp) -> mod_hdcp_status { let mut output = core::mem::zeroed::<mod_hdcp_output>(); HDCP_TOP_INTERFACE_TRACE(hdcp); let status = reset_connection(hdcp, &mut output); if status == MOD_HDCP_STATUS_SUCCESS { core::ptr::write_bytes(hdcp as *mut u8, 0, core::mem::size_of::<mod_hdcp>()); } else { push_error_status(hdcp, status); } status }

pub unsafe fn mod_hdcp_add_display(hdcp: *mut mod_hdcp, link: *mut mod_hdcp_link, display: *mut mod_hdcp_display, output: *mut mod_hdcp_output) -> mod_hdcp_status {
    HDCP_TOP_INTERFACE_TRACE_WITH_INDEX(hdcp, (*display).index); core::ptr::write_bytes(output as *mut u8, 0, core::mem::size_of::<mod_hdcp_output>()); if (*display).state != MOD_HDCP_DISPLAY_ACTIVE { return MOD_HDCP_STATUS_SUCCESS; } if !get_active_display_at_index(hdcp, (*display).index).is_null() { return MOD_HDCP_STATUS_SUCCESS; } let display_container = get_empty_display_container(hdcp); if display_container.is_null() { push_error_status(hdcp, MOD_HDCP_STATUS_DISPLAY_OUT_OF_BOUND); return MOD_HDCP_STATUS_DISPLAY_OUT_OF_BOUND; } let mut status = reset_authentication(hdcp, output); if status == MOD_HDCP_STATUS_SUCCESS { reset_retry_counts(hdcp); core::ptr::write_bytes(&mut (*hdcp).connection.trace as *mut _ as *mut u8, 0, core::mem::size_of_val(&(*hdcp).connection.trace)); (*hdcp).connection.link = *link; *display_container = *display; status = mod_hdcp_add_display_to_topology(hdcp, display_container); if status == MOD_HDCP_STATUS_SUCCESS { if current_state(hdcp) != HDCP_INITIALIZED { set_state_id(hdcp, output, HDCP_INITIALIZED); } callback_in_ms((*hdcp).connection.link.adjust.auth_delay * 1000, output); } } if status != MOD_HDCP_STATUS_SUCCESS { push_error_status(hdcp, status); } status
}

pub unsafe fn mod_hdcp_remove_display(hdcp: *mut mod_hdcp, index: u8, output: *mut mod_hdcp_output) -> mod_hdcp_status { HDCP_TOP_INTERFACE_TRACE_WITH_INDEX(hdcp, index); core::ptr::write_bytes(output as *mut u8, 0, core::mem::size_of::<mod_hdcp_output>()); let display = get_active_display_at_index(hdcp, index); if display.is_null() { return MOD_HDCP_STATUS_SUCCESS; } let mut status = reset_authentication(hdcp, output); if status == MOD_HDCP_STATUS_SUCCESS { reset_retry_counts(hdcp); core::ptr::write_bytes(&mut (*hdcp).connection.trace as *mut _ as *mut u8, 0, core::mem::size_of_val(&(*hdcp).connection.trace)); status = mod_hdcp_remove_display_from_topology(hdcp, index); if status == MOD_HDCP_STATUS_SUCCESS { core::ptr::write_bytes(display as *mut u8, 0, core::mem::size_of::<mod_hdcp_display>()); if current_state(hdcp) != HDCP_UNINITIALIZED { callback_in_ms((*hdcp).connection.link.adjust.auth_delay * 1000, output); } } } if status != MOD_HDCP_STATUS_SUCCESS { push_error_status(hdcp, status); } status }

pub unsafe fn mod_hdcp_update_display(hdcp: *mut mod_hdcp, index: u8, link_adjust: *mut mod_hdcp_link_adjustment, display_adjust: *mut mod_hdcp_display_adjustment, output: *mut mod_hdcp_output) -> mod_hdcp_status { HDCP_TOP_INTERFACE_TRACE_WITH_INDEX(hdcp, index); core::ptr::write_bytes(output as *mut u8, 0, core::mem::size_of::<mod_hdcp_output>()); let display = get_active_display_at_index(hdcp, index); if display.is_null() { return MOD_HDCP_STATUS_DISPLAY_NOT_FOUND; } let la = core::slice::from_raw_parts(link_adjust as *const u8, core::mem::size_of::<mod_hdcp_link_adjustment>()); let oldla = core::slice::from_raw_parts(&(*hdcp).connection.link.adjust as *const _ as *const u8, core::mem::size_of::<mod_hdcp_link_adjustment>()); let da = core::slice::from_raw_parts(display_adjust as *const u8, core::mem::size_of::<mod_hdcp_display_adjustment>()); let oldda = core::slice::from_raw_parts(&(*display).adjust as *const _ as *const u8, core::mem::size_of::<mod_hdcp_display_adjustment>()); if la == oldla && da == oldda { return MOD_HDCP_STATUS_SUCCESS; } if la == oldla && da != oldda { let s = update_display_adjustments(hdcp, display, display_adjust); if s != MOD_HDCP_STATUS_NOT_IMPLEMENTED { return s; } } let mut status = reset_authentication(hdcp, output); if status == MOD_HDCP_STATUS_SUCCESS { reset_retry_counts(hdcp); core::ptr::write_bytes(&mut (*hdcp).connection.trace as *mut _ as *mut u8, 0, core::mem::size_of_val(&(*hdcp).connection.trace)); (*hdcp).connection.link.adjust = *link_adjust; (*display).adjust = *display_adjust; if current_state(hdcp) != HDCP_UNINITIALIZED { callback_in_ms(100, output); } } if status != MOD_HDCP_STATUS_SUCCESS { push_error_status(hdcp, status); } status }

pub unsafe fn mod_hdcp_query_display(hdcp: *mut mod_hdcp, index: u8, query: *mut mod_hdcp_display_query) -> mod_hdcp_status { let display = get_active_display_at_index(hdcp, index); if display.is_null() { return MOD_HDCP_STATUS_DISPLAY_NOT_FOUND; } (*query).link = &mut (*hdcp).connection.link; (*query).display = display; (*query).trace = &mut (*hdcp).connection.trace; (*query).encryption_status = MOD_HDCP_ENCRYPTION_STATUS_HDCP_OFF; if is_display_encryption_enabled(display) { if is_hdcp1(hdcp) { (*query).encryption_status = MOD_HDCP_ENCRYPTION_STATUS_HDCP1_ON; } else if is_hdcp2(hdcp) { (*query).encryption_status = match (*query).link.adjust.hdcp2.force_type { MOD_HDCP_FORCE_TYPE_0 => MOD_HDCP_ENCRYPTION_STATUS_HDCP2_TYPE0_ON, MOD_HDCP_FORCE_TYPE_1 => MOD_HDCP_ENCRYPTION_STATUS_HDCP2_TYPE1_ON, _ => MOD_HDCP_ENCRYPTION_STATUS_HDCP2_ON }; } } MOD_HDCP_STATUS_SUCCESS }

pub unsafe fn mod_hdcp_reset_connection(hdcp: *mut mod_hdcp, output: *mut mod_hdcp_output) -> mod_hdcp_status { HDCP_TOP_INTERFACE_TRACE(hdcp); let status = reset_connection(hdcp, output); if status != MOD_HDCP_STATUS_SUCCESS { push_error_status(hdcp, status); } status }

pub unsafe fn mod_hdcp_process_event(hdcp: *mut mod_hdcp, event: mod_hdcp_event, output: *mut mod_hdcp_output) -> mod_hdcp_status { HDCP_EVENT_TRACE(hdcp, event); core::ptr::write_bytes(output as *mut u8, 0, core::mem::size_of::<mod_hdcp_output>()); let mut ctx = core::mem::zeroed::<mod_hdcp_event_context>(); ctx.event = event; let exec_status = execution(hdcp, &mut ctx, &mut (*hdcp).auth.trans_input); let trans_status = transition(hdcp, &mut ctx, &mut (*hdcp).auth.trans_input, output); let mut status = if trans_status == MOD_HDCP_STATUS_SUCCESS { MOD_HDCP_STATUS_SUCCESS } else if exec_status == MOD_HDCP_STATUS_SUCCESS { let s = MOD_HDCP_STATUS_INTERNAL_POLICY_FAILURE; push_error_status(hdcp, s); s } else { push_error_status(hdcp, exec_status); exec_status }; if trans_status == MOD_HDCP_STATUS_RESET_NEEDED { mod_hdcp_log_ddc_trace(hdcp); let s = reset_authentication(hdcp, output); if s != MOD_HDCP_STATUS_SUCCESS { push_error_status(hdcp, s); } } if ctx.event == MOD_HDCP_EVENT_CPIRQ { status = mod_hdcp_clear_cp_irq_status(hdcp); if status != MOD_HDCP_STATUS_SUCCESS { push_error_status(hdcp, status); } } status }

pub unsafe fn mod_hdcp_signal_type_to_operation_mode(signal: signal_type) -> mod_hdcp_operation_mode { match signal { SIGNAL_TYPE_DVI_SINGLE_LINK | SIGNAL_TYPE_HDMI_TYPE_A | SIGNAL_TYPE_HDMI_FRL => MOD_HDCP_MODE_DEFAULT, SIGNAL_TYPE_EDP | SIGNAL_TYPE_DISPLAY_PORT | SIGNAL_TYPE_DISPLAY_PORT_MST => MOD_HDCP_MODE_DP, _ => MOD_HDCP_MODE_OFF } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
