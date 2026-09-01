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


/******************************************************************************
	MIDI lowlevel code
******************************************************************************/

/* Start and stop Midi input */
unsafe fn enable_midi_input(chip: *mut echoaudio, enable: i8) -> i32
{
	dev_dbg!((*(*chip).card).dev, "enable_midi_input({})\n", enable);

	if wait_handshake(chip) != 0 {
		return -EIO;
	}

	if enable != 0 {
		(*chip).mtc_state = MIDI_IN_STATE_NORMAL;
		(*(*chip).comm_page).flags |=
			cpu_to_le32(DSP_FLAG_MIDI_INPUT);
	} else {
		(*(*chip).comm_page).flags &=
			!cpu_to_le32(DSP_FLAG_MIDI_INPUT);
	}

	clear_handshake(chip);
	return send_vector(chip, DSP_VC_UPDATE_FLAGS);
}



/* Send a buffer full of MIDI data to the DSP
Returns how many actually written or < 0 on error */
unsafe fn write_midi(chip: *mut echoaudio, data: *mut u8, bytes: i32) -> i32
{
	if snd_BUG_ON(bytes <= 0 || bytes >= MIDI_OUT_BUFFER_SIZE) != 0 {
		return -EINVAL;
	}

	if wait_handshake(chip) != 0 {
		return -EIO;
	}

	/* HF4 indicates that it is safe to write MIDI output data */
	if (get_dsp_register(chip, CHI32_STATUS_REG) & CHI32_STATUS_REG_HF4) == 0 {
		return 0;
	}

	(*(*chip).comm_page).midi_output[0] = bytes as _;
	core::ptr::copy_nonoverlapping(
		data,
		(*(*chip).comm_page).midi_output.as_mut_ptr().add(1) as *mut u8,
		bytes as usize,
	);
	(*(*chip).comm_page).midi_out_free_count = 0;
	clear_handshake(chip);
	send_vector(chip, DSP_VC_MIDI_WRITE);
	dev_dbg!((*(*chip).card).dev, "write_midi: {}\n", bytes);
	return bytes;
}



/* Run the state machine for MIDI input data
MIDI time code sync isn't supported by this code right now, but you still need
this state machine to parse the incoming MIDI data stream.  Every time the DSP
sees a 0xF1 byte come in, it adds the DSP sample position to the MIDI data
stream. The DSP sample position is represented as a 32 bit unsigned value,
with the high 16 bits first, followed by the low 16 bits. Since these aren't
real MIDI bytes, the following logic is needed to skip them. */
#[inline]
unsafe fn mtc_process_data(chip: *mut echoaudio, midi_byte: i16) -> i32
{
	match (*chip).mtc_state {
		MIDI_IN_STATE_NORMAL => {
			if midi_byte == 0xF1 {
				(*chip).mtc_state = MIDI_IN_STATE_TS_HIGH;
			}
		}
		MIDI_IN_STATE_TS_HIGH => {
			(*chip).mtc_state = MIDI_IN_STATE_TS_LOW;
			return MIDI_IN_SKIP_DATA;
		}
		MIDI_IN_STATE_TS_LOW => {
			(*chip).mtc_state = MIDI_IN_STATE_F1_DATA;
			return MIDI_IN_SKIP_DATA;
		}
		MIDI_IN_STATE_F1_DATA => {
			(*chip).mtc_state = MIDI_IN_STATE_NORMAL;
		}
		_ => {}
	}
	return 0;
}



/* This function is called from the IRQ handler and it reads the midi data
from the DSP's buffer.  It returns the number of bytes received. */
unsafe fn midi_service_irq(chip: *mut echoaudio) -> i32
{
	let mut count: i16;
	let mut midi_byte: i16;
	let mut i: i16;
	let mut received: i16;

	/* The count is at index 0, followed by actual data */
	count = le16_to_cpu((*(*chip).comm_page).midi_input[0]) as i16;

	if snd_BUG_ON(count >= MIDI_IN_BUFFER_SIZE as i16) != 0 {
		return 0;
	}

	/* Get the MIDI data from the comm page */
	received = 0;
	i = 1;
	while i <= count {
		/* Get the MIDI byte */
		midi_byte = le16_to_cpu((*(*chip).comm_page).midi_input[i as usize]) as i16;

		/* Parse the incoming MIDI stream. The incoming MIDI data
		consists of MIDI bytes and timestamps for the MIDI time code
		0xF1 bytes. mtc_process_data() is a little state machine that
		parses the stream. If you get MIDI_IN_SKIP_DATA back, then
		this is a timestamp byte, not a MIDI byte, so don't store it
		in the MIDI input buffer. */
		if mtc_process_data(chip, midi_byte) == MIDI_IN_SKIP_DATA {
			i += 1;
			continue;
		}

		(*chip).midi_buffer[received as usize] = midi_byte as u8;
		received += 1;
		i += 1;
	}

	return received as i32;
}




/******************************************************************************
	MIDI interface
******************************************************************************/

unsafe fn snd_echo_midi_input_open(substream: *mut snd_rawmidi_substream) -> i32
{
	let chip: *mut echoaudio = (*(*substream).rmidi).private_data as *mut echoaudio;

	(*chip).midi_in = substream;
	return 0;
}



unsafe fn snd_echo_midi_input_trigger(substream: *mut snd_rawmidi_substream,
					up: i32)
{
	let chip: *mut echoaudio = (*(*substream).rmidi).private_data as *mut echoaudio;

	if up != (*chip).midi_input_enabled {
		let _guard = guard_spinlock_irq(&mut (*chip).lock);
		enable_midi_input(chip, up as i8);
		(*chip).midi_input_enabled = up;
	}
}



unsafe fn snd_echo_midi_input_close(substream: *mut snd_rawmidi_substream) -> i32
{
	let chip: *mut echoaudio = (*(*substream).rmidi).private_data as *mut echoaudio;

	(*chip).midi_in = core::ptr::null_mut();
	return 0;
}



unsafe fn snd_echo_midi_output_open(substream: *mut snd_rawmidi_substream) -> i32
{
	let chip: *mut echoaudio = (*(*substream).rmidi).private_data as *mut echoaudio;

	(*chip).tinuse = 0;
	(*chip).midi_full = 0;
	(*chip).midi_out = substream;
	return 0;
}



unsafe fn snd_echo_midi_output_write(t: *mut timer_list)
{
	let chip: *mut echoaudio = timer_container_of!(chip, t, timer);
	let mut bytes: i32;
	let mut sent: i32;
	let mut time: i32;
	let mut buf: [u8; (MIDI_OUT_BUFFER_SIZE - 1) as usize] =
		[0; (MIDI_OUT_BUFFER_SIZE - 1) as usize];

	/* No interrupts are involved: we have to check at regular intervals
	if the card's output buffer has room for new data. */
	sent = 0;
	let _guard = guard_spinlock_irqsave(&mut (*chip).lock);
	(*chip).midi_full = 0;
	if snd_rawmidi_transmit_empty((*chip).midi_out) == 0 {
		bytes = snd_rawmidi_transmit_peek((*chip).midi_out, buf.as_mut_ptr(),
						  MIDI_OUT_BUFFER_SIZE - 1);
		dev_dbg!((*(*chip).card).dev, "Try to send {} bytes...\n", bytes);
		sent = write_midi(chip, buf.as_mut_ptr(), bytes);
		if sent < 0 {
			dev_err!((*(*chip).card).dev,
				"write_midi() error {}\n", sent);
			/* retry later */
			sent = 9000;
			(*chip).midi_full = 1;
		} else if sent > 0 {
			dev_dbg!((*(*chip).card).dev, "{} bytes sent\n", sent);
			snd_rawmidi_transmit_ack((*chip).midi_out, sent);
		} else {
			/* Buffer is full. DSP's internal buffer is 64 (128 ?)
			bytes long. Let's wait until half of them are sent */
			dev_dbg!((*(*chip).card).dev, "Full\n");
			sent = 32;
			(*chip).midi_full = 1;
		}
	}

	/* We restart the timer only if there is some data left to send */
	if snd_rawmidi_transmit_empty((*chip).midi_out) == 0 && (*chip).tinuse != 0 {
		/* The timer will expire slightly after the data has been
		   sent */
		time = (sent << 3) / 25 + 1;	/* 8/25=0.32ms to send a byte */
		mod_timer(&mut (*chip).timer, jiffies + (time * HZ + 999) / 1000);
		dev_dbg!((*(*chip).card).dev,
			"Timer armed({})\n", ((time * HZ + 999) / 1000));
	}
}



unsafe fn snd_echo_midi_output_trigger(substream: *mut snd_rawmidi_substream,
					 up: i32)
{
	let chip: *mut echoaudio = (*(*substream).rmidi).private_data as *mut echoaudio;
	let mut remove_timer: bool = false;

	dev_dbg!((*(*chip).card).dev, "snd_echo_midi_output_trigger({})\n", up);
	{
		let _guard = guard_spinlock_irq(&mut (*chip).lock);
		if up != 0 {
			if (*chip).tinuse == 0 {
				timer_setup(&mut (*chip).timer, snd_echo_midi_output_write,
					    0);
				(*chip).tinuse = 1;
			}
		} else {
			if (*chip).tinuse != 0 {
				(*chip).tinuse = 0;
				remove_timer = true;
			}
		}
	}

	if remove_timer {
		timer_delete_sync(&mut (*chip).timer);
		dev_dbg!((*(*chip).card).dev, "Timer removed\n");
		return;
	}

	if up != 0 && (*chip).midi_full == 0 {
		snd_echo_midi_output_write(&mut (*chip).timer);
	}
}



unsafe fn snd_echo_midi_output_close(substream: *mut snd_rawmidi_substream) -> i32
{
	let chip: *mut echoaudio = (*(*substream).rmidi).private_data as *mut echoaudio;

	(*chip).midi_out = core::ptr::null_mut();
	return 0;
}



static snd_echo_midi_input: snd_rawmidi_ops = snd_rawmidi_ops {
	open: Some(snd_echo_midi_input_open),
	close: Some(snd_echo_midi_input_close),
	trigger: Some(snd_echo_midi_input_trigger),
};

static snd_echo_midi_output: snd_rawmidi_ops = snd_rawmidi_ops {
	open: Some(snd_echo_midi_output_open),
	close: Some(snd_echo_midi_output_close),
	trigger: Some(snd_echo_midi_output_trigger),
};



/* <--snd_echo_probe() */
unsafe fn snd_echo_midi_create(card: *mut snd_card,
				chip: *mut echoaudio) -> i32
{
	let mut err: i32;

	err = snd_rawmidi_new(card, (*card).shortname, 0, 1, 1, &mut (*chip).rmidi);
	if err < 0 {
		return err;
	}

	strscpy((*(*chip).rmidi).name, (*card).shortname);
	(*(*chip).rmidi).private_data = chip as *mut _;

	snd_rawmidi_set_ops((*chip).rmidi, SNDRV_RAWMIDI_STREAM_INPUT,
			    &snd_echo_midi_input);
	snd_rawmidi_set_ops((*chip).rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT,
			    &snd_echo_midi_output);

	(*(*chip).rmidi).info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT |
		SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX;
	return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
