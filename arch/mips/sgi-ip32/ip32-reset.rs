/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 Keith M Wesolowski
 * Copyright (C) 2001 Paul Mundt
 * Copyright (C) 2003 Guido Guenther <agx@sigxcpu.org>
 */

// Linux and IP32 declarations are supplied by the surrounding kernel tree.

const POWERDOWN_TIMEOUT: c_ulong = 120;
/* Blink frequency during reboot grace period and when panicked. */
const POWERDOWN_FREQ: c_ulong = HZ / 4;
const PANIC_FREQ: c_ulong = HZ / 8;

extern "C" {
    static mut ip32_rtc_device: platform_device;
    static mut power_timer: timer_list;
    static mut blink_timer: timer_list;
    static mut blink_timer_timeout: c_ulong;
    static mut has_panicked: c_int;
    static mut shutting_down: c_int;

    static mut mace: *mut mace_t;
    static mut crime: *mut crime_t;
    static mut HZ: c_ulong;
    static mut jiffies: c_ulong;
    static mut panic_notifier_list: notifier_head;
    static mut _machine_restart: Option<unsafe extern "C" fn(*mut c_char)>;
    static mut _machine_halt: Option<unsafe extern "C" fn()>;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;

    fn msleep(msecs: c_uint);
    fn unreachable() -> !;
    fn pr_emerg(fmt: *const c_char, ...);
    fn request_module(name: *const c_char) -> c_int;
    fn kill_cad_pid(signum: c_int, val: c_int) -> c_int;
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn timer_setup(timer: *mut timer_list,
                   callback: unsafe extern "C" fn(*mut timer_list), flags: c_uint);
    fn add_timer(timer: *mut timer_list);
    fn atomic_notifier_chain_register(head: *mut notifier_head,
                                     block: *mut notifier_block) -> c_int;
    fn ds1685_rtc_poweroff(device: *mut platform_device);
}

type c_int = i32;
type c_uint = u32;
type c_ulong = usize;
type c_char = i8;

#[repr(C)]
pub struct platform_device { _private: [u8; 0] }
#[repr(C)]
pub struct timer_list { pub expires: c_ulong, _private: [u8; 0] }
#[repr(C)]
pub struct notifier_head { _private: [u8; 0] }
#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}
#[repr(C)]
pub struct mace_t { pub perif: mace_perif }
#[repr(C)]
pub struct mace_perif { pub ctrl: mace_ctrl }
#[repr(C)]
pub struct mace_ctrl { pub misc: c_ulong }
#[repr(C)]
pub struct crime_t { pub control: c_ulong }
type c_void = core::ffi::c_void;

const CRIME_CONTROL_HARD_RESET: c_ulong = 0;
const MACEISA_LED_RED: c_ulong = 0;
const MACEISA_LED_GREEN: c_ulong = 0;
const SIGINT: c_int = 2;
const NOTIFY_DONE: c_int = 0;

unsafe extern "C" fn ip32_poweroff(data: *mut c_void) -> ! {
    let poweroff_func: Option<unsafe extern "C" fn(*mut platform_device)> =
        Some(ds1685_rtc_poweroff);

    if poweroff_func.is_none() {
        pr_emerg(b"RTC not available for power-off.  Spinning forever ...\0".as_ptr() as *const c_char);
    } else {
        (poweroff_func.unwrap())(data as *mut platform_device);
    }

    unreachable()
}

unsafe extern "C" fn ip32_machine_restart(_cmd: *mut c_char) -> ! {
    msleep(20);
    (*crime).control = CRIME_CONTROL_HARD_RESET;
    unreachable()
}

unsafe extern "C" fn blink_timeout(_unused: *mut timer_list) {
    let led = (*mace).perif.ctrl.misc ^ MACEISA_LED_RED;
    (*mace).perif.ctrl.misc = led;
    mod_timer(&raw mut blink_timer, jiffies + blink_timer_timeout);
}

unsafe extern "C" fn ip32_machine_halt() {
    ip32_poweroff(&raw mut ip32_rtc_device as *mut c_void);
}

unsafe extern "C" fn power_timeout(_unused: *mut timer_list) {
    ip32_poweroff(&raw mut ip32_rtc_device as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn ip32_prepare_poweroff() {
    if has_panicked != 0 { return; }

    if shutting_down != 0 || kill_cad_pid(SIGINT, 1) != 0 {
        /* No init process or button pressed twice.  */
        ip32_poweroff(&raw mut ip32_rtc_device as *mut c_void);
    }

    shutting_down = 1;
    blink_timer_timeout = POWERDOWN_FREQ;
    blink_timeout(&raw mut blink_timer);

    timer_setup(&raw mut power_timer, power_timeout, 0);
    power_timer.expires = jiffies + POWERDOWN_TIMEOUT * HZ;
    add_timer(&raw mut power_timer);
}

unsafe extern "C" fn panic_event(_this: *mut notifier_block, _event: c_ulong,
                                  _ptr: *mut c_void) -> c_int {
    if has_panicked != 0 { return NOTIFY_DONE; }
    has_panicked = 1;

    /* turn off the green LED */
    let led = (*mace).perif.ctrl.misc | MACEISA_LED_GREEN;
    (*mace).perif.ctrl.misc = led;

    blink_timer_timeout = PANIC_FREQ;
    blink_timeout(&raw mut blink_timer);

    NOTIFY_DONE
}

static mut panic_block: notifier_block = notifier_block {
    notifier_call: Some(panic_event),
};

unsafe extern "C" fn ip32_reboot_setup() -> c_int {
    /* turn on the green led only */
    let mut led = (*mace).perif.ctrl.misc;
    led |= MACEISA_LED_RED;
    led &= !MACEISA_LED_GREEN;
    (*mace).perif.ctrl.misc = led;

    _machine_restart = Some(ip32_machine_restart);
    _machine_halt = Some(ip32_machine_halt);
    pm_power_off = Some(ip32_machine_halt);

    timer_setup(&raw mut blink_timer, blink_timeout, 0);
    atomic_notifier_chain_register(&raw mut panic_notifier_list, &raw mut panic_block);

    0
}

// subsys_initcall(ip32_reboot_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
