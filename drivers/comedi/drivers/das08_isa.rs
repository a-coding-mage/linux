// SPDX-License-Identifier: GPL-2.0+
/*
 *  das08_isa.c
 *  comedi driver for DAS08 ISA/PC-104 boards
 *
 *  COMEDI - Linux Control and Measurement Device Interface
 *  Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 *  Copyright (C) 2001,2002,2003 Frank Mori Hess <fmhess@users.sourceforge.net>
 *  Copyright (C) 2004 Salvador E. Tropea <set@users.sf.net> <set@ieee.org>
 */

/*
 * Driver: das08_isa
 * Description: DAS-08 ISA/PC-104 compatible boards
 * Devices: [Keithley Metrabyte] DAS08 (isa-das08),
 *   [ComputerBoards] DAS08 (isa-das08), DAS08-PGM (das08-pgm),
 *   DAS08-PGH (das08-pgh), DAS08-PGL (das08-pgl), DAS08-AOH (das08-aoh),
 *   DAS08-AOL (das08-aol), DAS08-AOM (das08-aom), DAS08/JR-AO (das08/jr-ao),
 *   DAS08/JR-16-AO (das08jr-16-ao), PC104-DAS08 (pc104-das08),
 *   DAS08/JR/16 (das08jr/16)
 * Author: Warren Jasper, ds, Frank Hess
 * Updated: Fri, 31 Aug 2012 19:19:06 +0100
 * Status: works
 *
 * This is the ISA/PC-104-specific support split off from the das08 driver.
 *
 * Configuration Options:
 *     [0] - base io address
 */

// Linux/comedi dependencies and symbols are supplied by the surrounding translation unit.

static DAS08_ISA_BOARDS: [Das08BoardStruct; 11] = [
    Das08BoardStruct { name: c"isa-das08".as_ptr(), ai_nbits: 12, ai_pg: das08_pg_none, ai_encoding: das08_encode12, di_nchan: 3, do_nchan: 4, i8255_offset: 8, i8254_offset: 0, iosize: 16, ..Das08BoardStruct::ZERO },
    Das08BoardStruct { name: c"das08-pgm".as_ptr(), ai_nbits: 12, ai_pg: das08_pgm, ai_encoding: das08_encode12, di_nchan: 3, do_nchan: 4, i8255_offset: 0, i8254_offset: 0x04, iosize: 16, ..Das08BoardStruct::ZERO },
    Das08BoardStruct { name: c"das08-pgh".as_ptr(), ai_nbits: 12, ai_pg: das08_pgh, ai_encoding: das08_encode12, di_nchan: 3, do_nchan: 4, i8254_offset: 0x04, iosize: 16, ..Das08BoardStruct::ZERO },
    Das08BoardStruct { name: c"das08-pgl".as_ptr(), ai_nbits: 12, ai_pg: das08_pgl, ai_encoding: das08_encode12, di_nchan: 3, do_nchan: 4, i8254_offset: 0x04, iosize: 16, ..Das08BoardStruct::ZERO },
    Das08BoardStruct { name: c"das08-aoh".as_ptr(), ai_nbits: 12, ai_pg: das08_pgh, ai_encoding: das08_encode12, ao_nbits: 12, di_nchan: 3, do_nchan: 4, i8255_offset: 0x0c, i8254_offset: 0x04, iosize: 16, ..Das08BoardStruct::ZERO },
    Das08BoardStruct { name: c"das08-aol".as_ptr(), ai_nbits: 12, ai_pg: das08_pgl, ai_encoding: das08_encode12, ao_nbits: 12, di_nchan: 3, do_nchan: 4, i8255_offset: 0x0c, i8254_offset: 0x04, iosize: 16, ..Das08BoardStruct::ZERO },
    Das08BoardStruct { name: c"das08-aom".as_ptr(), ai_nbits: 12, ai_pg: das08_pgm, ai_encoding: das08_encode12, ao_nbits: 12, di_nchan: 3, do_nchan: 4, i8255_offset: 0x0c, i8254_offset: 0x04, iosize: 16, ..Das08BoardStruct::ZERO },
    Das08BoardStruct { name: c"das08/jr-ao".as_ptr(), is_jr: true, ai_nbits: 12, ai_pg: das08_pg_none, ai_encoding: das08_encode12, ao_nbits: 12, di_nchan: 8, do_nchan: 8, iosize: 16, ..Das08BoardStruct::ZERO },
    Das08BoardStruct { name: c"das08jr-16-ao".as_ptr(), is_jr: true, ai_nbits: 16, ai_pg: das08_pg_none, ai_encoding: das08_encode16, ao_nbits: 16, di_nchan: 8, do_nchan: 8, i8254_offset: 0x04, iosize: 16, ..Das08BoardStruct::ZERO },
    Das08BoardStruct { name: c"pc104-das08".as_ptr(), ai_nbits: 12, ai_pg: das08_pg_none, ai_encoding: das08_encode12, di_nchan: 3, do_nchan: 4, i8254_offset: 4, iosize: 16, ..Das08BoardStruct::ZERO },
    Das08BoardStruct { name: c"das08jr/16".as_ptr(), is_jr: true, ai_nbits: 16, ai_pg: das08_pg_none, ai_encoding: das08_encode16, di_nchan: 8, do_nchan: 8, iosize: 16, ..Das08BoardStruct::ZERO },
];

unsafe extern "C" {
    fn comedi_alloc_devpriv(dev: *mut ComediDevice, size: usize) -> *mut Das08PrivateStruct;
    fn comedi_check_request_region(dev: *mut ComediDevice, from: u32, len: u32, min: u32, max: u32, align: u32) -> i32;
    fn das08_common_attach(dev: *mut ComediDevice, iobase: u32) -> i32;
    static comedi_legacy_detach: unsafe extern "C" fn(*mut ComediDevice) -> i32;
}

unsafe fn das08_isa_attach(dev: *mut ComediDevice, it: *mut ComediDevconfig) -> i32 {
    let board = (*dev).board_ptr as *const Das08BoardStruct;
    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<Das08PrivateStruct>());
    if devpriv.is_null() { return -12; }

    let ret = comedi_check_request_region(dev, (*it).options[0], (*board).iosize, 0, 0x3ff, (*board).iosize);
    if ret != 0 { return ret; }
    das08_common_attach(dev, (*dev).iobase)
}

// Equivalent of the C comedi driver descriptor and module_comedi_driver registration.
static mut DAS08_ISA_DRIVER: ComediDriver = ComediDriver {
    driver_name: c"isa-das08".as_ptr(),
    module: THIS_MODULE,
    attach: Some(das08_isa_attach),
    detach: Some(comedi_legacy_detach),
    board_name: unsafe { DAS08_ISA_BOARDS[0].name },
    num_names: DAS08_ISA_BOARDS.len(),
    offset: core::mem::size_of::<Das08BoardStruct>(),
};

// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi low-level driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
