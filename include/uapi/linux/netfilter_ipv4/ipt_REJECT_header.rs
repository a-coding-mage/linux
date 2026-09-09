/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// _IPT_REJECT_H

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ipt_reject_with {
    IPT_ICMP_NET_UNREACHABLE,
    IPT_ICMP_HOST_UNREACHABLE,
    IPT_ICMP_PROT_UNREACHABLE,
    IPT_ICMP_PORT_UNREACHABLE,
    IPT_ICMP_ECHOREPLY,
    IPT_ICMP_NET_PROHIBITED,
    IPT_ICMP_HOST_PROHIBITED,
    IPT_TCP_RESET,
    IPT_ICMP_ADMIN_PROHIBITED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipt_reject_info {
    pub with: ipt_reject_with, // reject type
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
