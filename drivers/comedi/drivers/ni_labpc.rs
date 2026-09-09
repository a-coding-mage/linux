// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/ni_labpc.c
 * Driver for National Instruments Lab-PC series boards and compatibles
 * Copyright (C) 2001-2003 Frank Mori Hess <fmhess@users.sourceforge.net>
 */

/*
 * Driver: ni_labpc
 * Description: National Instruments Lab-PC (& compatibles)
 * Devices: [National Instruments] Lab-PC-1200 (lab-pc-1200),
 *   Lab-PC-1200AI (lab-pc-1200ai), Lab-PC+ (lab-pc+)
 * Author: Frank Mori Hess <fmhess@users.sourceforge.net>
 * Status: works
 *
 * Configuration options - ISA boards:
 *   [0] - I/O port base address
 *   [1] - IRQ (optional, required for timed or externally triggered
 *		conversions)
 *   [2] - DMA channel (optional)
 *
 * Tested with lab-pc-1200.  For the older Lab-PC+, not all input
 * ranges and analog references will work, the available ranges/arefs
 * will depend on how you have configured the jumpers on your board
 * (see your owner's manual).
 *
 * Kernel-level ISA plug-and-play support for the lab-pc-1200 boards
 * has not yet been added to the driver, mainly due to the fact that
 * I don't know the device id numbers. If you have one of these boards,
 * please file a bug report at https://comedi.org/ so I can get the
 * necessary information from you.
 *
 * The 1200 series boards have onboard calibration dacs for correcting
 * analog input/output offsets and gains. The proper settings for these
 * caldacs are stored on the board's eeprom. To read the caldac values
 * from the eeprom and store them into a file that can be then be used
 * by comedilib, use the comedi_calibrate program.
 *
 * The Lab-pc+ has quirky chanlist requirements when scanning multiple
 * channels. Multiple channel scan sequence must start at highest channel,
 * then decrement down to channel 0. The rest of the cards can scan down
 * like lab-pc+ or scan up from channel zero. Chanlists consisting of all
 * one channel are also legal, and allow you to pace conversions in bursts.
 *
 * NI manuals:
 * 341309a (labpc-1200 register manual)
 * 320502b (lab-pc+)
 */

// External dependencies supplied by the surrounding Comedi translation.

extern "C" {
    fn comedi_check_request_region(
        dev: *mut comedi_device,
        from: u32,
        len: u32,
        min: u32,
        max: u32,
        align: u32,
    ) -> i32;
    fn labpc_common_attach(dev: *mut comedi_device, irq: u32, dma: u32) -> i32;
    fn labpc_init_dma_chan(dev: *mut comedi_device, dma_chan: u32);
    fn labpc_free_dma_chan(dev: *mut comedi_device);
    fn labpc_common_detach(dev: *mut comedi_device);
    fn comedi_legacy_detach(dev: *mut comedi_device);
}

#[repr(C)]
pub struct labpc_boardinfo {
    pub name: *const u8,
    pub ai_speed: u32,
    pub ai_scan_up: u32,
    pub has_ao: u32,
    pub is_labpc1200: u32,
}

#[repr(C)]
pub struct comedi_device {
    pub irq: u32,
}

#[repr(C)]
pub struct comedi_devconfig {
    pub options: [u32; 8],
}

#[repr(C)]
pub struct comedi_driver {
    pub driver_name: *const u8,
    pub module: *mut core::ffi::c_void,
    pub attach: Option<unsafe extern "C" fn(*mut comedi_device, *mut comedi_devconfig) -> i32>,
    pub detach: Option<unsafe extern "C" fn(*mut comedi_device)>,
    pub num_names: usize,
    pub board_name: *const *const u8,
    pub offset: usize,
}

static LABPC_BOARDS: [labpc_boardinfo; 3] = [
    labpc_boardinfo {
        name: b"lab-pc-1200\0".as_ptr(),
        ai_speed: 10000,
        ai_scan_up: 1,
        has_ao: 1,
        is_labpc1200: 1,
    },
    labpc_boardinfo {
        name: b"lab-pc-1200ai\0".as_ptr(),
        ai_speed: 10000,
        ai_scan_up: 1,
        has_ao: 0,
        is_labpc1200: 1,
    },
    labpc_boardinfo {
        name: b"lab-pc+\0".as_ptr(),
        ai_speed: 12000,
        ai_scan_up: 0,
        has_ao: 1,
        is_labpc1200: 0,
    },
];

unsafe extern "C" fn labpc_attach(
    dev: *mut comedi_device,
    it: *mut comedi_devconfig,
) -> i32 {
    let irq = (*it).options[1];
    let dma_chan = (*it).options[2];
    let mut ret: i32;

    ret = comedi_check_request_region(dev, (*it).options[0], 0x20, 0, 0x3ff, 32);
    if ret != 0 {
        return ret;
    }

    ret = labpc_common_attach(dev, irq, 0);
    if ret != 0 {
        return ret;
    }

    if (*dev).irq != 0 {
        labpc_init_dma_chan(dev, dma_chan);
    }

    0
}

unsafe extern "C" fn labpc_detach(dev: *mut comedi_device) {
    labpc_free_dma_chan(dev);
    labpc_common_detach(dev);
    comedi_legacy_detach(dev);
}

static mut LABPC_DRIVER: comedi_driver = comedi_driver {
    driver_name: b"ni_labpc\0".as_ptr(),
    module: core::ptr::null_mut(),
    attach: Some(labpc_attach),
    detach: Some(labpc_detach),
    num_names: LABPC_BOARDS.len(),
    board_name: &LABPC_BOARDS[0].name,
    offset: core::mem::size_of::<labpc_boardinfo>(),
};

// module_comedi_driver(labpc_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for NI Lab-PC ISA boards");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
