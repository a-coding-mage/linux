/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2025 Intel Corporation.
 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const USBIO_GPIO_CLIENT: &str = "usbio-gpio";
pub const USBIO_I2C_CLIENT: &str = "usbio-i2c";

pub const USBIO_QUIRK_BULK_MAXP_63: u32 = 1 << 0; // Force bulk endpoint maxp to 63
pub const USBIO_QUIRK_I2C_NO_INIT_ACK: u32 = 1 << 8; // Do not ask for ack on I2C init
pub const USBIO_QUIRK_I2C_MAX_RW_LEN_52: u32 = 1 << 9; // Set i2c-adapter max r/w len to 52
pub const USBIO_QUIRK_I2C_USE_CHUNK_LEN: u32 = 1 << 10; // Send chunk-len for split xfers
pub const USBIO_QUIRK_I2C_ALLOW_400KHZ: u32 = 1 << 11; // Override desc, allowing 400 KHz

pub const USBIO_PKTTYPE_CTRL: u8 = 1;
pub const USBIO_PKTTYPE_DBG: u8 = 2;
pub const USBIO_PKTTYPE_GPIO: u8 = 3;
pub const USBIO_PKTTYPE_I2C: u8 = 4;

#[repr(C, packed)]
pub struct usbio_packet_header {
    pub type_: u8,
    pub cmd: u8,
    pub flags: u8,
}

#[repr(C, packed)]
pub struct usbio_ctrl_packet {
    pub header: usbio_packet_header,
    pub len: u8,
    pub data: [u8; 0],
}

#[repr(C, packed)]
pub struct usbio_bulk_packet {
    pub header: usbio_packet_header,
    pub len: u16,
    pub data: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum usbio_gpio_cmd {
    USBIO_GPIOCMD_DEINIT,
    USBIO_GPIOCMD_INIT,
    USBIO_GPIOCMD_READ,
    USBIO_GPIOCMD_WRITE,
    USBIO_GPIOCMD_END,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum usbio_gpio_pincfg {
    USBIO_GPIO_PINCFG_DEFAULT,
    USBIO_GPIO_PINCFG_PULLUP,
    USBIO_GPIO_PINCFG_PULLDOWN,
    USBIO_GPIO_PINCFG_PUSHPULL,
}

pub const USBIO_GPIO_PINCFG_SHIFT: u32 = 2;
pub const USBIO_GPIO_PINCFG_MASK: u32 = 0x3 << USBIO_GPIO_PINCFG_SHIFT;
#[inline]
pub const fn USBIO_GPIO_SET_PINCFG(pincfg: u32) -> u32 {
    (pincfg << USBIO_GPIO_PINCFG_SHIFT) & USBIO_GPIO_PINCFG_MASK
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum usbio_gpio_pinmode {
    USBIO_GPIO_PINMOD_INVAL,
    USBIO_GPIO_PINMOD_INPUT,
    USBIO_GPIO_PINMOD_OUTPUT,
    USBIO_GPIO_PINMOD_MAXVAL,
}

pub const USBIO_GPIO_PINMOD_MASK: u32 = 0x3;
#[inline]
pub const fn USBIO_GPIO_SET_PINMOD(pin: u32) -> u32 {
    pin & USBIO_GPIO_PINMOD_MASK
}

pub const USBIO_MAX_GPIOBANKS: u32 = 5;
pub const USBIO_GPIOSPERBANK: u32 = 32;

#[repr(C, packed)]
pub struct usbio_gpio_bank_desc {
    pub id: u8,
    pub pins: u8,
    pub bmap: u32,
}

#[repr(C, packed)]
pub struct usbio_gpio_init {
    pub bankid: u8,
    pub config: u8,
    pub pincount: u8,
    pub pin: u8,
}

#[repr(C, packed)]
pub struct usbio_gpio_rw {
    pub bankid: u8,
    pub pincount: u8,
    pub pin: u8,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum usbio_i2c_cmd {
    USBIO_I2CCMD_UNINIT,
    USBIO_I2CCMD_INIT,
    USBIO_I2CCMD_READ,
    USBIO_I2CCMD_WRITE,
    USBIO_I2CCMD_END,
}

pub const USBIO_MAX_I2CBUSES: u32 = 5;
pub const USBIO_I2C_BUS_ADDR_CAP_10B: u32 = 1 << 3; // 10bit address support
pub const USBIO_I2C_BUS_MODE_CAP_MASK: u32 = 0x3;
pub const USBIO_I2C_BUS_MODE_CAP_SM: u32 = 0; // Standard Mode
pub const USBIO_I2C_BUS_MODE_CAP_FM: u32 = 1; // Fast Mode
pub const USBIO_I2C_BUS_MODE_CAP_FMP: u32 = 2; // Fast Mode+
pub const USBIO_I2C_BUS_MODE_CAP_HSM: u32 = 3; // High-Speed Mode

#[repr(C, packed)]
pub struct usbio_i2c_bus_desc {
    pub id: u8,
    pub caps: u8,
}

#[repr(C, packed)]
pub struct usbio_i2c_uninit {
    pub busid: u8,
    pub config: u16,
}

#[repr(C, packed)]
pub struct usbio_i2c_init {
    pub busid: u8,
    pub config: u16,
    pub speed: u32,
}

#[repr(C, packed)]
pub struct usbio_i2c_rw {
    pub busid: u8,
    pub config: u16,
    pub size: u16,
    pub data: [u8; 0],
}

extern "C" {
    pub fn usbio_control_msg(
        adev: *mut auxiliary_device,
        type_: u8,
        cmd: u8,
        obuf: *const core::ffi::c_void,
        obuf_len: u16,
        ibuf: *mut core::ffi::c_void,
        ibuf_len: u16,
    ) -> i32;

    pub fn usbio_bulk_msg(
        adev: *mut auxiliary_device,
        type_: u8,
        cmd: u8,
        last: bool,
        obuf: *const core::ffi::c_void,
        obuf_len: u16,
        ibuf: *mut core::ffi::c_void,
        ibuf_len: u16,
    ) -> i32;

    pub fn usbio_acquire(adev: *mut auxiliary_device) -> i32;
    pub fn usbio_release(adev: *mut auxiliary_device);
    pub fn usbio_get_txrxbuf_len(
        adev: *mut auxiliary_device,
        txbuf_len: *mut u16,
        rxbuf_len: *mut u16,
    );
    pub fn usbio_get_quirks(adev: *mut auxiliary_device) -> c_ulong;
    pub fn usbio_acpi_bind(adev: *mut auxiliary_device, hids: *const acpi_device_id);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
