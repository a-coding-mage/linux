/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnl_hook_msg_types {
    NFNL_MSG_HOOK_GET = 0,
    NFNL_MSG_HOOK_MAX = 1,
}

/**
 * enum nfnl_hook_attributes - netfilter hook netlink attributes
 *
 * @NFNLA_HOOK_HOOKNUM: netfilter hook number (NLA_U32)
 * @NFNLA_HOOK_PRIORITY: netfilter hook priority (NLA_U32)
 * @NFNLA_HOOK_DEV: netdevice name (NLA_STRING)
 * @NFNLA_HOOK_FUNCTION_NAME: hook function name (NLA_STRING)
 * @NFNLA_HOOK_MODULE_NAME: kernel module that registered this hook (NLA_STRING)
 * @NFNLA_HOOK_CHAIN_INFO: basechain hook metadata (NLA_NESTED)
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnl_hook_attributes {
    NFNLA_HOOK_UNSPEC = 0,
    NFNLA_HOOK_HOOKNUM = 1,
    NFNLA_HOOK_PRIORITY = 2,
    NFNLA_HOOK_DEV = 3,
    NFNLA_HOOK_FUNCTION_NAME = 4,
    NFNLA_HOOK_MODULE_NAME = 5,
    NFNLA_HOOK_CHAIN_INFO = 6,
    __NFNLA_HOOK_MAX = 7,
}
pub const NFNLA_HOOK_MAX: nfnl_hook_attributes = nfnl_hook_attributes::__NFNLA_HOOK_MAX;

/**
 * enum nfnl_hook_chain_info_attributes - chain description
 *
 * @NFNLA_HOOK_INFO_DESC: nft chain and table name (NLA_NESTED)
 * @NFNLA_HOOK_INFO_TYPE: chain type (enum nfnl_hook_chaintype) (NLA_U32)
 *
 * NFNLA_HOOK_INFO_DESC depends on NFNLA_HOOK_INFO_TYPE value:
 *   NFNL_HOOK_TYPE_NFTABLES: enum nft_table_attributes
 *   NFNL_HOOK_TYPE_BPF: enum nfnl_hook_bpf_attributes
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnl_hook_chain_info_attributes {
    NFNLA_HOOK_INFO_UNSPEC = 0,
    NFNLA_HOOK_INFO_DESC = 1,
    NFNLA_HOOK_INFO_TYPE = 2,
    __NFNLA_HOOK_INFO_MAX = 3,
}
pub const NFNLA_HOOK_INFO_MAX: nfnl_hook_chain_info_attributes =
    nfnl_hook_chain_info_attributes::__NFNLA_HOOK_INFO_MAX;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnl_hook_chain_desc_attributes {
    NFNLA_CHAIN_UNSPEC = 0,
    NFNLA_CHAIN_TABLE = 1,
    NFNLA_CHAIN_FAMILY = 2,
    NFNLA_CHAIN_NAME = 3,
    __NFNLA_CHAIN_MAX = 4,
}
pub const NFNLA_CHAIN_MAX: nfnl_hook_chain_desc_attributes =
    nfnl_hook_chain_desc_attributes::__NFNLA_CHAIN_MAX;

/**
 * enum nfnl_hook_chaintype - chain type
 *
 * @NFNL_HOOK_TYPE_NFTABLES: nf_tables base chain
 * @NFNL_HOOK_TYPE_BPF: bpf program
 * @NFNL_HOOK_TYPE_NFT_FLOWTABLE: nf_tables flowtable
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnl_hook_chaintype {
    NFNL_HOOK_TYPE_NFTABLES = 0x1,
    NFNL_HOOK_TYPE_BPF = 0x2,
    NFNL_HOOK_TYPE_NFT_FLOWTABLE = 0x3,
}

/**
 * enum nfnl_hook_bpf_attributes - bpf prog description
 *
 * @NFNLA_HOOK_BPF_ID: bpf program id (NLA_U32)
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nfnl_hook_bpf_attributes {
    NFNLA_HOOK_BPF_UNSPEC = 0,
    NFNLA_HOOK_BPF_ID = 1,
    __NFNLA_HOOK_BPF_MAX = 2,
}
pub const NFNLA_HOOK_BPF_MAX: nfnl_hook_bpf_attributes =
    nfnl_hook_bpf_attributes::__NFNLA_HOOK_BPF_MAX;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
