// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/* Dependencies in the original C source:
 * #include <vmlinux.h>
 * #include "bpf_tracing_net.h"
 * #include <bpf/bpf_helpers.h>
 */

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    #[link_name = "CONFIG_IPV6_SUBTREES"]
    static CONFIG_IPV6_SUBTREES: bool;
}

extern "C" {
    fn BPF_SEQ_PRINTF(seq: *mut seq_file, fmt: *const u8, ...);
}

#[link_section = "iter/ipv6_route"]
pub unsafe extern "C" fn dump_ipv6_route(ctx: *mut bpf_iter__ipv6_route) -> i32 {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let rt: *mut fib6_info = (*ctx).rt;
    let mut dev: *const net_device;
    let mut fib6_nh: *mut fib6_nh;
    let mut flags: ::core::ffi::c_uint;
    let nh: *mut nexthop;

    if rt == 0 as *mut fib6_info {
        return 0;
    }

    fib6_nh = &mut (*rt).fib6_nh[0] as *mut fib6_nh;
    flags = (*rt).fib6_flags;

    /* FIXME: nexthop_is_multipath is not handled here. */
    nh = (*rt).nh;
    if !(*rt).nh.is_null() {
        fib6_nh = &mut (*(*nh).nh_info).fib6_nh as *mut fib6_nh;
    }

    BPF_SEQ_PRINTF(
        seq,
        b"%pi6 %02x \0".as_ptr(),
        &(*rt).fib6_dst.addr as *const _,
        (*rt).fib6_dst.plen,
    );

    if CONFIG_IPV6_SUBTREES {
        BPF_SEQ_PRINTF(
            seq,
            b"%pi6 %02x \0".as_ptr(),
            &(*rt).fib6_src.addr as *const _,
            (*rt).fib6_src.plen,
        );
    } else {
        BPF_SEQ_PRINTF(
            seq,
            b"00000000000000000000000000000000 00 \0".as_ptr(),
        );
    }

    if (*fib6_nh).fib_nh_gw_family != 0 {
        flags |= RTF_GATEWAY;
        BPF_SEQ_PRINTF(
            seq,
            b"%pi6 \0".as_ptr(),
            &(*fib6_nh).fib_nh_gw6 as *const _,
        );
    } else {
        BPF_SEQ_PRINTF(
            seq,
            b"00000000000000000000000000000000 \0".as_ptr(),
        );
    }

    dev = (*fib6_nh).fib_nh_dev;
    if !dev.is_null() {
        BPF_SEQ_PRINTF(
            seq,
            b"%08x %08x %08x %08x %8s\n\0".as_ptr(),
            (*rt).fib6_metric,
            (*rt).fib6_ref.refs.counter,
            0,
            flags,
            (*dev).name.as_ptr(),
        );
    } else {
        BPF_SEQ_PRINTF(
            seq,
            b"%08x %08x %08x %08x\n\0".as_ptr(),
            (*rt).fib6_metric,
            (*rt).fib6_ref.refs.counter,
            0,
            flags,
        );
    }

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
