/* SPDX-License-Identifier: GPL-2.0 */
// Dependency: linux/pm_qos.h

unsafe fn device_pm_init_common(dev: *mut device) {
    if !(*dev).power.early_init {
        spin_lock_init(&mut (*dev).power.lock);
        (*dev).power.qos = core::ptr::null_mut();
        (*dev).power.early_init = true;
    }
}

// CONFIG_PM
unsafe fn pm_runtime_early_init(dev: *mut device) {
    (*dev).power.disable_depth = 1;
    device_pm_init_common(dev);
}

unsafe extern "C" {
    fn pm_runtime_init(dev: *mut device);
    fn pm_runtime_reinit(dev: *mut device);
    fn pm_runtime_remove(dev: *mut device);
    fn pm_runtime_active_time(dev: *mut device) -> u64;
}

const WAKE_IRQ_DEDICATED_ALLOCATED: u32 = 1u32 << 0;
const WAKE_IRQ_DEDICATED_MANAGED: u32 = 1u32 << 1;
const WAKE_IRQ_DEDICATED_REVERSE: u32 = 1u32 << 2;
const WAKE_IRQ_DEDICATED_MASK: u32 = WAKE_IRQ_DEDICATED_ALLOCATED
    | WAKE_IRQ_DEDICATED_MANAGED
    | WAKE_IRQ_DEDICATED_REVERSE;
const WAKE_IRQ_DEDICATED_ENABLED: u32 = 1u32 << 3;

#[repr(C)]
struct wake_irq {
    dev: *mut device,
    status: u32,
    irq: i32,
    name: *const core::ffi::c_char,
}

unsafe extern "C" {
    fn dev_pm_arm_wake_irq(wirq: *mut wake_irq);
    fn dev_pm_disarm_wake_irq(wirq: *mut wake_irq);
    fn dev_pm_enable_wake_irq_check(dev: *mut device, can_change_status: bool);
    fn dev_pm_disable_wake_irq_check(dev: *mut device, cond_disable: bool);
    fn dev_pm_enable_wake_irq_complete(dev: *mut device);
}

// CONFIG_PM_SLEEP
unsafe extern "C" {
    fn device_wakeup_attach_irq(dev: *mut device, wakeirq: *mut wake_irq);
    fn device_wakeup_detach_irq(dev: *mut device);
    fn device_wakeup_arm_wake_irqs();
    fn device_wakeup_disarm_wake_irqs();
}

// !CONFIG_PM_SLEEP
#[allow(dead_code)]
unsafe fn device_wakeup_attach_irq_no_sleep(_dev: *mut device, _wakeirq: *mut wake_irq) {}
#[allow(dead_code)]
unsafe fn device_wakeup_detach_irq_no_sleep(_dev: *mut device) {}

/* sysfs.c */
unsafe extern "C" {
    fn dpm_sysfs_add(dev: *mut device) -> i32;
    fn dpm_sysfs_remove(dev: *mut device);
    fn rpm_sysfs_remove(dev: *mut device);
    fn wakeup_sysfs_add(dev: *mut device) -> i32;
    fn wakeup_sysfs_remove(dev: *mut device);
    fn pm_qos_sysfs_add_resume_latency(dev: *mut device) -> i32;
    fn pm_qos_sysfs_remove_resume_latency(dev: *mut device);
    fn pm_qos_sysfs_add_flags(dev: *mut device) -> i32;
    fn pm_qos_sysfs_remove_flags(dev: *mut device);
    fn pm_qos_sysfs_add_latency_tolerance(dev: *mut device) -> i32;
    fn pm_qos_sysfs_remove_latency_tolerance(dev: *mut device);
    fn dpm_sysfs_change_owner(dev: *mut device, kuid: kuid_t, kgid: kgid_t) -> i32;
}

// !CONFIG_PM
unsafe fn pm_runtime_early_init_no_pm(dev: *mut device) { device_pm_init_common(dev); }
unsafe fn pm_runtime_init_no_pm(_dev: *mut device) {}
unsafe fn pm_runtime_reinit_no_pm(_dev: *mut device) {}
unsafe fn pm_runtime_remove_no_pm(_dev: *mut device) {}
unsafe fn dpm_sysfs_add_no_pm(_dev: *mut device) -> i32 { 0 }
unsafe fn dpm_sysfs_remove_no_pm(_dev: *mut device) {}
unsafe fn dpm_sysfs_change_owner_no_pm(_dev: *mut device, _kuid: kuid_t, _kgid: kgid_t) -> i32 { 0 }

// CONFIG_PM_SLEEP
unsafe extern "C" {
    static mut pm_async_enabled: i32;
    static mut dpm_list: list_head;
    fn device_pm_sleep_init(dev: *mut device);
    fn device_pm_add(dev: *mut device);
    fn device_pm_remove(dev: *mut device);
    fn device_pm_move_before(deva: *mut device, devb: *mut device);
    fn device_pm_move_after(deva: *mut device, devb: *mut device);
    fn device_pm_move_last(dev: *mut device);
    fn device_pm_check_callbacks(dev: *mut device);
}

unsafe fn to_device(entry: *mut list_head) -> *mut device {
    container_of(entry, (*((core::ptr::null_mut::<device>()))).power.entry)
}

unsafe fn device_pm_initialized(dev: *mut device) -> bool { (*dev).power.in_dpm_list }

unsafe extern "C" {
    fn wakeup_source_sysfs_add(parent: *mut device, ws: *mut wakeup_source) -> i32;
    fn wakeup_source_sysfs_remove(ws: *mut wakeup_source);
    fn pm_wakeup_source_sysfs_add(parent: *mut device) -> i32;
}

// !CONFIG_PM_SLEEP
unsafe fn device_pm_sleep_init_no_sleep(_dev: *mut device) {}
unsafe fn device_pm_add_no_sleep(_dev: *mut device) {}
unsafe fn device_pm_remove_no_sleep(dev: *mut device) { pm_runtime_remove(dev); }
unsafe fn device_pm_move_before_no_sleep(_deva: *mut device, _devb: *mut device) {}
unsafe fn device_pm_move_after_no_sleep(_deva: *mut device, _devb: *mut device) {}
unsafe fn device_pm_move_last_no_sleep(_dev: *mut device) {}
unsafe fn device_pm_check_callbacks_no_sleep(_dev: *mut device) {}
unsafe fn device_pm_initialized_no_sleep(dev: *mut device) -> bool { device_is_registered(dev) }
unsafe fn pm_wakeup_source_sysfs_add_no_sleep(_parent: *mut device) -> i32 { 0 }

unsafe fn device_pm_init(dev: *mut device) {
    device_pm_init_common(dev);
    device_pm_sleep_init(dev);
    pm_runtime_init(dev);
}

// CONFIG_BPF_SYSCALL
#[repr(C)]
struct bpf_ws_lock {}
unsafe extern "C" {
    fn bpf_wakeup_sources_read_lock() -> *mut bpf_ws_lock;
    fn bpf_wakeup_sources_read_unlock(lock: *mut bpf_ws_lock);
    fn bpf_wakeup_sources_get_head() -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
