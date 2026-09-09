// SPDX-License-Identifier: GPL-2.0-or-later
/* Amanda extension for TCP NAT alteration.
 * (C) 2002 by Brian J. Murrell <netfilter@interlinx.bc.ca>
 * based on a copy of HW's ip_nat_irc.c as well as other modules
 * (C) 2006-2012 Patrick McHardy <kaber@trash.net>
 */

// Dependencies supplied by the surrounding kernel translation unit.

const NAT_HELPER_NAME: &[u8] = b"amanda\0";

// MODULE_AUTHOR("Brian J. Murrell <netfilter@interlinx.bc.ca>");
// MODULE_DESCRIPTION("Amanda NAT helper");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS_NF_NAT_HELPER(NAT_HELPER_NAME);

static mut NAT_HELPER_AMANDA: nf_conntrack_nat_helper =
    NF_CT_NAT_HELPER_INIT(NAT_HELPER_NAME);

unsafe extern "C" fn help(
    skb: *mut sk_buff,
    ct: *mut nf_conn,
    ctinfo: ip_conntrack_info,
    protoff: u32,
    matchoff: u32,
    matchlen: u32,
    exp: *mut nf_conntrack_expect,
) -> u32 {
    let mut buffer: [u8; core::mem::size_of::<[u8; 6]>()] = [0; core::mem::size_of::<[u8; 6]>()];
    let mut port: u16;

    /* Connection comes from client. */
    (*exp).saved_proto.tcp.port = (*exp).tuple.dst.u.tcp.port;
    (*exp).dir = IP_CT_DIR_ORIGINAL;

    /* When you see the packet, we need to NAT it the same as the
     * this one (ie. same IP: it will be TCP and master is UDP). */
    (*exp).expectfn = Some(nf_nat_follow_master);

    /* Try to get same port: if not, try to change it. */
    port = nf_nat_exp_find_port(exp, ntohs((*exp).saved_proto.tcp.port));
    if port == 0 {
        nf_ct_helper_log(skb, ct, b"all ports in use\0".as_ptr() as *const i8);
        return NF_DROP;
    }

    snprintf(
        buffer.as_mut_ptr() as *mut i8,
        buffer.len(),
        b"%u\0".as_ptr() as *const i8,
        port as core::ffi::c_uint,
    );
    if !nf_nat_mangle_udp_packet(
        skb,
        ct,
        ctinfo,
        protoff,
        matchoff,
        matchlen,
        buffer.as_ptr() as *const i8,
        strlen(buffer.as_ptr() as *const i8),
    ) {
        nf_ct_helper_log(skb, ct, b"cannot mangle packet\0".as_ptr() as *const i8);
        nf_ct_unexpect_related(exp);
        return NF_DROP;
    }
    NF_ACCEPT
}

unsafe extern "C" fn nf_nat_amanda_fini() {
    nf_nat_helper_unregister(&raw mut NAT_HELPER_AMANDA);
    RCU_INIT_POINTER(nf_nat_amanda_hook, None);
    synchronize_rcu();
}

unsafe extern "C" fn nf_nat_amanda_init() -> i32 {
    BUG_ON(nf_nat_amanda_hook != None);
    nf_nat_helper_register(&raw mut NAT_HELPER_AMANDA);
    RCU_INIT_POINTER(nf_nat_amanda_hook, Some(help));
    0
}

// module_init(nf_nat_amanda_init);
// module_exit(nf_nat_amanda_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
