// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * X.25 Packet Layer release 002
 *
 * This is ALPHA test software. This code may break your machine,
 * randomly fail to work with new releases, misbehave and/or generally
 * screw up. It might even work.
 *
 * This code REQUIRES 2.1.15 or higher
 *
 * History
 * X.25 001 Jonathan Naylor Started coding.
 * X.25 002 Jonathan Naylor New timer architecture.
 *                    Centralised disconnection processing.
 */

// Linux kernel headers supplied by the surrounding translation unit.

unsafe fn x25_init_timers(sk: *mut sock) {
    let x25 = x25_sk(sk);

    timer_setup(&mut (*x25).timer, x25_timer_expiry, 0);

    // initialized by sock_init_data
    (*sk).sk_timer.function = x25_heartbeat_expiry;
}

unsafe fn x25_start_heartbeat(sk: *mut sock) {
    sk_reset_timer(sk, &mut (*sk).sk_timer, jiffies + 5 * HZ);
}

unsafe fn x25_stop_heartbeat(sk: *mut sock) {
    sk_stop_timer(sk, &mut (*sk).sk_timer);
}

unsafe fn x25_start_t2timer(sk: *mut sock) {
    let x25 = x25_sk(sk);

    sk_reset_timer(sk, &mut (*x25).timer, jiffies + (*x25).t2);
}

unsafe fn x25_start_t21timer(sk: *mut sock) {
    let x25 = x25_sk(sk);

    sk_reset_timer(sk, &mut (*x25).timer, jiffies + (*x25).t21);
}

unsafe fn x25_start_t22timer(sk: *mut sock) {
    let x25 = x25_sk(sk);

    sk_reset_timer(sk, &mut (*x25).timer, jiffies + (*x25).t22);
}

unsafe fn x25_start_t23timer(sk: *mut sock) {
    let x25 = x25_sk(sk);

    sk_reset_timer(sk, &mut (*x25).timer, jiffies + (*x25).t23);
}

unsafe fn x25_stop_timer(sk: *mut sock) {
    sk_stop_timer(sk, &mut (*x25_sk(sk)).timer);
}

unsafe fn x25_display_timer(sk: *mut sock) -> c_ulong {
    let x25 = x25_sk(sk);

    if !timer_pending(&(*x25).timer) {
        return 0;
    }

    (*x25).timer.expires - jiffies
}

unsafe extern "C" fn x25_heartbeat_expiry(t: *mut timer_list) {
    let sk = timer_container_of::<sock>(t, sk_timer);

    bh_lock_sock(sk);
    if sock_owned_by_user(sk) {
        goto_restart_heartbeat(sk);
        return;
    }

    match (*x25_sk(sk)).state {
        X25_STATE_0 => {
            // Magic here: If we listen() and a new link dies before it is
            // accepted() it isn't 'dead' so doesn't get removed.
            if sock_flag(sk, SOCK_DESTROY)
                || ((*sk).sk_state == TCP_LISTEN && sock_flag(sk, SOCK_DEAD))
            {
                bh_unlock_sock(sk);
                x25_destroy_socket_from_timer(sk);
                sock_put(sk);
                return;
            }
        }
        X25_STATE_3 => {
            // Check for the state of the receive buffer.
            x25_check_rbuf(sk);
        }
        _ => {}
    }

    goto_restart_heartbeat(sk);
}

unsafe fn goto_restart_heartbeat(sk: *mut sock) {
    // Do not rearm once __x25_destroy_socket() has unlinked the socket:
    // it is past its cancel point and owns the teardown from there on.
    if sk_hashed(sk) {
        x25_start_heartbeat(sk);
    }
    bh_unlock_sock(sk);
    sock_put(sk);
}

// Timer has expired, it may have been T2, T21, T22, or T23. We can tell
// by the state machine state.
unsafe fn x25_do_timer_expiry(sk: *mut sock) {
    let x25 = x25_sk(sk);

    match (*x25).state {
        X25_STATE_3 => {
            // T2
            if (*x25).condition & X25_COND_ACK_PENDING != 0 {
                (*x25).condition &= !X25_COND_ACK_PENDING;
                x25_enquiry_response(sk);
            }
        }
        X25_STATE_1 | X25_STATE_4 => {
            // T21, T22
            x25_write_internal(sk, X25_CLEAR_REQUEST);
            (*x25).state = X25_STATE_2;
            x25_start_t23timer(sk);
        }
        X25_STATE_2 => {
            // T23
            x25_disconnect(sk, ETIMEDOUT, 0, 0);
        }
        _ => {}
    }
}

unsafe extern "C" fn x25_timer_expiry(t: *mut timer_list) {
    let x25 = timer_container_of::<x25_sock>(t, timer);
    let sk = &mut (*x25).sk as *mut sock;

    bh_lock_sock(sk);
    if sock_owned_by_user(sk) {
        // can currently only occur in state 3
        if (*x25_sk(sk)).state == X25_STATE_3 {
            x25_start_t2timer(sk);
        }
    } else {
        x25_do_timer_expiry(sk);
    }
    bh_unlock_sock(sk);
    sock_put(sk);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
