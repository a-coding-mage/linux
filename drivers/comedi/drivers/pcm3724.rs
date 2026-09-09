// SPDX-License-Identifier: GPL-2.0
/*
 * pcm3724.c
 * Comedi driver for Advantech PCM-3724 Digital I/O board
 *
 * Drew Csillag <drew_csillag@yahoo.com>
 */

/*
 * Driver: pcm3724
 * Description: Advantech PCM-3724
 * Devices: [Advantech] PCM-3724 (pcm3724)
 * Author: Drew Csillag <drew_csillag@yahoo.com>
 * Status: tested
 *
 * This is driver for digital I/O boards PCM-3724 with 48 DIO.
 * It needs 8255.o for operations and only immediate mode is supported.
 * See the source for configuration details.
 *
 * Copy/pasted/hacked from pcm724.c
 *
 * Configuration Options:
 *   [0] - I/O port base address
 */

/* Linux/Comedi headers provide the external types, constants, and functions
 * referenced below. */

const PCM3724_8255_0_BASE: u32 = 0x00;
const PCM3724_8255_1_BASE: u32 = 0x04;
const PCM3724_DIO_DIR_REG: u32 = 0x08;
const PCM3724_DIO_DIR_C0_OUT: u32 = 1 << 0;
const PCM3724_DIO_DIR_B0_OUT: u32 = 1 << 1;
const PCM3724_DIO_DIR_A0_OUT: u32 = 1 << 2;
const PCM3724_DIO_DIR_C1_OUT: u32 = 1 << 3;
const PCM3724_DIO_DIR_B1_OUT: u32 = 1 << 4;
const PCM3724_DIO_DIR_A1_OUT: u32 = 1 << 5;
const PCM3724_GATE_CTRL_REG: u32 = 0x09;
const PCM3724_GATE_CTRL_C0_ENA: u32 = 1 << 0;
const PCM3724_GATE_CTRL_B0_ENA: u32 = 1 << 1;
const PCM3724_GATE_CTRL_A0_ENA: u32 = 1 << 2;
const PCM3724_GATE_CTRL_C1_ENA: u32 = 1 << 3;
const PCM3724_GATE_CTRL_B1_ENA: u32 = 1 << 4;
const PCM3724_GATE_CTRL_A1_ENA: u32 = 1 << 5;

/* used to track configured dios */
#[repr(C)]
pub struct priv_pcm3724 {
    pub dio_1: i32,
    pub dio_2: i32,
}

unsafe fn compute_buffer(config: i32, devno: i32, s: *mut comedi_subdevice) -> i32 {
    let mut config = config;
    if (*s).io_bits & 0x0000ff != 0 {
        if devno == 0 { config |= PCM3724_DIO_DIR_A0_OUT as i32; }
        else { config |= PCM3724_DIO_DIR_A1_OUT as i32; }
    }
    if (*s).io_bits & 0x00ff00 != 0 {
        if devno == 0 { config |= PCM3724_DIO_DIR_B0_OUT as i32; }
        else { config |= PCM3724_DIO_DIR_B1_OUT as i32; }
    }
    if (*s).io_bits & 0xff0000 != 0 {
        if devno == 0 { config |= PCM3724_DIO_DIR_C0_OUT as i32; }
        else { config |= PCM3724_DIO_DIR_C1_OUT as i32; }
    }
    config
}

unsafe fn do_3724_config(dev: *mut comedi_device, s: *mut comedi_subdevice, _chanspec: i32) {
    let s_dio1 = (*dev).subdevices;
    let s_dio2 = s_dio1.add(1);
    let mut config = I8255_CTRL_CW as i32;
    if (*s).io_bits & 0x0000ff == 0 { config |= I8255_CTRL_A_IO as i32; }
    if (*s).io_bits & 0x00ff00 == 0 { config |= I8255_CTRL_B_IO as i32; }
    if (*s).io_bits & 0xff0000 == 0 {
        config |= (I8255_CTRL_C_HI_IO | I8255_CTRL_C_LO_IO) as i32;
    }
    let buffer_config = compute_buffer(compute_buffer(0, 0, s_dio1), 1, s_dio2);
    let port_8255_cfg = if s == s_dio1 {
        (*dev).iobase + I8255_CTRL_REG as _
    } else {
        (*dev).iobase + I8255_SIZE as _ + I8255_CTRL_REG as _
    };
    outb(buffer_config as _, (*dev).iobase + PCM3724_DIO_DIR_REG as _);
    outb(config as _, port_8255_cfg);
}

unsafe fn enable_chan(dev: *mut comedi_device, s: *mut comedi_subdevice, chanspec: i32) {
    let priv_ = (*dev).private as *mut priv_pcm3724;
    let s_dio1 = (*dev).subdevices;
    let mask = (1i32) << CR_CHAN(chanspec);
    if s == s_dio1 { (*priv_).dio_1 |= mask; } else { (*priv_).dio_2 |= mask; }
    let mut gatecfg = 0;
    if (*priv_).dio_1 & 0xff0000 != 0 { gatecfg |= PCM3724_GATE_CTRL_C0_ENA as i32; }
    if (*priv_).dio_1 & 0xff00 != 0 { gatecfg |= PCM3724_GATE_CTRL_B0_ENA as i32; }
    if (*priv_).dio_1 & 0xff != 0 { gatecfg |= PCM3724_GATE_CTRL_A0_ENA as i32; }
    if (*priv_).dio_2 & 0xff0000 != 0 { gatecfg |= PCM3724_GATE_CTRL_C1_ENA as i32; }
    if (*priv_).dio_2 & 0xff00 != 0 { gatecfg |= PCM3724_GATE_CTRL_B1_ENA as i32; }
    if (*priv_).dio_2 & 0xff != 0 { gatecfg |= PCM3724_GATE_CTRL_A1_ENA as i32; }
    outb(gatecfg as _, (*dev).iobase + PCM3724_GATE_CTRL_REG as _);
}

unsafe fn subdev_3724_insn_config(dev: *mut comedi_device, s: *mut comedi_subdevice,
                                  insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    let mask = if chan < 8 { 0x0000ff } else if chan < 16 { 0x00ff00 }
               else if chan < 20 { 0x0f0000 } else { 0xf00000 };
    let ret = comedi_dio_insn_config(dev, s, insn, data, mask);
    if ret != 0 { return ret; }
    do_3724_config(dev, s, (*insn).chanspec);
    enable_chan(dev, s, (*insn).chanspec);
    (*insn).n
}

unsafe fn pcm3724_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let priv_ = comedi_alloc_devpriv(dev, core::mem::size_of::<priv_pcm3724>()) as *mut priv_pcm3724;
    if priv_.is_null() { return -12; }
    let mut ret = comedi_check_request_region(dev, (*it).options[0], 0x10, 0, 0x3ff, 16);
    if ret != 0 { return ret; }
    ret = comedi_alloc_subdevices(dev, 2);
    if ret != 0 { return ret; }
    for i in 0..(*dev).n_subdevices {
        let s = (*dev).subdevices.add(i as usize);
        ret = subdev_8255_io_init(dev, s, i * I8255_SIZE);
        if ret != 0 { return ret; }
        (*s).insn_config = Some(subdev_3724_insn_config);
    }
    0
}

/* module_comedi_driver(pcm3724_driver); */
/* MODULE_AUTHOR("Comedi https://www.comedi.org"); */
/* MODULE_DESCRIPTION("Comedi driver for Advantech PCM-3724 Digital I/O board"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
