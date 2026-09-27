/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * OSS compatible sequencer driver
 *
 * Copyright (C) 1998,99 Takashi Iwai
 */

/* Dependencies supplied by the surrounding translated sound subsystem. */

/*
 * argument structure for synthesizer operations
 */
#[repr(C)]
pub struct snd_seq_oss_arg {
	/* given by OSS sequencer */
	pub app_index: ::kernel::ffi::c_int, /* application unique index */
	pub file_mode: ::kernel::ffi::c_int, /* file mode - see below */
	pub seq_mode: ::kernel::ffi::c_int, /* sequencer mode - see below */

	/* following must be initialized in open callback */
	pub addr: snd_seq_addr, /* opened port address */
	pub private_data: *mut ::kernel::ffi::c_void, /* private data for lowlevel drivers */

	/* note-on event passing mode: initially given by OSS seq,
	 * but configurable by drivers - see below
	 */
	pub event_passing: ::kernel::ffi::c_int,
}

/*
 * synthesizer operation callbacks
 */
#[repr(C)]
pub struct snd_seq_oss_callback {
	pub owner: *mut module,
	pub open: Option<unsafe extern "C" fn(p: *mut snd_seq_oss_arg, closure: *mut ::kernel::ffi::c_void) -> ::kernel::ffi::c_int>,
	pub close: Option<unsafe extern "C" fn(p: *mut snd_seq_oss_arg) -> ::kernel::ffi::c_int>,
	pub ioctl: Option<unsafe extern "C" fn(p: *mut snd_seq_oss_arg, cmd: ::kernel::ffi::c_uint, arg: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int>,
	pub load_patch: Option<unsafe extern "C" fn(p: *mut snd_seq_oss_arg, format: ::kernel::ffi::c_int, buf: *const ::kernel::ffi::c_char, offs: ::kernel::ffi::c_int, count: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int>,
	pub reset: Option<unsafe extern "C" fn(p: *mut snd_seq_oss_arg) -> ::kernel::ffi::c_int>,
	pub raw_event: Option<unsafe extern "C" fn(p: *mut snd_seq_oss_arg, data: *mut u8) -> ::kernel::ffi::c_int>,
}

/* flag: file_mode */
pub const SNDRV_SEQ_OSS_FILE_ACMODE: ::kernel::ffi::c_int = 3;
pub const SNDRV_SEQ_OSS_FILE_READ: ::kernel::ffi::c_int = 1;
pub const SNDRV_SEQ_OSS_FILE_WRITE: ::kernel::ffi::c_int = 2;
pub const SNDRV_SEQ_OSS_FILE_NONBLOCK: ::kernel::ffi::c_int = 4;

/* flag: seq_mode */
pub const SNDRV_SEQ_OSS_MODE_SYNTH: ::kernel::ffi::c_int = 0;
pub const SNDRV_SEQ_OSS_MODE_MUSIC: ::kernel::ffi::c_int = 1;

/* flag: event_passing */
pub const SNDRV_SEQ_OSS_PROCESS_EVENTS: ::kernel::ffi::c_int = 0; /* key == 255 is processed as velocity change */
pub const SNDRV_SEQ_OSS_PASS_EVENTS: ::kernel::ffi::c_int = 1; /* pass all events to callback */
pub const SNDRV_SEQ_OSS_PROCESS_KEYPRESS: ::kernel::ffi::c_int = 2; /* key >= 128 will be processed as key-pressure */

/* default control rate: fixed */
pub const SNDRV_SEQ_OSS_CTRLRATE: ::kernel::ffi::c_int = 100;

/* default max queue length: configurable by module option */
pub const SNDRV_SEQ_OSS_MAX_QLEN: ::kernel::ffi::c_int = 1024;

/*
 * data pointer to snd_seq_register_device
 */
#[repr(C)]
pub struct snd_seq_oss_reg {
	pub r#type: ::kernel::ffi::c_int,
	pub subtype: ::kernel::ffi::c_int,
	pub nvoices: ::kernel::ffi::c_int,
	pub oper: snd_seq_oss_callback,
	pub private_data: *mut ::kernel::ffi::c_void,
}

/* device id */
pub const SNDRV_SEQ_DEV_ID_OSS: &[u8] = b"seq-oss\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
