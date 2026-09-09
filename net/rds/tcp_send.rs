/*
 * Copyright (c) 2006, 2017 Oracle and/or its affiliates. All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Linux kernel and local project dependencies are supplied externally.

pub unsafe fn rds_tcp_xmit_path_prepare(cp: *mut rds_conn_path) {
    let tc = (*cp).cp_transport_data;
    tcp_sock_set_cork((*tc).t_sock.sk, true);
}

pub unsafe fn rds_tcp_xmit_path_complete(cp: *mut rds_conn_path) {
    let tc = (*cp).cp_transport_data;
    tcp_sock_set_cork((*tc).t_sock.sk, false);
}

/* the core send_sem serializes this with other xmit and shutdown */
unsafe fn rds_tcp_sendmsg(sock: *mut socket, data: *mut core::ffi::c_void, len: u32) -> i32 {
    let mut vec = kvec {
        iov_base: data,
        iov_len: len as usize,
    };
    let mut msg = msghdr {
        msg_flags: MSG_DONTWAIT | MSG_NOSIGNAL,
        ..core::mem::zeroed()
    };
    kernel_sendmsg(sock, &mut msg, &mut vec, 1, vec.iov_len)
}

/* the core send_sem serializes this with other xmit and shutdown */
pub unsafe fn rds_tcp_xmit(
    conn: *mut rds_connection,
    rm: *mut rds_message,
    mut hdr_off: u32,
    mut sg: u32,
    mut off: u32,
) -> i32 {
    let cp = (*rm).m_inc.i_conn_path;
    let tc = (*cp).cp_transport_data;
    let mut msg: msghdr = core::mem::zeroed();
    let mut bvec: bio_vec = core::mem::zeroed();
    let mut done: i32 = 0;
    let mut ret: i32 = 0;

    if hdr_off == 0 {
        /* m_ack_seq is set to the sequence number of the last byte of
         * header and data.  see rds_tcp_is_acked(). */
        (*tc).t_last_sent_nxt = rds_tcp_write_seq(tc);
        (*rm).m_ack_seq = (*tc).t_last_sent_nxt
            + core::mem::size_of::<rds_header>() as u64
            + be32_to_cpu((*rm).m_inc.i_hdr.h_len) as u64 - 1;
        smp_mb__before_atomic();
        set_bit(RDS_MSG_HAS_ACK_SEQ, &mut (*rm).m_flags);
        (*tc).t_last_expected_una = (*rm).m_ack_seq + 1;

        if test_bit(RDS_MSG_RETRANSMITTED, &(*rm).m_flags) {
            (*rm).m_inc.i_hdr.h_flags |= RDS_FLAG_RETRANSMITTED;
        }

        rdsdebug!("rm {:?} tcp nxt {} ack_seq {}\n", rm, rds_tcp_write_seq(tc), (*rm).m_ack_seq);
    }

    if hdr_off < core::mem::size_of::<rds_header>() as u32 {
        /* see rds_tcp_write_space() */
        set_bit(SOCK_NOSPACE, &mut (*(*tc).t_sock.sk).sk_socket.flags);
        ret = rds_tcp_sendmsg(
            (*tc).t_sock,
            (&mut (*rm).m_inc.i_hdr as *mut _ as *mut u8).add(hdr_off as usize) as *mut core::ffi::c_void,
            core::mem::size_of_val(&(*rm).m_inc.i_hdr) as u32 - hdr_off,
        );
        if ret < 0 { return ret; }
        done += ret;
        if hdr_off + done as u32 != core::mem::size_of::<rds_header>() as u32 { return done; }
    }

    while sg < (*rm).data.op_nents {
        msg.msg_flags = MSG_SPLICE_PAGES | MSG_DONTWAIT | MSG_NOSIGNAL;
        if sg + 1 < (*rm).data.op_nents { msg.msg_flags |= MSG_MORE; }
        bvec_set_page(&mut bvec, sg_page(&(*rm).data.op_sg.add(sg as usize)),
            (*rm).data.op_sg.add(sg as usize).length - off,
            (*rm).data.op_sg.add(sg as usize).offset + off);
        iov_iter_bvec(&mut msg.msg_iter, ITER_SOURCE, &mut bvec, 1,
            (*rm).data.op_sg.add(sg as usize).length - off);
        ret = sock_sendmsg((*tc).t_sock, &mut msg);
        rdsdebug!("tcp sendpage {:?}:{}:{} ret {}\n", sg_page(&(*rm).data.op_sg.add(sg as usize)),
            (*rm).data.op_sg.add(sg as usize).offset + off,
            (*rm).data.op_sg.add(sg as usize).length - off, ret);
        if ret <= 0 { break; }
        off += ret as u32;
        done += ret;
        if off == (*rm).data.op_sg.add(sg as usize).length { off = 0; sg += 1; }
    }

    if ret <= 0 {
        /* write_space will hit after EAGAIN, all else fatal */
        if ret == -EAGAIN {
            rds_tcp_stats_inc(s_tcp_sndbuf_full);
            ret = 0;
        } else if rds_conn_path_up(cp) {
            pr_warn!("RDS/tcp: send to {:?} on cp [{}]returned {}, disconnecting and reconnecting\n",
                &(*conn).c_faddr, (*cp).cp_index, ret);
            rds_conn_path_drop(cp, false);
        }
    }
    if done == 0 { done = ret; }
    done
}

/*
 * rm->m_ack_seq is set to the tcp sequence number that corresponds to the
 * last byte of the message, including the header.  This means that the
 * entire message has been received if rm->m_ack_seq is "before" the next
 * unacked byte of the TCP sequence space.  We have to do very careful
 * wrapping 32bit comparisons here.
 */
pub unsafe fn rds_tcp_is_acked(rm: *mut rds_message, ack: u64) -> i32 {
    if !test_bit(RDS_MSG_HAS_ACK_SEQ, &(*rm).m_flags) { return 0; }
    (((*rm).m_ack_seq as u32).wrapping_sub(ack as u32) as i32) < 0
        as i32
}

pub unsafe fn rds_tcp_write_space(sk: *mut sock) {
    let write_space: unsafe extern "C" fn(*mut sock);
    let cp: *mut rds_conn_path;
    let tc: *mut rds_tcp_connection;

    read_lock_bh(&mut (*sk).sk_callback_lock);
    cp = (*sk).sk_user_data as *mut rds_conn_path;
    if cp.is_null() {
        write_space = (*sk).sk_write_space;
    } else {
        tc = (*cp).cp_transport_data;
        rdsdebug!("write_space for tc {:?}\n", tc);
        write_space = (*tc).t_orig_write_space;
        rds_tcp_stats_inc(s_tcp_write_space_calls);
        rdsdebug!("tcp una {}\n", rds_tcp_snd_una(tc));
        (*tc).t_last_seen_una = rds_tcp_snd_una(tc);
        rds_send_path_drop_acked(cp, rds_tcp_snd_una(tc), rds_tcp_is_acked);
        rcu_read_lock();
        if ((refcount_read(&(*sk).sk_wmem_alloc) << 1) <= (*sk).sk_sndbuf)
            && !rds_destroy_pending((*cp).cp_conn) {
            queue_delayed_work((*cp).cp_wq, &mut (*cp).cp_send_w, 0);
        }
        rcu_read_unlock();
    }
    read_unlock_bh(&mut (*sk).sk_callback_lock);

    /* See the C implementation for the SOCK_NOSPACE/write_space ordering rationale. */
    write_space(sk);
    if !(*sk).sk_socket.is_null() { set_bit(SOCK_NOSPACE, &mut (*(*sk).sk_socket).flags); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
