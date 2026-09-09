/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

/* Message types - V1 */
pub const PIM_V1_VERSION: __be32 = 0x10000000u32.to_be();
pub const PIM_V1_REGISTER: i32 = 1;

/* Message types - V2 */
pub const PIM_VERSION: i32 = 2;

/* RFC7761, sec 4.9:
 *  Type
 *        Types for specific PIM messages.  PIM Types are:
 *
 *  Message Type                          Destination
 *  ---------------------------------------------------------------------
 *  0 = Hello                             Multicast to ALL-PIM-ROUTERS
 *  1 = Register                          Unicast to RP
 *  2 = Register-Stop                     Unicast to source of Register
 *                                        packet
 *  3 = Join/Prune                        Multicast to ALL-PIM-ROUTERS
 *  4 = Bootstrap                         Multicast to ALL-PIM-ROUTERS
 *  5 = Assert                            Multicast to ALL-PIM-ROUTERS
 *  6 = Graft (used in PIM-DM only)       Unicast to RPF'(S)
 *  7 = Graft-Ack (used in PIM-DM only)   Unicast to source of Graft
 *                                        packet
 *  8 = Candidate-RP-Advertisement        Unicast to Domain's BSR
 */
pub const PIM_TYPE_HELLO: i32 = 0;
pub const PIM_TYPE_REGISTER: i32 = 1;
pub const PIM_TYPE_REGISTER_STOP: i32 = 2;
pub const PIM_TYPE_JOIN_PRUNE: i32 = 3;
pub const PIM_TYPE_BOOTSTRAP: i32 = 4;
pub const PIM_TYPE_ASSERT: i32 = 5;
pub const PIM_TYPE_GRAFT: i32 = 6;
pub const PIM_TYPE_GRAFT_ACK: i32 = 7;
pub const PIM_TYPE_CANDIDATE_RP_ADV: i32 = 8;

pub const PIM_NULL_REGISTER: __be32 = 0x40000000u32.to_be();

/* RFC7761, sec 4.9:
 * The PIM header common to all PIM messages is:
 *   0                   1                   2                   3
 *   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
 *  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 *  |PIM Ver| Type  |   Reserved    |           Checksum            |
 *  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 */
#[repr(C)]
pub struct pimhdr {
    pub type_: __u8,
    pub reserved: __u8,
    pub csum: __be16,
}

/* PIMv2 register message header layout (ietf-draft-idmr-pimvsm-v2-00.ps */
#[repr(C)]
pub struct pimreghdr {
    pub type_: __u8,
    pub reserved: __u8,
    pub csum: __be16,
    pub flags: __be32,
}

extern "C" {
    pub fn pim_rcv_v1(skb: *mut sk_buff) -> core::ffi::c_int;
}

#[inline]
pub fn ipmr_pimsm_enabled() -> bool {
    /* Build-time equivalent of IS_BUILTIN(CONFIG_IP_PIMSM_V1/V2). */
    cfg!(any(feature = "CONFIG_IP_PIMSM_V1", feature = "CONFIG_IP_PIMSM_V2"))
}

#[inline]
pub unsafe fn pim_hdr(skb: *const sk_buff) -> *mut pimhdr {
    skb_transport_header(skb) as *mut pimhdr
}

#[inline]
pub unsafe fn pim_hdr_version(pimhdr: *const pimhdr) -> __u8 {
    (*pimhdr).type_ >> 4
}

#[inline]
pub unsafe fn pim_hdr_type(pimhdr: *const pimhdr) -> __u8 {
    (*pimhdr).type_ & 0xf
}

/* check if the address is 224.0.0.13, RFC7761 sec 4.3.1 */
#[inline]
pub fn pim_ipv4_all_pim_routers(addr: __be32) -> bool {
    addr == 0xE000000Du32.to_be()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
