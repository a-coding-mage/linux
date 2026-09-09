// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies supplied by the corresponding Linux kernel headers and source:
// `struct nlattr`, `struct netlink_ext_ack`, `nla_get_u8`, `NL_SET_ERR_MSG`,
// protocol/address-family constants, and `EOPNOTSUPP`.

pub unsafe fn rtm_getroute_parse_ip_proto(
    attr: *mut nlattr,
    ip_proto: *mut u8,
    family: u8,
    extack: *mut netlink_ext_ack,
) -> i32 {
    unsafe {
        *ip_proto = nla_get_u8(attr);

        match *ip_proto {
            IPPROTO_TCP | IPPROTO_UDP => return 0,
            IPPROTO_ICMP => {
                if family != AF_INET {
                    // Continue to the common unsupported-protocol path.
                } else {
                    return 0;
                }
            }
            #[cfg(feature = "CONFIG_IPV6")]
            IPPROTO_ICMPV6 => {
                if family != AF_INET6 {
                    // Continue to the common unsupported-protocol path.
                } else {
                    return 0;
                }
            }
            _ => {}
        }

        NL_SET_ERR_MSG!(extack, "Unsupported ip proto");
        -EOPNOTSUPP
    }
}

// EXPORT_SYMBOL_GPL(rtm_getroute_parse_ip_proto);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
