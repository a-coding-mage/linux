/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
  drbd_limits.h
  This file is part of DRBD by Philipp Reisner and Lars Ellenberg.
*/

/*
 * Our current limitations.
 * Some of them are hard limits,
 * some of them are arbitrary range limits, that make it easier to provide
 * feedback about nonsense settings for certain configurable values.
 */

// Dependency: names supplied by linux/drbd.h remain external to this translation.

pub const DRBD_MINOR_COUNT_MIN: u32 = 1;
pub const DRBD_MINOR_COUNT_MAX: u32 = 255;
pub const DRBD_MINOR_COUNT_DEF: u32 = 32;
pub const DRBD_MINOR_COUNT_SCALE: i32 = '1' as i32;

pub const DRBD_VOLUME_MAX: u32 = 65534;

pub const DRBD_DIALOG_REFRESH_MIN: u32 = 0;
pub const DRBD_DIALOG_REFRESH_MAX: u32 = 600;
pub const DRBD_DIALOG_REFRESH_SCALE: i32 = '1' as i32;

/* valid port number */
pub const DRBD_PORT_MIN: u32 = 1;
pub const DRBD_PORT_MAX: u32 = 0xffff;
pub const DRBD_PORT_SCALE: i32 = '1' as i32;

/* startup { */
/* if you want more than 3.4 days, disable */
pub const DRBD_WFC_TIMEOUT_MIN: u32 = 0;
pub const DRBD_WFC_TIMEOUT_MAX: u32 = 300000;
pub const DRBD_WFC_TIMEOUT_DEF: u32 = 0;
pub const DRBD_WFC_TIMEOUT_SCALE: i32 = '1' as i32;
pub const DRBD_DEGR_WFC_TIMEOUT_MIN: u32 = 0;
pub const DRBD_DEGR_WFC_TIMEOUT_MAX: u32 = 300000;
pub const DRBD_DEGR_WFC_TIMEOUT_DEF: u32 = 0;
pub const DRBD_DEGR_WFC_TIMEOUT_SCALE: i32 = '1' as i32;
pub const DRBD_OUTDATED_WFC_TIMEOUT_MIN: u32 = 0;
pub const DRBD_OUTDATED_WFC_TIMEOUT_MAX: u32 = 300000;
pub const DRBD_OUTDATED_WFC_TIMEOUT_DEF: u32 = 0;
pub const DRBD_OUTDATED_WFC_TIMEOUT_SCALE: i32 = '1' as i32;
/* }*/

/* net { */
/* timeout, unit centi seconds
 * more than one minute timeout is not useful */
pub const DRBD_TIMEOUT_MIN: u32 = 1;
pub const DRBD_TIMEOUT_MAX: u32 = 600;
pub const DRBD_TIMEOUT_DEF: u32 = 60; /* 6 seconds */
pub const DRBD_TIMEOUT_SCALE: i32 = '1' as i32;
/* If backing disk takes longer than disk_timeout, mark the disk as failed */
pub const DRBD_DISK_TIMEOUT_MIN: u32 = 0; /* 0 = disabled */
pub const DRBD_DISK_TIMEOUT_MAX: u32 = 6000; /* 10 Minutes */
pub const DRBD_DISK_TIMEOUT_DEF: u32 = 0; /* disabled */
pub const DRBD_DISK_TIMEOUT_SCALE: i32 = '1' as i32;
/* active connection retries when C_WF_CONNECTION */
pub const DRBD_CONNECT_INT_MIN: u32 = 1;
pub const DRBD_CONNECT_INT_MAX: u32 = 120;
pub const DRBD_CONNECT_INT_DEF: u32 = 10; /* seconds */
pub const DRBD_CONNECT_INT_SCALE: i32 = '1' as i32;
/* keep-alive probes when idle */
pub const DRBD_PING_INT_MIN: u32 = 1;
pub const DRBD_PING_INT_MAX: u32 = 120;
pub const DRBD_PING_INT_DEF: u32 = 10;
pub const DRBD_PING_INT_SCALE: i32 = '1' as i32;
/* timeout for the ping packets.*/
pub const DRBD_PING_TIMEO_MIN: u32 = 1;
pub const DRBD_PING_TIMEO_MAX: u32 = 300;
pub const DRBD_PING_TIMEO_DEF: u32 = 5;
pub const DRBD_PING_TIMEO_SCALE: i32 = '1' as i32;
/* max number of write requests between write barriers */
pub const DRBD_MAX_EPOCH_SIZE_MIN: u32 = 1;
pub const DRBD_MAX_EPOCH_SIZE_MAX: u32 = 20000;
pub const DRBD_MAX_EPOCH_SIZE_DEF: u32 = 2048;
pub const DRBD_MAX_EPOCH_SIZE_SCALE: i32 = '1' as i32;
/* I don't think that a tcp send buffer of more than 10M is useful */
pub const DRBD_SNDBUF_SIZE_MIN: u32 = 0;
pub const DRBD_SNDBUF_SIZE_MAX: u32 = 10u32 << 20;
pub const DRBD_SNDBUF_SIZE_DEF: u32 = 0;
pub const DRBD_SNDBUF_SIZE_SCALE: i32 = '1' as i32;
pub const DRBD_RCVBUF_SIZE_MIN: u32 = 0;
pub const DRBD_RCVBUF_SIZE_MAX: u32 = 10u32 << 20;
pub const DRBD_RCVBUF_SIZE_DEF: u32 = 0;
pub const DRBD_RCVBUF_SIZE_SCALE: i32 = '1' as i32;
/* @4k PageSize -> 128kB - 512MB */
pub const DRBD_MAX_BUFFERS_MIN: u32 = 32;
pub const DRBD_MAX_BUFFERS_MAX: u32 = 131072;
pub const DRBD_MAX_BUFFERS_DEF: u32 = 2048;
pub const DRBD_MAX_BUFFERS_SCALE: i32 = '1' as i32;
/* @4k PageSize -> 4kB - 512MB */
pub const DRBD_UNPLUG_WATERMARK_MIN: u32 = 1;
pub const DRBD_UNPLUG_WATERMARK_MAX: u32 = 131072;
pub const DRBD_UNPLUG_WATERMARK_DEF: u32 = DRBD_MAX_BUFFERS_DEF / 16;
pub const DRBD_UNPLUG_WATERMARK_SCALE: i32 = '1' as i32;
/* 0 is disabled.
 * 200 should be more than enough even for very short timeouts */
pub const DRBD_KO_COUNT_MIN: u32 = 0;
pub const DRBD_KO_COUNT_MAX: u32 = 200;
pub const DRBD_KO_COUNT_DEF: u32 = 7;
pub const DRBD_KO_COUNT_SCALE: i32 = '1' as i32;
/* } */

/* syncer { */
/* FIXME allow rate to be zero? */
pub const DRBD_RESYNC_RATE_MIN: u32 = 1;
/* channel bonding 10 GbE, or other hardware */
pub const DRBD_RESYNC_RATE_MAX: u32 = 4 << 20;
pub const DRBD_RESYNC_RATE_DEF: u32 = 250;
pub const DRBD_RESYNC_RATE_SCALE: i32 = 'k' as i32; /* kilobytes */
pub const DRBD_AL_EXTENTS_MIN: u32 = 67;
/* we use u16 as "slot number", (u16)~0 is "FREE".
 * If you use >= 292 kB on-disk ring buffer,
 * this is the maximum you can use: */
pub const DRBD_AL_EXTENTS_MAX: u32 = 0xfffe;
pub const DRBD_AL_EXTENTS_DEF: u32 = 1237;
pub const DRBD_AL_EXTENTS_SCALE: i32 = '1' as i32;
pub const DRBD_MINOR_NUMBER_MIN: i32 = -1;
pub const DRBD_MINOR_NUMBER_MAX: i32 = (1 << 20) - 1;
pub const DRBD_MINOR_NUMBER_DEF: i32 = -1;
pub const DRBD_MINOR_NUMBER_SCALE: i32 = '1' as i32;
/* } */

/* drbdsetup XY resize -d Z
 * you are free to reduce the device size to nothing, if you want to.
 * the upper limit with 64bit kernel, enough ram and flexible meta data
 * is 1 PiB, currently. */
/* DRBD_MAX_SECTORS */
pub const DRBD_DISK_SIZE_MIN: u64 = 0;
pub const DRBD_DISK_SIZE_MAX: u64 = 1u64 * (2u64 << 40);
pub const DRBD_DISK_SIZE_DEF: u64 = 0; /* = disabled = no user size... */
pub const DRBD_DISK_SIZE_SCALE: i32 = 's' as i32; /* sectors */

pub const DRBD_ON_IO_ERROR_DEF: i32 = EP_DETACH;
pub const DRBD_FENCING_DEF: i32 = FP_DONT_CARE;
pub const DRBD_AFTER_SB_0P_DEF: i32 = ASB_DISCONNECT;
pub const DRBD_AFTER_SB_1P_DEF: i32 = ASB_DISCONNECT;
pub const DRBD_AFTER_SB_2P_DEF: i32 = ASB_DISCONNECT;
pub const DRBD_RR_CONFLICT_DEF: i32 = ASB_DISCONNECT;
pub const DRBD_ON_NO_DATA_DEF: i32 = OND_IO_ERROR;
pub const DRBD_ON_CONGESTION_DEF: i32 = OC_BLOCK;
pub const DRBD_READ_BALANCING_DEF: i32 = RB_PREFER_LOCAL;

pub const DRBD_MAX_BIO_BVECS_MIN: u32 = 0;
pub const DRBD_MAX_BIO_BVECS_MAX: u32 = 128;
pub const DRBD_MAX_BIO_BVECS_DEF: u32 = 0;
pub const DRBD_MAX_BIO_BVECS_SCALE: i32 = '1' as i32;
pub const DRBD_C_PLAN_AHEAD_MIN: u32 = 0;
pub const DRBD_C_PLAN_AHEAD_MAX: u32 = 300;
pub const DRBD_C_PLAN_AHEAD_DEF: u32 = 20;
pub const DRBD_C_PLAN_AHEAD_SCALE: i32 = '1' as i32;
pub const DRBD_C_DELAY_TARGET_MIN: u32 = 1;
pub const DRBD_C_DELAY_TARGET_MAX: u32 = 100;
pub const DRBD_C_DELAY_TARGET_DEF: u32 = 10;
pub const DRBD_C_DELAY_TARGET_SCALE: i32 = '1' as i32;
pub const DRBD_C_FILL_TARGET_MIN: u32 = 0;
pub const DRBD_C_FILL_TARGET_MAX: u32 = 1 << 20; /* 500MByte in sec */
pub const DRBD_C_FILL_TARGET_DEF: u32 = 100; /* Try to place 50KiB in socket send buffer during resync */
pub const DRBD_C_FILL_TARGET_SCALE: i32 = 's' as i32; /* sectors */
pub const DRBD_C_MAX_RATE_MIN: u32 = 250;
pub const DRBD_C_MAX_RATE_MAX: u32 = 4u32 << 20;
pub const DRBD_C_MAX_RATE_DEF: u32 = 102400;
pub const DRBD_C_MAX_RATE_SCALE: i32 = 'k' as i32; /* kilobytes */
pub const DRBD_C_MIN_RATE_MIN: u32 = 0;
pub const DRBD_C_MIN_RATE_MAX: u32 = 4u32 << 20;
pub const DRBD_C_MIN_RATE_DEF: u32 = 250;
pub const DRBD_C_MIN_RATE_SCALE: i32 = 'k' as i32; /* kilobytes */
pub const DRBD_CONG_FILL_MIN: u32 = 0;
pub const DRBD_CONG_FILL_MAX: u32 = 10u32 << 21; /* 10GByte in sectors */
pub const DRBD_CONG_FILL_DEF: u32 = 0;
pub const DRBD_CONG_FILL_SCALE: i32 = 's' as i32; /* sectors */
pub const DRBD_CONG_EXTENTS_MIN: u32 = DRBD_AL_EXTENTS_MIN;
pub const DRBD_CONG_EXTENTS_MAX: u32 = DRBD_AL_EXTENTS_MAX;
pub const DRBD_CONG_EXTENTS_DEF: u32 = DRBD_AL_EXTENTS_DEF;
pub const DRBD_CONG_EXTENTS_SCALE: i32 = DRBD_AL_EXTENTS_SCALE;
pub const DRBD_PROTOCOL_DEF: i32 = DRBD_PROT_C;
pub const DRBD_DISK_BARRIER_DEF: u32 = 0;
pub const DRBD_DISK_FLUSHES_DEF: u32 = 1;
pub const DRBD_DISK_DRAIN_DEF: u32 = 1;
pub const DRBD_MD_FLUSHES_DEF: u32 = 1;
pub const DRBD_TCP_CORK_DEF: u32 = 1;
pub const DRBD_AL_UPDATES_DEF: u32 = 1;
/* We used to ignore the discard_zeroes_data setting.
 * To not change established (and expected) behaviour,
 * by default assume that, for discard_zeroes_data=0,
 * we can make that an effective discard_zeroes_data=1,
 * if we only explicitly zero-out unaligned partial chunks. */
pub const DRBD_DISCARD_ZEROES_IF_ALIGNED_DEF: u32 = 1;
/* Some backends pretend to support WRITE SAME,
 * but fail such requests when they are actually submitted.
 * This is to tell DRBD to not even try. */
pub const DRBD_DISABLE_WRITE_SAME_DEF: u32 = 0;
pub const DRBD_ALLOW_TWO_PRIMARIES_DEF: u32 = 0;
pub const DRBD_ALWAYS_ASBP_DEF: u32 = 0;
pub const DRBD_USE_RLE_DEF: u32 = 1;
pub const DRBD_CSUMS_AFTER_CRASH_ONLY_DEF: u32 = 0;
pub const DRBD_AL_STRIPES_MIN: u32 = 1;
pub const DRBD_AL_STRIPES_MAX: u32 = 1024;
pub const DRBD_AL_STRIPES_DEF: u32 = 1;
pub const DRBD_AL_STRIPES_SCALE: i32 = '1' as i32;
pub const DRBD_AL_STRIPE_SIZE_MIN: u32 = 4;
pub const DRBD_AL_STRIPE_SIZE_MAX: u32 = 16777216;
pub const DRBD_AL_STRIPE_SIZE_DEF: u32 = 32;
pub const DRBD_AL_STRIPE_SIZE_SCALE: i32 = 'k' as i32; /* kilobytes */
pub const DRBD_SOCKET_CHECK_TIMEO_MIN: u32 = 0;
pub const DRBD_SOCKET_CHECK_TIMEO_MAX: u32 = DRBD_PING_TIMEO_MAX;
pub const DRBD_SOCKET_CHECK_TIMEO_DEF: u32 = 0;
pub const DRBD_SOCKET_CHECK_TIMEO_SCALE: i32 = '1' as i32;
pub const DRBD_RS_DISCARD_GRANULARITY_MIN: u32 = 0;
pub const DRBD_RS_DISCARD_GRANULARITY_MAX: u32 = 1u32 << 20; /* 1MiByte */
pub const DRBD_RS_DISCARD_GRANULARITY_DEF: u32 = 0; /* disabled by default */
pub const DRBD_RS_DISCARD_GRANULARITY_SCALE: i32 = '1' as i32; /* bytes */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
