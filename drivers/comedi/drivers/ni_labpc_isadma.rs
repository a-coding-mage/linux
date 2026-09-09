// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/ni_labpc_isadma.c
 * ISA DMA support for National Instruments Lab-PC series boards and
 * compatibles.
 *
 * Extracted from ni_labpc.c:
 * Copyright (C) 2001-2003 Frank Mori Hess <fmhess@users.sourceforge.net>
 */

// Linux and local header dependencies are supplied by the surrounding crate.

/* size in bytes of dma buffer */
pub const LABPC_ISADMA_BUFFER_SIZE: ::core::ffi::c_uint = 0xff00;

/* utility function that suggests a dma transfer size in bytes */
unsafe fn labpc_suggest_transfer_size(
    _dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    maxbytes: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let cmd = &mut (*(*s).async_).cmd;
    let sample_size = comedi_bytes_per_sample(s);
    let mut size: ::core::ffi::c_uint;
    let freq: ::core::ffi::c_uint;

    if cmd.convert_src == TRIG_TIMER {
        freq = 1000000000 / cmd.convert_arg;
    } else {
        /* return some default value */
        freq = 0xffffffff;
    }

    /* make buffer fill in no more than 1/3 second */
    size = (freq / 3) * sample_size;

    /* set a minimum and maximum size allowed */
    if size > maxbytes {
        size = maxbytes;
    } else if size < sample_size {
        size = sample_size;
    }

    size
}

pub unsafe fn labpc_setup_dma(dev: *mut comedi_device, s: *mut comedi_subdevice) {
    let devpriv = (*dev).private as *mut labpc_private;
    let desc = &mut (*(*devpriv).dma).desc[0];
    let cmd = &mut (*(*s).async_).cmd;
    let sample_size = comedi_bytes_per_sample(s);

    /* set appropriate size of transfer */
    desc.size = labpc_suggest_transfer_size(dev, s, desc.maxsize);
    if cmd.stop_src == TRIG_COUNT
        && (*devpriv).count * sample_size < desc.size
    {
        desc.size = (*devpriv).count * sample_size;
    }

    comedi_isadma_program(desc);

    /* set CMD3 bits for caller to enable DMA and interrupt */
    (*devpriv).cmd3 |= CMD3_DMAEN | CMD3_DMATCINTEN;
}

pub unsafe fn labpc_drain_dma(dev: *mut comedi_device) {
    let devpriv = (*dev).private as *mut labpc_private;
    let desc = &mut (*(*devpriv).dma).desc[0];
    let s = (*dev).read_subdev;
    let async_ = (*s).async_;
    let cmd = &mut (*async_).cmd;
    let max_samples = comedi_bytes_to_samples(s, desc.size);
    let residue: ::core::ffi::c_uint;
    let mut nsamples: ::core::ffi::c_uint;
    let mut leftover: ::core::ffi::c_uint;

    /*
     * residue is the number of bytes left to be done on the dma
     * transfer.  It should always be zero at this point unless
     * the stop_src is set to external triggering.
     */
    residue = comedi_isadma_disable(desc.chan);

    /*
     * Figure out how many samples to read for this transfer and
     * how many will be stored for next time.
     */
    nsamples = max_samples - comedi_bytes_to_samples(s, residue);
    if cmd.stop_src == TRIG_COUNT {
        if (*devpriv).count <= nsamples {
            nsamples = (*devpriv).count;
            leftover = 0;
        } else {
            leftover = (*devpriv).count - nsamples;
            if leftover > max_samples {
                leftover = max_samples;
            }
        }
        (*devpriv).count -= nsamples;
    } else {
        leftover = max_samples;
    }
    desc.size = comedi_samples_to_bytes(s, leftover);

    comedi_buf_write_samples(s, desc.virt_addr, nsamples);
}

unsafe fn handle_isa_dma(dev: *mut comedi_device) {
    let devpriv = (*dev).private as *mut labpc_private;
    let desc = &mut (*(*devpriv).dma).desc[0];

    labpc_drain_dma(dev);

    if desc.size != 0 {
        comedi_isadma_program(desc);
    }

    /* clear dma tc interrupt */
    ((*devpriv).write_byte)(dev, 0x1, DMATC_CLEAR_REG);
}

pub unsafe fn labpc_handle_dma_status(dev: *mut comedi_device) {
    let board = (*dev).board_ptr as *const labpc_boardinfo;
    let devpriv = (*dev).private as *mut labpc_private;

    /*
     * if a dma terminal count of external stop trigger
     * has occurred
     */
    if (*devpriv).stat1 & STAT1_GATA0 != 0
        || ((*board).is_labpc1200 && (*devpriv).stat2 & STAT2_OUTA1 != 0)
    {
        handle_isa_dma(dev);
    }
}

pub unsafe fn labpc_init_dma_chan(dev: *mut comedi_device, dma_chan: ::core::ffi::c_uint) {
    let devpriv = (*dev).private as *mut labpc_private;

    /* only DMA channels 3 and 1 are valid */
    if dma_chan != 1 && dma_chan != 3 {
        return;
    }

    /* DMA uses 1 buffer */
    (*devpriv).dma = comedi_isadma_alloc(
        dev,
        1,
        dma_chan,
        dma_chan,
        LABPC_ISADMA_BUFFER_SIZE,
        COMEDI_ISADMA_READ,
    );
}

pub unsafe fn labpc_free_dma_chan(dev: *mut comedi_device) {
    let devpriv = (*dev).private as *mut labpc_private;

    if !devpriv.is_null() {
        comedi_isadma_free((*devpriv).dma);
    }
}

// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi NI Lab-PC ISA DMA support");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
