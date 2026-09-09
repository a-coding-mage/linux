// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/renesas/r7780rp/psw.c
 *
 * push switch support for RDBRP-1/RDBREVRP-1 debug boards.
 *
 * Copyright (C) 2006  Paul Mundt
 */
// C includes translated as dependencies supplied by other files.

unsafe fn psw_irq_handler(irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let pdev = arg as *mut platform_device;
    let psw = platform_get_drvdata(pdev);
    let psw_info = (*pdev).dev.platform_data;
    let mut l: u32;
    let mask: u32;
    let mut ret: i32 = 0;

    l = __raw_readw(PA_DBSW) as u32;

    /* Nothing to do if there's no state change */
    if (*psw).state != 0 {
        ret = 1;
    } else {
        mask = l & 0x70;
        /* Figure out who raised it */
        if (mask & (1u32 << (*psw_info).bit)) != 0 {
            (*psw).state = if (mask & (1u32 << (*psw_info).bit)) != 0 { 1 } else { 0 };
            if (*psw).state != 0 { /* debounce */
                mod_timer(&mut (*psw).debounce, jiffies.wrapping_add(50));
            }
            ret = 1;
        }
    }

    /* Clear the switch IRQs */
    l |= 0x7 << 12;
    __raw_writew(l as u16, PA_DBSW);

    IRQ_RETVAL(ret)
}

static mut psw_resources: [resource; 1] = [resource {
    start: IRQ_PSW,
    flags: IORESOURCE_IRQ,
}];

static mut s2_platform_data: push_switch_platform_info = push_switch_platform_info {
    name: b"s2\0".as_ptr() as *const i8,
    bit: 6,
    irq_flags: IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_SHARED,
    irq_handler: Some(psw_irq_handler),
};

static mut s2_switch_device: platform_device = platform_device {
    name: b"push-switch\0".as_ptr() as *const i8,
    id: 0,
    num_resources: ARRAY_SIZE(psw_resources),
    resource: psw_resources.as_mut_ptr(),
    dev: device {
        platform_data: &mut s2_platform_data as *mut push_switch_platform_info as *mut core::ffi::c_void,
    },
};

static mut s3_platform_data: push_switch_platform_info = push_switch_platform_info {
    name: b"s3\0".as_ptr() as *const i8,
    bit: 5,
    irq_flags: IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_SHARED,
    irq_handler: Some(psw_irq_handler),
};

static mut s3_switch_device: platform_device = platform_device {
    name: b"push-switch\0".as_ptr() as *const i8,
    id: 1,
    num_resources: ARRAY_SIZE(psw_resources),
    resource: psw_resources.as_mut_ptr(),
    dev: device {
        platform_data: &mut s3_platform_data as *mut push_switch_platform_info as *mut core::ffi::c_void,
    },
};

static mut s4_platform_data: push_switch_platform_info = push_switch_platform_info {
    name: b"s4\0".as_ptr() as *const i8,
    bit: 4,
    irq_flags: IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_SHARED,
    irq_handler: Some(psw_irq_handler),
};

static mut s4_switch_device: platform_device = platform_device {
    name: b"push-switch\0".as_ptr() as *const i8,
    id: 2,
    num_resources: ARRAY_SIZE(psw_resources),
    resource: psw_resources.as_mut_ptr(),
    dev: device {
        platform_data: &mut s4_platform_data as *mut push_switch_platform_info as *mut core::ffi::c_void,
    },
};

static mut psw_devices: [*mut platform_device; 3] = [
    &mut s2_switch_device,
    &mut s3_switch_device,
    &mut s4_switch_device,
];

unsafe fn psw_init() -> i32 {
    platform_add_devices(psw_devices.as_mut_ptr(), ARRAY_SIZE(psw_devices))
}

module_init!(psw_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
