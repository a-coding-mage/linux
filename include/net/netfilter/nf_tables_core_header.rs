/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation unit. */

extern "C" {
    pub static mut nft_imm_type: nft_expr_type;
    pub static mut nft_cmp_type: nft_expr_type;
    pub static mut nft_counter_type: nft_expr_type;
    pub static mut nft_lookup_type: nft_expr_type;
    pub static mut nft_bitwise_type: nft_expr_type;
    pub static mut nft_byteorder_type: nft_expr_type;
    pub static mut nft_payload_type: nft_expr_type;
    pub static mut nft_dynset_type: nft_expr_type;
    pub static mut nft_range_type: nft_expr_type;
    pub static mut nft_meta_type: nft_expr_type;
    pub static mut nft_rt_type: nft_expr_type;
    pub static mut nft_exthdr_type: nft_expr_type;
    pub static mut nft_last_type: nft_expr_type;
    pub static mut nft_objref_type: nft_expr_type;
    pub static mut nft_inner_type: nft_expr_type;

    /* CONFIG_NETWORK_SECMARK */
    #[cfg(feature = "CONFIG_NETWORK_SECMARK")]
    pub static mut nft_secmark_obj_type: nft_object_type;
    pub static mut nft_counter_obj_type: nft_object_type;

    pub fn nf_tables_core_module_init() -> ::core::ffi::c_int;
    pub fn nf_tables_core_module_exit();
}

#[repr(C)]
pub struct nft_bitwise_fast_expr {
    pub mask: u32,
    pub xor_: u32,
    pub sreg: u8,
    pub dreg: u8,
}

#[repr(C)]
pub struct nft_cmp_fast_expr {
    pub data: u32,
    pub mask: u32,
    pub sreg: u8,
    pub len: u8,
    pub inv: bool,
}

#[repr(C)]
pub struct nft_cmp16_fast_expr {
    pub data: nft_data,
    pub mask: nft_data,
    pub sreg: u8,
    pub len: u8,
    pub inv: bool,
}

#[repr(C)]
pub struct nft_immediate_expr {
    pub data: nft_data,
    pub dreg: u8,
    pub dlen: u8,
}

extern "C" {
    pub static nft_cmp_fast_ops: nft_expr_ops;
    pub static nft_cmp16_fast_ops: nft_expr_ops;
}

#[repr(C)]
pub union nft_ct_dreg_sreg {
    pub dreg: u8,
    pub sreg: u8,
}

#[repr(C)]
pub struct nft_ct {
    pub key: nft_ct_keys,
    pub dir: ip_conntrack_dir,
    pub len: u8,
    pub dreg_sreg: nft_ct_dreg_sreg,
}

#[repr(C)]
pub struct nft_payload {
    pub base: nft_payload_bases,
    pub offset: u16,
    pub len: u8,
    pub dreg: u8,
}

extern "C" {
    pub static nft_payload_fast_ops: nft_expr_ops;
    pub static nft_bitwise_fast_ops: nft_expr_ops;
    pub static mut nft_counters_enabled: static_key_false;
    pub static mut nft_trace_enabled: static_key_false;
    pub static nft_set_rhash_type: nft_set_type;
    pub static nft_set_hash_type: nft_set_type;
    pub static nft_set_hash_fast_type: nft_set_type;
    pub static nft_set_rbtree_type: nft_set_type;
    pub static nft_set_bitmap_type: nft_set_type;
    pub static nft_set_pipapo_type: nft_set_type;
    pub static nft_set_pipapo_avx2_type: nft_set_type;

    /* CONFIG_MITIGATION_RETPOLINE */
    #[cfg(feature = "CONFIG_MITIGATION_RETPOLINE")]
    pub fn nft_rhash_lookup(net: *const net, set: *const nft_set, key: *const u32) -> *const nft_set_ext;
    #[cfg(feature = "CONFIG_MITIGATION_RETPOLINE")]
    pub fn nft_rbtree_lookup(net: *const net, set: *const nft_set, key: *const u32) -> *const nft_set_ext;
    #[cfg(feature = "CONFIG_MITIGATION_RETPOLINE")]
    pub fn nft_bitmap_lookup(net: *const net, set: *const nft_set, key: *const u32) -> *const nft_set_ext;
    #[cfg(feature = "CONFIG_MITIGATION_RETPOLINE")]
    pub fn nft_hash_lookup_fast(net: *const net, set: *const nft_set, key: *const u32) -> *const nft_set_ext;
    #[cfg(feature = "CONFIG_MITIGATION_RETPOLINE")]
    pub fn nft_hash_lookup(net: *const net, set: *const nft_set, key: *const u32) -> *const nft_set_ext;

    pub fn nft_set_do_lookup(net: *const net, set: *const nft_set, key: *const u32) -> *const nft_set_ext;
    pub fn nft_pipapo_lookup(net: *const net, set: *const nft_set, key: *const u32) -> *const nft_set_ext;
    pub fn nft_pipapo_avx2_lookup(net: *const net, set: *const nft_set, key: *const u32) -> *const nft_set_ext;
    pub fn nft_counter_init_seqcount();
}

pub enum nft_expr {}
pub enum nft_regs {}
pub enum nft_pktinfo {}

extern "C" {
    pub fn nft_meta_get_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_cmp_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_lookup_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_payload_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_immediate_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_bitwise_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_range_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_byteorder_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_dynset_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_rt_get_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_counter_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_ct_get_fast_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
}

pub const NFT_PAYLOAD_CTX_INNER_TUN: u32 = 1 << 0;
pub const NFT_PAYLOAD_CTX_INNER_LL: u32 = 1 << 1;
pub const NFT_PAYLOAD_CTX_INNER_NH: u32 = 1 << 2;
pub const NFT_PAYLOAD_CTX_INNER_TH: u32 = 1 << 3;

#[repr(C)]
pub struct nft_inner_tun_ctx {
    pub cookie: ::core::ffi::c_ulong,
    pub type_: u16,
    pub inner_tunoff: u16,
    pub inner_lloff: u16,
    pub inner_nhoff: u16,
    pub inner_thoff: u16,
    pub llproto: __be16,
    pub l4proto: u8,
    pub flags: u8,
}

extern "C" {
    pub fn nft_payload_inner_offset(pkt: *const nft_pktinfo) -> ::core::ffi::c_int;
    pub fn nft_payload_inner_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo, ctx: *mut nft_inner_tun_ctx);
    pub fn nft_objref_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_objref_map_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo);
    pub fn nft_dynset_new(set: *mut nft_set, expr: *const nft_expr, regs: *mut nft_regs) -> *mut nft_elem_priv;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
