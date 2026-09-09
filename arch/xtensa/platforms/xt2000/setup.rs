// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/xtensa/platforms/xt2000/setup.c
 *
 * Platform specific functions for the XT2000 board.
 *
 * Authors: Chris Zankel <chris@zankel.net>
 *          Joe Taylor <joe@tensilica.com>
 *
 * Copyright 2001 - 2004 Tensilica Inc.
 */

/* C header dependencies are supplied by the surrounding kernel translation. */

/* Assumes s points to an 8-chr string. No checking for NULL. */
unsafe fn led_print(f: i32, mut s: *const u8) {
    let mut led_addr = (XT2000_LED_ADDR + 0xE0) as *mut c_ulong;
    led_addr = led_addr.add(f as usize);
    let mut i = f;
    while i < 8 {
        let value = *s;
        *led_addr = value as c_ulong;
        led_addr = led_addr.add(1);
        s = s.add(1);
        if value == 0 {
            break;
        }
        i += 1;
    }
}

unsafe fn xt2000_power_off(_unused: *mut sys_off_data) -> i32 {
    led_print(0, b"POWEROFF\0".as_ptr());
    local_irq_disable();
    loop {}
}

unsafe fn xt2000_restart(_unused: *mut sys_off_data) -> i32 {
    /* Flush and reset the mmu, simulate a processor reset, and
     * jump to the reset vector. */
    cpu_reset();
    NOTIFY_DONE
}

unsafe fn platform_setup(_cmdline: *mut *mut c_char) {
    led_print(0, b"LINUX   \0".as_ptr());
}

/* Heartbeat. Let the LED blink. */

unsafe fn xt2000_heartbeat(_unused: *mut timer_list) {
    static mut I: i32 = 0;

    led_print(7, if I != 0 { b".\0".as_ptr() } else { b" \0".as_ptr() });
    I ^= 1;
    mod_timer(&mut heartbeat_timer, jiffies + HZ / 2);
}

/* DEFINE_TIMER(heartbeat_timer, xt2000_heartbeat); */
static mut heartbeat_timer: timer_list = timer_list {
    function: Some(xt2000_heartbeat),
    ..unsafe { core::mem::zeroed() }
};

/* The C _SERIAL_PORT(_base, _irq) macro expands to these fields. */
static mut xt2000_serial_data: [plat_serial8250_port; 3] = [
    plat_serial8250_port {
        // C condition: XCHAL_HAVE_BE selects the address plus 3 variant.
        mapbase: DUART16552_1_ADDR + if cfg!(xchal_have_be) { 3 } else { 0 },
        membase: (DUART16552_1_ADDR + if cfg!(xchal_have_be) { 3 } else { 0 }) as *mut c_void,
        irq: DUART16552_1_INTNUM,
        uartclk: DUART16552_XTAL_FREQ,
        iotype: UPIO_MEM,
        flags: UPF_BOOT_AUTOCONF,
        regshift: 2,
    },
    plat_serial8250_port {
        mapbase: DUART16552_2_ADDR + if cfg!(xchal_have_be) { 3 } else { 0 },
        membase: (DUART16552_2_ADDR + if cfg!(xchal_have_be) { 3 } else { 0 }) as *mut c_void,
        irq: DUART16552_2_INTNUM,
        uartclk: DUART16552_XTAL_FREQ,
        iotype: UPIO_MEM,
        flags: UPF_BOOT_AUTOCONF,
        regshift: 2,
    },
    plat_serial8250_port { ..unsafe { core::mem::zeroed() } },
];

static mut xt2000_serial8250_device: platform_device = platform_device {
    name: b"serial8250\0".as_ptr() as *const c_char,
    id: PLAT8250_DEV_PLATFORM,
    dev: device {
        platform_data: unsafe { &mut xt2000_serial_data as *mut _ as *mut c_void },
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

static mut xt2000_sonic_res: [resource; 2] = [
    resource { start: SONIC83934_ADDR, end: SONIC83934_ADDR + 0xff, flags: IORESOURCE_MEM, ..unsafe { core::mem::zeroed() } },
    resource { start: SONIC83934_INTNUM, end: SONIC83934_INTNUM, flags: IORESOURCE_IRQ, ..unsafe { core::mem::zeroed() } },
];

static mut xt2000_sonic_device: platform_device = platform_device {
    name: b"xtsonic\0".as_ptr() as *const c_char,
    num_resources: 2,
    resource: unsafe { xt2000_sonic_res.as_mut_ptr() },
    ..unsafe { core::mem::zeroed() }
};

unsafe fn xt2000_setup_devinit() -> i32 {
    platform_device_register(&mut xt2000_serial8250_device);
    platform_device_register(&mut xt2000_sonic_device);
    mod_timer(&mut heartbeat_timer, jiffies + HZ / 2);
    register_sys_off_handler(SYS_OFF_MODE_RESTART, SYS_OFF_PRIO_PLATFORM, Some(xt2000_restart), core::ptr::null_mut());
    register_sys_off_handler(SYS_OFF_MODE_POWER_OFF, SYS_OFF_PRIO_DEFAULT, Some(xt2000_power_off), core::ptr::null_mut());
    0
}

/* device_initcall(xt2000_setup_devinit); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
