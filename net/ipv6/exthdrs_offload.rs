// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *\tIPV6 GSO/GRO offload support
 *\tLinux INET6 implementation
 *
 *      IPV6 Extension Header GSO/GRO support
 */
// Dependency intent from <net/protocol.h> and "ip6_offload.h" is supplied by
// the surrounding kernel translation.

static rthdr_offload: net_offload = net_offload {
    flags: INET6_PROTO_GSO_EXTHDR,
};

static dstopt_offload: net_offload = net_offload {
    flags: INET6_PROTO_GSO_EXTHDR,
};

static hbh_offload: net_offload = net_offload {
    flags: INET6_PROTO_GSO_EXTHDR,
};

extern "C" {
    fn inet6_add_offload(offload: *const net_offload, proto: core::ffi::c_int) -> core::ffi::c_int;
    fn inet6_del_offload(offload: *const net_offload, proto: core::ffi::c_int);
}

// The C __init annotation is a build/link-time initialization attribute.
pub unsafe fn ipv6_exthdrs_offload_init() -> core::ffi::c_int {
    let mut ret: core::ffi::c_int;

    ret = inet6_add_offload(&rthdr_offload, IPPROTO_ROUTING);
    if ret != 0 {
        return ret;
    }

    ret = inet6_add_offload(&dstopt_offload, IPPROTO_DSTOPTS);
    if ret != 0 {
        inet6_del_offload(&rthdr_offload, IPPROTO_ROUTING);
        return ret;
    }

    ret = inet6_add_offload(&hbh_offload, IPPROTO_HOPOPTS);
    if ret != 0 {
        inet6_del_offload(&dstopt_offload, IPPROTO_DSTOPTS);
        inet6_del_offload(&rthdr_offload, IPPROTO_ROUTING);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
