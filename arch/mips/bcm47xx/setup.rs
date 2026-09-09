/*
 *  Copyright (C) 2004 Florian Schirmer <jolt@tuxbox.org>
 *  Copyright (C) 2006 Felix Fietkau <nbd@openwrt.org>
 *  Copyright (C) 2006 Michael Buesch <m@bues.ch>
 *  Copyright (C) 2010 Waldemar Brodkorb <wbx@openadk.org>
 *  Copyright (C) 2010-2012 Hauke Mehrtens <hauke@hauke-m.de>
 *
 *  This program is free software; you can redistribute  it and/or modify it
 *  under  the terms of  the GNU General  Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or (at your
 *  option) any later version.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

/* CBR addr doesn't change and we can cache it. */
pub static mut bmips_cbr_addr: *mut core::ffi::c_void = core::ptr::null_mut();

pub static mut bcm47xx_bus: union_bcm47xx_bus = union_bcm47xx_bus { ssb: unsafe { core::mem::zeroed() } };
pub static mut bcm47xx_bus_type: bcm47xx_bus_type = unsafe { core::mem::zeroed() };

// Opaque external types and symbols are supplied by other files.
extern "C" {
    static mut current_cpu_data: cpuinfo_mips;
    static mut _machine_restart: Option<unsafe extern "C" fn(*mut i8)>;
    static mut _machine_halt: Option<unsafe extern "C" fn()>;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
    static mut cpu_wait: *mut core::ffi::c_void;
}

unsafe fn bcm47xx_machine_restart(_command: *mut i8) {
    pr_alert!("Please stand by while rebooting the system...\n");
    local_irq_disable();
    match bcm47xx_bus_type {
        // CONFIG_BCM47XX_SSB
        bcm47xx_bus_type::SSB => {
            if bcm47xx_bus.ssb.chip_id == 0x4785 { write_c0_diag4(1 << 22); }
            ssb_watchdog_timer_set(&mut bcm47xx_bus.ssb, 1);
            if bcm47xx_bus.ssb.chip_id == 0x4785 {
                core::arch::asm!("sync", "wait", options(nostack, preserves_flags));
            }
        }
        // CONFIG_BCM47XX_BCMA
        bcm47xx_bus_type::BCMA => {
            bcma_chipco_watchdog_timer_set(&mut bcm47xx_bus.bcma.bus.drv_cc, 1);
        }
        _ => {}
    }
    loop { cpu_relax(); }
}

unsafe fn bcm47xx_machine_halt() {
    local_irq_disable();
    match bcm47xx_bus_type {
        // CONFIG_BCM47XX_SSB
        bcm47xx_bus_type::SSB => ssb_watchdog_timer_set(&mut bcm47xx_bus.ssb, 0),
        // CONFIG_BCM47XX_BCMA
        bcm47xx_bus_type::BCMA => bcma_chipco_watchdog_timer_set(&mut bcm47xx_bus.bcma.bus.drv_cc, 0),
        _ => {}
    }
    loop { cpu_relax(); }
}

// CONFIG_BCM47XX_SSB
unsafe fn bcm47xx_register_ssb() {
    let mut buf = [0i8; 100];
    let err = ssb_bus_host_soc_register(&mut bcm47xx_bus.ssb, SSB_ENUM_BASE);
    if err != 0 { panic!("Failed to initialize SSB bus (err {})", err); }
    let mcore = &mut bcm47xx_bus.ssb.mipscore;
    if bcm47xx_nvram_getenv(cstr!("kernel_args"), buf.as_mut_ptr(), buf.len()) >= 0 {
        if c_strstr(buf.as_ptr(), cstr!("console=ttyS1")) {
            let mut port: ssb_serial_port = core::mem::zeroed();
            pr_debug!("Swapping serial ports!\n");
            core::ptr::copy_nonoverlapping(&mcore.serial_ports[0], &mut port, 1);
            core::ptr::copy_nonoverlapping(&mcore.serial_ports[1], &mut mcore.serial_ports[0], 1);
            core::ptr::copy_nonoverlapping(&port, &mut mcore.serial_ports[1], 1);
        }
    }
}

// CONFIG_BCM47XX_BCMA
unsafe fn bcm47xx_register_bcma() {
    let err = bcma_host_soc_register(&mut bcm47xx_bus.bcma);
    if err != 0 { panic!("Failed to register BCMA bus (err {})", err); }
}

pub unsafe fn plat_mem_setup() {
    let c = &current_cpu_data;
    if c.cputype == CPU_74K {
        pr_info!("Using bcma bus\n");
        // CONFIG_BCM47XX_BCMA
        bcm47xx_bus_type = bcm47xx_bus_type::BCMA;
        bcm47xx_register_bcma();
        bcm47xx_set_system_type(bcm47xx_bus.bcma.bus.chipinfo.id);
        // CONFIG_HIGHMEM: bcm47xx_prom_highmem_init();
    } else {
        pr_info!("Using ssb bus\n");
        // CONFIG_BCM47XX_SSB
        bcm47xx_bus_type = bcm47xx_bus_type::SSB;
        bcm47xx_sprom_register_fallbacks();
        bcm47xx_register_ssb();
        bcm47xx_set_system_type(bcm47xx_bus.ssb.chip_id);
    }
    _machine_restart = Some(bcm47xx_machine_restart);
    _machine_halt = Some(bcm47xx_machine_halt);
    pm_power_off = Some(bcm47xx_machine_halt);
}

// CONFIG_BCM47XX_BCMA
unsafe fn bcm47xx_setup_device() -> *mut device {
    let dev = kzalloc_obj::<device>();
    if dev.is_null() { return core::ptr::null_mut(); }
    let err = dev_set_name(dev, cstr!("bcm47xx_soc"));
    if err != 0 { pr_err!("Failed to set SoC device name: {}\n", err); kfree(dev); return core::ptr::null_mut(); }
    let err = dma_coerce_mask_and_coherent(dev, DMA_BIT_MASK(32));
    if err != 0 { pr_err!("Failed to set SoC DMA mask: {}\n", err); }
    dev
}

pub unsafe fn bcm47xx_bus_setup() {
    // CONFIG_BCM47XX_BCMA
    if bcm47xx_bus_type == bcm47xx_bus_type::BCMA {
        bcm47xx_bus.bcma.dev = bcm47xx_setup_device();
        if bcm47xx_bus.bcma.dev.is_null() { panic!("Failed to setup SoC device\n"); }
        let err = bcma_host_soc_init(&mut bcm47xx_bus.bcma);
        if err != 0 { panic!("Failed to initialize BCMA bus (err {})", err); }
    }
    bcm47xx_board_detect();
    mips_set_machine_name(bcm47xx_board_get_name());
}

unsafe fn bcm47xx_cpu_fixes() -> i32 {
    match bcm47xx_bus_type {
        bcm47xx_bus_type::SSB => {}
        bcm47xx_bus_type::BCMA => {
            if bcm47xx_bus.bcma.bus.chipinfo.id == BCMA_CHIP_ID_BCM4706 { cpu_wait = core::ptr::null_mut(); }
        }
        _ => {}
    }
    0
}

unsafe fn bcm47xx_register_bus_complete() -> i32 {
    match bcm47xx_bus_type {
        bcm47xx_bus_type::SSB => {}
        bcm47xx_bus_type::BCMA => {
            if device_register(bcm47xx_bus.bcma.dev) != 0 { pr_err!("Failed to register SoC device\n"); }
            bcma_bus_register(&mut bcm47xx_bus.bcma.bus);
        }
        _ => {}
    }
    bcm47xx_buttons_register();
    bcm47xx_leds_register();
    bcm47xx_workarounds();
    0
}

// arch_initcall(bcm47xx_cpu_fixes);
// device_initcall(bcm47xx_register_bus_complete);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
