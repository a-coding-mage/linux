// SPDX-License-Identifier: GPL-2.0+
/*
 * Comedi driver for DAS008 PCMCIA boards
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 * Copyright (C) 2001,2002,2003 Frank Mori Hess <fmhess@users.sourceforge.net>
 *
 * PCMCIA support code for this driver is adapted from the dummy_cs.c
 * driver of the Linux PCMCIA Card Services package.
 *
 * The initial developer of the original code is David A. Hinds
 * <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
 * are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
 */

/*
 * Driver: das08_cs
 * Description: DAS-08 PCMCIA boards
 * Author: Warren Jasper, ds, Frank Hess
 * Devices: [ComputerBoards] PCM-DAS08 (pcm-das08)
 * Status: works
 *
 * This is the PCMCIA-specific support split off from the
 * das08 driver.
 *
 * Configuration Options: none, uses PCMCIA auto config
 *
 * Command support does not exist, but could be added for this board.
 */

// Dependencies supplied by the surrounding kernel/comedi translation unit.

static const das08_board_struct das08_cs_boards: [das08_board_struct; 1] = [
    das08_board_struct {
        name: b"pcm-das08\0".as_ptr() as *const _,
        ai_nbits: 12,
        ai_pg: das08_bipolar5,
        ai_encoding: das08_pcm_encode12,
        di_nchan: 3,
        do_nchan: 3,
        iosize: 16,
    },
];

unsafe extern "C" fn das08_cs_auto_attach(
    dev: *mut comedi_device,
    _context: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let link: *mut pcmcia_device = comedi_to_pcmcia_dev(dev);
    let mut devpriv: *mut das08_private_struct;
    let mut iobase: ::core::ffi::c_ulong;
    let ret: ::core::ffi::c_int;

    /* The das08 driver needs the board_ptr */
    (*dev).board_ptr = &das08_cs_boards[0] as *const _ as *mut _;

    (*link).config_flags |= CONF_AUTO_SET_IO;
    ret = comedi_pcmcia_enable(dev, core::ptr::null_mut());
    if ret != 0 {
        return ret;
    }
    iobase = (*(*link).resource[0]).start;

    devpriv = comedi_alloc_devpriv(
        dev,
        core::mem::size_of::<das08_private_struct>(),
    ) as *mut das08_private_struct;
    if devpriv.is_null() {
        return -12;
    }

    das08_common_attach(dev, iobase)
}

static mut driver_das08_cs: comedi_driver = comedi_driver {
    driver_name: b"das08_cs\0".as_ptr() as *const _,
    module: THIS_MODULE,
    auto_attach: Some(das08_cs_auto_attach),
    detach: Some(comedi_pcmcia_disable),
};

unsafe extern "C" fn das08_pcmcia_attach(
    link: *mut pcmcia_device,
) -> ::core::ffi::c_int {
    comedi_pcmcia_auto_config(link, &mut driver_das08_cs)
}

static das08_cs_id_table: [pcmcia_device_id; 2] = [
    PCMCIA_DEVICE_MANF_CARD!(0x01c5, 0x4001),
    PCMCIA_DEVICE_NULL!(),
];

static mut das08_cs_driver: pcmcia_driver = pcmcia_driver {
    name: b"pcm-das08\0".as_ptr() as *const _,
    owner: THIS_MODULE,
    id_table: das08_cs_id_table.as_ptr(),
    probe: Some(das08_pcmcia_attach),
    remove: Some(comedi_pcmcia_auto_unconfig),
};

// module_comedi_pcmcia_driver(driver_das08_cs, das08_cs_driver);
// MODULE_DEVICE_TABLE(pcmcia, das08_cs_id_table);
// MODULE_AUTHOR("David A. Schleef <ds@schleef.org>");
// MODULE_AUTHOR("Frank Mori Hess <fmhess@users.sourceforge.net>");
// MODULE_DESCRIPTION("Comedi driver for ComputerBoards DAS-08 PCMCIA boards");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
