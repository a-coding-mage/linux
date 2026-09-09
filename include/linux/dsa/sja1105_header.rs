/* SPDX-License-Identifier: GPL-2.0
 * Copyright (c) 2019, Vladimir Oltean <olteanv@gmail.com>
 */

/* Included by drivers/net/dsa/sja1105/sja1105.h and net/dsa/tag_sja1105.c */

/* Dependencies supplied by the surrounding kernel translation unit. */

pub const ETH_P_SJA1105: u16 = ETH_P_DSA_8021Q;
pub const ETH_P_SJA1105_META: u16 = 0x0008;
pub const ETH_P_SJA1110: u16 = 0xdadc;
pub const SJA1105_DEFAULT_VLAN: u16 = VLAN_N_VID - 1;

/* IEEE 802.3 Annex 57A: Slow Protocols PDUs (01:80:C2:xx:xx:xx) */
pub const SJA1105_LINKLOCAL_FILTER_A: u64 = 0x0180C2000000;
pub const SJA1105_LINKLOCAL_FILTER_A_MASK: u64 = 0xFFFFFF000000;
/* IEEE 1588 Annex F: Transport of PTP over Ethernet (01:1B:19:xx:xx:xx) */
pub const SJA1105_LINKLOCAL_FILTER_B: u64 = 0x011B19000000;
pub const SJA1105_LINKLOCAL_FILTER_B_MASK: u64 = 0xFFFFFF000000;

/* Source and Destination MAC of follow-up meta frames.
 * Whereas the choice of SMAC only affects the unique identification of the
 * switch as sender of meta frames, the DMAC must be an address that is present
 * in the DSA conduit port's multicast MAC filter.
 * 01-80-C2-00-00-0E is a good choice for this, as all profiles of IEEE 1588
 * over L2 use this address for some purpose already.
 */
pub const SJA1105_META_SMAC: u64 = 0x222222222222;
pub const SJA1105_META_DMAC: u64 = 0x0180C200000E;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sja1110_meta_tstamp {
    SJA1110_META_TSTAMP_TX = 0,
    SJA1110_META_TSTAMP_RX = 1,
}

#[repr(C)]
pub struct sja1105_deferred_xmit_work {
    pub dp: *mut dsa_port,
    pub skb: *mut sk_buff,
    pub work: kthread_work,
}

/* Global tagger data */
#[repr(C)]
pub struct sja1105_tagger_data {
    pub xmit_work_fn: Option<unsafe extern "C" fn(work: *mut kthread_work)>,
    pub meta_tstamp_handler: Option<unsafe extern "C" fn(
        ds: *mut dsa_switch,
        port: core::ffi::c_int,
        ts_id: u8,
        dir: sja1110_meta_tstamp,
        tstamp: u64,
    )>,
}

#[repr(C)]
pub struct sja1105_skb_cb {
    pub clone: *mut sk_buff,
    pub tstamp: u64,
    /* Only valid for packets cloned for 2-step TX timestamping */
    pub ts_id: u8,
}

#[inline]
pub unsafe fn SJA1105_SKB_CB(skb: *mut sk_buff) -> *mut sja1105_skb_cb {
    (*skb).cb.as_mut_ptr() as *mut sja1105_skb_cb
}

#[inline]
pub unsafe fn sja1105_tagger_data(ds: *mut dsa_switch) -> *mut sja1105_tagger_data {
    BUG_ON(
        (*(*ds).dst).tag_ops.proto != DSA_TAG_PROTO_SJA1105
            && (*(*ds).dst).tag_ops.proto != DSA_TAG_PROTO_SJA1110,
    );
    (*ds).tagger_data
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
