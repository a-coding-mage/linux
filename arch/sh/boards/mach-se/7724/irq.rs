// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/boards/se/7724/irq.c
 *
 * Copyright (C) 2009 Renesas Solutions Corp.
 *
 * Kuninori Morimoto <morimoto.kuninori@renesas.com>
 *
 * Based on  linux/arch/sh/boards/se/7722/irq.c
 * Copyright (C) 2007  Nobuhiro Iwamatsu
 *
 * Hitachi UL SolutionEngine 7724 Support.
 */

#[repr(C)]
struct fpga_irq {
    sraddr: ::core::ffi::c_ulong,
    mraddr: ::core::ffi::c_ulong,
    mask: u16,
    base: u32,
}

unsafe fn fpga2irq(irq: u32) -> u32 {
    if irq >= IRQ0_BASE && irq <= IRQ0_END {
        IRQ0_IRQ
    } else if irq >= IRQ1_BASE && irq <= IRQ1_END {
        IRQ1_IRQ
    } else {
        IRQ2_IRQ
    }
}

unsafe fn get_fpga_irq(irq: u32) -> fpga_irq {
    let mut set: fpga_irq;

    match irq {
        IRQ0_IRQ => {
            set = fpga_irq { sraddr: IRQ0_SR, mraddr: IRQ0_MR, mask: IRQ0_MASK, base: IRQ0_BASE };
        }
        IRQ1_IRQ => {
            set = fpga_irq { sraddr: IRQ1_SR, mraddr: IRQ1_MR, mask: IRQ1_MASK, base: IRQ1_BASE };
        }
        _ => {
            set = fpga_irq { sraddr: IRQ2_SR, mraddr: IRQ2_MR, mask: IRQ2_MASK, base: IRQ2_BASE };
        }
    }

    set
}

unsafe fn disable_se7724_irq(data: *mut irq_data) {
    let irq: u32 = (*data).irq;
    let set: fpga_irq = get_fpga_irq(fpga2irq(irq));
    let bit: u32 = irq - set.base;
    __raw_writew(__raw_readw(set.mraddr) | (0x0001u16 << bit), set.mraddr);
}

unsafe fn enable_se7724_irq(data: *mut irq_data) {
    let irq: u32 = (*data).irq;
    let set: fpga_irq = get_fpga_irq(fpga2irq(irq));
    let bit: u32 = irq - set.base;
    __raw_writew(__raw_readw(set.mraddr) & !(0x0001u16 << bit), set.mraddr);
}

static mut se7724_irq_chip: irq_chip = irq_chip {
    name: "SE7724-FPGA\\0".as_ptr() as *const ::core::ffi::c_char,
    irq_mask: Some(disable_se7724_irq),
    irq_unmask: Some(enable_se7724_irq),
};

unsafe fn se7724_irq_demux(desc: *mut irq_desc) {
    let irq: u32 = irq_desc_get_irq(desc);
    let set: fpga_irq = get_fpga_irq(irq);
    let mut intv: u16 = __raw_readw(set.sraddr);
    let mut ext_irq: u32 = set.base;

    intv &= set.mask;

    while intv != 0 {
        if (intv & 1) != 0 {
            generic_handle_irq(ext_irq);
        }
        intv >>= 1;
        ext_irq += 1;
    }
}

/*
 * Initialize IRQ setting
 */
unsafe fn init_se7724_IRQ() {
    let mut irq_base: i32;
    let mut i: i32;

    __raw_writew(0xffff, IRQ0_MR);  /* mask all */
    __raw_writew(0xffff, IRQ1_MR);  /* mask all */
    __raw_writew(0xffff, IRQ2_MR);  /* mask all */
    __raw_writew(0x0000, IRQ0_SR);  /* clear irq */
    __raw_writew(0x0000, IRQ1_SR);  /* clear irq */
    __raw_writew(0x0000, IRQ2_SR);  /* clear irq */
    __raw_writew(0x002a, IRQ_MODE); /* set irq type */

    irq_base = irq_alloc_descs(
        SE7724_FPGA_IRQ_BASE,
        SE7724_FPGA_IRQ_BASE,
        SE7724_FPGA_IRQ_NR,
        numa_node_id(),
    );
    if (IS_ERR_VALUE(irq_base)) {
        pr_err!("%s: failed hooking irqs for FPGA\\n", "init_se7724_IRQ");
        return;
    }

    i = 0;
    while i < SE7724_FPGA_IRQ_NR {
        irq_set_chip_and_handler_name(
            irq_base + i,
            &raw mut se7724_irq_chip,
            handle_level_irq,
            "level\\0".as_ptr() as *const ::core::ffi::c_char,
        );
        i += 1;
    }

    irq_set_chained_handler(IRQ0_IRQ, se7724_irq_demux);
    irq_set_irq_type(IRQ0_IRQ, IRQ_TYPE_LEVEL_LOW);

    irq_set_chained_handler(IRQ1_IRQ, se7724_irq_demux);
    irq_set_irq_type(IRQ1_IRQ, IRQ_TYPE_LEVEL_LOW);

    irq_set_chained_handler(IRQ2_IRQ, se7724_irq_demux);
    irq_set_irq_type(IRQ2_IRQ, IRQ_TYPE_LEVEL_LOW);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
