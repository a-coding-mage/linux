/* SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause) */
/* Copyright (c) 2002-2007 Volkswagen Group Electronic Research
 * Copyright (c) 2017 Pengutronix, Marc Kleine-Budde <kernel@pengutronix.de>
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of Volkswagen nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * Alternatively, provided that this notice is retained in full, this
 * software may be distributed under the terms of the GNU General
 * Public License ("GPL"), in which case the provisions of the
 * GPL apply INSTEAD OF those given above.
 *
 * The provided data structures and external interfaces from this code
 * are not restricted to be used by modules with a GPL compatible license.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// Dependencies supplied by the surrounding kernel translation.

pub const CAN_CAP_CC: u32 = 1u32 << 0; // CAN CC aka Classical CAN
pub const CAN_CAP_FD: u32 = 1u32 << 1; // CAN FD
pub const CAN_CAP_XL: u32 = 1u32 << 2; // CAN XL
pub const CAN_CAP_RO: u32 = 1u32 << 3; // read-only mode (LISTEN/RESTRICTED)

pub const CAN_SFF_RCV_ARRAY_SZ: usize = 1usize << CAN_SFF_ID_BITS;
pub const CAN_EFF_RCV_HASH_BITS: usize = 10;
pub const CAN_EFF_RCV_ARRAY_SZ: usize = 1usize << CAN_EFF_RCV_HASH_BITS;

#[repr(i32)]
pub enum RxListKind {
    RX_ERR,
    RX_ALL,
    RX_FIL,
    RX_INV,
    RX_MAX,
}

#[repr(C)]
pub struct can_dev_rcv_lists {
    pub rx: [hlist_head; RX_MAX as usize],
    pub rx_sff: [hlist_head; CAN_SFF_RCV_ARRAY_SZ],
    pub rx_eff: [hlist_head; CAN_EFF_RCV_ARRAY_SZ],
    pub entries: i32,
}

#[repr(C)]
pub struct can_ml_priv {
    pub dev_rcv_lists: can_dev_rcv_lists,
    // Preserved build-time condition: enabled when CAN_J1939 is defined.
    #[cfg(feature = "CAN_J1939")]
    pub j1939_priv: *mut j1939_priv,
    pub can_cap: u32,
}

#[inline]
pub unsafe fn can_get_ml_priv(dev: *mut net_device) -> *mut can_ml_priv {
    netdev_get_ml_priv(dev, ML_PRIV_CAN) as *mut can_ml_priv
}

#[inline]
pub unsafe fn can_set_ml_priv(dev: *mut net_device, ml_priv: *mut can_ml_priv) {
    netdev_set_ml_priv(dev, ml_priv as *mut _, ML_PRIV_CAN);
}

#[inline]
pub unsafe fn can_cap_enabled(dev: *mut net_device, cap: u32) -> bool {
    let can_ml = can_get_ml_priv(dev);

    if can_ml.is_null() {
        return false;
    }

    ((*can_ml).can_cap & cap) != 0
}

#[inline]
pub unsafe fn can_set_cap(dev: *mut net_device, cap: u32) {
    let can_ml = can_get_ml_priv(dev);

    (*can_ml).can_cap = cap;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
