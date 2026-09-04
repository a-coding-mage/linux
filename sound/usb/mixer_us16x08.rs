// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Tascam US-16x08 ALSA driver
 *
 *   Copyright (c) 2016 by Detlef Urban (onkel@paraair.de)
 */

// Requires: linux/slab.h, linux/usb.h, linux/usb/audio-v2.h
// Requires: sound/core.h, sound/control.h
// Requires: usbaudio.h, mixer.h, helper.h, mixer_us16x08.h

/* USB control message templates */
static const ROUTE_MSG: &[u8] = &[
	0x61,
	0x02,
	0x03, /* input from master (0x02) or input from computer bus (0x03) */
	0x62,
	0x02,
	0x01, /* input index (0x01/0x02 eq. left/right) or bus (0x01-0x08) */
	0x41,
	0x01,
	0x61,
	0x02,
	0x01,
	0x62,
	0x02,
	0x01, /* output index (0x01-0x08) */
	0x42,
	0x01,
	0x43,
	0x01,
	0x00,
	0x00
];

static const MIX_INIT_MSG1: &[u8] = &[
	0x71, 0x01, 0x00, 0x00
];

static const MIX_INIT_MSG2: &[u8] = &[
	0x62, 0x02, 0x00, 0x61, 0x02, 0x04, 0xb1, 0x01, 0x00, 0x00
];

static const MIX_MSG_IN: &[u8] = &[
	/* default message head, equal to all mixers */
	0x61, 0x02, 0x04, 0x62, 0x02, 0x01,
	0x81, /* 0x06: Controller ID */
	0x02, /* 0x07:  */
	0x00, /* 0x08: Value of common mixer */
	0x00,
	0x00
];

static const MIX_MSG_OUT: &[u8] = &[
	/* default message head, equal to all mixers */
	0x61, 0x02, 0x02, 0x62, 0x02, 0x01,
	0x81, /* 0x06: Controller ID */
	0x02, /*                    0x07:  */
	0x00, /*                    0x08: Value of common mixer */
	0x00,
	0x00
];

static const BYPASS_MSG_OUT: &[u8] = &[
	0x45,
	0x02,
	0x01, /* on/off flag */
	0x00,
	0x00
];

static const BUS_MSG_OUT: &[u8] = &[
	0x44,
	0x02,
	0x01, /* on/off flag */
	0x00,
	0x00
];

static const COMP_MSG: &[u8] = &[
	/* default message head, equal to all mixers */
	0x61, 0x02, 0x04, 0x62, 0x02, 0x01,
	0x91,
	0x02,
	0xf0, /* 0x08: Threshold db (8) (e0 ... 00) (+-0dB -- -32dB) x-32 */
	0x92,
	0x02,
	0x0a, /* 0x0b: Ratio (0a,0b,0d,0f,11,14,19,1e,23,28,32,3c,50,a0,ff)  */
	0x93,
	0x02,
	0x02, /* 0x0e: Attack (0x02 ... 0xc0) (2ms ... 200ms) */
	0x94,
	0x02,
	0x01, /* 0x11: Release (0x01 ... 0x64) (10ms ... 1000ms) x*10  */
	0x95,
	0x02,
	0x03, /* 0x14: gain (0 ... 20) (0dB .. 20dB) */
	0x96,
	0x02,
	0x01,
	0x97,
	0x02,
	0x01, /* 0x1a: main Comp switch (0 ... 1) (off ... on)) */
	0x00,
	0x00
];

static const EQS_MSQ: &[u8] = &[
	/* default message head, equal to all mixers */
	0x61, 0x02, 0x04, 0x62, 0x02, 0x01,
	0x51, /*                0x06: Controller ID  */
	0x02,
	0x04, /* 0x08: EQ set num (0x01..0x04) (LOW, LOWMID, HIGHMID, HIGH)) */
	0x52,
	0x02,
	0x0c, /* 0x0b: value dB (0 ... 12) (-12db .. +12db)  x-6 */
	0x53,
	0x02,
	0x0f, /* 0x0e: value freq (32-47) (1.7kHz..18kHz) */
	0x54,
	0x02,
	0x02, /* 0x11: band width (0-6) (Q16-Q0.25)  2^x/4 (EQ xxMID only) */
	0x55,
	0x02,
	0x01, /* 0x14: main EQ switch (0 ... 1) (off ... on)) */
	0x00,
	0x00
];

/* compressor ratio map */
static const RATIO_MAP: &[u8] = &[
	0x0a, 0x0b, 0x0d, 0x0f, 0x11, 0x14, 0x19, 0x1e,
	0x23, 0x28, 0x32, 0x3c, 0x50, 0xa0, 0xff
];

/* route enumeration names */
static const ROUTE_NAMES: &[&str] = &[
	"Master Left", "Master Right", "Output 1", "Output 2", "Output 3",
	"Output 4", "Output 5", "Output 6", "Output 7", "Output 8",
];

// External kernel/ALSA functions and types (from included headers)
extern "C" {
	type snd_usb_audio;
	type snd_kcontrol;
	type snd_ctl_elem_info;
	type snd_ctl_elem_value;
	type usb_mixer_elem_info;
	type snd_kcontrol_new;
	type usb_mixer_interface;
	type snd_us16x08_comp_store;
	type snd_us16x08_eq_store;
	type snd_us16x08_meter_store;
	type snd_us16x08_control_params;

	fn snd_usb_ctl_msg(dev: *mut core::ffi::c_void, pipe: u32, request: u32, requesttype: u32,
		value: u32, index: u32, buf: *mut core::ffi::c_void, size: i32) -> i32;
	fn usb_rcvctrlpipe(dev: *mut core::ffi::c_void, endpoint: u32) -> u32;
	fn usb_sndctrlpipe(dev: *mut core::ffi::c_void, endpoint: u32) -> u32;
	fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: u32, items: u32,
		names: *const *const core::ffi::c_char) -> i32;
	fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut usb_mixer_elem_info;
	fn usb_audio_dbg(chip: *mut snd_usb_audio, fmt: *const core::ffi::c_char, ...);
	fn snd_ctl_new1(kcontrol: *const snd_kcontrol_new, private_data: *mut core::ffi::c_void) -> *mut snd_kcontrol;
	fn snd_usb_mixer_add_control(head: *mut core::ffi::c_void, kctl: *mut snd_kcontrol) -> i32;
	fn snd_usb_mixer_elem_free(kctl: *mut snd_kcontrol);
	fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
	fn kmalloc_obj(obj: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
	fn kzalloc_obj(obj: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
	fn kfree(obj: *mut core::ffi::c_void);
	fn msleep(msecs: u32);
	fn strscpy(dest: *mut core::ffi::c_char, src: *const core::ffi::c_char, size: usize) -> isize;
}

static unsafe fn snd_us16x08_recv_urb(chip: *mut snd_usb_audio,
	buf: *mut u8, size: i32) -> i32
{
	// guard(mutex)(&chip->mutex);
	snd_usb_ctl_msg(chip as *mut core::ffi::c_void,
		usb_rcvctrlpipe(chip as *mut core::ffi::c_void, 0),
		0, /* SND_US16X08_URB_METER_REQUEST */
		0, /* SND_US16X08_URB_METER_REQUESTTYPE */
		0, 0, buf as *mut core::ffi::c_void, size);
	return 0;
}

/* wrapper function to send prepared URB buffer to usb device. Return an error
 * code if something went wrong
 */
static unsafe fn snd_us16x08_send_urb(chip: *mut snd_usb_audio, buf: *mut i8, size: i32) -> i32
{
	return snd_usb_ctl_msg(chip as *mut core::ffi::c_void, usb_sndctrlpipe(chip as *mut core::ffi::c_void, 0),
			0, /* SND_US16X08_URB_REQUEST */
			0, /* SND_US16X08_URB_REQUESTTYPE */
			0, 0, buf as *mut core::ffi::c_void, size);
}

static unsafe fn snd_us16x08_route_info(kcontrol: *mut snd_kcontrol,
	uinfo: *mut snd_ctl_elem_info) -> i32
{
	return snd_ctl_enum_info(uinfo, 1, 10, ROUTE_NAMES.as_ptr() as *const *const core::ffi::c_char);
}

static unsafe fn snd_us16x08_route_get(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);
	let index = (*ucontrol).id.index;

	/* route has no bias */
	// ucontrol->value.enumerated.item[0] = elem->cache_val[index];

	return 0;
}

static unsafe fn snd_us16x08_route_put(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);
	let chip = (*(*elem).head.mixer).chip;
	let index = (*ucontrol).id.index;
	let mut buf: [u8; 20] = [0; 20];
	let mut val: i32;
	let mut val_org: i32;
	let mut err: i32;

	/*  get the new value (no bias for routes) */
	val = 0; /* ucontrol->value.enumerated.item[0] */

	/* sanity check */
	if val < 0 || val > 9 {
		return -22; /* -EINVAL */
	}

	/* prepare the message buffer from template */
	memcpy(&mut buf as *mut u8 as *mut core::ffi::c_void,
		ROUTE_MSG.as_ptr() as *const core::ffi::c_void, ROUTE_MSG.len());

	if val < 2 {
		/* input comes from a master channel */
		val_org = val;
		buf[2] = 0x02;
	} else {
		/* input comes from a computer channel */
		buf[2] = 0x03;
		val_org = val - 2;
	}

	/* place new route selection in URB message */
	buf[5] = ((val_org & 0x0f) + 1) as u8;
	/* place route selector in URB message */
	buf[13] = (index + 1) as u8;

	err = snd_us16x08_send_urb(chip, &mut buf as *mut u8 as *mut i8, ROUTE_MSG.len() as i32);

	if err < 0 {
		usb_audio_dbg(chip, "Failed to set routing, err:%d\n" as *const i8, err);
		return err;
	}

	// elem->cached |= 1 << index;
	// elem->cache_val[index] = val;
	return 1;
}

static unsafe fn snd_us16x08_master_info(kcontrol: *mut snd_kcontrol,
	uinfo: *mut snd_ctl_elem_info) -> i32
{
	// uinfo->count = 1;
	// uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
	// uinfo->value.integer.max = SND_US16X08_KCMAX(kcontrol);
	// uinfo->value.integer.min = SND_US16X08_KCMIN(kcontrol);
	// uinfo->value.integer.step = SND_US16X08_KCSTEP(kcontrol);
	return 0;
}

static unsafe fn snd_us16x08_master_get(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);
	let index = (*ucontrol).id.index;

	// ucontrol->value.integer.value[0] = elem->cache_val[index];

	return 0;
}

static unsafe fn snd_us16x08_master_put(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);
	let chip = (*(*elem).head.mixer).chip;
	let mut buf: [u8; 11] = [0; 11];
	let mut val: i32;
	let mut err: i32;
	let index = (*ucontrol).id.index;

	/* new control value incl. bias*/
	val = 0; /* ucontrol->value.integer.value[0] */

	/* sanity check */
	// if (val < SND_US16X08_KCMIN(kcontrol)
	// 	|| val > SND_US16X08_KCMAX(kcontrol))
	// 	return -EINVAL;

	/* prepare the message buffer from template */
	memcpy(&mut buf as *mut u8 as *mut core::ffi::c_void,
		MIX_MSG_OUT.as_ptr() as *const core::ffi::c_void, MIX_MSG_OUT.len());

	// buf[8] = val - SND_US16X08_KCBIAS(kcontrol);
	// buf[6] = elem->head.id;

	/* place channel selector in URB message */
	buf[5] = (index + 1) as u8;
	err = snd_us16x08_send_urb(chip, &mut buf as *mut u8 as *mut i8, MIX_MSG_OUT.len() as i32);

	if err < 0 {
		usb_audio_dbg(chip, "Failed to set master, err:%d\n" as *const i8, err);
		return err;
	}

	// elem->cached |= 1 << index;
	// elem->cache_val[index] = val;
	return 1;
}

static unsafe fn snd_us16x08_bus_put(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);
	let chip = (*(*elem).head.mixer).chip;
	let mut buf: [u8; 11] = [0; 11];
	let mut val: i32;
	let mut err: i32 = 0;

	val = 0; /* ucontrol->value.integer.value[0] */

	/* prepare the message buffer from template */
	match (*elem).head.id {
	0 => { /* SND_US16X08_ID_BYPASS */
		memcpy(&mut buf as *mut u8 as *mut core::ffi::c_void,
			BYPASS_MSG_OUT.as_ptr() as *const core::ffi::c_void, BYPASS_MSG_OUT.len());
		buf[2] = val as u8;
		err = snd_us16x08_send_urb(chip, &mut buf as *mut u8 as *mut i8, BYPASS_MSG_OUT.len() as i32);
	},
	1 => { /* SND_US16X08_ID_BUSS_OUT */
		memcpy(&mut buf as *mut u8 as *mut core::ffi::c_void,
			BUS_MSG_OUT.as_ptr() as *const core::ffi::c_void, BUS_MSG_OUT.len());
		buf[2] = val as u8;
		err = snd_us16x08_send_urb(chip, &mut buf as *mut u8 as *mut i8, BUS_MSG_OUT.len() as i32);
	},
	2 => { /* SND_US16X08_ID_MUTE */
		memcpy(&mut buf as *mut u8 as *mut core::ffi::c_void,
			MIX_MSG_OUT.as_ptr() as *const core::ffi::c_void, MIX_MSG_OUT.len());
		buf[8] = val as u8;
		buf[6] = (*elem).head.id as u8;
		buf[5] = 1;
		err = snd_us16x08_send_urb(chip, &mut buf as *mut u8 as *mut i8, MIX_MSG_OUT.len() as i32);
	},
	_ => {}
	}

	if err < 0 {
		usb_audio_dbg(chip, "Failed to set bus parameter, err:%d\n" as *const i8, err);
		return err;
	}

	// elem->cached |= 1;
	// elem->cache_val[0] = val;
	return 1;
}

static unsafe fn snd_us16x08_bus_get(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);

	match (*elem).head.id {
	1 => { /* SND_US16X08_ID_BUSS_OUT */
		// ucontrol->value.integer.value[0] = elem->cache_val[0];
	},
	0 => { /* SND_US16X08_ID_BYPASS */
		// ucontrol->value.integer.value[0] = elem->cache_val[0];
	},
	2 => { /* SND_US16X08_ID_MUTE */
		// ucontrol->value.integer.value[0] = elem->cache_val[0];
	},
	_ => {}
	}

	return 0;
}

/* gets a current mixer value from common store */
static unsafe fn snd_us16x08_channel_get(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);
	let index = (*ucontrol).id.index;

	// ucontrol->value.integer.value[0] = elem->cache_val[index];

	return 0;
}

static unsafe fn snd_us16x08_channel_put(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);
	let chip = (*(*elem).head.mixer).chip;
	let mut buf: [u8; 11] = [0; 11];
	let mut val: i32;
	let mut err: i32;
	let index = (*ucontrol).id.index;

	val = 0; /* ucontrol->value.integer.value[0] */

	/* sanity check */
	// if (val < SND_US16X08_KCMIN(kcontrol)
	// 	|| val > SND_US16X08_KCMAX(kcontrol))
	// 	return -EINVAL;

	/* prepare URB message from template */
	memcpy(&mut buf as *mut u8 as *mut core::ffi::c_void,
		MIX_MSG_IN.as_ptr() as *const core::ffi::c_void, MIX_MSG_IN.len());

	/* add the bias to the new value */
	// buf[8] = val - SND_US16X08_KCBIAS(kcontrol);
	// buf[6] = elem->head.id;
	buf[5] = (index + 1) as u8;

	err = snd_us16x08_send_urb(chip, &mut buf as *mut u8 as *mut i8, MIX_MSG_IN.len() as i32);

	if err < 0 {
		usb_audio_dbg(chip, "Failed to set channel, err:%d\n" as *const i8, err);
		return err;
	}

	// elem->cached |= 1 << index;
	// elem->cache_val[index] = val;
	return 1;
}

static unsafe fn snd_us16x08_mix_info(kcontrol: *mut snd_kcontrol,
	uinfo: *mut snd_ctl_elem_info) -> i32
{
	// uinfo->count = 1;
	// uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
	// uinfo->value.integer.max = SND_US16X08_KCMAX(kcontrol);
	// uinfo->value.integer.min = SND_US16X08_KCMIN(kcontrol);
	// uinfo->value.integer.step = SND_US16X08_KCSTEP(kcontrol);
	return 0;
}

static unsafe fn snd_us16x08_comp_get(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);
	let store = (*elem).private_data as *mut snd_us16x08_comp_store;
	let index = (*ucontrol).id.index;
	// let val_idx = COMP_STORE_IDX(elem->head.id);

	// ucontrol->value.integer.value[0] = store->val[val_idx][index];

	return 0;
}

static unsafe fn snd_us16x08_comp_put(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);
	let chip = (*(*elem).head.mixer).chip;
	let store = (*elem).private_data as *mut snd_us16x08_comp_store;
	let index = (*ucontrol).id.index;
	let mut buf: [u8; 28] = [0; 28];
	let mut val_idx: i32;
	let mut val: i32;
	let mut threshold: i32;
	let mut ratio: i32;
	let mut attack: i32;
	let mut release: i32;
	let mut gain: i32;
	let mut switch_on: i32;
	let mut err: i32;

	val = 0; /* ucontrol->value.integer.value[0] */

	/* sanity check */
	// if (val < SND_US16X08_KCMIN(kcontrol)
	// 	|| val > SND_US16X08_KCMAX(kcontrol))
	// 	return -EINVAL;

	/* new control value incl. bias*/
	// val_idx = elem->head.id - SND_US16X08_ID_COMP_BASE;

	// threshold = store->val[COMP_STORE_IDX(SND_US16X08_ID_COMP_THRESHOLD)]
	// 	[index];
	// ratio = store->val[COMP_STORE_IDX(SND_US16X08_ID_COMP_RATIO)][index];
	// attack = store->val[COMP_STORE_IDX(SND_US16X08_ID_COMP_ATTACK)][index];
	// release = store->val[COMP_STORE_IDX(SND_US16X08_ID_COMP_RELEASE)]
	// 	[index];
	// gain = store->val[COMP_STORE_IDX(SND_US16X08_ID_COMP_GAIN)][index];
	// switch_on = store->val[COMP_STORE_IDX(SND_US16X08_ID_COMP_SWITCH)]
	// 	[index];

	// switch (val_idx) {
	// case COMP_STORE_IDX(SND_US16X08_ID_COMP_THRESHOLD):
	// 	threshold = val;
	// 	break;
	// case COMP_STORE_IDX(SND_US16X08_ID_COMP_RATIO):
	// 	ratio = val;
	// 	break;
	// case COMP_STORE_IDX(SND_US16X08_ID_COMP_ATTACK):
	// 	attack = val;
	// 	break;
	// case COMP_STORE_IDX(SND_US16X08_ID_COMP_RELEASE):
	// 	release = val;
	// 	break;
	// case COMP_STORE_IDX(SND_US16X08_ID_COMP_GAIN):
	// 	gain = val;
	// 	break;
	// case COMP_STORE_IDX(SND_US16X08_ID_COMP_SWITCH):
	// 	switch_on = val;
	// 	break;
	// }

	/* prepare compressor URB message from template  */
	memcpy(&mut buf as *mut u8 as *mut core::ffi::c_void,
		COMP_MSG.as_ptr() as *const core::ffi::c_void, COMP_MSG.len());

	/* place comp values in message buffer watch bias! */
	// buf[8] = threshold - SND_US16X08_COMP_THRESHOLD_BIAS;
	// buf[11] = ratio_map[ratio];
	// buf[14] = attack + SND_US16X08_COMP_ATTACK_BIAS;
	// buf[17] = release + SND_US16X08_COMP_RELEASE_BIAS;
	// buf[20] = gain;
	// buf[26] = switch_on;

	/* place channel selector in message buffer */
	buf[5] = (index + 1) as u8;

	err = snd_us16x08_send_urb(chip, &mut buf as *mut u8 as *mut i8, COMP_MSG.len() as i32);

	if err < 0 {
		usb_audio_dbg(chip, "Failed to set compressor, err:%d\n" as *const i8, err);
		return err;
	}

	// store->val[val_idx][index] = val;
	// elem->cached |= 1 << index;
	// elem->cache_val[index] = val;
	return 1;
}

static unsafe fn snd_us16x08_eqswitch_get(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let mut val: i32;
	let elem = snd_kcontrol_chip(kcontrol);
	let store = (*elem).private_data as *mut snd_us16x08_eq_store;
	let index = (*ucontrol).id.index;

	/* get low switch from cache is enough, cause all bands are together */
	// val = store->val[EQ_STORE_BAND_IDX(elem->head.id)]
	// 	[EQ_STORE_PARAM_IDX(elem->head.id)][index];
	// ucontrol->value.integer.value[0] = val;

	return 0;
}

static unsafe fn snd_us16x08_eqswitch_put(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);
	let chip = (*(*elem).head.mixer).chip;
	let store = (*elem).private_data as *mut snd_us16x08_eq_store;
	let index = (*ucontrol).id.index;
	let mut buf: [u8; 22] = [0; 22];
	let mut val: i32;
	let mut err: i32 = 0;
	let mut b_idx: i32;

	/* new control value incl. bias*/
	val = 0; /* ucontrol->value.integer.value[0] + SND_US16X08_KCBIAS(kcontrol) */

	/* prepare URB message from EQ template */
	memcpy(&mut buf as *mut u8 as *mut core::ffi::c_void,
		EQS_MSQ.as_ptr() as *const core::ffi::c_void, EQS_MSQ.len());

	/* place channel index in URB message */
	buf[5] = (index + 1) as u8;
	b_idx = 0;
	while b_idx < 4 { /* SND_US16X08_ID_EQ_BAND_COUNT */
		/* all four EQ bands have to be enabled/disabled in once */
		buf[20] = val as u8;
		// buf[17] = store->val[b_idx][2][index];
		// buf[14] = store->val[b_idx][1][index];
		// buf[11] = store->val[b_idx][0][index];
		buf[8] = (b_idx + 1) as u8;
		err = snd_us16x08_send_urb(chip, &mut buf as *mut u8 as *mut i8, EQS_MSQ.len() as i32);
		if err < 0 {
			break;
		}
		// store->val[b_idx][3][index] = val;
		msleep(15);
		b_idx += 1;
	}

	if err < 0 {
		usb_audio_dbg(chip, "Failed to set eq switch, err:%d\n" as *const i8, err);
		return err;
	}

	// elem->cached |= 1 << index;
	// elem->cache_val[index] = val;
	return 1;
}

static unsafe fn snd_us16x08_eq_get(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let mut val: i32;
	let elem = snd_kcontrol_chip(kcontrol);
	let store = (*elem).private_data as *mut snd_us16x08_eq_store;
	let index = (*ucontrol).id.index;
	// let b_idx = EQ_STORE_BAND_IDX(elem->head.id) - 1;
	// let p_idx = EQ_STORE_PARAM_IDX(elem->head.id);

	// val = store->val[b_idx][p_idx][index];

	// ucontrol->value.integer.value[0] = val;

	return 0;
}

static unsafe fn snd_us16x08_eq_put(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);
	let chip = (*(*elem).head.mixer).chip;
	let store = (*elem).private_data as *mut snd_us16x08_eq_store;
	let index = (*ucontrol).id.index;
	let mut buf: [u8; 22] = [0; 22];
	let mut val: i32;
	let mut err: i32;
	// let b_idx = EQ_STORE_BAND_IDX(elem->head.id) - 1;
	// let p_idx = EQ_STORE_PARAM_IDX(elem->head.id);

	val = 0; /* ucontrol->value.integer.value[0] */

	/* sanity check */
	// if (val < SND_US16X08_KCMIN(kcontrol)
	// 	|| val > SND_US16X08_KCMAX(kcontrol))
	// 	return -EINVAL;

	/* copy URB buffer from EQ template */
	memcpy(&mut buf as *mut u8 as *mut core::ffi::c_void,
		EQS_MSQ.as_ptr() as *const core::ffi::c_void, EQS_MSQ.len());

	// buf[20] = p_idx == 3 ? val : store->val[b_idx][3][index];
	// buf[17] = p_idx == 2 ? val : store->val[b_idx][2][index];
	// buf[14] = p_idx == 1 ? val : store->val[b_idx][1][index];
	// buf[11] = p_idx == 0 ? val : store->val[b_idx][0][index];

	/* place channel index in URB buffer */
	buf[5] = (index + 1) as u8;

	/* place EQ band in URB buffer */
	// buf[8] = b_idx + 1;

	err = snd_us16x08_send_urb(chip, &mut buf as *mut u8 as *mut i8, EQS_MSQ.len() as i32);

	if err < 0 {
		usb_audio_dbg(chip, "Failed to set eq param, err:%d\n" as *const i8, err);
		return err;
	}

	// store->val[b_idx][p_idx][index] = val;
	/* store new value in EQ band cache */
	// elem->cached |= 1 << index;
	// elem->cache_val[index] = val;
	return 1;
}

static unsafe fn snd_us16x08_meter_info(kcontrol: *mut snd_kcontrol,
	uinfo: *mut snd_ctl_elem_info) -> i32
{
	// uinfo->count = 34;
	// uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
	// uinfo->value.integer.max = 0x7FFF;
	// uinfo->value.integer.min = 0;

	return 0;
}

/* calculate compressor index for reduction level request */
static unsafe fn snd_get_meter_comp_index(store: *mut snd_us16x08_meter_store) -> i32
{
	let mut ret: i32;

	/* any channel active */
	if (*store).comp_active_index != 0 {
		/* check for stereo link */
		if (*store).comp_active_index & 0x20 != 0 {
			/* reset comp_index to left channel*/
			if (*store).comp_index - (*store).comp_active_index > 1 {
				(*store).comp_index = (*store).comp_active_index;
			}

			ret = (*store).comp_index & 0x1F;
			(*store).comp_index += 1;
		} else {
			/* no stereo link */
			ret = (*store).comp_active_index;
		}
	} else {
		/* skip channels with no compressor active */
		while (*store).comp_index <= 16 { /* SND_US16X08_MAX_CHANNELS */
			/* check if compressor active - access through stored pointer */
			(*store).comp_index += 1;
		}
		ret = (*store).comp_index;
		(*store).comp_index += 1;
		if (*store).comp_index > 16 { /* SND_US16X08_MAX_CHANNELS */
			(*store).comp_index = 1;
		}
	}
	return ret;
}

/* retrieve the meter level values from URB message */
static unsafe fn get_meter_levels_from_urb(s: i32,
	store: *mut snd_us16x08_meter_store,
	meter_urb: *const u8)
{
	let mut val: i32;
	let mut ch: i32;

	// val = MUC2(meter_urb, s) + (MUC3(meter_urb, s) << 8);
	// ch = MUB2(meter_urb, s) - 1;

	if ch < 0 {
		return;
	}

	// if (MUA0(meter_urb, s) == 0x61 && MUA1(meter_urb, s) == 0x02 &&
	// 	MUA2(meter_urb, s) == 0x04 && MUB0(meter_urb, s) == 0x62) {
	// 	if (ch < SND_US16X08_MAX_CHANNELS) {
	// 		if (MUC0(meter_urb, s) == 0x72)
	// 			store->meter_level[ch] = val;
	// 		if (MUC0(meter_urb, s) == 0xb2)
	// 			store->comp_level[ch] = val;
	// 	}
	// }
	// if (MUA0(meter_urb, s) == 0x61 && MUA1(meter_urb, s) == 0x02 &&
	// 	MUA2(meter_urb, s) == 0x02 && MUB0(meter_urb, s) == 0x62) {
	// 	if (ch < ARRAY_SIZE(store->master_level))
	// 		store->master_level[ch] = val;
	// }
}

/* Function to retrieve current meter values from the device.
 *
 * The device needs to be polled for meter values with an initial
 * requests. It will return with a sequence of different meter value
 * packages. The first request (case 0:) initiate this meter response sequence.
 * After the third response, an additional request can be placed,
 * to retrieve compressor reduction level value for given channel. This round
 * trip channel selector will skip all inactive compressors.
 * A mixer can interrupt this round-trip by selecting one ore two (stereo-link)
 * specific channels.
 */
static unsafe fn snd_us16x08_meter_get(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let mut i: i32;
	let mut set: i32;
	let elem = snd_kcontrol_chip(kcontrol);
	let chip = (*(*elem).head.mixer).chip;
	let store = (*elem).private_data as *mut snd_us16x08_meter_store;
	let mut meter_urb: [u8; 64] = [0; 64];

	match (*kcontrol).private_value {
	0 => {
		let mut tmp: [u8; 4] = [0; 4];

		memcpy(&mut tmp as *mut u8 as *mut core::ffi::c_void,
			MIX_INIT_MSG1.as_ptr() as *const core::ffi::c_void, MIX_INIT_MSG1.len());
		snd_us16x08_send_urb(chip, &mut tmp as *mut u8 as *mut i8, 4);
		snd_us16x08_recv_urb(chip, &mut meter_urb as *mut u8,
			64);
		(*kcontrol).private_value += 1;
	},
	1 => {
		snd_us16x08_recv_urb(chip, &mut meter_urb as *mut u8,
			64);
		(*kcontrol).private_value += 1;
	},
	2 => {
		snd_us16x08_recv_urb(chip, &mut meter_urb as *mut u8,
			64);
		(*kcontrol).private_value += 1;
	},
	3 => {
		let mut tmp: [u8; 10] = [0; 10];

		memcpy(&mut tmp as *mut u8 as *mut core::ffi::c_void,
			MIX_INIT_MSG2.as_ptr() as *const core::ffi::c_void, MIX_INIT_MSG2.len());
		tmp[2] = snd_get_meter_comp_index(store) as u8;
		snd_us16x08_send_urb(chip, &mut tmp as *mut u8 as *mut i8, 10);
		snd_us16x08_recv_urb(chip, &mut meter_urb as *mut u8,
			64);
		(*kcontrol).private_value = 0;
	},
	_ => {}
	}

	set = 0;
	while set < 6 {
		get_meter_levels_from_urb(set, store, &meter_urb as *const u8);
		set += 1;
	}

	i = 0;
	while i < 16 { /* SND_US16X08_MAX_CHANNELS */
		// ucontrol->value.integer.value[i] =
		// 	store ? store->meter_level[i] : 0;
		i += 1;
	}

	// ucontrol->value.integer.value[i++] = store ? store->master_level[0] : 0;
	// ucontrol->value.integer.value[i++] = store ? store->master_level[1] : 0;

	i = 2;
	while i < 16 + 2 { /* SND_US16X08_MAX_CHANNELS + 2 */
		// ucontrol->value.integer.value[i + SND_US16X08_MAX_CHANNELS] =
		// store ? store->comp_level[i - 2] : 0;
		i += 1;
	}

	return 1;
}

static unsafe fn snd_us16x08_meter_put(kcontrol: *mut snd_kcontrol,
	ucontrol: *mut snd_ctl_elem_value) -> i32
{
	let elem = snd_kcontrol_chip(kcontrol);
	let store = (*elem).private_data as *mut snd_us16x08_meter_store;
	let mut val: i32;

	val = 0; /* ucontrol->value.integer.value[0] */

	/* sanity check */
	if val < 0 || val >= 16 { /* SND_US16X08_MAX_CHANNELS */
		return -22; /* -EINVAL */
	}

	(*store).comp_active_index = val as u32;
	(*store).comp_index = val as u32;

	return 1;
}

/* snd_kcontrol_new structures - using opaque type for now */
/* snd_us16x08_ch_boolean_ctl */
/* snd_us16x08_ch_int_ctl */
/* snd_us16x08_pan_int_ctl */
/* snd_us16x08_master_ctl */
/* snd_us16x08_route_ctl */
/* snd_us16x08_bus_ctl */
/* snd_us16x08_compswitch_ctl */
/* snd_us16x08_comp_threshold_ctl */
/* snd_us16x08_comp_ratio_ctl */
/* snd_us16x08_comp_gain_ctl */
/* snd_us16x08_comp_attack_ctl */
/* snd_us16x08_comp_release_ctl */
/* snd_us16x08_eq_gain_ctl */
/* snd_us16x08_eq_low_freq_ctl */
/* snd_us16x08_eq_mid_freq_ctl */
/* snd_us16x08_eq_mid_width_ctl */
/* snd_us16x08_eq_high_freq_ctl */
/* snd_us16x08_eq_switch_ctl */
/* snd_us16x08_meter_ctl */

/* control store preparation */

/* setup compressor store and assign default value */
static unsafe fn snd_us16x08_create_comp_store() -> *mut snd_us16x08_comp_store
{
	let mut i: i32;
	let tmp: *mut snd_us16x08_comp_store;

	tmp = kmalloc_obj(0 as *mut core::ffi::c_void) as *mut snd_us16x08_comp_store;
	if tmp.is_null() {
		return core::ptr::null_mut();
	}

	i = 0;
	while i < 16 { /* SND_US16X08_MAX_CHANNELS */
		// tmp->val[COMP_STORE_IDX(SND_US16X08_ID_COMP_THRESHOLD)][i]
		// 	= 0x20;
		// tmp->val[COMP_STORE_IDX(SND_US16X08_ID_COMP_RATIO)][i] = 0x00;
		// tmp->val[COMP_STORE_IDX(SND_US16X08_ID_COMP_GAIN)][i] = 0x00;
		// tmp->val[COMP_STORE_IDX(SND_US16X08_ID_COMP_SWITCH)][i] = 0x00;
		// tmp->val[COMP_STORE_IDX(SND_US16X08_ID_COMP_ATTACK)][i] = 0x00;
		// tmp->val[COMP_STORE_IDX(SND_US16X08_ID_COMP_RELEASE)][i] = 0x00;
		i += 1;
	}
	return tmp;
}

/* setup EQ store and assign default values */
static unsafe fn snd_us16x08_create_eq_store() -> *mut snd_us16x08_eq_store
{
	let mut i: i32;
	let mut b_idx: i32;
	let tmp: *mut snd_us16x08_eq_store;

	tmp = kmalloc_obj(0 as *mut core::ffi::c_void) as *mut snd_us16x08_eq_store;
	if tmp.is_null() {
		return core::ptr::null_mut();
	}

	i = 0;
	while i < 16 { /* SND_US16X08_MAX_CHANNELS */
		b_idx = 0;
		while b_idx < 4 { /* SND_US16X08_ID_EQ_BAND_COUNT */
			// tmp->val[b_idx][0][i] = 0x0c;
			// tmp->val[b_idx][3][i] = 0x00;
			match b_idx {
			0 => { /* EQ Low */
				// tmp->val[b_idx][1][i] = 0x05;
				// tmp->val[b_idx][2][i] = 0xff;
			},
			1 => { /* EQ Mid low */
				// tmp->val[b_idx][1][i] = 0x0e;
				// tmp->val[b_idx][2][i] = 0x02;
			},
			2 => { /* EQ Mid High */
				// tmp->val[b_idx][1][i] = 0x1b;
				// tmp->val[b_idx][2][i] = 0x02;
			},
			3 => { /* EQ High */
				// tmp->val[b_idx][1][i] = 0x2f - SND_US16X08_EQ_HIGHFREQ_BIAS;
				// tmp->val[b_idx][2][i] = 0xff;
			},
			_ => {}
			}
			b_idx += 1;
		}
		i += 1;
	}
	return tmp;
}

static unsafe fn snd_us16x08_create_meter_store() -> *mut snd_us16x08_meter_store
{
	let tmp: *mut snd_us16x08_meter_store;

	tmp = kzalloc_obj(0 as *mut core::ffi::c_void) as *mut snd_us16x08_meter_store;
	if tmp.is_null() {
		return core::ptr::null_mut();
	}
	(*tmp).comp_index = 1;
	(*tmp).comp_active_index = 0;
	return tmp;
}

/* release elem->private_free as well; called only once for each *_store */
static unsafe fn elem_private_free(kctl: *mut snd_kcontrol)
{
	let elem = (*kctl).private_data as *mut usb_mixer_elem_info;

	if !elem.is_null() {
		kfree((*elem).private_data);
	}
	kfree(elem as *mut core::ffi::c_void);
	(*kctl).private_data = core::ptr::null_mut();
}

static unsafe fn add_new_ctl(mixer: *mut usb_mixer_interface,
	ncontrol: *const snd_kcontrol_new,
	index: i32, val_type: i32, channels: i32,
	name: *const core::ffi::c_char, opt: *mut core::ffi::c_void,
	do_private_free: bool,
	elem_ret: *mut *mut usb_mixer_elem_info) -> i32
{
	let mut kctl: *mut snd_kcontrol;
	let mut elem: *mut usb_mixer_elem_info;
	let mut err: i32;

	usb_audio_dbg((*mixer).chip, "us16x08 add mixer %s\n" as *const i8, name);

	elem = kzalloc_obj(0 as *mut core::ffi::c_void) as *mut usb_mixer_elem_info;
	if elem.is_null() {
		return -12; /* -ENOMEM */
	}

	// elem->head.mixer = mixer;
	// elem->head.resume = NULL;
	// elem->control = 0;
	// elem->idx_off = 0;
	// elem->head.id = index;
	// elem->val_type = val_type;
	// elem->channels = channels;
	// elem->private_data = opt;

	kctl = snd_ctl_new1(ncontrol, elem as *mut core::ffi::c_void);
	if kctl.is_null() {
		kfree(elem as *mut core::ffi::c_void);
		return -12; /* -ENOMEM */
	}

	if do_private_free {
		// kctl->private_free = elem_private_free;
	} else {
		// kctl->private_free = snd_usb_mixer_elem_free;
	}

	strscpy(core::ptr::null_mut(), name, 0);

	err = snd_usb_mixer_add_control(core::ptr::null_mut(), kctl);
	if err < 0 {
		return err;
	}

	if !elem_ret.is_null() {
		*elem_ret = elem;
	}

	return 0;
}

/* table of EQ controls - snd_us16x08_control_params structures */

/* table of compressor controls - snd_us16x08_control_params structures */

/* table of channel controls - snd_us16x08_control_params structures */

/* table of master controls - snd_us16x08_control_params structures */

#[no_mangle]
pub unsafe extern "C" fn snd_us16x08_controls_create(mixer: *mut usb_mixer_interface) -> i32
{
	let mut i: i32;
	let mut j: i32;
	let mut err: i32;
	let mut elem: *mut usb_mixer_elem_info;
	let mut comp_store: *mut snd_us16x08_comp_store;
	let mut meter_store: *mut snd_us16x08_meter_store;
	let mut eq_store: *mut snd_us16x08_eq_store;

	/* just check for non-MIDI interface */
	// if (mixer->hostif->desc.bInterfaceNumber == 3) {

		/* add routing control */
		// err = add_new_ctl(mixer, &snd_us16x08_route_ctl,
		// 	SND_US16X08_ID_ROUTE, USB_MIXER_U8, 8, "Line Out Route",
		// 	NULL, false, &elem);
		// if (err < 0) {
		// 	usb_audio_dbg(mixer->chip,
		// 		"Failed to create route control, err:%d\n",
		// 		err);
		// 	return err;
		// }
		// for (i = 0; i < 8; i++)
		// 	elem->cache_val[i] = i < 2 ? i : i + 2;
		// elem->cached = 0xff;

		/* create compressor mixer elements */
		comp_store = snd_us16x08_create_comp_store();
		if comp_store.is_null() {
			return -12; /* -ENOMEM */
		}

		/* add master controls */
		// for (i = 0; i < ARRAY_SIZE(master_controls); i++) {

		// 	err = add_new_ctl(mixer,
		// 		master_controls[i].kcontrol_new,
		// 		master_controls[i].control_id,
		// 		master_controls[i].type,
		// 		master_controls[i].num_channels,
		// 		master_controls[i].name,
		// 		comp_store,
		// 		i == 0, /* release comp_store only once */
		// 		&elem);
		// 	if (err < 0)
		// 		return err;
		// 	elem->cache_val[0] = master_controls[i].default_val;
		// 	elem->cached = 1;
		// }

		/* add channel controls */
		// for (i = 0; i < ARRAY_SIZE(channel_controls); i++) {

		// 	err = add_new_ctl(mixer,
		// 		channel_controls[i].kcontrol_new,
		// 		channel_controls[i].control_id,
		// 		channel_controls[i].type,
		// 		channel_controls[i].num_channels,
		// 		channel_controls[i].name,
		// 		comp_store,
		// 		false, &elem);
		// 	if (err < 0)
		// 		return err;
		// 	for (j = 0; j < SND_US16X08_MAX_CHANNELS; j++) {
		// 		elem->cache_val[j] =
		// 			channel_controls[i].default_val;
		// 	}
		// 	elem->cached = 0xffff;
		// }

		/* create eq store */
		eq_store = snd_us16x08_create_eq_store();
		if eq_store.is_null() {
			return -12; /* -ENOMEM */
		}

		/* add EQ controls */
		// for (i = 0; i < ARRAY_SIZE(eq_controls); i++) {

		// 	err = add_new_ctl(mixer,
		// 		eq_controls[i].kcontrol_new,
		// 		eq_controls[i].control_id,
		// 		eq_controls[i].type,
		// 		eq_controls[i].num_channels,
		// 		eq_controls[i].name,
		// 		eq_store,
		// 		i == 0, /* release eq_store only once */
		// 		NULL);
		// 	if (err < 0)
		// 		return err;
		// }

		/* add compressor controls */
		// for (i = 0; i < ARRAY_SIZE(comp_controls); i++) {

		// 	err = add_new_ctl(mixer,
		// 		comp_controls[i].kcontrol_new,
		// 		comp_controls[i].control_id,
		// 		comp_controls[i].type,
		// 		comp_controls[i].num_channels,
		// 		comp_controls[i].name,
		// 		comp_store,
		// 		false, NULL);
		// 	if (err < 0)
		// 		return err;
		// }

		/* create meters store */
		meter_store = snd_us16x08_create_meter_store();
		if meter_store.is_null() {
			return -12; /* -ENOMEM */
		}

		/* meter function 'get' must access to compressor store
		 * so place a reference here
		 */
		// meter_store->comp_store = comp_store;
		// err = add_new_ctl(mixer, &snd_us16x08_meter_ctl,
		// 	SND_US16X08_ID_METER, USB_MIXER_U16, 0, "Level Meter",
		// 	meter_store, true, NULL);
		// if (err < 0)
		// 	return err;
	// }

	return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
