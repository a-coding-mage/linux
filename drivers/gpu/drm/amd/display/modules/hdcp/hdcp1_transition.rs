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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

pub unsafe fn mod_hdcp_hdcp1_transition(
    hdcp: *mut mod_hdcp,
    event_ctx: *mut mod_hdcp_event_context,
    input: *mut mod_hdcp_transition_input_hdcp1,
    output: *mut mod_hdcp_output,
) -> mod_hdcp_status {
    let mut status = MOD_HDCP_STATUS_SUCCESS;
    let conn = &mut (*hdcp).connection;
    let adjust = &mut (*hdcp).connection.link.adjust;

    match current_state(hdcp) {
        H1_A0_WAIT_FOR_ACTIVE_RX => {
            if (*input).bksv_read != PASS || (*input).bcaps_read != PASS {
                /* 1A-04: repeatedly attempts on port access failure */
                callback_in_ms(500, output);
                increment_stay_counter(hdcp);
            } else {
                callback_in_ms(0, output);
                set_state_id(hdcp, output, H1_A1_EXCHANGE_KSVS);
            }
        }
        H1_A1_EXCHANGE_KSVS => {
            if (*input).create_session != PASS {
                /* out of sync with psp state */
                adjust.hdcp1.disable = 1;
                fail_and_restart_in_ms(0, &mut status, output);
            } else if (*input).an_write != PASS || (*input).aksv_write != PASS ||
                (*input).bksv_read != PASS || (*input).bksv_validation != PASS ||
                (*input).ainfo_write == FAIL {
                /* 1A-05: consider invalid bksv a failure */
                fail_and_restart_in_ms(0, &mut status, output);
            } else {
                callback_in_ms(300, output);
                set_state_id(hdcp, output, H1_A2_COMPUTATIONS_A3_VALIDATE_RX_A6_TEST_FOR_REPEATER);
            }
        }
        H1_A2_COMPUTATIONS_A3_VALIDATE_RX_A6_TEST_FOR_REPEATER => {
            if (*input).bcaps_read != PASS || (*input).r0p_read != PASS {
                fail_and_restart_in_ms(0, &mut status, output);
            } else if (*input).rx_validation != PASS {
                /* 1A-06: consider invalid r0' a failure */
                /* 1A-08: consider bksv listed in SRM a failure */
                /* some slow RX will fail rx validation when it is not ready. give it more time to react before retry. */
                fail_and_restart_in_ms(1000, &mut status, output);
            } else if !conn.is_repeater && (*input).encryption != PASS {
                fail_and_restart_in_ms(0, &mut status, output);
            } else if conn.is_repeater {
                callback_in_ms(0, output);
                set_watchdog_in_ms(hdcp, 5000, output);
                set_state_id(hdcp, output, H1_A8_WAIT_FOR_READY);
            } else {
                callback_in_ms(0, output);
                set_state_id(hdcp, output, H1_A45_AUTHENTICATED);
                set_auth_complete(hdcp, output);
            }
        }
        H1_A45_AUTHENTICATED => {
            if (*input).link_maintenance == FAIL {
                /* 1A-07: consider invalid ri' a failure */
                /* 1A-07a: consider read ri' not returned a failure */
                fail_and_restart_in_ms(0, &mut status, output);
            } else {
                callback_in_ms(500, output);
                increment_stay_counter(hdcp);
            }
        }
        H1_A8_WAIT_FOR_READY => {
            if (*input).ready_check != PASS {
                if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT {
                    /* 1B-03: fail hdcp on ksv list READY timeout */
                    /* prevent black screen in next attempt */
                    adjust.hdcp1.postpone_encryption = 1;
                    fail_and_restart_in_ms(0, &mut status, output);
                } else {
                    /* continue ksv list READY polling */
                    callback_in_ms(500, output);
                    increment_stay_counter(hdcp);
                }
            } else {
                callback_in_ms(0, output);
                set_state_id(hdcp, output, H1_A9_READ_KSV_LIST);
            }
        }
        H1_A9_READ_KSV_LIST => {
            if (*input).bstatus_read != PASS || (*input).max_cascade_check != PASS ||
                (*input).max_devs_check != PASS || (*input).device_count_check != PASS ||
                (*input).ksvlist_read != PASS || (*input).vp_read != PASS ||
                (*input).ksvlist_vp_validation != PASS || (*input).encryption != PASS {
                /* 1B-06: consider MAX_CASCADE_EXCEEDED a failure */
                /* 1B-05: consider MAX_DEVS_EXCEEDED a failure */
                /* 1B-04: consider invalid v' a failure */
                fail_and_restart_in_ms(0, &mut status, output);
            } else {
                callback_in_ms(0, output);
                set_state_id(hdcp, output, H1_A45_AUTHENTICATED);
                set_auth_complete(hdcp, output);
            }
        }
        _ => {
            status = MOD_HDCP_STATUS_INVALID_STATE;
            fail_and_restart_in_ms(0, &mut status, output);
        }
    }
    status
}

pub unsafe fn mod_hdcp_hdcp1_dp_transition(
    hdcp: *mut mod_hdcp,
    event_ctx: *mut mod_hdcp_event_context,
    input: *mut mod_hdcp_transition_input_hdcp1,
    output: *mut mod_hdcp_output,
) -> mod_hdcp_status {
    let mut status = MOD_HDCP_STATUS_SUCCESS;
    let conn = &mut (*hdcp).connection;
    let adjust = &mut (*hdcp).connection.link.adjust;

    match current_state(hdcp) {
        D1_A0_DETERMINE_RX_HDCP_CAPABLE => {
            if (*input).bcaps_read != PASS {
                /* 1A-04: no authentication on bcaps read failure */
                fail_and_restart_in_ms(0, &mut status, output);
            } else if (*input).hdcp_capable_dp != PASS {
                adjust.hdcp1.disable = 1;
                fail_and_restart_in_ms(0, &mut status, output);
            } else {
                callback_in_ms(0, output);
                set_state_id(hdcp, output, D1_A1_EXCHANGE_KSVS);
            }
        }
        D1_A1_EXCHANGE_KSVS => {
            if (*input).create_session != PASS {
                /* out of sync with psp state */
                adjust.hdcp1.disable = 1;
                fail_and_restart_in_ms(0, &mut status, output);
            } else if (*input).an_write != PASS || (*input).aksv_write != PASS ||
                (*input).bksv_read != PASS || (*input).bksv_validation != PASS ||
                (*input).ainfo_write == FAIL {
                /* 1A-05: consider invalid bksv a failure */
                fail_and_restart_in_ms(0, &mut status, output);
            } else {
                set_watchdog_in_ms(hdcp, 100, output);
                set_state_id(hdcp, output, D1_A23_WAIT_FOR_R0_PRIME);
            }
        }
        D1_A23_WAIT_FOR_R0_PRIME => {
            if (*input).bstatus_read != PASS {
                fail_and_restart_in_ms(0, &mut status, output);
            } else if (*input).r0p_available_dp != PASS {
                if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { fail_and_restart_in_ms(0, &mut status, output); } else { increment_stay_counter(hdcp); }
            } else {
                callback_in_ms(0, output);
                set_state_id(hdcp, output, D1_A2_COMPUTATIONS_A3_VALIDATE_RX_A5_TEST_FOR_REPEATER);
            }
        }
        D1_A2_COMPUTATIONS_A3_VALIDATE_RX_A5_TEST_FOR_REPEATER => {
            if (*input).r0p_read != PASS {
                fail_and_restart_in_ms(0, &mut status, output);
            } else if (*input).rx_validation != PASS {
                if (*hdcp).state.stay_count < 2 && !(*hdcp).connection.is_hdcp1_revoked { callback_in_ms(0, output); increment_stay_counter(hdcp); } else { fail_and_restart_in_ms(1000, &mut status, output); }
            } else if (!conn.is_repeater && (*input).encryption != PASS) || (!conn.is_repeater && is_dp_mst_hdcp(hdcp) && (*input).stream_encryption_dp != PASS) {
                fail_and_restart_in_ms(0, &mut status, output);
            } else if conn.hdcp1_retry_count < conn.link.adjust.hdcp1.min_auth_retries_wa {
                fail_and_restart_in_ms(200, &mut status, output);
            } else if conn.is_repeater {
                set_watchdog_in_ms(hdcp, 5000, output);
                set_state_id(hdcp, output, D1_A6_WAIT_FOR_READY);
            } else {
                set_state_id(hdcp, output, D1_A4_AUTHENTICATED);
                set_auth_complete(hdcp, output);
            }
        }
        D1_A4_AUTHENTICATED => {
            if (*input).link_integrity_check == FAIL || (*input).reauth_request_check == FAIL { /* 1A-07: restart hdcp on a link integrity failure */ fail_and_restart_in_ms(0, &mut status, output); }
        }
        D1_A6_WAIT_FOR_READY => {
            if (*input).link_integrity_check == FAIL || (*input).reauth_request_check == FAIL { fail_and_restart_in_ms(0, &mut status, output); }
            else if (*input).ready_check != PASS {
                if (*event_ctx).event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { /* 1B-04: fail hdcp on ksv list READY timeout */ /* prevent black screen in next attempt */ adjust.hdcp1.postpone_encryption = 1; fail_and_restart_in_ms(0, &mut status, output); }
                else { increment_stay_counter(hdcp); }
            } else { callback_in_ms(0, output); set_state_id(hdcp, output, D1_A7_READ_KSV_LIST); }
        }
        D1_A7_READ_KSV_LIST => {
            if (*input).binfo_read_dp != PASS || (*input).max_cascade_check != PASS || (*input).max_devs_check != PASS { /* 1B-06: consider MAX_DEVS_EXCEEDED a failure */ /* 1B-07: consider MAX_CASCADE_EXCEEDED a failure */ fail_and_restart_in_ms(0, &mut status, output); }
            else if (*input).device_count_check != PASS { /* some slow dongle doesn't update device count as soon as downstream is connected. give it more time to react. */ adjust.hdcp1.postpone_encryption = 1; fail_and_restart_in_ms(1000, &mut status, output); }
            else if (*input).ksvlist_read != PASS || (*input).vp_read != PASS { fail_and_restart_in_ms(0, &mut status, output); }
            else if (*input).ksvlist_vp_validation != PASS { if (*hdcp).state.stay_count < 2 && !(*hdcp).connection.is_hdcp1_revoked { callback_in_ms(0, output); increment_stay_counter(hdcp); } else { fail_and_restart_in_ms(0, &mut status, output); } }
            else if (*input).encryption != PASS || (is_dp_mst_hdcp(hdcp) && (*input).stream_encryption_dp != PASS) { fail_and_restart_in_ms(0, &mut status, output); }
            else { set_state_id(hdcp, output, D1_A4_AUTHENTICATED); set_auth_complete(hdcp, output); }
        }
        _ => { fail_and_restart_in_ms(0, &mut status, output); }
    }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
