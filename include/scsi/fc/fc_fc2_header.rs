/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright(c) 2007 Intel Corporation. All rights reserved.
 *
 * Maintained at www.Open-FCoE.org
 */

/*
 * Fibre Channel Exchanges and Sequences.
 *
 * The C header's PACKED macro is represented by repr(C, packed) below.
 */

/*
 * Sequence Status Block.
 * This format is set by the FC-FS standard and is sent over the wire.
 * Note that the fields aren't all naturally aligned.
 */
#[repr(C, packed)]
pub struct fc_ssb {
    pub ssb_seq_id: __u8,       /* sequence ID */
    pub _ssb_resvd: __u8,
    pub ssb_low_seq_cnt: __be16, /* lowest SEQ_CNT */

    pub ssb_high_seq_cnt: __be16, /* highest SEQ_CNT */
    pub ssb_s_stat: __be16,       /* sequence status flags */

    pub ssb_err_seq_cnt: __be16, /* error SEQ_CNT */
    pub ssb_fh_cs_ctl: __u8,     /* frame header CS_CTL */
    pub ssb_fh_ox_id: __be16,    /* frame header OX_ID */
    pub ssb_rx_id: __be16,       /* responder's exchange ID */
    pub _ssb_resvd2: [__u8; 2],
}

/* The SSB should be 17 bytes; length of fc_ssb for assert. */
pub const FC_SSB_SIZE: usize = 17;

/* ssb_s_stat - flags from FC-FS-2 T11/1619-D Rev 0.90. */
pub const SSB_ST_RESP: u32 = 1 << 15;        /* sequence responder */
pub const SSB_ST_ACTIVE: u32 = 1 << 14;      /* sequence is active */
pub const SSB_ST_ABNORMAL: u32 = 1 << 12;    /* abnormal ending condition */

pub const SSB_ST_REQ_MASK: u32 = 3 << 10;    /* ACK, abort sequence condition */
pub const SSB_ST_REQ_CONT: u32 = 0 << 10;
pub const SSB_ST_REQ_ABORT: u32 = 1 << 10;
pub const SSB_ST_REQ_STOP: u32 = 2 << 10;
pub const SSB_ST_REQ_RETRANS: u32 = 3 << 10;

pub const SSB_ST_ABTS: u32 = 1 << 9;          /* ABTS protocol completed */
pub const SSB_ST_RETRANS: u32 = 1 << 8;       /* retransmission completed */
pub const SSB_ST_TIMEOUT: u32 = 1 << 7;       /* sequence timed out by recipient */
pub const SSB_ST_P_RJT: u32 = 1 << 6;        /* P_RJT transmitted */

pub const SSB_ST_CLASS_BIT: u32 = 4;          /* class of service field LSB */
pub const SSB_ST_CLASS_MASK: u32 = 3;         /* class of service mask */
pub const SSB_ST_ACK: u32 = 1 << 3;           /* ACK (EOFt or EOFdt) transmitted */

/*
 * Exchange Status Block.
 * This format is set by the FC-FS standard and is sent over the wire.
 * Note that the fields aren't all naturally aligned.
 */
#[repr(C, packed)]
pub struct fc_esb {
    pub esb_cs_ctl: __u8,        /* CS_CTL for frame header */
    pub esb_ox_id: __be16,       /* originator exchange ID */
    pub esb_rx_id: __be16,       /* responder exchange ID */
    pub esb_orig_fid: __be32,    /* fabric ID of originator */
    pub esb_resp_fid: __be32,    /* fabric ID of responder */
    pub esb_e_stat: __be32,      /* status */
    pub _esb_resvd: [__u8; 4],
    pub esb_service_params: [__u8; 112], /* TBD */
    pub esb_seq_status: [__u8; 8],       /* sequence statuses, 8 bytes each */
}

/* Define expected size for ASSERTs. See comments on FC_SSB_SIZE. */
pub const FC_ESB_SIZE: usize = 1 + 5 * 4 + 112 + 8;

/* esb_e_stat - flags from FC-FS-2 T11/1619-D Rev 0.90. */
pub const ESB_ST_RESP: u32 = 1 << 31;       /* responder to exchange */
pub const ESB_ST_SEQ_INIT: u32 = 1 << 30;   /* port holds sequence initiative */
pub const ESB_ST_COMPLETE: u32 = 1 << 29;  /* exchange is complete */
pub const ESB_ST_ABNORMAL: u32 = 1 << 28;  /* abnormal ending condition */
pub const ESB_ST_REC_QUAL: u32 = 1 << 26;  /* recovery qualifier active */

pub const ESB_ST_ERRP_BIT: u32 = 24;        /* LSB for error policy */
pub const ESB_ST_ERRP_MASK: u32 = 3 << 24;  /* mask for error policy */
pub const ESB_ST_ERRP_MULT: u32 = 0 << 24;  /* abort, discard multiple sequences */
pub const ESB_ST_ERRP_SING: u32 = 1 << 24;  /* abort, discard single sequence */
pub const ESB_ST_ERRP_INF: u32 = 2 << 24;   /* process with infinite buffers */
pub const ESB_ST_ERRP_IMM: u32 = 3 << 24;   /* discard mult. with immed. retran. */

pub const ESB_ST_OX_ID_INVL: u32 = 1 << 23; /* originator XID invalid */
pub const ESB_ST_RX_ID_INVL: u32 = 1 << 22; /* responder XID invalid */
pub const ESB_ST_PRI_INUSE: u32 = 1 << 21;  /* priority / preemption in use */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
