// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2020 Cloudflare

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;

const EEXIST: i32 = 17;
const ESOCKTNOSUPPORT: i32 = 94;

const AF_INET: __u32 = 2;
const AF_INET6: __u32 = 10;
const SOCK_STREAM: __u32 = 1;
const IPPROTO_TCP: __u8 = 6;

const BPF_MAP_TYPE_SOCKMAP: __u32 = 15;
const BPF_MAP_TYPE_ARRAY: __u32 = 2;
const BPF_ANY: __u64 = 0;
const BPF_SK_LOOKUP_F_REPLACE: __u64 = 1 << 0;
const BPF_SK_LOOKUP_F_NO_REUSEPORT: __u64 = 1 << 1;
const BPF_TCP_LISTEN: __u32 = 10;

const SK_DROP: i32 = 0;
const SK_PASS: i32 = 1;

#[repr(C)]
pub struct bpf_sk_lookup {
    pub family: __u32,
    pub protocol: __u32,
    pub remote_ip4: __u32,
    pub remote_ip6: [__u32; 4],
    pub remote_port: __u32,
    pub local_ip4: __u32,
    pub local_ip6: [__u32; 4],
    pub local_port: __u32,
    pub ingress_ifindex: __u32,
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct sk_reuseport_md {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock {
    pub family: __u32,
    pub type_: __u32,
    pub protocol: __u32,
    pub mark: __u32,
    pub priority: __u32,
    pub src_ip4: __u32,
    pub src_ip6: [__u32; 4],
    pub src_port: __u32,
    pub dst_port: __u32,
    pub dst_ip4: __u32,
    pub dst_ip6: [__u32; 4],
    pub state: __u32,
    pub rx_queue_mapping: __u32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
}

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *const c_void, key: *const c_void) -> *mut bpf_sock;
    fn bpf_map_update_elem(
        map: *const c_void,
        key: *const c_void,
        value: *const c_void,
        flags: __u64,
    ) -> i64;
    fn bpf_sk_assign(ctx: *mut bpf_sk_lookup, sk: *mut bpf_sock, flags: __u64) -> i32;
    fn bpf_sk_release(sk: *mut bpf_sock);
    fn bpf_sk_select_reuseport(
        ctx: *mut sk_reuseport_md,
        map: *const c_void,
        key: *const c_void,
        flags: __u64,
    ) -> i32;
    fn bpf_printk(fmt: *const u8, ...) -> i32;
}

const fn bpf_htonl(x: __u32) -> __u32 {
    x.to_be()
}

const fn bpf_htons(x: __u16) -> __u16 {
    x.to_be()
}

const fn IP4(a: __u32, b: __u32, c: __u32, d: __u32) -> __u32 {
    bpf_htonl(((a & 0xff) << 24) | ((b & 0xff) << 16) | ((c & 0xff) << 8) | ((d & 0xff) << 0))
}

const fn IP6(aaaa: __u32, bbbb: __u32, cccc: __u32, dddd: __u32) -> [__u32; 4] {
    [bpf_htonl(aaaa), bpf_htonl(bbbb), bpf_htonl(cccc), bpf_htonl(dddd)]
}

/* Macros for least-significant byte and word accesses. */
unsafe fn LSB<T>(value: *const T, index: usize) -> __u8 {
    #[cfg(target_endian = "little")]
    {
        *(value as *const __u8).add(index)
    }
    #[cfg(target_endian = "big")]
    {
        *(value as *const __u8).add(core::mem::size_of::<T>() - index - 1)
    }
}

unsafe fn LSW<T>(value: *const T, index: usize) -> __u16 {
    #[cfg(target_endian = "little")]
    {
        *(value as *const __u16).add(index)
    }
    #[cfg(target_endian = "big")]
    {
        *(value as *const __u16).add(core::mem::size_of::<T>() / 2 - index - 1)
    }
}

const MAX_SOCKS: __u32 = 32;

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static redir_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: MAX_SOCKS,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static run_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 2,
};

const PROG1: i32 = 0;
const PROG2: i32 = 1;

const SERVER_A: __u32 = 0;
const SERVER_B: __u32 = 1;

/* Addressable key/value constants for convenience */
static KEY_PROG1: i32 = PROG1;
static KEY_PROG2: i32 = PROG2;
static PROG_DONE: i32 = 1;

static KEY_SERVER_A: __u32 = SERVER_A;
static KEY_SERVER_B: __u32 = SERVER_B;

static SRC_PORT: __u16 = bpf_htons(8008);
static SRC_IP4: __u32 = IP4(127, 0, 0, 2);
static SRC_IP6: [__u32; 4] = IP6(0xfd000000, 0x0, 0x0, 0x00000002);

static DST_PORT: __u32 = 7007; /* Host byte order */
static DST_IP4: __u32 = IP4(127, 0, 0, 1);
static DST_IP6: [__u32; 4] = IP6(0xfd000000, 0x0, 0x0, 0x00000001);

#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lookup_pass(_ctx: *mut bpf_sk_lookup) -> i32 {
    SK_PASS
}

#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lookup_drop(_ctx: *mut bpf_sk_lookup) -> i32 {
    SK_DROP
}

#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_ifindex(ctx: *mut bpf_sk_lookup) -> i32 {
    if (*ctx).ingress_ifindex == 1 {
        return SK_DROP;
    }
    SK_PASS
}

#[unsafe(link_section = "sk_reuseport")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reuseport_pass(_ctx: *mut sk_reuseport_md) -> i32 {
    SK_PASS
}

#[unsafe(link_section = "sk_reuseport")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reuseport_drop(_ctx: *mut sk_reuseport_md) -> i32 {
    SK_DROP
}

/* Redirect packets destined for port DST_PORT to socket at redir_map[0]. */
#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn redir_port(ctx: *mut bpf_sk_lookup) -> i32 {
    let sk: *mut bpf_sock;
    let err: i32;

    if (*ctx).local_port != DST_PORT {
        return SK_PASS;
    }

    sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_A as *const _ as *const c_void);
    if sk.is_null() {
        return SK_PASS;
    }

    err = bpf_sk_assign(ctx, sk, 0);
    bpf_sk_release(sk);
    if err != 0 { SK_DROP } else { SK_PASS }
}

/* Redirect packets destined for DST_IP4 address to socket at redir_map[0]. */
#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn redir_ip4(ctx: *mut bpf_sk_lookup) -> i32 {
    let sk: *mut bpf_sock;
    let err: i32;

    if (*ctx).family != AF_INET {
        return SK_PASS;
    }
    if (*ctx).local_port != DST_PORT {
        return SK_PASS;
    }
    if (*ctx).local_ip4 != DST_IP4 {
        return SK_PASS;
    }

    sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_A as *const _ as *const c_void);
    if sk.is_null() {
        return SK_PASS;
    }

    err = bpf_sk_assign(ctx, sk, 0);
    bpf_sk_release(sk);
    if err != 0 { SK_DROP } else { SK_PASS }
}

/* Redirect packets destined for DST_IP6 address to socket at redir_map[0]. */
#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn redir_ip6(ctx: *mut bpf_sk_lookup) -> i32 {
    let sk: *mut bpf_sock;
    let err: i32;

    if (*ctx).family != AF_INET6 {
        return SK_PASS;
    }
    if (*ctx).local_port != DST_PORT {
        return SK_PASS;
    }
    if (*ctx).local_ip6[0] != DST_IP6[0]
        || (*ctx).local_ip6[1] != DST_IP6[1]
        || (*ctx).local_ip6[2] != DST_IP6[2]
        || (*ctx).local_ip6[3] != DST_IP6[3]
    {
        return SK_PASS;
    }

    sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_A as *const _ as *const c_void);
    if sk.is_null() {
        return SK_PASS;
    }

    err = bpf_sk_assign(ctx, sk, 0);
    bpf_sk_release(sk);
    if err != 0 { SK_DROP } else { SK_PASS }
}

#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn select_sock_a(ctx: *mut bpf_sk_lookup) -> i32 {
    let sk: *mut bpf_sock;
    let err: i32;

    sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_A as *const _ as *const c_void);
    if sk.is_null() {
        return SK_PASS;
    }

    err = bpf_sk_assign(ctx, sk, 0);
    bpf_sk_release(sk);
    if err != 0 { SK_DROP } else { SK_PASS }
}

#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn select_sock_a_no_reuseport(ctx: *mut bpf_sk_lookup) -> i32 {
    let sk: *mut bpf_sock;
    let err: i32;

    sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_A as *const _ as *const c_void);
    if sk.is_null() {
        return SK_DROP;
    }

    err = bpf_sk_assign(ctx, sk, BPF_SK_LOOKUP_F_NO_REUSEPORT);
    bpf_sk_release(sk);
    if err != 0 { SK_DROP } else { SK_PASS }
}

#[unsafe(link_section = "sk_reuseport")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn select_sock_b(ctx: *mut sk_reuseport_md) -> i32 {
    let mut key: __u32 = KEY_SERVER_B;
    let err: i32;

    err = bpf_sk_select_reuseport(
        ctx,
        &redir_map as *const _ as *const c_void,
        &mut key as *mut _ as *const c_void,
        0,
    );
    if err != 0 { SK_DROP } else { SK_PASS }
}

/* Check that bpf_sk_assign() returns -EEXIST if socket already selected. */
#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sk_assign_eexist(ctx: *mut bpf_sk_lookup) -> i32 {
    let mut sk: *mut bpf_sock;
    let mut ret: i32;

    ret = SK_DROP;
    sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_B as *const _ as *const c_void);
    if !sk.is_null() {
        let mut err = bpf_sk_assign(ctx, sk, 0);
        if err == 0 {
            bpf_sk_release(sk);

            sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_A as *const _ as *const c_void);
            if !sk.is_null() {
                err = bpf_sk_assign(ctx, sk, 0);
                if err != -EEXIST {
                    bpf_printk(c"sk_assign returned %d, expected %d\n".as_ptr() as *const u8, err, -EEXIST);
                } else {
                    ret = SK_PASS; /* Success, redirect to KEY_SERVER_B */
                }
            }
        }
    }
    if !sk.is_null() {
        bpf_sk_release(sk);
    }
    ret
}

/* Check that bpf_sk_assign(BPF_SK_LOOKUP_F_REPLACE) can override selection. */
#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sk_assign_replace_flag(ctx: *mut bpf_sk_lookup) -> i32 {
    let mut sk: *mut bpf_sock;
    let mut ret: i32;

    ret = SK_DROP;
    sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_A as *const _ as *const c_void);
    if !sk.is_null() {
        let mut err = bpf_sk_assign(ctx, sk, 0);
        if err == 0 {
            bpf_sk_release(sk);

            sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_B as *const _ as *const c_void);
            if !sk.is_null() {
                err = bpf_sk_assign(ctx, sk, BPF_SK_LOOKUP_F_REPLACE);
                if err != 0 {
                    bpf_printk(c"sk_assign returned %d, expected 0\n".as_ptr() as *const u8, err);
                } else {
                    ret = SK_PASS; /* Success, redirect to KEY_SERVER_B */
                }
            }
        }
    }
    if !sk.is_null() {
        bpf_sk_release(sk);
    }
    ret
}

/* Check that bpf_sk_assign(sk=NULL) is accepted. */
#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sk_assign_null(ctx: *mut bpf_sk_lookup) -> i32 {
    let mut sk: *mut bpf_sock = ptr::null_mut();
    let mut ret: i32;

    ret = SK_DROP;

    let mut err = bpf_sk_assign(ctx, ptr::null_mut(), 0);
    if err != 0 {
        bpf_printk(c"sk_assign returned %d, expected 0\n".as_ptr() as *const u8, err);
    } else {
        sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_B as *const _ as *const c_void);
        if !sk.is_null() {
            err = bpf_sk_assign(ctx, sk, BPF_SK_LOOKUP_F_REPLACE);
            if err != 0 {
                bpf_printk(c"sk_assign returned %d, expected 0\n".as_ptr() as *const u8, err);
            } else if (*ctx).sk == sk {
                err = bpf_sk_assign(ctx, ptr::null_mut(), 0);
                if err == -EEXIST {
                    err = bpf_sk_assign(ctx, ptr::null_mut(), BPF_SK_LOOKUP_F_REPLACE);
                    if err == 0 {
                        err = bpf_sk_assign(ctx, sk, BPF_SK_LOOKUP_F_REPLACE);
                        if err == 0 {
                            ret = SK_PASS; /* Success, redirect to KEY_SERVER_B */
                        }
                    }
                }
            }
        }
    }
    if !sk.is_null() {
        bpf_sk_release(sk);
    }
    ret
}

/* Check that selected sk is accessible through context. */
#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn access_ctx_sk(ctx: *mut bpf_sk_lookup) -> i32 {
    let mut sk1: *mut bpf_sock = ptr::null_mut();
    let mut sk2: *mut bpf_sock = ptr::null_mut();
    let mut ret: i32;

    ret = SK_DROP;

    /* Try accessing unassigned (NULL) ctx->sk field */
    if !(*ctx).sk.is_null() && (*(*ctx).sk).family != AF_INET {
    } else {
        /* Assign a value to ctx->sk */
        sk1 = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_A as *const _ as *const c_void);
        if !sk1.is_null() {
            let mut err = bpf_sk_assign(ctx, sk1, 0);
            if err == 0 && (*ctx).sk == sk1 {
                /* Access ctx->sk fields */
                if (*(*ctx).sk).family == AF_INET
                    && (*(*ctx).sk).type_ == SOCK_STREAM
                    && (*(*ctx).sk).state == BPF_TCP_LISTEN
                {
                    /* Reset selection */
                    err = bpf_sk_assign(ctx, ptr::null_mut(), BPF_SK_LOOKUP_F_REPLACE);
                    if err == 0 && (*ctx).sk.is_null() {
                        /* Assign another socket */
                        sk2 = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_B as *const _ as *const c_void);
                        if !sk2.is_null() {
                            err = bpf_sk_assign(ctx, sk2, BPF_SK_LOOKUP_F_REPLACE);
                            if err == 0 && (*ctx).sk == sk2 {
                                /* Access reassigned ctx->sk fields */
                                if (*(*ctx).sk).family == AF_INET
                                    && (*(*ctx).sk).type_ == SOCK_STREAM
                                    && (*(*ctx).sk).state == BPF_TCP_LISTEN
                                {
                                    ret = SK_PASS; /* Success, redirect to KEY_SERVER_B */
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !sk1.is_null() {
        bpf_sk_release(sk1);
    }
    if !sk2.is_null() {
        bpf_sk_release(sk2);
    }
    ret
}

/* Check narrow loads from ctx fields that support them.
 *
 * Narrow loads of size >= target field size from a non-zero offset
 * are not covered because they give bogus results, that is the
 * verifier ignores the offset.
 */
#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ctx_narrow_access(ctx: *mut bpf_sk_lookup) -> i32 {
    let sk: *mut bpf_sock;
    let val_u32: __u32;
    let v4: bool;

    v4 = (*ctx).family == AF_INET;

    /* Narrow loads from family field */
    if LSB(&(*ctx).family as *const _, 0) as __u32 != if v4 { AF_INET } else { AF_INET6 }
        || LSB(&(*ctx).family as *const _, 1) != 0
        || LSB(&(*ctx).family as *const _, 2) != 0
        || LSB(&(*ctx).family as *const _, 3) != 0
    {
        return SK_DROP;
    }
    if LSW(&(*ctx).family as *const _, 0) as __u32 != if v4 { AF_INET } else { AF_INET6 } {
        return SK_DROP;
    }

    /* Narrow loads from protocol field */
    if LSB(&(*ctx).protocol as *const _, 0) != IPPROTO_TCP
        || LSB(&(*ctx).protocol as *const _, 1) != 0
        || LSB(&(*ctx).protocol as *const _, 2) != 0
        || LSB(&(*ctx).protocol as *const _, 3) != 0
    {
        return SK_DROP;
    }
    if LSW(&(*ctx).protocol as *const _, 0) != IPPROTO_TCP as __u16 {
        return SK_DROP;
    }

    /* Narrow loads from remote_port field. Expect SRC_PORT. */
    if LSB(&(*ctx).remote_port as *const _, 0) != ((SRC_PORT >> 0) & 0xff) as __u8
        || LSB(&(*ctx).remote_port as *const _, 1) != ((SRC_PORT >> 8) & 0xff) as __u8
    {
        return SK_DROP;
    }
    if LSW(&(*ctx).remote_port as *const _, 0) != SRC_PORT {
        return SK_DROP;
    }

    /*
     * NOTE: 4-byte load from bpf_sk_lookup at remote_port offset
     * is quirky. It gets rewritten by the access converter to a
     * 2-byte load for backward compatibility. Treating the load
     * result as a be16 value makes the code portable across
     * little- and big-endian platforms.
     */
    val_u32 = ptr::read_unaligned(&(*ctx).remote_port as *const _ as *const __u32);
    if val_u32 != SRC_PORT as __u32 {
        return SK_DROP;
    }

    /* Narrow loads from local_port field. Expect DST_PORT. */
    if LSB(&(*ctx).local_port as *const _, 0) != ((DST_PORT >> 0) & 0xff) as __u8
        || LSB(&(*ctx).local_port as *const _, 1) != ((DST_PORT >> 8) & 0xff) as __u8
        || LSB(&(*ctx).local_port as *const _, 2) != 0
        || LSB(&(*ctx).local_port as *const _, 3) != 0
    {
        return SK_DROP;
    }
    if LSW(&(*ctx).local_port as *const _, 0) as __u32 != DST_PORT {
        return SK_DROP;
    }

    /* Narrow loads from IPv4 fields */
    if v4 {
        /* Expect SRC_IP4 in remote_ip4 */
        if LSB(&(*ctx).remote_ip4 as *const _, 0) != ((SRC_IP4 >> 0) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip4 as *const _, 1) != ((SRC_IP4 >> 8) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip4 as *const _, 2) != ((SRC_IP4 >> 16) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip4 as *const _, 3) != ((SRC_IP4 >> 24) & 0xff) as __u8
        {
            return SK_DROP;
        }
        if LSW(&(*ctx).remote_ip4 as *const _, 0) != ((SRC_IP4 >> 0) & 0xffff) as __u16
            || LSW(&(*ctx).remote_ip4 as *const _, 1) != ((SRC_IP4 >> 16) & 0xffff) as __u16
        {
            return SK_DROP;
        }

        /* Expect DST_IP4 in local_ip4 */
        if LSB(&(*ctx).local_ip4 as *const _, 0) != ((DST_IP4 >> 0) & 0xff) as __u8
            || LSB(&(*ctx).local_ip4 as *const _, 1) != ((DST_IP4 >> 8) & 0xff) as __u8
            || LSB(&(*ctx).local_ip4 as *const _, 2) != ((DST_IP4 >> 16) & 0xff) as __u8
            || LSB(&(*ctx).local_ip4 as *const _, 3) != ((DST_IP4 >> 24) & 0xff) as __u8
        {
            return SK_DROP;
        }
        if LSW(&(*ctx).local_ip4 as *const _, 0) != ((DST_IP4 >> 0) & 0xffff) as __u16
            || LSW(&(*ctx).local_ip4 as *const _, 1) != ((DST_IP4 >> 16) & 0xffff) as __u16
        {
            return SK_DROP;
        }
    } else {
        /* Expect 0.0.0.0 IPs when family != AF_INET */
        if LSB(&(*ctx).remote_ip4 as *const _, 0) != 0
            || LSB(&(*ctx).remote_ip4 as *const _, 1) != 0
            || LSB(&(*ctx).remote_ip4 as *const _, 2) != 0
            || LSB(&(*ctx).remote_ip4 as *const _, 3) != 0
        {
            return SK_DROP;
        }
        if LSW(&(*ctx).remote_ip4 as *const _, 0) != 0
            || LSW(&(*ctx).remote_ip4 as *const _, 1) != 0
        {
            return SK_DROP;
        }

        if LSB(&(*ctx).local_ip4 as *const _, 0) != 0
            || LSB(&(*ctx).local_ip4 as *const _, 1) != 0
            || LSB(&(*ctx).local_ip4 as *const _, 2) != 0
            || LSB(&(*ctx).local_ip4 as *const _, 3) != 0
        {
            return SK_DROP;
        }
        if LSW(&(*ctx).local_ip4 as *const _, 0) != 0
            || LSW(&(*ctx).local_ip4 as *const _, 1) != 0
        {
            return SK_DROP;
        }
    }

    /* Narrow loads from IPv6 fields */
    if !v4 {
        /* Expect SRC_IP6 in remote_ip6 */
        if LSB(&(*ctx).remote_ip6[0] as *const _, 0) != ((SRC_IP6[0] >> 0) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[0] as *const _, 1) != ((SRC_IP6[0] >> 8) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[0] as *const _, 2) != ((SRC_IP6[0] >> 16) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[0] as *const _, 3) != ((SRC_IP6[0] >> 24) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[1] as *const _, 0) != ((SRC_IP6[1] >> 0) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[1] as *const _, 1) != ((SRC_IP6[1] >> 8) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[1] as *const _, 2) != ((SRC_IP6[1] >> 16) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[1] as *const _, 3) != ((SRC_IP6[1] >> 24) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[2] as *const _, 0) != ((SRC_IP6[2] >> 0) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[2] as *const _, 1) != ((SRC_IP6[2] >> 8) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[2] as *const _, 2) != ((SRC_IP6[2] >> 16) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[2] as *const _, 3) != ((SRC_IP6[2] >> 24) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[3] as *const _, 0) != ((SRC_IP6[3] >> 0) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[3] as *const _, 1) != ((SRC_IP6[3] >> 8) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[3] as *const _, 2) != ((SRC_IP6[3] >> 16) & 0xff) as __u8
            || LSB(&(*ctx).remote_ip6[3] as *const _, 3) != ((SRC_IP6[3] >> 24) & 0xff) as __u8
        {
            return SK_DROP;
        }
        if LSW(&(*ctx).remote_ip6[0] as *const _, 0) != ((SRC_IP6[0] >> 0) & 0xffff) as __u16
            || LSW(&(*ctx).remote_ip6[0] as *const _, 1) != ((SRC_IP6[0] >> 16) & 0xffff) as __u16
            || LSW(&(*ctx).remote_ip6[1] as *const _, 0) != ((SRC_IP6[1] >> 0) & 0xffff) as __u16
            || LSW(&(*ctx).remote_ip6[1] as *const _, 1) != ((SRC_IP6[1] >> 16) & 0xffff) as __u16
            || LSW(&(*ctx).remote_ip6[2] as *const _, 0) != ((SRC_IP6[2] >> 0) & 0xffff) as __u16
            || LSW(&(*ctx).remote_ip6[2] as *const _, 1) != ((SRC_IP6[2] >> 16) & 0xffff) as __u16
            || LSW(&(*ctx).remote_ip6[3] as *const _, 0) != ((SRC_IP6[3] >> 0) & 0xffff) as __u16
            || LSW(&(*ctx).remote_ip6[3] as *const _, 1) != ((SRC_IP6[3] >> 16) & 0xffff) as __u16
        {
            return SK_DROP;
        }
        /* Expect DST_IP6 in local_ip6 */
        if LSB(&(*ctx).local_ip6[0] as *const _, 0) != ((DST_IP6[0] >> 0) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[0] as *const _, 1) != ((DST_IP6[0] >> 8) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[0] as *const _, 2) != ((DST_IP6[0] >> 16) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[0] as *const _, 3) != ((DST_IP6[0] >> 24) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[1] as *const _, 0) != ((DST_IP6[1] >> 0) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[1] as *const _, 1) != ((DST_IP6[1] >> 8) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[1] as *const _, 2) != ((DST_IP6[1] >> 16) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[1] as *const _, 3) != ((DST_IP6[1] >> 24) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[2] as *const _, 0) != ((DST_IP6[2] >> 0) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[2] as *const _, 1) != ((DST_IP6[2] >> 8) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[2] as *const _, 2) != ((DST_IP6[2] >> 16) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[2] as *const _, 3) != ((DST_IP6[2] >> 24) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[3] as *const _, 0) != ((DST_IP6[3] >> 0) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[3] as *const _, 1) != ((DST_IP6[3] >> 8) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[3] as *const _, 2) != ((DST_IP6[3] >> 16) & 0xff) as __u8
            || LSB(&(*ctx).local_ip6[3] as *const _, 3) != ((DST_IP6[3] >> 24) & 0xff) as __u8
        {
            return SK_DROP;
        }
        if LSW(&(*ctx).local_ip6[0] as *const _, 0) != ((DST_IP6[0] >> 0) & 0xffff) as __u16
            || LSW(&(*ctx).local_ip6[0] as *const _, 1) != ((DST_IP6[0] >> 16) & 0xffff) as __u16
            || LSW(&(*ctx).local_ip6[1] as *const _, 0) != ((DST_IP6[1] >> 0) & 0xffff) as __u16
            || LSW(&(*ctx).local_ip6[1] as *const _, 1) != ((DST_IP6[1] >> 16) & 0xffff) as __u16
            || LSW(&(*ctx).local_ip6[2] as *const _, 0) != ((DST_IP6[2] >> 0) & 0xffff) as __u16
            || LSW(&(*ctx).local_ip6[2] as *const _, 1) != ((DST_IP6[2] >> 16) & 0xffff) as __u16
            || LSW(&(*ctx).local_ip6[3] as *const _, 0) != ((DST_IP6[3] >> 0) & 0xffff) as __u16
            || LSW(&(*ctx).local_ip6[3] as *const _, 1) != ((DST_IP6[3] >> 16) & 0xffff) as __u16
        {
            return SK_DROP;
        }
    } else {
        /* Expect :: IPs when family != AF_INET6 */
        if LSB(&(*ctx).remote_ip6[0] as *const _, 0) != 0
            || LSB(&(*ctx).remote_ip6[0] as *const _, 1) != 0
            || LSB(&(*ctx).remote_ip6[0] as *const _, 2) != 0
            || LSB(&(*ctx).remote_ip6[0] as *const _, 3) != 0
            || LSB(&(*ctx).remote_ip6[1] as *const _, 0) != 0
            || LSB(&(*ctx).remote_ip6[1] as *const _, 1) != 0
            || LSB(&(*ctx).remote_ip6[1] as *const _, 2) != 0
            || LSB(&(*ctx).remote_ip6[1] as *const _, 3) != 0
            || LSB(&(*ctx).remote_ip6[2] as *const _, 0) != 0
            || LSB(&(*ctx).remote_ip6[2] as *const _, 1) != 0
            || LSB(&(*ctx).remote_ip6[2] as *const _, 2) != 0
            || LSB(&(*ctx).remote_ip6[2] as *const _, 3) != 0
            || LSB(&(*ctx).remote_ip6[3] as *const _, 0) != 0
            || LSB(&(*ctx).remote_ip6[3] as *const _, 1) != 0
            || LSB(&(*ctx).remote_ip6[3] as *const _, 2) != 0
            || LSB(&(*ctx).remote_ip6[3] as *const _, 3) != 0
        {
            return SK_DROP;
        }
        if LSW(&(*ctx).remote_ip6[0] as *const _, 0) != 0
            || LSW(&(*ctx).remote_ip6[0] as *const _, 1) != 0
            || LSW(&(*ctx).remote_ip6[1] as *const _, 0) != 0
            || LSW(&(*ctx).remote_ip6[1] as *const _, 1) != 0
            || LSW(&(*ctx).remote_ip6[2] as *const _, 0) != 0
            || LSW(&(*ctx).remote_ip6[2] as *const _, 1) != 0
            || LSW(&(*ctx).remote_ip6[3] as *const _, 0) != 0
            || LSW(&(*ctx).remote_ip6[3] as *const _, 1) != 0
        {
            return SK_DROP;
        }

        if LSB(&(*ctx).local_ip6[0] as *const _, 0) != 0
            || LSB(&(*ctx).local_ip6[0] as *const _, 1) != 0
            || LSB(&(*ctx).local_ip6[0] as *const _, 2) != 0
            || LSB(&(*ctx).local_ip6[0] as *const _, 3) != 0
            || LSB(&(*ctx).local_ip6[1] as *const _, 0) != 0
            || LSB(&(*ctx).local_ip6[1] as *const _, 1) != 0
            || LSB(&(*ctx).local_ip6[1] as *const _, 2) != 0
            || LSB(&(*ctx).local_ip6[1] as *const _, 3) != 0
            || LSB(&(*ctx).local_ip6[2] as *const _, 0) != 0
            || LSB(&(*ctx).local_ip6[2] as *const _, 1) != 0
            || LSB(&(*ctx).local_ip6[2] as *const _, 2) != 0
            || LSB(&(*ctx).local_ip6[2] as *const _, 3) != 0
            || LSB(&(*ctx).local_ip6[3] as *const _, 0) != 0
            || LSB(&(*ctx).local_ip6[3] as *const _, 1) != 0
            || LSB(&(*ctx).local_ip6[3] as *const _, 2) != 0
            || LSB(&(*ctx).local_ip6[3] as *const _, 3) != 0
        {
            return SK_DROP;
        }
        if LSW(&(*ctx).remote_ip6[0] as *const _, 0) != 0
            || LSW(&(*ctx).remote_ip6[0] as *const _, 1) != 0
            || LSW(&(*ctx).remote_ip6[1] as *const _, 0) != 0
            || LSW(&(*ctx).remote_ip6[1] as *const _, 1) != 0
            || LSW(&(*ctx).remote_ip6[2] as *const _, 0) != 0
            || LSW(&(*ctx).remote_ip6[2] as *const _, 1) != 0
            || LSW(&(*ctx).remote_ip6[3] as *const _, 0) != 0
            || LSW(&(*ctx).remote_ip6[3] as *const _, 1) != 0
        {
            return SK_DROP;
        }
    }

    /* Success, redirect to KEY_SERVER_B */
    sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_B as *const _ as *const c_void);
    if !sk.is_null() {
        bpf_sk_assign(ctx, sk, 0);
        bpf_sk_release(sk);
    }
    SK_PASS
}

/* Check that sk_assign rejects SERVER_A socket with -ESOCKNOSUPPORT */
#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sk_assign_esocknosupport(ctx: *mut bpf_sk_lookup) -> i32 {
    let sk: *mut bpf_sock;
    let mut ret: i32;

    ret = SK_DROP;
    sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_A as *const _ as *const c_void);
    if !sk.is_null() {
        let err = bpf_sk_assign(ctx, sk, 0);
        if err != -ESOCKTNOSUPPORT {
            bpf_printk(c"sk_assign returned %d, expected %d\n".as_ptr() as *const u8, err, -ESOCKTNOSUPPORT);
        } else {
            ret = SK_PASS; /* Success, pass to regular lookup */
        }
    }
    if !sk.is_null() {
        bpf_sk_release(sk);
    }
    ret
}

#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multi_prog_pass1(_ctx: *mut bpf_sk_lookup) -> i32 {
    bpf_map_update_elem(
        &run_map as *const _ as *const c_void,
        &KEY_PROG1 as *const _ as *const c_void,
        &PROG_DONE as *const _ as *const c_void,
        BPF_ANY,
    );
    SK_PASS
}

#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multi_prog_pass2(_ctx: *mut bpf_sk_lookup) -> i32 {
    bpf_map_update_elem(
        &run_map as *const _ as *const c_void,
        &KEY_PROG2 as *const _ as *const c_void,
        &PROG_DONE as *const _ as *const c_void,
        BPF_ANY,
    );
    SK_PASS
}

#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multi_prog_drop1(_ctx: *mut bpf_sk_lookup) -> i32 {
    bpf_map_update_elem(
        &run_map as *const _ as *const c_void,
        &KEY_PROG1 as *const _ as *const c_void,
        &PROG_DONE as *const _ as *const c_void,
        BPF_ANY,
    );
    SK_DROP
}

#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multi_prog_drop2(_ctx: *mut bpf_sk_lookup) -> i32 {
    bpf_map_update_elem(
        &run_map as *const _ as *const c_void,
        &KEY_PROG2 as *const _ as *const c_void,
        &PROG_DONE as *const _ as *const c_void,
        BPF_ANY,
    );
    SK_DROP
}

#[inline(always)]
unsafe fn select_server_a(ctx: *mut bpf_sk_lookup) -> i32 {
    let sk: *mut bpf_sock;
    let err: i32;

    sk = bpf_map_lookup_elem(&redir_map as *const _ as *const c_void, &KEY_SERVER_A as *const _ as *const c_void);
    if sk.is_null() {
        return SK_DROP;
    }

    err = bpf_sk_assign(ctx, sk, 0);
    bpf_sk_release(sk);
    if err != 0 {
        return SK_DROP;
    }

    SK_PASS
}

#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multi_prog_redir1(ctx: *mut bpf_sk_lookup) -> i32 {
    let _ = select_server_a(ctx);
    bpf_map_update_elem(
        &run_map as *const _ as *const c_void,
        &KEY_PROG1 as *const _ as *const c_void,
        &PROG_DONE as *const _ as *const c_void,
        BPF_ANY,
    );
    SK_PASS
}

#[unsafe(link_section = "sk_lookup")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multi_prog_redir2(ctx: *mut bpf_sk_lookup) -> i32 {
    let _ = select_server_a(ctx);
    bpf_map_update_elem(
        &run_map as *const _ as *const c_void,
        &KEY_PROG2 as *const _ as *const c_void,
        &PROG_DONE as *const _ as *const c_void,
        BPF_ANY,
    );
    SK_PASS
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 13] = *b"Dual BSD/GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
