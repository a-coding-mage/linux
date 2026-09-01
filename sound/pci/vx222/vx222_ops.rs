// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram VX222 V2/Mic soundcards
 *
 * VX222-specific low-level routines
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

use crate::*;

static VX2_REG_OFFSET: [i32; VX_REG_MAX as usize] = {
    let mut a = [0i32; VX_REG_MAX as usize];
    a[VX_ICR as usize] = 0x00;
    a[VX_CVR as usize] = 0x04;
    a[VX_ISR as usize] = 0x08;
    a[VX_IVR as usize] = 0x0c;
    a[VX_RXH as usize] = 0x14;
    a[VX_RXM as usize] = 0x18;
    a[VX_RXL as usize] = 0x1c;
    a[VX_DMA as usize] = 0x10;
    a[VX_CDSP as usize] = 0x20;
    a[VX_CFG as usize] = 0x24;
    a[VX_RUER as usize] = 0x28;
    a[VX_DATA as usize] = 0x2c;
    a[VX_STATUS as usize] = 0x30;
    a[VX_LOFREQ as usize] = 0x34;
    a[VX_HIFREQ as usize] = 0x38;
    a[VX_CSUER as usize] = 0x3c;
    a[VX_SELMIC as usize] = 0x40;
    a[VX_COMPOT as usize] = 0x44; // Write: POTENTIOMETER ; Read: COMPRESSION LEVEL activate
    a[VX_SCOMPR as usize] = 0x48; // Read: COMPRESSION THRESHOLD activate
    a[VX_GLIMIT as usize] = 0x4c; // Read: LEVEL LIMITATION activate
    a[VX_INTCSR as usize] = 0x4c; // VX_INTCSR_REGISTER_OFFSET
    a[VX_CNTRL as usize] = 0x50; // VX_CNTRL_REGISTER_OFFSET
    a[VX_GPIOC as usize] = 0x54; // VX_GPIOC (new with PLX9030)
    a
};

static VX2_REG_INDEX: [i32; VX_REG_MAX as usize] = {
    let mut a = [0i32; VX_REG_MAX as usize];
    a[VX_ICR as usize] = 1;
    a[VX_CVR as usize] = 1;
    a[VX_ISR as usize] = 1;
    a[VX_IVR as usize] = 1;
    a[VX_RXH as usize] = 1;
    a[VX_RXM as usize] = 1;
    a[VX_RXL as usize] = 1;
    a[VX_DMA as usize] = 1;
    a[VX_CDSP as usize] = 1;
    a[VX_CFG as usize] = 1;
    a[VX_RUER as usize] = 1;
    a[VX_DATA as usize] = 1;
    a[VX_STATUS as usize] = 1;
    a[VX_LOFREQ as usize] = 1;
    a[VX_HIFREQ as usize] = 1;
    a[VX_CSUER as usize] = 1;
    a[VX_SELMIC as usize] = 1;
    a[VX_COMPOT as usize] = 1;
    a[VX_SCOMPR as usize] = 1;
    a[VX_GLIMIT as usize] = 1;
    a[VX_INTCSR as usize] = 0; /* on the PLX */
    a[VX_CNTRL as usize] = 0; /* on the PLX */
    a[VX_GPIOC as usize] = 0; /* on the PLX */
    a
};

#[inline]
unsafe fn vx2_reg_addr(_chip: *mut vx_core, reg: i32) -> libc::c_ulong {
    let chip = to_vx222(_chip);
    (*chip).port[VX2_REG_INDEX[reg as usize] as usize]
        + VX2_REG_OFFSET[reg as usize] as libc::c_ulong
}

/**
 * vx2_inb - read a byte from the register
 * @chip: VX core instance
 * @offset: register enum
 */
unsafe extern "C" fn vx2_inb(chip: *mut vx_core, offset: i32) -> libc::c_uchar {
    inb(vx2_reg_addr(chip, offset))
}

/**
 * vx2_outb - write a byte on the register
 * @chip: VX core instance
 * @offset: the register offset
 * @val: the value to write
 */
unsafe extern "C" fn vx2_outb(chip: *mut vx_core, offset: i32, val: libc::c_uchar) {
    outb(val, vx2_reg_addr(chip, offset));
    /*
    dev_dbg(chip->card->dev, "outb: %x -> %x\n", val, vx2_reg_addr(chip, offset));
    */
}

/**
 * vx2_inl - read a 32bit word from the register
 * @chip: VX core instance
 * @offset: register enum
 */
unsafe extern "C" fn vx2_inl(chip: *mut vx_core, offset: i32) -> libc::c_uint {
    inl(vx2_reg_addr(chip, offset))
}

/**
 * vx2_outl - write a 32bit word on the register
 * @chip: VX core instance
 * @offset: the register enum
 * @val: the value to write
 */
unsafe extern "C" fn vx2_outl(chip: *mut vx_core, offset: i32, val: libc::c_uint) {
    /*
    dev_dbg(chip->card->dev, "outl: %x -> %x\n", val, vx2_reg_addr(chip, offset));
    */
    outl(val, vx2_reg_addr(chip, offset));
}

/*
 * redefine macros to call directly
 */
unsafe fn vx_inb(chip: *mut vx_core, reg: i32) -> libc::c_uchar {
    vx2_inb(chip, reg)
}

unsafe fn vx_outb(chip: *mut vx_core, reg: i32, val: libc::c_uchar) {
    vx2_outb(chip, reg, val);
}

unsafe fn vx_inl(chip: *mut vx_core, reg: i32) -> libc::c_uint {
    vx2_inl(chip, reg)
}

unsafe fn vx_outl(chip: *mut vx_core, reg: i32, val: libc::c_uint) {
    vx2_outl(chip, reg, val);
}

/*
 * vx_reset_dsp - reset the DSP
 */

const XX_DSP_RESET_WAIT_TIME: libc::c_uint = 2; /* ms */

unsafe extern "C" fn vx2_reset_dsp(_chip: *mut vx_core) {
    let chip = to_vx222(_chip);

    /* set the reset dsp bit to 0 */
    vx_outl(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP & !VX_CDSP_DSP_RESET_MASK);

    mdelay(XX_DSP_RESET_WAIT_TIME);

    (*chip).regCDSP |= VX_CDSP_DSP_RESET_MASK;
    /* set the reset dsp bit to 1 */
    vx_outl(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP);
}

unsafe fn vx2_test_xilinx(_chip: *mut vx_core) -> i32 {
    let chip = to_vx222(_chip);
    let mut data: libc::c_uint;

    dev_dbg((*(*_chip).card).dev, c"testing xilinx...\n".as_ptr());
    /* This test uses several write/read sequences on TEST0 and TEST1 bits
     * to figure out whever or not the xilinx was correctly loaded
     */

    /* We write 1 on CDSP.TEST0. We should get 0 on STATUS.TEST0. */
    vx_outl(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP | VX_CDSP_TEST0_MASK);
    vx_inl(chip as *mut vx_core, VX_ISR);
    data = vx_inl(chip as *mut vx_core, VX_STATUS);
    if (data & VX_STATUS_VAL_TEST0_MASK) == VX_STATUS_VAL_TEST0_MASK {
        dev_dbg((*(*_chip).card).dev, c"bad!\n".as_ptr());
        return -ENODEV;
    }

    /* We write 0 on CDSP.TEST0. We should get 1 on STATUS.TEST0. */
    vx_outl(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP & !VX_CDSP_TEST0_MASK);
    vx_inl(chip as *mut vx_core, VX_ISR);
    data = vx_inl(chip as *mut vx_core, VX_STATUS);
    if (data & VX_STATUS_VAL_TEST0_MASK) == 0 {
        dev_dbg((*(*_chip).card).dev, c"bad! #2\n".as_ptr());
        return -ENODEV;
    }

    if (*_chip).type_ == VX_TYPE_BOARD {
        /* not implemented on VX_2_BOARDS */
        /* We write 1 on CDSP.TEST1. We should get 0 on STATUS.TEST1. */
        vx_outl(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP | VX_CDSP_TEST1_MASK);
        vx_inl(chip as *mut vx_core, VX_ISR);
        data = vx_inl(chip as *mut vx_core, VX_STATUS);
        if (data & VX_STATUS_VAL_TEST1_MASK) == VX_STATUS_VAL_TEST1_MASK {
            dev_dbg((*(*_chip).card).dev, c"bad! #3\n".as_ptr());
            return -ENODEV;
        }

        /* We write 0 on CDSP.TEST1. We should get 1 on STATUS.TEST1. */
        vx_outl(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP & !VX_CDSP_TEST1_MASK);
        vx_inl(chip as *mut vx_core, VX_ISR);
        data = vx_inl(chip as *mut vx_core, VX_STATUS);
        if (data & VX_STATUS_VAL_TEST1_MASK) == 0 {
            dev_dbg((*(*_chip).card).dev, c"bad! #4\n".as_ptr());
            return -ENODEV;
        }
    }
    dev_dbg((*(*_chip).card).dev, c"ok, xilinx fine.\n".as_ptr());
    0
}

/**
 * vx2_setup_pseudo_dma - set up the pseudo dma read/write mode.
 * @chip: VX core instance
 * @do_write: 0 = read, 1 = set up for DMA write
 */
unsafe fn vx2_setup_pseudo_dma(chip: *mut vx_core, do_write: i32) {
    /* Interrupt mode and HREQ pin enabled for host transmit data transfers
     * (in case of the use of the pseudo-dma facility).
     */
    vx_outl(chip, VX_ICR, if do_write != 0 { ICR_TREQ } else { ICR_RREQ });

    /* Reset the pseudo-dma register (in case of the use of the
     * pseudo-dma facility).
     */
    vx_outl(chip, VX_RESET_DMA, 0);
}

/*
 * vx_release_pseudo_dma - disable the pseudo-DMA mode
 */
#[inline]
unsafe fn vx2_release_pseudo_dma(chip: *mut vx_core) {
    /* HREQ pin disabled. */
    vx_outl(chip, VX_ICR, 0);
}

/* pseudo-dma write */
unsafe extern "C" fn vx2_dma_write(
    chip: *mut vx_core,
    runtime: *mut snd_pcm_runtime,
    pipe: *mut vx_pipe,
    mut count: i32,
) {
    let port = vx2_reg_addr(chip, VX_DMA);
    let offset = (*pipe).hw_ptr;
    let mut addr = (*runtime).dma_area.add(offset as usize) as *mut u32;

    if snd_BUG_ON((count % 4) != 0) != 0 {
        return;
    }

    vx2_setup_pseudo_dma(chip, 1);

    /* Transfer using pseudo-dma.
     */
    if offset + count >= (*pipe).buffer_bytes {
        let mut length = (*pipe).buffer_bytes - offset;
        count -= length;
        length >>= 2; /* in 32bit words */
        /* Transfer using pseudo-dma. */
        while length > 0 {
            outl(*addr, port);
            addr = addr.add(1);
            length -= 1;
        }
        addr = (*runtime).dma_area as *mut u32;
        (*pipe).hw_ptr = 0;
    }
    (*pipe).hw_ptr += count;
    count >>= 2; /* in 32bit words */
    /* Transfer using pseudo-dma. */
    while count > 0 {
        outl(*addr, port);
        addr = addr.add(1);
        count -= 1;
    }

    vx2_release_pseudo_dma(chip);
}

/* pseudo dma read */
unsafe extern "C" fn vx2_dma_read(
    chip: *mut vx_core,
    runtime: *mut snd_pcm_runtime,
    pipe: *mut vx_pipe,
    mut count: i32,
) {
    let offset = (*pipe).hw_ptr;
    let mut addr = (*runtime).dma_area.add(offset as usize) as *mut u32;
    let port = vx2_reg_addr(chip, VX_DMA);

    if snd_BUG_ON((count % 4) != 0) != 0 {
        return;
    }

    vx2_setup_pseudo_dma(chip, 0);
    /* Transfer using pseudo-dma.
     */
    if offset + count >= (*pipe).buffer_bytes {
        let mut length = (*pipe).buffer_bytes - offset;
        count -= length;
        length >>= 2; /* in 32bit words */
        /* Transfer using pseudo-dma. */
        while length > 0 {
            *addr = inl(port);
            addr = addr.add(1);
            length -= 1;
        }
        addr = (*runtime).dma_area as *mut u32;
        (*pipe).hw_ptr = 0;
    }
    (*pipe).hw_ptr += count;
    count >>= 2; /* in 32bit words */
    /* Transfer using pseudo-dma. */
    while count > 0 {
        *addr = inl(port);
        addr = addr.add(1);
        count -= 1;
    }

    vx2_release_pseudo_dma(chip);
}

const VX_XILINX_RESET_MASK: libc::c_uint = 0x40000000;
const VX_USERBIT0_MASK: libc::c_uint = 0x00000004;
const VX_USERBIT1_MASK: libc::c_uint = 0x00000020;
const VX_CNTRL_REGISTER_VALUE: libc::c_uint = 0x00172012;

/*
 * transfer counts bits to PLX
 */
unsafe fn put_xilinx_data(
    chip: *mut vx_core,
    port: libc::c_uint,
    counts: libc::c_uint,
    data: libc::c_uchar,
) -> i32 {
    let mut i: libc::c_uint = 0;

    while i < counts {
        let mut val: libc::c_uint;

        /* set the clock bit to 0. */
        val = VX_CNTRL_REGISTER_VALUE & !VX_USERBIT0_MASK;
        vx2_outl(chip, port as i32, val);
        vx2_inl(chip, port as i32);
        udelay(1);

        if (data as libc::c_uint & (1 << i)) != 0 {
            val |= VX_USERBIT1_MASK;
        } else {
            val &= !VX_USERBIT1_MASK;
        }
        vx2_outl(chip, port as i32, val);
        vx2_inl(chip, port as i32);

        /* set the clock bit to 1. */
        val |= VX_USERBIT0_MASK;
        vx2_outl(chip, port as i32, val);
        vx2_inl(chip, port as i32);
        udelay(1);
        i += 1;
    }
    0
}

/*
 * load the xilinx image
 */
unsafe fn vx2_load_xilinx_binary(chip: *mut vx_core, xilinx: *const firmware) -> i32 {
    let mut i: libc::c_uint;
    let port: libc::c_uint;
    let mut image: *const libc::c_uchar;

    /* XILINX reset (wait at least 1 millisecond between reset on and off). */
    vx_outl(chip, VX_CNTRL, VX_CNTRL_REGISTER_VALUE | VX_XILINX_RESET_MASK);
    vx_inl(chip, VX_CNTRL);
    msleep(10);
    vx_outl(chip, VX_CNTRL, VX_CNTRL_REGISTER_VALUE);
    vx_inl(chip, VX_CNTRL);
    msleep(10);

    if (*chip).type_ == VX_TYPE_BOARD {
        port = VX_CNTRL as libc::c_uint;
    } else {
        port = VX_GPIOC as libc::c_uint; /* VX222 V2 and VX222_MIC_BOARD with new PLX9030 use this register */
    }

    image = (*xilinx).data;
    i = 0;
    while (i as usize) < (*xilinx).size {
        if put_xilinx_data(chip, port, 8, *image) < 0 {
            return -EINVAL;
        }
        image = image.add(1);
        /* don't take too much time in this loop... */
        cond_resched();
        i += 1;
    }
    put_xilinx_data(chip, port, 4, 0xff); /* end signature */

    msleep(200);

    /* test after loading (is buggy with VX222) */
    if (*chip).type_ != VX_TYPE_BOARD {
        /* Test if load successful: test bit 8 of register GPIOC (VX222: use CNTRL) ! */
        i = vx_inl(chip, VX_GPIOC);
        if (i & 0x0100) != 0 {
            return 0;
        }
        dev_err(
            (*(*chip).card).dev,
            c"xilinx test failed after load, GPIOC=0x%x\n".as_ptr(),
            i,
        );
        return -EINVAL;
    }

    0
}

/*
 * load the boot/dsp images
 */
unsafe extern "C" fn vx2_load_dsp(vx: *mut vx_core, index: i32, dsp: *const firmware) -> i32 {
    let mut err: i32;

    match index {
        1 => {
            /* xilinx image */
            err = vx2_load_xilinx_binary(vx, dsp);
            if err < 0 {
                return err;
            }
            err = vx2_test_xilinx(vx);
            if err < 0 {
                return err;
            }
            0
        }
        2 => {
            /* DSP boot */
            snd_vx_dsp_boot(vx, dsp)
        }
        3 => {
            /* DSP image */
            snd_vx_dsp_load(vx, dsp)
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
unsafe extern "C" fn vx2_test_and_ack(chip: *mut vx_core) -> i32 {
    /* not booted yet? */
    if ((*chip).chip_status & VX_STAT_XILINX_LOADED) == 0 {
        return -ENXIO;
    }

    if (vx_inl(chip, VX_STATUS) & VX_STATUS_MEMIRQ_MASK) == 0 {
        return -EIO;
    }

    /* ok, interrupts generated, now ack it */
    /* set ACQUIT bit up and down */
    vx_outl(chip, VX_STATUS, 0);
    /* useless read just to spend some time and maintain
     * the ACQUIT signal up for a while ( a bus cycle )
     */
    vx_inl(chip, VX_STATUS);
    /* ack */
    vx_outl(chip, VX_STATUS, VX_STATUS_MEMIRQ_MASK);
    /* useless read just to spend some time and maintain
     * the ACQUIT signal up for a while ( a bus cycle ) */
    vx_inl(chip, VX_STATUS);
    /* clear */
    vx_outl(chip, VX_STATUS, 0);

    0
}

/*
 * vx_validate_irq - enable/disable IRQ
 */
unsafe extern "C" fn vx2_validate_irq(_chip: *mut vx_core, enable: i32) {
    let chip = to_vx222(_chip);

    /* Set the interrupt enable bit to 1 in CDSP register */
    if enable != 0 {
        /* Set the PCI interrupt enable bit to 1.*/
        vx_outl(chip as *mut vx_core, VX_INTCSR, VX_INTCSR_VALUE | VX_PCI_INTERRUPT_MASK);
        (*chip).regCDSP |= VX_CDSP_VALID_IRQ_MASK;
    } else {
        /* Set the PCI interrupt enable bit to 0. */
        vx_outl(chip as *mut vx_core, VX_INTCSR, VX_INTCSR_VALUE & !VX_PCI_INTERRUPT_MASK);
        (*chip).regCDSP &= !VX_CDSP_VALID_IRQ_MASK;
    }
    vx_outl(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP);
}

/*
 * write an AKM codec data (24bit)
 */
unsafe fn vx2_write_codec_reg(chip: *mut vx_core, mut data: libc::c_uint) {
    let mut i: libc::c_uint;

    vx_inl(chip, VX_HIFREQ);

    /* We have to send 24 bits (3 x 8 bits). Start with most signif. Bit */
    i = 0;
    while i < 24 {
        vx_outl(chip, VX_DATA, if (data & 0x800000) != 0 { VX_DATA_CODEC_MASK } else { 0 });
        data <<= 1;
        i += 1;
    }
    /* Terminate access to codec registers */
    vx_inl(chip, VX_RUER);
}

const AKM_CODEC_POWER_CONTROL_CMD: libc::c_uint = 0xA007;
const AKM_CODEC_RESET_ON_CMD: libc::c_uint = 0xA100;
const AKM_CODEC_RESET_OFF_CMD: libc::c_uint = 0xA103;
const AKM_CODEC_CLOCK_FORMAT_CMD: libc::c_uint = 0xA240;
const AKM_CODEC_MUTE_CMD: libc::c_uint = 0xA38D;
const AKM_CODEC_UNMUTE_CMD: libc::c_uint = 0xA30D;
const AKM_CODEC_LEFT_LEVEL_CMD: libc::c_uint = 0xA400;
const AKM_CODEC_RIGHT_LEVEL_CMD: libc::c_uint = 0xA500;

static VX2_AKM_GAINS_LUT: [u8; VX2_AKM_LEVEL_MAX as usize + 1] = [
    0x7f, 0x7d, 0x7c, 0x7a, 0x79, 0x77, 0x76, 0x75, 0x73, 0x72, 0x71, 0x70, 0x6f, 0x6d, 0x6c,
    0x6a, 0x69, 0x67, 0x66, 0x65, 0x64, 0x62, 0x61, 0x60, 0x5f, 0x5e, 0x5c, 0x5b, 0x59, 0x58,
    0x56, 0x55, 0x54, 0x53, 0x52, 0x51, 0x50, 0x4e, 0x4d, 0x4b, 0x4a, 0x48, 0x47, 0x46, 0x44,
    0x43, 0x42, 0x41, 0x40, 0x3f, 0x3e, 0x3c, 0x3b, 0x39, 0x38, 0x37, 0x36, 0x34, 0x33, 0x32,
    0x31, 0x31, 0x30, 0x2e, 0x2d, 0x2b, 0x2a, 0x29, 0x28, 0x27, 0x25, 0x24, 0x24, 0x23, 0x22,
    0x21, 0x20, 0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x14,
    0x13, 0x12, 0x12, 0x11, 0x11, 0x10, 0x10, 0x0f, 0x0e, 0x0d, 0x0d, 0x0c, 0x0b, 0x0b, 0x0a,
    0x0a, 0x09, 0x09, 0x08, 0x08, 0x07, 0x07, 0x06, 0x06, 0x06, 0x05, 0x05, 0x05, 0x05, 0x04,
    0x04, 0x04, 0x04, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
    0x02, 0x02, 0x02, 0x02, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00,
]; // [147] = -73.500 dB  ->  AKM(0x00) =  mute       error(+infini)

/*
 * pseudo-codec write entry
 */
unsafe extern "C" fn vx2_write_akm(chip: *mut vx_core, reg: i32, data: libc::c_uint) {
    let mut val: libc::c_uint;

    if reg == XX_CODEC_DAC_CONTROL_REGISTER {
        vx2_write_codec_reg(chip, if data != 0 { AKM_CODEC_MUTE_CMD } else { AKM_CODEC_UNMUTE_CMD });
        return;
    }

    /* `data' is a value between 0x0 and VX2_AKM_LEVEL_MAX = 0x093, in the case of the AKM codecs, we need
       a look up table, as there is no linear matching between the driver codec values
       and the real dBu value
    */
    if snd_BUG_ON(data as usize >= VX2_AKM_GAINS_LUT.len()) != 0 {
        return;
    }

    match reg {
        XX_CODEC_LEVEL_LEFT_REGISTER => {
            val = AKM_CODEC_LEFT_LEVEL_CMD;
        }
        XX_CODEC_LEVEL_RIGHT_REGISTER => {
            val = AKM_CODEC_RIGHT_LEVEL_CMD;
        }
        _ => {
            snd_BUG();
            return;
        }
    }
    val |= VX2_AKM_GAINS_LUT[data as usize] as libc::c_uint;

    vx2_write_codec_reg(chip, val);
}

/*
 * write codec bit for old VX222 board
 */
unsafe extern "C" fn vx2_old_write_codec_bit(
    chip: *mut vx_core,
    _codec: i32,
    mut data: libc::c_uint,
) {
    let mut i: i32;

    /* activate access to codec registers */
    vx_inl(chip, VX_HIFREQ);

    i = 0;
    while i < 24 {
        vx_outl(chip, VX_DATA, if (data & 0x800000) != 0 { VX_DATA_CODEC_MASK } else { 0 });
        data <<= 1;
        i += 1;
    }

    /* Terminate access to codec registers */
    vx_inl(chip, VX_RUER);
}

/*
 * reset codec bit
 */
unsafe extern "C" fn vx2_reset_codec(_chip: *mut vx_core) {
    let chip = to_vx222(_chip);

    /* Set the reset CODEC bit to 0. */
    vx_outl(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP & !VX_CDSP_CODEC_RESET_MASK);
    vx_inl(chip as *mut vx_core, VX_CDSP);
    msleep(10);
    /* Set the reset CODEC bit to 1. */
    (*chip).regCDSP |= VX_CDSP_CODEC_RESET_MASK;
    vx_outl(chip as *mut vx_core, VX_CDSP, (*chip).regCDSP);
    vx_inl(chip as *mut vx_core, VX_CDSP);
    if (*_chip).type_ == VX_TYPE_BOARD {
        msleep(1);
        return;
    }

    msleep(5); /* additionnel wait time for AKM's */

    vx2_write_codec_reg(_chip, AKM_CODEC_POWER_CONTROL_CMD); /* DAC power up, ADC power up, Vref power down */

    vx2_write_codec_reg(_chip, AKM_CODEC_CLOCK_FORMAT_CMD); /* default */
    vx2_write_codec_reg(_chip, AKM_CODEC_MUTE_CMD); /* Mute = ON ,Deemphasis = OFF */
    vx2_write_codec_reg(_chip, AKM_CODEC_RESET_OFF_CMD); /* DAC and ADC normal operation */

    if (*_chip).type_ == VX_TYPE_MIC {
        /* set up the micro input selector */
        (*chip).regSELMIC = MICRO_SELECT_INPUT_NORM
            | MICRO_SELECT_PREAMPLI_G_0
            | MICRO_SELECT_NOISE_T_52DB;

        /* reset phantom power supply */
        (*chip).regSELMIC &= !MICRO_SELECT_PHANTOM_ALIM;

        vx_outl(_chip, VX_SELMIC, (*chip).regSELMIC);
    }
}

/*
 * change the audio source
 */
unsafe extern "C" fn vx2_change_audio_source(_chip: *mut vx_core, src: i32) {
    let chip = to_vx222(_chip);

    match src {
        VX_AUDIO_SRC_DIGITAL => {
            (*chip).regCFG |= VX_CFG_DATAIN_SEL_MASK;
        }
        _ => {
            (*chip).regCFG &= !VX_CFG_DATAIN_SEL_MASK;
        }
    }
    vx_outl(chip as *mut vx_core, VX_CFG, (*chip).regCFG);
}

/*
 * set the clock source
 */
unsafe extern "C" fn vx2_set_clock_source(_chip: *mut vx_core, source: i32) {
    let chip = to_vx222(_chip);

    if source == INTERNAL_QUARTZ {
        (*chip).regCFG &= !VX_CFG_CLOCKIN_SEL_MASK;
    } else {
        (*chip).regCFG |= VX_CFG_CLOCKIN_SEL_MASK;
    }
    vx_outl(chip as *mut vx_core, VX_CFG, (*chip).regCFG);
}

/*
 * reset the board
 */
unsafe extern "C" fn vx2_reset_board(_chip: *mut vx_core, _cold_reset: i32) {
    let chip = to_vx222(_chip);

    /* initialize the register values */
    (*chip).regCDSP = VX_CDSP_CODEC_RESET_MASK | VX_CDSP_DSP_RESET_MASK;
    (*chip).regCFG = 0;
}

/*
 * input level controls for VX222 Mic
 */

/* Micro level is specified to be adjustable from -96dB to 63 dB (board coded 0x00 ... 318),
 * 318 = 210 + 36 + 36 + 36   (210 = +9dB variable) (3 * 36 = 3 steps of 18dB pre ampli)
 * as we will mute if less than -110dB, so let's simply use line input coded levels and add constant offset !
 */
const V2_MICRO_LEVEL_RANGE: i32 = 318 - 255;

unsafe fn vx2_set_input_level(chip: *mut snd_vx222) {
    let mut i: i32;
    let mut miclevel: i32;
    let mut preamp: i32;
    let mut data: libc::c_uint;

    miclevel = (*chip).mic_level;
    miclevel += V2_MICRO_LEVEL_RANGE; /* add 318 - 0xff */
    preamp = 0;
    while miclevel > 210 {
        /* limitation to +9dB of 3310 real gain */
        preamp += 1; /* raise pre ampli + 18dB */
        miclevel -= 18 * 2; /* lower level 18 dB (*2 because of 0.5 dB steps !) */
    }
    if snd_BUG_ON(preamp >= 4) != 0 {
        return;
    }

    /* set pre-amp level */
    (*chip).regSELMIC &= !MICRO_SELECT_PREAMPLI_MASK;
    (*chip).regSELMIC |= ((preamp as libc::c_uint) << MICRO_SELECT_PREAMPLI_OFFSET)
        & MICRO_SELECT_PREAMPLI_MASK;
    vx_outl(chip as *mut vx_core, VX_SELMIC, (*chip).regSELMIC);

    data = ((miclevel as libc::c_uint) << 16)
        | (((*chip).input_level[1] as libc::c_uint) << 8)
        | ((*chip).input_level[0] as libc::c_uint);
    vx_inl(chip as *mut vx_core, VX_DATA); /* Activate input level programming */

    /* We have to send 32 bits (4 x 8 bits) */
    i = 0;
    while i < 32 {
        vx_outl(
            chip as *mut vx_core,
            VX_DATA,
            if (data & 0x80000000) != 0 { VX_DATA_CODEC_MASK } else { 0 },
        );
        data <<= 1;
        i += 1;
    }

    vx_inl(chip as *mut vx_core, VX_RUER); /* Terminate input level programming */
}

const MIC_LEVEL_MAX: i32 = 0xff;

static DB_SCALE_MIC: [libc::c_uint; 4] = TLV_DB_SCALE_ITEM(-6450, 50, 0);

/*
 * controls API for input levels
 */

/* input levels */
unsafe extern "C" fn vx_input_level_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = MIC_LEVEL_MAX as libc::c_long;
    0
}

unsafe extern "C" fn vx_input_level_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let _chip: *mut vx_core = snd_kcontrol_chip(kcontrol);
    let chip = to_vx222(_chip);

    let _guard = mutex_guard(&mut (*_chip).mixer_mutex);
    (*ucontrol).value.integer.value[0] = (*chip).input_level[0] as libc::c_long;
    (*ucontrol).value.integer.value[1] = (*chip).input_level[1] as libc::c_long;
    0
}

unsafe extern "C" fn vx_input_level_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let _chip: *mut vx_core = snd_kcontrol_chip(kcontrol);
    let chip = to_vx222(_chip);
    if (*ucontrol).value.integer.value[0] < 0
        || (*ucontrol).value.integer.value[0] > MIC_LEVEL_MAX as libc::c_long
    {
        return -EINVAL;
    }
    if (*ucontrol).value.integer.value[1] < 0
        || (*ucontrol).value.integer.value[1] > MIC_LEVEL_MAX as libc::c_long
    {
        return -EINVAL;
    }
    let _guard = mutex_guard(&mut (*_chip).mixer_mutex);
    if (*chip).input_level[0] != (*ucontrol).value.integer.value[0] as i32
        || (*chip).input_level[1] != (*ucontrol).value.integer.value[1] as i32
    {
        (*chip).input_level[0] = (*ucontrol).value.integer.value[0] as i32;
        (*chip).input_level[1] = (*ucontrol).value.integer.value[1] as i32;
        vx2_set_input_level(chip);
        return 1;
    }
    0
}

/* mic level */
unsafe extern "C" fn vx_mic_level_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = MIC_LEVEL_MAX as libc::c_long;
    0
}

unsafe extern "C" fn vx_mic_level_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let _chip: *mut vx_core = snd_kcontrol_chip(kcontrol);
    let chip = to_vx222(_chip);
    (*ucontrol).value.integer.value[0] = (*chip).mic_level as libc::c_long;
    0
}

unsafe extern "C" fn vx_mic_level_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let _chip: *mut vx_core = snd_kcontrol_chip(kcontrol);
    let chip = to_vx222(_chip);
    if (*ucontrol).value.integer.value[0] < 0
        || (*ucontrol).value.integer.value[0] > MIC_LEVEL_MAX as libc::c_long
    {
        return -EINVAL;
    }
    let _guard = mutex_guard(&mut (*_chip).mixer_mutex);
    if (*chip).mic_level != (*ucontrol).value.integer.value[0] as i32 {
        (*chip).mic_level = (*ucontrol).value.integer.value[0] as i32;
        vx2_set_input_level(chip);
        return 1;
    }
    0
}

static VX_CONTROL_INPUT_LEVEL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    name: c"Capture Volume".as_ptr(),
    info: Some(vx_input_level_info),
    get: Some(vx_input_level_get),
    put: Some(vx_input_level_put),
    tlv: snd_kcontrol_new__bindgen_ty_1 {
        p: DB_SCALE_MIC.as_ptr(),
    },
    ..unsafe { core::mem::zeroed() }
};

static VX_CONTROL_MIC_LEVEL: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    name: c"Mic Capture Volume".as_ptr(),
    info: Some(vx_mic_level_info),
    get: Some(vx_mic_level_get),
    put: Some(vx_mic_level_put),
    tlv: snd_kcontrol_new__bindgen_ty_1 {
        p: DB_SCALE_MIC.as_ptr(),
    },
    ..unsafe { core::mem::zeroed() }
};

/*
 * FIXME: compressor/limiter implementation is missing yet...
 */

unsafe extern "C" fn vx2_add_mic_controls(_chip: *mut vx_core) -> i32 {
    let chip = to_vx222(_chip);
    let mut err: i32;

    if (*_chip).type_ != VX_TYPE_MIC {
        return 0;
    }

    /* mute input levels */
    (*chip).input_level[1] = 0;
    (*chip).input_level[0] = (*chip).input_level[1];
    (*chip).mic_level = 0;
    vx2_set_input_level(chip);

    /* controls */
    err = snd_ctl_add((*_chip).card, snd_ctl_new1(&VX_CONTROL_INPUT_LEVEL, chip as *mut libc::c_void));
    if err < 0 {
        return err;
    }
    err = snd_ctl_add((*_chip).card, snd_ctl_new1(&VX_CONTROL_MIC_LEVEL, chip as *mut libc::c_void));
    if err < 0 {
        return err;
    }

    0
}

/*
 * callbacks
 */
#[no_mangle]
pub static VX222_OPS: snd_vx_ops = snd_vx_ops {
    in8: Some(vx2_inb),
    in32: Some(vx2_inl),
    out8: Some(vx2_outb),
    out32: Some(vx2_outl),
    test_and_ack: Some(vx2_test_and_ack),
    validate_irq: Some(vx2_validate_irq),
    akm_write: Some(vx2_write_akm),
    reset_codec: Some(vx2_reset_codec),
    change_audio_source: Some(vx2_change_audio_source),
    set_clock_source: Some(vx2_set_clock_source),
    load_dsp: Some(vx2_load_dsp),
    reset_dsp: Some(vx2_reset_dsp),
    reset_board: Some(vx2_reset_board),
    dma_write: Some(vx2_dma_write),
    dma_read: Some(vx2_dma_read),
    add_controls: Some(vx2_add_mic_controls),
    ..unsafe { core::mem::zeroed() }
};

/* for old VX222 board */
#[no_mangle]
pub static VX222_OLD_OPS: snd_vx_ops = snd_vx_ops {
    in8: Some(vx2_inb),
    in32: Some(vx2_inl),
    out8: Some(vx2_outb),
    out32: Some(vx2_outl),
    test_and_ack: Some(vx2_test_and_ack),
    validate_irq: Some(vx2_validate_irq),
    write_codec: Some(vx2_old_write_codec_bit),
    reset_codec: Some(vx2_reset_codec),
    change_audio_source: Some(vx2_change_audio_source),
    set_clock_source: Some(vx2_set_clock_source),
    load_dsp: Some(vx2_load_dsp),
    reset_dsp: Some(vx2_reset_dsp),
    reset_board: Some(vx2_reset_board),
    dma_write: Some(vx2_dma_write),
    dma_read: Some(vx2_dma_read),
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
