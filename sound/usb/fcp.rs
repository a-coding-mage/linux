// SPDX-License-Identifier: GPL-2.0
//
// Focusrite Control Protocol Driver for ALSA
//
// Copyright (c) 2024-2025 by Geoffrey D. Bennett <g at b4.vu>

//! Theory of Operation
//!
//! The Focusrite Control Protocol (FCP) driver provides a minimal
//! kernel interface that allows a user-space driver (primarily
//! fcp-server) to communicate with Focusrite USB audio interfaces
//! using their vendor-specific protocol. This protocol is used by
//! Scarlett 2nd Gen, 3rd Gen, 4th Gen, Clarett USB, Clarett+, and
//! Vocaster series devices.
//!
//! Unlike the existing scarlett2 driver which implements all controls
//! in kernel space, this driver takes a lighter-weight approach by
//! moving most functionality to user space. The only control
//! implemented in kernel space is the Level Meter, since it requires
//! frequent polling of volatile data.
//!
//! The driver provides an hwdep interface that allows the user-space
//! driver to:
//!  - Initialise the protocol
//!  - Send arbitrary FCP commands to the device
//!  - Receive notifications from the device
//!  - Configure the Level Meter control
//!
//! ## Usage Flow
//! 1. Open the hwdep device (requires CAP_SYS_RAWIO)
//! 2. Get protocol version using FCP_IOCTL_PVERSION
//! 3. Initialise protocol using FCP_IOCTL_INIT
//! 4. Send commands using FCP_IOCTL_CMD
//! 5. Receive notifications using read()
//! 6. Optionally set up the Level Meter control using
//!    FCP_IOCTL_SET_METER_MAP
//! 7. Optionally add labels to the Level Meter control using
//!    FCP_IOCTL_SET_METER_LABELS
//!
//! ## Level Meter
//! The Level Meter is implemented as an ALSA control that provides
//! real-time level monitoring. When the control is read, the driver
//! requests the current meter levels from the device, translates the
//! levels using the configured mapping, and returns the result to the
//! user. The mapping between device meters and the ALSA control's
//! channels is configured with FCP_IOCTL_SET_METER_MAP.
//!
//! Labels for the Level Meter channels can be set using
//! FCP_IOCTL_SET_METER_LABELS and read by applications through the
//! control's TLV data. The labels are transferred as a sequence of
//! null-terminated strings.

use core::ffi::{c_int, c_uint, c_ulong};
use core::mem;
use core::ptr::{self, NonNull};

// External kernel types (from ALSA/USB subsystem)
// These are defined in external crates but referenced here as declarations
extern "C" {
    type WaitQueueHead;
    type Spinlock;
    type Mutex;
    type Completion;
    type File;
    type Urb;
    type UsbMixerInterface;
    type UsbDevice;
    type SndUsbAudio;
    type SndHwdep;
    type SndKcontrol;
    type SndKcontrolNew;
    type SndCtlElemInfo;
    type SndCtlElemValue;
    type UsbMixerElemInfo;
    type UsbHostConfig;
    type UsbInterface;
    type UsbInterfaceDescriptor;
    type UsbEndpointDescriptor;
}

// notify waiting to send to *file
#[repr(C)]
pub struct FcpNotify {
    pub queue: *mut WaitQueueHead,
    pub event: u32,
    pub lock: *mut Spinlock,
}

#[repr(C)]
pub struct FcpData {
    pub mixer: *mut UsbMixerInterface,

    pub mutex: *mut Mutex,           // serialise access to the device
    pub cmd_done: *mut Completion,   // wait for command completion
    pub file: *mut File,             // hwdep file
    pub urb: *mut Urb,               // FCP notification endpoint

    pub notify: FcpNotify,

    pub b_interface_number: u8,
    pub b_endpoint_address: u8,
    pub w_max_packet_size: u16,
    pub b_interval: u8,

    pub step0_resp_size: u16,
    pub step2_resp_size: u16,
    pub init1_opcode: u32,
    pub init2_opcode: u32,

    pub init: u8,
    pub seq: u16,

    pub num_meter_slots: u8,
    pub meter_level_map: *mut i16,
    pub meter_levels: *mut u32, // __le32
    pub meter_ctl: *mut SndKcontrol,

    pub meter_labels_tlv: *mut u32,
    pub meter_labels_tlv_size: c_int,
}

// USB Interactions

// FCP Command ACK notification bit
const FCP_NOTIFY_ACK: u32 = 1;

// Vendor-specific USB control requests
const FCP_USB_REQ_STEP0: u8 = 0;
const FCP_USB_REQ_CMD_TX: u8 = 2;
const FCP_USB_REQ_CMD_RX: u8 = 3;

// Focusrite Control Protocol opcodes that the kernel side needs to know about
const FCP_USB_REBOOT: u32 = 0x00000003;
const FCP_USB_GET_METER: u32 = 0x00001001;
const FCP_USB_FLASH_ERASE: u32 = 0x00004002;
const FCP_USB_FLASH_WRITE: u32 = 0x00004004;

const FCP_USB_METER_LEVELS_GET_MAGIC: u32 = 1;

const FCP_SEGMENT_APP_GOLD: u32 = 0;

// FCP_MAX_METER_MAP_SIZE macro converted to constant
// sizeof(long) is typically 8 on 64-bit systems
const FCP_MAX_METER_MAP_SIZE: usize = 64 / mem::size_of::<c_ulong>();

// Forward declarations
extern "C" {
    fn fcp_init(
        mixer: *mut UsbMixerInterface,
        step0_resp: *mut core::ffi::c_void,
        step2_resp: *mut core::ffi::c_void,
    ) -> c_int;
}

// FCP command request/response format
#[repr(C)]
pub struct FcpUsbPacket {
    pub opcode: u32,     // __le32
    pub size: u16,       // __le16
    pub seq: u16,        // __le16
    pub error: u32,      // __le32
    pub pad: u32,        // __le32
    pub data: [u8; 0],   // flexible array member
}

extern "C" {
    fn fcp_fill_request_header(
        private: *mut FcpData,
        req: *mut FcpUsbPacket,
        opcode: u32,
        req_size: u16,
    );

    fn fcp_usb_tx(dev: *mut UsbDevice, interface: c_int, buf: *mut core::ffi::c_void, size: u16)
        -> c_int;

    fn fcp_usb_rx(dev: *mut UsbDevice, interface: c_int, buf: *mut core::ffi::c_void, size: u16)
        -> c_int;

    fn fcp_usb(
        mixer: *mut UsbMixerInterface,
        opcode: u32,
        req_data: *const core::ffi::c_void,
        req_size: u16,
        resp_data: *mut core::ffi::c_void,
        resp_size: u16,
    ) -> c_int;

    fn fcp_reinit(mixer: *mut UsbMixerInterface) -> c_int;
}

// Control Functions

extern "C" {
    fn fcp_add_new_ctl(
        mixer: *mut UsbMixerInterface,
        ncontrol: *const SndKcontrolNew,
        index: c_int,
        channels: c_int,
        name: *const u8,
        kctl_return: *mut *mut SndKcontrol,
    ) -> c_int;
}

// Level Meter Control

extern "C" {
    fn fcp_meter_ctl_info(kctl: *mut SndKcontrol, uinfo: *mut SndCtlElemInfo) -> c_int;

    fn fcp_meter_ctl_get(kctl: *mut SndKcontrol, ucontrol: *mut SndCtlElemValue) -> c_int;

    fn fcp_meter_tlv_callback(
        kctl: *mut SndKcontrol,
        op_flag: c_int,
        size: c_uint,
        tlv: *mut u32,
    ) -> c_int;
}

#[repr(C)]
pub struct SndKcontrolNewStruct {
    pub iface: c_int,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut SndKcontrol, *mut SndCtlElemInfo) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut SndKcontrol, *mut SndCtlElemValue) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut SndKcontrol, *mut SndCtlElemValue) -> c_int>,
    pub tlv: Option<unsafe extern "C" fn(*mut SndKcontrol, c_int, c_uint, *mut u32) -> c_int>,
}

extern "C" {
    static fcp_meter_ctl: SndKcontrolNewStruct;
}

// hwdep interface

extern "C" {
    fn fcp_ioctl_init(
        mixer: *mut UsbMixerInterface,
        arg: *mut core::ffi::c_void,
    ) -> c_int;

    fn fcp_ioctl_cmd(mixer: *mut UsbMixerInterface, arg: *mut core::ffi::c_void) -> c_int;

    fn fcp_ioctl_set_meter_map(
        mixer: *mut UsbMixerInterface,
        arg: *mut core::ffi::c_void,
    ) -> c_int;

    fn fcp_ioctl_set_meter_labels(
        mixer: *mut UsbMixerInterface,
        arg: *mut core::ffi::c_void,
    ) -> c_int;
}

extern "C" {
    fn fcp_hwdep_open(hw: *mut SndHwdep, file: *mut File) -> c_int;

    fn fcp_hwdep_ioctl(
        hw: *mut SndHwdep,
        file: *mut File,
        cmd: c_uint,
        arg: c_ulong,
    ) -> c_int;

    fn fcp_hwdep_read(
        hw: *mut SndHwdep,
        buf: *mut u8,
        count: i64,
        offset: *mut i64,
    ) -> i64;

    fn fcp_hwdep_poll(
        hw: *mut SndHwdep,
        file: *mut File,
        wait: *mut core::ffi::c_void,
    ) -> c_uint;

    fn fcp_hwdep_release(hw: *mut SndHwdep, file: *mut File) -> c_int;

    fn fcp_hwdep_init(mixer: *mut UsbMixerInterface) -> c_int;
}

// Cleanup

extern "C" {
    fn fcp_cleanup_urb(mixer: *mut UsbMixerInterface);

    fn fcp_private_free(mixer: *mut UsbMixerInterface);

    fn fcp_private_suspend(mixer: *mut UsbMixerInterface);
}

// Callbacks

extern "C" {
    fn fcp_notify(urb: *mut Urb);

    fn fcp_init_notify(mixer: *mut UsbMixerInterface) -> c_int;
}

// Initialisation

extern "C" {
    fn fcp_init_private(mixer: *mut UsbMixerInterface) -> c_int;

    fn fcp_find_fc_interface(mixer: *mut UsbMixerInterface) -> c_int;
}

// Public exported function

#[no_mangle]
pub extern "C" fn snd_fcp_init(mixer: *mut UsbMixerInterface) -> c_int {
    unsafe {
        let chip = (*mixer).chip;

        // only use UAC_VERSION_2
        if (*mixer).protocol == 0 {
            return 0;
        }

        let err = fcp_init_private(mixer);
        if err < 0 {
            return err;
        }

        let err = fcp_find_fc_interface(mixer);
        if err < 0 {
            return err;
        }

        let err = fcp_hwdep_init(mixer);
        if err < 0 {
            return err;
        }

        // usb_audio_info macro call
        // This references external USB audio info logging
        // Placeholder comment: audio info log about Focusrite Control Protocol Driver ready
        // would include pid information from chip->usb_id

        return err;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
