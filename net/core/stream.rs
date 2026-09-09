// SPDX-License-Identifier: GPL-2.0
/*
 *     SUCS NET3:
 *
 *     Generic stream handling routines. These are generic for most
 *     protocols. Even IP. Tonight 8-).
 *     This is used because TCP, LLC (others too) layer all have mostly
 *     identical sendmsg() and recvmsg() code.
 *     So we (will) share it here.
 *
 *     Authors:        Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 *                     (from old tcp.c code)
 *                     Alan Cox <alan@lxorguk.ukuu.org.uk> (Borrowed comments 8-))
 */

/// sk_stream_write_space - stream socket write_space callback.
/// @sk: pointer to the socket structure
///
/// This function is invoked when there's space available in the socket's
/// send buffer for writing. It first checks if the socket is writable,
/// clears the SOCK_NOSPACE flag indicating that memory for writing
/// is now available, wakes up any processes waiting for write operations
/// and sends asynchronous notifications if needed.
pub unsafe fn sk_stream_write_space(sk: *mut sock) {
    let sock = (*sk).sk_socket;
    let mut wq: *mut socket_wq;

    if __sk_stream_is_writeable(sk, 1) && !sock.is_null() {
        clear_bit(SOCK_NOSPACE, &mut (*sock).flags);

        rcu_read_lock();
        wq = rcu_dereference((*sk).sk_wq);
        if skwq_has_sleeper(wq) {
            wake_up_interruptible_poll(&mut (*wq).wait,
                EPOLLOUT | EPOLLWRNORM | EPOLLWRBAND);
        }
        if !wq.is_null() && !(*wq).fasync_list.is_null() &&
            ((*sk).sk_shutdown & SEND_SHUTDOWN) == 0
        {
            sock_wake_async(wq, SOCK_WAKE_SPACE, POLL_OUT);
        }
        rcu_read_unlock();
    }
}

/// sk_stream_wait_connect - Wait for a socket to get into the connected state
/// @sk: sock to wait on
/// @timeo_p: for how long to wait
///
/// Must be called with the socket locked.
pub unsafe fn sk_stream_wait_connect(sk: *mut sock, timeo_p: *mut c_long) -> c_int {
    let mut wait = DEFINE_WAIT_FUNC!(woken_wake_function);
    let tsk = current;
    let mut done: c_int;

    loop {
        let err = sock_error(sk);
        if err != 0 { return err; }
        if ((1i32 << (*sk).sk_state) & !(TCPF_SYN_SENT | TCPF_SYN_RECV)) != 0 {
            return -EPIPE;
        }
        if *timeo_p == 0 { return -EAGAIN; }
        if signal_pending(tsk) { return sock_intr_errno(*timeo_p); }

        add_wait_queue(sk_sleep(sk), &mut wait);
        (*sk).sk_write_pending += 1;
        done = sk_wait_event(sk, timeo_p,
            READ_ONCE((*sk).sk_err) == 0 &&
            ((1i32 << READ_ONCE((*sk).sk_state)) &
             !(TCPF_ESTABLISHED | TCPF_CLOSE_WAIT)) == 0,
            &mut wait);
        remove_wait_queue(sk_sleep(sk), &mut wait);
        (*sk).sk_write_pending -= 1;
        if done != 0 { break; }
    }
    if done < 0 { done } else { 0 }
}

/// sk_stream_closing - Return 1 if we still have things to send in our buffers.
/// @sk: socket to verify
unsafe fn sk_stream_closing(sk: *const sock) -> c_int {
    (1i32 << READ_ONCE((*sk).sk_state)) &
        (TCPF_FIN_WAIT1 | TCPF_CLOSING | TCPF_LAST_ACK)
}

pub unsafe fn sk_stream_wait_close(sk: *mut sock, mut timeout: c_long) {
    if timeout != 0 {
        let mut wait = DEFINE_WAIT_FUNC!(woken_wake_function);
        add_wait_queue(sk_sleep(sk), &mut wait);
        loop {
            if sk_wait_event(sk, &mut timeout, sk_stream_closing(sk) == 0, &mut wait) != 0 {
                break;
            }
            if signal_pending(current) || timeout == 0 { break; }
        }
        remove_wait_queue(sk_sleep(sk), &mut wait);
    }
}

/// sk_stream_wait_memory - Wait for more memory for a socket
/// @sk: socket to wait for memory
/// @timeo_p: for how long
pub unsafe fn sk_stream_wait_memory(sk: *mut sock, timeo_p: *mut c_long) -> c_int {
    let mut ret: c_int;
    let mut err: c_int = 0;
    let mut vm_wait: c_long = 0;
    let mut current_timeo = *timeo_p;
    let mut wait = DEFINE_WAIT_FUNC!(woken_wake_function);

    if sk_stream_memory_free(sk) {
        current_timeo = vm_wait = get_random_u32_below(HZ / 5) as c_long + 2;
    }
    add_wait_queue(sk_sleep(sk), &mut wait);

    loop {
        sk_set_bit(SOCKWQ_ASYNC_NOSPACE, sk);
        if (*sk).sk_err != 0 || ((*sk).sk_shutdown & SEND_SHUTDOWN) != 0 { err = -EPIPE; break; }
        if *timeo_p == 0 { set_bit(SOCK_NOSPACE, &mut (*(*sk).sk_socket).flags); err = -EAGAIN; break; }
        if signal_pending(current) { err = sock_intr_errno(*timeo_p); break; }
        sk_clear_bit(SOCKWQ_ASYNC_NOSPACE, sk);
        if sk_stream_memory_free(sk) && vm_wait == 0 { break; }

        set_bit(SOCK_NOSPACE, &mut (*(*sk).sk_socket).flags);
        (*sk).sk_write_pending += 1;
        ret = sk_wait_event(sk, &mut current_timeo,
            READ_ONCE((*sk).sk_err) != 0 ||
            (READ_ONCE((*sk).sk_shutdown) & SEND_SHUTDOWN) != 0 ||
            (sk_stream_memory_free(sk) && vm_wait == 0), &mut wait);
        (*sk).sk_write_pending -= 1;
        if ret < 0 { err = -EPIPE; break; }
        if vm_wait != 0 {
            vm_wait -= current_timeo;
            current_timeo = *timeo_p;
            if current_timeo != MAX_SCHEDULE_TIMEOUT {
                current_timeo -= vm_wait;
                if current_timeo < 0 { current_timeo = 0; }
            }
            vm_wait = 0;
        }
        *timeo_p = current_timeo;
    }
    if !sock_flag(sk, SOCK_DEAD) { remove_wait_queue(sk_sleep(sk), &mut wait); }
    err
}

pub unsafe fn sk_stream_error(sk: *mut sock, flags: c_int, mut err: c_int) -> c_int {
    if err == -EPIPE { err = { let e = sock_error(sk); if e != 0 { e } else { -EPIPE } }; }
    if err == -EPIPE && (flags & MSG_NOSIGNAL) == 0 { send_sig(SIGPIPE, current, 0); }
    err
}

pub unsafe fn sk_stream_kill_queues(sk: *mut sock) {
    // First the read buffer.
    __skb_queue_purge(&mut (*sk).sk_receive_queue);
    // Next, the error queue. We need to use queue lock, because other threads might add packets.
    skb_queue_purge(&mut (*sk).sk_error_queue);
    // Next, the write queue.
    WARN_ON_ONCE(!skb_queue_empty(&(*sk).sk_write_queue));
    // Account for returned memory.
    sk_mem_reclaim_final(sk);
    WARN_ON_ONCE((*sk).sk_wmem_queued != 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
