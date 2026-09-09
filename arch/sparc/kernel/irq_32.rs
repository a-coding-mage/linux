// SPDX-License-Identifier: GPL-2.0
/*
 * Interrupt request handling routines. On the Sparc the IRQs are basically
 * 'cast in stone' and are discovered by probing the prom's device trees.
 *
 * This is a direct low-level translation of irq_32.c.
 */

// C dependencies supplied by the surrounding kernel translation.

extern "C" {
    static mut sparc_config: sparc_config;
    static mut irq_table: [irq_bucket; NR_IRQS];
    static mut irq_map: [*mut irq_bucket; SUN4D_MAX_IRQ];
    static mut floppy_irq: u32;
    static mut fdc_status: *mut u8;
    static mut pdma_vaddr: *mut i8;
    static mut pdma_size: c_ulong;
    static mut doing_pdma: c_int;
    static mut pdma_base: *mut i8;
    static mut pdma_areasize: c_ulong;
}

pub unsafe extern "C" fn arch_local_irq_save() -> c_ulong {
    let mut retval: c_ulong;
    let mut tmp: c_ulong;
    core::arch::asm!(
        "rd %psr, {0}\n\t",
        "or {0}, {2}, {1}\n\t",
        "wr {1}, 0, %psr\n\t",
        "nop; nop; nop",
        out(reg) retval, out(reg) tmp, const PSR_PIL,
        options(nostack)
    );
    retval
}

pub unsafe extern "C" fn arch_local_irq_enable() {
    let mut tmp: c_ulong;
    core::arch::asm!(
        "rd %psr, {0}\n\t", "andn {0}, {1}, {0}\n\t",
        "wr {0}, 0, %psr\n\t", "nop; nop; nop",
        out(reg) tmp, const PSR_PIL, options(nostack)
    );
}

pub unsafe extern "C" fn arch_local_irq_restore(mut old_psr: c_ulong) {
    let mut tmp: c_ulong;
    core::arch::asm!(
        "rd %psr, {0}\n\t", "and {2}, {1}, {2}\n\t",
        "andn {0}, {1}, {0}\n\t", "wr {0}, {2}, %psr\n\t",
        "nop; nop; nop", out(reg) tmp, const PSR_PIL, inout(reg) old_psr,
        options(nostack)
    );
}

static mut irq_table_lock: DEFINE_SPINLOCK = DEFINE_SPINLOCK::new();
static mut irq_map_lock: DEFINE_SPINLOCK = DEFINE_SPINLOCK::new();

pub unsafe extern "C" fn irq_alloc(real_irq: c_uint, pil: c_uint) -> c_uint {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut irq_table_lock, &mut flags);
    let mut i = 1;
    while i < NR_IRQS {
        if irq_table[i].real_irq == real_irq && irq_table[i].pil == pil { break; }
        i += 1;
    }
    if i == NR_IRQS {
        i = 1;
        while i < NR_IRQS && irq_table[i].irq != 0 { i += 1; }
        if i < NR_IRQS {
            irq_table[i].real_irq = real_irq;
            irq_table[i].irq = i;
            irq_table[i].pil = pil;
        } else { printk(KERN_ERR, "IRQ: Out of virtual IRQs.\n"); i = 0; }
    }
    spin_unlock_irqrestore(&mut irq_table_lock, flags);
    i
}

pub unsafe extern "C" fn irq_link(irq: c_uint) {
    BUG_ON(irq >= NR_IRQS);
    let mut flags = 0;
    spin_lock_irqsave(&mut irq_map_lock, &mut flags);
    let p = &mut irq_table[irq as usize];
    BUG_ON(p.pil >= SUN4D_MAX_IRQ);
    p.next = irq_map[p.pil as usize];
    irq_map[p.pil as usize] = p;
    spin_unlock_irqrestore(&mut irq_map_lock, flags);
}

pub unsafe extern "C" fn irq_unlink(irq: c_uint) {
    BUG_ON(irq >= NR_IRQS);
    let mut flags = 0;
    spin_lock_irqsave(&mut irq_map_lock, &mut flags);
    let p: *mut irq_bucket = &mut irq_table[irq as usize];
    BUG_ON((*p).pil >= SUN4D_MAX_IRQ);
    let mut pnext: *mut *mut irq_bucket = &mut irq_map[(*p).pil as usize];
    while *pnext != p { pnext = &mut (**pnext).next; }
    *pnext = (*p).next;
    spin_unlock_irqrestore(&mut irq_map_lock, flags);
}

pub unsafe extern "C" fn arch_show_interrupts(p: *mut seq_file, prec: c_int) -> c_int {
    #[cfg(CONFIG_SMP)]
    { seq_printf(p, "%*s:", prec, "RES"); for_each_online_cpu(|j| seq_put_decimal_ull_width(p, " ", cpu_data(j).irq_resched_count, 10)); seq_printf(p, " IPI rescheduling interrupts\n"); seq_printf(p, "%*s:", prec, "CAL"); for_each_online_cpu(|j| seq_put_decimal_ull_width(p, " ", cpu_data(j).irq_call_count, 10)); seq_printf(p, " IPI function call interrupts\n"); }
    seq_printf(p, "%*s:", prec, "NMI");
    for_each_online_cpu(|j| seq_put_decimal_ull_width(p, " ", cpu_data(j).counter, 10));
    seq_printf(p, " Non-maskable interrupts\n"); 0
}

pub unsafe extern "C" fn handler_irq(pil: c_uint, regs: *mut pt_regs) {
    BUG_ON(pil > 15); let old_regs = set_irq_regs(regs); irq_enter();
    let mut p = irq_map[pil as usize];
    while !p.is_null() { let next = (*p).next; generic_handle_irq((*p).irq); p = next; }
    irq_exit(); set_irq_regs(old_regs);
}

#[cfg(any(CONFIG_BLK_DEV_FD, CONFIG_BLK_DEV_FD_MODULE))]
pub unsafe extern "C" fn sparc_floppy_request_irq(irq: c_uint, irq_handler: irq_handler_t) -> c_int {
    if request_irq(irq, irq_handler, 0, "floppy", core::ptr::null_mut()) != 0 { return -1; }
    floppy_irq = irq; let cpu_irq = irq & (NR_IRQS - 1);
    // INSTANTIATE(sparc_ttable) and per-CPU trap tables write the four trap instructions.
    // The referenced trap-table symbols and instruction encodings are supplied externally.
    flush_cache_all(); 0
}

#[cfg(any(CONFIG_BLK_DEV_FD, CONFIG_BLK_DEV_FD_MODULE))]
pub unsafe extern "C" fn sparc_floppy_irq(_irq: c_int, _dev_id: *mut c_void, regs: *mut pt_regs) {
    let old_regs = set_irq_regs(regs); irq_enter(); generic_handle_irq(floppy_irq); irq_exit(); set_irq_regs(old_regs);
}

pub unsafe extern "C" fn init_IRQ() {
    match sparc_cpu_model {
        sun4m => { pcic_probe(); if pcic_present() { sun4m_pci_init_IRQ(); } else { sun4m_init_IRQ(); } }
        sun4d => sun4d_init_IRQ(),
        sparc_leon => leon_init_IRQ(),
        _ => prom_printf("Cannot initialize IRQs on this Sun machine..."),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
