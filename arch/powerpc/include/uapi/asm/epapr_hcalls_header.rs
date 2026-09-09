/* SPDX-License-Identifier: ((GPL-2.0+ WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * ePAPR hcall interface
 *
 * Copyright 2008-2011 Freescale Semiconductor, Inc.
 *
 * Author: Timur Tabi <timur@freescale.com>
 *
 * This file is provided under a dual BSD/GPL license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *     * Neither the name of Freescale Semiconductor nor the
 *       names of its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 *
 * ALTERNATIVELY, this software may be distributed under the terms of the
 * GNU General Public License ("GPL"), version 2 of that license or (at your
 * option) any later version.
 *
 * THIS SOFTWARE IS PROVIDED BY Freescale Semiconductor ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL Freescale Semiconductor BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

pub const EV_BYTE_CHANNEL_SEND: i32 = 1;
pub const EV_BYTE_CHANNEL_RECEIVE: i32 = 2;
pub const EV_BYTE_CHANNEL_POLL: i32 = 3;
pub const EV_INT_SET_CONFIG: i32 = 4;
pub const EV_INT_GET_CONFIG: i32 = 5;
pub const EV_INT_SET_MASK: i32 = 6;
pub const EV_INT_GET_MASK: i32 = 7;
pub const EV_INT_IACK: i32 = 9;
pub const EV_INT_EOI: i32 = 10;
pub const EV_INT_SEND_IPI: i32 = 11;
pub const EV_INT_SET_TASK_PRIORITY: i32 = 12;
pub const EV_INT_GET_TASK_PRIORITY: i32 = 13;
pub const EV_DOORBELL_SEND: i32 = 14;
pub const EV_MSGSND: i32 = 15;
pub const EV_IDLE: i32 = 16;

/* vendor ID: epapr */
pub const EV_LOCAL_VENDOR_ID: i32 = 0; /* for private use */
pub const EV_EPAPR_VENDOR_ID: i32 = 1;
pub const EV_FSL_VENDOR_ID: i32 = 2; /* Freescale Semiconductor */
pub const EV_IBM_VENDOR_ID: i32 = 3; /* IBM */
pub const EV_GHS_VENDOR_ID: i32 = 4; /* Green Hills Software */
pub const EV_ENEA_VENDOR_ID: i32 = 5; /* Enea */
pub const EV_WR_VENDOR_ID: i32 = 6; /* Wind River Systems */
pub const EV_AMCC_VENDOR_ID: i32 = 7; /* Applied Micro Circuits */
pub const EV_KVM_VENDOR_ID: i32 = 42; /* KVM */

/* The max number of bytes that a byte channel can send or receive per call */
pub const EV_BYTE_CHANNEL_MAX_BYTES: i32 = 16;

pub const fn _EV_HCALL_TOKEN(id: i32, num: i32) -> i32 {
    (id << 16) | num
}

pub const fn EV_HCALL_TOKEN(hcall_num: i32) -> i32 {
    _EV_HCALL_TOKEN(EV_EPAPR_VENDOR_ID, hcall_num)
}

/* epapr return codes */
pub const EV_SUCCESS: i32 = 0;
pub const EV_EPERM: i32 = 1; /* Operation not permitted */
pub const EV_ENOENT: i32 = 2; /*  Entry Not Found */
pub const EV_EIO: i32 = 3; /* I/O error occurred */
pub const EV_EAGAIN: i32 = 4; /* The operation had insufficient
                              * resources to complete and should be
                              * retried
                              */
pub const EV_ENOMEM: i32 = 5; /* There was insufficient memory to
                              * complete the operation */
pub const EV_EFAULT: i32 = 6; /* Bad guest address */
pub const EV_ENODEV: i32 = 7; /* No such device */
pub const EV_EINVAL: i32 = 8; /* An argument supplied to the hcall
                               was out of range or invalid */
pub const EV_INTERNAL: i32 = 9; /* An internal error occurred */
pub const EV_CONFIG: i32 = 10; /* A configuration error was detected */
pub const EV_INVALID_STATE: i32 = 11; /* The object is in an invalid state */
pub const EV_UNIMPLEMENTED: i32 = 12; /* Unimplemented hypercall */
pub const EV_BUFFER_OVERFLOW: i32 = 13; /* Caller-supplied buffer too small */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
