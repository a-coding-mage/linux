/*
 * net/tipc/diag.c: TIPC socket diag
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

// C includes translated as dependencies supplied by the surrounding kernel bindings.

unsafe fn __tipc_diag_gen_cookie(sk: *mut sock) -> u64 {
    let mut res: [u32; 2] = [0; 2];

    sock_diag_save_cookie(sk, res.as_mut_ptr());
    *(res.as_ptr() as *const u64)
}

unsafe fn __tipc_add_sock_diag(
    skb: *mut sk_buff,
    cb: *mut netlink_callback,
    tsk: *mut tipc_sock,
) -> i32 {
    let req = nlmsg_data((*cb).nlh) as *mut tipc_sock_diag_req;
    let mut nlh: *mut nlmsghdr;
    let err: i32;

    nlh = nlmsg_put_answer(skb, cb, SOCK_DIAG_BY_FAMILY, 0, NLM_F_MULTI);
    if nlh.is_null() {
        return -EMSGSIZE;
    }

    err = tipc_sk_fill_sock_diag(
        skb,
        cb,
        tsk,
        (*req).tidiag_states,
        __tipc_diag_gen_cookie,
    );
    if err != 0 {
        return err;
    }

    nlmsg_end(skb, nlh);
    0
}

unsafe fn tipc_diag_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    tipc_nl_sk_walk(skb, cb, __tipc_add_sock_diag)
}

unsafe fn tipc_sock_diag_handler_dump(
    skb: *mut sk_buff,
    h: *mut nlmsghdr,
) -> i32 {
    let hdrlen: i32 = core::mem::size_of::<tipc_sock_diag_req>() as i32;
    let net: *mut net = sock_net((*skb).sk);

    if nlmsg_len(h) < hdrlen {
        return -EINVAL;
    }

    if (*h).nlmsg_flags & NLM_F_DUMP != 0 {
        let c = netlink_dump_control {
            start: Some(tipc_dump_start),
            dump: Some(tipc_diag_dump),
            done: Some(tipc_dump_done),
        };
        netlink_dump_start((*net).diag_nlsk, skb, h, &c);
        return 0;
    }
    -EOPNOTSUPP
}

static tipc_sock_diag_handler: sock_diag_handler = sock_diag_handler {
    owner: THIS_MODULE,
    family: AF_TIPC,
    dump: Some(tipc_sock_diag_handler_dump),
};

unsafe fn tipc_diag_init() -> i32 {
    sock_diag_register(&tipc_sock_diag_handler)
}

unsafe fn tipc_diag_exit() {
    sock_diag_unregister(&tipc_sock_diag_handler);
}

// module_init(tipc_diag_init);
// module_exit(tipc_diag_exit);
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("TIPC socket monitoring via SOCK_DIAG");
// MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG, AF_TIPC);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
