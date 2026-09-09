// SPDX-License-Identifier: GPL-2.0-only
/* SANE connection tracking helper
 * (SANE = Scanner Access Now Easy)
 * For documentation about the SANE network protocol see
 * http://www.sane-project.org/html/doc015.html
 */

/* Copyright (C) 2007 Red Hat, Inc.
 * Author: Michal Schmidt <mschmidt@redhat.com>
 * Based on the FTP conntrack helper (net/netfilter/nf_conntrack_ftp.c):
 *  (C) 1999-2001 Paul `Rusty' Russell
 *  (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 *  (C) 2003,2004 USAGI/WIDE Project <http://www.linux-ipv6.org>
 *  (C) 2003 Yasuyuki Kozakai @USAGI <yasuyuki.kozakai@toshiba.co.jp>
 */

// pr_fmt(fmt) = KBUILD_MODNAME ": " fmt
// C dependencies: linux/module.h, linux/moduleparam.h, linux/netfilter.h,
// linux/slab.h, linux/in.h, linux/tcp.h, net/netfilter/nf_conntrack.h,
// net/netfilter/nf_conntrack_helper.h, net/netfilter/nf_conntrack_expect.h,
// linux/netfilter/nf_conntrack_sane.h

const HELPER_NAME: &str = "sane";

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Michal Schmidt <mschmidt@redhat.com>");
// MODULE_DESCRIPTION("SANE connection tracking helper");
// MODULE_ALIAS_NFCT_HELPER(HELPER_NAME);

#[repr(C)]
pub struct sane_request {
    pub RPC_code: __be32,
    // SANE_NET_START: RPC code
    pub handle: __be32,
}
pub const SANE_NET_START: u32 = 7;

#[repr(C)]
pub struct sane_reply_net_start {
    pub status: __be32,
    pub zero: __be16,
    pub port: __be16,
    // other fields aren't interesting for conntrack
}
pub const SANE_STATUS_SUCCESS: u32 = 0;

pub unsafe fn help(
    skb: *mut sk_buff,
    protoff: c_uint,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
) -> c_int {
    let mut dataoff: c_uint;
    let mut datalen: c_uint;
    let th: *const tcphdr;
    let mut _tcph: tcphdr = core::mem::zeroed();
    let mut ret: c_int = NF_ACCEPT;
    let dir: c_int = CTINFO2DIR(ctinfo);
    let ct_sane_info: *mut nf_ct_sane_master = nfct_help_data(ct);
    let mut exp: *mut nf_conntrack_expect;
    let mut tuple: *mut nf_conntrack_tuple;
    let mut reply: *mut sane_reply_net_start;
    let mut buf: sane_buf = core::mem::zeroed();

    if ct_sane_info.is_null() { return NF_DROP; }

    // Until there's been traffic both ways, don't look in packets.
    if ctinfo != IP_CT_ESTABLISHED && ctinfo != IP_CT_ESTABLISHED_REPLY { return NF_ACCEPT; }

    // Not a full tcp header?
    th = skb_header_pointer(skb, protoff, core::mem::size_of::<tcphdr>(), &mut _tcph as *mut _ as *mut core::ffi::c_void) as *const tcphdr;
    if th.is_null() { return NF_ACCEPT; }

    // No data?
    dataoff = protoff + ((*th).doff as c_uint) * 4;
    if dataoff >= (*skb).len { return NF_ACCEPT; }
    datalen = (*skb).len - dataoff;

    if dir == IP_CT_DIR_ORIGINAL {
        let req: *const sane_request;
        if datalen != core::mem::size_of::<sane_request>() as c_uint { return NF_ACCEPT; }
        req = skb_header_pointer(skb, dataoff, datalen, &mut buf.req as *mut _ as *mut core::ffi::c_void) as *const sane_request;
        if req.is_null() { return NF_ACCEPT; }
        if (*req).RPC_code != htonl(SANE_NET_START) {
            // Not an interesting command
            WRITE_ONCE((*ct_sane_info).state, SANE_STATE_NORMAL);
            return NF_ACCEPT;
        }
        // We're interested in the next reply
        WRITE_ONCE((*ct_sane_info).state, SANE_STATE_START_REQUESTED);
        return NF_ACCEPT;
    }

    // IP_CT_DIR_REPLY
    // Is it a reply to an uninteresting command?
    if READ_ONCE((*ct_sane_info).state) != SANE_STATE_START_REQUESTED { return NF_ACCEPT; }
    // It's a reply to SANE_NET_START.
    WRITE_ONCE((*ct_sane_info).state, SANE_STATE_NORMAL);
    if datalen < core::mem::size_of::<sane_reply_net_start>() as c_uint {
        pr_debug!("NET_START reply too short\n");
        return NF_ACCEPT;
    }
    datalen = core::mem::size_of::<sane_reply_net_start>() as c_uint;
    reply = skb_header_pointer(skb, dataoff, datalen, &mut buf.repl as *mut _ as *mut core::ffi::c_void) as *mut sane_reply_net_start;
    if reply.is_null() { return NF_ACCEPT; }
    if (*reply).status != htonl(SANE_STATUS_SUCCESS) {
        // saned refused the command
        pr_debug!("unsuccessful SANE_STATUS = %u\n", ntohl((*reply).status));
        return NF_ACCEPT;
    }
    // Invalid saned reply? Ignore it.
    if (*reply).zero != 0 { return NF_ACCEPT; }
    exp = nf_ct_expect_alloc(ct);
    if exp.is_null() {
        nf_ct_helper_log(skb, ct, "cannot alloc expectation");
        return NF_DROP;
    }
    tuple = &mut (*ct).tuplehash[IP_CT_DIR_ORIGINAL as usize].tuple;
    nf_ct_expect_init(exp, NF_CT_EXPECT_CLASS_DEFAULT, nf_ct_l3num(ct), &(*tuple).src.u3, &(*tuple).dst.u3, IPPROTO_TCP, core::ptr::null(), &(*reply).port);
    pr_debug!("expect: ");
    nf_ct_dump_tuple(&(*exp).tuple);
    if nf_ct_expect_related(exp, 0) != 0 {
        nf_ct_helper_log(skb, ct, "cannot add expectation");
        ret = NF_DROP;
    }
    nf_ct_expect_put(exp);
    ret
}

#[repr(C)]
pub union sane_buf { pub req: sane_request, pub repl: sane_reply_net_start }

static mut sane: nf_conntrack_helper = core::mem::zeroed();
static mut sane_ptr: *mut nf_conntrack_helper = core::ptr::null_mut();

#[repr(C)]
pub struct nf_conntrack_expect_policy { pub max_expected: c_uint, pub timeout: c_uint }
static sane_exp_policy: nf_conntrack_expect_policy = nf_conntrack_expect_policy { max_expected: 1, timeout: 5 * 60 };

pub unsafe fn nf_conntrack_sane_fini() { nf_conntrack_helper_unregister(sane_ptr); }

pub unsafe fn nf_conntrack_sane_init() -> c_int {
    let mut ret: c_int = 0;
    NF_CT_HELPER_BUILD_BUG_ON(core::mem::size_of::<nf_ct_sane_master>());
    nf_ct_helper_init(&mut sane, NFPROTO_UNSPEC, IPPROTO_TCP, HELPER_NAME, &sane_exp_policy, 0, help, core::ptr::null_mut(), THIS_MODULE);
    ret = nf_conntrack_helper_register(&mut sane, &mut sane_ptr);
    if ret < 0 { pr_err!("failed to register helpers\n"); return ret; }
    0
}

// module_init(nf_conntrack_sane_init);
// module_exit(nf_conntrack_sane_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
