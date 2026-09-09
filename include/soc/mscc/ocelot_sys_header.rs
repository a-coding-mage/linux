/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/* Microsemi Ocelot Switch driver */

// BIT and GENMASK are supplied by the surrounding translated headers.

pub const SYS_COUNT_RX_OCTETS_RSZ: u32 = 0x4;
pub const SYS_COUNT_TX_OCTETS_RSZ: u32 = 0x4;
pub const SYS_FRONT_PORT_MODE_RSZ: u32 = 0x4;
pub const SYS_FRONT_PORT_MODE_HDX_MODE: u32 = BIT!(0);
pub const SYS_FRM_AGING_AGE_TX_ENA: u32 = BIT!(20);
#[inline] pub const fn SYS_FRM_AGING_MAX_AGE(x: u32) -> u32 { x & GENMASK!(19, 0) }
pub const SYS_FRM_AGING_MAX_AGE_M: u32 = GENMASK!(19, 0);

#[inline] pub const fn SYS_STAT_CFG_STAT_CLEAR_SHOT(x: u32) -> u32 { (x << 10) & GENMASK!(16, 10) }
pub const SYS_STAT_CFG_STAT_CLEAR_SHOT_M: u32 = GENMASK!(16, 10);
#[inline] pub const fn SYS_STAT_CFG_STAT_CLEAR_SHOT_X(x: u32) -> u32 { (x & GENMASK!(16, 10)) >> 10 }
#[inline] pub const fn SYS_STAT_CFG_STAT_VIEW(x: u32) -> u32 { x & GENMASK!(9, 0) }
pub const SYS_STAT_CFG_STAT_VIEW_M: u32 = GENMASK!(9, 0);
pub const SYS_SW_STATUS_RSZ: u32 = 0x4;
pub const SYS_SW_STATUS_PORT_RX_PAUSED: u32 = BIT!(0);
pub const SYS_MISC_CFG_PTP_RSRV_CLR: u32 = BIT!(1);
pub const SYS_MISC_CFG_PTP_DIS_NEG_RO: u32 = BIT!(0);
pub const SYS_REW_MAC_HIGH_CFG_RSZ: u32 = 0x4;
pub const SYS_REW_MAC_LOW_CFG_RSZ: u32 = 0x4;

#[inline] pub const fn SYS_TIMESTAMP_OFFSET_ETH_TYPE_CFG(x: u32) -> u32 { (x << 6) & GENMASK!(21, 6) }
pub const SYS_TIMESTAMP_OFFSET_ETH_TYPE_CFG_M: u32 = GENMASK!(21, 6);
#[inline] pub const fn SYS_TIMESTAMP_OFFSET_ETH_TYPE_CFG_X(x: u32) -> u32 { (x & GENMASK!(21, 6)) >> 6 }
#[inline] pub const fn SYS_TIMESTAMP_OFFSET_TIMESTAMP_OFFSET(x: u32) -> u32 { x & GENMASK!(5, 0) }
pub const SYS_TIMESTAMP_OFFSET_TIMESTAMP_OFFSET_M: u32 = GENMASK!(5, 0);
#[inline] pub const fn SYS_PAUSE_TOT_CFG_PAUSE_TOT_START(x: u32) -> u32 { (x << 9) & GENMASK!(17, 9) }
pub const SYS_PAUSE_TOT_CFG_PAUSE_TOT_START_M: u32 = GENMASK!(17, 9);
#[inline] pub const fn SYS_PAUSE_TOT_CFG_PAUSE_TOT_START_X(x: u32) -> u32 { (x & GENMASK!(17, 9)) >> 9 }
#[inline] pub const fn SYS_PAUSE_TOT_CFG_PAUSE_TOT_STOP(x: u32) -> u32 { x & GENMASK!(8, 0) }
pub const SYS_PAUSE_TOT_CFG_PAUSE_TOT_STOP_M: u32 = GENMASK!(8, 0);
pub const SYS_ATOP_RSZ: u32 = 0x4;
pub const SYS_MAC_FC_CFG_RSZ: u32 = 0x4;

#[inline] pub const fn SYS_MAC_FC_CFG_FC_LINK_SPEED(x: u32) -> u32 { (x << 26) & GENMASK!(27, 26) }
pub const SYS_MAC_FC_CFG_FC_LINK_SPEED_M: u32 = GENMASK!(27, 26);
#[inline] pub const fn SYS_MAC_FC_CFG_FC_LINK_SPEED_X(x: u32) -> u32 { (x & GENMASK!(27, 26)) >> 26 }
#[inline] pub const fn SYS_MAC_FC_CFG_FC_LATENCY_CFG(x: u32) -> u32 { (x << 20) & GENMASK!(25, 20) }
pub const SYS_MAC_FC_CFG_FC_LATENCY_CFG_M: u32 = GENMASK!(25, 20);
#[inline] pub const fn SYS_MAC_FC_CFG_FC_LATENCY_CFG_X(x: u32) -> u32 { (x & GENMASK!(25, 20)) >> 20 }
pub const SYS_MAC_FC_CFG_ZERO_PAUSE_ENA: u32 = BIT!(18);
pub const SYS_MAC_FC_CFG_TX_FC_ENA: u32 = BIT!(17);
pub const SYS_MAC_FC_CFG_RX_FC_ENA: u32 = BIT!(16);
#[inline] pub const fn SYS_MAC_FC_CFG_PAUSE_VAL_CFG(x: u32) -> u32 { x & GENMASK!(15, 0) }
pub const SYS_MAC_FC_CFG_PAUSE_VAL_CFG_M: u32 = GENMASK!(15, 0);

#[inline] pub const fn SYS_MMGT_RELCNT(x: u32) -> u32 { (x << 16) & GENMASK!(31, 16) }
pub const SYS_MMGT_RELCNT_M: u32 = GENMASK!(31, 16);
#[inline] pub const fn SYS_MMGT_RELCNT_X(x: u32) -> u32 { (x & GENMASK!(31, 16)) >> 16 }
#[inline] pub const fn SYS_MMGT_FREECNT(x: u32) -> u32 { x & GENMASK!(15, 0) }
pub const SYS_MMGT_FREECNT_M: u32 = GENMASK!(15, 0);
#[inline] pub const fn SYS_MMGT_FAST_FREEVLD(x: u32) -> u32 { (x << 4) & GENMASK!(7, 4) }
pub const SYS_MMGT_FAST_FREEVLD_M: u32 = GENMASK!(7, 4);
#[inline] pub const fn SYS_MMGT_FAST_FREEVLD_X(x: u32) -> u32 { (x & GENMASK!(7, 4)) >> 4 }
#[inline] pub const fn SYS_MMGT_FAST_RELVLD(x: u32) -> u32 { x & GENMASK!(3, 0) }
pub const SYS_MMGT_FAST_RELVLD_M: u32 = GENMASK!(3, 0);

pub const SYS_EVENTS_DIF_RSZ: u32 = 0x4;
#[inline] pub const fn SYS_EVENTS_DIF_EV_DRX(x: u32) -> u32 { (x << 6) & GENMASK!(8, 6) }
pub const SYS_EVENTS_DIF_EV_DRX_M: u32 = GENMASK!(8, 6);
#[inline] pub const fn SYS_EVENTS_DIF_EV_DRX_X(x: u32) -> u32 { (x & GENMASK!(8, 6)) >> 6 }
#[inline] pub const fn SYS_EVENTS_DIF_EV_DTX(x: u32) -> u32 { x & GENMASK!(5, 0) }
pub const SYS_EVENTS_DIF_EV_DTX_M: u32 = GENMASK!(5, 0);
pub const SYS_EVENTS_CORE_EV_FWR: u32 = BIT!(2);
#[inline] pub const fn SYS_EVENTS_CORE_EV_ANA(x: u32) -> u32 { x & GENMASK!(1, 0) }
pub const SYS_EVENTS_CORE_EV_ANA_M: u32 = GENMASK!(1, 0);
pub const SYS_CNT_GSZ: u32 = 0x4;

pub const SYS_PTP_STATUS_PTP_TXSTAMP_OAM: u32 = BIT!(29);
pub const SYS_PTP_STATUS_PTP_OVFL: u32 = BIT!(28);
pub const SYS_PTP_STATUS_PTP_MESS_VLD: u32 = BIT!(27);
#[inline] pub const fn SYS_PTP_STATUS_PTP_MESS_ID(x: u32) -> u32 { (x << 21) & GENMASK!(26, 21) }
pub const SYS_PTP_STATUS_PTP_MESS_ID_M: u32 = GENMASK!(26, 21);
#[inline] pub const fn SYS_PTP_STATUS_PTP_MESS_ID_X(x: u32) -> u32 { (x & GENMASK!(26, 21)) >> 21 }
#[inline] pub const fn SYS_PTP_STATUS_PTP_MESS_TXPORT(x: u32) -> u32 { (x << 16) & GENMASK!(20, 16) }
pub const SYS_PTP_STATUS_PTP_MESS_TXPORT_M: u32 = GENMASK!(20, 16);
#[inline] pub const fn SYS_PTP_STATUS_PTP_MESS_TXPORT_X(x: u32) -> u32 { (x & GENMASK!(20, 16)) >> 16 }
#[inline] pub const fn SYS_PTP_STATUS_PTP_MESS_SEQ_ID(x: u32) -> u32 { x & GENMASK!(15, 0) }
pub const SYS_PTP_STATUS_PTP_MESS_SEQ_ID_M: u32 = GENMASK!(15, 0);
#[inline] pub const fn SYS_PTP_TXSTAMP_PTP_TXSTAMP(x: u32) -> u32 { x & GENMASK!(29, 0) }
pub const SYS_PTP_TXSTAMP_PTP_TXSTAMP_M: u32 = GENMASK!(29, 0);
pub const SYS_PTP_TXSTAMP_PTP_TXSTAMP_SEC: u32 = BIT!(31);
pub const SYS_PTP_NXT_PTP_NXT: u32 = BIT!(0);
#[inline] pub const fn SYS_PTP_CFG_PTP_STAMP_WID(x: u32) -> u32 { (x << 2) & GENMASK!(7, 2) }
pub const SYS_PTP_CFG_PTP_STAMP_WID_M: u32 = GENMASK!(7, 2);
#[inline] pub const fn SYS_PTP_CFG_PTP_STAMP_WID_X(x: u32) -> u32 { (x & GENMASK!(7, 2)) >> 2 }
#[inline] pub const fn SYS_PTP_CFG_PTP_CF_ROLL_MODE(x: u32) -> u32 { x & GENMASK!(1, 0) }
pub const SYS_PTP_CFG_PTP_CF_ROLL_MODE_M: u32 = GENMASK!(1, 0);
pub const SYS_RAM_INIT_RAM_INIT: u32 = BIT!(1);
pub const SYS_RAM_INIT_RAM_CFG_HOOK: u32 = BIT!(0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
