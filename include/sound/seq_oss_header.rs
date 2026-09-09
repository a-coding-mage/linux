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
	pub app_index: ::core::ffi::c_int, /* application unique index */
	pub file_mode: ::core::ffi::c_int, /* file mode - see below */
	pub seq_mode: ::core::ffi::c_int, /* sequencer mode - see below */

	/* following must be initialized in open callback */
	pub addr: snd_seq_addr, /* opened port address */
	pub private_data: *mut ::core::ffi::c_void, /* private data for lowlevel drivers */

	/* note-on event passing mode: initially given by OSS seq,
	 * but configurable by drivers - see below
	 */
	pub event_passing: ::core::ffi::c_int,
}

/*
 * synthesizer operation callbacks
 */
#[repr(C)]
pub struct snd_seq_oss_callback {
	pub owner: *mut module,
	pub open: Option<unsafe extern "C" fn(p: *mut snd_seq_oss_arg, closure: *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
	pub close: Option<unsafe extern "C" fn(p: *mut snd_seq_oss_arg) -> ::core::ffi::c_int>,
	pub ioctl: Option<unsafe extern "C" fn(p: *mut snd_seq_oss_arg, cmd: ::core::ffi::c_uint, arg: ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
	pub load_patch: Option<unsafe extern "C" fn(p: *mut snd_seq_oss_arg, format: ::core::ffi::c_int, buf: *const ::core::ffi::c_char, offs: ::core::ffi::c_int, count: ::core::ffi::c_int) -> ::core::ffi::c_int>,
	pub reset: Option<unsafe extern "C" fn(p: *mut snd_seq_oss_arg) -> ::core::ffi::c_int>,
	pub raw_event: Option<unsafe extern "C" fn(p: *mut snd_seq_oss_arg, data: *mut u8) -> ::core::ffi::c_int>,
}

/* flag: file_mode */
pub const SNDRV_SEQ_OSS_FILE_ACMODE: ::core::ffi::c_int = 3;
pub const SNDRV_SEQ_OSS_FILE_READ: ::core::ffi::c_int = 1;
pub const SNDRV_SEQ_OSS_FILE_WRITE: ::core::ffi::c_int = 2;
pub const SNDRV_SEQ_OSS_FILE_NONBLOCK: ::core::ffi::c_int = 4;

/* flag: seq_mode */
pub const SNDRV_SEQ_OSS_MODE_SYNTH: ::core::ffi::c_int = 0;
pub const SNDRV_SEQ_OSS_MODE_MUSIC: ::core::ffi::c_int = 1;

/* flag: event_passing */
pub const SNDRV_SEQ_OSS_PROCESS_EVENTS: ::core::ffi::c_int = 0; /* key == 255 is processed as velocity change */
pub const SNDRV_SEQ_OSS_PASS_EVENTS: ::core::ffi::c_int = 1; /* pass all events to callback */
pub const SNDRV_SEQ_OSS_PROCESS_KEYPRESS: ::core::ffi::c_int = 2; /* key >= 128 will be processed as key-pressure */

/* default control rate: fixed */
pub const SNDRV_SEQ_OSS_CTRLRATE: ::core::ffi::c_int = 100;

/* default max queue length: configurable by module option */
pub const SNDRV_SEQ_OSS_MAX_QLEN: ::core::ffi::c_int = 1024;

/*
 * data pointer to snd_seq_register_device
 */
#[repr(C)]
pub struct snd_seq_oss_reg {
	pub r#type: ::core::ffi::c_int,
	pub subtype: ::core::ffi::c_int,
	pub nvoices: ::core::ffi::c_int,
	pub oper: snd_seq_oss_callback,
	pub private_data: *mut ::core::ffi::c_void,
}

/* device id */
pub const SNDRV_SEQ_DEV_ID_OSS: &[u8] = b"seq-oss\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
