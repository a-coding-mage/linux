/*
 * Copyright (c) 2017 Redpine Signals Inc.
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

// Dependency supplied externally: Linux `struct sk_buff` and `u8`.

/* HAL queue information */
pub const RSI_COEX_Q: u32 = 0x0;
pub const RSI_BT_Q: u32 = 0x2;
pub const RSI_WLAN_Q: u32 = 0x3;
pub const RSI_WIFI_MGMT_Q: u32 = 0x4;
pub const RSI_WIFI_DATA_Q: u32 = 0x5;
pub const RSI_BT_MGMT_Q: u32 = 0x6;
pub const RSI_BT_DATA_Q: u32 = 0x7;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rsi_coex_queues {
    RSI_COEX_Q_INVALID = -1,
    RSI_COEX_Q_COMMON = 0,
    RSI_COEX_Q_BT,
    RSI_COEX_Q_WLAN,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rsi_host_intf {
    RSI_HOST_INTF_SDIO = 0,
    RSI_HOST_INTF_USB,
}

#[repr(C)]
pub struct rsi_proto_ops {
    pub coex_send_pkt: Option<unsafe extern "C" fn(
        priv_: *mut core::ffi::c_void,
        skb: *mut sk_buff,
        hal_queue: u8,
    ) -> i32>,
    pub get_host_intf: Option<unsafe extern "C" fn(
        priv_: *mut core::ffi::c_void,
    ) -> rsi_host_intf>,
    pub set_bt_context: Option<unsafe extern "C" fn(
        priv_: *mut core::ffi::c_void,
        context: *mut core::ffi::c_void,
    )>,
}

#[repr(C)]
pub struct rsi_mod_ops {
    pub attach: Option<unsafe extern "C" fn(
        priv_: *mut core::ffi::c_void,
        ops: *mut rsi_proto_ops,
    ) -> i32>,
    pub detach: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void)>,
    pub recv_pkt: Option<unsafe extern "C" fn(
        priv_: *mut core::ffi::c_void,
        msg: *const u8,
    ) -> i32>,
}

extern "C" {
    pub static rsi_bt_ops: rsi_mod_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
