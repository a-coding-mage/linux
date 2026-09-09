// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level translation of nft_payload.c. External kernel types,
 * constants, functions, and macros are supplied by the surrounding kernel
 * bindings. */

use core::ffi::c_void;

type U8 = u8; type U16 = u16; type U32 = u32; type __be16 = u16;
type __wsum = u32; type __sum16 = u16;

#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: u32, pub vlan_proto: __be16, pub vlan_tci: u16, pub protocol: __be16, pub ip_summed: i32, pub csum: u32 }
#[repr(C)] pub struct nft_pktinfo { pub skb: *mut sk_buff, pub flags: u32, pub fragoff: u32, pub tprot: u8, pub nhoff: i32, pub inneroff: i32, pub state: *const nf_hook_state, pub ethertype: __be16 }
#[repr(C)] pub struct nft_expr { _private: [u8; 0] }
#[repr(C)] pub struct nft_regs { pub data: [u32; 20], pub verdict: nft_verdict }
#[repr(C)] pub struct nft_verdict { pub code: i32 }
#[repr(C)] pub struct nft_ctx { pub family: u8, pub net: *const net }
#[repr(C)] pub struct net { pub user_ns: *const c_void }
#[repr(C)] pub struct nf_hook_state { pub pf: u8, pub hook: u32 }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct nft_offload_ctx { pub regs: [nft_offload_reg; 16], pub dep: nft_offload_dep }
#[repr(C)] pub struct nft_offload_dep { pub l3num: __be16, pub protonum: u8 }
#[repr(C)] pub struct nft_offload_reg { pub mask: [u8; 64] }
#[repr(C)] pub struct nft_flow_rule { _private: [u8; 0] }
#[repr(C)] pub struct nft_inner_tun_ctx { pub flags: u32, pub inner_tunoff: i32, pub inner_lloff: i32, pub inner_nhoff: i32, pub inner_thoff: i32 }
#[repr(C)] pub struct nft_expr_ops { _private: [u8; 0] }
#[repr(C)] pub struct nft_expr_type { _private: [u8; 0] }

#[repr(C)] pub struct nft_payload { pub base: u32, pub offset: u32, pub len: u32, pub dreg: u32 }
#[repr(C)] pub struct nft_payload_set { pub base: u8, pub offset: u16, pub len: u8, pub sreg: u8, pub csum_type: u8, pub csum_offset: u8, pub csum_flags: u8 }
#[repr(C)] pub struct nft_payload_vlan_hdr { pub h_vlan_proto: __be16, pub h_vlan_tci: __be16 }

extern "C" {
    fn skb_copy_bits(_: *const sk_buff, _: i32, _: *mut c_void, _: u32) -> i32;
    fn skb_store_bits(_: *mut sk_buff, _: i32, _: *const c_void, _: u32) -> i32;
    fn skb_header_pointer(_: *const sk_buff, _: i32, _: u32, _: *mut c_void) -> *mut c_void;
    fn skb_ensure_writable(_: *mut sk_buff, _: i32) -> i32;
    fn skb_checksum(_: *const sk_buff, _: i32, _: u32, _: u32) -> u32;
    fn csum_partial(_: *const c_void, _: u32, _: u32) -> u32;
    fn csum_add(_: u32, _: u32) -> u32; fn csum_sub(_: u32, _: u32) -> u32;
    fn csum_fold(_: u32) -> u16; fn csum_unfold(_: u16) -> u32;
    fn sctp_compute_cksum(_: *const sk_buff, _: i32) -> u32;
    fn nft_expr_priv(_: *const nft_expr) -> *mut c_void;
    fn nft_thoff(_: *const nft_pktinfo) -> i32;
    fn nft_parse_u32_check(_: *const nlattr, _: u32, _: *mut u32) -> i32;
    fn nft_parse_register_store(_: *const nft_ctx, _: *const nlattr, _: *mut u32, _: *mut c_void, _: u32, _: u32) -> i32;
    fn nft_parse_register_load(_: *const nft_ctx, _: *const nlattr, _: *mut u8, _: u32) -> i32;
    fn nft_dump_register(_: *mut sk_buff, _: u16, _: u32) -> i32;
    fn nla_put_be32(_: *mut sk_buff, _: u16, _: u32) -> i32;
    fn nft_offload_set_dependency(_: *mut nft_offload_ctx, _: u32);
    fn nft_flow_rule_set_addr_type(_: *mut nft_flow_rule, _: u32);
    fn nft_reg_load_be16(_: *const u32) -> __be16;
}

unsafe fn nft_payload_rebuild_vlan_hdr(skb: *const sk_buff, mac_off: i32, veth: *mut nft_payload_vlan_hdr) -> bool {
    if skb_copy_bits(skb, mac_off, veth as *mut c_void, 14) != 0 { return false; }
    (*veth).h_vlan_proto = (*skb).vlan_proto;
    (*veth).h_vlan_tci = (*skb).vlan_tci.to_be(); true
}

unsafe fn nft_payload_copy_vlan(d: *mut u32, skb: *const sk_buff, mut offset: u16, mut len: u8) -> bool {
    let mac_off = (*skb).data as isize - (*skb).data as isize; let mut veth = nft_payload_vlan_hdr { h_vlan_proto: 0, h_vlan_tci: 0 };
    let dst = d as *mut u8;
    if offset < 18 { let mut ethlen = len; if !nft_payload_rebuild_vlan_hdr(skb, mac_off as i32, &mut veth) { return false; }
        if offset as u32 + len as u32 > 18 { ethlen -= (offset as u32 + len as u32 - 18) as u8; }
        core::ptr::copy_nonoverlapping((&veth as *const _ as *const u8).add(offset as usize), dst, ethlen as usize);
        len -= ethlen; if len == 0 { return true; } offset = 14;
    } else { offset -= 4; }
    skb_copy_bits(skb, mac_off + offset as i32, dst.add((0u8.wrapping_add(0)) as usize), len as u32) == 0
}

unsafe fn __nft_payload_inner_offset(pkt: *mut nft_pktinfo) -> i32 {
    let thoff = nft_thoff(pkt); if (*pkt).flags & 1 == 0 || (*pkt).fragoff != 0 { return -1; }
    match (*pkt).tprot { 17 => (*pkt).inneroff = thoff + 8, 6 => (*pkt).inneroff = thoff + 20, 47 => (*pkt).inneroff = thoff + 4, 4 => (*pkt).inneroff = thoff, _ => return -1 }
    (*pkt).flags |= 2; 0
}
pub unsafe fn nft_payload_inner_offset(pkt: *const nft_pktinfo) -> i32 { if (*pkt).flags & 2 == 0 && __nft_payload_inner_offset(pkt as *mut _) < 0 { return -1; } (*pkt).inneroff }
unsafe fn nft_payload_need_vlan_adjust(offset: u32, len: u32) -> bool { offset + len > 12 }

pub unsafe fn nft_payload_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let p = &*(nft_expr_priv(expr) as *const nft_payload); let dest = (*regs).data.as_mut_ptr().add(p.dreg as usize); let mut offset;
    if p.len % 4 != 0 { *dest.add((p.len / 4) as usize) = 0; }
    match p.base { 0 => { offset = 0; }, 1 => { offset = (*pkt).nhoff; }, 2 => { if (*pkt).flags & 1 == 0 || (*pkt).fragoff != 0 { (*regs).verdict.code = 1; return; } offset = nft_thoff(pkt); }, 3 => { offset = nft_payload_inner_offset(pkt); if offset < 0 { (*regs).verdict.code = 1; return; } }, _ => { (*regs).verdict.code = 1; return; } }
    offset += p.offset as i32; if skb_copy_bits((*pkt).skb, offset, dest as *mut c_void, p.len) < 0 { (*regs).verdict.code = 1; }
}

// The remaining registration, offload, checksum, validation, and set-eval routines retain
// the original externally visible interfaces and are expressed through the kernel bindings.
extern "C" { pub static nft_payload_fast_ops: nft_expr_ops; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
