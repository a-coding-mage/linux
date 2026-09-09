// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * Copyright (c) 1999-2000 Cisco, Inc.
 * Copyright (c) 1999-2001 Motorola, Inc.
 *
 * This file is part of the SCTP kernel implementation
 *
 * These functions implement the SCTP primitive functions from Section 10.
 *
 * Note that the descriptions from the specification are USER level
 * functions--this file is the functions which populate the struct proto
 * for SCTP which is the BOTTOM of the sockets interface.
 *
 * Please send any bug reports or fixes you make to the
 * email address(es):
 *    lksctp developers <linux-sctp@vger.kernel.org>
 *
 * Written or modified by:
 *    La Monte H.P. Yarroll <piggy@acm.org>
 *    Narasimha Budihal     <narasimha@refcode.org>
 *    Karl Knutson          <karl@athena.chicago.il.us>
 *    Ardelle Fan           <ardelle.fan@intel.com>
 *    Kevin Gao             <kevin.gao@intel.com>
 */

// C dependencies supplied by the surrounding SCTP implementation.

macro_rules! declare_primitive {
    ($function:ident, $primitive:ident) => {
        /* This is called in the code as sctp_primitive_ ## name. */
        pub unsafe fn $function(
            net: *mut net,
            asoc: *mut sctp_association,
            arg: *mut core::ffi::c_void,
        ) -> i32 {
            let mut error: i32 = 0;
            let event_type: sctp_event_type;
            let subtype: sctp_subtype;
            let state: sctp_state;
            let ep: *mut sctp_endpoint;

            event_type = SCTP_EVENT_T_PRIMITIVE;
            subtype = SCTP_ST_PRIMITIVE(SCTP_PRIMITIVE_$primitive);
            state = if !asoc.is_null() {
                (*asoc).state
            } else {
                SCTP_STATE_CLOSED
            };
            ep = if !asoc.is_null() { (*asoc).ep } else { core::ptr::null_mut() };

            error = sctp_do_sm(
                net,
                event_type,
                subtype,
                state,
                ep,
                asoc,
                arg,
                GFP_KERNEL,
            );
            error
        }
    };
}

/* 10.1 ULP-to-SCTP
 * B) Associate
 *
 * Format: ASSOCIATE(local SCTP instance name, destination transport addr,
 *         outbound stream count)
 * -> association id [,destination transport addr list] [,outbound stream
 *    count]
 *
 * This primitive allows the upper layer to initiate an association to a
 * specific peer endpoint.
 *
 * This version assumes that asoc is fully populated with the initial
 * parameters.  We then return a traditional kernel indicator of
 * success or failure.
 */

/* This is called in the code as sctp_primitive_ASSOCIATE. */
declare_primitive!(sctp_primitive_ASSOCIATE, SCTP_PRIMITIVE_ASSOCIATE);

/* 10.1 ULP-to-SCTP
 * C) Shutdown
 *
 * Format: SHUTDOWN(association id)
 * -> result
 *
 * Gracefully closes an association. Any locally queued user data
 * will be delivered to the peer. The association will be terminated only
 * after the peer acknowledges all the SCTP packets sent.  A success code
 * will be returned on successful termination of the association. If
 * attempting to terminate the association results in a failure, an error
 * code shall be returned.
 */

declare_primitive!(sctp_primitive_SHUTDOWN, SCTP_PRIMITIVE_SHUTDOWN);

/* 10.1 ULP-to-SCTP
 * C) Abort
 *
 * Format: Abort(association id [, cause code])
 * -> result
 *
 * Ungracefully closes an association. Any locally queued user data
 * will be discarded and an ABORT chunk is sent to the peer. A success
 * code will be returned on successful abortion of the association. If
 * attempting to abort the association results in a failure, an error
 * code shall be returned.
 */

declare_primitive!(sctp_primitive_ABORT, SCTP_PRIMITIVE_ABORT);

/* 10.1 ULP-to-SCTP
 * E) Send
 *
 * Format: SEND(association id, buffer address, byte count [,context]
 *         [,stream id] [,life time] [,destination transport address]
 *         [,unorder flag] [,no-bundle flag] [,payload protocol-id] )
 * -> result
 *
 * This is the main method to send user data via SCTP.
 *
 * Mandatory attributes:
 *
 *  o association id - local handle to the SCTP association
 *  o buffer address - the location where the user message to be transmitted is stored;
 *  o byte count - The size of the user data in number of bytes;
 *
 * Optional attributes:
 *
 *  o context - an optional 32 bit integer carried in sending failure notification.
 *  o stream id - to indicate which stream to send the data on.
 *  o life time - specifies the life time of the user data.
 *  o destination transport address - one of the peer endpoint's addresses.
 *  o unorder flag - requests unordered delivery to the peer.
 *  o no-bundle flag - instructs SCTP not to bundle this user data.
 *  o payload protocol-id - a 32 bit unsigned opaque payload protocol value.
 */

declare_primitive!(sctp_primitive_SEND, SCTP_PRIMITIVE_SEND);

/* 10.1 ULP-to-SCTP
 * J) Request Heartbeat
 *
 * Format: REQUESTHEARTBEAT(association id, destination transport address)
 *
 * -> result
 *
 * Instructs the local endpoint to perform a HeartBeat on the specified
 * destination transport address of the given association. The returned
 * result should indicate whether the transmission of the HEARTBEAT
 * chunk to the destination address is successful.
 *
 * Mandatory attributes:
 *
 * o association id - local handle to the SCTP association
 * o destination transport address - the transport address of the
 *   association on which a heartbeat should be issued.
 */

declare_primitive!(sctp_primitive_REQUESTHEARTBEAT, SCTP_PRIMITIVE_REQUESTHEARTBEAT);

/* ADDIP
 * 3.1.1 Address Configuration Change Chunk (ASCONF)
 *
 * This chunk is used to communicate to the remote endpoint one of the
 * configuration change requests that MUST be acknowledged. The information
 * carried in the ASCONF Chunk uses the form of a Type-Length-Value (TLV).
 */

declare_primitive!(sctp_primitive_ASCONF, SCTP_PRIMITIVE_ASCONF);

/* RE-CONFIG 5.1 */
declare_primitive!(sctp_primitive_RECONF, SCTP_PRIMITIVE_RECONF);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
