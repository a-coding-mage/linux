// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

// Linux kernel dependencies supplied by other translation units.

#[cfg(any(feature = "CONFIG_IPV6", feature = "CONFIG_INET"))]
static mut NET_SECRET: siphash_aligned_key_t = siphash_aligned_key_t { key: [0; 16] };

#[cfg(any(feature = "CONFIG_IPV6", feature = "CONFIG_INET"))]
const EPHEMERAL_PORT_SHUFFLE_PERIOD: u64 = 10 * HZ as u64;

#[cfg(any(feature = "CONFIG_IPV6", feature = "CONFIG_INET"))]
#[inline(always)]
unsafe fn net_secret_init() {
    net_get_random_once(
        &mut NET_SECRET as *mut siphash_aligned_key_t as *mut core::ffi::c_void,
        core::mem::size_of::<siphash_aligned_key_t>(),
    );
}

#[cfg(feature = "CONFIG_INET")]
unsafe fn seq_scale(seq: u32) -> u32 {
    /*
     *\tAs close as possible to RFC 793, which
     *\tsuggests using a 250 kHz clock.
     *\tFurther reading shows this assumes 2 Mb/s networks.
     *\tFor 10 Mb/s Ethernet, a 1 MHz clock is appropriate.
     *\tFor 10 Gb/s Ethernet, a 1 GHz clock should be ok, but
     *\twe also need to limit the resolution so that the u32 seq
     *\toverlaps less than one time per MSL (2 minutes).
     *\tChoosing a clock of 64 ns period is OK. (period of 274 s)
     */
    seq.wrapping_add(ktime_get_real_ns() >> 6)
}

#[cfg(feature = "CONFIG_IPV6")]
#[repr(C)]
pub union tcp_seq_and_ts_off {
    pub hash64: u64,
    pub ts_off: u32,
    pub seq: u32,
}

#[cfg(feature = "CONFIG_IPV6")]
pub unsafe extern "C" fn secure_tcpv6_seq_and_ts_off(
    net: *const net,
    saddr: *const __be32,
    daddr: *const __be32,
    sport: __be16,
    dport: __be16,
) -> tcp_seq_and_ts_off {
    #[repr(C, align(8))]
    struct Combined {
        saddr: in6_addr,
        daddr: in6_addr,
        sport: __be16,
        dport: __be16,
    }
    let combined = Combined {
        saddr: *(saddr as *const in6_addr),
        daddr: *(daddr as *const in6_addr),
        sport,
        dport,
    };
    net_secret_init();
    let mut st = tcp_seq_and_ts_off {
        hash64: siphash(
            &combined as *const Combined as *const core::ffi::c_void,
            core::mem::offset_of!(Combined, dport) + core::mem::size_of::<__be16>(),
            &NET_SECRET,
        ),
    };
    if core::ptr::read_volatile(&(*net).ipv4.sysctl_tcp_timestamps) != 1 {
        st.ts_off = 0;
    }
    st.seq = seq_scale(st.seq);
    st
}

#[cfg(feature = "CONFIG_IPV6")]
pub unsafe extern "C" fn secure_ipv6_port_ephemeral(
    saddr: *const __be32,
    daddr: *const __be32,
    dport: __be16,
) -> u64 {
    #[repr(C, align(8))]
    struct Combined {
        saddr: in6_addr,
        daddr: in6_addr,
        timeseed: u32,
        dport: __be16,
    }
    let combined = Combined {
        saddr: *(saddr as *const in6_addr),
        daddr: *(daddr as *const in6_addr),
        timeseed: jiffies / EPHEMERAL_PORT_SHUFFLE_PERIOD,
        dport,
    };
    net_secret_init();
    siphash(
        &combined as *const Combined as *const core::ffi::c_void,
        core::mem::offset_of!(Combined, dport) + core::mem::size_of::<__be16>(),
        &NET_SECRET,
    )
}

#[cfg(feature = "CONFIG_INET")]
/* secure_tcp_seq_and_tsoff(a, b, 0, d) == secure_ipv4_port_ephemeral(a, b, d),
 * but fortunately, `sport' cannot be 0 in any circumstances. If this changes,
 * it would be easy enough to have the former function use siphash_4u32, passing
 * the arguments as separate u32.
 */
pub unsafe extern "C" fn secure_tcp_seq_and_ts_off(
    net: *const net,
    saddr: __be32,
    daddr: __be32,
    sport: __be16,
    dport: __be16,
) -> tcp_seq_and_ts_off {
    let ports = (sport as u32) << 16 | dport as u32;
    net_secret_init();
    let mut st = tcp_seq_and_ts_off {
        hash64: siphash_3u32(saddr as u32, daddr as u32, ports, &NET_SECRET),
    };
    if core::ptr::read_volatile(&(*net).ipv4.sysctl_tcp_timestamps) != 1 {
        st.ts_off = 0;
    }
    st.seq = seq_scale(st.seq);
    st
}

#[cfg(feature = "CONFIG_INET")]
pub unsafe extern "C" fn secure_ipv4_port_ephemeral(
    saddr: __be32,
    daddr: __be32,
    dport: __be16,
) -> u64 {
    net_secret_init();
    siphash_4u32(
        saddr as u32,
        daddr as u32,
        dport as u16 as u32,
        (jiffies / EPHEMERAL_PORT_SHUFFLE_PERIOD) as u32,
        &NET_SECRET,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
