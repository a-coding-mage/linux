/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (C) 2019 Texas Instruments Incorporated - https://www.ti.com
 */

// Dependency intent: definitions corresponding to <linux/types.h> are supplied externally.

pub const K3_PSIL_DST_THREAD_ID_OFFSET: u32 = 0x8000;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/**
 * enum udma_tp_level - Channel Throughput Levels
 * @UDMA_TP_NORMAL:      Normal channel
 * @UDMA_TP_HIGH:        High Throughput channel
 * @UDMA_TP_ULTRAHIGH:   Ultra High Throughput channel
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum udma_tp_level {
    UDMA_TP_NORMAL = 0,
    UDMA_TP_HIGH,
    UDMA_TP_ULTRAHIGH,
    UDMA_TP_LAST,
}

/**
 * enum psil_endpoint_type - PSI-L Endpoint type
 * @PSIL_EP_NATIVE:      Normal channel
 * @PSIL_EP_PDMA_XY:     XY mode PDMA
 * @PSIL_EP_PDMA_MCAN:   MCAN mode PDMA
 * @PSIL_EP_PDMA_AASRC:  AASRC mode PDMA
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum psil_endpoint_type {
    PSIL_EP_NATIVE = 0,
    PSIL_EP_PDMA_XY,
    PSIL_EP_PDMA_MCAN,
    PSIL_EP_PDMA_AASRC,
}

/**
 * struct psil_endpoint_config - PSI-L Endpoint configuration
 * @ep_type:              PSI-L endpoint type
 * @channel_tpl:          Desired throughput level for the channel
 * @pkt_mode:             If set, the channel must be in Packet mode, otherwise in
 *                        TR mode
 * @notdpkt:              TDCM must be suppressed on the TX channel
 * @needs_epib:           Endpoint needs EPIB
 * @pdma_acc32:           ACC32 must be enabled on the PDMA side
 * @pdma_burst:           BURST must be enabled on the PDMA side
 * @psd_size:             If set, PSdata is used by the endpoint
 * @mapped_channel_id:    PKTDMA thread to channel mapping for mapped channels.
 *                        The thread must be serviced by the specified channel if
 *                        mapped_channel_id is >= 0 in case of PKTDMA
 * @flow_start:           PKDMA flow range start of mapped channel. Unmapped
 *                        channels use flow_id == chan_id
 * @flow_num:             PKDMA flow count of mapped channel. Unmapped channels
 *                        use flow_id == chan_id
 * @default_flow_id:      PKDMA default (r)flow index of mapped channel.
 *                        Must be within the flow range of the mapped channel.
 */
#[repr(C)]
pub struct psil_endpoint_config {
    pub ep_type: psil_endpoint_type,
    pub channel_tpl: udma_tp_level,

    // C bit-fields, each width 1.
    pub pkt_mode: u32,
    pub notdpkt: u32,
    pub needs_epib: u32,
    /* PDMA properties, valid for PSIL_EP_PDMA_* */
    pub pdma_acc32: u32,
    pub pdma_burst: u32,

    pub psd_size: u32,
    /* PKDMA mapped channel */
    pub mapped_channel_id: i16,
    /* PKTDMA tflow and rflow ranges for mapped channel */
    pub flow_start: u16,
    pub flow_num: u16,
    pub default_flow_id: i16,
}

extern "C" {
    pub fn psil_set_new_ep_config(
        dev: *mut device,
        name: *const core::ffi::c_char,
        ep_config: *mut psil_endpoint_config,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
