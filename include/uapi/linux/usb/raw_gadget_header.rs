/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * USB Raw Gadget driver.
 *
 * See Documentation/usb/raw-gadget.rst for more details.
 */

/* Dependencies: asm/ioctl.h, linux/types.h, and linux/usb/ch9.h. */

/// Maximum length of driver_name/device_name in the usb_raw_init struct.
pub const UDC_NAME_LENGTH_MAX: usize = 128;

/*
 * struct usb_raw_init - argument for USB_RAW_IOCTL_INIT ioctl.
 * @speed: The speed of the emulated USB device, takes the same values as
 *     the usb_device_speed enum: USB_SPEED_FULL, USB_SPEED_HIGH, etc.
 * @driver_name: The name of the UDC driver.
 * @device_name: The name of a UDC instance.
 *
 * The last two fields identify a UDC the gadget driver should bind to.
 */
#[repr(C)]
pub struct usb_raw_init {
    pub driver_name: [u8; UDC_NAME_LENGTH_MAX],
    pub device_name: [u8; UDC_NAME_LENGTH_MAX],
    pub speed: u8,
}

/// The type of event fetched with the USB_RAW_IOCTL_EVENT_FETCH ioctl.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum usb_raw_event_type {
    USB_RAW_EVENT_INVALID = 0,
    USB_RAW_EVENT_CONNECT = 1,
    USB_RAW_EVENT_CONTROL = 2,
    USB_RAW_EVENT_SUSPEND = 3,
    USB_RAW_EVENT_RESUME = 4,
    USB_RAW_EVENT_RESET = 5,
    USB_RAW_EVENT_DISCONNECT = 6,
}

#[repr(C)]
pub struct usb_raw_event {
    pub type_: u32,
    pub length: u32,
    pub data: [u8; 0],
}

pub const USB_RAW_IO_FLAGS_ZERO: u16 = 0x0001;
pub const USB_RAW_IO_FLAGS_MASK: u16 = 0x0001;

#[inline]
pub fn usb_raw_io_flags_valid(flags: u16) -> i32 {
    (((flags & !USB_RAW_IO_FLAGS_MASK) == 0) as i32)
}

#[inline]
pub fn usb_raw_io_flags_zero(flags: u16) -> i32 {
    (flags & USB_RAW_IO_FLAGS_ZERO) as i32
}

#[repr(C)]
pub struct usb_raw_ep_io {
    pub ep: u16,
    pub flags: u16,
    pub length: u32,
    pub data: [u8; 0],
}

pub const USB_RAW_EPS_NUM_MAX: usize = 30;
pub const USB_RAW_EP_NAME_MAX: usize = 16;
pub const USB_RAW_EP_ADDR_ANY: u32 = 0xff;

#[repr(C)]
pub struct usb_raw_ep_caps {
    pub bits: u32,
}

impl usb_raw_ep_caps {
    pub const TYPE_CONTROL: u32 = 1 << 0;
    pub const TYPE_ISO: u32 = 1 << 1;
    pub const TYPE_BULK: u32 = 1 << 2;
    pub const TYPE_INT: u32 = 1 << 3;
    pub const DIR_IN: u32 = 1 << 4;
    pub const DIR_OUT: u32 = 1 << 5;
}

#[repr(C)]
pub struct usb_raw_ep_limits {
    pub maxpacket_limit: u16,
    pub max_streams: u16,
    pub reserved: u32,
}

#[repr(C)]
pub struct usb_raw_ep_info {
    pub name: [u8; USB_RAW_EP_NAME_MAX],
    pub addr: u32,
    pub caps: usb_raw_ep_caps,
    pub limits: usb_raw_ep_limits,
}

#[repr(C)]
pub struct usb_raw_eps_info {
    pub eps: [usb_raw_ep_info; USB_RAW_EPS_NUM_MAX],
}

/* Ioctl command encodings are supplied by asm/ioctl.h. */
macro_rules! USB_RAW_IOCTL_INIT { () => { _IOW!('U', 0, usb_raw_init) }; }
macro_rules! USB_RAW_IOCTL_RUN { () => { _IO!('U', 1) }; }
macro_rules! USB_RAW_IOCTL_EVENT_FETCH { () => { _IOR!('U', 2, usb_raw_event) }; }
macro_rules! USB_RAW_IOCTL_EP0_WRITE { () => { _IOW!('U', 3, usb_raw_ep_io) }; }
macro_rules! USB_RAW_IOCTL_EP0_READ { () => { _IOWR!('U', 4, usb_raw_ep_io) }; }
macro_rules! USB_RAW_IOCTL_EP_ENABLE { () => { _IOW!('U', 5, usb_endpoint_descriptor) }; }
macro_rules! USB_RAW_IOCTL_EP_DISABLE { () => { _IOW!('U', 6, u32) }; }
macro_rules! USB_RAW_IOCTL_EP_WRITE { () => { _IOW!('U', 7, usb_raw_ep_io) }; }
macro_rules! USB_RAW_IOCTL_EP_READ { () => { _IOWR!('U', 8, usb_raw_ep_io) }; }
macro_rules! USB_RAW_IOCTL_CONFIGURE { () => { _IO!('U', 9) }; }
macro_rules! USB_RAW_IOCTL_VBUS_DRAW { () => { _IOW!('U', 10, u32) }; }
macro_rules! USB_RAW_IOCTL_EPS_INFO { () => { _IOR!('U', 11, usb_raw_eps_info) }; }
macro_rules! USB_RAW_IOCTL_EP0_STALL { () => { _IO!('U', 12) }; }
macro_rules! USB_RAW_IOCTL_EP_SET_HALT { () => { _IOW!('U', 13, u32) }; }
macro_rules! USB_RAW_IOCTL_EP_CLEAR_HALT { () => { _IOW!('U', 14, u32) }; }
macro_rules! USB_RAW_IOCTL_EP_SET_WEDGE { () => { _IOW!('U', 15, u32) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
