// SPDX-License-Identifier: GPL-2.0+
/*
 * ke_counter.c
 * Comedi driver for Kolter-Electronic PCI Counter 1 Card
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2000 David A. Schleef <ds@schleef.org>
 */

/*
 * Driver: ke_counter
 * Description: Driver for Kolter Electronic Counter Card
 * Devices: [Kolter Electronic] PCI Counter Card (ke_counter)
 * Author: Michael Hillmann
 * Updated: Mon, 14 Apr 2008 15:42:42 +0100
 * Status: tested
 *
 * Configuration Options: not applicable, uses PCI auto config
 */

// Dependencies supplied by the Linux kernel and Comedi headers.

const KE_OSC_SEL_REG: usize = 0xf8;
const KE_DO_REG: usize = 0xfc;

const fn ke_reset_reg(x: usize) -> usize { 0x00 + x * 0x20 }
const fn ke_latch_reg(x: usize) -> usize { 0x00 + x * 0x20 }
const fn ke_lsb_reg(x: usize) -> usize { 0x04 + x * 0x20 }
const fn ke_mid_reg(x: usize) -> usize { 0x08 + x * 0x20 }
const fn ke_msb_reg(x: usize) -> usize { 0x0c + x * 0x20 }
const fn ke_sign_reg(x: usize) -> usize { 0x10 + x * 0x20 }
const fn ke_osc_sel_clk(x: u8) -> u8 { (x & 0x3) << 0 }
const KE_OSC_SEL_EXT: u8 = ke_osc_sel_clk(1);
const KE_OSC_SEL_4MHZ: u8 = ke_osc_sel_clk(2);
const KE_OSC_SEL_20MHZ: u8 = ke_osc_sel_clk(3);

unsafe fn ke_counter_insn_write(
    dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let chan = unsafe { cr_chan((*insn).chanspec) };
    let mut val: u32;
    let mut i = 0;
    while i < unsafe { (*insn).n } {
        val = unsafe { *data };
        unsafe {
            outb(((val >> 24) & 0xff) as u8, (*dev).iobase + ke_sign_reg(chan as usize) as u64);
            outb(((val >> 16) & 0xff) as u8, (*dev).iobase + ke_msb_reg(chan as usize) as u64);
            outb(((val >> 8) & 0xff) as u8, (*dev).iobase + ke_mid_reg(chan as usize) as u64);
            outb((val & 0xff) as u8, (*dev).iobase + ke_lsb_reg(chan as usize) as u64);
        }
        i += 1;
    }
    unsafe { (*insn).n as i32 }
}

unsafe fn ke_counter_insn_read(
    dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let chan = unsafe { cr_chan((*insn).chanspec) };
    let mut i = 0;
    while i < unsafe { (*insn).n } {
        unsafe {
            inb((*dev).iobase + ke_latch_reg(chan as usize) as u64);
            let mut val = inb((*dev).iobase + ke_lsb_reg(chan as usize) as u64) as u32;
            val |= (inb((*dev).iobase + ke_mid_reg(chan as usize) as u64) as u32) << 8;
            val |= (inb((*dev).iobase + ke_msb_reg(chan as usize) as u64) as u32) << 16;
            val |= (inb((*dev).iobase + ke_sign_reg(chan as usize) as u64) as u32) << 24;
            *data.add(i as usize) = val;
        }
        i += 1;
    }
    unsafe { (*insn).n as i32 }
}

unsafe fn ke_counter_reset(dev: *mut comedi_device) {
    let mut chan = 0;
    while chan < 3 {
        unsafe { outb(0, (*dev).iobase + ke_reset_reg(chan) as u64); }
        chan += 1;
    }
}

unsafe fn ke_counter_insn_config(
    dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    let src: u8;
    unsafe {
        match *data {
            INSN_CONFIG_SET_CLOCK_SRC => {
                src = match *data.add(1) {
                    KE_CLK_20MHZ => KE_OSC_SEL_20MHZ,
                    KE_CLK_4MHZ => KE_OSC_SEL_4MHZ,
                    KE_CLK_EXT => KE_OSC_SEL_EXT,
                    _ => return -EINVAL,
                };
                outb(src, (*dev).iobase + KE_OSC_SEL_REG as u64);
            }
            INSN_CONFIG_GET_CLOCK_SRC => {
                src = inb((*dev).iobase + KE_OSC_SEL_REG as u64);
                match src {
                    KE_OSC_SEL_20MHZ => { *data.add(1) = KE_CLK_20MHZ; *data.add(2) = 50; }
                    KE_OSC_SEL_4MHZ => { *data.add(1) = KE_CLK_4MHZ; *data.add(2) = 250; }
                    KE_OSC_SEL_EXT => { *data.add(1) = KE_CLK_EXT; *data.add(2) = 0; }
                    _ => return -EINVAL,
                }
            }
            INSN_CONFIG_RESET => ke_counter_reset(dev),
            _ => return -EINVAL,
        }
        (*insn).n as i32
    }
}

unsafe fn ke_counter_do_insn_bits(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut u32,
) -> i32 {
    unsafe {
        if comedi_dio_update_state(s, data) != 0 { outb((*s).state as u8, (*dev).iobase + KE_DO_REG as u64); }
        *data.add(1) = (*s).state;
        (*insn).n as i32
    }
}

unsafe fn ke_counter_auto_attach(dev: *mut comedi_device, _context_unused: usize) -> i32 {
    unsafe {
        let pcidev = comedi_to_pci_dev(dev);
        let mut ret = comedi_pci_enable(dev);
        if ret != 0 { return ret; }
        (*dev).iobase = pci_resource_start(pcidev, 0);
        ret = comedi_alloc_subdevices(dev, 2);
        if ret != 0 { return ret; }

        let s = &mut *(*dev).subdevices;
        s.type_ = COMEDI_SUBD_COUNTER;
        s.subdev_flags = SDF_READABLE;
        s.n_chan = 3;
        s.maxdata = 0x01ffffff;
        s.range_table = &range_unknown;
        s.insn_read = Some(ke_counter_insn_read);
        s.insn_write = Some(ke_counter_insn_write);
        s.insn_config = Some(ke_counter_insn_config);

        let s = &mut *(*dev).subdevices.add(1);
        s.type_ = COMEDI_SUBD_DO;
        s.subdev_flags = SDF_WRITABLE;
        s.n_chan = 3;
        s.maxdata = 1;
        s.range_table = &range_digital;
        s.insn_bits = Some(ke_counter_do_insn_bits);

        outb(KE_OSC_SEL_20MHZ, (*dev).iobase + KE_OSC_SEL_REG as u64);
        ke_counter_reset(dev);
        0
    }
}

static mut KE_COUNTER_DRIVER: comedi_driver = comedi_driver {
    driver_name: "ke_counter",
    module_: THIS_MODULE,
    auto_attach: Some(ke_counter_auto_attach),
    detach: Some(comedi_pci_detach),
};

unsafe fn ke_counter_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    unsafe { comedi_pci_auto_config(dev, &mut KE_COUNTER_DRIVER, (*id).driver_data) }
}

static KE_COUNTER_PCI_TABLE: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_KOLTER, device: 0x0014, driver_data: 0 },
    pci_device_id { vendor: 0, device: 0, driver_data: 0 },
];

static mut KE_COUNTER_PCI_DRIVER: pci_driver = pci_driver {
    name: "ke_counter",
    id_table: KE_COUNTER_PCI_TABLE.as_ptr(),
    probe: Some(ke_counter_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

// Equivalent of module_comedi_pci_driver(ke_counter_driver, ke_counter_pci_driver).
// MODULE_DEVICE_TABLE(pci, ke_counter_pci_table);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for Kolter Electronic Counter Card");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
