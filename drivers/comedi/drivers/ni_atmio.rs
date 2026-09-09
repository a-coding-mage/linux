// SPDX-License-Identifier: GPL-2.0+
/*
 * Comedi driver for NI AT-MIO E series cards
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 1997-2001 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: ni_atmio
 * Description: National Instruments AT-MIO-E series
 * Author: ds
 * Devices: [National Instruments] AT-MIO-16E-1 (ni_atmio),
 *   AT-MIO-16E-2, AT-MIO-16E-10, AT-MIO-16DE-10, AT-MIO-64E-3,
 *   AT-MIO-16XE-50, AT-MIO-16XE-10, AT-AI-16XE-10
 * Status: works
 * Updated: Thu May  1 20:03:02 CDT 2003
 */

/* The shared implementation is supplied by ni_mio_common.c in the C build. */

static const range comedi_lrange range_ni_E_ao_ext = comedi_lrange {
    length: 4,
    range: [
        BIP_RANGE(10),
        UNI_RANGE(10),
        RANGE_ext(-1, 1),
        RANGE_ext(0, 1),
    ],
};

/* AT specific setup */
static const ni_board_struct ni_boards[] = [
    ni_board_struct {
        name: "at-mio-16e-1",
        device_id: 44,
        isapnp_id: 0x0000, // XXX unknown
        n_adchan: 16,
        ai_maxdata: 0x0fff,
        ai_fifo_depth: 8192,
        gainlkup: ai_gain_16,
        ai_speed: 800,
        n_aochan: 2,
        ao_maxdata: 0x0fff,
        ao_fifo_depth: 2048,
        ao_range_table: &range_ni_E_ao_ext,
        ao_speed: 1000,
        caldac: [mb88341],
    },
    ni_board_struct {
        name: "at-mio-16e-2", device_id: 25, isapnp_id: 0x1900,
        n_adchan: 16, ai_maxdata: 0x0fff, ai_fifo_depth: 2048,
        gainlkup: ai_gain_16, ai_speed: 2000, n_aochan: 2,
        ao_maxdata: 0x0fff, ao_fifo_depth: 2048,
        ao_range_table: &range_ni_E_ao_ext, ao_speed: 1000,
        caldac: [mb88341],
    },
    ni_board_struct {
        name: "at-mio-16e-10", device_id: 36, isapnp_id: 0x2400,
        n_adchan: 16, ai_maxdata: 0x0fff, ai_fifo_depth: 512,
        gainlkup: ai_gain_16, ai_speed: 10000, n_aochan: 2,
        ao_maxdata: 0x0fff, ao_fifo_depth: 512,
        ao_range_table: &range_ni_E_ao_ext, ao_speed: 10000,
        caldac: [ad8804_debug],
    },
    ni_board_struct {
        name: "at-mio-16de-10", device_id: 37, isapnp_id: 0x2500,
        n_adchan: 16, ai_maxdata: 0x0fff, ai_fifo_depth: 512,
        gainlkup: ai_gain_16, ai_speed: 10000, n_aochan: 2,
        ao_maxdata: 0x0fff, ao_fifo_depth: 512,
        ao_range_table: &range_ni_E_ao_ext, ao_speed: 10000,
        caldac: [ad8804_debug], has_8255: 1,
    },
    ni_board_struct {
        name: "at-mio-64e-3", device_id: 38, isapnp_id: 0x2600,
        n_adchan: 64, ai_maxdata: 0x0fff, ai_fifo_depth: 2048,
        gainlkup: ai_gain_16, ai_speed: 2000, n_aochan: 2,
        ao_maxdata: 0x0fff, ao_fifo_depth: 2048,
        ao_range_table: &range_ni_E_ao_ext, ao_speed: 1000,
        caldac: [ad8804_debug],
    },
    ni_board_struct {
        name: "at-mio-16xe-50", device_id: 39, isapnp_id: 0x2700,
        n_adchan: 16, ai_maxdata: 0xffff, ai_fifo_depth: 512,
        alwaysdither: 1, gainlkup: ai_gain_8, ai_speed: 50000,
        n_aochan: 2, ao_maxdata: 0x0fff, ao_range_table: &range_bipolar10,
        ao_speed: 50000, caldac: [dac8800, dac8043],
    },
    ni_board_struct {
        name: "at-mio-16xe-10", device_id: 50, isapnp_id: 0x0000, // XXX unknown
        n_adchan: 16, ai_maxdata: 0xffff, ai_fifo_depth: 512,
        alwaysdither: 1, gainlkup: ai_gain_14, ai_speed: 10000,
        n_aochan: 2, ao_maxdata: 0xffff, ao_fifo_depth: 2048,
        ao_range_table: &range_ni_E_ao_ext, ao_speed: 1000,
        caldac: [dac8800, dac8043, ad8522],
    },
    ni_board_struct {
        name: "at-ai-16xe-10", device_id: 51, isapnp_id: 0x0000, // XXX unknown
        n_adchan: 16, ai_maxdata: 0xffff, ai_fifo_depth: 512,
        alwaysdither: 1, gainlkup: ai_gain_14, ai_speed: 10000,
        caldac: [dac8800, dac8043, ad8522],
    },
];

static const ni_irqpin: [i32; 16] = [-1, -1, -1, 0, 1, 2, -1, 3, -1, -1, 4, 5, 6, -1, -1, 7];

static const device_ids: [pnp_device_id; 6] = [
    pnp_device_id { id: "NIC1900" },
    pnp_device_id { id: "NIC2400" },
    pnp_device_id { id: "NIC2500" },
    pnp_device_id { id: "NIC2600" },
    pnp_device_id { id: "NIC2700" },
    pnp_device_id {},
];

unsafe fn ni_isapnp_find_board(dev: *mut *mut pnp_dev) -> i32 {
    let mut isapnp_dev: *mut pnp_dev = core::ptr::null_mut();
    let mut i: usize = 0;
    while i < ni_boards.len() {
        isapnp_dev = pnp_find_dev(core::ptr::null_mut(), ISAPNP_VENDOR('N' as i32, 'I' as i32, 'C' as i32), ISAPNP_FUNCTION(ni_boards[i].isapnp_id), core::ptr::null_mut());
        if isapnp_dev.is_null() || (*isapnp_dev).card.is_null() { i += 1; continue; }
        if pnp_device_attach(isapnp_dev) < 0 { i += 1; continue; }
        if pnp_activate_dev(isapnp_dev) < 0 {
            pnp_device_detach(isapnp_dev);
            return -EAGAIN;
        }
        if !pnp_port_valid(isapnp_dev, 0) || !pnp_irq_valid(isapnp_dev, 0) {
            pnp_device_detach(isapnp_dev);
            return -ENOMEM;
        }
        break;
    }
    if i == ni_boards.len() { return -ENODEV; }
    *dev = isapnp_dev;
    0
}

unsafe fn ni_atmio_probe(dev: *mut comedi_device) -> *const ni_board_struct {
    let device_id = ni_read_eeprom(dev, 511);
    let mut i = 0;
    while i < ni_boards.len() {
        let board = &ni_boards[i];
        if board.device_id == device_id { return board; }
        i += 1;
    }
    if device_id == 255 { dev_err((*dev).class_dev, "can't find board\n"); }
    else if device_id == 0 { dev_err((*dev).class_dev, "EEPROM read error (?) or device not found\n"); }
    else { dev_err((*dev).class_dev, "unknown device ID %d -- contact author\n", device_id); }
    core::ptr::null()
}

unsafe fn ni_atmio_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let mut ret = ni_alloc_private(dev);
    if ret != 0 { return ret; }
    let mut iobase = (*it).options[0] as u64;
    let mut irq = (*it).options[1] as u32;
    let mut isapnp_dev: *mut pnp_dev = core::ptr::null_mut();
    if iobase == 0 {
        ret = ni_isapnp_find_board(&mut isapnp_dev);
        if ret < 0 { return ret; }
        iobase = pnp_port_start(isapnp_dev, 0);
        irq = pnp_irq(isapnp_dev, 0);
        comedi_set_hw_dev(dev, &mut (*isapnp_dev).dev);
    }
    ret = comedi_check_request_region(dev, iobase, 0x20, 0x20, 0xffff, 32);
    if ret != 0 { return ret; }
    let board = ni_atmio_probe(dev);
    if board.is_null() { return -ENODEV; }
    (*dev).board_ptr = board as *mut core::ffi::c_void;
    (*dev).board_name = (*board).name;
    if irq != 0 {
        if irq > 15 || ni_irqpin[irq as usize] == -1 { return -EINVAL; }
        ret = request_irq(irq, ni_E_interrupt, 0, (*dev).board_name, dev);
        if ret < 0 { return -EINVAL; }
        (*dev).irq = irq;
    }
    ret = ni_E_init(dev, ni_irqpin[(*dev).irq as usize], 0);
    if ret < 0 { return ret; }
    0
}

unsafe fn ni_atmio_detach(dev: *mut comedi_device) {
    mio_common_detach(dev);
    comedi_legacy_detach(dev);
    let isapnp_dev = if !(*dev).hw_dev.is_null() { to_pnp_dev((*dev).hw_dev) } else { core::ptr::null_mut() };
    if !isapnp_dev.is_null() { pnp_device_detach(isapnp_dev); }
}

static mut ni_atmio_driver: comedi_driver = comedi_driver {
    driver_name: "ni_atmio",
    module: THIS_MODULE,
    attach: ni_atmio_attach,
    detach: ni_atmio_detach,
};

module_comedi_driver!(ni_atmio_driver);
MODULE_AUTHOR!("Comedi https://www.comedi.org");
MODULE_DESCRIPTION!("Comedi low-level driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
