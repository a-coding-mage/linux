/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translated from the C UAPI header. The linux/types.h dependency is supplied externally.

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lwtunnel_encap_types {
    LWTUNNEL_ENCAP_NONE = 0,
    LWTUNNEL_ENCAP_MPLS,
    LWTUNNEL_ENCAP_IP,
    LWTUNNEL_ENCAP_ILA,
    LWTUNNEL_ENCAP_IP6,
    LWTUNNEL_ENCAP_SEG6,
    LWTUNNEL_ENCAP_BPF,
    LWTUNNEL_ENCAP_SEG6_LOCAL,
    LWTUNNEL_ENCAP_RPL,
    LWTUNNEL_ENCAP_IOAM6,
    LWTUNNEL_ENCAP_XFRM,
    __LWTUNNEL_ENCAP_MAX,
}

pub const LWTUNNEL_ENCAP_MAX: i32 = __LWTUNNEL_ENCAP_MAX as i32 - 1;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lwtunnel_ip_t {
    LWTUNNEL_IP_UNSPEC = 0,
    LWTUNNEL_IP_ID,
    LWTUNNEL_IP_DST,
    LWTUNNEL_IP_SRC,
    LWTUNNEL_IP_TTL,
    LWTUNNEL_IP_TOS,
    LWTUNNEL_IP_FLAGS,
    LWTUNNEL_IP_PAD,
    LWTUNNEL_IP_OPTS,
    __LWTUNNEL_IP_MAX,
}

pub const LWTUNNEL_IP_MAX: i32 = __LWTUNNEL_IP_MAX as i32 - 1;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lwtunnel_ip6_t {
    LWTUNNEL_IP6_UNSPEC = 0,
    LWTUNNEL_IP6_ID,
    LWTUNNEL_IP6_DST,
    LWTUNNEL_IP6_SRC,
    LWTUNNEL_IP6_HOPLIMIT,
    LWTUNNEL_IP6_TC,
    LWTUNNEL_IP6_FLAGS,
    LWTUNNEL_IP6_PAD,
    LWTUNNEL_IP6_OPTS,
    __LWTUNNEL_IP6_MAX,
}

pub const LWTUNNEL_IP6_MAX: i32 = __LWTUNNEL_IP6_MAX as i32 - 1;

pub const LWTUNNEL_IP_OPTS_UNSPEC: i32 = 0;
pub const LWTUNNEL_IP_OPTS_GENEVE: i32 = 1;
pub const LWTUNNEL_IP_OPTS_VXLAN: i32 = 2;
pub const LWTUNNEL_IP_OPTS_ERSPAN: i32 = 3;
pub const __LWTUNNEL_IP_OPTS_MAX: i32 = 4;
pub const LWTUNNEL_IP_OPTS_MAX: i32 = __LWTUNNEL_IP_OPTS_MAX - 1;

pub const LWTUNNEL_IP_OPT_GENEVE_UNSPEC: i32 = 0;
pub const LWTUNNEL_IP_OPT_GENEVE_CLASS: i32 = 1;
pub const LWTUNNEL_IP_OPT_GENEVE_TYPE: i32 = 2;
pub const LWTUNNEL_IP_OPT_GENEVE_DATA: i32 = 3;
pub const __LWTUNNEL_IP_OPT_GENEVE_MAX: i32 = 4;
pub const LWTUNNEL_IP_OPT_GENEVE_MAX: i32 = __LWTUNNEL_IP_OPT_GENEVE_MAX - 1;

pub const LWTUNNEL_IP_OPT_VXLAN_UNSPEC: i32 = 0;
pub const LWTUNNEL_IP_OPT_VXLAN_GBP: i32 = 1;
pub const __LWTUNNEL_IP_OPT_VXLAN_MAX: i32 = 2;
pub const LWTUNNEL_IP_OPT_VXLAN_MAX: i32 = __LWTUNNEL_IP_OPT_VXLAN_MAX - 1;

pub const LWTUNNEL_IP_OPT_ERSPAN_UNSPEC: i32 = 0;
pub const LWTUNNEL_IP_OPT_ERSPAN_VER: i32 = 1;
pub const LWTUNNEL_IP_OPT_ERSPAN_INDEX: i32 = 2;
pub const LWTUNNEL_IP_OPT_ERSPAN_DIR: i32 = 3;
pub const LWTUNNEL_IP_OPT_ERSPAN_HWID: i32 = 4;
pub const __LWTUNNEL_IP_OPT_ERSPAN_MAX: i32 = 5;
pub const LWTUNNEL_IP_OPT_ERSPAN_MAX: i32 = __LWTUNNEL_IP_OPT_ERSPAN_MAX - 1;

pub const LWT_BPF_PROG_UNSPEC: i32 = 0;
pub const LWT_BPF_PROG_FD: i32 = 1;
pub const LWT_BPF_PROG_NAME: i32 = 2;
pub const __LWT_BPF_PROG_MAX: i32 = 3;
pub const LWT_BPF_PROG_MAX: i32 = __LWT_BPF_PROG_MAX - 1;

pub const LWT_BPF_UNSPEC: i32 = 0;
pub const LWT_BPF_IN: i32 = 1;
pub const LWT_BPF_OUT: i32 = 2;
pub const LWT_BPF_XMIT: i32 = 3;
pub const LWT_BPF_XMIT_HEADROOM: i32 = 4;
pub const __LWT_BPF_MAX: i32 = 5;
pub const LWT_BPF_MAX: i32 = __LWT_BPF_MAX - 1;

pub const LWT_BPF_MAX_HEADROOM: i32 = 256;

pub const LWT_XFRM_UNSPEC: i32 = 0;
pub const LWT_XFRM_IF_ID: i32 = 1;
pub const LWT_XFRM_LINK: i32 = 2;
pub const __LWT_XFRM_MAX: i32 = 3;
pub const LWT_XFRM_MAX: i32 = __LWT_XFRM_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
