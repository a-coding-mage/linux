// SPDX-License-Identifier: GPL-2.0-only
/*
 * EcoNet setup code
 *
 * Copyright (C) 2025 Caleb James DeLisle <cjd@cjdns.fr>
 */

// Dependencies supplied by the surrounding kernel translation unit.

const CR_AHB_RSTCR: *mut core::ffi::c_void = CKSEG1ADDR(0x1fb00040);
const RESET: u32 = BIT(31);

const UART_BASE: usize = CKSEG1ADDR(0x1fbf0003);
const UART_REG_SHIFT: u32 = 2;

unsafe fn hw_reset(_command: *mut core::ffi::c_char) {
    iowrite32(RESET, CR_AHB_RSTCR);
}

/*
 * vsmp_init_secondary expects either GIC or cascading interrupt configuration.
 * The EN751221 is a 34Kc with VEIC, but not GIC compatible. The cascading
 * configuration enables lines 6 and 7 for performance counters, but when this
 * is done on the EN751221 intc with VEIC enabled, it causes the whole intc to
 * stop sending interrupts.
 * Only unmask lines 0 and 1 (software interrupts) in init_secondary.
 */
#[cfg(CONFIG_MIPS_MT_SMP)]
extern "C" {
    static vsmp_smp_ops: plat_smp_ops;
}

#[cfg(CONFIG_MIPS_MT_SMP)]
static mut en75_smp_ops: plat_smp_ops = unsafe { core::mem::zeroed() };

#[cfg(CONFIG_MIPS_MT_SMP)]
unsafe fn en751221_init_secondary() {
    write_c0_status((read_c0_status() & !ST0_IM) | (STATUSF_IP0 | STATUSF_IP1));
}

#[cfg(CONFIG_MIPS_MT_SMP)]
unsafe fn en751221_register_vsmp_smp_ops() -> i32 {
    if !cpu_has_mipsmt {
        return -ENODEV;
    }

    en75_smp_ops = vsmp_smp_ops;
    en75_smp_ops.init_secondary = Some(en751221_init_secondary);
    register_smp_ops(&raw mut en75_smp_ops);
    0
}

#[cfg(not(CONFIG_MIPS_MT_SMP))]
unsafe fn en751221_register_vsmp_smp_ops() -> i32 {
    -ENODEV
}

/* 1. Bring up early printk. */
unsafe fn prom_init() {
    setup_8250_early_printk_port(UART_BASE, UART_REG_SHIFT, 0);
    _machine_restart = Some(hw_reset);
}

/* 2. Parse the DT and find memory */
unsafe fn plat_mem_setup() {
    let mut dtb: *mut core::ffi::c_void;

    set_io_port_base(KSEG1);

    dtb = get_fdt();
    if dtb.is_null() {
        panic!("no dtb found");
    }

    __dt_setup_arch(dtb);

    early_init_dt_scan_memory();
}

/* 3. Overload __weak device_tree_init(), add SMP ops */
unsafe fn device_tree_init() {
    unflatten_and_copy_device_tree();

    /* EN751221 dual-vpe */
    if en751221_register_vsmp_smp_ops() == 0 {
        return;
    }

    /* EN751221 with MT_SMP disabled */
    register_up_smp_ops();
}

unsafe fn get_system_type() -> *const core::ffi::c_char {
    b"EcoNet-EN75xx\0".as_ptr() as *const core::ffi::c_char
}

/* 4. Initialize the IRQ subsystem */
unsafe fn arch_init_irq() {
    irqchip_init();
}

/* 5. Timers */
unsafe fn plat_time_init() {
    of_clk_init(core::ptr::null());
    timer_probe();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
