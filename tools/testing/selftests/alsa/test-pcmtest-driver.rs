// SPDX-License-Identifier: GPL-2.0
/*
 * This is the test which covers PCM middle layer data transferring using
 * the virtual pcm test driver (snd-pcmtest).
 *
 * Copyright 2023 Ivan Orlov <ivan.orlov0322@gmail.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const CH_NUM: usize = 4;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

pub enum snd_pcm_t {}
pub enum snd_pcm_sw_params_t {}
pub enum snd_pcm_hw_params_t {}

pub type snd_pcm_access_t = c_int;
pub type snd_pcm_format_t = c_int;
pub type snd_pcm_stream_t = c_int;

pub const SND_PCM_ACCESS_RW_INTERLEAVED: snd_pcm_access_t = 3;
pub const SND_PCM_ACCESS_RW_NONINTERLEAVED: snd_pcm_access_t = 4;
pub const SND_PCM_FORMAT_S16_LE: snd_pcm_format_t = 2;
pub const SND_PCM_STREAM_PLAYBACK: snd_pcm_stream_t = 0;
pub const SND_PCM_STREAM_CAPTURE: snd_pcm_stream_t = 1;

#[repr(C)]
pub struct pattern_buf {
	pub buf: [c_char; 1024],
	pub len: c_int,
}

pub static mut patterns: [pattern_buf; CH_NUM] = [
	pattern_buf {
		buf: [0; 1024],
		len: 0,
	},
	pattern_buf {
		buf: [0; 1024],
		len: 0,
	},
	pattern_buf {
		buf: [0; 1024],
		len: 0,
	},
	pattern_buf {
		buf: [0; 1024],
		len: 0,
	},
];

#[repr(C)]
#[derive(Clone, Copy)]
pub struct pcmtest_test_params {
	pub buffer_size: c_ulong,
	pub period_size: c_ulong,
	pub channels: c_ulong,
	pub rate: c_uint,
	pub access: snd_pcm_access_t,
	pub sec_buf_len: usize,
	pub sample_size: usize,
	pub time: c_int,
	pub format: snd_pcm_format_t,
}

#[repr(C)]
pub struct pcmtest {
	pub card: c_int,
	pub swparams: *mut snd_pcm_sw_params_t,
	pub hwparams: *mut snd_pcm_hw_params_t,
	pub params: pcmtest_test_params,
}

unsafe extern "C" {
	fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
	fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
	fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn fclose(stream: *mut FILE) -> c_int;
	fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
	fn printf(format: *const c_char, ...) -> c_int;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn malloc(size: usize) -> *mut c_void;
	fn calloc(nmemb: usize, size: usize) -> *mut c_void;
	fn free(ptr: *mut c_void);
	fn geteuid() -> c_uint;

	fn snd_card_next(card: *mut c_int) -> c_int;
	fn snd_card_get_name(card: c_int, name: *mut *mut c_char) -> c_int;
	fn snd_pcm_open(
		pcmp: *mut *mut snd_pcm_t,
		name: *const c_char,
		stream: snd_pcm_stream_t,
		mode: c_int,
	) -> c_int;
	fn snd_pcm_close(pcm: *mut snd_pcm_t) -> c_int;
	fn snd_pcm_hw_params_any(pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t) -> c_int;
	fn snd_pcm_hw_params_set_rate_resample(
		pcm: *mut snd_pcm_t,
		params: *mut snd_pcm_hw_params_t,
		val: c_uint,
	) -> c_int;
	fn snd_pcm_hw_params_set_access(
		pcm: *mut snd_pcm_t,
		params: *mut snd_pcm_hw_params_t,
		access: snd_pcm_access_t,
	) -> c_int;
	fn snd_pcm_hw_params_set_format(
		pcm: *mut snd_pcm_t,
		params: *mut snd_pcm_hw_params_t,
		format: snd_pcm_format_t,
	) -> c_int;
	fn snd_pcm_hw_params_set_channels(
		pcm: *mut snd_pcm_t,
		params: *mut snd_pcm_hw_params_t,
		val: c_uint,
	) -> c_int;
	fn snd_pcm_hw_params_set_rate_near(
		pcm: *mut snd_pcm_t,
		params: *mut snd_pcm_hw_params_t,
		val: *mut c_uint,
		dir: *mut c_int,
	) -> c_int;
	fn snd_pcm_hw_params_set_period_size_near(
		pcm: *mut snd_pcm_t,
		params: *mut snd_pcm_hw_params_t,
		val: *mut c_ulong,
		dir: *mut c_int,
	) -> c_int;
	fn snd_pcm_hw_params_set_buffer_size_near(
		pcm: *mut snd_pcm_t,
		params: *mut snd_pcm_hw_params_t,
		val: *mut c_ulong,
	) -> c_int;
	fn snd_pcm_hw_params(pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t) -> c_int;
	fn snd_pcm_sw_params_current(pcm: *mut snd_pcm_t, params: *mut snd_pcm_sw_params_t)
		-> c_int;
	fn snd_pcm_sw_params_set_avail_min(
		pcm: *mut snd_pcm_t,
		params: *mut snd_pcm_sw_params_t,
		val: c_ulong,
	) -> c_int;
	fn snd_pcm_sw_params(pcm: *mut snd_pcm_t, params: *mut snd_pcm_sw_params_t) -> c_int;
	fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
	fn snd_pcm_format_set_silence(format: snd_pcm_format_t, data: *mut c_void, samples: c_uint)
		-> c_int;
	fn snd_pcm_writei(pcm: *mut snd_pcm_t, buffer: *const c_void, size: c_ulong) -> isize;
	fn snd_pcm_readi(pcm: *mut snd_pcm_t, buffer: *mut c_void, size: c_ulong) -> isize;
	fn snd_pcm_readn(pcm: *mut snd_pcm_t, bufs: *mut *mut c_void, size: c_ulong) -> isize;
	fn snd_pcm_writen(pcm: *mut snd_pcm_t, bufs: *mut *mut c_void, size: c_ulong) -> isize;
	fn snd_pcm_reset(pcm: *mut snd_pcm_t) -> c_int;

	/* External equivalents of snd_pcm_sw_params_alloca/snd_pcm_hw_params_alloca. */
	fn snd_pcm_sw_params_alloca(ptr: *mut *mut snd_pcm_sw_params_t);
	fn snd_pcm_hw_params_alloca(ptr: *mut *mut snd_pcm_hw_params_t);
}

macro_rules! ASSERT_NE {
	($left:expr, $right:expr) => {
		assert_ne!($left, $right)
	};
}

macro_rules! ASSERT_EQ {
	($left:expr, $right:expr) => {
		assert_eq!($left, $right)
	};
}

macro_rules! ASSERT_GE {
	($left:expr, $right:expr) => {
		assert!($left >= $right)
	};
}

unsafe fn read_patterns() -> c_int {
	let mut fp: *mut FILE;
	let mut fpl: *mut FILE;
	let mut i: c_int;
	let mut pf: [c_char; 64] = [0; 64];
	let mut plf: [c_char; 64] = [0; 64];

	i = 0;
	while i < CH_NUM as c_int {
		sprintf(
			plf.as_mut_ptr(),
			c"/sys/kernel/debug/pcmtest/fill_pattern%d_len".as_ptr(),
			i,
		);
		fpl = fopen(plf.as_ptr(), c"r".as_ptr());
		if fpl.is_null() {
			return -1;
		}
		fscanf(fpl, c"%u".as_ptr(), &mut patterns[i as usize].len);
		fclose(fpl);

		sprintf(
			pf.as_mut_ptr(),
			c"/sys/kernel/debug/pcmtest/fill_pattern%d".as_ptr(),
			i,
		);
		fp = fopen(pf.as_ptr(), c"r".as_ptr());
		if fp.is_null() {
			return -1;
		}
		fread(
			patterns[i as usize].buf.as_mut_ptr() as *mut c_void,
			1,
			patterns[i as usize].len as usize,
			fp,
		);
		fclose(fp);
		i += 1;
	}

	0
}

unsafe fn get_test_results(debug_name: *mut c_char) -> c_int {
	let mut result: c_int = 0;
	let mut f: *mut FILE;
	let mut fname: [c_char; 128] = [0; 128];

	sprintf(fname.as_mut_ptr(), c"/sys/kernel/debug/pcmtest/%s".as_ptr(), debug_name);

	f = fopen(fname.as_ptr(), c"r".as_ptr());
	if f.is_null() {
		printf(c"Failed to open file\n".as_ptr());
		return -1;
	}
	fscanf(f, c"%d".as_ptr(), &mut result);
	fclose(f);

	result
}

fn get_sec_buf_len(rate: c_uint, channels: c_ulong, format: snd_pcm_format_t) -> usize {
	unsafe { (rate as c_ulong * channels * snd_pcm_format_physical_width(format) as c_ulong / 8) as usize }
}

unsafe fn setup_handle(
	handle: *mut *mut snd_pcm_t,
	swparams: *mut snd_pcm_sw_params_t,
	hwparams: *mut snd_pcm_hw_params_t,
	params: *mut pcmtest_test_params,
	card: c_int,
	stream: snd_pcm_stream_t,
) -> c_int {
	let mut pcm_name: [c_char; 32] = [0; 32];
	let mut err: c_int;

	sprintf(pcm_name.as_mut_ptr(), c"hw:%d,0,0".as_ptr(), card);
	err = snd_pcm_open(handle, pcm_name.as_ptr(), stream, 0);
	if err < 0 {
		return err;
	}
	snd_pcm_hw_params_any(*handle, hwparams);
	snd_pcm_hw_params_set_rate_resample(*handle, hwparams, 0);
	snd_pcm_hw_params_set_access(*handle, hwparams, (*params).access);
	snd_pcm_hw_params_set_format(*handle, hwparams, (*params).format);
	snd_pcm_hw_params_set_channels(*handle, hwparams, (*params).channels as c_uint);
	snd_pcm_hw_params_set_rate_near(*handle, hwparams, &mut (*params).rate, core::ptr::null_mut());
	snd_pcm_hw_params_set_period_size_near(
		*handle,
		hwparams,
		&mut (*params).period_size,
		core::ptr::null_mut(),
	);
	snd_pcm_hw_params_set_buffer_size_near(*handle, hwparams, &mut (*params).buffer_size);
	snd_pcm_hw_params(*handle, hwparams);
	snd_pcm_sw_params_current(*handle, swparams);

	snd_pcm_hw_params_set_rate_resample(*handle, hwparams, 0);
	snd_pcm_sw_params_set_avail_min(*handle, swparams, (*params).period_size);
	snd_pcm_hw_params_set_buffer_size_near(*handle, hwparams, &mut (*params).buffer_size);
	snd_pcm_hw_params_set_period_size_near(
		*handle,
		hwparams,
		&mut (*params).period_size,
		core::ptr::null_mut(),
	);
	snd_pcm_sw_params(*handle, swparams);
	snd_pcm_hw_params(*handle, hwparams);

	0
}

unsafe fn pcmtest_teardown(_self: *mut pcmtest) {}

unsafe fn pcmtest_setup(self_: *mut pcmtest) {
	let mut card_name: *mut c_char;
	let mut err: c_int;

	if geteuid() != 0 {
		return;
	}

	err = read_patterns();
	if err != 0 {
		return;
	}

	card_name = malloc(127) as *mut c_char;
	ASSERT_NE!(card_name, core::ptr::null_mut());
	(*self_).params.buffer_size = 16384;
	(*self_).params.period_size = 4096;
	(*self_).params.channels = CH_NUM as c_ulong;
	(*self_).params.rate = 8000;
	(*self_).params.access = SND_PCM_ACCESS_RW_INTERLEAVED;
	(*self_).params.format = SND_PCM_FORMAT_S16_LE;
	(*self_).card = -1;
	(*self_).params.sample_size =
		(snd_pcm_format_physical_width((*self_).params.format) / 8) as usize;

	(*self_).params.sec_buf_len = get_sec_buf_len(
		(*self_).params.rate,
		(*self_).params.channels,
		(*self_).params.format,
	);
	(*self_).params.time = 4;

	while snd_card_next(&mut (*self_).card) >= 0 {
		if (*self_).card == -1 {
			break;
		}
		snd_card_get_name((*self_).card, &mut card_name);
		if strcmp(card_name, c"PCM-Test".as_ptr()) == 0 {
			break;
		}
	}
	free(card_name as *mut c_void);
	ASSERT_NE!((*self_).card, -1);
}

/*
 * Here we are trying to send the looped monotonically increasing sequence of bytes to the driver.
 * If our data isn't corrupted, the driver will set the content of 'pc_test' debugfs file to '1'
 */
unsafe fn pcmtest_playback(self_: *mut pcmtest) {
	let mut handle: *mut snd_pcm_t = core::ptr::null_mut();
	let mut it: *mut u8;
	let mut write_res: isize;
	let mut test_results: c_int;
	let mut i: c_int;
	let mut cur_ch: c_int;
	let mut pos_in_ch: c_int;
	let mut samples: *mut c_void;
	let params: *mut pcmtest_test_params = &mut (*self_).params;

	samples = calloc((*self_).params.sec_buf_len * (*self_).params.time as usize, 1);
	ASSERT_NE!(samples, core::ptr::null_mut());

	snd_pcm_sw_params_alloca(&mut (*self_).swparams);
	snd_pcm_hw_params_alloca(&mut (*self_).hwparams);

	ASSERT_EQ!(
		setup_handle(
			&mut handle,
			(*self_).swparams,
			(*self_).hwparams,
			params,
			(*self_).card,
			SND_PCM_STREAM_PLAYBACK,
		),
		0
	);
	snd_pcm_format_set_silence(
		(*params).format,
		samples,
		((*params).rate * (*params).channels as c_uint * (*params).time as c_uint) as c_uint,
	);
	it = samples as *mut u8;
	i = 0;
	while i < ((*self_).params.sec_buf_len * (*params).time as usize) as c_int {
		cur_ch = (i / (*params).sample_size as c_int) % CH_NUM as c_int;
		pos_in_ch = i / (*params).sample_size as c_int / CH_NUM as c_int
			* (*params).sample_size as c_int
			+ (i % (*params).sample_size as c_int);
		*it.add(i as usize) = patterns[cur_ch as usize].buf
			[(pos_in_ch % patterns[cur_ch as usize].len) as usize] as u8;
		i += 1;
	}
	write_res = snd_pcm_writei(
		handle,
		samples,
		((*params).rate * (*params).time as c_uint) as c_ulong,
	);
	ASSERT_GE!(write_res, 0);

	snd_pcm_close(handle);
	free(samples);
	test_results = get_test_results(c"pc_test".as_ptr() as *mut c_char);
	ASSERT_EQ!(test_results, 1);
}

/*
 * Here we test that the virtual alsa driver returns looped and monotonically increasing sequence
 * of bytes. In the interleaved mode the buffer will contain samples in the following order:
 * C0, C1, C2, C3, C0, C1, ...
 */
unsafe fn pcmtest_capture(self_: *mut pcmtest) {
	let mut handle: *mut snd_pcm_t = core::ptr::null_mut();
	let mut it: *mut u8;
	let mut read_res: isize;
	let mut i: c_int;
	let mut cur_ch: c_int;
	let mut pos_in_ch: c_int;
	let mut samples: *mut c_void;
	let params: *mut pcmtest_test_params = &mut (*self_).params;

	samples = calloc((*self_).params.sec_buf_len * (*self_).params.time as usize, 1);
	ASSERT_NE!(samples, core::ptr::null_mut());

	snd_pcm_sw_params_alloca(&mut (*self_).swparams);
	snd_pcm_hw_params_alloca(&mut (*self_).hwparams);

	ASSERT_EQ!(
		setup_handle(
			&mut handle,
			(*self_).swparams,
			(*self_).hwparams,
			params,
			(*self_).card,
			SND_PCM_STREAM_CAPTURE,
		),
		0
	);
	snd_pcm_format_set_silence(
		(*params).format,
		samples,
		((*params).rate * (*params).channels as c_uint * (*params).time as c_uint) as c_uint,
	);
	read_res = snd_pcm_readi(
		handle,
		samples,
		((*params).rate * (*params).time as c_uint) as c_ulong,
	);
	ASSERT_GE!(read_res, 0);
	snd_pcm_close(handle);
	it = samples as *mut u8;
	i = 0;
	while i < ((*self_).params.sec_buf_len * (*self_).params.time as usize) as c_int {
		cur_ch = (i / (*params).sample_size as c_int) % CH_NUM as c_int;
		pos_in_ch = i / (*params).sample_size as c_int / CH_NUM as c_int
			* (*params).sample_size as c_int
			+ (i % (*params).sample_size as c_int);
		ASSERT_EQ!(
			*it.add(i as usize),
			patterns[cur_ch as usize].buf
				[(pos_in_ch % patterns[cur_ch as usize].len) as usize] as u8
		);
		i += 1;
	}
	free(samples);
}

// Test capture in the non-interleaved access mode. The are buffers for each recorded channel
unsafe fn pcmtest_ni_capture(self_: *mut pcmtest) {
	let mut handle: *mut snd_pcm_t = core::ptr::null_mut();
	let mut params: pcmtest_test_params = (*self_).params;
	let mut chan_samples: *mut *mut c_char;
	let mut i: usize;
	let mut j: usize;
	let mut read_res: isize;

	chan_samples = calloc(CH_NUM, core::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
	ASSERT_NE!(chan_samples, core::ptr::null_mut());

	snd_pcm_sw_params_alloca(&mut (*self_).swparams);
	snd_pcm_hw_params_alloca(&mut (*self_).hwparams);

	params.access = SND_PCM_ACCESS_RW_NONINTERLEAVED;

	ASSERT_EQ!(
		setup_handle(
			&mut handle,
			(*self_).swparams,
			(*self_).hwparams,
			&mut params,
			(*self_).card,
			SND_PCM_STREAM_CAPTURE,
		),
		0
	);

	i = 0;
	while i < CH_NUM {
		*chan_samples.add(i) = calloc(params.sec_buf_len * params.time as usize, 1) as *mut c_char;
		i += 1;
	}

	i = 0;
	while i < 1 {
		read_res = snd_pcm_readn(
			handle,
			chan_samples as *mut *mut c_void,
			(params.rate * params.time as c_uint) as c_ulong,
		);
		ASSERT_GE!(read_res, 0);
		i += 1;
	}
	snd_pcm_close(handle);

	i = 0;
	while i < CH_NUM {
		j = 0;
		while j < (params.rate * params.time as c_uint) as usize {
			ASSERT_EQ!(
				*(*chan_samples.add(i)).add(j),
				patterns[i].buf[j % patterns[i].len as usize]
			);
			j += 1;
		}
		free(*chan_samples.add(i) as *mut c_void);
		i += 1;
	}
	free(chan_samples as *mut c_void);
}

unsafe fn pcmtest_ni_playback(self_: *mut pcmtest) {
	let mut handle: *mut snd_pcm_t = core::ptr::null_mut();
	let mut params: pcmtest_test_params = (*self_).params;
	let mut chan_samples: *mut *mut c_char;
	let mut i: usize;
	let mut j: usize;
	let mut read_res: isize;
	let mut test_res: c_int;

	chan_samples = calloc(CH_NUM, core::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
	ASSERT_NE!(chan_samples, core::ptr::null_mut());

	snd_pcm_sw_params_alloca(&mut (*self_).swparams);
	snd_pcm_hw_params_alloca(&mut (*self_).hwparams);

	params.access = SND_PCM_ACCESS_RW_NONINTERLEAVED;

	ASSERT_EQ!(
		setup_handle(
			&mut handle,
			(*self_).swparams,
			(*self_).hwparams,
			&mut params,
			(*self_).card,
			SND_PCM_STREAM_PLAYBACK,
		),
		0
	);

	i = 0;
	while i < CH_NUM {
		*chan_samples.add(i) = calloc(params.sec_buf_len * params.time as usize, 1) as *mut c_char;
		j = 0;
		while j < params.sec_buf_len * params.time as usize {
			*(*chan_samples.add(i)).add(j) = patterns[i].buf[j % patterns[i].len as usize];
			j += 1;
		}
		i += 1;
	}

	i = 0;
	while i < 1 {
		read_res = snd_pcm_writen(
			handle,
			chan_samples as *mut *mut c_void,
			(params.rate * params.time as c_uint) as c_ulong,
		);
		ASSERT_GE!(read_res, 0);
		i += 1;
	}

	snd_pcm_close(handle);
	test_res = get_test_results(c"pc_test".as_ptr() as *mut c_char);
	ASSERT_EQ!(test_res, 1);

	i = 0;
	while i < CH_NUM {
		free(*chan_samples.add(i) as *mut c_void);
		i += 1;
	}
	free(chan_samples as *mut c_void);
}

/*
 * Here we are testing the custom ioctl definition inside the virtual driver. If it triggers
 * successfully, the driver sets the content of 'ioctl_test' debugfs file to '1'.
 */
unsafe fn pcmtest_reset_ioctl(self_: *mut pcmtest) {
	let mut handle: *mut snd_pcm_t = core::ptr::null_mut();
	let mut test_res: c_int;
	let params: *mut pcmtest_test_params = &mut (*self_).params;

	snd_pcm_sw_params_alloca(&mut (*self_).swparams);
	snd_pcm_hw_params_alloca(&mut (*self_).hwparams);

	ASSERT_EQ!(
		setup_handle(
			&mut handle,
			(*self_).swparams,
			(*self_).hwparams,
			params,
			(*self_).card,
			SND_PCM_STREAM_CAPTURE,
		),
		0
	);
	snd_pcm_reset(handle);
	test_res = get_test_results(c"ioctl_test".as_ptr() as *mut c_char);
	ASSERT_EQ!(test_res, 1);
	snd_pcm_close(handle);
}

/* TEST_HARNESS_MAIN */
