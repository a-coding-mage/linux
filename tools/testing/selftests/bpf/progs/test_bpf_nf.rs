// SPDX-License-Identifier: GPL-2.0
// #define BPF_NO_KFUNC_PROTOTYPES
// C includes translated as external dependencies:
// <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type s32 = core::ffi::c_int;
type __u16 = u16;
type __be16 = u16;
type __be32 = u32;

const EAFNOSUPPORT: s32 = 97;
const EPROTO: s32 = 71;
const ENONET: s32 = 64;
const EINVAL: s32 = 22;
const ENOENT: s32 = 2;

const CT_OPTS_ERROR_GUARD: s32 = 0x12345678;

// External kernel/BPF constants from translated includes.
const IPPROTO_TCP: u8 = 6;
const IPPROTO_ICMP: u8 = 1;
const IP_CT_DIR_ORIGINAL: u32 = 0;
const IP_CT_DIR_REPLY: u32 = 1;
const IPS_CONFIRMED: u32 = 1 << 3;
const IPS_SEEN_REPLY: u32 = 1 << 1;

const NF_CT_ZONE_DIR_ORIG: u8 = (1 << IP_CT_DIR_ORIGINAL) as u8;
const NF_CT_ZONE_DIR_REPL: u8 = (1 << IP_CT_DIR_REPLY) as u8;

unsafe extern "C" {
    #[link_name = "CONFIG_HZ"]
    static mut CONFIG_HZ: core::ffi::c_ulong;
}

#[unsafe(no_mangle)]
pub static mut test_einval_reserved: s32 = 0;
#[unsafe(no_mangle)]
pub static mut test_einval_reserved_new: s32 = 0;
#[unsafe(no_mangle)]
pub static mut test_einval_netns_id: s32 = 0;
#[unsafe(no_mangle)]
pub static mut test_einval_len_opts: s32 = 0;
#[unsafe(no_mangle)]
pub static mut test_einval_len_opts_small_lookup: s32 = 0;
#[unsafe(no_mangle)]
pub static mut test_einval_len_opts_small_alloc: s32 = 0;
#[unsafe(no_mangle)]
pub static mut test_eproto_l4proto: s32 = 0;
#[unsafe(no_mangle)]
pub static mut test_enonet_netns_id: s32 = 0;
#[unsafe(no_mangle)]
pub static mut test_enoent_lookup: s32 = 0;
#[unsafe(no_mangle)]
pub static mut test_eafnosupport: s32 = 0;
#[unsafe(no_mangle)]
pub static mut test_alloc_entry: s32 = -EINVAL;
#[unsafe(no_mangle)]
pub static mut test_insert_entry: s32 = -EAFNOSUPPORT;
#[unsafe(no_mangle)]
pub static mut test_succ_lookup: s32 = -ENOENT;
#[unsafe(no_mangle)]
pub static mut test_ct_zone_id_alloc_entry: s32 = -EINVAL;
#[unsafe(no_mangle)]
pub static mut test_ct_zone_id_insert_entry: s32 = -EAFNOSUPPORT;
#[unsafe(no_mangle)]
pub static mut test_ct_zone_id_succ_lookup: s32 = -ENOENT;
#[unsafe(no_mangle)]
pub static mut test_ct_zone_dir_enoent_lookup: s32 = 0;
#[unsafe(no_mangle)]
pub static mut test_ct_zone_id_enoent_lookup: s32 = 0;
#[unsafe(no_mangle)]
pub static mut test_delta_timeout: u32 = 0;
#[unsafe(no_mangle)]
pub static mut test_status: u32 = 0;
#[unsafe(no_mangle)]
pub static mut test_insert_lookup_mark: u32 = 0;
#[unsafe(no_mangle)]
pub static mut test_snat_addr: s32 = -EINVAL;
#[unsafe(no_mangle)]
pub static mut test_dnat_addr: s32 = -EINVAL;
#[unsafe(no_mangle)]
pub static mut saddr: __be32 = 0;
#[unsafe(no_mangle)]
pub static mut sport: __be16 = 0;
#[unsafe(no_mangle)]
pub static mut daddr: __be32 = 0;
#[unsafe(no_mangle)]
pub static mut dport: __be16 = 0;
#[unsafe(no_mangle)]
pub static mut test_exist_lookup: s32 = -ENOENT;
#[unsafe(no_mangle)]
pub static mut test_exist_lookup_mark: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nf_nat_manip_type___local {
    NF_NAT_MANIP_SRC___local,
    NF_NAT_MANIP_DST___local,
}

#[repr(C)]
pub struct nf_conn {
    pub tuplehash: [nf_conntrack_tuple_hash; 2],
    pub status: u32,
    pub timeout: u32,
    pub mark: u32,
}

#[repr(C)]
pub struct nf_conntrack_tuple_hash {
    pub tuple: nf_conntrack_tuple,
}

#[repr(C)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_man,
    pub dst: nf_conntrack_man,
}

#[repr(C)]
pub struct nf_conntrack_man {
    pub u3: nf_inet_addr,
    pub u: nf_conntrack_man_proto,
}

#[repr(C)]
pub union nf_conntrack_man_proto {
    pub all: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_inet_addr {
    pub ip: __be32,
}

#[repr(C)]
pub struct bpf_ct_opts___local {
    pub netns_id: s32,
    pub error: s32,
    pub l4proto: u8,
    pub dir: u8,
    pub reserved: [u8; 2],
}

#[repr(C)]
pub struct bpf_ct_opts___new {
    pub netns_id: s32,
    pub error: s32,
    pub l4proto: u8,
    pub dir: u8,
    pub ct_zone_id: u16,
    pub ct_zone_dir: u8,
    pub reserved: [u8; 3],
}

#[repr(C)]
pub struct xdp_md {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock_tuple {
    pub ipv4: bpf_sock_tuple_ipv4,
}

#[repr(C)]
pub struct bpf_sock_tuple_ipv4 {
    pub saddr: __be32,
    pub daddr: __be32,
    pub sport: __be16,
    pub dport: __be16,
}

type ct_lookup_local_fn = unsafe extern "C" fn(
    *mut core::ffi::c_void,
    *mut bpf_sock_tuple,
    u32,
    *mut bpf_ct_opts___local,
    u32,
) -> *mut nf_conn;

type ct_alloc_local_fn = unsafe extern "C" fn(
    *mut core::ffi::c_void,
    *mut bpf_sock_tuple,
    u32,
    *mut bpf_ct_opts___local,
    u32,
) -> *mut nf_conn;

type ct_lookup_new_fn = unsafe extern "C" fn(
    *mut core::ffi::c_void,
    *mut bpf_sock_tuple,
    u32,
    *mut bpf_ct_opts___new,
    u32,
) -> *mut nf_conn;

type ct_alloc_new_fn = unsafe extern "C" fn(
    *mut core::ffi::c_void,
    *mut bpf_sock_tuple,
    u32,
    *mut bpf_ct_opts___new,
    u32,
) -> *mut nf_conn;

unsafe extern "C" {
    fn bpf_xdp_ct_alloc(
        ctx: *mut xdp_md,
        tuple: *mut bpf_sock_tuple,
        len_tuple: u32,
        opts: *mut bpf_ct_opts___local,
        len_opts: u32,
    ) -> *mut nf_conn;
    fn bpf_xdp_ct_lookup(
        ctx: *mut xdp_md,
        tuple: *mut bpf_sock_tuple,
        len_tuple: u32,
        opts: *mut bpf_ct_opts___local,
        len_opts: u32,
    ) -> *mut nf_conn;
    fn bpf_skb_ct_alloc(
        ctx: *mut __sk_buff,
        tuple: *mut bpf_sock_tuple,
        len_tuple: u32,
        opts: *mut bpf_ct_opts___local,
        len_opts: u32,
    ) -> *mut nf_conn;
    fn bpf_skb_ct_lookup(
        ctx: *mut __sk_buff,
        tuple: *mut bpf_sock_tuple,
        len_tuple: u32,
        opts: *mut bpf_ct_opts___local,
        len_opts: u32,
    ) -> *mut nf_conn;
    fn bpf_ct_insert_entry(ct: *mut nf_conn) -> *mut nf_conn;
    fn bpf_ct_release(ct: *mut nf_conn);
    fn bpf_ct_set_timeout(ct: *mut nf_conn, timeout: u32);
    fn bpf_ct_change_timeout(ct: *mut nf_conn, timeout: u32) -> s32;
    fn bpf_ct_set_status(ct: *mut nf_conn, status: u32) -> s32;
    fn bpf_ct_change_status(ct: *mut nf_conn, status: u32) -> s32;
    fn bpf_ct_set_nat_info(
        ct: *mut nf_conn,
        addr: *mut nf_inet_addr,
        port: core::ffi::c_int,
        manip: nf_nat_manip_type___local,
    ) -> s32;
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_jiffies64() -> u64;
    fn bpf_htons(val: u16) -> u16;
}

#[inline(always)]
unsafe fn nf_ct_test(
    lookup_fn: ct_lookup_local_fn,
    alloc_fn: ct_alloc_local_fn,
    ctx: *mut core::ffi::c_void,
) {
    let mut opts_def = bpf_ct_opts___local {
        netns_id: -1,
        error: 0,
        l4proto: IPPROTO_TCP,
        dir: 0,
        reserved: [0; 2],
    };
    let mut bpf_tuple: bpf_sock_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    core::ptr::write_bytes(
        &mut bpf_tuple as *mut bpf_sock_tuple as *mut u8,
        0,
        core::mem::size_of_val(&bpf_tuple.ipv4),
    );

    opts_def.reserved[0] = 1;
    ct = lookup_fn(
        ctx,
        &mut bpf_tuple,
        core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
        &mut opts_def,
        core::mem::size_of_val(&opts_def) as u32,
    );
    opts_def.reserved[0] = 0;
    opts_def.l4proto = IPPROTO_TCP;
    if !ct.is_null() {
        bpf_ct_release(ct);
    } else {
        test_einval_reserved = opts_def.error;
    }

    opts_def.netns_id = -2;
    ct = lookup_fn(
        ctx,
        &mut bpf_tuple,
        core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
        &mut opts_def,
        core::mem::size_of_val(&opts_def) as u32,
    );
    opts_def.netns_id = -1;
    if !ct.is_null() {
        bpf_ct_release(ct);
    } else {
        test_einval_netns_id = opts_def.error;
    }

    ct = lookup_fn(
        ctx,
        &mut bpf_tuple,
        core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
        &mut opts_def,
        (core::mem::size_of_val(&opts_def) - 1) as u32,
    );
    if !ct.is_null() {
        bpf_ct_release(ct);
    } else {
        test_einval_len_opts = opts_def.error;
    }

    opts_def.error = CT_OPTS_ERROR_GUARD;
    ct = lookup_fn(
        ctx,
        &mut bpf_tuple,
        core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
        &mut opts_def,
        core::mem::size_of_val(&opts_def.netns_id) as u32,
    );
    if !ct.is_null() {
        bpf_ct_release(ct);
        test_einval_len_opts_small_lookup = -EINVAL;
    } else {
        test_einval_len_opts_small_lookup = opts_def.error;
    }

    opts_def.error = CT_OPTS_ERROR_GUARD;
    ct = alloc_fn(
        ctx,
        &mut bpf_tuple,
        core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
        &mut opts_def,
        core::mem::size_of_val(&opts_def.netns_id) as u32,
    );
    if !ct.is_null() {
        ct = bpf_ct_insert_entry(ct);
        if !ct.is_null() {
            bpf_ct_release(ct);
        }
        test_einval_len_opts_small_alloc = -EINVAL;
    } else {
        test_einval_len_opts_small_alloc = opts_def.error;
    }

    opts_def.l4proto = IPPROTO_ICMP;
    ct = lookup_fn(
        ctx,
        &mut bpf_tuple,
        core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
        &mut opts_def,
        core::mem::size_of_val(&opts_def) as u32,
    );
    opts_def.l4proto = IPPROTO_TCP;
    if !ct.is_null() {
        bpf_ct_release(ct);
    } else {
        test_eproto_l4proto = opts_def.error;
    }

    opts_def.netns_id = 0xf00f;
    ct = lookup_fn(
        ctx,
        &mut bpf_tuple,
        core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
        &mut opts_def,
        core::mem::size_of_val(&opts_def) as u32,
    );
    opts_def.netns_id = -1;
    if !ct.is_null() {
        bpf_ct_release(ct);
    } else {
        test_enonet_netns_id = opts_def.error;
    }

    ct = lookup_fn(
        ctx,
        &mut bpf_tuple,
        core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
        &mut opts_def,
        core::mem::size_of_val(&opts_def) as u32,
    );
    if !ct.is_null() {
        bpf_ct_release(ct);
    } else {
        test_enoent_lookup = opts_def.error;
    }

    ct = lookup_fn(
        ctx,
        &mut bpf_tuple,
        (core::mem::size_of_val(&bpf_tuple.ipv4) - 1) as u32,
        &mut opts_def,
        core::mem::size_of_val(&opts_def) as u32,
    );
    if !ct.is_null() {
        bpf_ct_release(ct);
    } else {
        test_eafnosupport = opts_def.error;
    }

    bpf_tuple.ipv4.saddr = bpf_get_prandom_u32(); /* src IP */
    bpf_tuple.ipv4.daddr = bpf_get_prandom_u32(); /* dst IP */
    bpf_tuple.ipv4.sport = bpf_get_prandom_u32() as __be16; /* src port */
    bpf_tuple.ipv4.dport = bpf_get_prandom_u32() as __be16; /* dst port */

    ct = alloc_fn(
        ctx,
        &mut bpf_tuple,
        core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
        &mut opts_def,
        core::mem::size_of_val(&opts_def) as u32,
    );
    if !ct.is_null() {
        let sport_local: __u16 = bpf_get_prandom_u32() as __u16;
        let dport_local: __u16 = bpf_get_prandom_u32() as __u16;
        let mut saddr_local = nf_inet_addr { ip: 0 };
        let mut daddr_local = nf_inet_addr { ip: 0 };
        let ct_ins: *mut nf_conn;

        bpf_ct_set_timeout(ct, 10000);
        (*ct).mark = 77;

        /* snat */
        saddr_local.ip = bpf_get_prandom_u32();
        bpf_ct_set_nat_info(
            ct,
            &mut saddr_local,
            sport_local as core::ffi::c_int,
            nf_nat_manip_type___local::NF_NAT_MANIP_SRC___local,
        );
        /* dnat */
        daddr_local.ip = bpf_get_prandom_u32();
        bpf_ct_set_nat_info(
            ct,
            &mut daddr_local,
            dport_local as core::ffi::c_int,
            nf_nat_manip_type___local::NF_NAT_MANIP_DST___local,
        );

        ct_ins = bpf_ct_insert_entry(ct);
        if !ct_ins.is_null() {
            let ct_lk: *mut nf_conn;

            ct_lk = lookup_fn(
                ctx,
                &mut bpf_tuple,
                core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
                &mut opts_def,
                core::mem::size_of_val(&opts_def) as u32,
            );
            if !ct_lk.is_null() {
                let tuple: *mut nf_conntrack_tuple;

                /* check snat and dnat addresses */
                tuple = &mut (*ct_lk).tuplehash[IP_CT_DIR_REPLY as usize].tuple;
                if (*tuple).dst.u3.ip == saddr_local.ip
                    && (*tuple).dst.u.all == bpf_htons(sport_local)
                {
                    test_snat_addr = 0;
                }
                if (*tuple).src.u3.ip == daddr_local.ip
                    && (*tuple).src.u.all == bpf_htons(dport_local)
                {
                    test_dnat_addr = 0;
                }

                /* update ct entry timeout */
                bpf_ct_change_timeout(ct_lk, 10000);
                test_delta_timeout = ((*ct_lk).timeout as u64).wrapping_sub(bpf_jiffies64()) as u32;
                test_delta_timeout /= CONFIG_HZ as u32;
                test_insert_lookup_mark = (*ct_lk).mark;
                bpf_ct_change_status(ct_lk, IPS_CONFIRMED | IPS_SEEN_REPLY);
                test_status = (*ct_lk).status;

                bpf_ct_release(ct_lk);
                test_succ_lookup = 0;
            }
            bpf_ct_release(ct_ins);
            test_insert_entry = 0;
        }
        test_alloc_entry = 0;
    }

    bpf_tuple.ipv4.saddr = saddr;
    bpf_tuple.ipv4.daddr = daddr;
    bpf_tuple.ipv4.sport = sport;
    bpf_tuple.ipv4.dport = dport;
    ct = lookup_fn(
        ctx,
        &mut bpf_tuple,
        core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
        &mut opts_def,
        core::mem::size_of_val(&opts_def) as u32,
    );
    if !ct.is_null() {
        test_exist_lookup = 0;
        if (*ct).mark == 42 {
            (*ct).mark = (*ct).mark.wrapping_add(1);
            test_exist_lookup_mark = (*ct).mark;
        }
        bpf_ct_release(ct);
    } else {
        test_exist_lookup = opts_def.error;
    }
}

#[inline(always)]
unsafe fn nf_ct_opts_new_test(
    lookup_fn: ct_lookup_new_fn,
    alloc_fn: ct_alloc_new_fn,
    ctx: *mut core::ffi::c_void,
) {
    let mut opts_def = bpf_ct_opts___new {
        netns_id: -1,
        error: 0,
        l4proto: IPPROTO_TCP,
        dir: 0,
        ct_zone_id: 0,
        ct_zone_dir: 0,
        reserved: [0; 3],
    };
    let mut bpf_tuple: bpf_sock_tuple = core::mem::zeroed();
    let mut ct: *mut nf_conn;

    core::ptr::write_bytes(
        &mut bpf_tuple as *mut bpf_sock_tuple as *mut u8,
        0,
        core::mem::size_of_val(&bpf_tuple.ipv4),
    );

    opts_def.reserved[0] = 1;
    ct = lookup_fn(
        ctx,
        &mut bpf_tuple,
        core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
        &mut opts_def,
        core::mem::size_of_val(&opts_def) as u32,
    );
    opts_def.reserved[0] = 0;
    if !ct.is_null() {
        bpf_ct_release(ct);
    } else {
        test_einval_reserved_new = opts_def.error;
    }

    bpf_tuple.ipv4.saddr = bpf_get_prandom_u32(); /* src IP */
    bpf_tuple.ipv4.daddr = bpf_get_prandom_u32(); /* dst IP */
    bpf_tuple.ipv4.sport = bpf_get_prandom_u32() as __be16; /* src port */
    bpf_tuple.ipv4.dport = bpf_get_prandom_u32() as __be16; /* dst port */

    /* use non-default ct zone */
    opts_def.ct_zone_id = 10;
    opts_def.ct_zone_dir = NF_CT_ZONE_DIR_ORIG;
    ct = alloc_fn(
        ctx,
        &mut bpf_tuple,
        core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
        &mut opts_def,
        core::mem::size_of_val(&opts_def) as u32,
    );
    if !ct.is_null() {
        let sport_local: __u16 = bpf_get_prandom_u32() as __u16;
        let dport_local: __u16 = bpf_get_prandom_u32() as __u16;
        let mut saddr_local = nf_inet_addr { ip: 0 };
        let mut daddr_local = nf_inet_addr { ip: 0 };
        let ct_ins: *mut nf_conn;

        bpf_ct_set_timeout(ct, 10000);

        /* snat */
        saddr_local.ip = bpf_get_prandom_u32();
        bpf_ct_set_nat_info(
            ct,
            &mut saddr_local,
            sport_local as core::ffi::c_int,
            nf_nat_manip_type___local::NF_NAT_MANIP_SRC___local,
        );
        /* dnat */
        daddr_local.ip = bpf_get_prandom_u32();
        bpf_ct_set_nat_info(
            ct,
            &mut daddr_local,
            dport_local as core::ffi::c_int,
            nf_nat_manip_type___local::NF_NAT_MANIP_DST___local,
        );

        ct_ins = bpf_ct_insert_entry(ct);
        if !ct_ins.is_null() {
            let mut ct_lk: *mut nf_conn;

            /* entry should exist in same ct zone we inserted it */
            ct_lk = lookup_fn(
                ctx,
                &mut bpf_tuple,
                core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
                &mut opts_def,
                core::mem::size_of_val(&opts_def) as u32,
            );
            if !ct_lk.is_null() {
                bpf_ct_release(ct_lk);
                test_ct_zone_id_succ_lookup = 0;
            }

            /* entry should not exist with wrong direction */
            opts_def.ct_zone_dir = NF_CT_ZONE_DIR_REPL;
            ct_lk = lookup_fn(
                ctx,
                &mut bpf_tuple,
                core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
                &mut opts_def,
                core::mem::size_of_val(&opts_def) as u32,
            );
            opts_def.ct_zone_dir = NF_CT_ZONE_DIR_ORIG;
            if !ct_lk.is_null() {
                bpf_ct_release(ct_lk);
            } else {
                test_ct_zone_dir_enoent_lookup = opts_def.error;
            }

            /* entry should not exist in default ct zone */
            opts_def.ct_zone_id = 0;
            ct_lk = lookup_fn(
                ctx,
                &mut bpf_tuple,
                core::mem::size_of_val(&bpf_tuple.ipv4) as u32,
                &mut opts_def,
                core::mem::size_of_val(&opts_def) as u32,
            );
            if !ct_lk.is_null() {
                bpf_ct_release(ct_lk);
            } else {
                test_ct_zone_id_enoent_lookup = opts_def.error;
            }

            bpf_ct_release(ct_ins);
            test_ct_zone_id_insert_entry = 0;
        }
        test_ct_zone_id_alloc_entry = 0;
    }
}

// SEC("xdp")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_xdp_ct_test(ctx: *mut xdp_md) -> core::ffi::c_int {
    nf_ct_test(
        core::mem::transmute::<
            unsafe extern "C" fn(
                *mut xdp_md,
                *mut bpf_sock_tuple,
                u32,
                *mut bpf_ct_opts___local,
                u32,
            ) -> *mut nf_conn,
            ct_lookup_local_fn,
        >(bpf_xdp_ct_lookup),
        core::mem::transmute::<
            unsafe extern "C" fn(
                *mut xdp_md,
                *mut bpf_sock_tuple,
                u32,
                *mut bpf_ct_opts___local,
                u32,
            ) -> *mut nf_conn,
            ct_alloc_local_fn,
        >(bpf_xdp_ct_alloc),
        ctx as *mut core::ffi::c_void,
    );
    nf_ct_opts_new_test(
        core::mem::transmute::<
            unsafe extern "C" fn(
                *mut xdp_md,
                *mut bpf_sock_tuple,
                u32,
                *mut bpf_ct_opts___local,
                u32,
            ) -> *mut nf_conn,
            ct_lookup_new_fn,
        >(bpf_xdp_ct_lookup),
        core::mem::transmute::<
            unsafe extern "C" fn(
                *mut xdp_md,
                *mut bpf_sock_tuple,
                u32,
                *mut bpf_ct_opts___local,
                u32,
            ) -> *mut nf_conn,
            ct_alloc_new_fn,
        >(bpf_xdp_ct_alloc),
        ctx as *mut core::ffi::c_void,
    );
    0
}

// SEC("tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_skb_ct_test(ctx: *mut __sk_buff) -> core::ffi::c_int {
    nf_ct_test(
        core::mem::transmute::<
            unsafe extern "C" fn(
                *mut __sk_buff,
                *mut bpf_sock_tuple,
                u32,
                *mut bpf_ct_opts___local,
                u32,
            ) -> *mut nf_conn,
            ct_lookup_local_fn,
        >(bpf_skb_ct_lookup),
        core::mem::transmute::<
            unsafe extern "C" fn(
                *mut __sk_buff,
                *mut bpf_sock_tuple,
                u32,
                *mut bpf_ct_opts___local,
                u32,
            ) -> *mut nf_conn,
            ct_alloc_local_fn,
        >(bpf_skb_ct_alloc),
        ctx as *mut core::ffi::c_void,
    );
    nf_ct_opts_new_test(
        core::mem::transmute::<
            unsafe extern "C" fn(
                *mut __sk_buff,
                *mut bpf_sock_tuple,
                u32,
                *mut bpf_ct_opts___local,
                u32,
            ) -> *mut nf_conn,
            ct_lookup_new_fn,
        >(bpf_skb_ct_lookup),
        core::mem::transmute::<
            unsafe extern "C" fn(
                *mut __sk_buff,
                *mut bpf_sock_tuple,
                u32,
                *mut bpf_ct_opts___local,
                u32,
            ) -> *mut nf_conn,
            ct_alloc_new_fn,
        >(bpf_skb_ct_alloc),
        ctx as *mut core::ffi::c_void,
    );
    0
}

// char _license[] SEC("license") = "GPL";
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
