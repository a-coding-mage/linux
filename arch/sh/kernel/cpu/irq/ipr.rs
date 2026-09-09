// SPDX-License-Identifier: GPL-2.0
/*
 * Interrupt handling for IPR-based IRQ.
 *
 * Copyright (C) 1999  Niibe Yutaka & Takeshi Yaegashi
 * Copyright (C) 2000  Kazumoto Kojima
 * Copyright (C) 2003  Takashi Kusuda <kusuda-takashi@hitachi-ul.co.jp>
 * Copyright (C) 2006  Paul Mundt
 *
 * Supported system:
 *\tOn-chip supporting modules (TMU, RTC, etc.).
 *\tOn-chip supporting modules for SH7709/SH7709A/SH7729.
 *\tHitachi SolutionEngine external I/O:
 *\t\tMS7709SE01, MS7709ASE01, and MS7750SE01
 */

/* C dependencies supplied by the surrounding kernel translation. */

#[inline]
unsafe fn get_ipr_desc(data: *mut irq_data) -> *mut ipr_desc {
    let chip = irq_data_get_irq_chip(data);
    container_of_irq_chip(chip)
}

unsafe fn disable_ipr_irq(data: *mut irq_data) {
    let p = irq_data_get_irq_chip_data(data);
    let addr = (*get_ipr_desc(data)).ipr_offsets[(*p).ipr_idx as usize];
    /* Set the priority in IPR to 0 */
    __raw_writew(
        __raw_readw(addr) & (0xffffu16 ^ (0xfu16 << (*p).shift)),
        addr,
    );
    let _ = __raw_readw(addr); /* Read back to flush write posting */
}

unsafe fn enable_ipr_irq(data: *mut irq_data) {
    let p = irq_data_get_irq_chip_data(data);
    let addr = (*get_ipr_desc(data)).ipr_offsets[(*p).ipr_idx as usize];
    /* Set priority in IPR back to original value */
    __raw_writew(__raw_readw(addr) | ((*p).priority << (*p).shift), addr);
}

/*
 * The shift value is now the number of bits to shift, not the number of
 * bits/4. This is to make it easier to read the value directly from the
 * datasheets. The IPR address is calculated using the ipr_offset table.
 */
pub unsafe fn register_ipr_controller(desc: *mut ipr_desc) {
    let mut i = 0;

    (*desc).chip.irq_mask = Some(disable_ipr_irq);
    (*desc).chip.irq_unmask = Some(enable_ipr_irq);

    while i < (*desc).nr_irqs {
        let p = (*desc).ipr_data.add(i as usize);
        let res: i32;

        BUG_ON((*p).ipr_idx >= (*desc).nr_offsets);
        BUG_ON((*desc).ipr_offsets[(*p).ipr_idx as usize] == 0);

        res = irq_alloc_desc_at((*p).irq, numa_node_id());
        if res != (*p).irq && res != -EEXIST {
            printk(KERN_INFO, "can not get irq_desc for %d\n", (*p).irq);
            i += 1;
            continue;
        }

        disable_irq_nosync((*p).irq);
        irq_set_chip_and_handler_name(
            (*p).irq,
            &mut (*desc).chip,
            handle_level_irq,
            "level",
        );
        irq_set_chip_data((*p).irq, p);
        disable_ipr_irq(irq_get_irq_data((*p).irq));
        i += 1;
    }
}

extern "C" {
    fn irq_data_get_irq_chip(data: *mut irq_data) -> *mut irq_chip;
    fn container_of_irq_chip(chip: *mut irq_chip) -> *mut ipr_desc;
    fn irq_data_get_irq_chip_data(data: *mut irq_data) -> *mut ipr_data;
    fn __raw_readw(addr: usize) -> u16;
    fn __raw_writew(value: u16, addr: usize);
    fn BUG_ON(condition: bool);
    fn irq_alloc_desc_at(irq: i32, node: i32) -> i32;
    fn numa_node_id() -> i32;
    fn printk(level: i32, format: *const u8, ...);
    fn disable_irq_nosync(irq: i32);
    fn irq_set_chip_and_handler_name(
        irq: i32,
        chip: *mut irq_chip,
        handler: unsafe extern "C" fn(),
        name: *const u8,
    );
    fn irq_set_chip_data(irq: i32, data: *mut ipr_data);
    fn irq_get_irq_data(irq: i32) -> *mut irq_data;
    fn handle_level_irq();
}

extern "C" {
    static EEXIST: i32;
    static KERN_INFO: i32;
}

#[repr(C)]
pub struct irq_data;
#[repr(C)]
pub struct irq_chip {
    pub irq_mask: Option<unsafe fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe fn(*mut irq_data)>,
}
#[repr(C)]
pub struct ipr_data {
    pub ipr_idx: u32,
    pub shift: u32,
    pub priority: u16,
    pub irq: i32,
}
#[repr(C)]
pub struct ipr_desc {
    pub chip: irq_chip,
    pub ipr_offsets: *mut usize,
    pub nr_offsets: u32,
    pub ipr_data: *mut ipr_data,
    pub nr_irqs: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
