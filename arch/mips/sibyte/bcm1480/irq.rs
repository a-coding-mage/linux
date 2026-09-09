// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000,2001,2002,2003,2004 Broadcom Corporation
 */

// Linux and architecture headers supplying the declarations used below.

#[cfg(CONFIG_PCI)]
extern "C" {
    static mut ht_eoi_space: usize;
}

/* Store the CPU id (not the logical number) */
#[no_mangle]
pub static mut bcm1480_irq_owner: [i32; BCM1480_NR_IRQS as usize] = [0; BCM1480_NR_IRQS as usize];

// DEFINE_RAW_SPINLOCK(bcm1480_imr_lock)
static mut bcm1480_imr_lock: usize = 0;

pub unsafe fn bcm1480_mask_irq(mut cpu: i32, mut irq: i32) {
    let mut flags: usize;
    let mut hl_spacing: usize;
    let mut cur_ints: u64;
    raw_spin_lock_irqsave(&mut bcm1480_imr_lock, &mut flags);
    hl_spacing = 0;
    if irq >= BCM1480_NR_IRQS_HALF && irq <= BCM1480_NR_IRQS {
        hl_spacing = BCM1480_IMR_HL_SPACING;
        irq -= BCM1480_NR_IRQS_HALF;
    }
    cur_ints = ____raw_readq(IOADDR(A_BCM1480_IMR_MAPPER(cpu) + R_BCM1480_IMR_INTERRUPT_MASK_H + hl_spacing));
    cur_ints |= 1u64 << (irq as u32);
    ____raw_writeq(cur_ints, IOADDR(A_BCM1480_IMR_MAPPER(cpu) + R_BCM1480_IMR_INTERRUPT_MASK_H + hl_spacing));
    raw_spin_unlock_irqrestore(&mut bcm1480_imr_lock, flags);
}

pub unsafe fn bcm1480_unmask_irq(mut cpu: i32, mut irq: i32) {
    let mut flags: usize;
    let mut hl_spacing: usize;
    let mut cur_ints: u64;
    raw_spin_lock_irqsave(&mut bcm1480_imr_lock, &mut flags);
    hl_spacing = 0;
    if irq >= BCM1480_NR_IRQS_HALF && irq <= BCM1480_NR_IRQS {
        hl_spacing = BCM1480_IMR_HL_SPACING;
        irq -= BCM1480_NR_IRQS_HALF;
    }
    cur_ints = ____raw_readq(IOADDR(A_BCM1480_IMR_MAPPER(cpu) + R_BCM1480_IMR_INTERRUPT_MASK_H + hl_spacing));
    cur_ints &= !(1u64 << (irq as u32));
    ____raw_writeq(cur_ints, IOADDR(A_BCM1480_IMR_MAPPER(cpu) + R_BCM1480_IMR_INTERRUPT_MASK_H + hl_spacing));
    raw_spin_unlock_irqrestore(&mut bcm1480_imr_lock, flags);
}

#[cfg(CONFIG_SMP)]
unsafe fn bcm1480_set_affinity(d: *const irq_data, mask: *const cpumask, _force: bool) -> i32 {
    let irq = (*d).irq;
    let i = cpumask_first_and(mask, cpu_online_mask);
    let cpu = cpu_logical_map(i);
    let mut flags: usize;
    raw_spin_lock_irqsave(&mut bcm1480_imr_lock, &mut flags);
    let old_cpu = bcm1480_irq_owner[irq as usize];
    let mut irq_dirty = irq;
    if irq_dirty >= BCM1480_NR_IRQS_HALF && irq_dirty <= BCM1480_NR_IRQS { irq_dirty -= BCM1480_NR_IRQS_HALF; }
    for k in 0..2 {
        let off = k * BCM1480_IMR_HL_SPACING;
        let mut cur_ints = ____raw_readq(IOADDR(A_BCM1480_IMR_MAPPER(old_cpu) + R_BCM1480_IMR_INTERRUPT_MASK_H + off));
        let int_on = (cur_ints & (1u64 << irq_dirty as u32)) == 0;
        if int_on {
            cur_ints |= 1u64 << irq_dirty as u32;
            ____raw_writeq(cur_ints, IOADDR(A_BCM1480_IMR_MAPPER(old_cpu) + R_BCM1480_IMR_INTERRUPT_MASK_H + off));
        }
        bcm1480_irq_owner[irq as usize] = cpu;
        if int_on {
            cur_ints = ____raw_readq(IOADDR(A_BCM1480_IMR_MAPPER(cpu) + R_BCM1480_IMR_INTERRUPT_MASK_H + off));
            cur_ints &= !(1u64 << irq_dirty as u32);
            ____raw_writeq(cur_ints, IOADDR(A_BCM1480_IMR_MAPPER(cpu) + R_BCM1480_IMR_INTERRUPT_MASK_H + off));
        }
    }
    raw_spin_unlock_irqrestore(&mut bcm1480_imr_lock, flags);
    0
}

unsafe fn disable_bcm1480_irq(d: *const irq_data) { let irq = (*d).irq; bcm1480_mask_irq(bcm1480_irq_owner[irq as usize], irq); }
unsafe fn enable_bcm1480_irq(d: *const irq_data) { let irq = (*d).irq; bcm1480_unmask_irq(bcm1480_irq_owner[irq as usize], irq); }

unsafe fn ack_bcm1480_irq(d: *const irq_data) {
    let irq = (*d).irq;
    let mut irq_dirty = irq;
    if irq_dirty >= BCM1480_NR_IRQS_HALF && irq_dirty <= BCM1480_NR_IRQS { irq_dirty -= BCM1480_NR_IRQS_HALF; }
    for k in 0..2 {
        let off = k * BCM1480_IMR_HL_SPACING;
        let mut pending = __raw_readq(IOADDR(A_BCM1480_IMR_REGISTER(bcm1480_irq_owner[irq as usize], R_BCM1480_IMR_LDT_INTERRUPT_H + off)));
        pending &= 1u64 << irq_dirty as u32;
        if pending != 0 {
            #[cfg(CONFIG_SMP)]
            for i in 0..NR_CPUS { __raw_writeq(pending, IOADDR(A_BCM1480_IMR_REGISTER(cpu_logical_map(i), R_BCM1480_IMR_LDT_INTERRUPT_CLR_H + off))); }
            #[cfg(not(CONFIG_SMP))]
            __raw_writeq(pending, IOADDR(A_BCM1480_IMR_REGISTER(0, R_BCM1480_IMR_LDT_INTERRUPT_CLR_H + off)));
            #[cfg(CONFIG_PCI)]
            if ht_eoi_space != 0 { *((ht_eoi_space + ((irq as usize) << 16) + (7 << 2)) as *mut u32) = 0; }
        }
    }
    bcm1480_mask_irq(bcm1480_irq_owner[irq as usize], irq);
}

// struct irq_chip bcm1480_irq_type
static mut bcm1480_irq_type: irq_chip = irq_chip {
    name: "BCM1480-IMR", irq_mask_ack: Some(ack_bcm1480_irq), irq_mask: Some(disable_bcm1480_irq),
    irq_unmask: Some(enable_bcm1480_irq),
    #[cfg(CONFIG_SMP)] irq_set_affinity: Some(bcm1480_set_affinity),
};

pub unsafe fn init_bcm1480_irqs() {
    for i in 0..BCM1480_NR_IRQS { irq_set_chip_and_handler(i, &mut bcm1480_irq_type, handle_level_irq); bcm1480_irq_owner[i as usize] = 0; }
}

// Build-time interrupt-map constants.
const IMR_IP2_VAL: u64 = K_BCM1480_INT_MAP_I0;
const IMR_IP3_VAL: u64 = K_BCM1480_INT_MAP_I1;

pub unsafe fn arch_init_irq() {
    let imask = STATUSF_IP4 | STATUSF_IP3 | STATUSF_IP2 | STATUSF_IP1 | STATUSF_IP0;
    for i in 1..BCM1480_NR_IRQS_HALF { for cpu in 0..4 { __raw_writeq(IMR_IP2_VAL, IOADDR(A_BCM1480_IMR_REGISTER(cpu, R_BCM1480_IMR_INTERRUPT_MAP_BASE_H) + (i << 3))); } }
    for i in 0..BCM1480_NR_IRQS_HALF { for cpu in 0..4 { __raw_writeq(IMR_IP2_VAL, IOADDR(A_BCM1480_IMR_REGISTER(cpu, R_BCM1480_IMR_INTERRUPT_MAP_BASE_L) + (i << 3))); } }
    init_bcm1480_irqs();
    for cpu in 0..4 { __raw_writeq(IMR_IP3_VAL, IOADDR(A_BCM1480_IMR_REGISTER(cpu, R_BCM1480_IMR_INTERRUPT_MAP_BASE_H) + (K_BCM1480_INT_MBOX_0_0 << 3))); }
    for cpu in 0..4 { __raw_writeq(0xffffffffffffffff, IOADDR(A_BCM1480_IMR_REGISTER(cpu, R_BCM1480_IMR_MAILBOX_0_CLR_CPU))); __raw_writeq(0xffffffffffffffff, IOADDR(A_BCM1480_IMR_REGISTER(cpu, R_BCM1480_IMR_MAILBOX_1_CLR_CPU))); }
    let tmp = !0u64 ^ (1u64 << K_BCM1480_INT_MBOX_0_0);
    for cpu in 0..4 { __raw_writeq(tmp, IOADDR(A_BCM1480_IMR_REGISTER(cpu, R_BCM1480_IMR_INTERRUPT_MASK_H))); }
    for cpu in 0..4 { __raw_writeq(!0u64, IOADDR(A_BCM1480_IMR_REGISTER(cpu, R_BCM1480_IMR_INTERRUPT_MASK_L))); }
    change_c0_status(ST0_IM, imask);
}

extern "C" { fn bcm1480_mailbox_interrupt(); }

unsafe fn dispatch_ip2() {
    let cpu = smp_processor_id();
    let mask_h = __raw_readq(IOADDR(A_BCM1480_IMR_MAPPER(cpu) + R_BCM1480_IMR_INTERRUPT_STATUS_BASE_H));
    let mask_l = __raw_readq(IOADDR(A_BCM1480_IMR_MAPPER(cpu) + R_BCM1480_IMR_INTERRUPT_STATUS_BASE_L));
    if mask_h != 0 { if mask_h ^ 1 != 0 { do_IRQ(fls64(mask_h) - 1); } else if mask_l != 0 { do_IRQ(63 + fls64(mask_l)); } }
}

pub unsafe extern "C" fn plat_irq_dispatch() {
    let pending = read_c0_cause() & read_c0_status();
    if pending & CAUSEF_IP4 != 0 { do_IRQ(K_BCM1480_INT_TIMER_0 + smp_processor_id()); }
    #[cfg(CONFIG_SMP)]
    else if pending & CAUSEF_IP3 != 0 { bcm1480_mailbox_interrupt(); }
    else if pending & CAUSEF_IP2 != 0 { dispatch_ip2(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
