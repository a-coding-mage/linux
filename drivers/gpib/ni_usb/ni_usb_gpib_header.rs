/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *   copyright            : (C) 2004 by Frank Mori Hess
 ***************************************************************************/

// C dependencies supplied by the surrounding kernel/GPIB translation.

pub const USB_VENDOR_ID_NI: i32 = 0x3923;

pub const USB_DEVICE_ID_NI_USB_B: i32 = 0x702a;
pub const USB_DEVICE_ID_NI_USB_B_PREINIT: i32 = 0x702b; // device id before firmware is loaded
pub const USB_DEVICE_ID_NI_USB_HS: i32 = 0x709b;
pub const USB_DEVICE_ID_NI_USB_HS_PLUS: i32 = 0x7618;
pub const USB_DEVICE_ID_KUSB_488A: i32 = 0x725c;
pub const USB_DEVICE_ID_MC_USB_488: i32 = 0x725d;

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum ni_usb_device {
    NIUSB_SUBDEV_TNT4882 = 1,
    NIUSB_SUBDEV_UNKNOWN2 = 2,
    NIUSB_SUBDEV_UNKNOWN3 = 3,
}

pub const NIUSB_B_BULK_OUT_ENDPOINT: i32 = 0x2;
pub const NIUSB_B_BULK_IN_ENDPOINT: i32 = 0x2;
pub const NIUSB_B_BULK_IN_ALT_ENDPOINT: i32 = 0x6;
pub const NIUSB_B_INTERRUPT_IN_ENDPOINT: i32 = 0x4;

pub const NIUSB_HS_BULK_OUT_ENDPOINT: i32 = 0x2;
pub const NIUSB_HS_BULK_OUT_ALT_ENDPOINT: i32 = 0x6;
pub const NIUSB_HS_BULK_IN_ENDPOINT: i32 = 0x4;
pub const NIUSB_HS_BULK_IN_ALT_ENDPOINT: i32 = 0x8;
pub const NIUSB_HS_INTERRUPT_IN_ENDPOINT: i32 = 0x1;

pub const NIUSB_HS_PLUS_BULK_OUT_ENDPOINT: i32 = 0x1;
pub const NIUSB_HS_PLUS_BULK_OUT_ALT_ENDPOINT: i32 = 0x4;
pub const NIUSB_HS_PLUS_BULK_IN_ENDPOINT: i32 = 0x2;
pub const NIUSB_HS_PLUS_BULK_IN_ALT_ENDPOINT: i32 = 0x5;
pub const NIUSB_HS_PLUS_INTERRUPT_IN_ENDPOINT: i32 = 0x3;

#[repr(C)]
pub struct ni_usb_urb_ctx {
    pub complete: completion,
    pub timed_out: u8, // C: unsigned timed_out : 1;
}

// struct which defines private_data for ni_usb devices
#[repr(C)]
pub struct ni_usb_priv {
    pub bus_interface: *mut usb_interface,
    pub bulk_out_endpoint: i32,
    pub bulk_in_endpoint: i32,
    pub interrupt_in_endpoint: i32,
    pub eos_char: u8,
    pub eos_mode: u16,
    pub monitored_ibsta_bits: u32,
    pub bulk_urb: *mut urb,
    pub interrupt_urb: *mut urb,
    pub interrupt_buffer: [u8; 0x11],
    pub addressed_transfer_lock: mutex,
    pub bulk_transfer_lock: mutex,
    pub control_transfer_lock: mutex,
    pub interrupt_transfer_lock: mutex,
    pub bulk_timer: timer_list,
    pub context: ni_usb_urb_ctx,
    pub product_id: i32,
    pub ren_state: u16,
}

#[repr(C)]
pub struct ni_usb_status_block {
    pub id: i16,
    pub ibsta: u16,
    pub error_code: i16,
    pub count: u16,
}

#[repr(C)]
pub struct ni_usb_register {
    pub device: ni_usb_device,
    pub address: i16,
    pub value: u16,
}

pub const NIUSB_IBCAC_ID: i32 = 0x1;
pub const NIUSB_UNKNOWN3_ID: i32 = 0x3; // device level function id?
pub const NIUSB_TERM_ID: i32 = 0x4;
pub const NIUSB_IBGTS_ID: i32 = 0x6;
pub const NIUSB_IBRPP_ID: i32 = 0x7;
pub const NIUSB_REG_READ_ID: i32 = 0x8;
pub const NIUSB_REG_WRITE_ID: i32 = 0x9;
pub const NIUSB_IBSIC_ID: i32 = 0xf;
pub const NIUSB_REGISTER_READ_DATA_START_ID: i32 = 0x34;
pub const NIUSB_REGISTER_READ_DATA_END_ID: i32 = 0x35;
pub const NIUSB_IBRD_DATA_ID: i32 = 0x36;
pub const NIUSB_IBRD_EXTENDED_DATA_ID: i32 = 0x37;
pub const NIUSB_IBRD_STATUS_ID: i32 = 0x38;

pub const NIUSB_NO_ERROR: i32 = 0;
pub const NIUSB_ABORTED_ERROR: i32 = 1;
pub const NIUSB_ATN_STATE_ERROR: i32 = 2;
pub const NIUSB_ADDRESSING_ERROR: i32 = 3;
pub const NIUSB_EOSMODE_ERROR: i32 = 4;
pub const NIUSB_NO_BUS_ERROR: i32 = 5;
pub const NIUSB_NO_LISTENER_ERROR: i32 = 8;
pub const NIUSB_TIMEOUT_ERROR: i32 = 10;

pub const NI_USB_STOP_REQUEST: i32 = 0x20;
pub const NI_USB_WAIT_REQUEST: i32 = 0x21;
pub const NI_USB_POLL_READY_REQUEST: i32 = 0x40;
pub const NI_USB_SERIAL_NUMBER_REQUEST: i32 = 0x41;
pub const NI_USB_HS_PLUS_0x48_REQUEST: i32 = 0x48;
pub const NI_USB_HS_PLUS_LED_REQUEST: i32 = 0x4b;
pub const NI_USB_HS_PLUS_0xf8_REQUEST: i32 = 0xf8;

pub const ni_usb_ibsta_monitor_mask: u32 = SRQI | LOK | REM | CIC | ATN | TACS | LACS | DTAS | DCAS;

#[inline]
pub fn nec7210_to_tnt4882_offset(offset: i32) -> i32 { 2 * offset }

#[inline]
pub unsafe fn ni_usb_bulk_termination(buffer: *mut u8) -> i32 {
    let mut i = 0;
    *buffer.add(i) = NIUSB_TERM_ID as u8; i += 1;
    *buffer.add(i) = 0x0; i += 1;
    *buffer.add(i) = 0x0; i += 1;
    *buffer.add(i) = 0x0; i += 1;
    i
}

pub const SERIAL_NUMBER_4_REG: i32 = 0x8;
pub const SERIAL_NUMBER_3_REG: i32 = 0x9;
pub const SERIAL_NUMBER_2_REG: i32 = 0xa;
pub const SERIAL_NUMBER_1_REG: i32 = 0xb;

#[inline]
pub unsafe fn ni_usb_bulk_register_write_header(buffer: *mut u8, num_writes: i32) -> i32 {
    let mut i = 0;
    *buffer.add(i) = NIUSB_REG_WRITE_ID as u8; i += 1;
    *buffer.add(i) = num_writes as u8; i += 1;
    *buffer.add(i) = 0x0; i += 1;
    i
}

#[inline]
pub unsafe fn ni_usb_bulk_register_write(buffer: *mut u8, reg: ni_usb_register) -> i32 {
    let mut i = 0;
    *buffer.add(i) = reg.device as u8; i += 1;
    *buffer.add(i) = reg.address as u8; i += 1;
    *buffer.add(i) = reg.value as u8; i += 1;
    i
}

#[inline]
pub unsafe fn ni_usb_bulk_register_read_header(buffer: *mut u8, num_reads: i32) -> i32 {
    let mut i = 0;
    *buffer.add(i) = NIUSB_REG_READ_ID as u8; i += 1;
    *buffer.add(i) = num_reads as u8; i += 1;
    i
}

#[inline]
pub unsafe fn ni_usb_bulk_register_read(buffer: *mut u8, device: i32, address: i32) -> i32 {
    let mut i = 0;
    *buffer.add(i) = device as u8; i += 1;
    *buffer.add(i) = address as u8; i += 1;
    i
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
