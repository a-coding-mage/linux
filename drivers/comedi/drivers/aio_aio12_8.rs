// SPDX-License-Identifier: GPL-2.0+
/*
 * aio_aio12_8.c
 * Driver for ACCES I/O Products PC-104 AIO12-8 Analog I/O Board
 * Copyright (C) 2006 C&C Technologies, Inc.
 */

/*
 * Driver: aio_aio12_8
 * Description: ACCES I/O Products PC-104 AIO12-8 Analog I/O Board
 * Author: Pablo Mejia <pablo.mejia@cctechnol.com>
 * Devices: [ACCES I/O] PC-104 AIO12-8 (aio_aio12_8),
 *   [ACCES I/O] PC-104 AI12-8 (aio_ai12_8),
 *   [ACCES I/O] PC-104 AO12-4 (aio_ao12_4)
 * Status: experimental
 *
 * Configuration Options:
 *   [0] - I/O port base address
 *
 * Notes:
 * Only synchronous operations are supported.
 */

// Linux/Comedi dependencies supplied by the surrounding repository.

const AIO12_8_STATUS_REG: u8 = 0x00;
const AIO12_8_STATUS_ADC_EOC: u8 = 1 << 7;
const AIO12_8_STATUS_PORT_C_COS: u8 = 1 << 6;
const AIO12_8_STATUS_IRQ_ENA: u8 = 1 << 2;
const AIO12_8_INTERRUPT_REG: u8 = 0x01;
const AIO12_8_INTERRUPT_ADC: u8 = 1 << 7;
const AIO12_8_INTERRUPT_COS: u8 = 1 << 6;
const AIO12_8_INTERRUPT_COUNTER1: u8 = 1 << 5;
const AIO12_8_INTERRUPT_PORT_C3: u8 = 1 << 4;
const AIO12_8_INTERRUPT_PORT_C0: u8 = 1 << 3;
const AIO12_8_INTERRUPT_ENA: u8 = 1 << 2;
const AIO12_8_ADC_REG: u8 = 0x02;
const AIO12_8_ADC_MODE_NORMAL: u8 = (0 & 0x3) << 6;
const AIO12_8_ADC_MODE_INT_CLK: u8 = (1 & 0x3) << 6;
const AIO12_8_ADC_MODE_STANDBY: u8 = (2 & 0x3) << 6;
const AIO12_8_ADC_MODE_POWERDOWN: u8 = (3 & 0x3) << 6;
const AIO12_8_ADC_ACQ_3USEC: u8 = (0 & 0x1) << 5;
const AIO12_8_ADC_ACQ_PROGRAM: u8 = (1 & 0x1) << 5;
const AIO12_8_DAC_REG_BASE: u8 = 0x04;
const AIO12_8_8254_BASE_REG: u8 = 0x0c;
const AIO12_8_8255_BASE_REG: u8 = 0x10;
const AIO12_8_DIO_CONTROL_REG: u8 = 0x14;
const AIO12_8_DIO_CONTROL_TST: u8 = 1 << 0;
const AIO12_8_ADC_TRIGGER_REG: u8 = 0x15;
const AIO12_8_TRIGGER_REG: u8 = 0x16;
const AIO12_8_COS_REG: u8 = 0x17;
const AIO12_8_DAC_ENABLE_REG: u8 = 0x18;
const AIO12_8_DAC_ENABLE_REF_ENA: u8 = 1 << 0;

const AIO12_8_ADC_MODE: fn(u8) -> u8 = |x| (x & 0x3) << 6;
const AIO12_8_ADC_ACQ: fn(u8) -> u8 = |x| (x & 0x1) << 5;
const AIO12_8_ADC_RANGE: fn(u8) -> u8 = |x| x << 3;
const AIO12_8_ADC_CHAN: fn(u8) -> u8 = |x| x;
const AIO12_8_DAC_REG: fn(u8) -> u8 = |x| 0x04u8.wrapping_add(x.wrapping_mul(2));
const AIO12_8_ADC_TRIGGER_RANGE: fn(u8) -> u8 = |x| x << 3;
const AIO12_8_ADC_TRIGGER_CHAN: fn(u8) -> u8 = |x| x;

static AIO_AIO12_8_RANGE: comedi_lrange = comedi_lrange {
    length: 4,
    range: [UNI_RANGE(5), BIP_RANGE(5), UNI_RANGE(10), BIP_RANGE(10)],
};

#[repr(C)]
struct aio12_8_boardtype {
    name: *const core::ffi::c_char,
    has_ai: u32,
    has_ao: u32,
}

static BOARD_TYPES: [aio12_8_boardtype; 3] = [
    aio12_8_boardtype { name: c"aio_aio12_8".as_ptr(), has_ai: 1, has_ao: 1 },
    aio12_8_boardtype { name: c"aio_ai12_8".as_ptr(), has_ai: 1, has_ao: 0 },
    aio12_8_boardtype { name: c"aio_ao12_4".as_ptr(), has_ai: 0, has_ao: 1 },
];

unsafe extern "C" {
    fn inb(port: usize) -> u8;
    fn inw(port: usize) -> u16;
    fn outb(value: u8, port: usize);
    fn outw(value: u16, port: usize);
}

unsafe fn aio_aio12_8_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                             _insn: *mut comedi_insn, _context: usize) -> i32 {
    let status = inb((*dev).iobase + AIO12_8_STATUS_REG as usize);
    if status & AIO12_8_STATUS_ADC_EOC != 0 { 0 } else { -EBUSY }
}

unsafe fn aio_aio12_8_ai_read(dev: *mut comedi_device, s: *mut comedi_subdevice,
                              insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    let range = CR_RANGE((*insn).chanspec);
    let control = AIO12_8_ADC_MODE_NORMAL | AIO12_8_ADC_ACQ_3USEC |
                  AIO12_8_ADC_RANGE(range as u8) | AIO12_8_ADC_CHAN(chan as u8);
    inb((*dev).iobase + AIO12_8_STATUS_REG as usize);
    for i in 0..(*insn).n {
        outb(control, (*dev).iobase + AIO12_8_ADC_REG as usize);
        let ret = comedi_timeout(dev, s, insn, Some(aio_aio12_8_ai_eoc), 0);
        if ret != 0 { return ret; }
        let mut val = (inw((*dev).iobase + AIO12_8_ADC_REG as usize) as u32) & (*s).maxdata;
        if comedi_range_is_bipolar(s, range) != 0 { val = comedi_offset_munge(s, val); }
        *data.add(i as usize) = val;
    }
    (*insn).n as i32
}

unsafe fn aio_aio12_8_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice,
                                    insn: *mut comedi_insn, data: *const u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    let mut val = (*s).readback.add(chan as usize).read();
    outb(AIO12_8_DAC_ENABLE_REF_ENA, (*dev).iobase + AIO12_8_DAC_ENABLE_REG as usize);
    for i in 0..(*insn).n {
        val = *data.add(i as usize);
        outw(val as u16, (*dev).iobase + AIO12_8_DAC_REG(chan as u8) as usize);
    }
    (*s).readback.add(chan as usize).write(val);
    (*insn).n as i32
}

unsafe fn aio_aio12_8_counter_insn_config(_dev: *mut comedi_device, _s: *mut comedi_subdevice,
                                          insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    match *data {
        INSN_CONFIG_GET_CLOCK_SRC => {
            *data = 0;
            *data.add(1) = if chan == 1 { I8254_OSC_BASE_1MHZ } else { 0 };
        }
        _ => return -EINVAL,
    }
    (*insn).n as i32
}

unsafe fn aio_aio12_8_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let board = (*dev).board_ptr as *const aio12_8_boardtype;
    let mut ret = comedi_check_request_region(dev, (*it).options[0], 32, 0x100, 0x3ff, 32);
    if ret != 0 { return ret; }
    (*dev).pacer = comedi_8254_io_alloc((*dev).iobase + AIO12_8_8254_BASE_REG as usize, 0, I8254_IO8, 0);
    if IS_ERR((*dev).pacer) { return PTR_ERR((*dev).pacer); }
    ret = comedi_alloc_subdevices(dev, 4);
    if ret != 0 { return ret; }
    let s = (*dev).subdevices;
    let ai = s;
    if (*board).has_ai != 0 {
        (*ai).type_ = COMEDI_SUBD_AI; (*ai).subdev_flags = SDF_READABLE | SDF_GROUND | SDF_DIFF;
        (*ai).n_chan = 8; (*ai).maxdata = 0x0fff; (*ai).range_table = &AIO_AIO12_8_RANGE;
        (*ai).insn_read = Some(aio_aio12_8_ai_read);
    } else { (*ai).type_ = COMEDI_SUBD_UNUSED; }
    let ao = s.add(1);
    if (*board).has_ao != 0 {
        (*ao).type_ = COMEDI_SUBD_AO; (*ao).subdev_flags = SDF_WRITABLE | SDF_GROUND;
        (*ao).n_chan = 4; (*ao).maxdata = 0x0fff; (*ao).range_table = &AIO_AIO12_8_RANGE;
        (*ao).insn_write = Some(aio_aio12_8_ao_insn_write);
        ret = comedi_alloc_subdev_readback(ao); if ret != 0 { return ret; }
    } else { (*ao).type_ = COMEDI_SUBD_UNUSED; }
    ret = subdev_8255_io_init(dev, s.add(2), AIO12_8_8255_BASE_REG as usize); if ret != 0 { return ret; }
    comedi_8254_subdevice_init(s.add(3), (*dev).pacer);
    (*dev).pacer.insn_config = Some(aio_aio12_8_counter_insn_config);
    0
}

static mut AIO_AIO12_8_DRIVER: comedi_driver = comedi_driver {
    driver_name: c"aio_aio12_8".as_ptr(), module: THIS_MODULE,
    attach: Some(aio_aio12_8_attach), detach: Some(comedi_legacy_detach),
    board_name: BOARD_TYPES.as_ptr().cast(), num_names: 3,
    offset: core::mem::size_of::<aio12_8_boardtype>(),
};

module_comedi_driver!(AIO_AIO12_8_DRIVER);
MODULE_AUTHOR!("Comedi https://www.comedi.org");
MODULE_DESCRIPTION!("Comedi driver for ACCES I/O AIO12-8 Analog I/O Board");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
