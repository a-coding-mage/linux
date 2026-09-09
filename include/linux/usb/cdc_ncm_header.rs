// SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause)
/*
 * Copyright (C) ST-Ericsson 2010-2012
 * Contact: Alexey Orishko <alexey.orishko@stericsson.com>
 * Original author: Hans Petter Selasky <hans.petter.selasky@stericsson.com>
 *
 * USB Host Driver for Network Control Model (NCM)
 * http://www.usb.org/developers/devclass_docs/NCM10.zip
 *
 * The NCM encoding, decoding and initialization logic
 * derives from FreeBSD 8.x. if_cdce.c and if_cdcereg.h
 *
 * This software is available to you under a choice of one of two
 * licenses. You may choose this file to be licensed under the terms
 * of the GNU General Public License (GPL) Version 2 or the 2-clause
 * BSD license listed below.
 */

pub const CDC_NCM_COMM_ALTSETTING_NCM: u32 = 0;
pub const CDC_NCM_COMM_ALTSETTING_MBIM: u32 = 1;

pub const CDC_NCM_DATA_ALTSETTING_NCM: u32 = 1;
pub const CDC_NCM_DATA_ALTSETTING_MBIM: u32 = 2;

/* CDC NCM subclass 3.3.1 */
pub const USB_CDC_NCM_NDP16_LENGTH_MIN: u32 = 0x10;

/* CDC NCM subclass 3.3.2 */
pub const USB_CDC_NCM_NDP32_LENGTH_MIN: u32 = 0x20;

/* Maximum NTB length */
pub const CDC_NCM_NTB_MAX_SIZE_TX: u32 = 65536; /* bytes */
pub const CDC_NCM_NTB_MAX_SIZE_RX: u32 = 65536; /* bytes */

/* Initial NTB length */
pub const CDC_NCM_NTB_DEF_SIZE_TX: u32 = 16384; /* bytes */
pub const CDC_NCM_NTB_DEF_SIZE_RX: u32 = 16384; /* bytes */

/* Minimum value for MaxDatagramSize, ch. 6.2.9 */
pub const CDC_NCM_MIN_DATAGRAM_SIZE: u32 = 1514; /* bytes */

/* Minimum value for MaxDatagramSize, ch. 8.1.3 */
pub const CDC_MBIM_MIN_DATAGRAM_SIZE: u32 = 2048; /* bytes */

pub const CDC_NCM_MIN_TX_PKT: u32 = 512; /* bytes */

/* Default value for MaxDatagramSize */
pub const CDC_NCM_MAX_DATAGRAM_SIZE: u32 = 8192; /* bytes */

/* Maximum amount of datagrams in NCM Datagram Pointer Table, not counting
 * the last NULL entry.
 */
pub const CDC_NCM_DPT_DATAGRAMS_MAX: u32 = 40;

/* Restart the timer, if amount of datagrams is less than given value */
pub const CDC_NCM_RESTART_TIMER_DATAGRAM_CNT: u32 = 3;
pub const CDC_NCM_TIMER_PENDING_CNT: u32 = 2;
pub const CDC_NCM_TIMER_INTERVAL_USEC: u32 = 400;
pub const CDC_NCM_TIMER_INTERVAL_MIN: u32 = 5;
pub const CDC_NCM_TIMER_INTERVAL_MAX: u32 = U32_MAX / NSEC_PER_USEC;

/* Driver flags */
pub const CDC_NCM_FLAG_NDP_TO_END: u32 = 0x02; /* NDP is placed at end of frame */
pub const CDC_MBIM_FLAG_AVOID_ALTSETTING_TOGGLE: u32 = 0x04; /* Avoid altsetting toggle during init */
pub const CDC_NCM_FLAG_PREFER_NTB32: u32 = 0x08; /* prefer NDP32 over NDP16 */

#[inline]
pub unsafe fn cdc_ncm_comm_intf_is_mbim(x: *const usb_interface) -> bool {
    (*x).desc.bInterfaceSubClass == USB_CDC_SUBCLASS_MBIM &&
        (*x).desc.bInterfaceProtocol == USB_CDC_PROTO_NONE
}

#[inline]
pub unsafe fn cdc_ncm_data_intf_is_mbim(x: *const usb_interface) -> bool {
    (*x).desc.bInterfaceProtocol == USB_CDC_MBIM_PROTO_NTB
}

#[repr(C)]
pub union cdc_ncm_ctx_delayed_ndp {
    pub delayed_ndp16: *mut usb_cdc_ncm_ndp16,
    pub delayed_ndp32: *mut usb_cdc_ncm_ndp32,
}

#[repr(C)]
pub struct cdc_ncm_ctx {
    pub ncm_parm: usb_cdc_ncm_ntb_parameters,
    pub tx_timer: hrtimer,
    pub bh: tasklet_struct,
    pub dev: *mut usbnet,
    pub func_desc: *const usb_cdc_ncm_desc,
    pub mbim_desc: *const usb_cdc_mbim_desc,
    pub mbim_extended_desc: *const usb_cdc_mbim_extended_desc,
    pub ether_desc: *const usb_cdc_ether_desc,
    pub control: *mut usb_interface,
    pub data: *mut usb_interface,
    pub tx_curr_skb: *mut sk_buff,
    pub tx_rem_skb: *mut sk_buff,
    pub tx_rem_sign: __le32,
    pub mtx: spinlock_t,
    pub stop: atomic_t,
    pub drvflags: i32,
    pub timer_interval: u32,
    pub max_ndp_size: u32,
    pub is_ndp16: bool,
    pub filtering_supported: bool,
    pub delayed_ndp: cdc_ncm_ctx_delayed_ndp,
    pub tx_timer_pending: u32,
    pub tx_curr_frame_num: u32,
    pub rx_max: u32,
    pub tx_max: u32,
    pub tx_curr_size: u32,
    pub tx_low_mem_max_cnt: u32,
    pub tx_low_mem_val: u32,
    pub max_datagram_size: u32,
    pub tx_max_datagrams: u16,
    pub tx_remainder: u16,
    pub tx_modulus: u16,
    pub tx_ndp_modulus: u16,
    pub tx_seq: u16,
    pub rx_seq: u16,
    pub min_tx_pkt: u16,
    /* statistics */
    pub tx_curr_frame_payload: u32,
    pub tx_reason_ntb_full: u32,
    pub tx_reason_ndp_full: u32,
    pub tx_reason_timeout: u32,
    pub tx_reason_max_datagram: u32,
    pub tx_overhead: u64,
    pub tx_ntbs: u64,
    pub rx_overhead: u64,
    pub rx_ntbs: u64,
}

extern "C" {
    pub fn cdc_ncm_select_altsetting(intf: *mut usb_interface) -> u8;
    pub fn cdc_ncm_change_mtu(net: *mut net_device, new_mtu: i32) -> i32;
    pub fn cdc_ncm_bind_common(dev: *mut usbnet, intf: *mut usb_interface, data_altsetting: u8, drvflags: i32) -> i32;
    pub fn cdc_ncm_unbind(dev: *mut usbnet, intf: *mut usb_interface);
    pub fn cdc_ncm_fill_tx_frame(dev: *mut usbnet, skb: *mut sk_buff, sign: __le32) -> *mut sk_buff;
    pub fn cdc_ncm_rx_verify_nth16(ctx: *mut cdc_ncm_ctx, skb_in: *mut sk_buff) -> i32;
    pub fn cdc_ncm_rx_verify_ndp16(skb_in: *mut sk_buff, ndpoffset: i32) -> i32;
    pub fn cdc_ncm_rx_verify_nth32(ctx: *mut cdc_ncm_ctx, skb_in: *mut sk_buff) -> i32;
    pub fn cdc_ncm_rx_verify_ndp32(skb_in: *mut sk_buff, ndpoffset: i32) -> i32;
    pub fn cdc_ncm_tx_fixup(dev: *mut usbnet, skb: *mut sk_buff, flags: gfp_t) -> *mut sk_buff;
    pub fn cdc_ncm_rx_fixup(dev: *mut usbnet, skb_in: *mut sk_buff) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
