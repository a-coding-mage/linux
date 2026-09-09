// SPDX-License-Identifier: GPL-2.0
/* Device wakeirq helper functions */

// Translated from wakeirq.c. Kernel declarations and constants are supplied
// by the surrounding build.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn dev_pm_attach_wake_irq(dev: *mut device, wirq: *mut wake_irq) -> c_int;
    fn device_wakeup_attach_irq(dev: *mut device, wirq: *mut wake_irq);
    fn device_wakeup_detach_irq(dev: *mut device);
    fn dev_WARN_ONCE(dev: *mut device, condition: bool, fmt: *const c_char) -> bool;
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: c_ulong);
    fn kzalloc_wake_irq() -> *mut wake_irq;
    fn kfree(ptr: *mut c_void);
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn irq_set_status_flags(irq: c_int, flags: c_uint);
    fn request_threaded_irq(irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
                            thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
                            flags: c_uint, name: *const c_char, dev: *mut c_void) -> c_int;
    fn irq_get_irq_data(irq: c_int) -> *mut c_void;
    fn irqd_is_wakeup_set(data: *mut c_void) -> bool;
    fn pm_wakeup_event(dev: *mut device, msec: c_uint);
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn enable_irq(irq: c_int);
    fn disable_irq_nosync(irq: c_int);
    fn enable_irq_wake(irq: c_int) -> c_int;
    fn disable_irq_wake(irq: c_int) -> c_int;
    fn device_may_wakeup(dev: *mut device) -> bool;
    fn devm_add_action_or_reset(dev: *mut device, action: Option<unsafe extern "C" fn(*mut c_void)>, data: *mut c_void) -> c_int;
}

#[repr(C)]
pub struct device { pub power: device_power }
#[repr(C)]
pub struct device_power { pub lock: c_void, pub wakeirq: *mut wake_irq }
#[repr(C)]
pub struct wake_irq {
    pub dev: *mut device,
    pub irq: c_int,
    pub name: *mut c_char,
    pub status: c_uint,
}
pub type irqreturn_t = c_int;

const EINVAL: c_int = 22;
const EEXIST: c_int = 17;
const ENOMEM: c_int = 12;
const IRQ_HANDLED: irqreturn_t = 1;
const IRQ_DISABLE_UNLAZY: c_uint = 1 << 20;
const IRQF_ONESHOT: c_uint = 1 << 0;
const IRQF_NO_AUTOEN: c_uint = 1 << 1;
const GFP_KERNEL: c_uint = 0;
const WAKE_IRQ_DEDICATED_ALLOCATED: c_uint = 1 << 0;
const WAKE_IRQ_DEDICATED_REVERSE: c_uint = 1 << 1;
const WAKE_IRQ_DEDICATED_MANAGED: c_uint = 1 << 2;
const WAKE_IRQ_DEDICATED_ENABLED: c_uint = 1 << 3;
const WAKE_IRQ_DEDICATED_MASK: c_uint = WAKE_IRQ_DEDICATED_ALLOCATED | WAKE_IRQ_DEDICATED_REVERSE | WAKE_IRQ_DEDICATED_MANAGED | WAKE_IRQ_DEDICATED_ENABLED;

#[inline]
unsafe fn dev_pm_attach_wake_irq_impl(dev: *mut device, wirq: *mut wake_irq) -> c_int {
    let mut flags = 0;
    if dev.is_null() || wirq.is_null() { return -EINVAL; }
    spin_lock_irqsave(&mut (*dev).power.lock, &mut flags);
    if dev_WARN_ONCE(dev, !(*dev).power.wakeirq.is_null(), c"wake irq already initialized\n".as_ptr()) {
        spin_unlock_irqrestore(&mut (*dev).power.lock, flags); return -EEXIST;
    }
    (*dev).power.wakeirq = wirq;
    device_wakeup_attach_irq(dev, wirq);
    spin_unlock_irqrestore(&mut (*dev).power.lock, flags); 0
}

#[no_mangle]
pub unsafe extern "C" fn dev_pm_set_wake_irq(dev: *mut device, irq: c_int) -> c_int {
    if irq < 0 { return -EINVAL; }
    let wirq = kzalloc_wake_irq();
    if wirq.is_null() { return -ENOMEM; }
    (*wirq).dev = dev; (*wirq).irq = irq;
    let err = dev_pm_attach_wake_irq_impl(dev, wirq);
    if err != 0 { kfree(wirq as *mut c_void); }
    err
}

#[no_mangle]
pub unsafe extern "C" fn dev_pm_clear_wake_irq(dev: *mut device) {
    let mut flags = 0;
    spin_lock_irqsave(&mut (*dev).power.lock, &mut flags);
    let wirq = (*dev).power.wakeirq;
    if wirq.is_null() { spin_unlock_irqrestore(&mut (*dev).power.lock, flags); return; }
    device_wakeup_detach_irq(dev); (*dev).power.wakeirq = core::ptr::null_mut();
    spin_unlock_irqrestore(&mut (*dev).power.lock, flags);
    if (*wirq).status & WAKE_IRQ_DEDICATED_ALLOCATED != 0 { free_irq((*wirq).irq, wirq as *mut c_void); (*wirq).status &= !WAKE_IRQ_DEDICATED_MASK; }
    kfree((*wirq).name as *mut c_void); kfree(wirq as *mut c_void);
}

unsafe extern "C" fn devm_pm_clear_wake_irq(dev: *mut c_void) { dev_pm_clear_wake_irq(dev as *mut device); }

#[no_mangle]
pub unsafe extern "C" fn devm_pm_set_wake_irq(dev: *mut device, irq: c_int) -> c_int {
    let ret = dev_pm_set_wake_irq(dev, irq); if ret != 0 { return ret; }
    devm_add_action_or_reset(dev, Some(devm_pm_clear_wake_irq), dev as *mut c_void)
}

unsafe extern "C" fn handle_threaded_wake_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    let wirq = data as *mut wake_irq;
    if irqd_is_wakeup_set(irq_get_irq_data(irq)) { pm_wakeup_event((*wirq).dev, 0); return IRQ_HANDLED; }
    let res = pm_runtime_resume((*wirq).dev);
    if res < 0 { dev_warn((*wirq).dev, c"wake IRQ with no resume: %i\n".as_ptr(), res); }
    IRQ_HANDLED
}

unsafe fn __dev_pm_set_dedicated_wake_irq(dev: *mut device, irq: c_int, flag: c_uint) -> c_int {
    if irq < 0 { return -EINVAL; }
    let wirq = kzalloc_wake_irq(); if wirq.is_null() { return -ENOMEM; }
    (*wirq).name = kasprintf(GFP_KERNEL, c"%s:wakeup".as_ptr(), dev_name(dev));
    if (*wirq).name.is_null() { kfree(wirq as *mut c_void); return -ENOMEM; }
    (*wirq).dev = dev; (*wirq).irq = irq;
    irq_set_status_flags(irq, IRQ_DISABLE_UNLAZY);
    let mut err = request_threaded_irq(irq, None, Some(handle_threaded_wake_irq), IRQF_ONESHOT | IRQF_NO_AUTOEN, (*wirq).name, wirq as *mut c_void);
    if err != 0 { kfree((*wirq).name as *mut c_void); kfree(wirq as *mut c_void); return err; }
    err = dev_pm_attach_wake_irq_impl(dev, wirq);
    if err != 0 { free_irq(irq, wirq as *mut c_void); kfree((*wirq).name as *mut c_void); kfree(wirq as *mut c_void); return err; }
    (*wirq).status = WAKE_IRQ_DEDICATED_ALLOCATED | flag; err
}

#[no_mangle] pub unsafe extern "C" fn dev_pm_set_dedicated_wake_irq(dev: *mut device, irq: c_int) -> c_int { __dev_pm_set_dedicated_wake_irq(dev, irq, 0) }
#[no_mangle] pub unsafe extern "C" fn dev_pm_set_dedicated_wake_irq_reverse(dev: *mut device, irq: c_int) -> c_int { __dev_pm_set_dedicated_wake_irq(dev, irq, WAKE_IRQ_DEDICATED_REVERSE) }

#[no_mangle]
pub unsafe extern "C" fn dev_pm_enable_wake_irq_check(dev: *mut device, can_change_status: bool) {
    let wirq = (*dev).power.wakeirq; if wirq.is_null() || (*wirq).status & WAKE_IRQ_DEDICATED_MASK == 0 { return; }
    if (*wirq).status & WAKE_IRQ_DEDICATED_MANAGED == 0 { if !can_change_status { return; } (*wirq).status |= WAKE_IRQ_DEDICATED_MANAGED; }
    if !can_change_status || (*wirq).status & WAKE_IRQ_DEDICATED_REVERSE == 0 { enable_irq((*wirq).irq); (*wirq).status |= WAKE_IRQ_DEDICATED_ENABLED; }
}

#[no_mangle]
pub unsafe extern "C" fn dev_pm_disable_wake_irq_check(dev: *mut device, cond_disable: bool) {
    let wirq = (*dev).power.wakeirq; if wirq.is_null() || (*wirq).status & WAKE_IRQ_DEDICATED_MASK == 0 { return; }
    if cond_disable && (*wirq).status & WAKE_IRQ_DEDICATED_REVERSE != 0 { return; }
    if (*wirq).status & WAKE_IRQ_DEDICATED_MANAGED != 0 { (*wirq).status &= !WAKE_IRQ_DEDICATED_ENABLED; disable_irq_nosync((*wirq).irq); }
}

#[no_mangle]
pub unsafe extern "C" fn dev_pm_enable_wake_irq_complete(dev: *mut device) {
    let wirq = (*dev).power.wakeirq; if wirq.is_null() || (*wirq).status & WAKE_IRQ_DEDICATED_MASK == 0 { return; }
    if (*wirq).status & WAKE_IRQ_DEDICATED_MANAGED != 0 && (*wirq).status & WAKE_IRQ_DEDICATED_REVERSE != 0 { enable_irq((*wirq).irq); (*wirq).status |= WAKE_IRQ_DEDICATED_ENABLED; }
}

#[no_mangle]
pub unsafe extern "C" fn dev_pm_arm_wake_irq(wirq: *mut wake_irq) {
    if wirq.is_null() { return; }
    if device_may_wakeup((*wirq).dev) { if (*wirq).status & WAKE_IRQ_DEDICATED_ALLOCATED != 0 && (*wirq).status & WAKE_IRQ_DEDICATED_ENABLED == 0 { enable_irq((*wirq).irq); } enable_irq_wake((*wirq).irq); }
}

#[no_mangle]
pub unsafe extern "C" fn dev_pm_disarm_wake_irq(wirq: *mut wake_irq) {
    if wirq.is_null() { return; }
    if device_may_wakeup((*wirq).dev) { disable_irq_wake((*wirq).irq); if (*wirq).status & WAKE_IRQ_DEDICATED_ALLOCATED != 0 && (*wirq).status & WAKE_IRQ_DEDICATED_ENABLED == 0 { disable_irq_nosync((*wirq).irq); } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
