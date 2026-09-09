/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Focusrite Control Protocol Driver for ALSA
 *
 * Copyright (c) 2024-2025 by Geoffrey D. Bennett <g at b4.vu>
 */
/*
 * DOC: FCP (Focusrite Control Protocol) User-Space API
 *
 * This header defines the interface between the FCP kernel driver and
 * user-space programs to enable the use of the proprietary features
 * available in Focusrite USB audio interfaces. This includes Scarlett
 * 2nd Gen, 3rd Gen, 4th Gen, Clarett USB, Clarett+, and Vocaster
 * series devices.
 *
 * The interface is provided via ALSA's hwdep interface. Opening the
 * hwdep device requires CAP_SYS_RAWIO privileges as this interface
 * provides near-direct access.
 *
 * For details on the FCP protocol, refer to the kernel scarlett2
 * driver in sound/usb/mixer_scarlett2.c and the fcp-support project
 * at https://github.com/geoffreybennett/fcp-support
 *
 * For examples of using these IOCTLs, see the fcp-server source in
 * the fcp-support project.
 *
 * IOCTL Interface
 * --------------
 * FCP_IOCTL_PVERSION:
 *   Returns the protocol version supported by the driver.
 *
 * FCP_IOCTL_INIT:
 *   Initialises the protocol and synchronises sequence numbers
 *   between the driver and device. Must be called at least once
 *   before sending commands. Can be safely called again at any time.
 *
 * FCP_IOCTL_CMD:
 *   Sends an FCP command to the device and returns the response.
 *   Requires prior initialisation via FCP_IOCTL_INIT.
 *
 * FCP_IOCTL_SET_METER_MAP:
 *   Configures the Level Meter control's mapping between device
 *   meters and control channels. Requires FCP_IOCTL_INIT to have
 *   been called first. The map size and number of slots cannot be
 *   changed after initial configuration, although the map itself can
 *   be updated. Once configured, the Level Meter remains functional
 *   even after the hwdep device is closed.
 *
 * FCP_IOCTL_SET_METER_LABELS:
 *   Set the labels for the Level Meter control. Requires
 *   FCP_IOCTL_SET_METER_MAP to have been called first. labels[]
 *   should contain a sequence of null-terminated labels corresponding
 *   to the control's channels.
 */

pub const FCP_HWDEP_MAJOR: u32 = 2;
pub const FCP_HWDEP_MINOR: u32 = 0;
pub const FCP_HWDEP_SUBMINOR: u32 = 0;

pub const FCP_HWDEP_VERSION: u32 =
    (FCP_HWDEP_MAJOR << 16) | (FCP_HWDEP_MINOR << 8) | FCP_HWDEP_SUBMINOR;

#[inline]
pub const fn FCP_HWDEP_VERSION_MAJOR(v: u32) -> u32 { (v >> 16) & 0xFF }
#[inline]
pub const fn FCP_HWDEP_VERSION_MINOR(v: u32) -> u32 { (v >> 8) & 0xFF }
#[inline]
pub const fn FCP_HWDEP_VERSION_SUBMINOR(v: u32) -> u32 { v & 0xFF }

/* Get protocol version. Values are the Linux _IOR/_IOWR/_IOW encodings. */
pub const FCP_IOCTL_PVERSION: u32 = 0x8004_5360;

/* Start the protocol */

/* Step 0 and step 2 responses are variable length and placed in
 * resp[] one after the other.
 */
#[repr(C, packed)]
pub struct fcp_init {
    pub step0_resp_size: u16,
    pub step2_resp_size: u16,
    pub init1_opcode: u32,
    pub init2_opcode: u32,
    pub resp: [u8; 0],
}

pub const FCP_IOCTL_INIT: u32 = 0xC00C_5364;

/* Perform a command */

/* The request data is placed in data[] and the response data will
 * overwrite it.
 */
#[repr(C, packed)]
pub struct fcp_cmd {
    pub opcode: u32,
    pub req_size: u16,
    pub resp_size: u16,
    pub data: [u8; 0],
}

pub const FCP_IOCTL_CMD: u32 = 0xC008_5365;

/* Set the meter map */
#[repr(C, packed)]
pub struct fcp_meter_map {
    pub map_size: u16,
    pub meter_slots: u16,
    pub map: [i16; 0],
}

pub const FCP_IOCTL_SET_METER_MAP: u32 = 0x4004_5366;

/* Set the meter labels */
#[repr(C, packed)]
pub struct fcp_meter_labels {
    pub labels_size: u16,
    pub labels: [core::ffi::c_char; 0],
}

pub const FCP_IOCTL_SET_METER_LABELS: u32 = 0x4002_5367;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
