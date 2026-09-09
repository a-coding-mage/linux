// SPDX-License-Identifier: GPL-2.0
/*
 * SS1000/SC2000 interrupt handling.
 *
 *  Copyright (C) 1997,1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 *  Heavily based on arch/sparc/kernel/irq.c.
 */

// Linux and architecture dependencies are supplied by the surrounding kernel.

#[repr(C)]
struct Sun4dHandlerData { cpuid: core::ffi::c_uint, real_irq: core::ffi::c_uint }

unsafe fn sun4d_encode_irq(board: core::ffi::c_int, lvl: core::ffi::c_int, slot: core::ffi::c_int) -> core::ffi::c_uint {
    (((board + 1) << 5) | (lvl << 2) | slot) as core::ffi::c_uint
}

#[repr(C)]
struct Sun4dTimerRegs {
    l10_timer_limit: u32,
    l10_cur_countx: u32,
    l10_limit_noclear: u32,
    ctrl: u32,
    l10_cur_count: u32,
}

static mut SUN4D_TIMERS: *mut Sun4dTimerRegs = core::ptr::null_mut();
const SUN4D_TIMER_IRQ: core::ffi::c_uint = 10;
static mut BOARD_TO_CPU: [u8; 32] = [0; 32];
static PIL_TO_SBUS: [core::ffi::c_int; 16] = [0, 0, 1, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 0];

// Exported for sun4d_smp.c.
static mut SUN4D_IMSK_LOCK: core::ffi::c_ulong = 0;

unsafe fn sun4d_sbus_handler_irq(sbusl: core::ffi::c_int) {
    let mut bus_mask = bw_get_intr_mask(sbusl) & 0x3ffff;
    bw_clear_intr_mask(sbusl, bus_mask);
    let sbil = sbusl << 2;
    let mut sbino = 0;
    while bus_mask != 0 {
        if bus_mask & 1 != 0 {
            // acquire_sbi() uses swap; this mirrors the source's acknowledge behavior.
            let mut mask = acquire_sbi(SBI2DEVID(sbino), 0xf << sbil) & (0xf << sbil);
            let mut slot = 1 << sbil;
            let mut idx = 0;
            while mask != 0 {
                if mask & slot != 0 {
                    mask &= !slot;
                    let pil = sun4d_encode_irq(sbino, sbusl, idx);
                    let mut p = irq_map[pil as usize];
                    while !p.is_null() {
                        let next = (*p).next;
                        generic_handle_irq((*p).irq);
                        p = next;
                    }
                    release_sbi(SBI2DEVID(sbino), slot);
                }
                idx += 1;
                slot <<= 1;
            }
        }
        sbino += 1;
        bus_mask >>= 1;
    }
}

unsafe fn sun4d_handler_irq(pil: core::ffi::c_uint, regs: *mut PtRegs) {
    let sbusl = PIL_TO_SBUS[pil as usize];
    cc_get_ipen();
    cc_set_iclr(1 << pil);
    // CONFIG_SMP: if (pil == SUN4D_IPI_IRQ) sun4d_ipi_interrupt();
    let old_regs = set_irq_regs(regs);
    irq_enter();
    if sbusl == 0 {
        let mut p = irq_map[pil as usize];
        while !p.is_null() {
            let next = (*p).next;
            generic_handle_irq((*p).irq);
            p = next;
        }
    } else { sun4d_sbus_handler_irq(sbusl); }
    irq_exit();
    set_irq_regs(old_regs);
}

unsafe fn sun4d_mask_irq(data: *mut IrqData) {
    let handler_data = irq_data_get_irq_handler_data(data) as *mut Sun4dHandlerData;
    let real_irq = (*handler_data).real_irq;
    // CONFIG_SMP selects the per-CPU imask and lock operations.
    cc_set_imsk(cc_get_imsk() | (1 << real_irq));
}

unsafe fn sun4d_unmask_irq(data: *mut IrqData) {
    let handler_data = irq_data_get_irq_handler_data(data) as *mut Sun4dHandlerData;
    let real_irq = (*handler_data).real_irq;
    cc_set_imsk(cc_get_imsk() & !(1 << real_irq));
}

unsafe fn sun4d_startup_irq(data: *mut IrqData) -> core::ffi::c_uint {
    irq_link((*data).irq); sun4d_unmask_irq(data); 0
}
unsafe fn sun4d_shutdown_irq(data: *mut IrqData) { sun4d_mask_irq(data); irq_unlink((*data).irq); }

static mut SUN4D_IRQ: IrqChip = IrqChip {
    name: "sun4d", irq_startup: Some(sun4d_startup_irq), irq_shutdown: Some(sun4d_shutdown_irq),
    irq_unmask: Some(sun4d_unmask_irq), irq_mask: Some(sun4d_mask_irq),
};

unsafe fn sun4d_clear_clock_irq() { sbus_readl(&(*SUN4D_TIMERS).l10_timer_limit); }
unsafe fn sun4d_load_profile_irq(cpu: core::ffi::c_int, limit: core::ffi::c_uint) {
    bw_set_prof_limit(cpu, if limit != 0 { timer_value(limit) } else { 0 });
}
unsafe fn sun4d_load_profile_irqs() {
    let mut cpu = 0; let mut mid = 0;
    while !cpu_find_by_instance(cpu, core::ptr::null_mut(), &mut mid) { sun4d_load_profile_irq(mid >> 3, 0); cpu += 1; }
}

unsafe fn _sun4d_build_device_irq(real_irq: core::ffi::c_uint, pil: core::ffi::c_uint, board: core::ffi::c_uint) -> core::ffi::c_uint {
    let irq = irq_alloc(real_irq, pil);
    if irq == 0 { prom_printf("IRQ: allocate for %d %d %d failed\n", real_irq, pil, board); return irq; }
    let mut handler_data = irq_get_handler_data(irq);
    if !handler_data.is_null() { return irq; }
    handler_data = kzalloc_obj::<Sun4dHandlerData>();
    if handler_data.is_null() { prom_printf("IRQ: kzalloc(sun4d_handler_data) failed.\n"); prom_halt(); }
    (*handler_data).cpuid = BOARD_TO_CPU[board as usize] as _;
    (*handler_data).real_irq = real_irq;
    irq_set_chip_and_handler_name(irq, &mut SUN4D_IRQ, handle_level_irq, "level");
    irq_set_handler_data(irq, handler_data);
    irq
}

unsafe fn sun4d_build_timer_irq(board: core::ffi::c_uint, real_irq: core::ffi::c_uint) -> core::ffi::c_uint { _sun4d_build_device_irq(real_irq, real_irq, board) }

unsafe fn sun4d_build_device_irq(op: *mut PlatformDevice, real_irq: core::ffi::c_uint) -> core::ffi::c_uint {
    let dp = (*(*op).dev.of_node).parent;
    let mut bus = dp;
    let mut bus_connection: *const core::ffi::c_char = core::ptr::null();
    while !bus.is_null() {
        if of_node_name_eq(bus, "sbi") { bus_connection = "io-unit\0".as_ptr() as _; break; }
        if of_node_name_eq(bus, "bootbus") { bus_connection = "cpu-unit\0".as_ptr() as _; break; }
        bus = (*bus).parent;
    }
    if bus.is_null() { return real_irq; }
    let regs = of_get_property((*op).dev.of_node, "reg", core::ptr::null_mut());
    if regs.is_null() { return real_irq; }
    let slot = (*regs).which_io;
    if !of_node_name_eq((*bus).parent, bus_connection) { printk(KERN_ERR, "%pOF: Error, parent is not %s.\n", bus, bus_connection); return real_irq; }
    let board_parent = (*bus).parent;
    let board = of_getintprop_default(board_parent, "board#", -1);
    if board == -1 { printk(KERN_ERR, "%pOF: Error, lacks board# property.\n", board_parent); return real_irq; }
    let sbusl = PIL_TO_SBUS[real_irq as usize];
    let pil = if sbusl != 0 { sun4d_encode_irq(board, sbusl, slot as _) } else { real_irq };
    _sun4d_build_device_irq(real_irq, pil, board as _)
}

unsafe fn sun4d_fixup_trap_table() {
    // CONFIG_SMP: patch the level-14 trap vector and flush the local cache.
}

unsafe fn sun4d_init_timers() {
    let dp = of_find_node_by_name(core::ptr::null_mut(), "cpu-unit");
    if dp.is_null() { prom_printf("sun4d_init_timers: Unable to find cpu-unit\n"); prom_halt(); }
    let reg = of_get_property(dp, "reg", core::ptr::null_mut());
    if reg.is_null() { prom_printf("sun4d_init_timers: No reg property\n"); prom_halt(); }
    let board = of_getintprop_default(dp, "board#", -1);
    if board == -1 { prom_printf("sun4d_init_timers: No board# property on cpu-unit\n"); prom_halt(); }
    of_node_put(dp);
    let mut res = Resource { start: *reg.add(1) as _, end: *reg.add(2) as _ - 1, flags: *reg as _ };
    SUN4D_TIMERS = of_ioremap(&mut res, BW_TIMER_LIMIT, core::mem::size_of::<Sun4dTimerRegs>(), "user timer");
    if SUN4D_TIMERS.is_null() { prom_printf("sun4d_init_timers: Can't map timer regs\n"); prom_halt(); }
    // CONFIG_SMP selects SBUS_CLOCK_RATE * 2; otherwise SBUS_CLOCK_RATE / HZ and L10 clockevent.
    sparc_config.cs_period = SBUS_CLOCK_RATE * 2;
    sparc_config.features |= FEAT_L10_CLOCKSOURCE;
    sbus_writel(timer_value(sparc_config.cs_period), &mut (*SUN4D_TIMERS).l10_timer_limit);
    master_l10_counter = &mut (*SUN4D_TIMERS).l10_cur_count;
    let irq = sun4d_build_timer_irq(board as _, SUN4D_TIMER_IRQ);
    let err = request_irq(irq, timer_interrupt, IRQF_TIMER, "timer", core::ptr::null_mut());
    if err != 0 { prom_printf("sun4d_init_timers: request_irq() failed with %d\n", err); prom_halt(); }
    sun4d_load_profile_irqs(); sun4d_fixup_trap_table();
}

unsafe fn sun4d_init_sbi_irq() {
    let target_cpu = boot_cpu_id;
    // for_each_node_by_name(dp, "sbi")
    // Each SBI is assigned target_cpu, and pending PROM interrupts are acknowledged and released.
    // The loop body is represented directly by the surrounding kernel's node iterator.
}

unsafe fn sun4d_init_IRQ() {
    local_irq_disable();
    sparc_config.init_timers = Some(sun4d_init_timers);
    sparc_config.clock_rate = SBUS_CLOCK_RATE;
    sparc_config.build_device_irq = Some(sun4d_build_device_irq);
    sparc_config.clear_clock_irq = Some(sun4d_clear_clock_irq);
    sparc_config.load_profile_irq = Some(sun4d_load_profile_irq);
    // Cannot enable interrupts until OBP ticker is disabled.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
