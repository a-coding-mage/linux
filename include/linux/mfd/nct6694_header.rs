/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2025 Nuvoton Technology Corp.
 *
 * Nuvoton NCT6694 USB transaction and data structure.
 */

pub const NCT6694_VENDOR_ID: u32 = 0x0416;
pub const NCT6694_PRODUCT_ID: u32 = 0x200B;
pub const NCT6694_INT_IN_EP: u32 = 0x81;
pub const NCT6694_BULK_IN_EP: u32 = 0x02;
pub const NCT6694_BULK_OUT_EP: u32 = 0x03;

pub const NCT6694_HCTRL_SET: u32 = 0x40;
pub const NCT6694_HCTRL_GET: u32 = 0x80;

pub const NCT6694_URB_TIMEOUT: u32 = 1000;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum nct6694_irq_id {
    NCT6694_IRQ_GPIO0 = 0,
    NCT6694_IRQ_GPIO1,
    NCT6694_IRQ_GPIO2,
    NCT6694_IRQ_GPIO3,
    NCT6694_IRQ_GPIO4,
    NCT6694_IRQ_GPIO5,
    NCT6694_IRQ_GPIO6,
    NCT6694_IRQ_GPIO7,
    NCT6694_IRQ_GPIO8,
    NCT6694_IRQ_GPIO9,
    NCT6694_IRQ_GPIOA,
    NCT6694_IRQ_GPIOB,
    NCT6694_IRQ_GPIOC,
    NCT6694_IRQ_GPIOD,
    NCT6694_IRQ_GPIOE,
    NCT6694_IRQ_GPIOF,
    NCT6694_IRQ_CAN0,
    NCT6694_IRQ_CAN1,
    NCT6694_IRQ_RTC,
    NCT6694_NR_IRQS,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum nct6694_response_err_status {
    NCT6694_NO_ERROR = 0,
    NCT6694_FORMAT_ERROR,
    NCT6694_RESERVED1,
    NCT6694_RESERVED2,
    NCT6694_NOT_SUPPORT_ERROR,
    NCT6694_NO_RESPONSE_ERROR,
    NCT6694_TIMEOUT_ERROR,
    NCT6694_PENDING,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nct6694_cmd_sel {
    pub cmd: u8,
    pub sel: u8,
}

#[repr(C)]
pub union nct6694_cmd_offset {
    pub offset: __le16,
    pub cmd_sel: nct6694_cmd_sel,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nct6694_cmd_header {
    pub rsv1: u8,
    pub mod_: u8,
    pub offset_or_cmd_sel: nct6694_cmd_offset,
    pub hctrl: u8,
    pub rsv2: u8,
    pub len: __le16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nct6694_response_header {
    pub sequence_id: u8,
    pub sts: u8,
    pub reserved: [u8; 4],
    pub len: __le16,
}

#[repr(C)]
pub union nct6694_usb_msg {
    pub cmd_header: nct6694_cmd_header,
    pub response_header: nct6694_response_header,
}

#[repr(C)]
pub struct nct6694 {
    pub dev: *mut device,
    pub gpio_ida: ida,
    pub i2c_ida: ida,
    pub canfd_ida: ida,
    pub wdt_ida: ida,
    pub domain: *mut irq_domain,
    pub access_lock: mutex,
    pub irq_lock: spinlock_t,
    pub int_in_urb: *mut urb,
    pub udev: *mut usb_device,
    pub usb_msg: *mut nct6694_usb_msg,
    pub int_buffer: *mut __le32,
    pub irq_enable: u32,
}

extern "C" {
    pub fn nct6694_read_msg(
        nct6694: *mut nct6694,
        cmd_hd: *const nct6694_cmd_header,
        buf: *mut core::ffi::c_void,
    ) -> i32;
    pub fn nct6694_write_msg(
        nct6694: *mut nct6694,
        cmd_hd: *const nct6694_cmd_header,
        buf: *mut core::ffi::c_void,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
