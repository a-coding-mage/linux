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

#[inline]
unsafe fn validate_bksv(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    let mut n: u64 = 0;
    let mut count: u8 = 0;
    let bksv = core::slice::from_raw_parts_mut((&mut n as *mut u64) as *mut u8, 8);
    core::ptr::copy_nonoverlapping((*hdcp).auth.msg.hdcp1.bksv.as_ptr(), bksv.as_mut_ptr(), core::mem::size_of_val(&(*hdcp).auth.msg.hdcp1.bksv));
    while n != 0 { count += 1; n &= n - 1; }
    if count == 20 { (*hdcp).connection.trace.hdcp1.attempt_count += 1; MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP1_INVALID_BKSV }
}

#[inline] unsafe fn check_ksv_ready(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    if is_dp_hdcp(hdcp) { if (*hdcp).auth.msg.hdcp1.bstatus & DP_BSTATUS_READY != 0 { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP1_KSV_LIST_NOT_READY } }
    else if (*hdcp).auth.msg.hdcp1.bcaps & DRM_HDCP_DDC_BCAPS_KSV_FIFO_READY != 0 { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP1_KSV_LIST_NOT_READY }
}
#[inline] unsafe fn check_hdcp_capable_dp(hdcp: *mut mod_hdcp) -> mod_hdcp_status { if (*hdcp).auth.msg.hdcp1.bcaps & DP_BCAPS_HDCP_CAPABLE != 0 { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP1_NOT_CAPABLE } }
#[inline] unsafe fn check_r0p_available_dp(hdcp: *mut mod_hdcp) -> mod_hdcp_status { if is_dp_hdcp(hdcp) { if (*hdcp).auth.msg.hdcp1.bstatus & DP_BSTATUS_R0_PRIME_READY != 0 { MOD_HDCP_STATUS_SUCCESS } else { MOD_HDCP_STATUS_HDCP1_R0_PRIME_PENDING } } else { MOD_HDCP_STATUS_INVALID_OPERATION } }
#[inline] unsafe fn check_link_integrity_dp(hdcp: *mut mod_hdcp) -> mod_hdcp_status { if (*hdcp).auth.msg.hdcp1.bstatus & DP_BSTATUS_LINK_FAILURE != 0 { MOD_HDCP_STATUS_HDCP1_LINK_INTEGRITY_FAILURE } else { MOD_HDCP_STATUS_SUCCESS } }
#[inline] unsafe fn check_no_reauthentication_request_dp(hdcp: *mut mod_hdcp) -> mod_hdcp_status { if (*hdcp).auth.msg.hdcp1.bstatus & DP_BSTATUS_REAUTH_REQ != 0 { MOD_HDCP_STATUS_HDCP1_REAUTH_REQUEST_ISSUED } else { MOD_HDCP_STATUS_SUCCESS } }
#[inline] unsafe fn check_no_max_cascade(hdcp: *mut mod_hdcp) -> mod_hdcp_status { let n = if is_dp_hdcp(hdcp) { (*hdcp).auth.msg.hdcp1.binfo_dp >> 8 } else { (*hdcp).auth.msg.hdcp1.bstatus >> 8 }; if DRM_HDCP_MAX_CASCADE_EXCEEDED(n) { MOD_HDCP_STATUS_HDCP1_MAX_CASCADE_EXCEEDED_FAILURE } else { MOD_HDCP_STATUS_SUCCESS } }
#[inline] unsafe fn check_no_max_devs(hdcp: *mut mod_hdcp) -> mod_hdcp_status { let n = if is_dp_hdcp(hdcp) { (*hdcp).auth.msg.hdcp1.binfo_dp } else { (*hdcp).auth.msg.hdcp1.bstatus }; if DRM_HDCP_MAX_DEVICE_EXCEEDED(n) { MOD_HDCP_STATUS_HDCP1_MAX_DEVS_EXCEEDED_FAILURE } else { MOD_HDCP_STATUS_SUCCESS } }
#[inline] unsafe fn get_device_count(hdcp: *mut mod_hdcp) -> u8 { if is_dp_hdcp(hdcp) { DRM_HDCP_NUM_DOWNSTREAM((*hdcp).auth.msg.hdcp1.binfo_dp) } else { DRM_HDCP_NUM_DOWNSTREAM((*hdcp).auth.msg.hdcp1.bstatus) } }
#[inline] unsafe fn check_device_count(hdcp: *mut mod_hdcp) -> mod_hdcp_status {
    let count = get_device_count(hdcp);
    if count == 0 { return MOD_HDCP_STATUS_HDCP1_DEVICE_COUNT_MISMATCH_FAILURE; }
    (*hdcp).connection.trace.hdcp1.downstream_device_count = count;
    if (1 + count as u32) < get_active_display_count(hdcp) { MOD_HDCP_STATUS_HDCP1_DEVICE_COUNT_MISMATCH_FAILURE } else { MOD_HDCP_STATUS_SUCCESS }
}

unsafe fn run_event(hdcp: *mut mod_hdcp, event_ctx: *mut mod_hdcp_event_context) -> bool { (*event_ctx).event == MOD_HDCP_EVENT_CALLBACK }

unsafe fn wait_for_active_rx(hdcp: *mut mod_hdcp, event_ctx: *mut mod_hdcp_event_context, input: *mut mod_hdcp_transition_input_hdcp1) -> mod_hdcp_status { let mut s=MOD_HDCP_STATUS_SUCCESS; if !run_event(hdcp,event_ctx){(*event_ctx).unexpected_event=1;return s;} if !mod_hdcp_execute_and_set(mod_hdcp_read_bksv,&mut (*input).bksv_read,&mut s,hdcp,"bksv_read".as_mut_ptr() as *mut i8){return s;} mod_hdcp_execute_and_set(mod_hdcp_read_bcaps,&mut (*input).bcaps_read,&mut s,hdcp,"bcaps_read".as_mut_ptr() as *mut i8); s }

unsafe fn execute_sequence(hdcp:*mut mod_hdcp,event_ctx:*mut mod_hdcp_event_context,input:*mut mod_hdcp_transition_input_hdcp1, funcs:&[mod_hdcp_action], flags:&mut [*mut u8], names:&[&str])->mod_hdcp_status { let mut s=MOD_HDCP_STATUS_SUCCESS; if (*event_ctx).event != MOD_HDCP_EVENT_CALLBACK {(*event_ctx).unexpected_event=1;return s;} for i in 0..funcs.len(){if !mod_hdcp_execute_and_set(funcs[i],flags[i],&mut s,hdcp,names[i].as_ptr() as *mut i8){break;}} s }

unsafe fn exchange_ksvs(h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,i:*mut mod_hdcp_transition_input_hdcp1)->mod_hdcp_status { execute_sequence(h,e,i,&[mod_hdcp_hdcp1_create_session,mod_hdcp_write_an,mod_hdcp_write_aksv,mod_hdcp_read_bksv,validate_bksv],&mut [&mut (*i).create_session,&mut (*i).an_write,&mut (*i).aksv_write,&mut (*i).bksv_read,&mut (*i).bksv_validation], &["create_session","an_write","aksv_write","bksv_read","bksv_validation"]) }

pub unsafe fn mod_hdcp_execute_and_set(func: mod_hdcp_action, flag:*mut u8, status:*mut mod_hdcp_status, hdcp:*mut mod_hdcp, _str:*mut i8)->u8 { *status=func(hdcp); if *status==MOD_HDCP_STATUS_SUCCESS && *flag!=PASS {*flag=PASS;} else if *status!=MOD_HDCP_STATUS_SUCCESS && *flag!=FAIL {*flag=FAIL;} (*status==MOD_HDCP_STATUS_SUCCESS) as u8 }

unsafe fn computations_validate_rx_test_for_repeater(h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,i:*mut mod_hdcp_transition_input_hdcp1)->mod_hdcp_status { let mut s=MOD_HDCP_STATUS_SUCCESS; if (*e).event!=MOD_HDCP_EVENT_CALLBACK {(*e).unexpected_event=1;return s;} if !mod_hdcp_execute_and_set(mod_hdcp_read_r0p,&mut (*i).r0p_read,&mut s,h,"r0p_read".as_ptr() as *mut i8){return s;} if !mod_hdcp_execute_and_set(mod_hdcp_hdcp1_validate_rx,&mut (*i).rx_validation,&mut s,h,"rx_validation".as_ptr() as *mut i8){return s;} mod_hdcp_execute_and_set(mod_hdcp_hdcp1_enable_encryption,&mut (*i).encryption,&mut s,h,"encryption".as_ptr() as *mut i8); s }
unsafe fn authenticated(h:*mut mod_hdcp,e:*mut mod_hdcp,i:*mut mod_hdcp_transition_input_hdcp1)->mod_hdcp_status { let mut s=MOD_HDCP_STATUS_SUCCESS; if (*e).event!=MOD_HDCP_EVENT_CALLBACK {(*e).unexpected_event=1;return s;} mod_hdcp_execute_and_set(mod_hdcp_hdcp1_link_maintenance,&mut (*i).link_maintenance,&mut s,h,"link_maintenance".as_ptr() as *mut i8);s }
unsafe fn wait_for_ready(_h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,_i:*mut mod_hdcp_transition_input_hdcp1)->mod_hdcp_status { if (*e).event!=MOD_HDCP_EVENT_CALLBACK && (*e).event!=MOD_HDCP_EVENT_CPIRQ && (*e).event!=MOD_HDCP_EVENT_WATCHDOG_TIMEOUT {(*e).unexpected_event=1;} MOD_HDCP_STATUS_SUCCESS }
unsafe fn read_ksv_list(_h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,_i:*mut mod_hdcp_transition_input_hdcp1)->mod_hdcp_status { if (*e).event!=MOD_HDCP_EVENT_CALLBACK {(*e).unexpected_event=1;} MOD_HDCP_STATUS_SUCCESS }
unsafe fn determine_rx_hdcp_capable_dp(h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,i:*mut mod_hdcp_transition_input_hdcp1)->mod_hdcp_status { let mut s=MOD_HDCP_STATUS_SUCCESS;if (*e).event!=MOD_HDCP_EVENT_CALLBACK {(*e).unexpected_event=1;return s;} mod_hdcp_execute_and_set(mod_hdcp_read_bcaps,&mut (*i).bcaps_read,&mut s,h,"bcaps_read".as_ptr() as *mut i8);s }
unsafe fn wait_for_r0_prime_dp(_h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,_i:*mut mod_hdcp_transition_input_hdcp1)->mod_hdcp_status { if (*e).event!=MOD_HDCP_EVENT_CPIRQ && (*e).event!=MOD_HDCP_EVENT_WATCHDOG_TIMEOUT {(*e).unexpected_event=1;} MOD_HDCP_STATUS_SUCCESS }
unsafe fn authenticated_dp(_h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,_i:*mut mod_hdcp_transition_input_hdcp1)->mod_hdcp_status { if (*e).event!=MOD_HDCP_EVENT_CPIRQ {(*e).unexpected_event=1;} MOD_HDCP_STATUS_SUCCESS }
pub unsafe fn mod_hdcp_hdcp1_execution(h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,i:*mut mod_hdcp_transition_input_hdcp1)->mod_hdcp_status { match current_state(h) { H1_A0_WAIT_FOR_ACTIVE_RX=>wait_for_active_rx(h,e,i),H1_A1_EXCHANGE_KSVS=>exchange_ksvs(h,e,i),H1_A2_COMPUTATIONS_A3_VALIDATE_RX_A6_TEST_FOR_REPEATER=>computations_validate_rx_test_for_repeater(h,e,i),H1_A45_AUTHENTICATED=>authenticated(h,e,i),H1_A8_WAIT_FOR_READY=>wait_for_ready(h,e,i),H1_A9_READ_KSV_LIST=>read_ksv_list(h,e,i),_=>MOD_HDCP_STATUS_INVALID_STATE } }
pub unsafe fn mod_hdcp_hdcp1_dp_execution(h:*mut mod_hdcp,e:*mut mod_hdcp_event_context,i:*mut mod_hdcp_transition_input_hdcp1)->mod_hdcp_status { match current_state(h) { D1_A0_DETERMINE_RX_HDCP_CAPABLE=>determine_rx_hdcp_capable_dp(h,e,i),D1_A1_EXCHANGE_KSVS=>exchange_ksvs(h,e,i),D1_A23_WAIT_FOR_R0_PRIME=>wait_for_r0_prime_dp(h,e,i),D1_A2_COMPUTATIONS_A3_VALIDATE_RX_A5_TEST_FOR_REPEATER=>computations_validate_rx_test_for_repeater(h,e,i),D1_A4_AUTHENTICATED=>authenticated_dp(h,e,i),D1_A6_WAIT_FOR_READY=>wait_for_ready(h,e,i),D1_A7_READ_KSV_LIST=>read_ksv_list(h,e,i),_=>MOD_HDCP_STATUS_INVALID_STATE } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
