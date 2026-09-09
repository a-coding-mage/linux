/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * PTP 1588 support
 *
 * This file implements a BPF that recognizes PTP event messages.
 *
 * Copyright (C) 2010 OMICRON electronics GmbH
 */

// Dependencies supplied by the surrounding kernel translation.

pub const PTP_CLASS_NONE: u32 = 0x00; /* not a PTP event message */
pub const PTP_CLASS_V1: u32 = 0x01; /* protocol version 1 */
pub const PTP_CLASS_V2: u32 = 0x02; /* protocol version 2 */
pub const PTP_CLASS_VMASK: u32 = 0x0f; /* max protocol version is 15 */
pub const PTP_CLASS_IPV4: u32 = 0x10; /* event in an IPV4 UDP packet */
pub const PTP_CLASS_IPV6: u32 = 0x20; /* event in an IPV6 UDP packet */
pub const PTP_CLASS_L2: u32 = 0x40; /* event in a L2 packet */
pub const PTP_CLASS_PMASK: u32 = 0x70; /* mask for the packet type field */
pub const PTP_CLASS_VLAN: u32 = 0x80; /* event in a VLAN tagged packet */

pub const PTP_CLASS_V1_IPV4: u32 = PTP_CLASS_V1 | PTP_CLASS_IPV4;
pub const PTP_CLASS_V1_IPV6: u32 = PTP_CLASS_V1 | PTP_CLASS_IPV6; /* probably DNE */
pub const PTP_CLASS_V2_IPV4: u32 = PTP_CLASS_V2 | PTP_CLASS_IPV4;
pub const PTP_CLASS_V2_IPV6: u32 = PTP_CLASS_V2 | PTP_CLASS_IPV6;
pub const PTP_CLASS_V2_L2: u32 = PTP_CLASS_V2 | PTP_CLASS_L2;
pub const PTP_CLASS_V2_VLAN: u32 = PTP_CLASS_V2 | PTP_CLASS_VLAN;
pub const PTP_CLASS_L4: u32 = PTP_CLASS_IPV4 | PTP_CLASS_IPV6;

pub const PTP_MSGTYPE_SYNC: u8 = 0x0;
pub const PTP_MSGTYPE_DELAY_REQ: u8 = 0x1;
pub const PTP_MSGTYPE_PDELAY_REQ: u8 = 0x2;
pub const PTP_MSGTYPE_PDELAY_RESP: u8 = 0x3;

pub const PTP_EV_PORT: u16 = 319;
pub const PTP_GEN_PORT: u16 = 320;
pub const PTP_GEN_BIT: u8 = 0x08; /* indicates general message, if set in message type */

pub const OFF_PTP_SOURCE_UUID: usize = 22; /* PTPv1 only */
pub const OFF_PTP_SEQUENCE_ID: usize = 30;
pub const PTP_FLAG_TWOSTEP: u16 = 1 << 1;
pub const IP6_HLEN: usize = 40;
pub const UDP_HLEN: usize = 8;
pub const OFF_IHL: usize = 14;

#[repr(C, packed)]
pub struct clock_identity { pub id: [u8; 8] }

#[repr(C, packed)]
pub struct port_identity {
    pub clock_identity: clock_identity,
    pub port_number: u16,
}

#[repr(C, packed)]
pub struct ptp_header {
    pub tsmt: u8,
    pub ver: u8,
    pub message_length: u16,
    pub domain_number: u8,
    pub reserved1: u8,
    pub flag_field: [u8; 2],
    pub correction: u64,
    pub reserved2: u32,
    pub source_port_identity: port_identity,
    pub sequence_id: u16,
    pub control: u8,
    pub log_message_interval: u8,
}

#[cfg(feature = "CONFIG_NET_PTP_CLASSIFY")]
extern "C" {
    pub fn ptp_classify_raw(skb: *const sk_buff) -> u32;
    pub fn ptp_parse_header(skb: *mut sk_buff, type_: u32) -> *mut ptp_header;
    pub fn ptp_msg_is_sync(skb: *mut sk_buff, type_: u32) -> bool;
    pub fn ptp_classifier_init();
}

#[inline]
pub unsafe fn ptp_get_msgtype(hdr: *const ptp_header, type_: u32) -> u8 {
    if type_ & PTP_CLASS_V1 != 0 {
        (*hdr).control
    } else {
        (*hdr).tsmt & 0x0f
    }
}

#[inline]
pub unsafe fn ptp_check_diff8(old: u64, new: u64, oldsum: __wsum) -> __wsum {
    let diff: [u64; 2] = [!old, new];
    csum_partial(diff.as_ptr() as *const u8, core::mem::size_of_val(&diff), oldsum)
}

#[inline]
pub unsafe fn ptp_header_update_correction(
    skb: *mut sk_buff, type_: u32, hdr: *mut ptp_header, correction: i64,
) {
    let correction_old = core::ptr::read_unaligned(core::ptr::addr_of!((*hdr).correction));
    core::ptr::write_unaligned(core::ptr::addr_of_mut!((*hdr).correction), correction as u64);
    if type_ & PTP_CLASS_PMASK != PTP_CLASS_IPV4 && type_ & PTP_CLASS_PMASK != PTP_CLASS_IPV6 {
        return;
    }
    let uhdr = (hdr as *mut u8).sub(core::mem::size_of::<udphdr>()) as *mut udphdr;
    (*uhdr).check = csum_fold(ptp_check_diff8(correction_old, (*hdr).correction, !csum_unfold((*uhdr).check)));
    if (*uhdr).check == 0 { (*uhdr).check = CSUM_MANGLED_0; }
    (*skb).ip_summed = CHECKSUM_NONE;
}

#[cfg(not(feature = "CONFIG_NET_PTP_CLASSIFY"))]
pub fn ptp_classifier_init() {}
#[cfg(not(feature = "CONFIG_NET_PTP_CLASSIFY"))]
pub unsafe fn ptp_classify_raw(_skb: *mut sk_buff) -> u32 { PTP_CLASS_NONE }
#[cfg(not(feature = "CONFIG_NET_PTP_CLASSIFY"))]
pub unsafe fn ptp_parse_header(_skb: *mut sk_buff, _type_: u32) -> *mut ptp_header { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_NET_PTP_CLASSIFY"))]
pub unsafe fn ptp_get_msgtype(_hdr: *const ptp_header, _type_: u32) -> u8 { PTP_MSGTYPE_SYNC }
#[cfg(not(feature = "CONFIG_NET_PTP_CLASSIFY"))]
pub unsafe fn ptp_msg_is_sync(_skb: *mut sk_buff, _type_: u32) -> bool { false }

// Types and checksum functions are supplied by included kernel headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
