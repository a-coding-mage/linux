/* SPDX-License-Identifier: GPL-2.0 */
/* Shared Memory Communications over RDMA (SMC-R) and RoCE. */
/* CLC (connection layer control) handshake over initial TCP socket. */

pub const SMC_CLC_PROPOSAL: u8 = 0x01;
pub const SMC_CLC_ACCEPT: u8 = 0x02;
pub const SMC_CLC_CONFIRM: u8 = 0x03;
pub const SMC_CLC_DECLINE: u8 = 0x04;

pub const SMC_TYPE_R: i32 = 0;
pub const SMC_TYPE_D: i32 = 1;
pub const SMC_TYPE_N: i32 = 2;
pub const SMC_TYPE_B: i32 = 3;
pub const CLC_WAIT_TIME: usize = 6 * HZ;
pub const CLC_WAIT_TIME_SHORT: usize = HZ;

pub const SMC_CLC_DECL_MEM: u32 = 0x01010000;
pub const SMC_CLC_DECL_TIMEOUT_CL: u32 = 0x02010000;
pub const SMC_CLC_DECL_TIMEOUT_AL: u32 = 0x02020000;
pub const SMC_CLC_DECL_CNFERR: u32 = 0x03000000;
pub const SMC_CLC_DECL_PEERNOSMC: u32 = 0x03010000;
pub const SMC_CLC_DECL_IPSEC: u32 = 0x03020000;
pub const SMC_CLC_DECL_NOSMCDEV: u32 = 0x03030000;
pub const SMC_CLC_DECL_NOSMCDDEV: u32 = 0x03030001;
pub const SMC_CLC_DECL_NOSMCRDEV: u32 = 0x03030002;
pub const SMC_CLC_DECL_NOISM2SUPP: u32 = 0x03030003;
pub const SMC_CLC_DECL_NOV2EXT: u32 = 0x03030004;
pub const SMC_CLC_DECL_NOV2DEXT: u32 = 0x03030005;
pub const SMC_CLC_DECL_NOSEID: u32 = 0x03030006;
pub const SMC_CLC_DECL_NOSMCD2DEV: u32 = 0x03030007;
pub const SMC_CLC_DECL_NOUEID: u32 = 0x03030008;
pub const SMC_CLC_DECL_RELEASEERR: u32 = 0x03030009;
pub const SMC_CLC_DECL_MAXCONNERR: u32 = 0x0303000a;
pub const SMC_CLC_DECL_MAXLINKERR: u32 = 0x0303000b;
pub const SMC_CLC_DECL_MODEUNSUPP: u32 = 0x03040000;
pub const SMC_CLC_DECL_RMBE_EC: u32 = 0x03050000;
pub const SMC_CLC_DECL_OPTUNSUPP: u32 = 0x03060000;
pub const SMC_CLC_DECL_DIFFPREFIX: u32 = 0x03070000;
pub const SMC_CLC_DECL_GETVLANERR: u32 = 0x03080000;
pub const SMC_CLC_DECL_ISMVLANERR: u32 = 0x03090000;
pub const SMC_CLC_DECL_NOACTLINK: u32 = 0x030a0000;
pub const SMC_CLC_DECL_NOSRVLINK: u32 = 0x030b0000;
pub const SMC_CLC_DECL_VERSMISMAT: u32 = 0x030c0000;
pub const SMC_CLC_DECL_MAX_DMB: u32 = 0x030d0000;
pub const SMC_CLC_DECL_NOROUTE: u32 = 0x030e0000;
pub const SMC_CLC_DECL_NOINDIRECT: u32 = 0x030f0000;
pub const SMC_CLC_DECL_SYNCERR: u32 = 0x04000000;
pub const SMC_CLC_DECL_PEERDECL: u32 = 0x05000000;
pub const SMC_CLC_DECL_INTERR: u32 = 0x09990000;
pub const SMC_CLC_DECL_ERR_RTOK: u32 = 0x09990001;
pub const SMC_CLC_DECL_ERR_RDYLNK: u32 = 0x09990002;
pub const SMC_CLC_DECL_ERR_REGBUF: u32 = 0x09990003;
pub const SMC_FIRST_CONTACT_MASK: u8 = 0b10;

#[repr(C, packed)]
pub struct smc_clc_msg_hdr {
    pub eyecatcher: [u8; 4], pub r#type: u8, pub length: u16,
    pub version: u8, pub typev2: u8, pub typev1: u8,
}
#[repr(C)] pub struct smc_clc_msg_trail { pub eyecatcher: [u8; 4] }
#[repr(C)] pub struct smc_clc_msg_local { pub id_for_peer: [u8; SMC_SYSTEMID_LEN], pub gid: [u8; 16], pub mac: [u8; 6] }
#[repr(C, packed)] pub struct smc_clc_ipv6_prefix { pub prefix: in6_addr, pub prefix_len: u8 }
#[repr(C)] pub struct smc_clc_v2_flag { pub release: u8, pub rsvd: u8, pub seid: u8 }
#[repr(C)] pub struct smc_clnt_opts_area_hdr { pub eid_cnt: u8, pub ism_gid_cnt: u8, pub reserved1: u8, pub flag: smc_clc_v2_flag, pub reserved2: [u8; 2], pub smcd_v2_ext_offset: u16 }
#[repr(C, packed)] pub struct smc_clc_smcd_gid_chid { pub gid: u64, pub chid: u16 }
#[repr(C)] pub struct smc_clc_v2_extension_fixed { pub hdr: smc_clnt_opts_area_hdr, pub roce: [u8; 16], pub max_conns: u8, pub max_links: u8, pub feature_mask: u16, pub reserved: [u8; 12] }
#[repr(C)] pub struct smc_clc_v2_extension { pub fixed: smc_clc_v2_extension_fixed, pub user_eids: [u8; 0] }
#[repr(C, align(4))] pub struct smc_clc_msg_proposal_prefix { pub outgoing_subnet: u32, pub prefix_len: u8, pub reserved: [u8; 2], pub ipv6_prefixes_cnt: u8 }
#[repr(C)] pub struct smc_clc_msg_smcd { pub ism: smc_clc_smcd_gid_chid, pub v2_ext_offset: u16, pub vendor_oui: [u8; 3], pub vendor_exp_options: [u8; 5], pub reserved: [u8; 20] }
#[repr(C)] pub struct smc_clc_smcd_v2_extension_fixed { pub system_eid: [u8; SMC_MAX_EID_LEN], pub reserved: [u8; 16] }
#[repr(C)] pub struct smc_clc_smcd_v2_extension { pub fixed: smc_clc_smcd_v2_extension_fixed, pub gidchid: [smc_clc_smcd_gid_chid; 0] }
#[repr(C, align(4))] pub struct smc_clc_msg_proposal { pub hdr: smc_clc_msg_hdr, pub lcl: smc_clc_msg_local, pub iparea_offset: u16 }
pub const SMC_CLC_MAX_V6_PREFIX: usize = 8; pub const SMC_CLC_MAX_UEID: usize = 8; pub const SMCD_CLC_MAX_V2_GID_ENTRIES: usize = 8;
#[repr(C)] pub struct smc_clc_msg_proposal_area { pub pclc_base: smc_clc_msg_proposal, pub pclc_smcd: smc_clc_msg_smcd, pub pclc_prfx: smc_clc_msg_proposal_prefix, pub pclc_prfx_ipv6: [smc_clc_ipv6_prefix; SMC_CLC_MAX_V6_PREFIX], pub pclc_v2_ext: smc_clc_v2_extension_fixed, pub user_eids: [[u8; SMC_MAX_EID_LEN]; SMC_CLC_MAX_UEID], pub pclc_smcd_v2_ext: smc_clc_smcd_v2_extension_fixed, pub pclc_gidchids: [smc_clc_smcd_gid_chid; SMCD_CLC_MAX_V2_GID_ENTRIES], pub pclc_trl: smc_clc_msg_trail }
#[repr(C, packed)] pub struct smcr_clc_msg_accept_confirm { pub lcl: smc_clc_msg_local, pub qpn: [u8; 3], pub rmb_rkey: u32, pub rmbe_idx: u8, pub rmbe_alert_token: u32, pub rmbe_size: u8, pub qp_mtu: u8, pub reserved: u8, pub rmb_dma_addr: u64, pub reserved2: u8, pub psn: [u8; 3] }
#[repr(C, packed)] pub struct smcd_clc_msg_accept_confirm_common { pub gid: u64, pub token: u64, pub dmbe_idx: u8, pub dmbe_size: u8, pub reserved4: u16, pub linkid: u32 }
pub const SMC_CLC_OS_ZOS: u8 = 1; pub const SMC_CLC_OS_LINUX: u8 = 2; pub const SMC_CLC_OS_AIX: u8 = 3;
#[repr(C)] pub struct smc_clc_first_contact_ext { pub v2_direct: u8, pub reserved: u8, pub os_type: u8, pub release: u8, pub reserved2: [u8; 2], pub hostname: [u8; SMC_MAX_HOSTNAME_LEN] }
#[repr(C, packed)] pub struct smc_clc_first_contact_ext_v2x { pub fce_v2_base: smc_clc_first_contact_ext, pub max_conns: u8, pub max_links: u8, pub feature_mask: u16, pub vendor_exp_options: u32, pub reserved4: [u8; 8] }
#[repr(C)] pub struct smc_clc_fce_gid_ext { pub gid_cnt: u8, pub reserved2: [u8; 3], pub gid: [u8; 0] }
#[repr(C)] pub union smc_clc_msg_accept_confirm_union { pub r0: smcr_clc_msg_accept_confirm, pub d0: smcd_clc_msg_accept_confirm_common }
#[repr(C)] pub struct smc_clc_msg_accept_confirm { pub hdr: smc_clc_msg_hdr, pub data: smc_clc_msg_accept_confirm_union }
#[repr(C, align(4))] pub struct smc_clc_msg_decline { pub hdr: smc_clc_msg_hdr, pub id_for_peer: [u8; SMC_SYSTEMID_LEN], pub peer_diagnosis: u32, pub os_type: u8, pub reserved2: [u8; 3], pub trl: smc_clc_msg_trail }
pub const SMC_DECL_DIAG_COUNT_V2: usize = 4;
#[repr(C, align(4))] pub struct smc_clc_msg_decline_v2 { pub hdr: smc_clc_msg_hdr, pub id_for_peer: [u8; SMC_SYSTEMID_LEN], pub peer_diagnosis: u32, pub os_type: u8, pub reserved2: [u8; 3], pub peer_diagnosis_v2: [u32; SMC_DECL_DIAG_COUNT_V2], pub trl: smc_clc_msg_trail }

pub unsafe fn smc_clc_proposal_get_prefix(pclc: *mut smc_clc_msg_proposal) -> *mut smc_clc_msg_proposal_prefix { let offset = u16::from_be((*pclc).iparea_offset); if offset as usize > core::mem::size_of::<smc_clc_msg_smcd>() { core::ptr::null_mut() } else { (pclc.cast::<u8>().add(core::mem::size_of::<smc_clc_msg_proposal>() + offset as usize)).cast() } }
pub fn smcr_indicated(smc_type: i32) -> bool { smc_type == SMC_TYPE_R || smc_type == SMC_TYPE_B }
pub fn smcd_indicated(smc_type: i32) -> bool { smc_type == SMC_TYPE_D || smc_type == SMC_TYPE_B }
pub fn smc_indicated_type(is_smcd: i32, is_smcr: i32) -> u8 { if is_smcd != 0 && is_smcr != 0 { SMC_TYPE_B as u8 } else if is_smcd != 0 { SMC_TYPE_D as u8 } else if is_smcr != 0 { SMC_TYPE_R as u8 } else { SMC_TYPE_N as u8 } }
pub unsafe fn smc_get_clc_msg_smcd(prop: *mut smc_clc_msg_proposal) -> *mut smc_clc_msg_smcd { if smcd_indicated((*prop).hdr.typev1 as i32) && u16::from_be((*prop).iparea_offset) as usize != core::mem::size_of::<smc_clc_msg_smcd>() { core::ptr::null_mut() } else { prop.add(1).cast() } }
pub unsafe fn smc_get_clc_v2_ext(prop: *mut smc_clc_msg_proposal) -> *mut smc_clc_v2_extension {
    let prop_smcd = smc_get_clc_msg_smcd(prop);
    if prop_smcd.is_null() { return core::ptr::null_mut(); }
    let off = u16::from_be((*prop_smcd).v2_ext_offset);
    let max_offset = core::mem::size_of::<smc_clc_msg_proposal_area>()
        - core::mem::size_of::<smc_clc_msg_proposal>()
        - core::mem::size_of::<u16>();
    if off == 0 || off as usize > max_offset { return core::ptr::null_mut(); }
    (prop_smcd.cast::<u8>().add(core::mem::size_of::<smc_clc_smcd>() + off as usize)).cast()
}
pub unsafe fn smc_get_clc_smcd_v2_ext(prop_v2ext: *mut smc_clc_v2_extension) -> *mut smc_clc_smcd_v2_extension {
    if prop_v2ext.is_null() { return core::ptr::null_mut(); }
    let off = u16::from_be((*prop_v2ext).fixed.hdr.smcd_v2_ext_offset);
    let max_offset = core::mem::size_of::<smc_clc_msg_proposal_area>()
        - core::mem::size_of::<smc_clc_v2_extension_fixed>()
        - core::mem::size_of::<u16>();
    if off == 0 || off as usize > max_offset { return core::ptr::null_mut(); }
    (prop_v2ext.cast::<u8>().add(core::mem::size_of::<smc_clnt_opts_area_hdr>() + core::mem::size_of::<u16>() + off as usize)).cast()
}
pub unsafe fn smc_get_clc_first_contact_ext(clc: *mut smc_clc_msg_accept_confirm, is_smcd: bool) -> *mut smc_clc_first_contact_ext {
    if (*clc).hdr.version == SMC_V1 || ((*clc).hdr.typev2 & SMC_FIRST_CONTACT_MASK) == 0 { return core::ptr::null_mut(); }
    let clc_v2_len = core::mem::size_of::<smc_clc_msg_accept_confirm>();
    let _ = is_smcd;
    clc.cast::<u8>().add(clc_v2_len).cast()
}

extern "C" {
    pub fn smc_clc_prfx_match(clcsock: *mut socket, prop: *mut smc_clc_msg_proposal_prefix) -> i32;
    pub fn smc_clc_wait_msg(smc: *mut smc_sock, buf: *mut core::ffi::c_void, buflen: i32, expected_type: u8, timeout: usize) -> i32;
    pub fn smc_clc_send_decline(smc: *mut smc_sock, peer_diag_info: u32, version: u8) -> i32;
    pub fn smc_clc_send_proposal(smc: *mut smc_sock, ini: *mut smc_init_info) -> i32;
    pub fn smc_clc_send_confirm(smc: *mut smc_sock, clnt_first_contact: bool, version: u8, eid: *mut u8, ini: *mut smc_init_info) -> i32;
    pub fn smc_clc_send_accept(smc: *mut smc_sock, srv_first_contact: bool, version: u8, negotiated_eid: *mut u8, ini: *mut smc_init_info) -> i32;
    pub fn smc_clc_srv_v2x_features_validate(smc: *mut smc_sock, pclc: *mut smc_clc_msg_proposal, ini: *mut smc_init_info) -> i32;
    pub fn smc_clc_clnt_v2x_features_validate(fce: *mut smc_clc_first_contact_ext, ini: *mut smc_init_info) -> i32;
    pub fn smc_clc_v2x_features_confirm_check(cclc: *mut smc_clc_msg_accept_confirm, ini: *mut smc_init_info) -> i32;
    pub fn smc_clc_init(); pub fn smc_clc_exit(); pub fn smc_clc_get_hostname(host: *mut *mut u8);
    pub fn smc_clc_match_eid(negotiated_eid: *mut u8, smc_v2_ext: *mut smc_clc_v2_extension, peer_eid: *mut u8, local_eid: *mut u8) -> bool;
    pub fn smc_clc_ueid_count() -> i32;
    pub fn smc_nl_dump_ueid(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn smc_nl_add_ueid(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn smc_nl_remove_ueid(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn smc_nl_flush_ueid(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn smc_nl_dump_seid(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
    pub fn smc_nl_enable_seid(skb: *mut sk_buff, info: *mut genl_info) -> i32;
    pub fn smc_nl_disable_seid(skb: *mut sk_buff, info: *mut genl_info) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
