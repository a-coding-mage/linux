// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/arm/mach-footbridge/ebsa285.c
 *
 * EBSA285 machine fixup
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/io.h, linux/spinlock.h, linux/slab.h, linux/leds.h,
// asm/hardware/dec21285.h, asm/mach-types.h, asm/mach/arch.h, and common.h.

/* LEDs */
// Preserve the original CONFIG_NEW_LEDS && CONFIG_LEDS_CLASS build condition.
#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
const XBUS_AMBER_L: u8 = 1u8 << 0;
#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
const XBUS_GREEN_L: u8 = 1u8 << 1;
#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
const XBUS_RED_L: u8 = 1u8 << 2;
#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
const XBUS_TOGGLE: u8 = 1u8 << 7;

#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
#[repr(C)]
struct ebsa285_led {
    cdev: led_classdev,
    mask: u8,
}

/*
 * The triggers lines up below will only be used if the
 * LED triggers are compiled in.
 */
#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
#[repr(C)]
struct ebsa285_led_desc {
    name: *const c_char,
    trigger: *const c_char,
}

#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
static ebsa285_leds: [ebsa285_led_desc; 3] = [
    ebsa285_led_desc { name: c"ebsa285:amber".as_ptr(), trigger: c"cpu0".as_ptr() },
    ebsa285_led_desc { name: c"ebsa285:green".as_ptr(), trigger: c"heartbeat".as_ptr() },
    ebsa285_led_desc { name: c"ebsa285:red".as_ptr(), trigger: core::ptr::null() },
];

#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
static mut hw_led_state: u8 = 0;
#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
static mut xbus: *mut core::ffi::c_void = core::ptr::null_mut();

#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
unsafe fn ebsa285_led_set(cdev: *mut led_classdev, b: led_brightness) {
    let led = container_of!(cdev, ebsa285_led, cdev);

    if b == LED_OFF {
        hw_led_state |= (*led).mask;
    } else {
        hw_led_state &= !(*led).mask;
    }
    writeb(hw_led_state, xbus);
}

#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
unsafe fn ebsa285_led_get(cdev: *mut led_classdev) -> led_brightness {
    let led = container_of!(cdev, ebsa285_led, cdev);

    if hw_led_state & (*led).mask != 0 { LED_OFF } else { LED_FULL }
}

#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
unsafe fn ebsa285_leds_init() -> i32 {
    let mut i: usize;

    if !machine_is_ebsa285() {
        return -ENODEV;
    }

    xbus = ioremap(XBUS_CS2, SZ_4K);
    if xbus.is_null() {
        return -ENOMEM;
    }

    /* 3 LEDS all off */
    hw_led_state = XBUS_AMBER_L | XBUS_GREEN_L | XBUS_RED_L;
    writeb(hw_led_state, xbus);

    i = 0;
    while i < ebsa285_leds.len() {
        let led = kzalloc_obj::<ebsa285_led>();
        if led.is_null() {
            break;
        }

        (*led).cdev.name = ebsa285_leds[i].name;
        (*led).cdev.brightness_set = Some(ebsa285_led_set);
        (*led).cdev.brightness_get = Some(ebsa285_led_get);
        (*led).cdev.default_trigger = ebsa285_leds[i].trigger;
        (*led).mask = 1u8 << i;

        if led_classdev_register(core::ptr::null_mut(), &mut (*led).cdev) < 0 {
            kfree(led);
            break;
        }
        i += 1;
    }

    0
}

/*
 * Since we may have triggers on any subsystem, defer registration
 * until after subsystem_init.
 */
#[cfg(all(feature = "CONFIG_NEW_LEDS", feature = "CONFIG_LEDS_CLASS"))]
fs_initcall!(ebsa285_leds_init);

// MACHINE_START(EBSA285, "EBSA285")
//     /* Maintainer: Russell King */
//     .atag_offset = 0x100,
//     .video_start = 0x000a0000,
//     .video_end = 0x000bffff,
//     .map_io = footbridge_map_io,
//     .init_early = footbridge_sched_clock,
//     .init_irq = footbridge_init_irq,
//     .init_time = footbridge_timer_init,
//     .restart = footbridge_restart,
// MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
