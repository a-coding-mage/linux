// SPDX-License-Identifier: GPL-2.0+
//
// soc-jack.c  --  ALSA SoC jack handling
//
// Copyright 2008 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub type bool_t = bool;
pub type irqreturn_t = c_int;

pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const GFP_KERNEL: c_int = 0;
pub const IRQ_HANDLED: irqreturn_t = 1;
pub const IRQF_SHARED: c_ulong = 0x0000_0080;
pub const IRQF_TRIGGER_RISING: c_ulong = 0x0000_0001;
pub const IRQF_TRIGGER_FALLING: c_ulong = 0x0000_0002;
pub const PM_POST_SUSPEND: c_ulong = 0x0003;
pub const PM_POST_HIBERNATION: c_ulong = 0x0004;
pub const PM_POST_RESTORE: c_ulong = 0x0005;
pub const NOTIFY_DONE: c_int = 0x0000;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct blocking_notifier_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call:
        Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
    pub card: *mut snd_soc_card,
    pub mutex: mutex,
    pub status: c_int,
    pub pins: list_head,
    pub jack_zones: list_head,
    pub notifier: blocking_notifier_head,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub list: list_head,
    pub pin: *const c_char,
    pub mask: c_int,
    pub invert: c_int,
}

#[repr(C)]
pub struct snd_soc_jack_zone {
    pub list: list_head,
    pub min_mv: c_int,
    pub max_mv: c_int,
    pub jack_type: c_int,
}

#[repr(C)]
pub struct snd_soc_jack_gpio {
    pub name: *const c_char,
    pub report: c_int,
    pub invert: c_int,
    pub debounce_time: c_int,
    pub wake: c_int,
    pub jack_status_check: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub data: *mut c_void,
    pub jack: *mut snd_soc_jack,
    pub desc: *mut gpio_desc,
    pub gpiod_dev: *mut device,
    pub idx: c_int,
    pub work: delayed_work,
    pub pm_notifier: notifier_block,
}

#[repr(C)]
pub struct jack_gpio_tbl {
    pub count: c_int,
    pub jack: *mut snd_soc_jack,
    pub gpios: *mut snd_soc_jack_gpio,
}

unsafe extern "C" {
    static mut system_power_efficient_wq: *mut c_void;

    fn trace_snd_soc_jack_report(jack: *mut snd_soc_jack, mask: c_int, status: c_int);
    fn trace_snd_soc_jack_notify(jack: *mut snd_soc_jack, status: c_int);
    fn trace_snd_soc_jack_irq(name: *const c_char);
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn snd_soc_dapm_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char);
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char);
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context);
    fn blocking_notifier_call_chain(
        nh: *mut blocking_notifier_head,
        val: c_ulong,
        v: *mut c_void,
    ) -> c_int;
    fn snd_jack_report(jack: *mut snd_jack, status: c_int);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn snd_jack_add_new_kctl(jack: *mut snd_jack, name: *const c_char, mask: c_int) -> c_int;
    fn blocking_notifier_chain_register(
        nh: *mut blocking_notifier_head,
        nb: *mut notifier_block,
    ) -> c_int;
    fn blocking_notifier_chain_unregister(
        nh: *mut blocking_notifier_head,
        nb: *mut notifier_block,
    ) -> c_int;
    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> c_int;
    fn device_may_wakeup(dev: *mut device) -> c_int;
    fn pm_wakeup_event(dev: *mut device, msec: c_int);
    fn queue_delayed_work(wq: *mut c_void, dwork: *mut delayed_work, delay: c_ulong) -> bool_t;
    fn msecs_to_jiffies(msecs: c_int) -> c_ulong;
    fn gpiod_unexport(desc: *mut gpio_desc);
    fn unregister_pm_notifier(nb: *mut notifier_block) -> c_int;
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn gpiod_to_irq(desc: *mut gpio_desc) -> c_int;
    fn cancel_delayed_work_sync(dwork: *mut delayed_work) -> bool_t;
    fn gpiod_put(desc: *mut gpio_desc);
    fn devres_alloc(
        release: unsafe extern "C" fn(*mut device, *mut c_void),
        size: usize,
        gfp: c_int,
    ) -> *mut c_void;
    fn devres_add(dev: *mut device, res: *mut c_void);
    fn devres_free(res: *mut c_void);
    fn devres_destroy(
        dev: *mut device,
        release: unsafe extern "C" fn(*mut device, *mut c_void),
        match_: *mut c_void,
        match_data: *mut c_void,
    ) -> c_int;
    fn gpiod_get_index(
        dev: *mut device,
        con_id: *const c_char,
        idx: c_int,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn INIT_DELAYED_WORK(
        work: *mut delayed_work,
        func: unsafe extern "C" fn(*mut work_struct),
    );
    fn request_any_context_irq(
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn irq_set_irq_wake(irq: c_int, on: c_int) -> c_int;
    fn register_pm_notifier(nb: *mut notifier_block) -> c_int;
    fn gpiod_export(desc: *mut gpio_desc, direction_may_change: bool_t) -> c_int;
    fn schedule_delayed_work(dwork: *mut delayed_work, delay: c_ulong) -> bool_t;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

const GPIOD_IN: c_int = 0;

unsafe fn list_for_each_snd_soc_jack_pin(
    head: *mut list_head,
    mut f: impl FnMut(*mut snd_soc_jack_pin),
) {
    let mut pos = (*head).next;
    while pos != head {
        let pin = pos as *mut snd_soc_jack_pin;
        pos = (*pos).next;
        f(pin);
    }
}

unsafe fn list_for_each_snd_soc_jack_zone(
    head: *mut list_head,
    mut f: impl FnMut(*mut snd_soc_jack_zone) -> Option<c_int>,
) -> Option<c_int> {
    let mut pos = (*head).next;
    while pos != head {
        let zone = pos as *mut snd_soc_jack_zone;
        pos = (*pos).next;
        if let Some(ret) = f(zone) {
            return Some(ret);
        }
    }
    None
}

/**
 * snd_soc_jack_report - Report the current status for a jack
 *
 * @jack:   the jack
 * @status: a bitmask of enum snd_jack_type values that are currently detected.
 * @mask:   a bitmask of enum snd_jack_type values that being reported.
 *
 * If configured using snd_soc_jack_add_pins() then the associated
 * DAPM pins will be enabled or disabled as appropriate and DAPM
 * synchronised.
 *
 * Note: This function uses mutexes and should be called from a
 * context which can sleep (such as a workqueue).
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_jack_report(
    jack: *mut snd_soc_jack,
    status: c_int,
    mask: c_int,
) {
    let dapm: *mut snd_soc_dapm_context;
    let mut sync: c_int = 0;

    if jack.is_null() || (*jack).jack.is_null() {
        return;
    }
    trace_snd_soc_jack_report(jack, mask, status);

    dapm = snd_soc_card_to_dapm((*jack).card);

    mutex_lock(&mut (*jack).mutex);

    (*jack).status &= !mask;
    (*jack).status |= status & mask;

    trace_snd_soc_jack_notify(jack, status);

    list_for_each_snd_soc_jack_pin(&mut (*jack).pins, |pin| {
        let mut enable = (*pin).mask & (*jack).status;

        if (*pin).invert != 0 {
            enable = if enable == 0 { 1 } else { 0 };
        }

        if enable != 0 {
            snd_soc_dapm_enable_pin(dapm, (*pin).pin);
        } else {
            snd_soc_dapm_disable_pin(dapm, (*pin).pin);
        }

        /* we need to sync for this case only */
        sync = 1;
    });

    /* Report before the DAPM sync to help users updating micbias status */
    blocking_notifier_call_chain(&mut (*jack).notifier, (*jack).status as c_ulong, jack as *mut c_void);

    if sync != 0 {
        snd_soc_dapm_sync(dapm);
    }

    snd_jack_report((*jack).jack, (*jack).status);

    mutex_unlock(&mut (*jack).mutex);
}

/**
 * snd_soc_jack_add_zones - Associate voltage zones with jack
 *
 * @jack:  ASoC jack
 * @count: Number of zones
 * @zones:  Array of zones
 *
 * After this function has been called the zones specified in the
 * array will be associated with the jack.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_jack_add_zones(
    jack: *mut snd_soc_jack,
    count: c_int,
    zones: *mut snd_soc_jack_zone,
) -> c_int {
    let mut i: c_int = 0;

    while i < count {
        INIT_LIST_HEAD(&mut (*zones.add(i as usize)).list);
        list_add(
            &mut (*zones.add(i as usize)).list,
            &mut (*jack).jack_zones,
        );
        i += 1;
    }
    0
}

/**
 * snd_soc_jack_get_type - Based on the mic bias value, this function returns
 * the type of jack from the zones declared in the jack type
 *
 * @jack:  ASoC jack
 * @micbias_voltage:  mic bias voltage at adc channel when jack is plugged in
 *
 * Based on the mic bias value passed, this function helps identify
 * the type of jack from the already declared jack zones
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_jack_get_type(
    jack: *mut snd_soc_jack,
    micbias_voltage: c_int,
) -> c_int {
    list_for_each_snd_soc_jack_zone(&mut (*jack).jack_zones, |zone| {
        if micbias_voltage >= (*zone).min_mv && micbias_voltage < (*zone).max_mv {
            Some((*zone).jack_type)
        } else {
            None
        }
    })
    .unwrap_or(0)
}

/**
 * snd_soc_jack_add_pins - Associate DAPM pins with an ASoC jack
 *
 * @jack:  ASoC jack created with snd_soc_card_jack_new_pins()
 * @count: Number of pins
 * @pins:  Array of pins
 *
 * After this function has been called the DAPM pins specified in the
 * pins array will have their status updated to reflect the current
 * state of the jack whenever the jack status is updated.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_jack_add_pins(
    jack: *mut snd_soc_jack,
    count: c_int,
    pins: *mut snd_soc_jack_pin,
) -> c_int {
    let mut i: c_int = 0;

    while i < count {
        if (*pins.add(i as usize)).pin.is_null() {
            dev_err(
                (*(*jack).card).dev,
                b"ASoC: No name for pin %d\n\0".as_ptr() as *const c_char,
                i,
            );
            return -EINVAL;
        }
        if (*pins.add(i as usize)).mask == 0 {
            dev_err(
                (*(*jack).card).dev,
                b"ASoC: No mask for pin %d (%s)\n\0".as_ptr() as *const c_char,
                i,
                (*pins.add(i as usize)).pin,
            );
            return -EINVAL;
        }

        INIT_LIST_HEAD(&mut (*pins.add(i as usize)).list);
        list_add(&mut (*pins.add(i as usize)).list, &mut (*jack).pins);
        snd_jack_add_new_kctl(
            (*jack).jack,
            (*pins.add(i as usize)).pin,
            (*pins.add(i as usize)).mask,
        );
        i += 1;
    }

    /* Update to reflect the last reported status; canned jack
     * implementations are likely to set their state before the
     * card has an opportunity to associate pins.
     */
    snd_soc_jack_report(jack, 0, 0);

    0
}

/**
 * snd_soc_jack_notifier_register - Register a notifier for jack status
 *
 * @jack:  ASoC jack
 * @nb:    Notifier block to register
 *
 * Register for notification of the current status of the jack.  Note
 * that it is not possible to report additional jack events in the
 * callback from the notifier, this is intended to support
 * applications such as enabling electrical detection only when a
 * mechanical detection event has occurred.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_jack_notifier_register(
    jack: *mut snd_soc_jack,
    nb: *mut notifier_block,
) {
    blocking_notifier_chain_register(&mut (*jack).notifier, nb);
}

/**
 * snd_soc_jack_notifier_unregister - Unregister a notifier for jack status
 *
 * @jack:  ASoC jack
 * @nb:    Notifier block to unregister
 *
 * Stop notifying for status changes.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_jack_notifier_unregister(
    jack: *mut snd_soc_jack,
    nb: *mut notifier_block,
) {
    blocking_notifier_chain_unregister(&mut (*jack).notifier, nb);
}

/* CONFIG_GPIOLIB */

/* gpio detect */
unsafe extern "C" fn snd_soc_jack_gpio_detect(gpio: *mut snd_soc_jack_gpio) {
    let jack: *mut snd_soc_jack = (*gpio).jack;
    let mut enable: c_int;
    let mut report: c_int;

    enable = gpiod_get_value_cansleep((*gpio).desc);
    if (*gpio).invert != 0 {
        enable = if enable == 0 { 1 } else { 0 };
    }

    if enable != 0 {
        report = (*gpio).report;
    } else {
        report = 0;
    }

    if let Some(jack_status_check) = (*gpio).jack_status_check {
        report = jack_status_check((*gpio).data);
    }

    snd_soc_jack_report(jack, report, (*gpio).report);
}

/* irq handler for gpio pin */
unsafe extern "C" fn gpio_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let gpio: *mut snd_soc_jack_gpio = data as *mut snd_soc_jack_gpio;
    let dev: *mut device = (*(*(*gpio).jack).card).dev;

    trace_snd_soc_jack_irq((*gpio).name);

    if device_may_wakeup(dev) != 0 {
        pm_wakeup_event(dev, (*gpio).debounce_time + 50);
    }

    queue_delayed_work(
        system_power_efficient_wq,
        &mut (*gpio).work,
        msecs_to_jiffies((*gpio).debounce_time),
    );

    IRQ_HANDLED
}

/* gpio work */
unsafe extern "C" fn gpio_work(work: *mut work_struct) {
    let gpio: *mut snd_soc_jack_gpio =
        (work as *mut u8).sub(core::mem::offset_of!(snd_soc_jack_gpio, work)) as *mut snd_soc_jack_gpio;

    snd_soc_jack_gpio_detect(gpio);
}

unsafe extern "C" fn snd_soc_jack_pm_notifier(
    nb: *mut notifier_block,
    action: c_ulong,
    _data: *mut c_void,
) -> c_int {
    let gpio: *mut snd_soc_jack_gpio =
        (nb as *mut u8).sub(core::mem::offset_of!(snd_soc_jack_gpio, pm_notifier)) as *mut snd_soc_jack_gpio;

    match action {
        PM_POST_SUSPEND | PM_POST_HIBERNATION | PM_POST_RESTORE => {
            /*
             * Use workqueue so we do not have to care about running
             * concurrently with work triggered by the interrupt handler.
             */
            queue_delayed_work(system_power_efficient_wq, &mut (*gpio).work, 0);
        }
        _ => {}
    }

    NOTIFY_DONE
}

unsafe extern "C" fn jack_free_gpios(
    _jack: *mut snd_soc_jack,
    count: c_int,
    gpios: *mut snd_soc_jack_gpio,
) {
    let mut i: c_int = 0;

    while i < count {
        gpiod_unexport((*gpios.add(i as usize)).desc);
        unregister_pm_notifier(&mut (*gpios.add(i as usize)).pm_notifier);
        free_irq(
            gpiod_to_irq((*gpios.add(i as usize)).desc),
            gpios.add(i as usize) as *mut c_void,
        );
        cancel_delayed_work_sync(&mut (*gpios.add(i as usize)).work);
        gpiod_put((*gpios.add(i as usize)).desc);
        (*gpios.add(i as usize)).jack = core::ptr::null_mut();
        i += 1;
    }
}

unsafe extern "C" fn jack_devres_free_gpios(_dev: *mut device, res: *mut c_void) {
    let tbl: *mut jack_gpio_tbl = res as *mut jack_gpio_tbl;

    jack_free_gpios((*tbl).jack, (*tbl).count, (*tbl).gpios);
}

/**
 * snd_soc_jack_add_gpios - Associate GPIO pins with an ASoC jack
 *
 * @jack:  ASoC jack
 * @count: number of pins
 * @gpios: array of gpio pins
 *
 * This function will request gpio, set data direction and request irq
 * for each gpio in the array.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_jack_add_gpios(
    jack: *mut snd_soc_jack,
    count: c_int,
    gpios: *mut snd_soc_jack_gpio,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;
    let tbl: *mut jack_gpio_tbl;

    tbl = devres_alloc(
        jack_devres_free_gpios,
        core::mem::size_of::<jack_gpio_tbl>(),
        GFP_KERNEL,
    ) as *mut jack_gpio_tbl;
    if tbl.is_null() {
        return -ENOMEM;
    }
    (*tbl).jack = jack;
    (*tbl).count = count;
    (*tbl).gpios = gpios;

    i = 0;
    while i < count {
        if (*gpios.add(i as usize)).name.is_null() {
            dev_err(
                (*(*jack).card).dev,
                b"ASoC: No name for gpio at index %d\n\0".as_ptr() as *const c_char,
                i,
            );
            ret = -EINVAL;
            jack_free_gpios(jack, i, gpios);
            devres_free(tbl as *mut c_void);
            return ret;
        }

        if !(*gpios.add(i as usize)).desc.is_null() {
            /* Already have a GPIO descriptor. */
        } else if !(*gpios.add(i as usize)).gpiod_dev.is_null() {
            /* Get a GPIO descriptor */
            (*gpios.add(i as usize)).desc = gpiod_get_index(
                (*gpios.add(i as usize)).gpiod_dev,
                (*gpios.add(i as usize)).name,
                (*gpios.add(i as usize)).idx,
                GPIOD_IN,
            );
            if IS_ERR((*gpios.add(i as usize)).desc as *const c_void) != 0 {
                ret = PTR_ERR((*gpios.add(i as usize)).desc as *const c_void);
                dev_err(
                    (*gpios.add(i as usize)).gpiod_dev,
                    b"ASoC: Cannot get gpio at index %d: %d\0".as_ptr() as *const c_char,
                    i,
                    ret,
                );
                jack_free_gpios(jack, i, gpios);
                devres_free(tbl as *mut c_void);
                return ret;
            }
        } else {
            dev_err(
                (*(*jack).card).dev,
                b"ASoC: Invalid gpio at index %d\n\0".as_ptr() as *const c_char,
                i,
            );
            ret = -EINVAL;
            jack_free_gpios(jack, i, gpios);
            devres_free(tbl as *mut c_void);
            return ret;
        }

        INIT_DELAYED_WORK(&mut (*gpios.add(i as usize)).work, gpio_work);
        (*gpios.add(i as usize)).jack = jack;

        ret = request_any_context_irq(
            gpiod_to_irq((*gpios.add(i as usize)).desc),
            gpio_handler,
            IRQF_SHARED | IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING,
            (*gpios.add(i as usize)).name,
            gpios.add(i as usize) as *mut c_void,
        );
        if ret < 0 {
            jack_free_gpios(jack, i, gpios);
            devres_free(tbl as *mut c_void);
            return ret;
        }

        if (*gpios.add(i as usize)).wake != 0 {
            ret = irq_set_irq_wake(gpiod_to_irq((*gpios.add(i as usize)).desc), 1);
            if ret != 0 {
                dev_err(
                    (*(*jack).card).dev,
                    b"ASoC: Failed to mark GPIO at index %d as wake source: %d\n\0".as_ptr()
                        as *const c_char,
                    i,
                    ret,
                );
            }
        }

        /*
         * Register PM notifier so we do not miss state transitions
         * happening while system is asleep.
         */
        (*gpios.add(i as usize)).pm_notifier.notifier_call = Some(snd_soc_jack_pm_notifier);
        register_pm_notifier(&mut (*gpios.add(i as usize)).pm_notifier);

        /* Expose GPIO value over sysfs for diagnostic purposes */
        gpiod_export((*gpios.add(i as usize)).desc, false);

        /* Update initial jack status */
        schedule_delayed_work(
            &mut (*gpios.add(i as usize)).work,
            msecs_to_jiffies((*gpios.add(i as usize)).debounce_time),
        );

        i += 1;
    }

    devres_add((*(*jack).card).dev, tbl as *mut c_void);
    0
}

/**
 * snd_soc_jack_add_gpiods - Associate GPIO descriptor pins with an ASoC jack
 *
 * @gpiod_dev: GPIO consumer device
 * @jack:      ASoC jack
 * @count:     number of pins
 * @gpios:     array of gpio pins
 *
 * This function will request gpio, set data direction and request irq
 * for each gpio in the array.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_jack_add_gpiods(
    gpiod_dev: *mut device,
    jack: *mut snd_soc_jack,
    count: c_int,
    gpios: *mut snd_soc_jack_gpio,
) -> c_int {
    let mut i: c_int = 0;

    while i < count {
        (*gpios.add(i as usize)).gpiod_dev = gpiod_dev;
        i += 1;
    }

    snd_soc_jack_add_gpios(jack, count, gpios)
}

/**
 * snd_soc_jack_free_gpios - Release GPIO pins' resources of an ASoC jack
 *
 * @jack:  ASoC jack
 * @count: number of pins
 * @gpios: array of gpio pins
 *
 * Release gpio and irq resources for gpio pins associated with an ASoC jack.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_jack_free_gpios(
    jack: *mut snd_soc_jack,
    count: c_int,
    gpios: *mut snd_soc_jack_gpio,
) {
    jack_free_gpios(jack, count, gpios);
    devres_destroy(
        (*(*jack).card).dev,
        jack_devres_free_gpios,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
