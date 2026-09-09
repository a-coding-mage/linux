// SPDX-License-Identifier: GPL-2.0
/*
 * Hitachi UL SolutionEngine 7722 FPGA IRQ Support.
 *
 * Copyright (C) 2007  Nobuhiro Iwamatsu
 * Copyright (C) 2012  Paul Mundt
 */

// Dependency intent: declarations supplied by the Linux IRQ, I/O, and SE7722
// platform interfaces are referenced below and are provided by other modules.

use core::ffi::c_void;

const DRV_NAME: &str = "SE7722-FPGA";

const IRQ01_BASE_ADDR: usize = 0x11800000;
const IRQ01_MODE_REG: usize = 0;
const IRQ01_STS_REG: usize = 4;
const IRQ01_MASK_REG: usize = 8;

extern "C" {
    static mut se7722_irq_domain: *mut irq_domain;
    static IRQ0_IRQ: c_uint;
    static IRQ1_IRQ: c_uint;
    static SE7722_FPGA_IRQ_NR: c_uint;

    fn irq_desc_get_irq_data(desc: *mut irq_desc) -> *mut irq_data;
    fn irq_data_get_irq_chip(data: *mut irq_data) -> *mut irq_chip;
    fn ioread16(addr: *mut c_void) -> u16;
    fn generic_handle_domain_irq(domain: *mut irq_domain, hwirq: c_uint);
    fn irq_domain_create_linear(
        fwnode: *mut c_void,
        size: c_uint,
        ops: *const irq_domain_ops,
        host_data: *mut c_void,
    ) -> *mut irq_domain;
    static irq_domain_simple_ops: irq_domain_ops;
    fn printk(fmt: *const u8, ...);
    fn irq_create_mapping(domain: *mut irq_domain, hwirq: c_uint) -> c_uint;
    fn irq_find_mapping(domain: *mut irq_domain, hwirq: c_uint) -> c_uint;
    fn irq_alloc_generic_chip(
        name: *const u8,
        num_ct: c_int,
        irq_base: c_uint,
        reg_base: *mut c_void,
        handler: unsafe extern "C" fn(*mut irq_desc),
    ) -> *mut irq_chip_generic;
    fn handle_level_irq(desc: *mut irq_desc);
    fn irq_gc_mask_set_bit(data: *mut irq_data);
    fn irq_gc_mask_clr_bit(data: *mut irq_data);
    fn irq_setup_generic_chip(
        gc: *mut irq_chip_generic,
        msk: c_ulong,
        flags: c_ulong,
        clr: c_ulong,
        set: c_ulong,
    );
    fn irq_set_chained_handler(irq: c_uint, handler: unsafe extern "C" fn(*mut irq_desc));
    fn irq_set_irq_type(irq: c_uint, irq_type: c_uint);
    fn ioremap(addr: usize, size: usize) -> *mut c_void;
    fn iowrite16(value: u16, addr: *mut c_void);
    fn __raw_writew(value: u16, addr: *mut c_void);
}

#[repr(C)]
pub struct irq_desc;
#[repr(C)]
pub struct irq_data;
#[repr(C)]
pub struct irq_chip;
#[repr(C)]
pub struct irq_domain;
#[repr(C)]
pub struct irq_domain_ops;
#[repr(C)]
pub struct irq_chip_generic {
    pub chip_types: *mut irq_chip_type,
}
#[repr(C)]
pub struct irq_chip_type {
    pub chip: irq_chip,
    pub regs: irq_chip_regs,
}
#[repr(C)]
pub struct irq_chip_regs {
    pub mask: usize,
}

type c_int = i32;
type c_uint = u32;
type c_ulong = usize;

const IRQ_GC_INIT_MASK_CACHE: c_ulong = 1;
const IRQ_NOREQUEST: c_ulong = 1 << 1;
const IRQ_NOPROBE: c_ulong = 1 << 2;
const IRQ_TYPE_LEVEL_LOW: c_uint = 8;

static mut se7722_irq_regs: *mut c_void = core::ptr::null_mut();

unsafe extern "C" fn se7722_irq_demux(desc: *mut irq_desc) {
    let data = irq_desc_get_irq_data(desc);
    let chip = irq_data_get_irq_chip(data);
    let mut mask: c_ulong;
    let mut bit: c_uint;

    ((*chip).irq_mask_ack.unwrap())(data);

    mask = ioread16((se7722_irq_regs as *mut u8).add(IRQ01_STS_REG)) as c_ulong;

    bit = 0;
    while bit < SE7722_FPGA_IRQ_NR {
        if (mask & (1usize << bit)) != 0 {
            generic_handle_domain_irq(se7722_irq_domain, bit);
        }
        bit += 1;
    }

    ((*chip).irq_unmask.unwrap())(data);
}

unsafe extern "C" fn se7722_domain_init() {
    se7722_irq_domain = irq_domain_create_linear(
        core::ptr::null_mut(),
        SE7722_FPGA_IRQ_NR,
        &irq_domain_simple_ops,
        core::ptr::null_mut(),
    );
    if se7722_irq_domain.is_null() {
        printk(b"Failed to get IRQ domain\0".as_ptr());
        return;
    }

    let mut i: c_uint = 0;
    while i < SE7722_FPGA_IRQ_NR {
        let irq = irq_create_mapping(se7722_irq_domain, i);
        if irq == 0 {
            printk(b"Failed to allocate IRQ %d\n\0".as_ptr(), i);
            return;
        }
        i += 1;
    }
}

unsafe extern "C" fn se7722_gc_init() {
    let irq_base = irq_find_mapping(se7722_irq_domain, 0);
    let gc = irq_alloc_generic_chip(
        b"SE7722-FPGA\0".as_ptr(),
        1,
        irq_base,
        se7722_irq_regs,
        handle_level_irq,
    );
    if gc.is_null() {
        return;
    }

    let ct = (*gc).chip_types;
    (*ct).chip.irq_mask = Some(irq_gc_mask_set_bit);
    (*ct).chip.irq_unmask = Some(irq_gc_mask_clr_bit);
    (*ct).regs.mask = IRQ01_MASK_REG;

    irq_setup_generic_chip(
        gc,
        (1usize << SE7722_FPGA_IRQ_NR) - 1,
        IRQ_GC_INIT_MASK_CACHE,
        IRQ_NOREQUEST | IRQ_NOPROBE,
        0,
    );

    irq_set_chained_handler(IRQ0_IRQ, se7722_irq_demux);
    irq_set_irq_type(IRQ0_IRQ, IRQ_TYPE_LEVEL_LOW);
    irq_set_chained_handler(IRQ1_IRQ, se7722_irq_demux);
    irq_set_irq_type(IRQ1_IRQ, IRQ_TYPE_LEVEL_LOW);
}

/*
 * Initialize FPGA IRQs
 */
pub unsafe extern "C" fn init_se7722_IRQ() {
    se7722_irq_regs = ioremap(IRQ01_BASE_ADDR, 16);
    if se7722_irq_regs.is_null() {
        printk(b"Failed to remap IRQ01 regs\n\0".as_ptr());
        return;
    }

    /*
     * All FPGA IRQs disabled by default
     */
    iowrite16(0, (se7722_irq_regs as *mut u8).add(IRQ01_MASK_REG) as *mut c_void);

    __raw_writew(0x2000, 0xb03fffec as *mut c_void); /* mrshpc irq enable */

    se7722_domain_init();
    se7722_gc_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
