/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2017-2018 Covalent IO, Inc. http://covalent.io */

/*
 * Translated from the C header test_sockmap_kern.h.
 *
 * Original dependency intent:
 *   <stddef.h>, <string.h>, Linux/BPF networking headers,
 *   <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>, and "bpf_misc.h".
 */

type __u32 = u32;
type __u64 = u64;

unsafe extern "C" {
    static TEST_MAP_TYPE: u32;
    static BPF_MAP_TYPE_ARRAY: u32;
    static BPF_NOEXIST: __u64;
    static BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB: u32;
    static BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB: u32;
    static SK_DROP: i32;
    static SK_PASS: i32;
}

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
    pub data: __u32,
    pub data_end: __u32,
    pub local_port: __u32,
    pub remote_port: __u32,
}

#[repr(C)]
pub struct bpf_sock_ops {
    pub op: __u32,
    pub local_port: __u32,
    pub remote_port: __u32,
}

#[repr(C)]
pub struct sk_msg_md {
    pub data: __u64,
    pub data_end: __u64,
}

#[repr(C)]
pub struct bpf_map_def_int_int {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[repr(C)]
pub struct bpf_map_def_typed_int_int {
    pub type_: u32,
    pub max_entries: u32,
}

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_sk_redirect_map(
        skb: *mut __sk_buff,
        map: *mut core::ffi::c_void,
        key: i32,
        flags: __u64,
    ) -> i32;
    fn bpf_sk_redirect_hash(
        skb: *mut __sk_buff,
        map: *mut core::ffi::c_void,
        key: *const i32,
        flags: __u64,
    ) -> i32;
    fn bpf_sock_map_update(
        skops: *mut bpf_sock_ops,
        map: *mut core::ffi::c_void,
        key: *const i32,
        flags: __u64,
    ) -> i32;
    fn bpf_sock_hash_update(
        skops: *mut bpf_sock_ops,
        map: *mut core::ffi::c_void,
        key: *const i32,
        flags: __u64,
    ) -> i32;
    fn bpf_msg_apply_bytes(msg: *mut sk_msg_md, bytes: i32) -> i32;
    fn bpf_msg_cork_bytes(msg: *mut sk_msg_md, bytes: i32) -> i32;
    fn bpf_msg_pull_data(msg: *mut sk_msg_md, start: i32, end: i32, flags: i32) -> i32;
    fn bpf_msg_push_data(msg: *mut sk_msg_md, start: i32, len: i32, flags: i32) -> i32;
    fn bpf_msg_pop_data(msg: *mut sk_msg_md, start: i32, len: i32, flags: i32) -> i32;
    fn bpf_msg_redirect_map(
        msg: *mut sk_msg_md,
        map: *mut core::ffi::c_void,
        key: i32,
        flags: __u64,
    ) -> i32;
    fn bpf_msg_redirect_hash(
        msg: *mut sk_msg_md,
        map: *mut core::ffi::c_void,
        key: *const i32,
        flags: __u64,
    ) -> i32;
    fn bpf_ntohl(x: __u32) -> __u32;
    fn __sink<T>(x: T);
}

/*
 * Sockmap sample program connects a client and a backend together
 * using cgroups.
 *
 *    client:X <---> frontend:80 client:X <---> backend:80
 *
 * For simplicity we hard code values here and bind 1:1. The hard
 * coded values are part of the setup in sockmap.sh script that
 * is associated with this BPF program.
 *
 * The bpf_printk is verbose and prints information as connections
 * are established and verdicts are decided.
 */

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_map: bpf_map_def_int_int = bpf_map_def_int_int {
    type_: unsafe { TEST_MAP_TYPE },
    max_entries: 20,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_map_txmsg: bpf_map_def_int_int = bpf_map_def_int_int {
    type_: unsafe { TEST_MAP_TYPE },
    max_entries: 20,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_map_redir: bpf_map_def_int_int = bpf_map_def_int_int {
    type_: unsafe { TEST_MAP_TYPE },
    max_entries: 20,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_apply_bytes: bpf_map_def_typed_int_int = bpf_map_def_typed_int_int {
    type_: unsafe { BPF_MAP_TYPE_ARRAY },
    max_entries: 1,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_cork_bytes: bpf_map_def_typed_int_int = bpf_map_def_typed_int_int {
    type_: unsafe { BPF_MAP_TYPE_ARRAY },
    max_entries: 1,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_bytes: bpf_map_def_typed_int_int = bpf_map_def_typed_int_int {
    type_: unsafe { BPF_MAP_TYPE_ARRAY },
    max_entries: 6,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_redir_flags: bpf_map_def_typed_int_int = bpf_map_def_typed_int_int {
    type_: unsafe { BPF_MAP_TYPE_ARRAY },
    max_entries: 1,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_skb_opts: bpf_map_def_typed_int_int = bpf_map_def_typed_int_int {
    type_: unsafe { BPF_MAP_TYPE_ARRAY },
    max_entries: 3,
};

#[no_mangle]
#[link_section = "sk_skb/stream_parser"]
pub unsafe extern "C" fn bpf_prog1(skb: *mut __sk_buff) -> i32 {
    let mut f: *mut i32;
    let two: i32 = 2;

    f = bpf_map_lookup_elem(
        &mut sock_skb_opts as *mut _ as *mut core::ffi::c_void,
        &two as *const _ as *const core::ffi::c_void,
    ) as *mut i32;
    if !f.is_null() && *f != 0 {
        return *f;
    }
    (*skb).len as i32
}

#[no_mangle]
#[link_section = "sk_skb/stream_verdict"]
pub unsafe extern "C" fn bpf_prog2(skb: *mut __sk_buff) -> i32 {
    let lport: __u32 = (*skb).local_port;
    let rport: __u32 = (*skb).remote_port;
    let len: i32;
    let mut f: *mut i32;
    let mut ret: i32;
    let zero: i32 = 0;
    let mut flags: __u64 = 0;

    __sink(rport);
    if lport == 10000 {
        ret = 10;
    } else {
        ret = 1;
    }

    len = ((*skb).data_end as __u32).wrapping_sub((*skb).data as __u32) as i32;
    __sink(len);

    f = bpf_map_lookup_elem(
        &mut sock_skb_opts as *mut _ as *mut core::ffi::c_void,
        &zero as *const _ as *const core::ffi::c_void,
    ) as *mut i32;
    if !f.is_null() && *f != 0 {
        ret = 3;
        flags = *f as __u64;
    }

    #[cfg(feature = "SOCKMAP")]
    {
        return bpf_sk_redirect_map(
            skb,
            &mut sock_map as *mut _ as *mut core::ffi::c_void,
            ret,
            flags,
        );
    }
    #[cfg(not(feature = "SOCKMAP"))]
    {
        return bpf_sk_redirect_hash(
            skb,
            &mut sock_map as *mut _ as *mut core::ffi::c_void,
            &ret,
            flags,
        );
    }
}

#[no_mangle]
#[link_section = "sockops"]
pub unsafe extern "C" fn bpf_sockmap(skops: *mut bpf_sock_ops) -> i32 {
    let mut lport: __u32;
    let mut rport: __u32;
    let op: i32;
    let mut ret: i32;

    op = (*skops).op as i32;

    match op as u32 {
        x if x == BPF_SOCK_OPS_PASSIVE_ESTABLISHED_CB => {
            lport = (*skops).local_port;
            rport = (*skops).remote_port;

            if lport == 10000 {
                ret = 1;
                #[cfg(feature = "SOCKMAP")]
                {
                    bpf_sock_map_update(
                        skops,
                        &mut sock_map as *mut _ as *mut core::ffi::c_void,
                        &ret,
                        BPF_NOEXIST,
                    );
                }
                #[cfg(not(feature = "SOCKMAP"))]
                {
                    bpf_sock_hash_update(
                        skops,
                        &mut sock_map as *mut _ as *mut core::ffi::c_void,
                        &ret,
                        BPF_NOEXIST,
                    );
                }
            }
            let _ = rport;
        }
        x if x == BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB => {
            lport = (*skops).local_port;
            rport = (*skops).remote_port;

            if bpf_ntohl(rport) == 10001 {
                ret = 10;
                #[cfg(feature = "SOCKMAP")]
                {
                    bpf_sock_map_update(
                        skops,
                        &mut sock_map as *mut _ as *mut core::ffi::c_void,
                        &ret,
                        BPF_NOEXIST,
                    );
                }
                #[cfg(not(feature = "SOCKMAP"))]
                {
                    bpf_sock_hash_update(
                        skops,
                        &mut sock_map as *mut _ as *mut core::ffi::c_void,
                        &ret,
                        BPF_NOEXIST,
                    );
                }
            }
            let _ = lport;
        }
        _ => {}
    }

    0
}

#[no_mangle]
#[link_section = "sk_msg"]
pub unsafe extern "C" fn bpf_prog4(msg: *mut sk_msg_md) -> i32 {
    let zero: i32 = 0;
    let one: i32 = 1;
    let two: i32 = 2;
    let three: i32 = 3;
    let four: i32 = 4;
    let five: i32 = 5;
    let mut bytes: *mut i32;
    let mut start: *mut i32;
    let mut end: *mut i32;
    let mut start_push: *mut i32;
    let mut end_push: *mut i32;
    let mut start_pop: *mut i32;
    let mut pop: *mut i32;
    let mut err: i32 = 0;

    bytes = bpf_map_lookup_elem(&mut sock_apply_bytes as *mut _ as *mut core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void) as *mut i32;
    if !bytes.is_null() {
        bpf_msg_apply_bytes(msg, *bytes);
    }
    bytes = bpf_map_lookup_elem(&mut sock_cork_bytes as *mut _ as *mut core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void) as *mut i32;
    if !bytes.is_null() {
        bpf_msg_cork_bytes(msg, *bytes);
    }
    start = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void) as *mut i32;
    end = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &one as *const _ as *const core::ffi::c_void) as *mut i32;
    if !start.is_null() && !end.is_null() {
        bpf_msg_pull_data(msg, *start, *end, 0);
    }
    start_push = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &two as *const _ as *const core::ffi::c_void) as *mut i32;
    end_push = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &three as *const _ as *const core::ffi::c_void) as *mut i32;
    if !start_push.is_null() && !end_push.is_null() {
        err = bpf_msg_push_data(msg, *start_push, *end_push, 0);
        if err != 0 {
            return SK_DROP;
        }
    }
    start_pop = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &four as *const _ as *const core::ffi::c_void) as *mut i32;
    pop = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &five as *const _ as *const core::ffi::c_void) as *mut i32;
    if !start_pop.is_null() && !pop.is_null() {
        bpf_msg_pop_data(msg, *start_pop, *pop, 0);
    }
    SK_PASS
}

#[no_mangle]
#[link_section = "sk_msg"]
pub unsafe extern "C" fn bpf_prog6(msg: *mut sk_msg_md) -> i32 {
    let zero: i32 = 0;
    let one: i32 = 1;
    let two: i32 = 2;
    let three: i32 = 3;
    let four: i32 = 4;
    let five: i32 = 5;
    let mut key: i32 = 0;
    let mut bytes: *mut i32;
    let mut start: *mut i32;
    let mut end: *mut i32;
    let mut start_push: *mut i32;
    let mut end_push: *mut i32;
    let mut start_pop: *mut i32;
    let mut pop: *mut i32;
    let mut f: *mut i32;
    let mut err: i32 = 0;
    let mut flags: __u64 = 0;

    bytes = bpf_map_lookup_elem(&mut sock_apply_bytes as *mut _ as *mut core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void) as *mut i32;
    if !bytes.is_null() {
        bpf_msg_apply_bytes(msg, *bytes);
    }
    bytes = bpf_map_lookup_elem(&mut sock_cork_bytes as *mut _ as *mut core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void) as *mut i32;
    if !bytes.is_null() {
        bpf_msg_cork_bytes(msg, *bytes);
    }

    start = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void) as *mut i32;
    end = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &one as *const _ as *const core::ffi::c_void) as *mut i32;
    if !start.is_null() && !end.is_null() {
        bpf_msg_pull_data(msg, *start, *end, 0);
    }

    start_push = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &two as *const _ as *const core::ffi::c_void) as *mut i32;
    end_push = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &three as *const _ as *const core::ffi::c_void) as *mut i32;
    if !start_push.is_null() && !end_push.is_null() {
        err = bpf_msg_push_data(msg, *start_push, *end_push, 0);
        if err != 0 {
            return SK_DROP;
        }
    }

    start_pop = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &four as *const _ as *const core::ffi::c_void) as *mut i32;
    pop = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &five as *const _ as *const core::ffi::c_void) as *mut i32;
    if !start_pop.is_null() && !pop.is_null() {
        bpf_msg_pop_data(msg, *start_pop, *pop, 0);
    }

    f = bpf_map_lookup_elem(&mut sock_redir_flags as *mut _ as *mut core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void) as *mut i32;
    if !f.is_null() && *f != 0 {
        key = 2;
        flags = *f as __u64;
    }
    #[cfg(feature = "SOCKMAP")]
    {
        return bpf_msg_redirect_map(
            msg,
            &mut sock_map_redir as *mut _ as *mut core::ffi::c_void,
            key,
            flags,
        );
    }
    #[cfg(not(feature = "SOCKMAP"))]
    {
        return bpf_msg_redirect_hash(
            msg,
            &mut sock_map_redir as *mut _ as *mut core::ffi::c_void,
            &key,
            flags,
        );
    }
}

#[no_mangle]
#[link_section = "sk_msg"]
pub unsafe extern "C" fn bpf_prog8(msg: *mut sk_msg_md) -> i32 {
    let data_end: *mut core::ffi::c_void = (*msg).data_end as usize as *mut core::ffi::c_void;
    let data: *mut core::ffi::c_void = (*msg).data as usize as *mut core::ffi::c_void;
    let mut ret: i32 = 0;
    let mut bytes: *mut i32;
    let zero: i32 = 0;

    bytes = bpf_map_lookup_elem(&mut sock_apply_bytes as *mut _ as *mut core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void) as *mut i32;
    if !bytes.is_null() {
        ret = bpf_msg_apply_bytes(msg, *bytes);
        if ret != 0 {
            return SK_DROP;
        }
    } else {
        return SK_DROP;
    }

    __sink(data_end);
    __sink(data);

    SK_PASS
}

#[no_mangle]
#[link_section = "sk_msg"]
pub unsafe extern "C" fn bpf_prog9(msg: *mut sk_msg_md) -> i32 {
    let data_end: *mut core::ffi::c_void = (*msg).data_end as usize as *mut core::ffi::c_void;
    let data: *mut core::ffi::c_void = (*msg).data as usize as *mut core::ffi::c_void;
    let mut ret: i32 = 0;
    let mut bytes: *mut i32;
    let zero: i32 = 0;

    bytes = bpf_map_lookup_elem(&mut sock_cork_bytes as *mut _ as *mut core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void) as *mut i32;
    if !bytes.is_null() {
        if ((data_end as __u64).wrapping_sub(data as __u64)) >= *bytes as __u64 {
            return SK_PASS;
        }
        ret = bpf_msg_cork_bytes(msg, *bytes);
        if ret != 0 {
            return SK_DROP;
        }
    }
    SK_PASS
}

#[no_mangle]
#[link_section = "sk_msg"]
pub unsafe extern "C" fn bpf_prog10(msg: *mut sk_msg_md) -> i32 {
    let mut bytes: *mut i32;
    let mut start: *mut i32;
    let mut end: *mut i32;
    let mut start_push: *mut i32;
    let mut end_push: *mut i32;
    let mut start_pop: *mut i32;
    let mut pop: *mut i32;
    let zero: i32 = 0;
    let one: i32 = 1;
    let two: i32 = 2;
    let three: i32 = 3;
    let four: i32 = 4;
    let five: i32 = 5;
    let mut err: i32 = 0;

    bytes = bpf_map_lookup_elem(&mut sock_apply_bytes as *mut _ as *mut core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void) as *mut i32;
    if !bytes.is_null() {
        bpf_msg_apply_bytes(msg, *bytes);
    }
    bytes = bpf_map_lookup_elem(&mut sock_cork_bytes as *mut _ as *mut core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void) as *mut i32;
    if !bytes.is_null() {
        bpf_msg_cork_bytes(msg, *bytes);
    }
    start = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void) as *mut i32;
    end = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &one as *const _ as *const core::ffi::c_void) as *mut i32;
    if !start.is_null() && !end.is_null() {
        bpf_msg_pull_data(msg, *start, *end, 0);
    }
    start_push = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &two as *const _ as *const core::ffi::c_void) as *mut i32;
    end_push = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &three as *const _ as *const core::ffi::c_void) as *mut i32;
    if !start_push.is_null() && !end_push.is_null() {
        err = bpf_msg_push_data(msg, *start_push, *end_push, 0);
        if err != 0 {
            return SK_PASS;
        }
    }
    start_pop = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &four as *const _ as *const core::ffi::c_void) as *mut i32;
    pop = bpf_map_lookup_elem(&mut sock_bytes as *mut _ as *mut core::ffi::c_void, &five as *const _ as *const core::ffi::c_void) as *mut i32;
    if !start_pop.is_null() && !pop.is_null() {
        bpf_msg_pop_data(msg, *start_pop, *pop, 0);
    }
    SK_DROP
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
