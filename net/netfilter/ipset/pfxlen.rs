// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the kernel headers:
// linux/netfilter/ipset/pfxlen.h and linux/export.h

/* Prefixlen maps for fast conversions, by Jan Engelhardt. */

/* This table works for both IPv4 and IPv6; just use
 * ip_set_netmask_map[prefixlength].ip.
 */
#[allow(non_camel_case_types)]
pub static ip_set_netmask_map: [nf_inet_addr; 129] = make_netmask_map();

/* This table works for both IPv4 and IPv6; just use
 * ip_set_hostmask_map[prefixlength].ip.
 */
#[allow(non_camel_case_types)]
pub static ip_set_hostmask_map: [nf_inet_addr; 129] = make_hostmask_map();

/* The C source exports both tables with EXPORT_SYMBOL_GPL. */

const fn htonl(value: u32) -> u32 {
    value.to_be()
}

const fn netmask_word(prefix: usize, word: usize) -> u32 {
    let bits = prefix.saturating_sub(word * 32);
    if bits >= 32 {
        0xffff_ffff
    } else if bits == 0 {
        0
    } else {
        0xffff_ffffu32 << (32 - bits)
    }
}

const fn make_netmask_map() -> [nf_inet_addr; 129] {
    let mut result = [nf_inet_addr { ip6: [0; 4] }; 129];
    let mut prefix = 0;
    while prefix <= 128 {
        result[prefix] = nf_inet_addr {
            ip6: [
                htonl(netmask_word(prefix, 0)),
                htonl(netmask_word(prefix, 1)),
                htonl(netmask_word(prefix, 2)),
                htonl(netmask_word(prefix, 3)),
            ],
        };
        prefix += 1;
    }
    result
}

const fn make_hostmask_map() -> [nf_inet_addr; 129] {
    let mut result = [nf_inet_addr { ip6: [0; 4] }; 129];
    let mut prefix = 0;
    while prefix <= 128 {
        result[prefix] = nf_inet_addr {
            ip6: [
                netmask_word(prefix, 0),
                netmask_word(prefix, 1),
                netmask_word(prefix, 2),
                netmask_word(prefix, 3),
            ],
        };
        prefix += 1;
    }
    result
}

/* Find the largest network which matches the range from left, in host order. */
pub unsafe fn ip_set_range_to_cidr(from: u32, to: u32, cidr: *mut u8) -> u32 {
    let mut last: u32;
    let mut i: u8 = 1;

    while i < 32 {
        if (from & ip_set_hostmask(i)) != from {
            i = i.wrapping_add(1);
            continue;
        }
        last = from | !ip_set_hostmask(i);
        if !after(last, to) {
            *cidr = i;
            return last;
        }
        i = i.wrapping_add(1);
    }
    *cidr = 32;
    from
}

// External declarations supplied by linux/netfilter/ipset/pfxlen.h.
extern "C" {
    static ip_set_hostmask_map_external: [nf_inet_addr; 129];
    fn ip_set_hostmask(prefix: u8) -> u32;
    fn after(a: u32, b: u32) -> bool;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
