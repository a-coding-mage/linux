// SPDX-License-Identifier: GPL-2.0-only
/* FTP extension for TCP NAT alteration. */

/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
 */

// C kernel includes and build-time module machinery are supplied by the
// surrounding kernel translation environment.

pub const NAT_HELPER_NAME: &[u8] = b"ftp\0";
pub const NFPROTO_IPV4: i32 = 2;
pub const NF_ACCEPT: u32 = 1;
pub const NF_DROP: u32 = 0;
pub const INET6_ADDRSTRLEN: usize = 46;

#[repr(C)]
pub union nf_inet_addr {
    pub ip: u32,
    pub ip6: [u32; 4],
}

#[repr(C)]
pub struct nf_conntrack_nat_helper {
    pub name: *const u8,
}

#[repr(C)]
pub struct nf_conntrack_expect {
    pub saved_proto: nf_conntrack_expect_saved_proto,
    pub tuple: nf_conntrack_tuple,
    pub dir: i32,
    pub expectfn: Option<unsafe extern "C" fn(*mut nf_conntrack_expect)>,
}

#[repr(C)]
pub struct nf_conntrack_expect_saved_proto {
    pub tcp: nf_conntrack_tcp,
}

#[repr(C)]
pub struct nf_conntrack_tcp {
    pub port: u16,
}

#[repr(C)]
pub struct nf_conntrack_tuple {
    pub dst: nf_conntrack_tuple_dst,
}

#[repr(C)]
pub struct nf_conntrack_tuple_dst {
    pub u: nf_conntrack_tuple_dst_u,
    pub u3: nf_inet_addr,
}

#[repr(C)]
pub union nf_conntrack_tuple_dst_u {
    pub tcp: nf_conntrack_tcp,
}

#[repr(C)]
pub struct nf_conntrack_tuplehash {
    pub tuple: nf_conntrack_tuple,
}

#[repr(C)]
pub struct nf_conn {
    pub tuplehash: [nf_conntrack_tuplehash; 2],
}

#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct kernel_param;

pub type size_t = usize;
pub type u_int16_t = u16;
pub type nf_ct_ftp_type = i32;
pub type ip_conntrack_info = i32;

pub const NF_CT_FTP_PORT: nf_ct_ftp_type = 0;
pub const NF_CT_FTP_PASV: nf_ct_ftp_type = 1;
pub const NF_CT_FTP_EPRT: nf_ct_ftp_type = 2;
pub const NF_CT_FTP_EPSV: nf_ct_ftp_type = 3;

extern "C" {
    static mut nf_nat_ftp_hook: Option<unsafe extern "C" fn(
        *mut sk_buff, *mut nf_conn, ip_conntrack_info, nf_ct_ftp_type,
        u32, u32, u32, *mut nf_conntrack_expect,
    ) -> u32>;
    fn nf_ct_l3num(ct: *mut nf_conn) -> i32;
    fn nf_nat_exp_find_port(exp: *mut nf_conntrack_expect, port: u16) -> u16;
    fn nf_ct_helper_log(skb: *mut sk_buff, ct: *mut nf_conn, msg: *const u8);
    fn nf_nat_follow_master(exp: *mut nf_conntrack_expect);
    fn nf_nat_mangle_tcp_packet(
        skb: *mut sk_buff, ct: *mut nf_conn, ctinfo: ip_conntrack_info,
        protoff: u32, matchoff: u32, matchlen: u32, buffer: *const u8,
        buflen: u32,
    ) -> bool;
    fn nf_ct_unexpect_related(exp: *mut nf_conntrack_expect);
    fn nf_nat_helper_register(helper: *mut nf_conntrack_nat_helper) -> i32;
    fn nf_nat_helper_unregister(helper: *mut nf_conntrack_nat_helper);
    fn synchronize_rcu();
    fn snprintf(buffer: *mut u8, buflen: usize, fmt: *const u8, ...) -> i32;
    fn ntohs(x: u16) -> u16;
}

static mut nat_helper_ftp: nf_conntrack_nat_helper = nf_conntrack_nat_helper {
    name: NAT_HELPER_NAME.as_ptr(),
};

unsafe fn nf_nat_ftp_fmt_cmd(
    ct: *mut nf_conn, typ: nf_ct_ftp_type, buffer: *mut u8, buflen: size_t,
    addr: *mut nf_inet_addr, port: u16,
) -> i32 {
    match typ {
        NF_CT_FTP_PORT | NF_CT_FTP_PASV => {
            let octets = (addr as *mut u8);
            snprintf(buffer, buflen, b"%u,%u,%u,%u,%u,%u\0".as_ptr(),
                *octets, *octets.add(1), *octets.add(2), *octets.add(3),
                port >> 8, port & 0xFF)
        }
        NF_CT_FTP_EPRT => {
            if nf_ct_l3num(ct) == NFPROTO_IPV4 {
                snprintf(buffer, buflen, b"|1|%pI4|%u|\0".as_ptr(), addr, port)
            } else {
                snprintf(buffer, buflen, b"|2|%pI6|%u|\0".as_ptr(), addr, port)
            }
        }
        NF_CT_FTP_EPSV => snprintf(buffer, buflen, b"|||%u|\0".as_ptr(), port),
        _ => 0,
    }
}

unsafe fn nf_nat_ftp(
    skb: *mut sk_buff, ct: *mut nf_conn, ctinfo: ip_conntrack_info,
    typ: nf_ct_ftp_type, protoff: u32, matchoff: u32, matchlen: u32,
    exp: *mut nf_conntrack_expect,
) -> u32 {
    let dir = ctinfo & 1;
    let mut newaddr: nf_inet_addr;
    let mut buffer = [0u8; b"|1||65535|\0".len() - 1 + INET6_ADDRSTRLEN];
    let opposite = (!dir) as usize;

    newaddr = (*ct).tuplehash[opposite].tuple.dst.u3;
    (*exp).saved_proto.tcp.port = (*exp).tuple.dst.u.tcp.port;
    (*exp).dir = !dir;
    (*exp).expectfn = Some(nf_nat_follow_master);

    let port = nf_nat_exp_find_port(exp, ntohs((*exp).saved_proto.tcp.port));
    if port == 0 {
        nf_ct_helper_log(skb, ct, b"all ports in use\0".as_ptr());
        return NF_DROP;
    }

    let buflen = nf_nat_ftp_fmt_cmd(ct, typ, buffer.as_mut_ptr(), buffer.len(), &mut newaddr, port);
    if buflen == 0 { return nf_nat_ftp_out(skb, ct, exp); }
    if !nf_nat_mangle_tcp_packet(skb, ct, ctinfo, protoff, matchoff, matchlen,
                                 buffer.as_ptr(), buflen as u32) {
        return nf_nat_ftp_out(skb, ct, exp);
    }
    NF_ACCEPT
}

unsafe fn nf_nat_ftp_out(skb: *mut sk_buff, ct: *mut nf_conn, exp: *mut nf_conntrack_expect) -> u32 {
    nf_ct_helper_log(skb, ct, b"cannot mangle packet\0".as_ptr());
    nf_ct_unexpect_related(exp);
    NF_DROP
}

unsafe extern "C" fn nf_nat_ftp_fini() {
    nf_nat_helper_unregister(&mut nat_helper_ftp);
    nf_nat_ftp_hook = None;
    synchronize_rcu();
}

unsafe extern "C" fn nf_nat_ftp_init() -> i32 {
    // BUG_ON(nf_nat_ftp_hook != NULL)
    nf_nat_helper_register(&mut nat_helper_ftp);
    nf_nat_ftp_hook = Some(nf_nat_ftp);
    0
}

unsafe extern "C" fn warn_set(_val: *const u8, _kp: *const kernel_param) -> i32 {
    // pr_info("kernel >= 2.6.10 only uses 'ports' for conntrack modules\n");
    0
}

// MODULE_LICENSE("GPL"); MODULE_AUTHOR(...); MODULE_DESCRIPTION(...);
// MODULE_ALIAS_NF_NAT_HELPER(NAT_HELPER_NAME);
// module_param_call(ports, warn_set, NULL, NULL, 0);
// module_init(nf_nat_ftp_init); module_exit(nf_nat_ftp_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
