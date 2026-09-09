// SPDX-License-Identifier: GPL-2.0-only
/*
 * This is the 1999 rewrite of IP Firewalling, aiming for kernel 2.3.x.
 *
 * Copyright (C) 1999 Paul `Rusty' Russell & Michael J. Neuling
 * Copyright (C) 2000-2004 Netfilter Core Team <coreteam@netfilter.org>
 */
// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    static THIS_MODULE: *mut module;
    fn ipt_do_table(priv_: *mut core::ffi::c_void, skb: *mut sk_buff,
                    state: *const nf_hook_state) -> u32;
    fn ip_route_me_harder(net: *mut net, sk: *mut sock, skb: *mut sk_buff,
                          addr_type: i32) -> i32;
    fn ipt_alloc_initial_table(table: *const xt_table) -> *mut ipt_replace;
    fn ipt_register_table(net: *mut net, table: *const xt_table,
                          repl: *mut ipt_replace,
                          ops: *mut nf_hook_ops) -> i32;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn xt_unregister_table_pre_exit(net: *mut net, family: u16,
                                    name: *const i8);
    fn ipt_unregister_table_exit(net: *mut net, name: *const i8);
    fn xt_hook_ops_alloc(table: *const xt_table, hook: nf_hookfn)
        -> *mut nf_hook_ops;
    fn register_pernet_subsys(ops: *mut pernet_operations) -> i32;
    fn unregister_pernet_subsys(ops: *mut pernet_operations);
    fn xt_register_template(table: *const xt_table,
                            init: table_initfn) -> i32;
    fn xt_unregister_template(table: *const xt_table);
}

#[repr(C)]
struct module;
#[repr(C)]
struct sk_buff { mark: u32, _opaque: [u8; 0] }
#[repr(C)]
struct nf_hook_state { hook: u32, net: *mut net, sk: *mut sock, _opaque: [u8; 0] }
#[repr(C)]
struct net;
#[repr(C)]
struct sock;
#[repr(C)]
struct iphdr { saddr: u32, daddr: u32, tos: u8, _opaque: [u8; 0] }
#[repr(C)]
struct xt_table {
    name: *const i8,
    valid_hooks: u32,
    me: *mut module,
    af: u16,
    priority: i32,
}
#[repr(C)]
struct ipt_replace;
#[repr(C)]
struct nf_hook_ops;
#[repr(C)]
struct pernet_operations {
    pre_exit: Option<unsafe extern "C" fn(*mut net)>,
    exit: Option<unsafe extern "C" fn(*mut net)>,
}

type nf_hookfn = unsafe extern "C" fn(*mut core::ffi::c_void,
                                        *mut sk_buff,
                                        *const nf_hook_state) -> u32;
type table_initfn = unsafe extern "C" fn(*mut net) -> i32;

const NF_INET_PRE_ROUTING: u32 = 0;
const NF_INET_LOCAL_IN: u32 = 1;
const NF_INET_FORWARD: u32 = 2;
const NF_INET_LOCAL_OUT: u32 = 3;
const NF_INET_POST_ROUTING: u32 = 4;
const NFPROTO_IPV4: u16 = 2;
const NF_IP_PRI_MANGLE: i32 = -150;
const RTN_UNSPEC: i32 = 0;
const NF_DROP: u32 = 0;
const NF_STOLEN: u32 = 2;
const NF_VERDICT_MASK: u32 = 0xff;

#[inline]
const fn nf_drop_err(err: i32) -> u32 {
    NF_DROP | ((-err) as u32) << 16
}

const MANGLE_VALID_HOOKS: u32 = (1 << NF_INET_PRE_ROUTING)
    | (1 << NF_INET_LOCAL_IN)
    | (1 << NF_INET_FORWARD)
    | (1 << NF_INET_LOCAL_OUT)
    | (1 << NF_INET_POST_ROUTING);

static mut packet_mangler: xt_table = xt_table {
    name: b"mangle\0".as_ptr() as *const i8,
    valid_hooks: MANGLE_VALID_HOOKS,
    me: core::ptr::null_mut(),
    af: NFPROTO_IPV4,
    priority: NF_IP_PRI_MANGLE,
};

unsafe fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr {
    skb.add(1) as *mut iphdr
}

unsafe extern "C" fn ipt_mangle_out(priv_: *mut core::ffi::c_void,
                                     skb: *mut sk_buff,
                                     state: *const nf_hook_state) -> u32 {
    let mut ret: u32;
    let verdict: u32;
    let iph: *mut iphdr;
    let saddr: u32;
    let daddr: u32;
    let mark: u32;
    let mut err: i32;
    let tos: u8;

    mark = (*skb).mark;
    iph = ip_hdr(skb);
    saddr = (*iph).saddr;
    daddr = (*iph).daddr;
    tos = (*iph).tos;

    ret = ipt_do_table(priv_, skb, state);
    verdict = ret & NF_VERDICT_MASK;
    if verdict != NF_DROP && verdict != NF_STOLEN {
        let iph = ip_hdr(skb);
        if (*iph).saddr != saddr || (*iph).daddr != daddr
            || (*skb).mark != mark || (*iph).tos != tos
        {
            err = ip_route_me_harder((*state).net, (*state).sk, skb, RTN_UNSPEC);
            if err < 0 {
                ret = nf_drop_err(err);
            }
        }
    }
    ret
}

unsafe extern "C" fn iptable_mangle_hook(priv_: *mut core::ffi::c_void,
                                          skb: *mut sk_buff,
                                          state: *const nf_hook_state) -> u32 {
    if (*state).hook == NF_INET_LOCAL_OUT {
        return ipt_mangle_out(priv_, skb, state);
    }
    ipt_do_table(priv_, skb, state)
}

static mut mangle_ops: *mut nf_hook_ops = core::ptr::null_mut();

unsafe extern "C" fn iptable_mangle_table_init(net_: *mut net) -> i32 {
    let repl = ipt_alloc_initial_table(&packet_mangler);
    if repl.is_null() {
        return -12;
    }
    let ret = ipt_register_table(net_, &packet_mangler, repl, mangle_ops);
    kfree(repl as *mut core::ffi::c_void);
    ret
}

unsafe extern "C" fn iptable_mangle_net_pre_exit(net_: *mut net) {
    xt_unregister_table_pre_exit(net_, NFPROTO_IPV4, b"mangle\0".as_ptr() as *const i8);
}

unsafe extern "C" fn iptable_mangle_net_exit(net_: *mut net) {
    ipt_unregister_table_exit(net_, b"mangle\0".as_ptr() as *const i8);
}

static mut iptable_mangle_net_ops: pernet_operations = pernet_operations {
    pre_exit: Some(iptable_mangle_net_pre_exit),
    exit: Some(iptable_mangle_net_exit),
};

unsafe extern "C" fn iptable_mangle_init() -> i32 {
    mangle_ops = xt_hook_ops_alloc(&packet_mangler, iptable_mangle_hook);
    if mangle_ops.is_null() {
        return -1;
    }
    let mut ret = register_pernet_subsys(&mut iptable_mangle_net_ops);
    if ret < 0 {
        kfree(mangle_ops as *mut core::ffi::c_void);
        return ret;
    }
    ret = xt_register_template(&packet_mangler, iptable_mangle_table_init);
    if ret < 0 {
        unregister_pernet_subsys(&mut iptable_mangle_net_ops);
        kfree(mangle_ops as *mut core::ffi::c_void);
    }
    ret
}

unsafe extern "C" fn iptable_mangle_fini() {
    xt_unregister_template(&packet_mangler);
    unregister_pernet_subsys(&mut iptable_mangle_net_ops);
    kfree(mangle_ops as *mut core::ffi::c_void);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
