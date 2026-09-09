// SPDX-License-Identifier: GPL-2.0+
/*
 * Comedi driver for NI PCMCIA MIO E series cards
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-2000 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: ni_mio_cs
 * Description: National Instruments DAQCard E series
 * Author: ds
 * Status: works
 * Devices: [National Instruments] DAQCard-AI-16XE-50 (ni_mio_cs),
 *   DAQCard-AI-16E-4, DAQCard-6062E, DAQCard-6024E, DAQCard-6036E
 * Updated: Thu Oct 23 19:43:17 CDT 2003
 *
 * See the notes in the ni_atmio.o driver.
 */

/* The real guts of the driver is in ni_mio_common.c, included by all E series drivers. */
/* Kernel, Comedi, PCMCIA, and ni_stc declarations are supplied by the surrounding crate. */

/* AT specific setup */
static NI_BOARDS: [ni_board_struct; 5] = [
    ni_board_struct {
        name: c"DAQCard-ai-16xe-50",
        device_id: 0x010d,
        n_adchan: 16,
        ai_maxdata: 0xffff,
        ai_fifo_depth: 1024,
        gainlkup: ai_gain_8,
        ai_speed: 5000,
        caldac: [dac8800, dac8043],
        ..NI_BOARD_STRUCT_ZERO
    },
    ni_board_struct {
        name: c"DAQCard-ai-16e-4",
        device_id: 0x010c,
        n_adchan: 16,
        ai_maxdata: 0x0fff,
        ai_fifo_depth: 1024,
        gainlkup: ai_gain_16,
        ai_speed: 4000,
        caldac: [mb88341, 0], // verified
        ..NI_BOARD_STRUCT_ZERO
    },
    ni_board_struct {
        name: c"DAQCard-6062E",
        device_id: 0x02c4,
        n_adchan: 16,
        ai_maxdata: 0x0fff,
        ai_fifo_depth: 8192,
        gainlkup: ai_gain_16,
        ai_speed: 2000,
        n_aochan: 2,
        ao_maxdata: 0x0fff,
        ao_fifo_depth: 2048,
        ao_range_table: &range_bipolar10,
        ao_speed: 1176,
        caldac: [ad8804_debug, 0], // verified
        ..NI_BOARD_STRUCT_ZERO
    },
    ni_board_struct {
        // specs incorrect!
        name: c"DAQCard-6024E",
        device_id: 0x075e,
        n_adchan: 16,
        ai_maxdata: 0x0fff,
        ai_fifo_depth: 1024,
        gainlkup: ai_gain_4,
        ai_speed: 5000,
        n_aochan: 2,
        ao_maxdata: 0x0fff,
        ao_range_table: &range_bipolar10,
        ao_speed: 1000000,
        caldac: [ad8804_debug, 0],
        ..NI_BOARD_STRUCT_ZERO
    },
    ni_board_struct {
        // specs incorrect!
        name: c"DAQCard-6036E",
        device_id: 0x0245,
        n_adchan: 16,
        ai_maxdata: 0xffff,
        ai_fifo_depth: 1024,
        alwaysdither: 1,
        gainlkup: ai_gain_4,
        ai_speed: 5000,
        n_aochan: 2,
        ao_maxdata: 0xffff,
        ao_range_table: &range_bipolar10,
        ao_speed: 1000000,
        caldac: [ad8804_debug, 0],
        ..NI_BOARD_STRUCT_ZERO
    },
];

// ni_mio_common.c is included here in the original translation unit.

unsafe fn ni_getboardtype(
    _dev: *mut comedi_device,
    link: *mut pcmcia_device,
) -> *const ni_board_struct {
    let mut board: *const ni_board_struct;
    let mut i = 0usize;
    while i < NI_BOARDS.len() {
        board = &NI_BOARDS[i];
        if (*board).device_id == (*link).card_id {
            return board;
        }
        i += 1;
    }
    core::ptr::null()
}

unsafe fn mio_pcmcia_config_loop(
    p_dev: *mut pcmcia_device,
    _priv_data: *mut core::ffi::c_void,
) -> i32 {
    (*(*p_dev).resource[0]).flags &= !IO_DATA_PATH_WIDTH;
    (*(*p_dev).resource[0]).flags |= IO_DATA_PATH_WIDTH_16;

    let mut base = 0x000;
    while base < 0x400 {
        (*(*p_dev).resource[0]).start = base;
        let ret = pcmcia_request_io(p_dev);
        if ret == 0 {
            return 0;
        }
        base += 0x20;
    }
    -ENODEV
}

unsafe fn mio_cs_auto_attach(dev: *mut comedi_device, _context: c_ulong) -> i32 {
    let link = comedi_to_pcmcia_dev(dev);
    let board = ni_getboardtype(dev, link);
    if board.is_null() {
        return -ENODEV;
    }
    (*dev).board_ptr = board as *mut _;
    (*dev).board_name = (*board).name;

    (*link).config_flags |= CONF_AUTO_SET_IO | CONF_ENABLE_IRQ;
    let mut ret = comedi_pcmcia_enable(dev, mio_pcmcia_config_loop);
    if ret != 0 { return ret; }
    (*dev).iobase = (*(*link).resource[0]).start;

    (*link).priv_ = dev as *mut _;
    ret = pcmcia_request_irq(link, ni_E_interrupt);
    if ret != 0 { return ret; }
    (*dev).irq = (*link).irq;

    ret = ni_alloc_private(dev);
    if ret != 0 { return ret; }
    ni_E_init(dev, 0, 1)
}

unsafe fn mio_cs_detach(dev: *mut comedi_device) {
    mio_common_detach(dev);
    comedi_pcmcia_disable(dev);
}

static mut DRIVER_NI_MIO_CS: comedi_driver = comedi_driver {
    driver_name: c"ni_mio_cs",
    module: THIS_MODULE,
    auto_attach: Some(mio_cs_auto_attach),
    detach: Some(mio_cs_detach),
    ..COMEDI_DRIVER_ZERO
};

unsafe fn cs_attach(link: *mut pcmcia_device) -> i32 {
    comedi_pcmcia_auto_config(link, &mut DRIVER_NI_MIO_CS)
}

static NI_MIO_CS_IDS: [pcmcia_device_id; 6] = [
    PCMCIA_DEVICE_MANF_CARD(0x010b, 0x010d), // DAQCard-ai-16xe-50
    PCMCIA_DEVICE_MANF_CARD(0x010b, 0x010c), // DAQCard-ai-16e-4
    PCMCIA_DEVICE_MANF_CARD(0x010b, 0x02c4), // DAQCard-6062E
    PCMCIA_DEVICE_MANF_CARD(0x010b, 0x075e), // DAQCard-6024E
    PCMCIA_DEVICE_MANF_CARD(0x010b, 0x0245), // DAQCard-6036E
    PCMCIA_DEVICE_NULL,
];

static mut NI_MIO_CS_DRIVER: pcmcia_driver = pcmcia_driver {
    name: c"ni_mio_cs",
    owner: THIS_MODULE,
    id_table: NI_MIO_CS_IDS.as_ptr(),
    probe: Some(cs_attach),
    remove: Some(comedi_pcmcia_auto_unconfig),
    ..PCMCIA_DRIVER_ZERO
};

module_comedi_pcmcia_driver!(DRIVER_NI_MIO_CS, NI_MIO_CS_DRIVER);
module_description!("Comedi driver for National Instruments DAQCard E series");
module_author!("David A. Schleef <ds@schleef.org>");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
