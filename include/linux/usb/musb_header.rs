/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of linux/usb/musb.h.
 * The C header's external types and configuration symbols are supplied by
 * the surrounding kernel translation unit.
 */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musb_mode {
    MUSB_UNDEFINED = 0,
    MUSB_HOST,
    MUSB_PERIPHERAL,
    MUSB_OTG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musb_fifo_style {
    FIFO_RXTX,
    FIFO_TX,
    FIFO_RX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musb_buf_mode {
    BUF_SINGLE,
    BUF_DOUBLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb_fifo_cfg {
    pub hw_ep_num: u8,
    pub style: musb_fifo_style,
    pub mode: musb_buf_mode,
    pub maxpacket: u16,
}

#[macro_export]
macro_rules! MUSB_EP_FIFO {
    ($ep:expr, $st:expr, $m:expr, $pkt:expr) => {
        $crate::musb_fifo_cfg {
            hw_ep_num: $ep,
            style: $st,
            mode: $m,
            maxpacket: $pkt,
        }
    };
}

#[macro_export]
macro_rules! MUSB_EP_FIFO_SINGLE {
    ($ep:expr, $st:expr, $pkt:expr) => {
        $crate::MUSB_EP_FIFO!($ep, $st, $crate::musb_buf_mode::BUF_SINGLE, $pkt)
    };
}

#[macro_export]
macro_rules! MUSB_EP_FIFO_DOUBLE {
    ($ep:expr, $st:expr, $pkt:expr) => {
        $crate::MUSB_EP_FIFO!($ep, $st, $crate::musb_buf_mode::BUF_DOUBLE, $pkt)
    };
}

#[repr(C)]
pub struct musb_hdrc_eps_bits {
    pub name: [core::ffi::c_char; 16],
    pub bits: u8,
}

#[repr(C)]
pub struct musb_hdrc_config {
    pub fifo_cfg: *const musb_fifo_cfg,
    pub fifo_cfg_size: u32,
    /* C bit-fields, packed here as their underlying unsigned storage. */
    pub multipoint: u32,
    pub dyn_fifo: u32,
    pub host_port_deassert_reset_at_resume: u32,
    pub num_eps: u8,
    pub ram_bits: u8,
    pub maximum_speed: u32,
}

#[repr(C)]
pub struct musb_hdrc_platform_data {
    pub mode: u8,
    pub clock: *const core::ffi::c_char,
    pub set_vbus: Option<unsafe extern "C" fn(dev: *mut device, is_on: i32) -> i32>,
    pub power: u8,
    pub min_power: u8,
    pub potpgt: u8,
    /* C bit-field extvbus:1, represented by its underlying storage. */
    pub extvbus: u32,
    pub config: *const musb_hdrc_config,
    pub board_data: *mut core::ffi::c_void,
    pub platform_ops: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum musb_vbus_id_status {
    MUSB_UNKNOWN = 0,
    MUSB_ID_GROUND,
    MUSB_ID_FLOAT,
    MUSB_VBUS_VALID,
    MUSB_VBUS_OFF,
}

/* CONFIG_USB_MUSB_HDRC selects the external implementation in C. */
#[cfg(musb_hdrc)]
extern "C" {
    pub fn musb_mailbox(status: musb_vbus_id_status) -> i32;
}

#[cfg(not(musb_hdrc))]
#[inline]
pub fn musb_mailbox(_status: musb_vbus_id_status) -> i32 {
    0
}

pub const TUSB6010_OSCCLK_60: u32 = 16667;
pub const TUSB6010_REFCLK_24: u32 = 41667;
pub const TUSB6010_REFCLK_19: u32 = 52083;

/* External kernel type referenced by set_vbus. */
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
