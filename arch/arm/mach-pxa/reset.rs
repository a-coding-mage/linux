// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies supplied by the corresponding kernel headers and local
// translation units are intentionally referenced here but not implemented.

use core::ffi::c_char;

extern "C" {
    fn gpio_request(gpio: i32, label: *const c_char) -> i32;
    fn gpio_direction_output(gpio: i32, value: i32) -> i32;
    fn gpio_direction_input(gpio: i32) -> i32;
    fn gpio_free(gpio: i32);
    fn gpio_set_value(gpio: i32, value: i32);
    fn mdelay(milliseconds: u32);
    fn writel_relaxed(value: u32, address: *mut u32);
    fn readl_relaxed(address: *const u32) -> u32;
    fn local_irq_disable();
    fn local_fiq_disable();
    fn clear_reset_status(status: u32);
    fn soft_restart(address: u32);
    fn printk(format: *const c_char, ...);
}

// These names are provided by the translated register and reboot definitions.
extern "C" {
    static mut OWER: u32;
    static mut OSSR: u32;
    static mut OSCR: u32;
    static mut OSMR3: u32;
    static mut MDREFR: u32;
}

// Build-time/kernel constants supplied by other translation units.
extern "C" {
    static OWER_WME: u32;
    static OSSR_M3: u32;
    static MDREFR_SLFRSH: u32;
    static RESET_STATUS_ALL: u32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum reboot_mode {
    REBOOT_SOFT,
    REBOOT_GPIO,
    REBOOT_HARD,
}

static mut reset_gpio: i32 = -1;

#[no_mangle]
pub unsafe extern "C" fn init_gpio_reset(gpio: i32, output: i32, level: i32) -> i32 {
    let mut rc: i32;

    rc = gpio_request(gpio, b"reset generator\0".as_ptr() as *const c_char);
    if rc != 0 {
        printk(b"Can't request reset_gpio\n\0".as_ptr() as *const c_char);
        return rc;
    }

    if output != 0 {
        rc = gpio_direction_output(gpio, level);
    } else {
        rc = gpio_direction_input(gpio);
    }
    if rc != 0 {
        printk(b"Can't configure reset_gpio\n\0".as_ptr() as *const c_char);
        gpio_free(gpio);
        return rc;
    }

    reset_gpio = gpio;
    rc
}

unsafe fn do_gpio_reset() {
    assert!(reset_gpio != -1);

    gpio_direction_output(reset_gpio, 0);
    mdelay(2);
    gpio_set_value(reset_gpio, 1);
    mdelay(2);
    gpio_set_value(reset_gpio, 0);
    mdelay(10);

    // WARN_ON(1);
    do_hw_reset();
}

unsafe fn do_hw_reset() {
    writel_relaxed(OWER_WME, &mut OWER);
    writel_relaxed(OSSR_M3, &mut OSSR);
    writel_relaxed(readl_relaxed(&OSCR).wrapping_add(368640), &mut OSMR3);

    // SDRAM hangs on watchdog reset on Marvell PXA270 (erratum 71).
    // Put SDRAM into self-refresh to prevent that.
    loop {
        writel_relaxed(MDREFR_SLFRSH, &mut MDREFR);
    }
}

#[no_mangle]
pub unsafe extern "C" fn pxa_restart(mode: reboot_mode, _cmd: *const c_char) {
    local_irq_disable();
    local_fiq_disable();

    clear_reset_status(RESET_STATUS_ALL);

    match mode {
        reboot_mode::REBOOT_SOFT => {
            // Jump into ROM at address 0.
            soft_restart(0);
        }
        reboot_mode::REBOOT_GPIO => {
            do_gpio_reset();
        }
        reboot_mode::REBOOT_HARD => {
            do_hw_reset();
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
