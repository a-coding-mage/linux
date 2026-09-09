// Translated from nf_tables_offload.h.
// Dependencies provided by the surrounding kernel translation are referenced
// here but are not defined in this header.

#[repr(u32)]
pub enum nft_offload_reg_flags {
    NFT_OFFLOAD_F_NETWORK2HOST = 1 << 0,
}

#[repr(C)]
pub struct nft_offload_reg {
    pub key: u32,
    pub len: u32,
    pub base_offset: u32,
    pub offset: u32,
    pub flags: u32,
    pub data: nft_data,
    pub mask: nft_data,
}

#[repr(i32)]
pub enum nft_offload_dep_type {
    NFT_OFFLOAD_DEP_UNSPEC = 0,
    NFT_OFFLOAD_DEP_NETWORK,
    NFT_OFFLOAD_DEP_TRANSPORT,
}

#[repr(C)]
pub struct nft_offload_dep {
    pub r#type: nft_offload_dep_type,
    pub l3num: __be16,
    pub protonum: u8,
}

#[repr(C)]
pub struct nft_offload_ctx {
    pub dep: nft_offload_dep,
    pub num_actions: ::core::ffi::c_uint,
    pub net: *mut net,
    pub regs: [nft_offload_reg; (NFT_REG32_15 + 1) as usize],
}

extern "C" {
    pub fn nft_offload_set_dependency(ctx: *mut nft_offload_ctx,
                                      r#type: nft_offload_dep_type);
    pub fn nft_offload_update_dependency(ctx: *mut nft_offload_ctx,
                                         data: *const ::core::ffi::c_void,
                                         len: u32);
}

#[repr(C)]
pub union nft_flow_key_addresses {
    pub ipv4: flow_dissector_key_ipv4_addrs,
    pub ipv6: flow_dissector_key_ipv6_addrs,
}

#[repr(C, align(8))]
pub struct nft_flow_key {
    pub basic: flow_dissector_key_basic,
    pub control: flow_dissector_key_control,
    pub addresses: nft_flow_key_addresses,
    pub tp: flow_dissector_key_ports,
    pub ip: flow_dissector_key_ip,
    pub vlan: flow_dissector_key_vlan,
    pub cvlan: flow_dissector_key_vlan,
    pub eth_addrs: flow_dissector_key_eth_addrs,
    pub meta: flow_dissector_key_meta,
}

#[repr(C)]
pub struct nft_flow_match {
    pub dissector: flow_dissector,
    pub key: nft_flow_key,
    pub mask: nft_flow_key,
}

#[repr(C)]
pub struct nft_flow_rule {
    pub proto: __be16,
    pub r#match: nft_flow_match,
    pub rule: *mut flow_rule,
}

#[inline]
pub unsafe fn nft_flow_action_entry_next(
    ctx: *mut nft_offload_ctx,
    flow: *mut nft_flow_rule,
) -> *mut flow_action_entry {
    if (*ctx).num_actions >= (*(*flow).rule).action.num_entries {
        return core::ptr::null_mut();
    }

    let entry = (*(*flow).rule).action.entries.add((*ctx).num_actions as usize);
    (*ctx).num_actions = (*ctx).num_actions.wrapping_add(1);
    entry
}

extern "C" {
    pub fn nft_flow_rule_set_addr_type(flow: *mut nft_flow_rule,
                                       addr_type: flow_dissector_key_id);
    pub fn nft_flow_rule_create(net: *mut net, rule: *const nft_rule) -> *mut nft_flow_rule;
    pub fn nft_flow_rule_stats(chain: *const nft_chain, rule: *const nft_rule) -> i32;
    pub fn nft_flow_rule_destroy(flow: *mut nft_flow_rule);
    pub fn nft_flow_rule_offload_commit(net: *mut net) -> i32;
}

#[macro_export]
macro_rules! NFT_OFFLOAD_MATCH_FLAGS {
    ($key:expr, $base:ident, $field:ident, $len:expr, $reg:expr, $flags:expr) => {{
        (*$reg).base_offset = core::mem::offset_of!(nft_flow_key, $base) as u32;
        (*$reg).offset =
            (core::mem::offset_of!(nft_flow_key, $base) +
             core::mem::offset_of!($base, $field)) as u32;
        (*$reg).len = $len;
        (*$reg).key = $key;
        (*$reg).flags = $flags;
    }};
}

#[macro_export]
macro_rules! NFT_OFFLOAD_MATCH {
    ($key:expr, $base:ident, $field:ident, $len:expr, $reg:expr) => {
        NFT_OFFLOAD_MATCH_FLAGS!($key, $base, $field, $len, $reg, 0)
    };
}

#[macro_export]
macro_rules! NFT_OFFLOAD_MATCH_EXACT {
    ($key:expr, $base:ident, $field:ident, $len:expr, $reg:expr) => {{
        NFT_OFFLOAD_MATCH!($key, $base, $field, $len, $reg);
        core::ptr::write_bytes((*$reg).mask as *mut nft_data, 0xff, (*$reg).len as usize);
    }};
}

extern "C" {
    pub fn nft_chain_offload_support(basechain: *const nft_base_chain) -> bool;
    pub fn nft_offload_init() -> i32;
    pub fn nft_offload_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
