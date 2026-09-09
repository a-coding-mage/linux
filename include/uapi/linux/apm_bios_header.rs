/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Include file for the interface to an APM BIOS
 * Copyright 1994-2001 Stephen Rothwell (sfr@canb.auug.org.au)
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2, or (at your option) any
 * later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 */

// Dependency: linux/types.h
pub type apm_event_t = u16;
pub type apm_eventinfo_t = u16;

#[repr(C)]
pub struct apm_bios_info {
	pub version: u16,
	pub cseg: u16,
	pub offset: u32,
	pub cseg_16: u16,
	pub dseg: u16,
	pub flags: u16,
	pub cseg_len: u16,
	pub cseg_16_len: u16,
	pub dseg_len: u16,
}

/*
 * Power states
 */
pub const APM_STATE_READY: u32 = 0x0000;
pub const APM_STATE_STANDBY: u32 = 0x0001;
pub const APM_STATE_SUSPEND: u32 = 0x0002;
pub const APM_STATE_OFF: u32 = 0x0003;
pub const APM_STATE_BUSY: u32 = 0x0004;
pub const APM_STATE_REJECT: u32 = 0x0005;
pub const APM_STATE_OEM_SYS: u32 = 0x0020;
pub const APM_STATE_OEM_DEV: u32 = 0x0040;

pub const APM_STATE_DISABLE: u32 = 0x0000;
pub const APM_STATE_ENABLE: u32 = 0x0001;

pub const APM_STATE_DISENGAGE: u32 = 0x0000;
pub const APM_STATE_ENGAGE: u32 = 0x0001;

/*
 * Events (results of Get PM Event)
 */
pub const APM_SYS_STANDBY: u32 = 0x0001;
pub const APM_SYS_SUSPEND: u32 = 0x0002;
pub const APM_NORMAL_RESUME: u32 = 0x0003;
pub const APM_CRITICAL_RESUME: u32 = 0x0004;
pub const APM_LOW_BATTERY: u32 = 0x0005;
pub const APM_POWER_STATUS_CHANGE: u32 = 0x0006;
pub const APM_UPDATE_TIME: u32 = 0x0007;
pub const APM_CRITICAL_SUSPEND: u32 = 0x0008;
pub const APM_USER_STANDBY: u32 = 0x0009;
pub const APM_USER_SUSPEND: u32 = 0x000a;
pub const APM_STANDBY_RESUME: u32 = 0x000b;
pub const APM_CAPABILITY_CHANGE: u32 = 0x000c;
pub const APM_USER_HIBERNATION: u32 = 0x000d;
pub const APM_HIBERNATION_RESUME: u32 = 0x000e;

/*
 * Error codes
 */
pub const APM_SUCCESS: u32 = 0x00;
pub const APM_DISABLED: u32 = 0x01;
pub const APM_CONNECTED: u32 = 0x02;
pub const APM_NOT_CONNECTED: u32 = 0x03;
pub const APM_16_CONNECTED: u32 = 0x05;
pub const APM_16_UNSUPPORTED: u32 = 0x06;
pub const APM_32_CONNECTED: u32 = 0x07;
pub const APM_32_UNSUPPORTED: u32 = 0x08;
pub const APM_BAD_DEVICE: u32 = 0x09;
pub const APM_BAD_PARAM: u32 = 0x0a;
pub const APM_NOT_ENGAGED: u32 = 0x0b;
pub const APM_BAD_FUNCTION: u32 = 0x0c;
pub const APM_RESUME_DISABLED: u32 = 0x0d;
pub const APM_NO_ERROR: u32 = 0x53;
pub const APM_BAD_STATE: u32 = 0x60;
pub const APM_NO_EVENTS: u32 = 0x80;
pub const APM_NOT_PRESENT: u32 = 0x86;

/*
 * APM Device IDs
 */
pub const APM_DEVICE_BIOS: u32 = 0x0000;
pub const APM_DEVICE_ALL: u32 = 0x0001;
pub const APM_DEVICE_DISPLAY: u32 = 0x0100;
pub const APM_DEVICE_STORAGE: u32 = 0x0200;
pub const APM_DEVICE_PARALLEL: u32 = 0x0300;
pub const APM_DEVICE_SERIAL: u32 = 0x0400;
pub const APM_DEVICE_NETWORK: u32 = 0x0500;
pub const APM_DEVICE_PCMCIA: u32 = 0x0600;
pub const APM_DEVICE_BATTERY: u32 = 0x8000;
pub const APM_DEVICE_OEM: u32 = 0xe000;
pub const APM_DEVICE_OLD_ALL: u32 = 0xffff;
pub const APM_DEVICE_CLASS: u32 = 0x00ff;
pub const APM_DEVICE_MASK: u32 = 0xff00;

/*
 * Battery status
 */
pub const APM_MAX_BATTERIES: u32 = 2;

/*
 * APM defined capability bit flags
 */
pub const APM_CAP_GLOBAL_STANDBY: u32 = 0x0001;
pub const APM_CAP_GLOBAL_SUSPEND: u32 = 0x0002;
pub const APM_CAP_RESUME_STANDBY_TIMER: u32 = 0x0004; /* Timer resume from standby */
pub const APM_CAP_RESUME_SUSPEND_TIMER: u32 = 0x0008; /* Timer resume from suspend */
pub const APM_CAP_RESUME_STANDBY_RING: u32 = 0x0010; /* Resume on Ring fr standby */
pub const APM_CAP_RESUME_SUSPEND_RING: u32 = 0x0020; /* Resume on Ring fr suspend */
pub const APM_CAP_RESUME_STANDBY_PCMCIA: u32 = 0x0040; /* Resume on PCMCIA Ring */
pub const APM_CAP_RESUME_SUSPEND_PCMCIA: u32 = 0x0080; /* Resume on PCMCIA Ring */

/*
 * ioctl operations
 */
// Dependency: linux/ioctl.h
pub const APM_IOC_STANDBY: _ = _IO('A', 1);
pub const APM_IOC_SUSPEND: _ = _IO('A', 2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
