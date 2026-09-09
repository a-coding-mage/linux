/*
 * net/tipc/trace.c: TIPC tracepoints code
 *
 * Copyright (c) 2018, Ericsson AB
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the names of the copyright holders nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed under the terms of the
 * GNU General Public License ("GPL") version 2 as published by the Free
 * Software Foundation.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "ASIS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

// CREATE_TRACE_POINTS; dependency declarations from trace.h are supplied externally.

pub static mut SYSCTL_TIPC_SK_FILTER: [core::ffi::c_ulong; 5] = [0; 5];

/// tipc_skb_dump - dump TIPC skb data
/// @skb: skb to be dumped
/// @more: dump more?
///        - false: dump only tipc msg data
///        - true: dump kernel-related skb data and tipc cb[] array as well
/// @buf: returned buffer of dump data in format
pub unsafe fn tipc_skb_dump(skb: *mut sk_buff, more: bool, buf: *mut core::ffi::c_char) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let sz: usize = if more { SKB_LMAX } else { SKB_LMIN };
    let hdr: *mut tipc_msg;
    let skbcb: *mut tipc_skb_cb;

    if skb.is_null() {
        i += scnprintf(buf, sz, c"msg: (null)\n".as_ptr());
        return i;
    }

    hdr = buf_msg(skb);
    skbcb = TIPC_SKB_CB(skb);

    i += scnprintf(buf, sz, c"msg: %u".as_ptr(), msg_user(hdr));
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_type(hdr));
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_hdr_sz(hdr));
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_data_sz(hdr));
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %x".as_ptr(), msg_orignode(hdr));
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %x".as_ptr(), msg_destnode(hdr));
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_seqno(hdr));
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_ack(hdr));
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_bcast_ack(hdr));
    match msg_user(hdr) {
        LINK_PROTOCOL => {
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" %c".as_ptr(), msg_net_plane(hdr));
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_probe(hdr));
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_peer_stopping(hdr));
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_session(hdr));
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_next_sent(hdr));
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_seq_gap(hdr));
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_bc_snd_nxt(hdr));
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_bc_gap(hdr));
        }
        TIPC_LOW_IMPORTANCE | TIPC_MEDIUM_IMPORTANCE | TIPC_HIGH_IMPORTANCE |
        TIPC_CRITICAL_IMPORTANCE | CONN_MANAGER | SOCK_WAKEUP => {
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" | %u".as_ptr(), msg_origport(hdr));
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_destport(hdr));
            match msg_type(hdr) {
                TIPC_NAMED_MSG => {
                    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_nametype(hdr));
                    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_nameinst(hdr));
                }
                TIPC_MCAST_MSG => {
                    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_nametype(hdr));
                    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_namelower(hdr));
                    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_nameupper(hdr));
                }
                _ => {}
            }
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" | %u".as_ptr(), msg_src_droppable(hdr));
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_dest_droppable(hdr));
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_errcode(hdr));
            i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), msg_reroute_cnt(hdr));
        }
        _ => {}
    }
    i += scnprintf(buf.add(i as usize), sz - i as usize, c"\n".as_ptr());
    if !more { return i; }

    i += scnprintf(buf.add(i as usize), sz - i as usize, c"skb: %s".as_ptr(), if (*skb).dev.is_null() { c"n/a".as_ptr() } else { (*(*skb).dev).name.as_ptr() });
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), (*skb).len);
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), (*skb).data_len);
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), (*skb).hdr_len);
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), (*skb).truesize);
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), skb_cloned(skb));
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %p".as_ptr(), (*skb).sk);
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), (*skb).shinfo.nr_frags);
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %llx".as_ptr(), ktime_to_ms(skb_get_ktime(skb)));
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %llx\n".as_ptr(), ktime_to_ms(skb_hwtstamps(skb).hwtstamp));

    i += scnprintf(buf.add(i as usize), sz - i as usize, c"cb[]: %u".as_ptr(), (*skbcb).bytes_read);
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), (*skbcb).orig_member);
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), jiffies_to_msecs((*skbcb).nxt_retr));
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), (*skbcb).validated);
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u".as_ptr(), (*skbcb).chain_imp);
    i += scnprintf(buf.add(i as usize), sz - i as usize, c" %u\n".as_ptr(), (*skbcb).ackers);
    i
}

/// tipc_list_dump - dump TIPC skb list/queue
pub unsafe fn tipc_list_dump(list: *mut sk_buff_head, more: bool, buf: *mut core::ffi::c_char) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let sz = if more { LIST_LMAX } else { LIST_LMIN };
    if list.is_null() { i += scnprintf(buf, sz, c"(null)\n".as_ptr()); return i; }
    let len = skb_queue_len(list);
    i += scnprintf(buf, sz, c"len = %d\n".as_ptr(), len);
    if len == 0 { return i; }
    if !more {
        let hskb = skb_peek(list);
        i += scnprintf(buf.add(i as usize), sz - i as usize, c"  head ".as_ptr());
        i += tipc_skb_dump(hskb, false, buf.add(i as usize));
        if len > 1 { let tskb = skb_peek_tail(list); i += scnprintf(buf.add(i as usize), sz - i as usize, c"  tail ".as_ptr()); i += tipc_skb_dump(tskb, false, buf.add(i as usize)); }
    } else {
        let mut count = 0;
        let mut skb = skb_peek(list);
        while !skb.is_null() {
            count += 1;
            if count == 6 { i += scnprintf(buf.add(i as usize), sz - i as usize, c"  .\n  .\n".as_ptr()); }
            if count <= 5 || count > len - 5 { i += scnprintf(buf.add(i as usize), sz - i as usize, c"  #%d ".as_ptr(), count); i += tipc_skb_dump(skb, false, buf.add(i as usize)); }
            skb = skb_next(skb);
        }
    }
    i
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
