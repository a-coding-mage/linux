// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram VXpocket soundcards
 *
 * lowlevel routines for VXpocket soundcards
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

/* Dependencies from the original C includes:
 * linux/delay.h, linux/device.h, linux/firmware.h, linux/io.h,
 * sound/core.h, and "vxpocket.h".
 */

use core::ffi::{c_int, c_long, c_uint, c_ulong, c_uchar};

static vxp_reg_offset: [c_int; VX_REG_MAX as usize] = [
	0x00,		// ICR
	0x01,		// CVR
	0x02,		// ISR
	0x03,		// IVR
	0x05,		// RXH
	0x06,		// RXM
	0x07,		// RXL
	0x04,		// DMA
	0x08,		// CDSP
	0x09,		// LFREQ
	0x0a,		// HFREQ
	0x0b,		// DATA
	0x0c,		// MICRO
	0x0d,		// DIALOG
	0x0e,		// CSUER
	0x0f,		// RUER
];

#[inline]
unsafe fn vxp_reg_addr(_chip: *mut vx_core, reg: c_int) -> c_ulong {
	let chip: *mut snd_vxpocket = to_vxpocket(_chip);
	(*chip).port + vxp_reg_offset[reg as usize] as c_ulong
}

/*
 * snd_vx_inb - read a byte from the register
 * @offset: register offset
 */
unsafe fn vxp_inb(chip: *mut vx_core, offset: c_int) -> c_uchar {
	inb(vxp_reg_addr(chip, offset))
}

/*
 * snd_vx_outb - write a byte on the register
 * @offset: the register offset
 * @val: the value to write
 */
unsafe fn vxp_outb(chip: *mut vx_core, offset: c_int, val: c_uchar) {
	outb(val, vxp_reg_addr(chip, offset));
}

/*
 * The C source redefines vx_inb/vx_outb macros to call vxp_inb/vxp_outb
 * directly with VX_* register constants.
 */

/*
 * vx_check_magic - check the magic word on xilinx
 *
 * returns zero if a magic word is detected, or a negative error code.
 */
unsafe fn vx_check_magic(chip: *mut vx_core) -> c_int {
	let end_time: c_ulong = jiffies + HZ / 5;
	let mut c: c_int;
	loop {
		c = vxp_inb(chip, VX_CDSP) as c_int;
		if c == CDSP_MAGIC {
			return 0;
		}
		msleep(10);
		if !time_after_eq(end_time, jiffies) {
			break;
		}
	}
	dev_err((*(*chip).card).dev, "cannot find xilinx magic word (%x)\n", c);
	-EIO
}

/*
 * vx_reset_dsp - reset the DSP
 */

const XX_DSP_RESET_WAIT_TIME: c_uint = 2;	/* ms */

unsafe fn vxp_reset_dsp(_chip: *mut vx_core) {
	let chip: *mut snd_vxpocket = to_vxpocket(_chip);

	/* set the reset dsp bit to 1 */
	vxp_outb(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP | VXP_CDSP_DSP_RESET_MASK);
	vxp_inb(chip as *mut vx_core, VX_CDSP);
	mdelay(XX_DSP_RESET_WAIT_TIME);
	/* reset the bit */
	(*chip).regCDSP &= !VXP_CDSP_DSP_RESET_MASK;
	vxp_outb(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP);
	vxp_inb(chip as *mut vx_core, VX_CDSP);
	mdelay(XX_DSP_RESET_WAIT_TIME);
}

/*
 * reset codec bit
 */
unsafe fn vxp_reset_codec(_chip: *mut vx_core) {
	let chip: *mut snd_vxpocket = to_vxpocket(_chip);

	/* Set the reset CODEC bit to 1. */
	vxp_outb(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP | VXP_CDSP_CODEC_RESET_MASK);
	vxp_inb(chip as *mut vx_core, VX_CDSP);
	msleep(10);
	/* Set the reset CODEC bit to 0. */
	(*chip).regCDSP &= !VXP_CDSP_CODEC_RESET_MASK;
	vxp_outb(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP);
	vxp_inb(chip as *mut vx_core, VX_CDSP);
	msleep(1);
}

/*
 * vx_load_xilinx_binary - load the xilinx binary image
 * the binary image is the binary array converted from the bitstream file.
 */
unsafe fn vxp_load_xilinx_binary(_chip: *mut vx_core, fw: *const firmware) -> c_int {
	let chip: *mut snd_vxpocket = to_vxpocket(_chip);
	let mut i: c_uint;
	let mut c: c_int;
	let regCSUER: c_int;
	let regRUER: c_int;
	let mut image: *const c_uchar;
	let mut data: c_uchar;

	/* Switch to programmation mode */
	(*chip).regDIALOG |= VXP_DLG_XILINX_REPROG_MASK;
	vxp_outb(chip as *mut vx_core, VX_DIALOG, (*chip).regDIALOG);

	/* Save register CSUER and RUER */
	regCSUER = vxp_inb(chip as *mut vx_core, VX_CSUER) as c_int;
	regRUER = vxp_inb(chip as *mut vx_core, VX_RUER) as c_int;

	/* reset HF0 and HF1 */
	vxp_outb(chip as *mut vx_core, VX_ICR, 0);

	/* Wait for answer HF2 equal to 1 */
	if vx_check_isr(_chip, ISR_HF2, ISR_HF2, 20) < 0 {
		goto_error(chip, regCSUER, regRUER);
		return -EIO;
	}

	/* set HF1 for loading xilinx binary */
	vxp_outb(chip as *mut vx_core, VX_ICR, ICR_HF1);
	image = (*fw).data;
	i = 0;
	while i < (*fw).size as c_uint {
		data = *image;
		if vx_wait_isr_bit(_chip, ISR_TX_EMPTY) < 0 {
			goto_error(chip, regCSUER, regRUER);
			return -EIO;
		}
		vxp_outb(chip as *mut vx_core, VX_TXL, data);
		/* wait for reading */
		if vx_wait_for_rx_full(_chip) < 0 {
			goto_error(chip, regCSUER, regRUER);
			return -EIO;
		}
		c = vxp_inb(chip as *mut vx_core, VX_RXL) as c_int;
		if c != data as c_int {
			dev_err((*(*_chip).card).dev,
				"vxpocket: load xilinx mismatch at %d: 0x%x != 0x%x\n",
				i, c, data as c_int);
		}
		i += 1;
		image = image.add(1);
	}

	/* reset HF1 */
	vxp_outb(chip as *mut vx_core, VX_ICR, 0);

	/* wait for HF3 */
	if vx_check_isr(_chip, ISR_HF3, ISR_HF3, 20) < 0 {
		goto_error(chip, regCSUER, regRUER);
		return -EIO;
	}

	/* read the number of bytes received */
	if vx_wait_for_rx_full(_chip) < 0 {
		goto_error(chip, regCSUER, regRUER);
		return -EIO;
	}

	c = (vxp_inb(chip as *mut vx_core, VX_RXH) as c_int) << 16;
	c |= (vxp_inb(chip as *mut vx_core, VX_RXM) as c_int) << 8;
	c |= vxp_inb(chip as *mut vx_core, VX_RXL) as c_int;

	dev_dbg((*(*_chip).card).dev,
		"xilinx: dsp size received 0x%x, orig 0x%zx\n", c, (*fw).size);

	vxp_outb(chip as *mut vx_core, VX_ICR, ICR_HF0);

	/* TEMPO 250ms : wait until Xilinx is downloaded */
	msleep(300);

	/* test magical word */
	if vx_check_magic(_chip) < 0 {
		goto_error(chip, regCSUER, regRUER);
		return -EIO;
	}

	/* Restore register 0x0E and 0x0F (thus replacing COR and FCSR) */
	vxp_outb(chip as *mut vx_core, VX_CSUER, regCSUER as c_uchar);
	vxp_outb(chip as *mut vx_core, VX_RUER, regRUER as c_uchar);

	/* Reset the Xilinx's signal enabling IO access */
	(*chip).regDIALOG |= VXP_DLG_XILINX_REPROG_MASK;
	vxp_outb(chip as *mut vx_core, VX_DIALOG, (*chip).regDIALOG);
	vxp_inb(chip as *mut vx_core, VX_DIALOG);
	msleep(10);
	(*chip).regDIALOG &= !VXP_DLG_XILINX_REPROG_MASK;
	vxp_outb(chip as *mut vx_core, VX_DIALOG, (*chip).regDIALOG);
	vxp_inb(chip as *mut vx_core, VX_DIALOG);

	/* Reset of the Codec */
	vxp_reset_codec(_chip);
	vx_reset_dsp(_chip);

	0
}

unsafe fn goto_error(chip: *mut snd_vxpocket, regCSUER: c_int, regRUER: c_int) {
	vxp_outb(chip as *mut vx_core, VX_CSUER, regCSUER as c_uchar);
	vxp_outb(chip as *mut vx_core, VX_RUER, regRUER as c_uchar);
	(*chip).regDIALOG &= !VXP_DLG_XILINX_REPROG_MASK;
	vxp_outb(chip as *mut vx_core, VX_DIALOG, (*chip).regDIALOG);
}

/*
 * vxp_load_dsp - load_dsp callback
 */
unsafe fn vxp_load_dsp(vx: *mut vx_core, index: c_int, fw: *const firmware) -> c_int {
	let mut err: c_int;

	match index {
		0 => {
			/* xilinx boot */
			err = vx_check_magic(vx);
			if err < 0 {
				return err;
			}
			err = snd_vx_load_boot_image(vx, fw);
			if err < 0 {
				return err;
			}
			0
		}
		1 => {
			/* xilinx image */
			vxp_load_xilinx_binary(vx, fw)
		}
		2 => {
			/* DSP boot */
			snd_vx_dsp_boot(vx, fw)
		}
		3 => {
			/* DSP image */
			snd_vx_dsp_load(vx, fw)
		}
		_ => {
			snd_BUG();
			-EINVAL
		}
	}
}

/*
 * vx_test_and_ack - test and acknowledge interrupt
 *
 * called from irq hander, too
 *
 * spinlock held!
 */
unsafe fn vxp_test_and_ack(_chip: *mut vx_core) -> c_int {
	let chip: *mut snd_vxpocket = to_vxpocket(_chip);

	/* not booted yet? */
	if ((*_chip).chip_status & VX_STAT_XILINX_LOADED) == 0 {
		return -ENXIO;
	}

	if (vxp_inb(chip as *mut vx_core, VX_DIALOG) & VXP_DLG_MEMIRQ_MASK) == 0 {
		return -EIO;
	}

	/* ok, interrupts generated, now ack it */
	/* set ACQUIT bit up and down */
	vxp_outb(chip as *mut vx_core, VX_DIALOG, (*chip).regDIALOG | VXP_DLG_ACK_MEMIRQ_MASK);
	/* useless read just to spend some time and maintain
	 * the ACQUIT signal up for a while ( a bus cycle )
	 */
	vxp_inb(chip as *mut vx_core, VX_DIALOG);
	vxp_outb(chip as *mut vx_core, VX_DIALOG, (*chip).regDIALOG & !VXP_DLG_ACK_MEMIRQ_MASK);

	0
}

/*
 * vx_validate_irq - enable/disable IRQ
 */
unsafe fn vxp_validate_irq(_chip: *mut vx_core, enable: c_int) {
	let chip: *mut snd_vxpocket = to_vxpocket(_chip);

	/* Set the interrupt enable bit to 1 in CDSP register */
	if enable != 0 {
		(*chip).regCDSP |= VXP_CDSP_VALID_IRQ_MASK;
	} else {
		(*chip).regCDSP &= !VXP_CDSP_VALID_IRQ_MASK;
	}
	vxp_outb(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP);
}

/*
 * vx_setup_pseudo_dma - set up the pseudo dma read/write mode.
 * @do_write: 0 = read, 1 = set up for DMA write
 */
unsafe fn vx_setup_pseudo_dma(_chip: *mut vx_core, do_write: c_int) {
	let chip: *mut snd_vxpocket = to_vxpocket(_chip);

	/* Interrupt mode and HREQ pin enabled for host transmit / receive data transfers */
	vxp_outb(chip as *mut vx_core, VX_ICR, if do_write != 0 { ICR_TREQ } else { ICR_RREQ });
	/* Reset the pseudo-dma register */
	vxp_inb(chip as *mut vx_core, VX_ISR);
	vxp_outb(chip as *mut vx_core, VX_ISR, 0);

	/* Select DMA in read/write transfer mode and in 16-bit accesses */
	(*chip).regDIALOG |= VXP_DLG_DMA16_SEL_MASK;
	(*chip).regDIALOG |= if do_write != 0 { VXP_DLG_DMAWRITE_SEL_MASK } else { VXP_DLG_DMAREAD_SEL_MASK };
	vxp_outb(chip as *mut vx_core, VX_DIALOG, (*chip).regDIALOG);
}

/*
 * vx_release_pseudo_dma - disable the pseudo-DMA mode
 */
unsafe fn vx_release_pseudo_dma(_chip: *mut vx_core) {
	let chip: *mut snd_vxpocket = to_vxpocket(_chip);

	/* Disable DMA and 16-bit accesses */
	(*chip).regDIALOG &= !(VXP_DLG_DMAWRITE_SEL_MASK |
			     VXP_DLG_DMAREAD_SEL_MASK |
			     VXP_DLG_DMA16_SEL_MASK);
	vxp_outb(chip as *mut vx_core, VX_DIALOG, (*chip).regDIALOG);
	/* HREQ pin disabled. */
	vxp_outb(chip as *mut vx_core, VX_ICR, 0);
}

/*
 * vx_pseudo_dma_write - write bulk data on pseudo-DMA mode
 * @count: data length to transfer in bytes
 *
 * data size must be aligned to 6 bytes to ensure the 24bit alignment on DSP.
 * NB: call with a certain lock!
 */
unsafe fn vxp_dma_write(chip: *mut vx_core, runtime: *mut snd_pcm_runtime,
			  pipe: *mut vx_pipe, mut count: c_int) {
	let port: c_long = vxp_reg_addr(chip, VX_DMA) as c_long;
	let offset: c_int = (*pipe).hw_ptr;
	let mut addr: *mut u16 = (*runtime).dma_area.add(offset as usize) as *mut u16;

	vx_setup_pseudo_dma(chip, 1);
	if offset + count >= (*pipe).buffer_bytes {
		let mut length: c_int = (*pipe).buffer_bytes - offset;
		count -= length;
		length >>= 1; /* in 16bit words */
		/* Transfer using pseudo-dma. */
		while length > 0 {
			outw(*addr, port);
			addr = addr.add(1);
			length -= 1;
		}
		addr = (*runtime).dma_area as *mut u16;
		(*pipe).hw_ptr = 0;
	}
	(*pipe).hw_ptr += count;
	count >>= 1; /* in 16bit words */
	/* Transfer using pseudo-dma. */
	while count > 0 {
		outw(*addr, port);
		addr = addr.add(1);
		count -= 1;
	}
	vx_release_pseudo_dma(chip);
}

/*
 * vx_pseudo_dma_read - read bulk data on pseudo DMA mode
 * @offset: buffer offset in bytes
 * @count: data length to transfer in bytes
 *
 * the read length must be aligned to 6 bytes, as well as write.
 * NB: call with a certain lock!
 */
unsafe fn vxp_dma_read(chip: *mut vx_core, runtime: *mut snd_pcm_runtime,
			 pipe: *mut vx_pipe, mut count: c_int) {
	let pchip: *mut snd_vxpocket = to_vxpocket(chip);
	let port: c_long = vxp_reg_addr(chip, VX_DMA) as c_long;
	let offset: c_int = (*pipe).hw_ptr;
	let mut addr: *mut u16 = (*runtime).dma_area.add(offset as usize) as *mut u16;

	if snd_BUG_ON(count % 2) != 0 {
		return;
	}
	vx_setup_pseudo_dma(chip, 0);
	if offset + count >= (*pipe).buffer_bytes {
		let mut length: c_int = (*pipe).buffer_bytes - offset;
		count -= length;
		length >>= 1; /* in 16bit words */
		/* Transfer using pseudo-dma. */
		while length > 0 {
			*addr = inw(port);
			addr = addr.add(1);
			length -= 1;
		}
		addr = (*runtime).dma_area as *mut u16;
		(*pipe).hw_ptr = 0;
	}
	(*pipe).hw_ptr += count;
	count >>= 1; /* in 16bit words */
	/* Transfer using pseudo-dma. */
	while count > 1 {
		*addr = inw(port);
		addr = addr.add(1);
		count -= 1;
	}
	/* Disable DMA */
	(*pchip).regDIALOG &= !VXP_DLG_DMAREAD_SEL_MASK;
	vxp_outb(chip, VX_DIALOG, (*pchip).regDIALOG);
	/* Read the last word (16 bits) */
	*addr = inw(port);
	/* Disable 16-bit accesses */
	(*pchip).regDIALOG &= !VXP_DLG_DMA16_SEL_MASK;
	vxp_outb(chip, VX_DIALOG, (*pchip).regDIALOG);
	/* HREQ pin disabled. */
	vxp_outb(chip, VX_ICR, 0);
}

/*
 * write a codec data (24bit)
 */
unsafe fn vxp_write_codec_reg(chip: *mut vx_core, codec: c_int, mut data: c_uint) {
	let mut i: c_int;

	/* Activate access to the corresponding codec register */
	if codec == 0 {
		vxp_inb(chip, VX_LOFREQ);
	} else {
		vxp_inb(chip, VX_CODEC2);
	}

	/* We have to send 24 bits (3 x 8 bits). Start with most signif. Bit */
	i = 0;
	while i < 24 {
		vxp_outb(chip, VX_DATA, if (data & 0x800000) != 0 { VX_DATA_CODEC_MASK } else { 0 });
		i += 1;
		data <<= 1;
	}

	/* Terminate access to codec registers */
	vxp_inb(chip, VX_HIFREQ);
}

/*
 * vx_set_mic_boost - set mic boost level (on vxp440 only)
 * @boost: 0 = 20dB, 1 = +38dB
 */
pub unsafe fn vx_set_mic_boost(chip: *mut vx_core, boost: c_int) {
	let pchip: *mut snd_vxpocket = to_vxpocket(chip);

	if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
		return;
	}

	let _guard = guard_mutex(&mut (*chip).lock);
	if ((*pchip).regCDSP & P24_CDSP_MICS_SEL_MASK) != 0 {
		if boost != 0 {
			/* boost: 38 dB */
			(*pchip).regCDSP &= !P24_CDSP_MIC20_SEL_MASK;
			(*pchip).regCDSP |=  P24_CDSP_MIC38_SEL_MASK;
		} else {
			/* minimum value: 20 dB */
			(*pchip).regCDSP |=  P24_CDSP_MIC20_SEL_MASK;
			(*pchip).regCDSP &= !P24_CDSP_MIC38_SEL_MASK;
		}
		vxp_outb(chip, VX_CDSP, (*pchip).regCDSP);
	}
}

/*
 * remap the linear value (0-8) to the actual value (0-15)
 */
fn vx_compute_mic_level(mut level: c_int) -> c_int {
	match level {
		5 => level = 6,
		6 => level = 8,
		7 => level = 11,
		8 => level = 15,
		_ => {}
	}
	level
}

/*
 * vx_set_mic_level - set mic level (on vxpocket only)
 * @level: the mic level = 0 - 8 (max)
 */
pub unsafe fn vx_set_mic_level(chip: *mut vx_core, mut level: c_int) {
	let pchip: *mut snd_vxpocket = to_vxpocket(chip);

	if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
		return;
	}

	let _guard = guard_mutex(&mut (*chip).lock);
	if ((*pchip).regCDSP & VXP_CDSP_MIC_SEL_MASK) != 0 {
		level = vx_compute_mic_level(level);
		vxp_outb(chip, VX_MICRO, level as c_uchar);
	}
}

/*
 * change the input audio source
 */
unsafe fn vxp_change_audio_source(_chip: *mut vx_core, src: c_int) {
	let chip: *mut snd_vxpocket = to_vxpocket(_chip);

	match src {
		VX_AUDIO_SRC_DIGITAL => {
			(*chip).regCDSP |= VXP_CDSP_DATAIN_SEL_MASK;
			vxp_outb(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP);
		}
		VX_AUDIO_SRC_LINE => {
			(*chip).regCDSP &= !VXP_CDSP_DATAIN_SEL_MASK;
			if (*_chip).type_ == VX_TYPE_VXP440 {
				(*chip).regCDSP &= !P24_CDSP_MICS_SEL_MASK;
			} else {
				(*chip).regCDSP &= !VXP_CDSP_MIC_SEL_MASK;
			}
			vxp_outb(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP);
		}
		VX_AUDIO_SRC_MIC => {
			(*chip).regCDSP &= !VXP_CDSP_DATAIN_SEL_MASK;
			/* reset mic levels */
			if (*_chip).type_ == VX_TYPE_VXP440 {
				(*chip).regCDSP &= !P24_CDSP_MICS_SEL_MASK;
				if (*chip).mic_level != 0 {
					(*chip).regCDSP |=  P24_CDSP_MIC38_SEL_MASK;
				} else {
					(*chip).regCDSP |= P24_CDSP_MIC20_SEL_MASK;
				}
				vxp_outb(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP);
			} else {
				(*chip).regCDSP |= VXP_CDSP_MIC_SEL_MASK;
				vxp_outb(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP);
				vxp_outb(chip as *mut vx_core, VX_MICRO, vx_compute_mic_level((*chip).mic_level) as c_uchar);
			}
		}
		_ => {}
	}
}

/*
 * change the clock source
 * source = INTERNAL_QUARTZ or UER_SYNC
 */
unsafe fn vxp_set_clock_source(_chip: *mut vx_core, source: c_int) {
	let chip: *mut snd_vxpocket = to_vxpocket(_chip);

	if source == INTERNAL_QUARTZ {
		(*chip).regCDSP &= !VXP_CDSP_CLOCKIN_SEL_MASK;
	} else {
		(*chip).regCDSP |= VXP_CDSP_CLOCKIN_SEL_MASK;
	}
	vxp_outb(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP);
}

/*
 * reset the board
 */
unsafe fn vxp_reset_board(_chip: *mut vx_core, cold_reset: c_int) {
	let chip: *mut snd_vxpocket = to_vxpocket(_chip);
	let _ = cold_reset;

	(*chip).regCDSP = 0;
	(*chip).regDIALOG = 0;
}

/*
 * callbacks
 */
/* exported */
pub static snd_vxpocket_ops: snd_vx_ops = snd_vx_ops {
	in8: Some(vxp_inb),
	out8: Some(vxp_outb),
	test_and_ack: Some(vxp_test_and_ack),
	validate_irq: Some(vxp_validate_irq),
	write_codec: Some(vxp_write_codec_reg),
	reset_codec: Some(vxp_reset_codec),
	change_audio_source: Some(vxp_change_audio_source),
	set_clock_source: Some(vxp_set_clock_source),
	load_dsp: Some(vxp_load_dsp),
	add_controls: Some(vxp_add_mic_controls),
	reset_dsp: Some(vxp_reset_dsp),
	reset_board: Some(vxp_reset_board),
	dma_write: Some(vxp_dma_write),
	dma_read: Some(vxp_dma_read),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
