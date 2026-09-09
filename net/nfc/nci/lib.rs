// SPDX-License-Identifier: GPL-2.0-only
/*
 *  The NFC Controller Interface is the communication protocol between an
 *  NFC Controller (NFCC) and a Device Host (DH).
 *
 *  Copyright (C) 2011 Texas Instruments, Inc.
 *
 *  Written by Ilan Elias <ilane@ti.com>
 *
 *  Acknowledgements:
 *  This file is based on lib.c, which was written
 *  by Maxim Krasnyansky.
 */

/* NCI status codes to Unix errno mapping */
pub fn nci_to_errno(code: u8) -> i32 {
    match code {
        NCI_STATUS_OK => 0,

        NCI_STATUS_REJECTED => -EBUSY,

        NCI_STATUS_RF_FRAME_CORRUPTED => -EBADMSG,

        NCI_STATUS_NOT_INITIALIZED => -EHOSTDOWN,

        NCI_STATUS_SYNTAX_ERROR
        | NCI_STATUS_SEMANTIC_ERROR
        | NCI_STATUS_INVALID_PARAM
        | NCI_STATUS_RF_PROTOCOL_ERROR
        | NCI_STATUS_NFCEE_PROTOCOL_ERROR => -EPROTO,

        NCI_STATUS_UNKNOWN_GID | NCI_STATUS_UNKNOWN_OID => -EBADRQC,

        NCI_STATUS_MESSAGE_SIZE_EXCEEDED => -EMSGSIZE,

        NCI_STATUS_DISCOVERY_ALREADY_STARTED => -EALREADY,

        NCI_STATUS_DISCOVERY_TARGET_ACTIVATION_FAILED
        | NCI_STATUS_NFCEE_INTERFACE_ACTIVATION_FAILED => -ECONNREFUSED,

        NCI_STATUS_RF_TRANSMISSION_ERROR | NCI_STATUS_NFCEE_TRANSMISSION_ERROR => -ECOMM,

        NCI_STATUS_RF_TIMEOUT_ERROR | NCI_STATUS_NFCEE_TIMEOUT_ERROR => -ETIMEDOUT,

        NCI_STATUS_FAILED => -ENOSYS,
        _ => -ENOSYS,
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
