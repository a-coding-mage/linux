/* SPDX-License-Identifier: GPL-2.0-only
 * Copyright (C) 2014 Broadcom Corporation
 */

/* Included by drivers/net/ethernet/broadcom/bcmsysport.c and
 * net/dsa/tag_brcm.c
 */

/* Broadcom tag specific helpers to insert and extract queue/port number */
macro_rules! BRCM_TAG_SET_PORT_QUEUE {
    ($p:expr, $q:expr) => {
        (($p) << 8 | ($q))
    };
}

macro_rules! BRCM_TAG_GET_PORT {
    ($v:expr) => {
        (($v) >> 8)
    };
}

macro_rules! BRCM_TAG_GET_QUEUE {
    ($v:expr) => {
        (($v) & 0xff)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
