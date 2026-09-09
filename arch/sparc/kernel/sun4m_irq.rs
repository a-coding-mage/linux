// SPDX-License-Identifier: GPL-2.0
/*
 * sun4m irq support
 *
 *  djhr: Hacked out of irq.c into a CPU dependent version.
 *
 *  Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 *  Copyright (C) 1995 Miguel de Icaza (miguel@nuclecu.unam.mx)
 *  Copyright (C) 1995 Pete A. Zaitcev (zaitcev@yahoo.com)
 *  Copyright (C) 1996 Dave Redman (djhr@tadpole.co.uk)
 */

// C headers and kernel headers are supplied by the surrounding translation unit.

/* Code in entry.S needs to get at these register mappings. */
pub static mut sun4m_irq_percpu: [*mut sun4m_irq_percpu; SUN4M_NCPUS] = [core::ptr::null_mut(); SUN4M_NCPUS];
pub static mut sun4m_irq_global: *mut sun4m_irq_global = core::ptr::null_mut();

#[repr(C)]
pub struct sun4m_handler_data {
    pub percpu: bool,
    pub mask: core::ffi::c_long,
}

pub const SUN4M_INT_ENABLE: u32 = 0x80000000;
pub const SUN4M_INT_E14: u32 = 0x00000080;
pub const SUN4M_INT_E10: u32 = 0x00080000;
pub const SUN4M_INT_MASKALL: u32 = 0x80000000;
pub const SUN4M_INT_MODULE_ERR: u32 = 0x40000000;
pub const SUN4M_INT_M2S_WRITE_ERR: u32 = 0x20000000;
pub const SUN4M_INT_ECC_ERR: u32 = 0x10000000;
pub const SUN4M_INT_VME_ERR: u32 = 0x08000000;
pub const SUN4M_INT_FLOPPY: u32 = 0x00400000;
pub const SUN4M_INT_MODULE: u32 = 0x00200000;
pub const SUN4M_INT_VIDEO: u32 = 0x00100000;
pub const SUN4M_INT_REALTIME: u32 = 0x00080000;
pub const SUN4M_INT_SCSI: u32 = 0x00040000;
pub const SUN4M_INT_AUDIO: u32 = 0x00020000;
pub const SUN4M_INT_ETHERNET: u32 = 0x00010000;
pub const SUN4M_INT_SERIAL: u32 = 0x00008000;
pub const SUN4M_INT_KBDMS: u32 = 0x00004000;
pub const SUN4M_INT_SBUSBITS: u32 = 0x00003F80;
pub const SUN4M_INT_VMEBITS: u32 = 0x0000007F;
pub const SUN4M_INT_ERROR: u32 = SUN4M_INT_MODULE_ERR | SUN4M_INT_M2S_WRITE_ERR | SUN4M_INT_ECC_ERR | SUN4M_INT_VME_ERR;
pub const fn SUN4M_INT_SBUS(x: u32) -> u32 { 1u32 << (x + 7) }
pub const fn SUN4M_INT_VME(x: u32) -> u32 { 1u32 << x }
pub const OBP_INT_LEVEL_SOFT: u32 = 0x10;
pub const OBP_INT_LEVEL_ONBOARD: u32 = 0x20;
pub const OBP_INT_LEVEL_SBUS: u32 = 0x30;
pub const OBP_INT_LEVEL_VME: u32 = 0x40;
pub const SUN4M_TIMER_IRQ: u32 = OBP_INT_LEVEL_ONBOARD | 10;
pub const SUN4M_PROFILE_IRQ: u32 = OBP_INT_LEVEL_ONBOARD | 14;

const fn sun4m_soft_int(x: u32) -> u32 { 1u32 << x }

static mut sun4m_imask: [u64; 0x50] = [
    0, sun4m_soft_int(1), sun4m_soft_int(2), sun4m_soft_int(3), sun4m_soft_int(4), sun4m_soft_int(5), sun4m_soft_int(6), sun4m_soft_int(7), sun4m_soft_int(8), sun4m_soft_int(9), sun4m_soft_int(10), sun4m_soft_int(11), sun4m_soft_int(12), sun4m_soft_int(13), sun4m_soft_int(14), sun4m_soft_int(15),
    0, sun4m_soft_int(1), sun4m_soft_int(2), sun4m_soft_int(3), sun4m_soft_int(4), sun4m_soft_int(5), sun4m_soft_int(6), sun4m_soft_int(7), sun4m_soft_int(8), sun4m_soft_int(9), sun4m_soft_int(10), sun4m_soft_int(11), sun4m_soft_int(12), sun4m_soft_int(13), sun4m_soft_int(14), sun4m_soft_int(15),
    0, 0, 0, 0, SUN4M_INT_SCSI as u64, 0, SUN4M_INT_ETHERNET as u64, 0, SUN4M_INT_VIDEO as u64, SUN4M_INT_MODULE as u64, SUN4M_INT_REALTIME as u64, SUN4M_INT_FLOPPY as u64, (SUN4M_INT_SERIAL | SUN4M_INT_KBDMS) as u64, SUN4M_INT_AUDIO as u64, SUN4M_INT_E14 as u64, SUN4M_INT_MODULE_ERR as u64,
    0, 0, SUN4M_INT_SBUS(0) as u64, SUN4M_INT_SBUS(1) as u64, 0, SUN4M_INT_SBUS(2) as u64, 0, SUN4M_INT_SBUS(3) as u64, 0, SUN4M_INT_SBUS(4) as u64, 0, SUN4M_INT_SBUS(5) as u64, 0, SUN4M_INT_SBUS(6) as u64, 0, 0,
    0, 0, SUN4M_INT_VME(0) as u64, SUN4M_INT_VME(1) as u64, 0, SUN4M_INT_VME(2) as u64, 0, SUN4M_INT_VME(3) as u64, 0, SUN4M_INT_VME(4) as u64, 0, SUN4M_INT_VME(5) as u64, 0, SUN4M_INT_VME(6) as u64, 0, 0,
];

unsafe fn sun4m_mask_irq(data: *mut irq_data) {
    let h = irq_data_get_irq_handler_data(data) as *mut sun4m_handler_data;
    let cpu = smp_processor_id();
    if (*h).mask != 0 { let mut flags = 0u64; local_irq_save(&mut flags); if (*h).percpu { sbus_writel((*h).mask as u32, &mut (*sun4m_irq_percpu[cpu as usize]).set); } else { sbus_writel((*h).mask as u32, &mut (*sun4m_irq_global).mask_set); } local_irq_restore(flags); }
}

unsafe fn sun4m_unmask_irq(data: *mut irq_data) {
    let h = irq_data_get_irq_handler_data(data) as *mut sun4m_handler_data;
    let cpu = smp_processor_id();
    if (*h).mask != 0 { let mut flags = 0u64; local_irq_save(&mut flags); if (*h).percpu { sbus_writel((*h).mask as u32, &mut (*sun4m_irq_percpu[cpu as usize]).clear); } else { sbus_writel((*h).mask as u32, &mut (*sun4m_irq_global).mask_clear); } local_irq_restore(flags); }
}

unsafe fn sun4m_startup_irq(data: *mut irq_data) -> u32 { irq_link((*data).irq); sun4m_unmask_irq(data); 0 }
unsafe fn sun4m_shutdown_irq(data: *mut irq_data) { sun4m_mask_irq(data); irq_unlink((*data).irq); }

static mut sun4m_irq: irq_chip = irq_chip { name: "sun4m", irq_startup: Some(sun4m_startup_irq), irq_shutdown: Some(sun4m_shutdown_irq), irq_mask: Some(sun4m_mask_irq), irq_unmask: Some(sun4m_unmask_irq) };

unsafe fn sun4m_build_device_irq(_op: *mut platform_device, real_irq: u32) -> u32 {
    if real_irq >= OBP_INT_LEVEL_VME { prom_printf("Bogus sun4m IRQ %u\n", real_irq); prom_halt(); }
    let pil = real_irq & 0xf; let irq = irq_alloc(real_irq, pil); if irq == 0 { return irq; }
    let mut h = irq_get_handler_data(irq) as *mut sun4m_handler_data;
    if !h.is_null() { return irq; }
    h = kzalloc_obj::<sun4m_handler_data>(GFP_ATOMIC); if h.is_null() { prom_printf("IRQ: kzalloc(sun4m_handler_data) failed.\n"); prom_halt(); }
    (*h).mask = sun4m_imask[real_irq as usize] as core::ffi::c_long; (*h).percpu = real_irq < OBP_INT_LEVEL_ONBOARD;
    irq_set_chip_and_handler_name(irq, &mut sun4m_irq, handle_level_irq, "level"); irq_set_handler_data(irq, h as *mut core::ffi::c_void); irq
}

#[repr(C)] pub struct sun4m_timer_percpu { pub l14_limit: u32, pub l14_count: u32, pub l14_limit_noclear: u32, pub user_timer_start_stop: u32 }
static mut timers_percpu: [*mut sun4m_timer_percpu; SUN4M_NCPUS] = [core::ptr::null_mut(); SUN4M_NCPUS];
#[repr(C)] pub struct sun4m_timer_global { pub l10_limit: u32, pub l10_count: u32, pub l10_limit_noclear: u32, pub reserved: u32, pub timer_config: u32 }
static mut timers_global: *mut sun4m_timer_global = core::ptr::null_mut();

unsafe fn sun4m_clear_clock_irq() { sbus_readl(&(*timers_global).l10_limit); }

pub unsafe fn sun4m_nmi(regs: *mut pt_regs) {
    let (afsr, afar) = (0u64, 0u64); printk(KERN_ERR, "Aieee: sun4m NMI received!\n");
    // The original HyperSparc inline assembly reads AFSR and AFAR from address spaces 4.
    printk(KERN_ERR, "afsr=%08lx afar=%08lx\n", afsr, afar); let si = sbus_readl(&(*sun4m_irq_global).pending) as u64; printk(KERN_ERR, "si=%08lx\n", si);
    if si & SUN4M_INT_MODULE_ERR as u64 != 0 { printk(KERN_ERR, "Module async error\n"); } if si & SUN4M_INT_M2S_WRITE_ERR as u64 != 0 { printk(KERN_ERR, "MBus/SBus async error\n"); } if si & SUN4M_INT_ECC_ERR as u64 != 0 { printk(KERN_ERR, "ECC memory error\n"); } if si & SUN4M_INT_VME_ERR as u64 != 0 { printk(KERN_ERR, "VME async error\n"); }
    printk(KERN_ERR, "you lose buddy boy...\n"); show_regs(regs); prom_halt();
}

pub unsafe fn sun4m_unmask_profile_irq() { let mut flags = 0u64; local_irq_save(&mut flags); sbus_writel(sun4m_imask[SUN4M_PROFILE_IRQ as usize] as u32, &mut (*sun4m_irq_global).mask_clear); local_irq_restore(flags); }
pub unsafe fn sun4m_clear_profile_irq(cpu: i32) { sbus_readl(&(*timers_percpu[cpu as usize]).l14_limit); }
unsafe fn sun4m_load_profile_irq(cpu: i32, limit: u32) { let value = if limit != 0 { timer_value(limit) } else { 0 }; sbus_writel(value, &mut (*timers_percpu[cpu as usize]).l14_limit); }

unsafe fn sun4m_init_timers() {
    let dp = of_find_node_by_name(core::ptr::null_mut(), "counter"); if dp.is_null() { printk(KERN_ERR, "sun4m_init_timers: No 'counter' node.\n"); return; }
    let mut len = 0; let addr = of_get_property(dp, "address", &mut len); of_node_put(dp); if addr.is_null() { printk(KERN_ERR, "sun4m_init_timers: No 'address' prop.\n"); return; }
    let num_cpu_timers = (len / core::mem::size_of::<u32>() as i32) - 1; for i in 0..num_cpu_timers { timers_percpu[i as usize] = (*addr.add(i as usize)) as usize as *mut sun4m_timer_percpu; } timers_global = (*addr.add(num_cpu_timers as usize)) as usize as *mut sun4m_timer_global;
    sbus_writel(0, &mut (*timers_global).timer_config);
    #[cfg(feature = "CONFIG_SMP")] { sparc_config.cs_period = SBUS_CLOCK_RATE * 2; sparc_config.features |= FEAT_L14_ONESHOT; }
    #[cfg(not(feature = "CONFIG_SMP"))] { sparc_config.cs_period = SBUS_CLOCK_RATE / HZ; sparc_config.features |= FEAT_L10_CLOCKEVENT; }
    sparc_config.features |= FEAT_L10_CLOCKSOURCE; sbus_writel(timer_value(sparc_config.cs_period), &mut (*timers_global).l10_limit); master_l10_counter = &mut (*timers_global).l10_count;
    let irq = sun4m_build_device_irq(core::ptr::null_mut(), SUN4M_TIMER_IRQ); let err = request_irq(irq, timer_interrupt, IRQF_TIMER, "timer", core::ptr::null_mut()); if err != 0 { printk(KERN_ERR, "sun4m_init_timers: Register IRQ error %d.\n", err); return; }
    for i in 0..num_cpu_timers { sbus_writel(0, &mut (*timers_percpu[i as usize]).l14_limit); } if num_cpu_timers == 4 { sbus_writel(SUN4M_INT_E14, &mut (*sun4m_irq_global).mask_set); }
}

pub unsafe fn sun4m_init_IRQ() {
    let dp = of_find_node_by_name(core::ptr::null_mut(), "interrupt"); if dp.is_null() { printk(KERN_ERR, "sun4m_init_IRQ: No 'interrupt' node.\n"); return; }
    let mut len = 0; let addr = of_get_property(dp, "address", &mut len); of_node_put(dp); if addr.is_null() { printk(KERN_ERR, "sun4m_init_IRQ: No 'address' prop.\n"); return; }
    let num_cpu_iregs = (len / core::mem::size_of::<u32>() as i32) - 1; for i in 0..num_cpu_iregs { sun4m_irq_percpu[i as usize] = (*addr.add(i as usize)) as usize as *mut sun4m_irq_percpu; } sun4m_irq_global = (*addr.add(num_cpu_iregs as usize)) as usize as *mut sun4m_irq_global;
    local_irq_disable(); sbus_writel(!SUN4M_INT_MASKALL, &mut (*sun4m_irq_global).mask_set); let mut i = 0; let mut mid = 0; while !cpu_find_by_instance(i, core::ptr::null_mut(), &mut mid) { sbus_writel(!0x17fff, &mut (*sun4m_irq_percpu[mid as usize]).clear); i += 1; }
    if num_cpu_iregs == 4 { sbus_writel(0, &mut (*sun4m_irq_global).interrupt_target); }
    sparc_config.init_timers = Some(sun4m_init_timers); sparc_config.build_device_irq = Some(sun4m_build_device_irq); sparc_config.clock_rate = SBUS_CLOCK_RATE; sparc_config.clear_clock_irq = Some(sun4m_clear_clock_irq); sparc_config.load_profile_irq = Some(sun4m_load_profile_irq);
    /* Cannot enable interrupts until OBP ticker is disabled. */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
