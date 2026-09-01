// SPDX-License-Identifier: GPL-2.0-only
/****************************************************************************

   Copyright Echo Digital Audio Corporation (c) 1998 - 2004
   All rights reserved
   www.echoaudio.com

   This file is part of Echo Digital Audio's generic driver library.
   *************************************************************************

 Translation from C++ and adaptation for use in ALSA-Driver
 were made by Giuliano Pochini <pochini@shiny.it>

****************************************************************************/

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct device {
	_private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
	pub dev: *mut device,
}

#[repr(C)]
pub struct comm_page {
	pub control_register: u32,
	pub sample_rate: u32,
	pub vmixer: *mut c_int,
}

#[repr(C)]
pub struct echoaudio {
	pub card: *mut snd_card,
	pub device_id: u16,
	pub subdevice_id: u16,
	pub bad_board: bool,
	pub dsp_code_to_load: c_int,
	pub asic_loaded: bool,
	pub input_clock_types: u32,
	pub comm_page: *mut comm_page,
	pub sample_rate: u32,
	pub vmixer_gain: *mut *mut c_int,
}

unsafe extern "C" {
	fn init_dsp_comm_page(chip: *mut echoaudio) -> c_int;
	fn load_firmware(chip: *mut echoaudio) -> c_int;
	fn init_line_levels(chip: *mut echoaudio) -> c_int;
	fn wait_handshake(chip: *mut echoaudio) -> c_int;
	fn clear_handshake(chip: *mut echoaudio);
	fn send_vector(chip: *mut echoaudio, vector: c_int) -> c_int;
	fn num_pipes_out(chip: *mut echoaudio) -> u16;
	fn num_busses_out(chip: *mut echoaudio) -> u16;
	fn le32_to_cpu(value: u32) -> u32;
	fn cpu_to_le32(value: u32) -> u32;
}

const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const EIO: c_int = 5;

unsafe extern "C" {
	static INDIGO_DJ: u16;
	static FW_INDIGO_DJ_DSP: c_int;
	static ECHO_CLOCK_BIT_INTERNAL: u32;
	static MIA_96000: u32;
	static MIA_88200: u32;
	static MIA_48000: u32;
	static MIA_44100: u32;
	static MIA_32000: u32;
	static DSP_VC_UPDATE_CLOCKS: c_int;
	static DSP_VC_SET_VMIXER_GAIN: c_int;
}

// External C preprocessor diagnostics/macros supplied by the surrounding driver.
unsafe extern "C" {
	fn snd_BUG_ON(condition: bool) -> bool;
	fn dev_err(dev: *mut device, fmt: *const u8, ...);
	fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
}

// Forward declarations in the original C source:
// static int set_vmixer_gain(struct echoaudio *chip, u16 output, u16 pipe, int gain);
// static int update_vmixer_level(struct echoaudio *chip);

unsafe fn init_hw(chip: *mut echoaudio, device_id: u16, subdevice_id: u16) -> c_int {
	let mut err: c_int;

	if snd_BUG_ON((subdevice_id & 0xfff0) != INDIGO_DJ) {
		return -ENODEV;
	}

	err = init_dsp_comm_page(chip);
	if err != 0 {
		dev_err(
			(*(*chip).card).dev,
			c"init_hw - could not initialize DSP comm page\n".as_ptr() as *const u8,
		);
		return err;
	}

	(*chip).device_id = device_id;
	(*chip).subdevice_id = subdevice_id;
	(*chip).bad_board = true;
	(*chip).dsp_code_to_load = FW_INDIGO_DJ_DSP;
	/* Since this card has no ASIC, mark it as loaded so everything
	   works OK */
	(*chip).asic_loaded = true;
	(*chip).input_clock_types = ECHO_CLOCK_BIT_INTERNAL;

	err = load_firmware(chip);
	if err < 0 {
		return err;
	}
	(*chip).bad_board = false;

	err
}

unsafe fn set_mixer_defaults(chip: *mut echoaudio) -> c_int {
	init_line_levels(chip)
}

unsafe fn detect_input_clocks(_chip: *const echoaudio) -> u32 {
	ECHO_CLOCK_BIT_INTERNAL
}

/* The IndigoDJ has no ASIC. Just do nothing */
unsafe fn load_asic(_chip: *mut echoaudio) -> c_int {
	0
}

unsafe fn set_sample_rate(chip: *mut echoaudio, rate: u32) -> c_int {
	let control_reg: u32;

	match rate {
		96000 => {
			control_reg = MIA_96000;
		}
		88200 => {
			control_reg = MIA_88200;
		}
		48000 => {
			control_reg = MIA_48000;
		}
		44100 => {
			control_reg = MIA_44100;
		}
		32000 => {
			control_reg = MIA_32000;
		}
		_ => {
			dev_err(
				(*(*chip).card).dev,
				c"set_sample_rate: %d invalid!\n".as_ptr() as *const u8,
				rate,
			);
			return -EINVAL;
		}
	}

	/* Set the control register if it has changed */
	if control_reg != le32_to_cpu((*(*chip).comm_page).control_register) {
		if wait_handshake(chip) != 0 {
			return -EIO;
		}

		(*(*chip).comm_page).sample_rate = cpu_to_le32(rate); /* ignored by the DSP */
		(*(*chip).comm_page).control_register = cpu_to_le32(control_reg);
		(*chip).sample_rate = rate;

		clear_handshake(chip);
		return send_vector(chip, DSP_VC_UPDATE_CLOCKS);
	}
	0
}

/* This function routes the sound from a virtual channel to a real output */
unsafe fn set_vmixer_gain(
	chip: *mut echoaudio,
	output: u16,
	pipe: u16,
	gain: c_int,
) -> c_int {
	let index: c_int;

	if snd_BUG_ON(pipe >= num_pipes_out(chip) || output >= num_busses_out(chip)) {
		return -EINVAL;
	}

	if wait_handshake(chip) != 0 {
		return -EIO;
	}

	*(*(*chip).vmixer_gain.add(output as usize)).add(pipe as usize) = gain;
	index = output as c_int * num_pipes_out(chip) as c_int + pipe as c_int;
	*(*(*chip).comm_page).vmixer.add(index as usize) = gain;

	dev_dbg(
		(*(*chip).card).dev,
		c"set_vmixer_gain: pipe %d, out %d = %d\n".as_ptr() as *const u8,
		pipe as c_int,
		output as c_int,
		gain,
	);
	0
}

/* Tell the DSP to read and update virtual mixer levels in comm page. */
unsafe fn update_vmixer_level(chip: *mut echoaudio) -> c_int {
	if wait_handshake(chip) != 0 {
		return -EIO;
	}
	clear_handshake(chip);
	send_vector(chip, DSP_VC_SET_VMIXER_GAIN)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
