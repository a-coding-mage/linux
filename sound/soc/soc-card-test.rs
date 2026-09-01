// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2024 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

// C includes translated as external dependencies:
// <kunit/device.h>, <kunit/test.h>, <linux/module.h>,
// <sound/control.h>, <sound/soc.h>, <sound/soc-card.h>

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct module {
	_private: [u8; 0],
}

#[repr(C)]
pub struct kunit {
	pub priv_: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_card {
	pub name: *const c_char,
	pub dev: *mut device,
	pub owner: *mut module,
}

#[repr(C)]
pub struct snd_kcontrol {
	pub private_value: c_ulong,
}

#[repr(C)]
pub struct soc_mixer_control {
	pub shift: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
	pub name: *const c_char,
	pub private_value: c_ulong,
}

#[repr(C)]
pub struct kunit_case {
	pub run_case: Option<unsafe extern "C" fn(*mut kunit)>,
}

#[repr(C)]
pub struct kunit_suite {
	pub name: *const c_char,
	pub test_cases: *mut kunit_case,
	pub init: Option<unsafe extern "C" fn(*mut kunit) -> c_int>,
	pub exit: Option<unsafe extern "C" fn(*mut kunit)>,
}

#[repr(C)]
struct soc_card_test_priv {
	card_dev: *mut device,
	card: *mut snd_soc_card,
}

const GFP_KERNEL: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

extern "C" {
	static mut THIS_MODULE: *mut module;

	fn kunit_kzalloc(test: *mut kunit, size: usize, flags: c_uint) -> *mut c_void;
	fn kunit_device_register(test: *mut kunit, name: *const c_char) -> *mut device;
	fn get_device(dev: *mut device) -> *mut device;
	fn put_device(dev: *mut device);

	fn snd_soc_add_card_controls(
		card: *mut snd_soc_card,
		controls: *const snd_kcontrol_new,
		num_controls: c_int,
	) -> c_int;
	fn snd_soc_card_get_kcontrol(
		card: *mut snd_soc_card,
		name: *const c_char,
	) -> *mut snd_kcontrol;
	fn snd_soc_register_card(card: *mut snd_soc_card) -> c_int;
	fn snd_soc_unregister_card(card: *mut snd_soc_card);

	fn KUNIT_ASSERT_EQ(test: *mut kunit, left: c_int, right: c_int);
	fn KUNIT_EXPECT_NOT_ERR_OR_NULL_MSG(
		test: *mut kunit,
		ptr: *mut snd_kcontrol,
		fmt: *const c_char,
		...
	);
	fn KUNIT_EXPECT_EQ_MSG(
		test: *mut kunit,
		left: c_uint,
		right: c_int,
		fmt: *const c_char,
		...
	);
	fn KUNIT_EXPECT_NULL(test: *mut kunit, ptr: *mut snd_kcontrol);

	fn kunit_test_suites(suite: *mut kunit_suite);
}

const fn soc_single(
	name: *const c_char,
	_reg: c_int,
	shift: c_uint,
	_max: c_uint,
	_invert: c_uint,
) -> snd_kcontrol_new {
	snd_kcontrol_new {
		name,
		private_value: shift as c_ulong,
	}
}

static test_card_controls: [snd_kcontrol_new; 12] = [
	soc_single(c"Fee".as_ptr(), SND_SOC_NOPM, 0, 1, 0),
	soc_single(c"Fi".as_ptr(), SND_SOC_NOPM, 1, 1, 0),
	soc_single(c"Fo".as_ptr(), SND_SOC_NOPM, 2, 1, 0),
	soc_single(c"Fum".as_ptr(), SND_SOC_NOPM, 3, 1, 0),
	soc_single(c"Left Fee".as_ptr(), SND_SOC_NOPM, 4, 1, 0),
	soc_single(c"Right Fee".as_ptr(), SND_SOC_NOPM, 5, 1, 0),
	soc_single(c"Left Fi".as_ptr(), SND_SOC_NOPM, 6, 1, 0),
	soc_single(c"Right Fi".as_ptr(), SND_SOC_NOPM, 7, 1, 0),
	soc_single(c"Left Fo".as_ptr(), SND_SOC_NOPM, 8, 1, 0),
	soc_single(c"Right Fo".as_ptr(), SND_SOC_NOPM, 9, 1, 0),
	soc_single(c"Left Fum".as_ptr(), SND_SOC_NOPM, 10, 1, 0),
	soc_single(c"Right Fum".as_ptr(), SND_SOC_NOPM, 11, 1, 0),
];

unsafe extern "C" fn test_snd_soc_card_get_kcontrol(test: *mut kunit) {
	let priv_ = (*test).priv_ as *mut soc_card_test_priv;
	let card = (*priv_).card;
	let mut kc: *mut snd_kcontrol;
	let mut mc: *mut soc_mixer_control;
	let mut i: c_int;
	let ret: c_int;

	ret = snd_soc_add_card_controls(
		card,
		test_card_controls.as_ptr(),
		test_card_controls.len() as c_int,
	);
	KUNIT_ASSERT_EQ(test, ret, 0);

	/* Look up every control */
	i = 0;
	while i < test_card_controls.len() as c_int {
		kc = snd_soc_card_get_kcontrol(card, test_card_controls[i as usize].name);
		KUNIT_EXPECT_NOT_ERR_OR_NULL_MSG(
			test,
			kc,
			c"Failed to find '%s'\n".as_ptr(),
			test_card_controls[i as usize].name,
		);
		if kc.is_null() {
			i += 1;
			continue;
		}

		/* Test that it is the correct control */
		mc = (*kc).private_value as *mut soc_mixer_control;
		KUNIT_EXPECT_EQ_MSG(
			test,
			(*mc).shift,
			i,
			c"For '%s'\n".as_ptr(),
			test_card_controls[i as usize].name,
		);
		i += 1;
	}

	/* Test some names that should not be found */
	kc = snd_soc_card_get_kcontrol(card, c"None".as_ptr());
	KUNIT_EXPECT_NULL(test, kc);

	kc = snd_soc_card_get_kcontrol(card, c"Left None".as_ptr());
	KUNIT_EXPECT_NULL(test, kc);

	kc = snd_soc_card_get_kcontrol(card, c"Left".as_ptr());
	KUNIT_EXPECT_NULL(test, kc);

	kc = snd_soc_card_get_kcontrol(card, ptr::null());
	KUNIT_EXPECT_NULL(test, kc);
}

unsafe extern "C" fn soc_card_test_case_init(test: *mut kunit) -> c_int {
	let priv_: *mut soc_card_test_priv;
	let ret: c_int;

	priv_ = kunit_kzalloc(test, size_of::<soc_card_test_priv>(), GFP_KERNEL)
		as *mut soc_card_test_priv;
	if priv_.is_null() {
		return -ENOMEM;
	}

	(*test).priv_ = priv_ as *mut c_void;

	(*priv_).card = kunit_kzalloc(test, size_of::<snd_soc_card>(), GFP_KERNEL)
		as *mut snd_soc_card;
	if (*priv_).card.is_null() {
		return -ENOMEM;
	}

	(*priv_).card_dev = kunit_device_register(test, c"sound-soc-card-test".as_ptr());
	(*priv_).card_dev = get_device((*priv_).card_dev);
	if (*priv_).card_dev.is_null() {
		return -ENODEV;
	}

	(*(*priv_).card).name = c"soc-card-test".as_ptr();
	(*(*priv_).card).dev = (*priv_).card_dev;
	(*(*priv_).card).owner = THIS_MODULE;

	ret = snd_soc_register_card((*priv_).card);
	if ret != 0 {
		put_device((*priv_).card_dev);
		return ret;
	}

	0
}

unsafe extern "C" fn soc_card_test_case_exit(test: *mut kunit) {
	let priv_ = (*test).priv_ as *mut soc_card_test_priv;

	if !(*priv_).card.is_null() {
		snd_soc_unregister_card((*priv_).card);
	}

	if !(*priv_).card_dev.is_null() {
		put_device((*priv_).card_dev);
	}
}

static mut soc_card_test_cases: [kunit_case; 2] = [
	kunit_case {
		run_case: Some(test_snd_soc_card_get_kcontrol),
	},
	kunit_case { run_case: None },
];

static mut soc_card_test_suite: kunit_suite = kunit_suite {
	name: c"soc-card".as_ptr(),
	test_cases: unsafe { soc_card_test_cases.as_mut_ptr() },
	init: Some(soc_card_test_case_init),
	exit: Some(soc_card_test_case_exit),
};

#[used]
static mut __KUNIT_TEST_SUITES_INIT: unsafe extern "C" fn() = {
	unsafe extern "C" fn init() {
		kunit_test_suites(&raw mut soc_card_test_suite);
	}
	init
};

// MODULE_DESCRIPTION("ASoC soc-card KUnit test");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
