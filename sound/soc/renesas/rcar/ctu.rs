// SPDX-License-Identifier: GPL-2.0
//
// ctu.c
//
// Copyright (c) 2015 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

use crate::*;

const CTU_NAME: *const ::core::ffi::c_char = b"ctu\0".as_ptr() as *const ::core::ffi::c_char;

/*
 * User needs to setup CTU by amixer, and its settings are
 * based on below registers
 *
 * CTUn_CPMDR : amixser set "CTU Pass"
 * CTUn_SV0xR : amixser set "CTU SV0"
 * CTUn_SV1xR : amixser set "CTU SV1"
 * CTUn_SV2xR : amixser set "CTU SV2"
 * CTUn_SV3xR : amixser set "CTU SV3"
 *
 * [CTU Pass]
 * 0000: default
 * 0001: Connect input data of channel 0
 * 0010: Connect input data of channel 1
 * 0011: Connect input data of channel 2
 * 0100: Connect input data of channel 3
 * 0101: Connect input data of channel 4
 * 0110: Connect input data of channel 5
 * 0111: Connect input data of channel 6
 * 1000: Connect input data of channel 7
 * 1001: Connect calculated data by scale values of matrix row 0
 * 1010: Connect calculated data by scale values of matrix row 1
 * 1011: Connect calculated data by scale values of matrix row 2
 * 1100: Connect calculated data by scale values of matrix row 3
 *
 * [CTU SVx]
 * [Output0] = [SV00, SV01, SV02, SV03, SV04, SV05, SV06, SV07]
 * [Output1] = [SV10, SV11, SV12, SV13, SV14, SV15, SV16, SV17]
 * [Output2] = [SV20, SV21, SV22, SV23, SV24, SV25, SV26, SV27]
 * [Output3] = [SV30, SV31, SV32, SV33, SV34, SV35, SV36, SV37]
 * [Output4] = [ 0,   0,    0,    0,    0,    0,    0,    0   ]
 * [Output5] = [ 0,   0,    0,    0,    0,    0,    0,    0   ]
 * [Output6] = [ 0,   0,    0,    0,    0,    0,    0,    0   ]
 * [Output7] = [ 0,   0,    0,    0,    0,    0,    0,    0   ]
 *
 * [SVxx]
 * Plus					Minus
 * value	time		dB	value		time		dB
 * -----------------------------------------------------------------------
 * H'7F_FFFF	2		6	H'80_0000	2		6
 * ...
 * H'40_0000	1		0	H'C0_0000	1		0
 * ...
 * H'00_0001	2.38 x 10^-7	-132
 * H'00_0000	0		Mute	H'FF_FFFF	2.38 x 10^-7	-132
 *
 *
 * Ex) Input ch -> Output ch
 *	1ch     ->  0ch
 *	0ch     ->  1ch
 *
 *	amixer set "CTU Reset" on
 *	amixer set "CTU Pass" 9,10
 *	amixer set "CTU SV0" 0,4194304
 *	amixer set "CTU SV1" 4194304,0
 * or
 *	amixer set "CTU Reset" on
 *	amixer set "CTU Pass" 2,1
 */

#[repr(C)]
pub struct rsnd_ctu {
	pub mod_: rsnd_mod,
	pub pass: rsnd_kctrl_cfg_m,
	pub sv: [rsnd_kctrl_cfg_m; 4],
	pub reset: rsnd_kctrl_cfg_s,
	pub channels: ::core::ffi::c_int,
	pub flags: u32,
}

const KCTRL_INITIALIZED: u32 = 1 << 0;

unsafe fn rsnd_ctu_nr(priv_: *mut rsnd_priv) -> ::core::ffi::c_int {
	(*priv_).ctu_nr
}

unsafe fn rsnd_mod_to_ctu(mod_: *mut rsnd_mod) -> *mut rsnd_ctu {
	mod_ as *mut rsnd_ctu
}

unsafe fn rsnd_ctu_get(priv_: *mut rsnd_priv, id: ::core::ffi::c_int) -> *mut rsnd_ctu {
	((*priv_).ctu as *mut rsnd_ctu).offset(id as isize)
}

unsafe extern "C" fn rsnd_ctu_activation(mod_: *mut rsnd_mod) {
	rsnd_mod_write(mod_, CTU_SWRSR, 0);
	rsnd_mod_write(mod_, CTU_SWRSR, 1);
}

unsafe extern "C" fn rsnd_ctu_halt(mod_: *mut rsnd_mod) {
	rsnd_mod_write(mod_, CTU_CTUIR, 1);
	rsnd_mod_write(mod_, CTU_SWRSR, 0);
}

unsafe extern "C" fn rsnd_ctu_probe_(
	mod_: *mut rsnd_mod,
	io: *mut rsnd_dai_stream,
	priv_: *mut rsnd_priv,
) -> ::core::ffi::c_int {
	rsnd_cmd_attach(io, rsnd_mod_id(mod_))
}

unsafe extern "C" fn rsnd_ctu_value_init(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) {
	let ctu = rsnd_mod_to_ctu(mod_);
	let mut cpmdr: u32 = 0;
	let mut scmdr: u32 = 0;
	let mut i: ::core::ffi::c_int;
	let mut j: ::core::ffi::c_int;

	i = 0;
	while i < RSND_MAX_CHANNELS {
		let val: u32 = rsnd_kctrl_valm(&mut (*ctu).pass, i);

		cpmdr |= val << (28 - (i * 4));

		if val > 0x8 && scmdr < val - 0x8 {
			scmdr = val - 0x8;
		}

		i += 1;
	}

	rsnd_mod_write(mod_, CTU_CTUIR, 1);

	rsnd_mod_write(mod_, CTU_ADINR, rsnd_runtime_channel_original(io) as u32);

	rsnd_mod_write(mod_, CTU_CPMDR, cpmdr);

	rsnd_mod_write(mod_, CTU_SCMDR, scmdr);

	i = 0;
	while i < 4 {
		if i as u32 >= scmdr {
			break;
		}

		j = 0;
		while j < RSND_MAX_CHANNELS {
			rsnd_mod_write(
				mod_,
				CTU_SVxxR(i, j),
				rsnd_kctrl_valm(&mut (*ctu).sv[i as usize], j),
			);
			j += 1;
		}

		i += 1;
	}

	rsnd_mod_write(mod_, CTU_CTUIR, 0);
}

unsafe extern "C" fn rsnd_ctu_value_reset(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) {
	let ctu = rsnd_mod_to_ctu(mod_);
	let mut i: ::core::ffi::c_int;

	if rsnd_kctrl_vals(&mut (*ctu).reset) == 0 {
		return;
	}

	i = 0;
	while i < RSND_MAX_CHANNELS {
		*rsnd_kctrl_valm_mut(&mut (*ctu).pass, i) = 0;
		*rsnd_kctrl_valm_mut(&mut (*ctu).sv[0], i) = 0;
		*rsnd_kctrl_valm_mut(&mut (*ctu).sv[1], i) = 0;
		*rsnd_kctrl_valm_mut(&mut (*ctu).sv[2], i) = 0;
		*rsnd_kctrl_valm_mut(&mut (*ctu).sv[3], i) = 0;
		i += 1;
	}
	*rsnd_kctrl_vals_mut(&mut (*ctu).reset) = 0;
}

unsafe extern "C" fn rsnd_ctu_init(
	mod_: *mut rsnd_mod,
	io: *mut rsnd_dai_stream,
	priv_: *mut rsnd_priv,
) -> ::core::ffi::c_int {
	let ret: ::core::ffi::c_int;

	ret = rsnd_mod_power_on(mod_);
	if ret < 0 {
		return ret;
	}

	rsnd_ctu_activation(mod_);

	rsnd_ctu_value_init(io, mod_);

	0
}

unsafe extern "C" fn rsnd_ctu_quit(
	mod_: *mut rsnd_mod,
	io: *mut rsnd_dai_stream,
	priv_: *mut rsnd_priv,
) -> ::core::ffi::c_int {
	rsnd_ctu_halt(mod_);

	rsnd_mod_power_off(mod_);

	0
}

unsafe extern "C" fn rsnd_ctu_pcm_new(
	mod_: *mut rsnd_mod,
	io: *mut rsnd_dai_stream,
	rtd: *mut snd_soc_pcm_runtime,
) -> ::core::ffi::c_int {
	let ctu = rsnd_mod_to_ctu(mod_);
	let mut ret: ::core::ffi::c_int;

	if rsnd_flags_has(ctu, KCTRL_INITIALIZED) {
		return 0;
	}

	/* CTU Pass */
	ret = rsnd_kctrl_new_m(
		mod_,
		io,
		rtd,
		b"CTU Pass\0".as_ptr() as *const ::core::ffi::c_char,
		Some(rsnd_kctrl_accept_anytime),
		::core::ptr::null_mut(),
		&mut (*ctu).pass,
		RSND_MAX_CHANNELS,
		0xC,
	);
	if ret < 0 {
		return ret;
	}

	/* ROW0 */
	ret = rsnd_kctrl_new_m(
		mod_,
		io,
		rtd,
		b"CTU SV0\0".as_ptr() as *const ::core::ffi::c_char,
		Some(rsnd_kctrl_accept_anytime),
		::core::ptr::null_mut(),
		&mut (*ctu).sv[0],
		RSND_MAX_CHANNELS,
		0x00FFFFFF,
	);
	if ret < 0 {
		return ret;
	}

	/* ROW1 */
	ret = rsnd_kctrl_new_m(
		mod_,
		io,
		rtd,
		b"CTU SV1\0".as_ptr() as *const ::core::ffi::c_char,
		Some(rsnd_kctrl_accept_anytime),
		::core::ptr::null_mut(),
		&mut (*ctu).sv[1],
		RSND_MAX_CHANNELS,
		0x00FFFFFF,
	);
	if ret < 0 {
		return ret;
	}

	/* ROW2 */
	ret = rsnd_kctrl_new_m(
		mod_,
		io,
		rtd,
		b"CTU SV2\0".as_ptr() as *const ::core::ffi::c_char,
		Some(rsnd_kctrl_accept_anytime),
		::core::ptr::null_mut(),
		&mut (*ctu).sv[2],
		RSND_MAX_CHANNELS,
		0x00FFFFFF,
	);
	if ret < 0 {
		return ret;
	}

	/* ROW3 */
	ret = rsnd_kctrl_new_m(
		mod_,
		io,
		rtd,
		b"CTU SV3\0".as_ptr() as *const ::core::ffi::c_char,
		Some(rsnd_kctrl_accept_anytime),
		::core::ptr::null_mut(),
		&mut (*ctu).sv[3],
		RSND_MAX_CHANNELS,
		0x00FFFFFF,
	);
	if ret < 0 {
		return ret;
	}

	/* Reset */
	ret = rsnd_kctrl_new_s(
		mod_,
		io,
		rtd,
		b"CTU Reset\0".as_ptr() as *const ::core::ffi::c_char,
		Some(rsnd_kctrl_accept_anytime),
		Some(rsnd_ctu_value_reset),
		&mut (*ctu).reset,
		1,
	);

	rsnd_flags_set(ctu, KCTRL_INITIALIZED);

	ret
}

unsafe extern "C" fn rsnd_ctu_id(mod_: *mut rsnd_mod) -> ::core::ffi::c_int {
	/*
	 * ctu00: -> 0, ctu01: -> 0, ctu02: -> 0, ctu03: -> 0
	 * ctu10: -> 1, ctu11: -> 1, ctu12: -> 1, ctu13: -> 1
	 */
	(*mod_).id / 4
}

unsafe extern "C" fn rsnd_ctu_id_sub(mod_: *mut rsnd_mod) -> ::core::ffi::c_int {
	/*
	 * ctu00: -> 0, ctu01: -> 1, ctu02: -> 2, ctu03: -> 3
	 * ctu10: -> 0, ctu11: -> 1, ctu12: -> 2, ctu13: -> 3
	 */
	(*mod_).id % 4
}

/* CONFIG_DEBUG_FS: debug_info field is present only when debugfs support is enabled. */
#[cfg(CONFIG_DEBUG_FS)]
unsafe extern "C" fn rsnd_ctu_debug_info(
	m: *mut seq_file,
	io: *mut rsnd_dai_stream,
	mod_: *mut rsnd_mod,
) {
	rsnd_debugfs_mod_reg_show(
		m,
		mod_,
		RSND_BASE_SCU,
		0x500 + rsnd_mod_id_raw(mod_) * 0x100,
		0x100,
	);
}

static mut rsnd_ctu_ops: rsnd_mod_ops = rsnd_mod_ops {
	name: CTU_NAME,
	probe: Some(rsnd_ctu_probe_),
	init: Some(rsnd_ctu_init),
	quit: Some(rsnd_ctu_quit),
	pcm_new: Some(rsnd_ctu_pcm_new),
	get_status: Some(rsnd_mod_get_status),
	id: Some(rsnd_ctu_id),
	id_sub: Some(rsnd_ctu_id_sub),
	id_cmd: Some(rsnd_mod_id_raw),
	#[cfg(CONFIG_DEBUG_FS)]
	debug_info: Some(rsnd_ctu_debug_info),
};

pub unsafe extern "C" fn rsnd_ctu_mod_get(
	priv_: *mut rsnd_priv,
	mut id: ::core::ffi::c_int,
) -> *mut rsnd_mod {
	if WARN_ON(id < 0 || id >= rsnd_ctu_nr(priv_)) {
		id = 0;
	}

	rsnd_mod_get(rsnd_ctu_get(priv_, id))
}

pub unsafe extern "C" fn rsnd_ctu_probe(priv_: *mut rsnd_priv) -> ::core::ffi::c_int {
	let mut node: *mut device_node;
	let dev: *mut device = rsnd_priv_to_dev(priv_);
	let mut ctu: *mut rsnd_ctu;
	let mut clk: *mut clk;
	let mut i: ::core::ffi::c_int;
	let nr: ::core::ffi::c_int;
	let mut ret: ::core::ffi::c_int;

	node = rsnd_ctu_of_node(priv_);
	if node.is_null() {
		return 0; /* not used is not error */
	}

	nr = of_get_child_count(node);
	if nr == 0 {
		ret = -EINVAL;
		goto_rsnd_ctu_probe_done(node, ret)
	} else {
		ctu = devm_kcalloc(
			dev,
			nr as usize,
			::core::mem::size_of::<rsnd_ctu>(),
			GFP_KERNEL,
		) as *mut rsnd_ctu;
		if ctu.is_null() {
			ret = -ENOMEM;
			goto_rsnd_ctu_probe_done(node, ret)
		} else {
			(*priv_).ctu_nr = nr;
			(*priv_).ctu = ctu as *mut ::core::ffi::c_void;

			i = 0;
			ret = 0;
			for_each_child_of_node_scoped!(node, np, {
				ctu = rsnd_ctu_get(priv_, i);

				/*
				 * CTU00, CTU01, CTU02, CTU03 => CTU0
				 * CTU10, CTU11, CTU12, CTU13 => CTU1
				 */
				clk = rsnd_devm_clk_get_indexed(dev, CTU_NAME, i / 4);
				if IS_ERR(clk) {
					ret = PTR_ERR(clk);
					break;
				}

				ret = rsnd_mod_init(
					priv_,
					rsnd_mod_get(ctu),
					&raw mut rsnd_ctu_ops,
					clk,
					::core::ptr::null_mut(),
					RSND_MOD_CTU,
					i,
				);
				if ret != 0 {
					break;
				}

				i += 1;
			});

			goto_rsnd_ctu_probe_done(node, ret)
		}
	}
}

unsafe fn goto_rsnd_ctu_probe_done(
	node: *mut device_node,
	ret: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
	of_node_put(node);

	ret
}

pub unsafe extern "C" fn rsnd_ctu_remove(priv_: *mut rsnd_priv) {
	let mut ctu: *mut rsnd_ctu;
	let mut i: ::core::ffi::c_int;

	i = 0;
	while i < rsnd_ctu_nr(priv_) {
		ctu = rsnd_ctu_get(priv_, i);
		rsnd_mod_quit(rsnd_mod_get(ctu));
		i += 1;
	}
}

pub unsafe extern "C" fn rsnd_ctu_suspend(priv_: *mut rsnd_priv) {
	let mut ctu: *mut rsnd_ctu;
	let mut i: ::core::ffi::c_int;

	i = 0;
	while i < rsnd_ctu_nr(priv_) {
		ctu = rsnd_ctu_get(priv_, i);
		rsnd_suspend_clk_reset((*rsnd_mod_get(ctu)).clk, (*rsnd_mod_get(ctu)).rstc);
		i += 1;
	}
}

pub unsafe extern "C" fn rsnd_ctu_resume(priv_: *mut rsnd_priv) {
	let mut ctu: *mut rsnd_ctu;
	let mut i: ::core::ffi::c_int;

	i = 0;
	while i < rsnd_ctu_nr(priv_) {
		ctu = rsnd_ctu_get(priv_, i);
		rsnd_resume_clk_reset((*rsnd_mod_get(ctu)).clk, (*rsnd_mod_get(ctu)).rstc);
		i += 1;
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
