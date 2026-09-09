/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency: <linux/types.h> supplies __u32. */

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Ip6tRejectWith {
    IP6T_ICMP6_NO_ROUTE = 0,
    IP6T_ICMP6_ADM_PROHIBITED = 1,
    IP6T_ICMP6_NOT_NEIGHBOUR = 2,
    IP6T_ICMP6_ADDR_UNREACH = 3,
    IP6T_ICMP6_PORT_UNREACH = 4,
    IP6T_ICMP6_ECHOREPLY = 5,
    IP6T_TCP_RESET = 6,
    IP6T_ICMP6_POLICY_FAIL = 7,
    IP6T_ICMP6_REJECT_ROUTE = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ip6tRejectInfo {
    pub with: u32, /* reject type */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
