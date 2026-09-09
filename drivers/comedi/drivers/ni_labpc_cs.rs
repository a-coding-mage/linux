// SPDX-License-Identifier: GPL-2.0+
/*
 * Driver for National Instruments daqcard-1200 boards
 * Copyright (C) 2001, 2002, 2003 Frank Mori Hess <fmhess@users.sourceforge.net>
 *
 * PCMCIA crap is adapted from dummy_cs.c 1.31 2001/08/24 12:13:13
 * from the pcmcia package.
 * The initial developer of the pcmcia dummy_cs.c code is David A. Hinds
 * <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
 * are Copyright (C) 1999 David A. Hinds.
 */

/*
 * Driver: ni_labpc_cs
 * Description: National Instruments Lab-PC (& compatibles)
 * Author: Frank Mori Hess <fmhess@users.sourceforge.net>
 * Devices: [National Instruments] DAQCard-1200 (daqcard-1200)
 * Status: works
 *
 * Thanks go to Fredrik Lingvall for much testing and perseverance in
 * helping to debug daqcard-1200 support.
 *
 * The 1200 series boards have onboard calibration dacs for correcting
 * analog input/output offsets and gains. The proper settings for these
 * caldacs are stored on the board's eeprom. To read the caldac values
 * from the eeprom and store them into a file that can be then be used by
 * comedilib, use the comedi_calibrate program.
 *
 * Configuration options: none
 *
 * The daqcard-1200 has quirky chanlist requirements when scanning multiple
 * channels. Multiple channel scan sequence must start at highest channel,
 * then decrement down to channel 0. Chanlists consisting of all one channel
 * are also legal, and allow you to pace conversions in bursts.
 *
 * NI manuals:
 *   340988a (daqcard-1200)
 */

// External Linux, Comedi, PCMCIA, and ni_labpc declarations are supplied by
// the surrounding translation unit/dependencies.

static LABPC_CS_BOARDS: [labpc_boardinfo; 1] = [labpc_boardinfo {
    name: "daqcard-1200",
    ai_speed: 10000,
    has_ao: 1,
    is_labpc1200: 1,
}];

unsafe extern "C" {
    fn comedi_to_pcmcia_dev(dev: *mut comedi_device) -> *mut pcmcia_device;
    fn comedi_pcmcia_enable(dev: *mut comedi_device, data: *mut core::ffi::c_void) -> i32;
    fn labpc_common_attach(dev: *mut comedi_device, irq: u32, flags: u32) -> i32;
    fn labpc_common_detach(dev: *mut comedi_device);
    fn comedi_pcmcia_disable(dev: *mut comedi_device);
    fn comedi_pcmcia_auto_config(link: *mut pcmcia_device, driver: *mut comedi_driver) -> i32;
    fn comedi_pcmcia_auto_unconfig(link: *mut pcmcia_device);
}

unsafe extern "C" fn labpc_cs_auto_attach(
    dev: *mut comedi_device,
    _context: usize,
) -> i32 {
    let link: *mut pcmcia_device = comedi_to_pcmcia_dev(dev);
    let ret: i32;

    /* The ni_labpc driver needs the board_ptr */
    (*dev).board_ptr = &LABPC_CS_BOARDS[0] as *const labpc_boardinfo as *mut _;

    (*link).config_flags |= CONF_AUTO_SET_IO | CONF_ENABLE_IRQ | CONF_ENABLE_PULSE_IRQ;
    ret = comedi_pcmcia_enable(dev, core::ptr::null_mut());
    if ret != 0 {
        return ret;
    }
    (*dev).iobase = (*(*link).resource.add(0)).start;

    if (*link).irq == 0 {
        return -EINVAL;
    }

    labpc_common_attach(dev, (*link).irq, IRQF_SHARED)
}

unsafe extern "C" fn labpc_cs_detach(dev: *mut comedi_device) {
    labpc_common_detach(dev);
    comedi_pcmcia_disable(dev);
}

static mut driver_labpc_cs: comedi_driver = comedi_driver {
    driver_name: "ni_labpc_cs",
    module: THIS_MODULE,
    auto_attach: Some(labpc_cs_auto_attach),
    detach: Some(labpc_cs_detach),
};

unsafe extern "C" fn labpc_cs_attach(link: *mut pcmcia_device) -> i32 {
    comedi_pcmcia_auto_config(link, &raw mut driver_labpc_cs)
}

static labpc_cs_ids: [pcmcia_device_id; 2] = [
    pcmcia_device_id {
        match_flags: PCMCIA_DEV_ID_MATCH_MANF_ID | PCMCIA_DEV_ID_MATCH_CARD_ID,
        manf_id: 0x010b,
        card_id: 0x0103,
        ..pcmcia_device_id::default()
    },
    pcmcia_device_id::null(),
];

static mut labpc_cs_driver: pcmcia_driver = pcmcia_driver {
    name: "daqcard-1200",
    owner: THIS_MODULE,
    id_table: labpc_cs_ids.as_ptr(),
    probe: Some(labpc_cs_attach),
    remove: Some(comedi_pcmcia_auto_unconfig),
};

// MODULE_DEVICE_TABLE(pcmcia, labpc_cs_ids);
// module_comedi_pcmcia_driver(driver_labpc_cs, labpc_cs_driver);
// MODULE_DESCRIPTION("Comedi driver for National Instruments Lab-PC");
// MODULE_AUTHOR("Frank Mori Hess <fmhess@users.sourceforge.net>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
