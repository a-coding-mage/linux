// SPDX-License-Identifier: GPL-2.0-only
/*
 * Apple Onboard Audio feature call GPIO control
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 *
 * This file contains the GPIO control routines for
 * direct (through feature calls) access to the GPIO
 * registers.
 */

// C dependencies: <linux/of_irq.h>, <linux/interrupt.h>,
// <asm/pmac_feature.h>, "../aoa.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u32 = c_uint;
type irqreturn_t = c_int;
type notify_func_t = Option<unsafe extern "C" fn(*mut c_void)>;

const PMAC_FTR_READ_GPIO: c_int = 0;
const PMAC_FTR_WRITE_GPIO: c_int = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;

const AOA_NOTIFY_HEADPHONE: notify_type = 0;
const AOA_NOTIFY_LINE_IN: notify_type = 1;
const AOA_NOTIFY_LINE_OUT: notify_type = 2;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
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
pub struct mutex {
    _private: [u8; 0],
}

pub type notify_type = c_int;

#[repr(C)]
pub struct gpio_notification {
    pub work: delayed_work,
    pub mutex: mutex,
    pub notify: notify_func_t,
    pub data: *mut c_void,
    pub gpio_private: *mut c_void,
}

#[repr(C)]
pub struct gpio_runtime {
    pub implementation_private: c_int,
    pub headphone_notify: gpio_notification,
    pub line_in_notify: gpio_notification,
    pub line_out_notify: gpio_notification,
}

#[repr(C)]
pub struct gpio_methods {
    pub init: Option<unsafe extern "C" fn(*mut gpio_runtime)>,
    pub exit: Option<unsafe extern "C" fn(*mut gpio_runtime)>,
    pub all_amps_off: Option<unsafe extern "C" fn(*mut gpio_runtime)>,
    pub all_amps_restore: Option<unsafe extern "C" fn(*mut gpio_runtime)>,
    pub set_headphone: Option<unsafe extern "C" fn(*mut gpio_runtime, c_int)>,
    pub set_speakers: Option<unsafe extern "C" fn(*mut gpio_runtime, c_int)>,
    pub set_lineout: Option<unsafe extern "C" fn(*mut gpio_runtime, c_int)>,
    pub set_hw_reset: Option<unsafe extern "C" fn(*mut gpio_runtime, c_int)>,
    pub get_headphone: Option<unsafe extern "C" fn(*mut gpio_runtime) -> c_int>,
    pub get_speakers: Option<unsafe extern "C" fn(*mut gpio_runtime) -> c_int>,
    pub get_lineout: Option<unsafe extern "C" fn(*mut gpio_runtime) -> c_int>,
    pub set_master: Option<unsafe extern "C" fn(*mut gpio_runtime, c_int)>,
    pub get_master: Option<unsafe extern "C" fn(*mut gpio_runtime) -> c_int>,
    pub set_notify: Option<
        unsafe extern "C" fn(*mut gpio_runtime, notify_type, notify_func_t, *mut c_void) -> c_int,
    >,
    pub get_detect: Option<unsafe extern "C" fn(*mut gpio_runtime, notify_type) -> c_int>,
}

unsafe extern "C" {
    fn of_find_node_by_name(from: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_get_next_child(node: *mut device_node, prev: *mut device_node) -> *mut device_node;
    fn of_get_property(
        node: *mut device_node,
        name: *const c_char,
        lenp: *mut c_int,
    ) -> *const c_void;
    fn of_node_put(node: *mut device_node);
    fn irq_of_parse_and_map(dev: *mut device_node, index: c_int) -> c_int;
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn pmac_call_feature(
        selector: c_int,
        node: *mut device_node,
        param: c_int,
        value: c_int,
    ) -> c_int;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn mutex_init(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> c_int;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_int) -> c_int;
    fn request_irq(
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_uint,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
}

/* TODO: these are lots of global variables
 * that aren't used on most machines...
 * Move them into a dynamically allocated
 * structure and use that.
 */

/* these are the GPIO numbers (register addresses as offsets into
 * the GPIO space) */
static mut headphone_mute_gpio: c_int = 0;
static mut master_mute_gpio: c_int = 0;
static mut amp_mute_gpio: c_int = 0;
static mut lineout_mute_gpio: c_int = 0;
static mut hw_reset_gpio: c_int = 0;
static mut lineout_detect_gpio: c_int = 0;
static mut headphone_detect_gpio: c_int = 0;
static mut linein_detect_gpio: c_int = 0;

/* see the SWITCH_GPIO macro */
static mut headphone_mute_gpio_activestate: c_int = 0;
static mut master_mute_gpio_activestate: c_int = 0;
static mut amp_mute_gpio_activestate: c_int = 0;
static mut lineout_mute_gpio_activestate: c_int = 0;
static mut hw_reset_gpio_activestate: c_int = 0;
static mut lineout_detect_gpio_activestate: c_int = 0;
static mut headphone_detect_gpio_activestate: c_int = 0;
static mut linein_detect_gpio_activestate: c_int = 0;

/* node pointers that we save when getting the GPIO number
 * to get the interrupt later */
static mut lineout_detect_node: *mut device_node = ptr::null_mut();
static mut linein_detect_node: *mut device_node = ptr::null_mut();
static mut headphone_detect_node: *mut device_node = ptr::null_mut();

static mut lineout_detect_irq: c_int = 0;
static mut linein_detect_irq: c_int = 0;
static mut headphone_detect_irq: c_int = 0;

unsafe extern "C" fn get_gpio(
    name: *mut c_char,
    altname: *mut c_char,
    gpioptr: *mut c_int,
    gpioactiveptr: *mut c_int,
) -> *mut device_node {
    let mut np: *mut device_node;
    let gpio: *mut device_node;
    let mut reg: *const u32;
    let mut audio_gpio: *const c_char;

    *gpioptr = -1;

    /* check if we can get it the easy way ... */
    np = of_find_node_by_name(ptr::null_mut(), name);
    if np.is_null() {
        /* some machines have only gpioX/extint-gpioX nodes,
         * and an audio-gpio property saying what it is ...
         * So what we have to do is enumerate all children
         * of the gpio node and check them all. */
        gpio = of_find_node_by_name(ptr::null_mut(), c"gpio".as_ptr());
        if gpio.is_null() {
            return ptr::null_mut();
        }
        loop {
            np = of_get_next_child(gpio, np);
            if np.is_null() {
                break;
            }
            audio_gpio = of_get_property(np, c"audio-gpio".as_ptr(), ptr::null_mut()) as *const c_char;
            if audio_gpio.is_null() {
                continue;
            }
            if strcmp(audio_gpio, name) == 0 {
                break;
            }
            if !altname.is_null() && strcmp(audio_gpio, altname) == 0 {
                break;
            }
        }
        of_node_put(gpio);
        /* still not found, assume not there */
        if np.is_null() {
            return ptr::null_mut();
        }
    }

    reg = of_get_property(np, c"reg".as_ptr(), ptr::null_mut()) as *const u32;
    if reg.is_null() {
        of_node_put(np);
        return ptr::null_mut();
    }

    *gpioptr = *reg as c_int;

    /* this is a hack, usually the GPIOs 'reg' property
     * should have the offset based from the GPIO space
     * which is at 0x50, but apparently not always... */
    if *gpioptr < 0x50 {
        *gpioptr += 0x50;
    }

    reg = of_get_property(
        np,
        c"audio-gpio-active-state".as_ptr(),
        ptr::null_mut(),
    ) as *const u32;
    if reg.is_null() {
        /* Apple seems to default to 1, but
         * that doesn't seem right at least on most
         * machines. So until proven that the opposite
         * is necessary, we default to 0
         * (which, incidentally, snd-powermac also does...) */
        *gpioactiveptr = 0;
    } else {
        *gpioactiveptr = *reg as c_int;
    }

    np
}

unsafe extern "C" fn get_irq(np: *mut device_node, irqptr: *mut c_int) {
    if !np.is_null() {
        *irqptr = irq_of_parse_and_map(np, 0);
    } else {
        *irqptr = 0;
    }
}

/* 0x4 is outenable, 0x1 is out, thus 4 or 5 */
unsafe fn switch_gpio(v: c_int, on: c_int, active_state: c_int) -> c_int {
    (v & !1)
        | if on != 0 {
            if active_state == 0 { 4 } else { 5 }
        } else if active_state == 0 {
            5
        } else {
            4
        }
}

unsafe extern "C" fn ftr_gpio_set_headphone(rt: *mut gpio_runtime, on: c_int) {
    let mut v: c_int;

    if rt.is_null() {
        return;
    }

    if headphone_mute_gpio < 0 {
        return;
    }

    v = pmac_call_feature(PMAC_FTR_READ_GPIO, ptr::null_mut(), headphone_mute_gpio, 0);

    /* muted = !on... */
    v = switch_gpio(v, (on == 0) as c_int, headphone_mute_gpio_activestate);

    pmac_call_feature(PMAC_FTR_WRITE_GPIO, ptr::null_mut(), headphone_mute_gpio, v);

    (*rt).implementation_private &= !(1 << 0);
    (*rt).implementation_private |= ((on != 0) as c_int) << 0;
}

unsafe extern "C" fn ftr_gpio_get_headphone(rt: *mut gpio_runtime) -> c_int {
    if rt.is_null() {
        return 0;
    }
    ((*rt).implementation_private >> 0) & 1
}

unsafe extern "C" fn ftr_gpio_set_amp(rt: *mut gpio_runtime, on: c_int) {
    let mut v: c_int;

    if rt.is_null() {
        return;
    }

    if amp_mute_gpio < 0 {
        return;
    }

    v = pmac_call_feature(PMAC_FTR_READ_GPIO, ptr::null_mut(), amp_mute_gpio, 0);

    /* muted = !on... */
    v = switch_gpio(v, (on == 0) as c_int, amp_mute_gpio_activestate);

    pmac_call_feature(PMAC_FTR_WRITE_GPIO, ptr::null_mut(), amp_mute_gpio, v);

    (*rt).implementation_private &= !(1 << 1);
    (*rt).implementation_private |= ((on != 0) as c_int) << 1;
}

unsafe extern "C" fn ftr_gpio_get_amp(rt: *mut gpio_runtime) -> c_int {
    if rt.is_null() {
        return 0;
    }
    ((*rt).implementation_private >> 1) & 1
}

unsafe extern "C" fn ftr_gpio_set_lineout(rt: *mut gpio_runtime, on: c_int) {
    let mut v: c_int;

    if rt.is_null() {
        return;
    }

    if lineout_mute_gpio < 0 {
        return;
    }

    v = pmac_call_feature(PMAC_FTR_READ_GPIO, ptr::null_mut(), lineout_mute_gpio, 0);

    /* muted = !on... */
    v = switch_gpio(v, (on == 0) as c_int, lineout_mute_gpio_activestate);

    pmac_call_feature(PMAC_FTR_WRITE_GPIO, ptr::null_mut(), lineout_mute_gpio, v);

    (*rt).implementation_private &= !(1 << 2);
    (*rt).implementation_private |= ((on != 0) as c_int) << 2;
}

unsafe extern "C" fn ftr_gpio_get_lineout(rt: *mut gpio_runtime) -> c_int {
    if rt.is_null() {
        return 0;
    }
    ((*rt).implementation_private >> 2) & 1
}

unsafe extern "C" fn ftr_gpio_set_master(rt: *mut gpio_runtime, on: c_int) {
    let mut v: c_int;

    if rt.is_null() {
        return;
    }

    if master_mute_gpio < 0 {
        return;
    }

    v = pmac_call_feature(PMAC_FTR_READ_GPIO, ptr::null_mut(), master_mute_gpio, 0);

    /* muted = !on... */
    v = switch_gpio(v, (on == 0) as c_int, master_mute_gpio_activestate);

    pmac_call_feature(PMAC_FTR_WRITE_GPIO, ptr::null_mut(), master_mute_gpio, v);

    (*rt).implementation_private &= !(1 << 3);
    (*rt).implementation_private |= ((on != 0) as c_int) << 3;
}

unsafe extern "C" fn ftr_gpio_get_master(rt: *mut gpio_runtime) -> c_int {
    if rt.is_null() {
        return 0;
    }
    ((*rt).implementation_private >> 3) & 1
}

unsafe extern "C" fn ftr_gpio_set_hw_reset(rt: *mut gpio_runtime, on: c_int) {
    let mut v: c_int;

    if rt.is_null() {
        return;
    }
    if hw_reset_gpio < 0 {
        return;
    }

    v = pmac_call_feature(PMAC_FTR_READ_GPIO, ptr::null_mut(), hw_reset_gpio, 0);
    v = switch_gpio(v, on, hw_reset_gpio_activestate);
    pmac_call_feature(PMAC_FTR_WRITE_GPIO, ptr::null_mut(), hw_reset_gpio, v);
}

static mut methods: gpio_methods = gpio_methods {
    init: Some(ftr_gpio_init),
    exit: Some(ftr_gpio_exit),
    all_amps_off: Some(ftr_gpio_all_amps_off),
    all_amps_restore: Some(ftr_gpio_all_amps_restore),
    set_headphone: Some(ftr_gpio_set_headphone),
    set_speakers: Some(ftr_gpio_set_amp),
    set_lineout: Some(ftr_gpio_set_lineout),
    set_hw_reset: Some(ftr_gpio_set_hw_reset),
    get_headphone: Some(ftr_gpio_get_headphone),
    get_speakers: Some(ftr_gpio_get_amp),
    get_lineout: Some(ftr_gpio_get_lineout),
    set_master: None,
    get_master: None,
    set_notify: Some(ftr_set_notify),
    get_detect: Some(ftr_get_detect),
};

unsafe extern "C" fn ftr_gpio_all_amps_off(rt: *mut gpio_runtime) {
    let saved: c_int;

    if rt.is_null() {
        return;
    }
    saved = (*rt).implementation_private;
    ftr_gpio_set_headphone(rt, 0);
    ftr_gpio_set_amp(rt, 0);
    ftr_gpio_set_lineout(rt, 0);
    if methods.set_master.is_some() {
        ftr_gpio_set_master(rt, 0);
    }
    (*rt).implementation_private = saved;
}

unsafe extern "C" fn ftr_gpio_all_amps_restore(rt: *mut gpio_runtime) {
    let s: c_int;

    if rt.is_null() {
        return;
    }
    s = (*rt).implementation_private;
    ftr_gpio_set_headphone(rt, (s >> 0) & 1);
    ftr_gpio_set_amp(rt, (s >> 1) & 1);
    ftr_gpio_set_lineout(rt, (s >> 2) & 1);
    if methods.set_master.is_some() {
        ftr_gpio_set_master(rt, (s >> 3) & 1);
    }
}

unsafe extern "C" fn ftr_handle_notify(work: *mut work_struct) {
    let notif = work as *mut gpio_notification;

    mutex_lock(&mut (*notif).mutex);
    if let Some(notify) = (*notif).notify {
        notify((*notif).data);
    }
    mutex_unlock(&mut (*notif).mutex);
}

unsafe extern "C" fn gpio_enable_dual_edge(gpio: c_int) {
    let mut v: c_int;

    if gpio == -1 {
        return;
    }
    v = pmac_call_feature(PMAC_FTR_READ_GPIO, ptr::null_mut(), gpio, 0);
    v |= 0x80; /* enable dual edge */
    pmac_call_feature(PMAC_FTR_WRITE_GPIO, ptr::null_mut(), gpio, v);
}

unsafe extern "C" fn ftr_gpio_init(rt: *mut gpio_runtime) {
    get_gpio(
        c"headphone-mute".as_ptr() as *mut c_char,
        ptr::null_mut(),
        &mut headphone_mute_gpio,
        &mut headphone_mute_gpio_activestate,
    );
    get_gpio(
        c"amp-mute".as_ptr() as *mut c_char,
        ptr::null_mut(),
        &mut amp_mute_gpio,
        &mut amp_mute_gpio_activestate,
    );
    get_gpio(
        c"lineout-mute".as_ptr() as *mut c_char,
        ptr::null_mut(),
        &mut lineout_mute_gpio,
        &mut lineout_mute_gpio_activestate,
    );
    get_gpio(
        c"hw-reset".as_ptr() as *mut c_char,
        c"audio-hw-reset".as_ptr() as *mut c_char,
        &mut hw_reset_gpio,
        &mut hw_reset_gpio_activestate,
    );
    if !get_gpio(
        c"master-mute".as_ptr() as *mut c_char,
        ptr::null_mut(),
        &mut master_mute_gpio,
        &mut master_mute_gpio_activestate,
    )
    .is_null()
    {
        methods.set_master = Some(ftr_gpio_set_master);
        methods.get_master = Some(ftr_gpio_get_master);
    }

    headphone_detect_node = get_gpio(
        c"headphone-detect".as_ptr() as *mut c_char,
        ptr::null_mut(),
        &mut headphone_detect_gpio,
        &mut headphone_detect_gpio_activestate,
    );
    /* go Apple, and thanks for giving these different names
     * across the board... */
    lineout_detect_node = get_gpio(
        c"lineout-detect".as_ptr() as *mut c_char,
        c"line-output-detect".as_ptr() as *mut c_char,
        &mut lineout_detect_gpio,
        &mut lineout_detect_gpio_activestate,
    );
    linein_detect_node = get_gpio(
        c"linein-detect".as_ptr() as *mut c_char,
        c"line-input-detect".as_ptr() as *mut c_char,
        &mut linein_detect_gpio,
        &mut linein_detect_gpio_activestate,
    );

    gpio_enable_dual_edge(headphone_detect_gpio);
    gpio_enable_dual_edge(lineout_detect_gpio);
    gpio_enable_dual_edge(linein_detect_gpio);

    get_irq(headphone_detect_node, &mut headphone_detect_irq);
    get_irq(lineout_detect_node, &mut lineout_detect_irq);
    get_irq(linein_detect_node, &mut linein_detect_irq);

    ftr_gpio_all_amps_off(rt);
    (*rt).implementation_private = 0;
    INIT_DELAYED_WORK(&mut (*rt).headphone_notify.work, ftr_handle_notify);
    INIT_DELAYED_WORK(&mut (*rt).line_in_notify.work, ftr_handle_notify);
    INIT_DELAYED_WORK(&mut (*rt).line_out_notify.work, ftr_handle_notify);
    mutex_init(&mut (*rt).headphone_notify.mutex);
    mutex_init(&mut (*rt).line_in_notify.mutex);
    mutex_init(&mut (*rt).line_out_notify.mutex);
}

unsafe extern "C" fn ftr_gpio_exit(rt: *mut gpio_runtime) {
    ftr_gpio_all_amps_off(rt);
    (*rt).implementation_private = 0;
    if (*rt).headphone_notify.notify.is_some() {
        free_irq(headphone_detect_irq, &mut (*rt).headphone_notify as *mut _ as *mut c_void);
    }
    if !(*rt).line_in_notify.gpio_private.is_null() {
        free_irq(linein_detect_irq, &mut (*rt).line_in_notify as *mut _ as *mut c_void);
    }
    if !(*rt).line_out_notify.gpio_private.is_null() {
        free_irq(lineout_detect_irq, &mut (*rt).line_out_notify as *mut _ as *mut c_void);
    }
    cancel_delayed_work_sync(&mut (*rt).headphone_notify.work);
    cancel_delayed_work_sync(&mut (*rt).line_in_notify.work);
    cancel_delayed_work_sync(&mut (*rt).line_out_notify.work);
    mutex_destroy(&mut (*rt).headphone_notify.mutex);
    mutex_destroy(&mut (*rt).line_in_notify.mutex);
    mutex_destroy(&mut (*rt).line_out_notify.mutex);
}

unsafe extern "C" fn ftr_handle_notify_irq(_xx: c_int, data: *mut c_void) -> irqreturn_t {
    let notif = data as *mut gpio_notification;

    schedule_delayed_work(&mut (*notif).work, 0);

    IRQ_HANDLED
}

unsafe extern "C" fn ftr_set_notify(
    rt: *mut gpio_runtime,
    type_: notify_type,
    notify: notify_func_t,
    data: *mut c_void,
) -> c_int {
    let notif: *mut gpio_notification;
    let old: notify_func_t;
    let irq: c_int;
    let name: *mut c_char;
    let mut err: c_int = -EBUSY;

    match type_ {
        AOA_NOTIFY_HEADPHONE => {
            notif = &mut (*rt).headphone_notify;
            name = c"headphone-detect".as_ptr() as *mut c_char;
            irq = headphone_detect_irq;
        }
        AOA_NOTIFY_LINE_IN => {
            notif = &mut (*rt).line_in_notify;
            name = c"linein-detect".as_ptr() as *mut c_char;
            irq = linein_detect_irq;
        }
        AOA_NOTIFY_LINE_OUT => {
            notif = &mut (*rt).line_out_notify;
            name = c"lineout-detect".as_ptr() as *mut c_char;
            irq = lineout_detect_irq;
        }
        _ => {
            return -EINVAL;
        }
    }

    if irq == 0 {
        return -ENODEV;
    }

    mutex_lock(&mut (*notif).mutex);

    old = (*notif).notify;

    if old.is_none() && notify.is_none() {
        mutex_unlock(&mut (*notif).mutex);
        return 0;
    }

    if old.is_some() && notify.is_some() {
        if old == notify && (*notif).data == data {
            err = 0;
        }
        mutex_unlock(&mut (*notif).mutex);
        return err;
    }

    if old.is_some() && notify.is_none() {
        free_irq(irq, notif as *mut c_void);
    }

    if old.is_none() && notify.is_some() {
        err = request_irq(irq, ftr_handle_notify_irq, 0, name, notif as *mut c_void);
        if err != 0 {
            mutex_unlock(&mut (*notif).mutex);
            return err;
        }
    }

    (*notif).notify = notify;
    (*notif).data = data;

    mutex_unlock(&mut (*notif).mutex);

    0
}

unsafe extern "C" fn ftr_get_detect(rt: *mut gpio_runtime, type_: notify_type) -> c_int {
    let gpio: c_int;
    let ret: c_int;
    let active: c_int;

    match type_ {
        AOA_NOTIFY_HEADPHONE => {
            gpio = headphone_detect_gpio;
            active = headphone_detect_gpio_activestate;
        }
        AOA_NOTIFY_LINE_IN => {
            gpio = linein_detect_gpio;
            active = linein_detect_gpio_activestate;
        }
        AOA_NOTIFY_LINE_OUT => {
            gpio = lineout_detect_gpio;
            active = lineout_detect_gpio_activestate;
        }
        _ => {
            return -EINVAL;
        }
    }

    if gpio == -1 {
        return -ENODEV;
    }

    ret = pmac_call_feature(PMAC_FTR_READ_GPIO, ptr::null_mut(), gpio, 0);
    if ret < 0 {
        return ret;
    }
    ((((ret >> 1) & 1) == active) as c_int)
}

#[unsafe(no_mangle)]
pub static mut ftr_gpio_methods: *mut gpio_methods = unsafe { &mut methods };

// EXPORT_SYMBOL_GPL(ftr_gpio_methods);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
