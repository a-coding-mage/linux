// Dependency intent from <linux/if_ether.h> and <linux/types.h> is preserved
// through the externally supplied Ethernet constants and byte-order helper.

/* One byte protocol values as defined by VXLAN-GPE and NSH. These will
 * hopefully get a shared IANA registry.
 */
pub const TUN_P_IPV4: u8 = 0x01;
pub const TUN_P_IPV6: u8 = 0x02;
pub const TUN_P_ETHERNET: u8 = 0x03;
pub const TUN_P_NSH: u8 = 0x04;
pub const TUN_P_MPLS_UC: u8 = 0x05;

#[inline]
pub fn tun_p_to_eth_p(proto: u8) -> u16 {
    match proto {
        TUN_P_IPV4 => htons(ETH_P_IP),
        TUN_P_IPV6 => htons(ETH_P_IPV6),
        TUN_P_ETHERNET => htons(ETH_P_TEB),
        TUN_P_NSH => htons(ETH_P_NSH),
        TUN_P_MPLS_UC => htons(ETH_P_MPLS_UC),
        _ => 0,
    }
}

#[inline]
pub fn tun_p_from_eth_p(proto: u16) -> u8 {
    if proto == htons(ETH_P_IP) {
        TUN_P_IPV4
    } else if proto == htons(ETH_P_IPV6) {
        TUN_P_IPV6
    } else if proto == htons(ETH_P_TEB) {
        TUN_P_ETHERNET
    } else if proto == htons(ETH_P_NSH) {
        TUN_P_NSH
    } else if proto == htons(ETH_P_MPLS_UC) {
        TUN_P_MPLS_UC
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
