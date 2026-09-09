/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1997, 1998, 2001, 03, 05, 06 by Ralf Baechle
 */

// Dependencies supplied by the Linux/MIPS environment are intentionally not
// defined here.

const POWERDOWN_TIMEOUT: u64 = 120;
const POWERDOWN_FREQ: usize = HZ / 4;
const PANIC_FREQ: usize = HZ / 8;

static mut power_timer: timer_list = timer_list::default();
static mut blink_timer: timer_list = timer_list::default();
static mut debounce_timer: timer_list = timer_list::default();
static mut blink_timer_timeout: usize = 0;

const MACHINE_PANICKED: i32 = 1;
const MACHINE_SHUTTING_DOWN: i32 = 2;
static mut machine_state: i32 = 0;

unsafe fn sgi_machine_power_off() -> ! {
    let mut tmp: u32;

    local_irq_disable();

    // Disable watchdog
    tmp = hpc3c0.rtcregs[RTC_CMD] & 0xff;
    hpc3c0.rtcregs[RTC_CMD] = tmp | RTC_WAM;
    hpc3c0.rtcregs[RTC_WSEC] = 0;
    hpc3c0.rtcregs[RTC_WHSEC] = 0;

    loop {
        sgioc.panel = !SGIOC_PANEL_POWERON;
        // Good bye cruel world ...

        // If we're still running, we probably got sent an alarm interrupt.
        // Read the flag to clear it.
        tmp = hpc3c0.rtcregs[RTC_HOURS_ALARM];
        let _ = tmp;
    }
}

unsafe fn sgi_machine_restart(_command: *mut u8) -> ! {
    if machine_state & MACHINE_SHUTTING_DOWN != 0 {
        sgi_machine_power_off();
    }
    sgimc.cpuctrl0 |= SGIMC_CCTRL0_SYSINIT;
    loop {}
}

unsafe fn sgi_machine_halt() -> ! {
    if machine_state & MACHINE_SHUTTING_DOWN != 0 {
        sgi_machine_power_off();
    }
    ArcEnterInteractiveMode();
    loop {}
}

unsafe extern "C" fn power_timeout(_unused: *mut timer_list) {
    sgi_machine_power_off();
}

unsafe extern "C" fn blink_timeout(_unused: *mut timer_list) {
    // XXX fix this for fullhouse
    sgi_ioc_reset ^= SGIOC_RESET_LC0OFF | SGIOC_RESET_LC1OFF;
    sgioc.reset = sgi_ioc_reset;

    mod_timer(&mut blink_timer, jiffies + blink_timer_timeout);
}

unsafe extern "C" fn debounce(_unused: *mut timer_list) {
    timer_delete(&mut debounce_timer);
    if sgint.istat1 & SGINT_ISTAT1_PWR != 0 {
        // Interrupt still being sent.
        debounce_timer.expires = jiffies + (HZ / 20); // 0.05s
        add_timer(&mut debounce_timer);

        sgioc.panel = SGIOC_PANEL_POWERON
            | SGIOC_PANEL_POWERINTR
            | SGIOC_PANEL_VOLDNINTR
            | SGIOC_PANEL_VOLDNHOLD
            | SGIOC_PANEL_VOLUPINTR
            | SGIOC_PANEL_VOLUPHOLD;
        return;
    }

    if machine_state & MACHINE_PANICKED != 0 {
        sgimc.cpuctrl0 |= SGIMC_CCTRL0_SYSINIT;
    }
    enable_irq(SGI_PANEL_IRQ);
}

#[inline]
unsafe fn power_button() {
    if machine_state & MACHINE_PANICKED != 0 {
        return;
    }

    if machine_state & MACHINE_SHUTTING_DOWN != 0 || kill_cad_pid(SIGINT, 1) != 0 {
        // No init process or button pressed twice.
        sgi_machine_power_off();
    }

    machine_state |= MACHINE_SHUTTING_DOWN;
    blink_timer_timeout = POWERDOWN_FREQ;
    blink_timeout(&mut blink_timer);

    timer_setup(&mut power_timer, power_timeout, 0);
    power_timer.expires = jiffies + POWERDOWN_TIMEOUT * HZ;
    add_timer(&mut power_timer);
}

unsafe extern "C" fn panel_int(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let buttons = sgioc.panel;
    sgioc.panel = SGIOC_PANEL_POWERON | SGIOC_PANEL_POWERINTR;

    if sgint.istat1 & SGINT_ISTAT1_PWR != 0 {
        // Wait until interrupt goes away
        disable_irq_nosync(SGI_PANEL_IRQ);
        timer_setup(&mut debounce_timer, debounce, 0);
        debounce_timer.expires = jiffies + 5;
        add_timer(&mut debounce_timer);
    }

    // Power button was pressed.  The panel register uses the power interrupt
    // bit; all bits are pulled high on fullhouse.
    if buttons & SGIOC_PANEL_POWERINTR == 0 {
        power_button();
    }

    IRQ_HANDLED
}

unsafe extern "C" fn panic_event(
    _this: *mut notifier_block,
    _event: usize,
    _ptr: *mut core::ffi::c_void,
) -> i32 {
    if machine_state & MACHINE_PANICKED != 0 {
        return NOTIFY_DONE;
    }
    machine_state |= MACHINE_PANICKED;

    blink_timer_timeout = PANIC_FREQ;
    blink_timeout(&mut blink_timer);

    NOTIFY_DONE
}

static mut panic_block: notifier_block = notifier_block {
    notifier_call: Some(panic_event),
};

unsafe extern "C" fn reboot_setup() -> i32 {
    let res: i32;

    _machine_restart = Some(sgi_machine_restart);
    _machine_halt = Some(sgi_machine_halt);
    pm_power_off = Some(sgi_machine_power_off);

    res = request_irq(SGI_PANEL_IRQ, Some(panel_int), 0, "Front Panel", core::ptr::null_mut());
    if res != 0 {
        printk(KERN_ERR, "Allocation of front panel IRQ failed\n");
        return res;
    }

    timer_setup(&mut blink_timer, blink_timeout, 0);
    atomic_notifier_chain_register(&mut panic_notifier_list, &mut panic_block);

    0
}

// Equivalent of subsys_initcall(reboot_setup).
subsys_initcall!(reboot_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
