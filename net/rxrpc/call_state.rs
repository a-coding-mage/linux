// SPDX-License-Identifier: GPL-2.0-or-later
/* Call state changing functions.
 *
 * Copyright (C) 2022 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Definitions supplied by the surrounding rxrpc implementation are expected
// to provide `rxrpc_call`, the rxrpc enums and constants, and these helpers.

extern "C" {
    fn __rxrpc_call_state(call: *mut rxrpc_call) -> rxrpc_call_state;
    fn rxrpc_set_call_state(call: *mut rxrpc_call, state: rxrpc_call_state);
    fn trace_rxrpc_call_complete(call: *mut rxrpc_call);
    fn wake_up(waitq: *mut core::ffi::c_void);
    fn rxrpc_notify_socket(call: *mut rxrpc_call);
    fn trace_rxrpc_abort(
        debug_id: u32,
        why: rxrpc_abort_reason,
        cid: u32,
        call_id: u32,
        seq: rxrpc_seq_t,
        abort_code: u32,
        error: i32,
    );
    fn rxrpc_send_abort_packet(call: *mut rxrpc_call);
    fn test_bit(bit: u32, addr: *const core::ffi::c_ulong) -> bool;
    fn __test_and_set_bit(bit: u32, addr: *mut core::ffi::c_ulong) -> bool;
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

/*
 * Transition a call to the complete state.
 */
pub unsafe fn rxrpc_set_call_completion(
    call: *mut rxrpc_call,
    compl: rxrpc_call_completion,
    abort_code: u32,
    error: i32,
) -> bool {
    if __rxrpc_call_state(call) == RXRPC_CALL_COMPLETE {
        return false;
    }

    (*call).abort_code = abort_code;
    (*call).error = error;
    (*call).completion = compl;
    /* Allow reader of completion state to operate locklessly */
    rxrpc_set_call_state(call, RXRPC_CALL_COMPLETE);
    trace_rxrpc_call_complete(call);
    wake_up(&mut (*call).waitq as *mut _ as *mut core::ffi::c_void);
    rxrpc_notify_socket(call);
    true
}

/*
 * Record that a call successfully completed.
 */
pub unsafe fn rxrpc_call_completed(call: *mut rxrpc_call) -> bool {
    rxrpc_set_call_completion(call, RXRPC_CALL_SUCCEEDED, 0, 0)
}

/*
 * Record that a call is locally aborted.
 */
pub unsafe fn rxrpc_abort_call(
    call: *mut rxrpc_call,
    seq: rxrpc_seq_t,
    abort_code: u32,
    error: i32,
    why: rxrpc_abort_reason,
) -> bool {
    trace_rxrpc_abort(
        (*call).debug_id,
        why,
        (*call).cid,
        (*call).call_id,
        seq,
        abort_code,
        error,
    );
    if !rxrpc_set_call_completion(call, RXRPC_CALL_LOCALLY_ABORTED, abort_code, error) {
        return false;
    }
    if test_bit(RXRPC_CALL_EXPOSED, &(*call).flags) {
        rxrpc_send_abort_packet(call);
    }
    true
}

/*
 * Record that a call errored out before even getting off the ground, thereby
 * setting the state to allow it to be destroyed.
 */
pub unsafe fn rxrpc_prefail_call(
    call: *mut rxrpc_call,
    compl: rxrpc_call_completion,
    error: i32,
) {
    (*call).abort_code = RX_CALL_DEAD;
    (*call).error = error;
    (*call).completion = compl;
    (*call)._state = RXRPC_CALL_COMPLETE;
    trace_rxrpc_call_complete(call);
    WARN_ON_ONCE(__test_and_set_bit(RXRPC_CALL_RELEASED, &mut (*call).flags));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
