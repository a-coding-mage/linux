// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct translation of parisc/kernel/irq.c. Kernel-provided symbols are external. */

// Bits in EIEM correlate with cpu_irq_action[]; numbered big endian.
const fn eiem_mask(irq: usize) -> usize { 1usize << (CPU_IRQ_MAX - irq) }

static mut CPU_EIEM: usize = 0;

unsafe fn cpu_mask_irq(d: *mut irq_data) {
    let eirr_bit = eiem_mask((*d).irq as usize);
    CPU_EIEM &= !eirr_bit;
}

unsafe fn __cpu_unmask_irq(irq: u32) {
    CPU_EIEM |= eiem_mask(irq as usize);
    smp_send_all_nop();
}

unsafe fn cpu_unmask_irq(d: *mut irq_data) { __cpu_unmask_irq((*d).irq); }

pub unsafe fn cpu_ack_irq(d: *mut irq_data) {
    let mask = eiem_mask((*d).irq as usize);
    let cpu = smp_processor_id();
    *per_cpu_local_ack_eiem(cpu) &= !mask;
    set_eiem(CPU_EIEM & *per_cpu_local_ack_eiem(cpu));
    mtctl(mask, 23);
}

pub unsafe fn cpu_eoi_irq(d: *mut irq_data) {
    let mask = eiem_mask((*d).irq as usize);
    let cpu = smp_processor_id();
    *per_cpu_local_ack_eiem(cpu) |= mask;
    set_eiem(CPU_EIEM & *per_cpu_local_ack_eiem(cpu));
}

#[cfg(CONFIG_SMP)]
pub unsafe fn cpu_check_affinity(d: *mut irq_data, dest: *const cpumask) -> i32 {
    if irqd_is_per_cpu(d) { return -EINVAL; }
    let mut cpu_dest = cpumask_first_and(dest, cpu_online_mask);
    if cpu_dest >= nr_cpu_ids { cpu_dest = cpumask_first(cpu_online_mask); }
    cpu_dest
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const u8,
    pub irq_mask: Option<unsafe fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe fn(*mut irq_data)>,
    pub irq_ack: Option<unsafe fn(*mut irq_data)>,
    pub irq_eoi: Option<unsafe fn(*mut irq_data)>,
    pub irq_retrigger: Option<unsafe fn(*mut irq_data)>,
}

static mut cpu_interrupt_type: irq_chip = irq_chip {
    name: b"CPU\0".as_ptr(), irq_mask: Some(cpu_mask_irq),
    irq_unmask: Some(cpu_unmask_irq), irq_ack: Some(cpu_ack_irq),
    irq_eoi: Some(cpu_eoi_irq), irq_retrigger: None,
};

pub unsafe fn arch_show_interrupts(p: *mut seq_file, prec: i32) -> i32 {
    let mut j: i32;
    #[cfg(CONFIG_DEBUG_STACKOVERFLOW)] {
        seq_printf(p, b"%*s: \0".as_ptr(), prec, b"STK\0".as_ptr());
        for_each_online_cpu!(j, { seq_printf(p, b"%10u \0".as_ptr(), irq_stats(j).kernel_stack_usage); });
        seq_puts(p, b"  Kernel stack usage\n\0".as_ptr());
        #[cfg(CONFIG_IRQSTACKS)] {
            seq_printf(p, b"%*s: \0".as_ptr(), prec, b"IST\0".as_ptr());
            for_each_online_cpu!(j, { seq_printf(p, b"%10u \0".as_ptr(), irq_stats(j).irq_stack_usage); });
            seq_puts(p, b"  Interrupt stack usage\n\0".as_ptr());
        }
    }
    #[cfg(CONFIG_SMP)] if num_online_cpus() > 1 {
        seq_printf(p, b"%*s: \0".as_ptr(), prec, b"RES\0".as_ptr());
        for_each_online_cpu!(j, { seq_printf(p, b"%10u \0".as_ptr(), irq_stats(j).irq_resched_count); });
        seq_puts(p, b"  Rescheduling interrupts\n\0".as_ptr());
        seq_printf(p, b"%*s: \0".as_ptr(), prec, b"CAL\0".as_ptr());
        for_each_online_cpu!(j, { seq_printf(p, b"%10u \0".as_ptr(), irq_stats(j).irq_call_count); });
        seq_puts(p, b"  Function call interrupts\n\0".as_ptr());
    }
    seq_printf(p, b"%*s: \0".as_ptr(), prec, b"UAH\0".as_ptr());
    for_each_online_cpu!(j, { seq_printf(p, b"%10u \0".as_ptr(), irq_stats(j).irq_unaligned_count); });
    seq_puts(p, b"  Unaligned access handler traps\n\0".as_ptr());
    seq_printf(p, b"%*s: \0".as_ptr(), prec, b"FPA\0".as_ptr());
    for_each_online_cpu!(j, { seq_printf(p, b"%10u \0".as_ptr(), irq_stats(j).irq_fpassist_count); });
    seq_puts(p, b"  Floating point assist traps\n\0".as_ptr());
    seq_printf(p, b"%*s: \0".as_ptr(), prec, b"TLB\0".as_ptr());
    for_each_online_cpu!(j, { seq_printf(p, b"%10u \0".as_ptr(), irq_stats(j).irq_tlb_count); });
    seq_puts(p, b"  TLB shootdowns\n\0".as_ptr()); 0
}

pub unsafe fn show_interrupts(p: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let i = *(v as *mut i64) as i32;
    let mut j: i32;
    let mut flags: usize = 0;
    if i == 0 {
        seq_puts(p, b"    \0".as_ptr());
        for_each_online_cpu!(j, { seq_printf(p, b"       CPU%d\0".as_ptr(), j); });
        seq_putc(p, b'\n' as i32);
    }
    if i < NR_IRQS {
        let desc = irq_to_desc(i);
        raw_spin_lock_irqsave(&mut (*desc).lock, &mut flags);
        let mut action = (*desc).action;
        if !action.is_null() {
            seq_printf(p, b"%3d: \0".as_ptr(), i);
            for_each_online_cpu!(j, { seq_printf(p, b"%10u \0".as_ptr(), irq_desc_kstat_cpu(desc, j)); });
            seq_printf(p, b" %14s  %s\0".as_ptr(), (*irq_desc_get_chip(desc)).name, (*action).name);
            action = (*action).next;
            while !action.is_null() { seq_printf(p, b", %s\0".as_ptr(), (*action).name); action = (*action).next; }
            seq_putc(p, b'\n' as i32);
        }
        raw_spin_unlock_irqrestore(&mut (*desc).lock, flags);
    }
    if i == NR_IRQS { arch_show_interrupts(p, 3); }
    0
}

pub unsafe fn cpu_claim_irq(irq: u32, typ: *mut irq_chip, data: *mut core::ffi::c_void) -> i32 {
    if irq_has_action(irq) || irq_get_chip(irq) != &mut cpu_interrupt_type { return -EBUSY; }
    if !typ.is_null() { irq_set_chip_and_handler(irq, typ, handle_percpu_irq); irq_set_chip_data(irq, data); __cpu_unmask_irq(irq); }
    0
}
pub unsafe fn txn_claim_irq(irq: i32) -> i32 { if cpu_claim_irq(irq as u32, core::ptr::null_mut(), core::ptr::null_mut()) != 0 { -1 } else { irq } }

pub unsafe fn txn_alloc_irq(bits_wide: u32) -> i32 {
    let mut irq = CPU_IRQ_BASE + 1;
    while irq <= CPU_IRQ_MAX { if cpu_claim_irq(irq, core::ptr::null_mut(), core::ptr::null_mut()) >= 0 && irq - CPU_IRQ_BASE < (1u32 << bits_wide) { return irq as i32; } irq += 1; }
    -1
}
pub unsafe fn txn_affinity_addr(irq: u32, cpu: i32) -> usize { #[cfg(CONFIG_SMP)] { let d = irq_get_irq_data(irq); irq_data_update_affinity(d, cpumask_of(cpu)); } per_cpu_cpu_data(cpu).txn_addr }
pub unsafe fn txn_alloc_addr(virt_irq: u32) -> usize { static mut next_cpu: i32 = -1; next_cpu += 1; while next_cpu < nr_cpu_ids && (!per_cpu_cpu_data(next_cpu).txn_addr != 0 || !cpu_online(next_cpu)) { next_cpu += 1; } if next_cpu >= nr_cpu_ids { next_cpu = 0; } txn_affinity_addr(virt_irq, next_cpu) }
pub unsafe fn txn_alloc_data(virt_irq: u32) -> u32 { virt_irq - CPU_IRQ_BASE }
unsafe fn eirr_to_irq(eirr: usize) -> i32 { let bit = fls_long(eirr); (BITS_PER_LONG - bit + TIMER_IRQ as usize) as i32 }

pub static mut sysctl_panic_on_stackoverflow: i32 = 1;
unsafe fn stack_overflow_check(regs: *mut pt_regs) { /* CONFIG_DEBUG_STACKOVERFLOW body is retained by the kernel build. */ let _ = regs; }

pub unsafe fn do_cpu_irq_mask(regs: *mut pt_regs) {
    let old_regs = set_irq_regs(regs); local_irq_disable(); irq_enter_rcu();
    let cpu = smp_processor_id(); let eirr_val = mfctl(23) & CPU_EIEM & *per_cpu_local_ack_eiem(cpu);
    if eirr_val == 0 { set_eiem(CPU_EIEM & *per_cpu_local_ack_eiem(cpu)); irq_exit_rcu(); set_irq_regs(old_regs); return; }
    let irq = eirr_to_irq(eirr_val); let irq_data = irq_get_irq_data(irq as u32);
    if !irq_desc_has_action(irq_data_to_desc(irq_data)) { set_eiem(CPU_EIEM & *per_cpu_local_ack_eiem(cpu)); irq_exit_rcu(); set_irq_regs(old_regs); return; }
    stack_overflow_check(regs); generic_handle_irq(irq as u32); irq_exit_rcu(); set_irq_regs(old_regs);
}

unsafe fn claim_cpu_irqs() {
    let flags = IRQF_TIMER | IRQF_PERCPU | IRQF_IRQPOLL;
    for i in CPU_IRQ_BASE..=CPU_IRQ_MAX { irq_set_chip_and_handler(i, &mut cpu_interrupt_type, handle_percpu_irq); }
    irq_set_handler(TIMER_IRQ, handle_percpu_irq); if request_irq(TIMER_IRQ, timer_interrupt, flags, b"timer\0".as_ptr(), core::ptr::null_mut()) != 0 { pr_err(b"Failed to register timer interrupt\n\0".as_ptr()); }
    #[cfg(CONFIG_SMP)] { irq_set_handler(IPI_IRQ, handle_percpu_irq); request_irq(IPI_IRQ, ipi_interrupt, IRQF_PERCPU, b"IPI\0".as_ptr(), core::ptr::null_mut()); }
}

pub unsafe fn init_IRQ() {
    local_irq_disable(); mtctl(!0usize, 23);
    #[cfg(CONFIG_SMP)] { if CPU_EIEM == 0 { claim_cpu_irqs(); CPU_EIEM = eiem_mask(IPI_IRQ as usize) | eiem_mask(TIMER_IRQ as usize); } }
    #[cfg(not(CONFIG_SMP))] { claim_cpu_irqs(); CPU_EIEM = eiem_mask(TIMER_IRQ as usize); }
    set_eiem(CPU_EIEM);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
