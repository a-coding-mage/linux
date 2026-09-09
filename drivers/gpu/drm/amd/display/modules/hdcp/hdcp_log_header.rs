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

macro_rules! HDCP_LOG_ERR { ($hdcp:expr, $($arg:tt)*) => { DRM_DEBUG_KMS!($($arg)*) }; }
macro_rules! HDCP_LOG_VER { ($hdcp:expr, $($arg:tt)*) => { DRM_DEBUG_KMS!($($arg)*) }; }
macro_rules! HDCP_LOG_FSM { ($hdcp:expr, $($arg:tt)*) => { DRM_DEBUG_KMS!($($arg)*) }; }
macro_rules! HDCP_LOG_TOP { ($hdcp:expr, $($arg:tt)*) => { pr_debug!(concat!("[HDCP_TOP]:", $($arg)*)); }; }
macro_rules! HDCP_LOG_DDC { ($hdcp:expr, $($arg:tt)*) => { pr_debug!(concat!("[HDCP_DDC]:", $($arg)*)); }; }
macro_rules! HDCP_LOG_TRA { ($hdcp:expr) => {{}}; }

macro_rules! HDCP_ERROR_TRACE { ($hdcp:expr, $status:expr) => {
    HDCP_LOG_ERR!($hdcp, "[Link %d] WARNING %s IN STATE %s STAY COUNT %d",
        $hdcp.config.index, mod_hdcp_status_to_str($status),
        mod_hdcp_state_id_to_str($hdcp.state.id), $hdcp.state.stay_count);
} }
macro_rules! HDCP_HDCP1_ENABLED_TRACE { ($hdcp:expr, $displayIndex:expr) => {
    HDCP_LOG_VER!($hdcp, "[Link %d] HDCP 1.4 enabled on display %d", $hdcp.config.index, $displayIndex);
} }
macro_rules! HDCP_HDCP2_ENABLED_TRACE { ($hdcp:expr, $displayIndex:expr) => {
    HDCP_LOG_VER!($hdcp, "[Link %d] HDCP 2.2 enabled on display %d", $hdcp.config.index, $displayIndex);
} }
macro_rules! HDCP_HDCP1_DISABLED_TRACE { ($hdcp:expr, $displayIndex:expr) => {
    HDCP_LOG_VER!($hdcp, "[Link %d] HDCP 1.4 disabled on display %d", $hdcp.config.index, $displayIndex);
} }
macro_rules! HDCP_HDCP2_DISABLED_TRACE { ($hdcp:expr, $displayIndex:expr) => {
    HDCP_LOG_VER!($hdcp, "[Link %d] HDCP 2.2 disabled on display %d", $hdcp.config.index, $displayIndex);
} }

macro_rules! HDCP_REMOVE_DISPLAY_TRACE { ($hdcp:expr, $displayIndex:expr) => {
    HDCP_LOG_FSM!($hdcp, "[Link %d] HDCP_REMOVE_DISPLAY index %d", $hdcp.config.index, $displayIndex);
} }
macro_rules! HDCP_INPUT_PASS_TRACE { ($hdcp:expr, $str:expr) => {
    HDCP_LOG_FSM!($hdcp, "[Link %d]\tPASS %s", $hdcp.config.index, $str);
} }
macro_rules! HDCP_INPUT_FAIL_TRACE { ($hdcp:expr, $str:expr) => {
    HDCP_LOG_FSM!($hdcp, "[Link %d]\tFAIL %s", $hdcp.config.index, $str);
} }
macro_rules! HDCP_NEXT_STATE_TRACE { ($hdcp:expr, $id:expr, $output:expr) => {{
    if $output.watchdog_timer_needed {
        HDCP_LOG_FSM!($hdcp, "[Link %d] > %s with %d ms watchdog", $hdcp.config.index,
            mod_hdcp_state_id_to_str($id), $output.watchdog_timer_delay);
    } else {
        HDCP_LOG_FSM!($hdcp, "[Link %d] > %s", $hdcp.config.index, mod_hdcp_state_id_to_str($id));
    }
}} }
macro_rules! HDCP_TIMEOUT_TRACE { ($hdcp:expr) => { HDCP_LOG_FSM!($hdcp, "[Link %d] --> TIMEOUT", $hdcp.config.index); } }
macro_rules! HDCP_CPIRQ_TRACE { ($hdcp:expr) => { HDCP_LOG_FSM!($hdcp, "[Link %d] --> CPIRQ", $hdcp.config.index); } }
macro_rules! HDCP_EVENT_TRACE { ($hdcp:expr, $event:expr) => {{
    if $event == MOD_HDCP_EVENT_WATCHDOG_TIMEOUT { HDCP_TIMEOUT_TRACE!($hdcp); }
    else if $event == MOD_HDCP_EVENT_CPIRQ { HDCP_CPIRQ_TRACE!($hdcp); }
}} }

/* TODO: find some way to tell if logging is off to save time */
macro_rules! HDCP_DDC_READ_TRACE { ($hdcp:expr, $msg_name:expr, $msg:expr, $msg_size:expr) => {{
    mod_hdcp_dump_binary_message($msg, $msg_size, $hdcp.buf, core::mem::size_of_val(&$hdcp.buf));
    HDCP_LOG_DDC!($hdcp, "[Link %d] Read %s%s", $hdcp.config.index, $msg_name, $hdcp.buf);
}} }
macro_rules! HDCP_DDC_WRITE_TRACE { ($hdcp:expr, $msg_name:expr, $msg:expr, $msg_size:expr) => {{
    mod_hdcp_dump_binary_message($msg, $msg_size, $hdcp.buf, core::mem::size_of_val(&$hdcp.buf));
    HDCP_LOG_DDC!($hdcp, "[Link %d] Write %s%s", $hdcp.config.index, $msg_name, $hdcp.buf);
}} }
macro_rules! HDCP_TOP_ADD_DISPLAY_TRACE { ($hdcp:expr, $i:expr) => { HDCP_LOG_TOP!($hdcp, "[Link %d]\tadd display %d", $hdcp.config.index, $i); } }
macro_rules! HDCP_TOP_REMOVE_DISPLAY_TRACE { ($hdcp:expr, $i:expr) => { HDCP_LOG_TOP!($hdcp, "[Link %d]\tremove display %d", $hdcp.config.index, $i); } }
macro_rules! HDCP_TOP_HDCP1_DESTROY_SESSION_TRACE { ($hdcp:expr) => { HDCP_LOG_TOP!($hdcp, "[Link %d]\tdestroy hdcp1 session", $hdcp.config.index); } }
macro_rules! HDCP_TOP_HDCP2_DESTROY_SESSION_TRACE { ($hdcp:expr) => { HDCP_LOG_TOP!($hdcp, "[Link %d]\tdestroy hdcp2 session", $hdcp.config.index); } }
macro_rules! HDCP_TOP_RESET_AUTH_TRACE { ($hdcp:expr) => { HDCP_LOG_TOP!($hdcp, "[Link %d]\treset authentication", $hdcp.config.index); } }
macro_rules! HDCP_TOP_RESET_CONN_TRACE { ($hdcp:expr) => { HDCP_LOG_TOP!($hdcp, "[Link %d]\treset connection", $hdcp.config.index); } }
macro_rules! HDCP_TOP_INTERFACE_TRACE { ($hdcp:expr) => {{ HDCP_LOG_TOP!($hdcp, "\n"); HDCP_LOG_TOP!($hdcp, "[Link %d] %s", $hdcp.config.index, __func__); }} }
macro_rules! HDCP_TOP_INTERFACE_TRACE_WITH_INDEX { ($hdcp:expr, $i:expr) => {{ HDCP_LOG_TOP!($hdcp, "\n"); HDCP_LOG_TOP!($hdcp, "[Link %d] %s display %d", $hdcp.config.index, __func__, $i); }} }
macro_rules! HDCP_AUTH_COMPLETE_TRACE { ($hdcp:expr) => {{ mod_hdcp_log_ddc_trace($hdcp); HDCP_LOG_TRA!($hdcp); }} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
