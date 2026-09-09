// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2008 Ilya Yanok, Emcraft Systems
 */

// Linux IRQ, device-tree, MMIO, and socrates_fpga_pic dependencies supplied externally.

const SOCRATES_FPGA_NUM_IRQS: usize = 9;

const FPGA_PIC_IRQCFG: usize = 0x0;
#[inline]
const fn FPGA_PIC_IRQMASK(n: usize) -> usize { 0x4 + 0x4 * n }

const SOCRATES_FPGA_IRQ_MASK: u32 = (1u32 << SOCRATES_FPGA_NUM_IRQS) - 1;

#[repr(C)]
struct socrates_fpga_irq_info {
    irq_line: u32,
    type_: i32,
}

/*
 * Interrupt routing and type table
 *
 * IRQ_TYPE_NONE means the interrupt type is configurable,
 * otherwise it's fixed to the specified value.
 */
static mut fpga_irqs: [socrates_fpga_irq_info; SOCRATES_FPGA_NUM_IRQS] = [
    socrates_fpga_irq_info { irq_line: 0, type_: IRQ_TYPE_NONE },
    socrates_fpga_irq_info { irq_line: 0, type_: IRQ_TYPE_LEVEL_HIGH },
    socrates_fpga_irq_info { irq_line: 0, type_: IRQ_TYPE_LEVEL_LOW },
    socrates_fpga_irq_info { irq_line: 0, type_: IRQ_TYPE_NONE },
    socrates_fpga_irq_info { irq_line: 0, type_: IRQ_TYPE_NONE },
    socrates_fpga_irq_info { irq_line: 0, type_: IRQ_TYPE_NONE },
    socrates_fpga_irq_info { irq_line: 0, type_: IRQ_TYPE_NONE },
    socrates_fpga_irq_info { irq_line: 0, type_: IRQ_TYPE_NONE },
    socrates_fpga_irq_info { irq_line: 0, type_: IRQ_TYPE_LEVEL_HIGH },
];

static mut socrates_fpga_pic_lock: raw_spinlock_t = DEFINE_RAW_SPINLOCK!();
static mut socrates_fpga_pic_iobase: *mut core::ffi::c_void = core::ptr::null_mut();
static mut socrates_fpga_pic_irq_host: *mut irq_domain = core::ptr::null_mut();
static mut socrates_fpga_irqs: [u32; 3] = [0; 3];

#[inline]
unsafe fn socrates_fpga_pic_read(reg: usize) -> u32 {
    in_be32(socrates_fpga_pic_iobase.cast::<u8>().add(reg).cast())
}

#[inline]
unsafe fn socrates_fpga_pic_write(reg: usize, val: u32) {
    out_be32(socrates_fpga_pic_iobase.cast::<u8>().add(reg).cast(), val);
}

#[inline]
unsafe fn socrates_fpga_pic_get_irq(irq: u32) -> u32 {
    let mut cause: u32;
    let mut flags: ulong;
    let mut i: i32 = 0;

    /* Check irq line routed to the MPIC */
    while i < 3 {
        if irq == socrates_fpga_irqs[i as usize] { break; }
        i += 1;
    }
    if i == 3 { return 0; }

    raw_spin_lock_irqsave(&raw mut socrates_fpga_pic_lock, &raw mut flags);
    cause = socrates_fpga_pic_read(FPGA_PIC_IRQMASK(i as usize));
    raw_spin_unlock_irqrestore(&raw mut socrates_fpga_pic_lock, flags);
    i = SOCRATES_FPGA_NUM_IRQS as i32 - 1;
    while i >= 0 {
        if (cause >> (i + 16)) != 0 { break; }
        i -= 1;
    }
    irq_find_mapping(socrates_fpga_pic_irq_host, i as irq_hw_number_t)
}

unsafe fn socrates_fpga_pic_cascade(desc: *mut irq_desc) {
    let chip = irq_desc_get_chip(desc);
    let irq = irq_desc_get_irq(desc);
    let cascade_irq = socrates_fpga_pic_get_irq(irq);
    if cascade_irq != 0 { generic_handle_irq(cascade_irq); }
    ((*chip).irq_eoi)(&mut (*desc).irq_data);
}

unsafe fn socrates_fpga_pic_ack(d: *mut irq_data) {
    let mut flags: ulong; let hwirq = irqd_to_hwirq(d) as usize;
    let irq_line = fpga_irqs[hwirq].irq_line as usize;
    raw_spin_lock_irqsave(&raw mut socrates_fpga_pic_lock, &raw mut flags);
    let mut mask = socrates_fpga_pic_read(FPGA_PIC_IRQMASK(irq_line)) & SOCRATES_FPGA_IRQ_MASK;
    mask |= 1u32 << (hwirq + 16);
    socrates_fpga_pic_write(FPGA_PIC_IRQMASK(irq_line), mask);
    raw_spin_unlock_irqrestore(&raw mut socrates_fpga_pic_lock, flags);
}

unsafe fn socrates_fpga_pic_mask(d: *mut irq_data) {
    let mut flags: ulong; let hwirq = irqd_to_hwirq(d) as usize;
    let irq_line = fpga_irqs[hwirq].irq_line as usize;
    raw_spin_lock_irqsave(&raw mut socrates_fpga_pic_lock, &raw mut flags);
    let mut mask = socrates_fpga_pic_read(FPGA_PIC_IRQMASK(irq_line)) & SOCRATES_FPGA_IRQ_MASK;
    mask &= !(1u32 << hwirq);
    socrates_fpga_pic_write(FPGA_PIC_IRQMASK(irq_line), mask);
    raw_spin_unlock_irqrestore(&raw mut socrates_fpga_pic_lock, flags);
}

unsafe fn socrates_fpga_pic_mask_ack(d: *mut irq_data) {
    let mut flags: ulong; let hwirq = irqd_to_hwirq(d) as usize;
    let irq_line = fpga_irqs[hwirq].irq_line as usize;
    raw_spin_lock_irqsave(&raw mut socrates_fpga_pic_lock, &raw mut flags);
    let mut mask = socrates_fpga_pic_read(FPGA_PIC_IRQMASK(irq_line)) & SOCRATES_FPGA_IRQ_MASK;
    mask &= !(1u32 << hwirq); mask |= 1u32 << (hwirq + 16);
    socrates_fpga_pic_write(FPGA_PIC_IRQMASK(irq_line), mask);
    raw_spin_unlock_irqrestore(&raw mut socrates_fpga_pic_lock, flags);
}

unsafe fn socrates_fpga_pic_unmask(d: *mut irq_data) {
    let mut flags: ulong; let hwirq = irqd_to_hwirq(d) as usize;
    let irq_line = fpga_irqs[hwirq].irq_line as usize;
    raw_spin_lock_irqsave(&raw mut socrates_fpga_pic_lock, &raw mut flags);
    let mut mask = socrates_fpga_pic_read(FPGA_PIC_IRQMASK(irq_line)) & SOCRATES_FPGA_IRQ_MASK;
    mask |= 1u32 << hwirq; socrates_fpga_pic_write(FPGA_PIC_IRQMASK(irq_line), mask);
    raw_spin_unlock_irqrestore(&raw mut socrates_fpga_pic_lock, flags);
}

unsafe fn socrates_fpga_pic_eoi(d: *mut irq_data) {
    let mut flags: ulong; let hwirq = irqd_to_hwirq(d) as usize;
    let irq_line = fpga_irqs[hwirq].irq_line as usize;
    raw_spin_lock_irqsave(&raw mut socrates_fpga_pic_lock, &raw mut flags);
    let mut mask = socrates_fpga_pic_read(FPGA_PIC_IRQMASK(irq_line)) & SOCRATES_FPGA_IRQ_MASK;
    mask |= 1u32 << (hwirq + 16); socrates_fpga_pic_write(FPGA_PIC_IRQMASK(irq_line), mask);
    raw_spin_unlock_irqrestore(&raw mut socrates_fpga_pic_lock, flags);
}

unsafe fn socrates_fpga_pic_set_type(d: *mut irq_data, flow_type: u32) -> i32 {
    let mut flags: ulong; let hwirq = irqd_to_hwirq(d) as usize;
    if fpga_irqs[hwirq].type_ != IRQ_TYPE_NONE { return -EINVAL; }
    let polarity = match flow_type & IRQ_TYPE_SENSE_MASK {
        IRQ_TYPE_LEVEL_HIGH => 1, IRQ_TYPE_LEVEL_LOW => 0, _ => return -EINVAL,
    };
    raw_spin_lock_irqsave(&raw mut socrates_fpga_pic_lock, &raw mut flags);
    let mut mask = socrates_fpga_pic_read(FPGA_PIC_IRQCFG);
    if polarity != 0 { mask |= 1u32 << hwirq; } else { mask &= !(1u32 << hwirq); }
    socrates_fpga_pic_write(FPGA_PIC_IRQCFG, mask);
    raw_spin_unlock_irqrestore(&raw mut socrates_fpga_pic_lock, flags); 0
}

static mut socrates_fpga_pic_chip: irq_chip = irq_chip {
    name: "FPGA-PIC", irq_ack: Some(socrates_fpga_pic_ack), irq_mask: Some(socrates_fpga_pic_mask),
    irq_mask_ack: Some(socrates_fpga_pic_mask_ack), irq_unmask: Some(socrates_fpga_pic_unmask),
    irq_eoi: Some(socrates_fpga_pic_eoi), irq_set_type: Some(socrates_fpga_pic_set_type),
};

unsafe fn socrates_fpga_pic_host_map(_h: *mut irq_domain, virq: u32, _hwirq: irq_hw_number_t) -> i32 {
    irq_set_status_flags(virq, IRQ_LEVEL);
    irq_set_chip_and_handler(virq, &raw mut socrates_fpga_pic_chip, handle_fasteoi_irq); 0
}

unsafe fn socrates_fpga_pic_host_xlate(_h: *mut irq_domain, _ct: *mut device_node,
    intspec: *const u32, _intsize: u32, out_hwirq: *mut irq_hw_number_t, out_flags: *mut u32) -> i32 {
    let fpga_irq = &mut fpga_irqs[*intspec as usize];
    *out_hwirq = *intspec as irq_hw_number_t;
    if fpga_irq.type_ == IRQ_TYPE_NONE {
        if *intspec.add(1) != IRQ_TYPE_LEVEL_LOW && *intspec.add(1) != IRQ_TYPE_LEVEL_HIGH {
            pr_warn!("FPGA PIC: invalid irq type, setting default active low\n"); *out_flags = IRQ_TYPE_LEVEL_LOW;
        } else { *out_flags = *intspec.add(1); }
    } else { *out_flags = fpga_irq.type_ as u32; }
    if *intspec.add(2) <= 2 { fpga_irq.irq_line = *intspec.add(2); }
    else { pr_warn!("FPGA PIC: invalid irq routing\n"); }
    0
}

static socrates_fpga_pic_host_ops: irq_domain_ops = irq_domain_ops {
    map: Some(socrates_fpga_pic_host_map), xlate: Some(socrates_fpga_pic_host_xlate),
};

unsafe fn socrates_fpga_pic_init(pic: *mut device_node) {
    let mut flags: ulong;
    socrates_fpga_pic_irq_host = irq_domain_create_linear(of_fwnode_handle(pic), SOCRATES_FPGA_NUM_IRQS as u32, &socrates_fpga_pic_host_ops, core::ptr::null_mut());
    if socrates_fpga_pic_irq_host.is_null() { pr_err!("FPGA PIC: Unable to allocate host\n"); return; }
    for i in 0..3 {
        socrates_fpga_irqs[i] = irq_of_parse_and_map(pic, i as u32);
        if socrates_fpga_irqs[i] == 0 { pr_warn!("FPGA PIC: can't get irq{}\n", i); continue; }
        irq_set_chained_handler(socrates_fpga_irqs[i], socrates_fpga_pic_cascade);
    }
    socrates_fpga_pic_iobase = of_iomap(pic, 0);
    raw_spin_lock_irqsave(&raw mut socrates_fpga_pic_lock, &raw mut flags);
    for i in 0..3 { socrates_fpga_pic_write(FPGA_PIC_IRQMASK(i), SOCRATES_FPGA_IRQ_MASK << 16); }
    raw_spin_unlock_irqrestore(&raw mut socrates_fpga_pic_lock, flags);
    pr_info!("FPGA PIC: Setting up Socrates FPGA PIC\n");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
