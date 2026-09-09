/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 1997 by Procom Technology,Inc.
 * 		 2001 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
 */
/* Defines LLC interface to network layer */
/* Available primitives */
/* C dependencies: linux/if.h, linux/if_arp.h, linux/llc.h,
 * linux/etherdevice.h, and net/llc.h. */

pub const LLC_DATAUNIT_PRIM: i32 = 1;
pub const LLC_CONN_PRIM: i32 = 2;
pub const LLC_DATA_PRIM: i32 = 3;
pub const LLC_DISC_PRIM: i32 = 4;
pub const LLC_RESET_PRIM: i32 = 5;
pub const LLC_FLOWCONTROL_PRIM: i32 = 6; /* Not supported at this time */
pub const LLC_DISABLE_PRIM: i32 = 7;
pub const LLC_XID_PRIM: i32 = 8;
pub const LLC_TEST_PRIM: i32 = 9;
pub const LLC_SAP_ACTIVATION: i32 = 10;
pub const LLC_SAP_DEACTIVATION: i32 = 11;

pub const LLC_NBR_PRIMITIVES: i32 = 11;

pub const LLC_IND: i32 = 1;
pub const LLC_CONFIRM: i32 = 2;

/* Primitive type */
pub const LLC_PRIM_TYPE_REQ: i32 = 1;
pub const LLC_PRIM_TYPE_IND: i32 = 2;
pub const LLC_PRIM_TYPE_RESP: i32 = 3;
pub const LLC_PRIM_TYPE_CONFIRM: i32 = 4;

/* Reset reasons, remote entity or local LLC */
pub const LLC_RESET_REASON_REMOTE: i32 = 1;
pub const LLC_RESET_REASON_LOCAL: i32 = 2;

/* Disconnect reasons */
pub const LLC_DISC_REASON_RX_DM_RSP_PDU: i32 = 0;
pub const LLC_DISC_REASON_RX_DISC_CMD_PDU: i32 = 1;
pub const LLC_DISC_REASON_ACK_TMR_EXP: i32 = 2;

/* Confirm reasons */
pub const LLC_STATUS_CONN: i32 = 0; /* connect confirm & reset confirm */
pub const LLC_STATUS_DISC: i32 = 1; /* connect confirm & reset confirm */
pub const LLC_STATUS_FAILED: i32 = 2; /* connect confirm & reset confirm */
pub const LLC_STATUS_IMPOSSIBLE: i32 = 3; /* connect confirm */
pub const LLC_STATUS_RECEIVED: i32 = 4; /* data conn */
pub const LLC_STATUS_REMOTE_BUSY: i32 = 5; /* data conn */
pub const LLC_STATUS_REFUSE: i32 = 6; /* data conn */
pub const LLC_STATUS_CONFLICT: i32 = 7; /* disconnect conn */
pub const LLC_STATUS_RESET_DONE: i32 = 8; /*  */

extern "C" {
    pub fn llc_establish_connection(
        sk: *mut crate::sock,
        lmac: *const u8,
        dmac: *mut u8,
        dsap: u8,
    ) -> i32;
    pub fn llc_build_and_send_pkt(sk: *mut crate::sock, skb: *mut crate::sk_buff) -> i32;
    pub fn llc_send_disc(sk: *mut crate::sock) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
