// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022-2024 Red Hat */

/* Translated from hid_common.h, linux/input.h, string.h, and sys/ioctl.h uses. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;

type __u8 = u8;

#[repr(C)]
pub struct pthread_t {
	_private: [u8; 0],
}

#[repr(C)]
pub struct pthread_mutex_t {
	_private: [u8; 0],
}

#[repr(C)]
pub struct pthread_cond_t {
	_private: [u8; 0],
}

#[repr(C)]
pub struct timespec {
	pub tv_sec: c_long,
	pub tv_nsec: c_long,
}

#[repr(C)]
pub struct pollfd {
	pub fd: c_int,
	pub events: c_short,
	pub revents: c_short,
}

type c_short = i16;

#[repr(C)]
pub struct uhid_device {
	pub tid: pthread_t,
	pub dev_id: c_int,
}

#[repr(C)]
pub struct hidraw_report_descriptor {
	pub size: c_uint,
	pub value: [__u8; 4096],
}

#[repr(C)]
pub struct hidraw_devinfo {
	pub bustype: c_uint,
	pub vendor: c_short,
	pub product: c_short,
}

#[repr(C)]
pub struct hidraw {
	pub hid: uhid_device,
	pub hidraw_fd: c_int,
}

unsafe extern "C" {
	static rdesc: [__u8; 0];
	static feature_data: [__u8; 0];
	static mut errno: c_int;
	static mut uhid_output_mtx: pthread_mutex_t;
	static mut uhid_output_cond: pthread_cond_t;
	static mut output_report: [__u8; 0];

	fn close(fd: c_int) -> c_int;
	fn uhid_destroy(metadata: *mut c_void, hid: *mut uhid_device);
	fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
	fn setup_uhid(
		metadata: *mut c_void,
		hid: *mut uhid_device,
		bustype: c_uint,
		vendor: c_uint,
		product: c_uint,
		rdesc: *const __u8,
		rdesc_size: usize,
	) -> c_int;
	fn open_hidraw(hid: *mut uhid_device) -> c_int;
	fn uhid_send_event(metadata: *mut c_void, hid: *mut uhid_device, buf: *mut __u8, size: usize);
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
	fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
	fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
	fn poll(fds: *mut pollfd, nfds: c_ulong, timeout: c_int) -> c_int;
	fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
	fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
	fn pthread_cond_timedwait(
		cond: *mut pthread_cond_t,
		mutex: *mut pthread_mutex_t,
		abstime: *const timespec,
	) -> c_int;
	fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
	fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
	fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
	fn strlen(s: *const c_char) -> usize;
	fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

unsafe extern "C" {
	fn TH_LOG(fmt: *const c_char, ...);
	fn ASSERT_OK(value: c_int);
	fn ASSERT_GE(left: c_int, right: c_int);
	fn ASSERT_GT(left: c_int, right: c_int);
	fn ASSERT_LT(left: c_int, right: c_int);
	fn ASSERT_EQ_i32(left: c_int, right: c_int);
	fn ASSERT_EQ_usize(left: usize, right: usize);
	fn ASSERT_EQ_u8(left: __u8, right: __u8);
	fn ASSERT_TRUE(value: bool);
}

const BUS_USB: c_uint = 0x03;
const ENODEV: c_int = 19;
const EIO: c_int = 5;
const ENOTTY: c_int = 25;
const EINVAL: c_int = 22;
const POLLIN: c_short = 0x0001;
const POLLHUP: c_short = 0x0010;
const CLOCK_REALTIME: c_int = 0;

const IOC_NRBITS: c_uint = 8;
const IOC_TYPEBITS: c_uint = 8;
const IOC_SIZEBITS: c_uint = 14;
const IOC_DIRBITS: c_uint = 2;

const IOC_NRSHIFT: c_uint = 0;
const IOC_TYPESHIFT: c_uint = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: c_uint = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: c_uint = IOC_SIZESHIFT + IOC_SIZEBITS;

const IOC_NONE: c_uint = 0;
const IOC_WRITE: c_uint = 1;
const IOC_READ: c_uint = 2;

const fn _IOC(dir: c_uint, type_: c_uint, nr: c_uint, size: usize) -> c_ulong {
	((dir as c_ulong) << IOC_DIRSHIFT)
		| ((type_ as c_ulong) << IOC_TYPESHIFT)
		| ((nr as c_ulong) << IOC_NRSHIFT)
		| ((size as c_ulong) << IOC_SIZESHIFT)
}

const fn _IOW(type_: c_uint, nr: c_uint, size: usize) -> c_ulong {
	_IOC(IOC_WRITE, type_, nr, size)
}

/* for older kernels */
const HIDIOCREVOKE: c_ulong = _IOW(b'H' as c_uint, 0x0D, size_of::<c_int>()); /* Revoke device access */

const HIDIOCGRDESCSIZE: c_ulong = _IOC(IOC_READ, b'H' as c_uint, 0x01, size_of::<c_int>());
const HIDIOCGRDESC: c_ulong = _IOC(
	IOC_READ,
	b'H' as c_uint,
	0x02,
	size_of::<hidraw_report_descriptor>(),
);
const HIDIOCGRAWINFO: c_ulong = _IOC(IOC_READ, b'H' as c_uint, 0x03, size_of::<hidraw_devinfo>());

const fn HIDIOCGRAWNAME(len: usize) -> c_ulong {
	_IOC(IOC_READ, b'H' as c_uint, 0x04, len)
}

const fn HIDIOCGRAWPHYS(len: usize) -> c_ulong {
	_IOC(IOC_READ, b'H' as c_uint, 0x05, len)
}

const fn HIDIOCGRAWUNIQ(len: usize) -> c_ulong {
	_IOC(IOC_READ, b'H' as c_uint, 0x08, len)
}

const fn HIDIOCGFEATURE(len: usize) -> c_ulong {
	_IOC(IOC_WRITE | IOC_READ, b'H' as c_uint, 0x07, len)
}

const fn HIDIOCSFEATURE(len: usize) -> c_ulong {
	_IOC(IOC_WRITE | IOC_READ, b'H' as c_uint, 0x06, len)
}

const fn HIDIOCGINPUT(len: usize) -> c_ulong {
	_IOC(IOC_WRITE | IOC_READ, b'H' as c_uint, 0x0A, len)
}

const fn HIDIOCSINPUT(len: usize) -> c_ulong {
	_IOC(IOC_WRITE | IOC_READ, b'H' as c_uint, 0x09, len)
}

const fn HIDIOCGOUTPUT(len: usize) -> c_ulong {
	_IOC(IOC_WRITE | IOC_READ, b'H' as c_uint, 0x0C, len)
}

const fn HIDIOCSOUTPUT(len: usize) -> c_ulong {
	_IOC(IOC_WRITE | IOC_READ, b'H' as c_uint, 0x0B, len)
}

unsafe fn rdesc_len() -> usize {
	size_of_val(&rdesc)
}

unsafe fn feature_data_len() -> usize {
	size_of_val(&feature_data)
}

unsafe fn output_report_len() -> usize {
	size_of_val(&output_report)
}

fn size_of_val<T: ?Sized>(val: &T) -> usize {
	core::mem::size_of_val(val)
}

static READ_HIDRAW: &[u8] = b"read_hidraw\0";
static OPEN_HIDRAW: &[u8] = b"open_hidraw\0";
static POLL_RETURN_VALUE: &[u8] = b"poll return value\0";
static COULDNT_REVOKE: &[u8] = b"couldn't revoke the hidraw fd\0";

static UNEXPECTED_READ_ERR: &[u8] =
	b"unexpected error code while reading the hidraw node: %d\0";
static IOCTL_HIDRAW: &[u8] = b"ioctl_hidraw\0";
static UNEXPECTED_IOCTL_ERR: &[u8] = b"unexpected error code while doing an ioctl: %d\0";
static UNEXPECTED_WRITE_ERR: &[u8] =
	b"unexpected error while writing to hidraw node: %d\0";
static CONDITION_WAIT_ERR: &[u8] = b"error while calling waiting for the condition\0";
static UNEXPECTED_WRITE_SUCCESS: &[u8] =
	b"unexpected success while writing to hidraw node: %d\0";
static UNEXPECTED_WRITE_ERRNO: &[u8] =
	b"unexpected error code while writing to hidraw node: %d\0";

unsafe fn close_hidraw(self_: *mut hidraw) {
	if (*self_).hidraw_fd != 0 {
		close((*self_).hidraw_fd);
	}
	(*self_).hidraw_fd = 0;
}

unsafe fn hidraw_teardown(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut uhid_err: *mut c_void = core::ptr::null_mut();

	uhid_destroy(_metadata, &mut (*self_).hid);

	close_hidraw(self_);
	pthread_join((*self_).hid.tid, &mut uhid_err);
}

macro_rules! TEARDOWN_LOG {
	($metadata:expr, $self:expr, $variant:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
		TH_LOG($fmt.as_ptr() as *const c_char $(, $arg)*);
		hidraw_teardown($metadata, $self, $variant);
	}};
}

unsafe fn hidraw_setup(_metadata: *mut c_void, self_: *mut hidraw) {
	let mut err: c_int;

	err = setup_uhid(
		_metadata,
		&mut (*self_).hid,
		BUS_USB,
		0x0001,
		0x0a37,
		rdesc.as_ptr(),
		rdesc_len(),
	);
	ASSERT_OK(err);

	(*self_).hidraw_fd = open_hidraw(&mut (*self_).hid);
	ASSERT_GE((*self_).hidraw_fd, 0);
	TH_LOG(OPEN_HIDRAW.as_ptr() as *const c_char);
}

/*
 * A simple test to see if the fixture is working fine.
 * If this fails, none of the other tests will pass.
 */
unsafe fn test_create_uhid(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {}

/*
 * Inject one event in the uhid device,
 * check that we get the same data through hidraw
 */
unsafe fn raw_event(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;

	/* inject one event */
	buf[0] = 1;
	buf[1] = 42;
	uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

	/* read the data from hidraw */
	memset(buf.as_mut_ptr() as *mut c_void, 0, size_of_val(&buf));
	err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf)) as c_int;
	ASSERT_EQ_i32(err, 6);
	TH_LOG(READ_HIDRAW.as_ptr() as *const c_char);
	ASSERT_EQ_u8(buf[0], 1);
	ASSERT_EQ_u8(buf[1], 42);
}

/*
 * After initial opening/checks of hidraw, revoke the hidraw
 * node and check that we can not read any more data.
 */
unsafe fn raw_event_revoked(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;

	/* inject one event */
	buf[0] = 1;
	buf[1] = 42;
	uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

	/* read the data from hidraw */
	memset(buf.as_mut_ptr() as *mut c_void, 0, size_of_val(&buf));
	err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf)) as c_int;
	ASSERT_EQ_i32(err, 6);
	TH_LOG(READ_HIDRAW.as_ptr() as *const c_char);
	ASSERT_EQ_u8(buf[0], 1);
	ASSERT_EQ_u8(buf[1], 42);

	/* call the revoke ioctl */
	err = ioctl((*self_).hidraw_fd, HIDIOCREVOKE, core::ptr::null_mut::<c_void>());
	ASSERT_OK(err);
	TH_LOG(COULDNT_REVOKE.as_ptr() as *const c_char);

	/* inject one other event */
	buf[0] = 1;
	buf[1] = 43;
	uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

	/* read the data from hidraw */
	memset(buf.as_mut_ptr() as *mut c_void, 0, size_of_val(&buf));
	err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf)) as c_int;
	ASSERT_EQ_i32(err, -1);
	TH_LOG(READ_HIDRAW.as_ptr() as *const c_char);
	ASSERT_EQ_i32(errno, ENODEV);
	TH_LOG(UNEXPECTED_READ_ERR.as_ptr() as *const c_char, errno);
}

/*
 * Revoke the hidraw node and check that we can not do any ioctl.
 */
unsafe fn ioctl_revoked(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut desc_size: c_int = 0;
	let mut err: c_int;

	/* call the revoke ioctl */
	err = ioctl((*self_).hidraw_fd, HIDIOCREVOKE, core::ptr::null_mut::<c_void>());
	ASSERT_OK(err);
	TH_LOG(COULDNT_REVOKE.as_ptr() as *const c_char);

	/* do an ioctl */
	err = ioctl((*self_).hidraw_fd, HIDIOCGRDESCSIZE, &mut desc_size);
	ASSERT_EQ_i32(err, -1);
	TH_LOG(IOCTL_HIDRAW.as_ptr() as *const c_char);
	ASSERT_EQ_i32(errno, ENODEV);
	TH_LOG(UNEXPECTED_IOCTL_ERR.as_ptr() as *const c_char, errno);
}

/*
 * Setup polling of the fd, and check that revoke works properly.
 */
unsafe fn poll_revoked(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut pfds: [pollfd; 1] = [pollfd {
		fd: 0,
		events: 0,
		revents: 0,
	}; 1];
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;
	let mut ready: c_int;

	/* setup polling */
	pfds[0].fd = (*self_).hidraw_fd;
	pfds[0].events = POLLIN;

	/* inject one event */
	buf[0] = 1;
	buf[1] = 42;
	uhid_send_event(_metadata, &mut (*self_).hid, buf.as_mut_ptr(), 6);

	while true {
		ready = poll(pfds.as_mut_ptr(), 1, 5000);
		ASSERT_EQ_i32(ready, 1);
		TH_LOG(POLL_RETURN_VALUE.as_ptr() as *const c_char);

		if (pfds[0].revents & POLLIN) != 0 {
			memset(buf.as_mut_ptr() as *mut c_void, 0, size_of_val(&buf));
			err = read((*self_).hidraw_fd, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf))
				as c_int;
			ASSERT_EQ_i32(err, 6);
			TH_LOG(READ_HIDRAW.as_ptr() as *const c_char);
			ASSERT_EQ_u8(buf[0], 1);
			ASSERT_EQ_u8(buf[1], 42);

			/* call the revoke ioctl */
			err = ioctl((*self_).hidraw_fd, HIDIOCREVOKE, core::ptr::null_mut::<c_void>());
			ASSERT_OK(err);
			TH_LOG(COULDNT_REVOKE.as_ptr() as *const c_char);
		} else {
			break;
		}
	}

	ASSERT_TRUE((pfds[0].revents & POLLHUP) != 0);
}

/*
 * After initial opening/checks of hidraw, revoke the hidraw
 * node and check that we can not read any more data.
 */
unsafe fn write_event_revoked(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut time_to_wait: timespec = core::mem::zeroed();
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;

	/* inject one event from hidraw */
	buf[0] = 1; /* report ID */
	buf[1] = 2;
	buf[2] = 42;

	pthread_mutex_lock(&mut uhid_output_mtx);

	memset(output_report.as_mut_ptr() as *mut c_void, 0, output_report_len());
	clock_gettime(CLOCK_REALTIME, &mut time_to_wait);
	time_to_wait.tv_sec += 2;

	err = write((*self_).hidraw_fd, buf.as_ptr() as *const c_void, 3) as c_int;
	ASSERT_EQ_i32(err, 3);
	TH_LOG(UNEXPECTED_WRITE_ERR.as_ptr() as *const c_char, err);

	err = pthread_cond_timedwait(&mut uhid_output_cond, &mut uhid_output_mtx, &time_to_wait);
	ASSERT_OK(err);
	TH_LOG(CONDITION_WAIT_ERR.as_ptr() as *const c_char);

	ASSERT_EQ_u8(output_report[0], 1);
	ASSERT_EQ_u8(output_report[1], 2);
	ASSERT_EQ_u8(output_report[2], 42);

	/* call the revoke ioctl */
	err = ioctl((*self_).hidraw_fd, HIDIOCREVOKE, core::ptr::null_mut::<c_void>());
	ASSERT_OK(err);
	TH_LOG(COULDNT_REVOKE.as_ptr() as *const c_char);

	/* inject one other event */
	buf[0] = 1;
	buf[1] = 43;
	err = write((*self_).hidraw_fd, buf.as_ptr() as *const c_void, 3) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(UNEXPECTED_WRITE_SUCCESS.as_ptr() as *const c_char, err);
	ASSERT_EQ_i32(errno, ENODEV);
	TH_LOG(UNEXPECTED_WRITE_ERRNO.as_ptr() as *const c_char, errno);

	pthread_mutex_unlock(&mut uhid_output_mtx);
}

/*
 * Test HIDIOCGRDESCSIZE ioctl to get report descriptor size
 */
unsafe fn ioctl_rdescsize(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut desc_size: c_int = 0;
	let mut err: c_int;

	/* call HIDIOCGRDESCSIZE ioctl */
	err = ioctl((*self_).hidraw_fd, HIDIOCGRDESCSIZE, &mut desc_size);
	ASSERT_EQ_i32(err, 0);
	TH_LOG(b"HIDIOCGRDESCSIZE ioctl failed\0".as_ptr() as *const c_char);

	/* verify the size matches our test report descriptor */
	ASSERT_EQ_i32(desc_size, rdesc_len() as c_int);
	TH_LOG(
		b"expected size %zu, got %d\0".as_ptr() as *const c_char,
		rdesc_len(),
		desc_size,
	);
}

/*
 * Test HIDIOCGRDESC ioctl to get report descriptor data
 */
unsafe fn ioctl_rdesc(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut desc: hidraw_report_descriptor = core::mem::zeroed();
	let mut err: c_int;

	/* get the full report descriptor */
	desc.size = rdesc_len() as c_uint;
	err = ioctl((*self_).hidraw_fd, HIDIOCGRDESC, &mut desc);
	ASSERT_EQ_i32(err, 0);
	TH_LOG(b"HIDIOCGRDESC ioctl failed\0".as_ptr() as *const c_char);

	/* verify the descriptor data matches our test descriptor */
	ASSERT_EQ_i32(
		memcmp(desc.value.as_ptr() as *const c_void, rdesc.as_ptr() as *const c_void, rdesc_len()),
		0,
	);
	TH_LOG(b"report descriptor data mismatch\0".as_ptr() as *const c_char);
}

/*
 * Test HIDIOCGRDESC ioctl with smaller buffer size
 */
unsafe fn ioctl_rdesc_small_buffer(
	_metadata: *mut c_void,
	self_: *mut hidraw,
	_variant: *mut c_void,
) {
	let mut desc: hidraw_report_descriptor = core::mem::zeroed();
	let mut err: c_int;
	let small_size: usize = rdesc_len() / 2; /* request half the descriptor size */

	/* get partial report descriptor */
	desc.size = small_size as c_uint;
	err = ioctl((*self_).hidraw_fd, HIDIOCGRDESC, &mut desc);
	ASSERT_EQ_i32(err, 0);
	TH_LOG(b"HIDIOCGRDESC ioctl failed with small buffer\0".as_ptr() as *const c_char);

	/* verify we got the first part of the descriptor */
	ASSERT_EQ_i32(
		memcmp(desc.value.as_ptr() as *const c_void, rdesc.as_ptr() as *const c_void, small_size),
		0,
	);
	TH_LOG(b"partial report descriptor data mismatch\0".as_ptr() as *const c_char);
}

/*
 * Test HIDIOCGRAWINFO ioctl to get device information
 */
unsafe fn ioctl_rawinfo(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut devinfo: hidraw_devinfo = core::mem::zeroed();
	let mut err: c_int;

	/* get device info */
	err = ioctl((*self_).hidraw_fd, HIDIOCGRAWINFO, &mut devinfo);
	ASSERT_EQ_i32(err, 0);
	TH_LOG(b"HIDIOCGRAWINFO ioctl failed\0".as_ptr() as *const c_char);

	/* verify device info matches our test setup */
	ASSERT_EQ_i32(devinfo.bustype as c_int, BUS_USB as c_int);
	TH_LOG(b"expected bustype 0x03, got 0x%x\0".as_ptr() as *const c_char, devinfo.bustype);
	ASSERT_EQ_i32(devinfo.vendor as c_int, 0x0001);
	TH_LOG(b"expected vendor 0x0001, got 0x%x\0".as_ptr() as *const c_char, devinfo.vendor as c_int);
	ASSERT_EQ_i32(devinfo.product as c_int, 0x0a37);
	TH_LOG(
		b"expected product 0x0a37, got 0x%x\0".as_ptr() as *const c_char,
		devinfo.product as c_int,
	);
}

/*
 * Test HIDIOCGFEATURE ioctl to get feature report
 */
unsafe fn ioctl_gfeature(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;

	/* set report ID 1 in first byte */
	buf[0] = 1;

	/* get feature report */
	err = ioctl((*self_).hidraw_fd, HIDIOCGFEATURE(size_of_val(&buf)), buf.as_mut_ptr()) as c_int;
	ASSERT_EQ_i32(err, feature_data_len() as c_int);
	TH_LOG(b"HIDIOCGFEATURE ioctl failed, got %d\0".as_ptr() as *const c_char, err);

	/* verify we got the expected feature data */
	ASSERT_EQ_u8(buf[0], feature_data[0]);
	TH_LOG(
		b"expected feature_data[0] = %d, got %d\0".as_ptr() as *const c_char,
		feature_data[0] as c_int,
		buf[0] as c_int,
	);
	ASSERT_EQ_u8(buf[1], feature_data[1]);
	TH_LOG(
		b"expected feature_data[1] = %d, got %d\0".as_ptr() as *const c_char,
		feature_data[1] as c_int,
		buf[1] as c_int,
	);
}

/*
 * Test HIDIOCGFEATURE ioctl with invalid report ID
 */
unsafe fn ioctl_gfeature_invalid(
	_metadata: *mut c_void,
	self_: *mut hidraw,
	_variant: *mut c_void,
) {
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;

	/* set invalid report ID (not 1) */
	buf[0] = 2;

	/* try to get feature report */
	err = ioctl((*self_).hidraw_fd, HIDIOCGFEATURE(size_of_val(&buf)), buf.as_mut_ptr()) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(b"HIDIOCGFEATURE should have failed with invalid report ID\0".as_ptr() as *const c_char);
	ASSERT_EQ_i32(errno, EIO);
	TH_LOG(b"expected EIO, got errno %d\0".as_ptr() as *const c_char, errno);
}

/*
 * Test ioctl with incorrect nr bits
 */
unsafe fn ioctl_invalid_nr(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut buf: [c_char; 256] = [0; 256];
	let mut err: c_int;
	let mut bad_cmd: c_uint;

	/*
	 * craft an ioctl command with wrong _IOC_NR bits
	 */
	bad_cmd = _IOC(IOC_WRITE | IOC_READ, b'H' as c_uint, 0x00, size_of_val(&buf)) as c_uint; /* 0 is not valid */

	/* test the ioctl */
	err = ioctl((*self_).hidraw_fd, bad_cmd as c_ulong, buf.as_mut_ptr()) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(
		b"ioctl read-write with wrong _IOC_NR (0) should have failed\0".as_ptr() as *const c_char,
	);
	ASSERT_EQ_i32(errno, ENOTTY);
	TH_LOG(
		b"expected ENOTTY for wrong read-write _IOC_NR (0), got errno %d\0".as_ptr()
			as *const c_char,
		errno,
	);

	/*
	 * craft an ioctl command with wrong _IOC_NR bits
	 */
	bad_cmd = _IOC(IOC_READ, b'H' as c_uint, 0x00, size_of_val(&buf)) as c_uint; /* 0 is not valid */

	/* test the ioctl */
	err = ioctl((*self_).hidraw_fd, bad_cmd as c_ulong, buf.as_mut_ptr()) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(b"ioctl read-only with wrong _IOC_NR (0) should have failed\0".as_ptr() as *const c_char);
	ASSERT_EQ_i32(errno, ENOTTY);
	TH_LOG(
		b"expected ENOTTY for wrong read-only _IOC_NR (0), got errno %d\0".as_ptr()
			as *const c_char,
		errno,
	);

	/* also test with bigger number */
	bad_cmd = _IOC(IOC_READ, b'H' as c_uint, 0x42, size_of_val(&buf)) as c_uint; /* 0x42 is not valid as well */

	err = ioctl((*self_).hidraw_fd, bad_cmd as c_ulong, buf.as_mut_ptr()) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(
		b"ioctl read-only with wrong _IOC_NR (0x42) should have failed\0".as_ptr()
			as *const c_char,
	);
	ASSERT_EQ_i32(errno, ENOTTY);
	TH_LOG(
		b"expected ENOTTY for wrong read-only _IOC_NR (0x42), got errno %d\0".as_ptr()
			as *const c_char,
		errno,
	);

	/* also test with bigger number: 0x42 is not valid as well */
	bad_cmd = _IOC(IOC_WRITE | IOC_READ, b'H' as c_uint, 0x42, size_of_val(&buf)) as c_uint;

	err = ioctl((*self_).hidraw_fd, bad_cmd as c_ulong, buf.as_mut_ptr()) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(
		b"ioctl read-write with wrong _IOC_NR (0x42) should have failed\0".as_ptr()
			as *const c_char,
	);
	ASSERT_EQ_i32(errno, ENOTTY);
	TH_LOG(
		b"expected ENOTTY for wrong read-write _IOC_NR (0x42), got errno %d\0".as_ptr()
			as *const c_char,
		errno,
	);
}

/*
 * Test ioctl with incorrect type bits
 */
unsafe fn ioctl_invalid_type(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut buf: [c_char; 256] = [0; 256];
	let mut err: c_int;
	let mut bad_cmd: c_uint;

	/*
	 * craft an ioctl command with wrong _IOC_TYPE bits
	 */
	bad_cmd = _IOC(IOC_WRITE | IOC_READ, b'I' as c_uint, 0x01, size_of_val(&buf)) as c_uint; /* 'I' should be 'H' */

	/* test the ioctl */
	err = ioctl((*self_).hidraw_fd, bad_cmd as c_ulong, buf.as_mut_ptr()) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(b"ioctl with wrong _IOC_TYPE (I) should have failed\0".as_ptr() as *const c_char);
	ASSERT_EQ_i32(errno, EINVAL);
	TH_LOG(b"expected EINVAL for wrong _IOC_NR, got errno %d\0".as_ptr() as *const c_char, errno);
}

/*
 * Test HIDIOCGFEATURE ioctl with incorrect _IOC_DIR bits
 */
unsafe fn ioctl_gfeature_invalid_dir(
	_metadata: *mut c_void,
	self_: *mut hidraw,
	_variant: *mut c_void,
) {
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;
	let mut bad_cmd: c_uint;

	/* set report ID 1 in first byte */
	buf[0] = 1;

	/*
	 * craft an ioctl command with wrong _IOC_DIR bits
	 * HIDIOCGFEATURE should have _IOC_WRITE|_IOC_READ, let's use only _IOC_WRITE
	 */
	bad_cmd = _IOC(IOC_WRITE, b'H' as c_uint, 0x07, size_of_val(&buf)) as c_uint; /* should be _IOC_WRITE|_IOC_READ */

	/* try to get feature report with wrong direction bits */
	err = ioctl((*self_).hidraw_fd, bad_cmd as c_ulong, buf.as_mut_ptr()) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(b"HIDIOCGFEATURE with wrong _IOC_DIR should have failed\0".as_ptr() as *const c_char);
	ASSERT_EQ_i32(errno, EINVAL);
	TH_LOG(b"expected EINVAL for wrong _IOC_DIR, got errno %d\0".as_ptr() as *const c_char, errno);

	/* also test with only _IOC_READ */
	bad_cmd = _IOC(IOC_READ, b'H' as c_uint, 0x07, size_of_val(&buf)) as c_uint; /* should be _IOC_WRITE|_IOC_READ */

	err = ioctl((*self_).hidraw_fd, bad_cmd as c_ulong, buf.as_mut_ptr()) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(b"HIDIOCGFEATURE with wrong _IOC_DIR should have failed\0".as_ptr() as *const c_char);
	ASSERT_EQ_i32(errno, EINVAL);
	TH_LOG(b"expected EINVAL for wrong _IOC_DIR, got errno %d\0".as_ptr() as *const c_char, errno);
}

/*
 * Test read-only ioctl with incorrect _IOC_DIR bits
 */
unsafe fn ioctl_readonly_invalid_dir(
	_metadata: *mut c_void,
	self_: *mut hidraw,
	_variant: *mut c_void,
) {
	let mut buf: [c_char; 256] = [0; 256];
	let mut err: c_int;
	let mut bad_cmd: c_uint;

	/*
	 * craft an ioctl command with wrong _IOC_DIR bits
	 * HIDIOCGRAWNAME should have _IOC_READ, let's use _IOC_WRITE
	 */
	bad_cmd = _IOC(IOC_WRITE, b'H' as c_uint, 0x04, size_of_val(&buf)) as c_uint; /* should be _IOC_READ */

	/* try to get device name with wrong direction bits */
	err = ioctl((*self_).hidraw_fd, bad_cmd as c_ulong, buf.as_mut_ptr()) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(b"HIDIOCGRAWNAME with wrong _IOC_DIR should have failed\0".as_ptr() as *const c_char);
	ASSERT_EQ_i32(errno, EINVAL);
	TH_LOG(b"expected EINVAL for wrong _IOC_DIR, got errno %d\0".as_ptr() as *const c_char, errno);

	/* also test with _IOC_WRITE|_IOC_READ */
	bad_cmd = _IOC(IOC_WRITE | IOC_READ, b'H' as c_uint, 0x04, size_of_val(&buf)) as c_uint; /* should be only _IOC_READ */

	err = ioctl((*self_).hidraw_fd, bad_cmd as c_ulong, buf.as_mut_ptr()) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(b"HIDIOCGRAWNAME with wrong _IOC_DIR should have failed\0".as_ptr() as *const c_char);
	ASSERT_EQ_i32(errno, EINVAL);
	TH_LOG(b"expected EINVAL for wrong _IOC_DIR, got errno %d\0".as_ptr() as *const c_char, errno);
}

/*
 * Test HIDIOCSFEATURE ioctl to set feature report
 */
unsafe fn ioctl_sfeature(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;

	/* prepare feature report data */
	buf[0] = 1; /* report ID */
	buf[1] = 0x42;
	buf[2] = 0x24;

	/* set feature report */
	err = ioctl((*self_).hidraw_fd, HIDIOCSFEATURE(3), buf.as_mut_ptr()) as c_int;
	ASSERT_EQ_i32(err, 3);
	TH_LOG(b"HIDIOCSFEATURE ioctl failed, got %d\0".as_ptr() as *const c_char, err);

	/*
	 * Note: The uhid mock doesn't validate the set report data,
	 * so we just verify the ioctl succeeds
	 */
}

/*
 * Test HIDIOCGINPUT ioctl to get input report
 */
unsafe fn ioctl_ginput(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;

	/* set report ID 1 in first byte */
	buf[0] = 1;

	/* get input report */
	err = ioctl((*self_).hidraw_fd, HIDIOCGINPUT(size_of_val(&buf)), buf.as_mut_ptr()) as c_int;
	ASSERT_EQ_i32(err, feature_data_len() as c_int);
	TH_LOG(b"HIDIOCGINPUT ioctl failed, got %d\0".as_ptr() as *const c_char, err);

	/* verify we got the expected input data */
	ASSERT_EQ_u8(buf[0], feature_data[0]);
	TH_LOG(
		b"expected feature_data[0] = %d, got %d\0".as_ptr() as *const c_char,
		feature_data[0] as c_int,
		buf[0] as c_int,
	);
	ASSERT_EQ_u8(buf[1], feature_data[1]);
	TH_LOG(
		b"expected feature_data[1] = %d, got %d\0".as_ptr() as *const c_char,
		feature_data[1] as c_int,
		buf[1] as c_int,
	);
}

/*
 * Test HIDIOCGINPUT ioctl with invalid report ID
 */
unsafe fn ioctl_ginput_invalid(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;

	/* set invalid report ID (not 1) */
	buf[0] = 2;

	/* try to get input report */
	err = ioctl((*self_).hidraw_fd, HIDIOCGINPUT(size_of_val(&buf)), buf.as_mut_ptr()) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(b"HIDIOCGINPUT should have failed with invalid report ID\0".as_ptr() as *const c_char);
	ASSERT_EQ_i32(errno, EIO);
	TH_LOG(b"expected EIO, got errno %d\0".as_ptr() as *const c_char, errno);
}

/*
 * Test HIDIOCSINPUT ioctl to set input report
 */
unsafe fn ioctl_sinput(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;

	/* prepare input report data */
	buf[0] = 1; /* report ID */
	buf[1] = 0x55;
	buf[2] = 0xAA;

	/* set input report */
	err = ioctl((*self_).hidraw_fd, HIDIOCSINPUT(3), buf.as_mut_ptr()) as c_int;
	ASSERT_EQ_i32(err, 3);
	TH_LOG(b"HIDIOCSINPUT ioctl failed, got %d\0".as_ptr() as *const c_char, err);

	/*
	 * Note: The uhid mock doesn't validate the set report data,
	 * so we just verify the ioctl succeeds
	 */
}

/*
 * Test HIDIOCGOUTPUT ioctl to get output report
 */
unsafe fn ioctl_goutput(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;

	/* set report ID 1 in first byte */
	buf[0] = 1;

	/* get output report */
	err = ioctl((*self_).hidraw_fd, HIDIOCGOUTPUT(size_of_val(&buf)), buf.as_mut_ptr()) as c_int;
	ASSERT_EQ_i32(err, feature_data_len() as c_int);
	TH_LOG(b"HIDIOCGOUTPUT ioctl failed, got %d\0".as_ptr() as *const c_char, err);

	/* verify we got the expected output data */
	ASSERT_EQ_u8(buf[0], feature_data[0]);
	TH_LOG(
		b"expected feature_data[0] = %d, got %d\0".as_ptr() as *const c_char,
		feature_data[0] as c_int,
		buf[0] as c_int,
	);
	ASSERT_EQ_u8(buf[1], feature_data[1]);
	TH_LOG(
		b"expected feature_data[1] = %d, got %d\0".as_ptr() as *const c_char,
		feature_data[1] as c_int,
		buf[1] as c_int,
	);
}

/*
 * Test HIDIOCGOUTPUT ioctl with invalid report ID
 */
unsafe fn ioctl_goutput_invalid(
	_metadata: *mut c_void,
	self_: *mut hidraw,
	_variant: *mut c_void,
) {
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;

	/* set invalid report ID (not 1) */
	buf[0] = 2;

	/* try to get output report */
	err = ioctl((*self_).hidraw_fd, HIDIOCGOUTPUT(size_of_val(&buf)), buf.as_mut_ptr()) as c_int;
	ASSERT_LT(err, 0);
	TH_LOG(b"HIDIOCGOUTPUT should have failed with invalid report ID\0".as_ptr() as *const c_char);
	ASSERT_EQ_i32(errno, EIO);
	TH_LOG(b"expected EIO, got errno %d\0".as_ptr() as *const c_char, errno);
}

/*
 * Test HIDIOCSOUTPUT ioctl to set output report
 */
unsafe fn ioctl_soutput(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut buf: [__u8; 10] = [0; 10];
	let mut err: c_int;

	/* prepare output report data */
	buf[0] = 1; /* report ID */
	buf[1] = 0x33;
	buf[2] = 0xCC;

	/* set output report */
	err = ioctl((*self_).hidraw_fd, HIDIOCSOUTPUT(3), buf.as_mut_ptr()) as c_int;
	ASSERT_EQ_i32(err, 3);
	TH_LOG(b"HIDIOCSOUTPUT ioctl failed, got %d\0".as_ptr() as *const c_char, err);

	/*
	 * Note: The uhid mock doesn't validate the set report data,
	 * so we just verify the ioctl succeeds
	 */
}

/*
 * Test HIDIOCGRAWNAME ioctl to get device name string
 */
unsafe fn ioctl_rawname(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut name: [c_char; 256] = [0; 256];
	let mut expected_name: [c_char; 64] = [0; 64];
	let mut err: c_int;

	/* get device name */
	err = ioctl((*self_).hidraw_fd, HIDIOCGRAWNAME(size_of_val(&name)), name.as_mut_ptr())
		as c_int;
	ASSERT_GT(err, 0);
	TH_LOG(b"HIDIOCGRAWNAME ioctl failed, got %d\0".as_ptr() as *const c_char, err);

	/* construct expected name based on device id */
	snprintf(
		expected_name.as_mut_ptr(),
		size_of_val(&expected_name),
		b"test-uhid-device-%d\0".as_ptr() as *const c_char,
		(*self_).hid.dev_id,
	);

	/* verify the name matches expected pattern */
	ASSERT_EQ_i32(strcmp(name.as_ptr(), expected_name.as_ptr()), 0);
	TH_LOG(
		b"expected name '%s', got '%s'\0".as_ptr() as *const c_char,
		expected_name.as_ptr(),
		name.as_ptr(),
	);
}

/*
 * Test HIDIOCGRAWPHYS ioctl to get device physical address string
 */
unsafe fn ioctl_rawphys(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut phys: [c_char; 256] = [0; 256];
	let mut expected_phys: [c_char; 64] = [0; 64];
	let mut err: c_int;

	/* get device physical address */
	err = ioctl((*self_).hidraw_fd, HIDIOCGRAWPHYS(size_of_val(&phys)), phys.as_mut_ptr())
		as c_int;
	ASSERT_GT(err, 0);
	TH_LOG(b"HIDIOCGRAWPHYS ioctl failed, got %d\0".as_ptr() as *const c_char, err);

	/* construct expected phys based on device id */
	snprintf(
		expected_phys.as_mut_ptr(),
		size_of_val(&expected_phys),
		b"%d\0".as_ptr() as *const c_char,
		(*self_).hid.dev_id,
	);

	/* verify the phys matches expected value */
	ASSERT_EQ_i32(strcmp(phys.as_ptr(), expected_phys.as_ptr()), 0);
	TH_LOG(
		b"expected phys '%s', got '%s'\0".as_ptr() as *const c_char,
		expected_phys.as_ptr(),
		phys.as_ptr(),
	);
}

/*
 * Test HIDIOCGRAWUNIQ ioctl to get device unique identifier string
 */
unsafe fn ioctl_rawuniq(_metadata: *mut c_void, self_: *mut hidraw, _variant: *mut c_void) {
	let mut uniq: [c_char; 256] = [0; 256];
	let mut err: c_int;

	/* get device unique identifier */
	err = ioctl((*self_).hidraw_fd, HIDIOCGRAWUNIQ(size_of_val(&uniq)), uniq.as_mut_ptr())
		as c_int;
	ASSERT_GE(err, 0);
	TH_LOG(b"HIDIOCGRAWUNIQ ioctl failed, got %d\0".as_ptr() as *const c_char, err);

	/* uniq is typically empty in our test setup */
	ASSERT_EQ_usize(strlen(uniq.as_ptr()), 0);
	TH_LOG(b"expected empty uniq, got '%s'\0".as_ptr() as *const c_char, uniq.as_ptr());
}

/*
 * Test device string ioctls with small buffer sizes
 */
unsafe fn ioctl_strings_small_buffer(
	_metadata: *mut c_void,
	self_: *mut hidraw,
	_variant: *mut c_void,
) {
	let mut small_buf: [c_char; 8] = [0; 8];
	let mut expected_name: [c_char; 64] = [0; 64];
	let mut err: c_int;

	/* test HIDIOCGRAWNAME with small buffer */
	err = ioctl(
		(*self_).hidraw_fd,
		HIDIOCGRAWNAME(size_of_val(&small_buf)),
		small_buf.as_mut_ptr(),
	) as c_int;
	ASSERT_EQ_i32(err, size_of_val(&small_buf) as c_int);
	TH_LOG(b"HIDIOCGRAWNAME with small buffer failed, got %d\0".as_ptr() as *const c_char, err);

	/* construct expected truncated name */
	snprintf(
		expected_name.as_mut_ptr(),
		size_of_val(&expected_name),
		b"test-uhid-device-%d\0".as_ptr() as *const c_char,
		(*self_).hid.dev_id,
	);

	/* verify we got truncated name (first 8 chars, no null terminator guaranteed) */
	ASSERT_EQ_i32(
		strncmp(small_buf.as_ptr(), expected_name.as_ptr(), size_of_val(&small_buf)),
		0,
	);
	TH_LOG(
		b"expected truncated name to match first %zu chars\0".as_ptr() as *const c_char,
		size_of_val(&small_buf),
	);

	/* Note: hidraw driver doesn't guarantee null termination when buffer is too small */
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
	test_harness_run(argc, argv)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
