// SPDX-License-Identifier: GPL-2.0
/*
 * bios-less APM driver for hp680
 *
 * Copyright 2005 (c) Andriy Skulysh <askulysh@gmail.com>
 * Copyright 2008 (c) Kristoffer Ericson <kristoffer.ericson@gmail.com>
 */

// Dependencies supplied by the kernel and architecture-specific headers:
// linux/module.h, linux/kernel.h, linux/init.h, linux/interrupt.h,
// linux/apm-emulation.h, linux/io.h, asm/adc.h, mach/hp6xx.h

/* percentage values */
const APM_CRITICAL: i32 = 10;
const APM_LOW: i32 = 30;

/* resonably sane values */
const HP680_BATTERY_MAX: i32 = 898;
const HP680_BATTERY_MIN: i32 = 486;
const HP680_BATTERY_AC_ON: i32 = 1023;

const MODNAME: &[u8] = b"hp6x0_apm\0";
const PGDR: usize = 0xa400012c;

unsafe fn hp6x0_apm_get_power_status(info: *mut apm_power_info) {
    let battery: i32 = adc_single(ADC_CHANNEL_BATTERY) as i32;
    let backup: i32 = adc_single(ADC_CHANNEL_BACKUP) as i32;
    let charging: i32 = adc_single(ADC_CHANNEL_CHARGE) as i32;
    let _ = backup;

    let percentage = 100 * (battery - HP680_BATTERY_MIN)
        / (HP680_BATTERY_MAX - HP680_BATTERY_MIN);

    /* % of full battery */
    (*info).battery_life = percentage;

    /* We want our estimates in minutes */
    (*info).units = 0;

    /* Extremely(!!) rough estimate, we will replace this with a datalist later on */
    (*info).time = 2 * battery;

    (*info).ac_line_status = if battery > HP680_BATTERY_AC_ON {
        APM_AC_ONLINE
    } else {
        APM_AC_OFFLINE
    };

    let pgdr = __raw_readb(PGDR as *const u8);
    if pgdr & PGDR_MAIN_BATTERY_OUT != 0 {
        (*info).battery_status = APM_BATTERY_STATUS_NOT_PRESENT;
        (*info).battery_flag = 0x80;
    } else if charging < 8 {
        (*info).battery_status = APM_BATTERY_STATUS_CHARGING;
        (*info).battery_flag = 0x08;
        (*info).ac_line_status = 0x01;
    } else if percentage <= APM_CRITICAL {
        (*info).battery_status = APM_BATTERY_STATUS_CRITICAL;
        (*info).battery_flag = 0x04;
    } else if percentage <= APM_LOW {
        (*info).battery_status = APM_BATTERY_STATUS_LOW;
        (*info).battery_flag = 0x02;
    } else {
        (*info).battery_status = APM_BATTERY_STATUS_HIGH;
        (*info).battery_flag = 0x01;
    }
}

unsafe fn hp6x0_apm_interrupt(_irq: i32, _dev: *mut core::ffi::c_void) -> irqreturn_t {
    if !APM_DISABLED {
        apm_queue_event(APM_USER_SUSPEND);
    }

    IRQ_HANDLED
}

unsafe fn hp6x0_apm_init() -> i32 {
    let ret = request_irq(
        HP680_BTN_IRQ,
        hp6x0_apm_interrupt,
        0,
        MODNAME.as_ptr() as *const i8,
        core::ptr::null_mut(),
    );
    if unlikely(ret < 0) {
        printk(KERN_ERR, MODNAME.as_ptr(), HP680_BTN_IRQ);
        return ret;
    }

    apm_get_power_status = Some(hp6x0_apm_get_power_status);

    ret
}

unsafe fn hp6x0_apm_exit() {
    free_irq(HP680_BTN_IRQ, 0);
}

// module_init!(hp6x0_apm_init);
// module_exit!(hp6x0_apm_exit);

// MODULE_AUTHOR!("Adriy Skulysh");
// MODULE_DESCRIPTION!("hp6xx Advanced Power Management");
// MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
