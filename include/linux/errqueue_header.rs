/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding C headers:
// net/ip.h, linux/ipv6.h (when CONFIG_IPV6 is enabled), and
// uapi/linux/errqueue.h.

// #define SKB_EXT_ERR(skb) ((struct sock_exterr_skb *) ((skb)->cb))
#[inline]
pub unsafe fn skb_ext_err(skb: *mut sk_buff) -> *mut sock_exterr_skb {
    // The C macro casts the socket buffer control buffer to sock_exterr_skb.
    (*skb).cb.as_mut_ptr() as *mut sock_exterr_skb
}

#[repr(C)]
pub union sock_exterr_skb_header {
    pub h4: inet_skb_parm,
    // Equivalent to IS_ENABLED(CONFIG_IPV6); enabled when the CONFIG_IPV6
    // feature is present in the consuming build.
    #[cfg(feature = "CONFIG_IPV6")]
    pub h6: inet6_skb_parm,
}

#[repr(C)]
pub struct sock_exterr_skb {
    pub header: sock_exterr_skb_header,
    pub ee: sock_extended_err,
    pub addr_offset: u16,
    pub port: __be16,
    // C bit-fields: opt_stats:1, unused:7.
    pub opt_stats_unused: u8,
}

impl sock_exterr_skb {
    #[inline]
    pub fn opt_stats(&self) -> u8 {
        self.opt_stats_unused & 1
    }

    #[inline]
    pub fn set_opt_stats(&mut self, value: u8) {
        self.opt_stats_unused = (self.opt_stats_unused & !1) | (value & 1);
    }

    #[inline]
    pub fn unused(&self) -> u8 {
        self.opt_stats_unused >> 1
    }

    #[inline]
    pub fn set_unused(&mut self, value: u8) {
        self.opt_stats_unused = (self.opt_stats_unused & 1) | ((value & 0x7f) << 1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
