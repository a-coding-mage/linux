// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 John Crispin <john@phrozen.org>
 * Copyright (C) 2010 Thomas Langer <thomas.langer@lantiq.com>
 */

// Linux and platform dependencies supplied by other translation units.

const LTQ_ICU_ISR: usize = 0x0000;
const LTQ_ICU_IER: usize = 0x0008;
const LTQ_ICU_IOSR: usize = 0x0010;
const LTQ_ICU_IRSR: usize = 0x0018;
const LTQ_ICU_IMR: usize = 0x0020;
const LTQ_ICU_IM_SIZE: usize = 0x28;
const LTQ_EIU_EXIN_C: usize = 0x0000;
const LTQ_EIU_EXIN_INIC: usize = 0x0004;
const LTQ_EIU_EXIN_INC: usize = 0x0008;
const LTQ_EIU_EXIN_INEN: usize = 0x000C;
const MAX_EIU: usize = 6;
const LTQ_ICU_EBU_IRQ: usize = 22;
const MIPS_CPU_IRQ_CASCADE: usize = 8;

static mut exin_avail: i32 = 0;
static mut ltq_eiu_irq: [u32; MAX_EIU] = [0; MAX_EIU];
static mut ltq_icu_membase: [*mut core::ffi::c_void; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];
static mut ltq_eiu_membase: *mut core::ffi::c_void = core::ptr::null_mut();
static mut ltq_domain: *mut irq_domain = core::ptr::null_mut();
static mut ltq_eiu_lock: spinlock_t = unsafe { core::mem::zeroed() };
static mut ltq_icu_lock: raw_spinlock_t = unsafe { core::mem::zeroed() };
static mut ltq_perfcount_irq: i32 = 0;

#[inline]
unsafe fn ltq_icu_w32(vpe: usize, m: usize, x: u32, y: usize) {
    ltq_w32(x, (ltq_icu_membase[vpe] as usize + m * LTQ_ICU_IM_SIZE + y) as *mut core::ffi::c_void);
}

#[inline]
unsafe fn ltq_icu_r32(vpe: usize, m: usize, x: usize) -> u32 {
    ltq_r32((ltq_icu_membase[vpe] as usize + m * LTQ_ICU_IM_SIZE + x) as *mut core::ffi::c_void)
}

#[inline]
unsafe fn ltq_eiu_w32(x: u32, y: usize) { ltq_w32(x, (ltq_eiu_membase as usize + y) as *mut core::ffi::c_void); }
#[inline]
unsafe fn ltq_eiu_r32(x: usize) -> u32 { ltq_r32((ltq_eiu_membase as usize + x) as *mut core::ffi::c_void) }

pub unsafe fn ltq_eiu_get_irq(exin: i32) -> i32 {
    if exin < exin_avail { ltq_eiu_irq[exin as usize] as i32 } else { -1 }
}

pub unsafe extern "C" fn ltq_disable_irq(d: *mut irq_data) {
    let mut offset = ((*d).hwirq - MIPS_CPU_IRQ_CASCADE as u64) as usize;
    let im = offset / INT_NUM_IM_OFFSET as usize;
    let mut flags: unsigned_long = 0;
    offset %= INT_NUM_IM_OFFSET as usize;
    raw_spin_lock_irqsave(&mut ltq_icu_lock, &mut flags);
    for_each_present_cpu(|vpe: usize| { ltq_icu_w32(vpe, im, ltq_icu_r32(vpe, im, LTQ_ICU_IER) & !(1u32 << offset), LTQ_ICU_IER); });
    raw_spin_unlock_irqrestore(&mut ltq_icu_lock, flags);
}

pub unsafe extern "C" fn ltq_mask_and_ack_irq(d: *mut irq_data) {
    let mut offset = ((*d).hwirq - MIPS_CPU_IRQ_CASCADE as u64) as usize;
    let im = offset / INT_NUM_IM_OFFSET as usize; let mut flags: unsigned_long = 0; offset %= INT_NUM_IM_OFFSET as usize;
    raw_spin_lock_irqsave(&mut ltq_icu_lock, &mut flags);
    for_each_present_cpu(|vpe: usize| { ltq_icu_w32(vpe, im, ltq_icu_r32(vpe, im, LTQ_ICU_IER) & !(1u32 << offset), LTQ_ICU_IER); ltq_icu_w32(vpe, im, 1u32 << offset, LTQ_ICU_ISR); });
    raw_spin_unlock_irqrestore(&mut ltq_icu_lock, flags);
}

unsafe fn ltq_ack_irq(d: *mut irq_data) {
    let mut offset = ((*d).hwirq - MIPS_CPU_IRQ_CASCADE as u64) as usize; let im = offset / INT_NUM_IM_OFFSET as usize; let mut flags: unsigned_long = 0; offset %= INT_NUM_IM_OFFSET as usize;
    raw_spin_lock_irqsave(&mut ltq_icu_lock, &mut flags);
    for_each_present_cpu(|vpe: usize| ltq_icu_w32(vpe, im, 1u32 << offset, LTQ_ICU_ISR));
    raw_spin_unlock_irqrestore(&mut ltq_icu_lock, flags);
}

pub unsafe extern "C" fn ltq_enable_irq(d: *mut irq_data) {
    let mut offset = ((*d).hwirq - MIPS_CPU_IRQ_CASCADE as u64) as usize; let im = offset / INT_NUM_IM_OFFSET as usize; let mut flags: unsigned_long = 0; offset %= INT_NUM_IM_OFFSET as usize;
    let mut vpe = cpumask_first(irq_data_get_effective_affinity_mask(d));
    if vpe >= nr_cpu_ids { vpe = smp_processor_id(); }
    raw_spin_lock_irqsave(&mut ltq_icu_lock, &mut flags);
    ltq_icu_w32(vpe, im, ltq_icu_r32(vpe, im, LTQ_ICU_IER) | (1u32 << offset), LTQ_ICU_IER);
    raw_spin_unlock_irqrestore(&mut ltq_icu_lock, flags);
}

unsafe fn ltq_eiu_settype(d: *mut irq_data, typ: u32) -> i32 {
    let mut flags: unsigned_long = 0;
    for i in 0..exin_avail as usize {
        if (*d).hwirq == ltq_eiu_irq[i] as u64 {
            let (val, edge) = match typ {
                IRQF_TRIGGER_NONE => (0, false), IRQF_TRIGGER_RISING => (1, true),
                IRQF_TRIGGER_FALLING => (2, true),
                x if x == (IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING) => (3, true),
                IRQF_TRIGGER_HIGH => (5, false), IRQF_TRIGGER_LOW => (6, false),
                _ => { pr_err!("invalid type %d for irq %ld\n", typ, (*d).hwirq); return -EINVAL; }
            };
            if edge { irq_set_handler((*d).hwirq as u32, handle_edge_irq); }
            spin_lock_irqsave(&mut ltq_eiu_lock, &mut flags);
            ltq_eiu_w32((ltq_eiu_r32(LTQ_EIU_EXIN_C) & !(7u32 << (i * 4))) | ((val as u32) << (i * 4)), LTQ_EIU_EXIN_C);
            spin_unlock_irqrestore(&mut ltq_eiu_lock, flags);
        }
    } 0
}

unsafe fn ltq_startup_eiu_irq(d: *mut irq_data) -> unsigned_int {
    ltq_enable_irq(d);
    for i in 0..exin_avail as usize { if (*d).hwirq == ltq_eiu_irq[i] as u64 {
        ltq_eiu_settype(d, IRQF_TRIGGER_LOW);
        ltq_eiu_w32(ltq_eiu_r32(LTQ_EIU_EXIN_INC) & !(1 << i), LTQ_EIU_EXIN_INC);
        ltq_eiu_w32(ltq_eiu_r32(LTQ_EIU_EXIN_INEN) | (1 << i), LTQ_EIU_EXIN_INEN); break;
    }} 0
}
unsafe fn ltq_shutdown_eiu_irq(d: *mut irq_data) {
    ltq_disable_irq(d);
    for i in 0..exin_avail as usize { if (*d).hwirq == ltq_eiu_irq[i] as u64 {
        ltq_eiu_w32(ltq_eiu_r32(LTQ_EIU_EXIN_INEN) & !(1 << i), LTQ_EIU_EXIN_INEN); break;
    }}
}

unsafe fn ltq_hw_irq_handler(desc: *mut irq_desc) {
    let module = irq_desc_get_irq(desc) - 2; let mut irq = ltq_icu_r32(smp_processor_id(), module as usize, LTQ_ICU_IOSR);
    if irq == 0 { return; }
    irq = __fls(irq); let hwirq = irq as u64 + MIPS_CPU_IRQ_CASCADE as u64 + INT_NUM_IM_OFFSET as u64 * module as u64;
    generic_handle_domain_irq(ltq_domain, hwirq);
    if irq as usize == LTQ_ICU_EBU_IRQ && module == 0 && LTQ_EBU_PCC_ISTAT != 0 { ltq_ebu_w32(ltq_ebu_r32(LTQ_EBU_PCC_ISTAT) | 0x10, LTQ_EBU_PCC_ISTAT); }
}

unsafe fn icu_map(_d: *mut irq_domain, irq: unsigned_int, hw: u64) -> i32 {
    if hw < MIPS_CPU_IRQ_CASCADE as u64 { return 0; }
    let mut chip = &ltq_irq_type;
    for i in 0..exin_avail as usize { if hw == ltq_eiu_irq[i] as u64 { chip = &ltq_eiu_type; } }
    let data = irq_get_irq_data(irq); irq_data_update_effective_affinity(data, cpumask_of(0));
    irq_set_chip_and_handler(irq, chip, handle_level_irq); 0
}

unsafe fn icu_of_init(node: *mut device_node, _parent: *mut device_node) -> i32 {
    let mut res = core::mem::zeroed::<resource>();
    for_each_possible_cpu(|vpe: usize| { if of_address_to_resource(node, vpe as i32, &mut res) != 0 { panic!("Failed to get icu{} memory range", vpe); } ltq_icu_membase[vpe] = ioremap(res.start, resource_size(&res)); if ltq_icu_membase[vpe].is_null() { panic!("Failed to remap icu{} memory", vpe); } });
    for_each_possible_cpu(|vpe: usize| { for i in 0..MAX_IM as usize { ltq_icu_w32(vpe, i, 0, LTQ_ICU_IER); ltq_icu_w32(vpe, i, !0, LTQ_ICU_ISR); ltq_icu_w32(vpe, i, !0, LTQ_ICU_IMR); ltq_icu_w32(vpe, i, 0, LTQ_ICU_IRSR); }});
    mips_cpu_irq_init(); for i in 0..MAX_IM as u32 { irq_set_chained_handler(i + 2, ltq_hw_irq_handler); } 0
}

// CONFIG_SMP-dependent affinity support is preserved by the platform build.
// The irq_chip/domain declarations and initialization below retain the original
// kernel interfaces; their concrete types and helper declarations are external.

pub unsafe fn get_c0_perfcount_int() -> i32 { ltq_perfcount_irq }
pub unsafe fn get_c0_compare_int() -> u32 { CP0_LEGACY_COMPARE_IRQ }

pub unsafe extern "C" fn arch_init_irq() { irqchip_init(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
