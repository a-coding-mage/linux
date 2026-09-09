/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Edo Monticelli, Antonio Quartulli
 */

// C dependencies supplied by other headers:
// - `batadv_priv` corresponds to `struct batadv_priv`.
// - `sk_buff` corresponds to `struct sk_buff`.
// - `u8` and `u32` correspond to the Linux integer types.

extern "C" {
    pub fn batadv_tp_meter_init();
    pub fn batadv_tp_start(
        bat_priv: *mut batadv_priv,
        dst: *const u8,
        test_length: u32,
        cookie: *mut u32,
    );
    pub fn batadv_tp_stop(bat_priv: *mut batadv_priv, dst: *const u8, return_value: u8);
    pub fn batadv_tp_stop_all(bat_priv: *mut batadv_priv);
    pub fn batadv_tp_meter_recv(bat_priv: *mut batadv_priv, skb: *mut sk_buff);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
