/* SPDX-License-Identifier: GPL-2.0 */
/* Generic RTC interface. */

// C dependencies supplied by the surrounding kernel translation.

unsafe extern "C" {
    pub fn rtc_month_days(month: ::core::ffi::c_uint, year: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn rtc_year_days(day: ::core::ffi::c_uint, month: ::core::ffi::c_uint, year: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn rtc_valid_tm(tm: *mut rtc_time) -> ::core::ffi::c_int;
    pub fn rtc_tm_to_time64(tm: *mut rtc_time) -> time64_t;
    pub fn rtc_time64_to_tm(time: time64_t, tm: *mut rtc_time);
    pub fn rtc_tm_to_ktime(tm: rtc_time) -> ktime_t;
    pub fn rtc_ktime_to_tm(kt: ktime_t) -> rtc_time;
}

#[inline]
pub unsafe fn rtc_tm_sub(lhs: *mut rtc_time, rhs: *mut rtc_time) -> time64_t {
    rtc_tm_to_time64(lhs) - rtc_tm_to_time64(rhs)
}

#[repr(C)]
pub struct rtc_class_ops {
    pub ioctl: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_uint, ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    pub read_time: Option<unsafe extern "C" fn(*mut device, *mut rtc_time) -> ::core::ffi::c_int>,
    pub set_time: Option<unsafe extern "C" fn(*mut device, *mut rtc_time) -> ::core::ffi::c_int>,
    pub read_alarm: Option<unsafe extern "C" fn(*mut device, *mut rtc_wkalrm) -> ::core::ffi::c_int>,
    pub set_alarm: Option<unsafe extern "C" fn(*mut device, *mut rtc_wkalrm) -> ::core::ffi::c_int>,
    pub proc: Option<unsafe extern "C" fn(*mut device, *mut seq_file) -> ::core::ffi::c_int>,
    pub alarm_irq_enable: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub read_offset: Option<unsafe extern "C" fn(*mut device, *mut ::core::ffi::c_long) -> ::core::ffi::c_int>,
    pub set_offset: Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_long) -> ::core::ffi::c_int>,
    pub param_get: Option<unsafe extern "C" fn(*mut device, *mut rtc_param) -> ::core::ffi::c_int>,
    pub param_set: Option<unsafe extern "C" fn(*mut device, *mut rtc_param) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct rtc_timer {
    pub node: timerqueue_node,
    pub period: ktime_t,
    pub func: Option<unsafe extern "C" fn(*mut rtc_device)>,
    pub rtc: *mut rtc_device,
    pub enabled: ::core::ffi::c_int,
}

pub const RTC_DEV_BUSY: ::core::ffi::c_uint = 0;
pub const RTC_NO_CDEV: ::core::ffi::c_uint = 1;

#[repr(C)]
pub struct rtc_device {
    pub dev: device,
    pub owner: *mut module,
    pub id: ::core::ffi::c_int,
    pub ops: *const rtc_class_ops,
    pub ops_lock: mutex,
    pub char_dev: cdev,
    pub flags: ::core::ffi::c_ulong,
    pub irq_data: ::core::ffi::c_ulong,
    pub irq_lock: spinlock_t,
    pub irq_queue: wait_queue_head_t,
    pub async_queue: *mut fasync_struct,
    pub irq_freq: ::core::ffi::c_int,
    pub max_user_freq: ::core::ffi::c_int,
    pub timerqueue: timerqueue_head,
    pub aie_timer: rtc_timer,
    pub uie_rtctimer: rtc_timer,
    pub pie_timer: hrtimer,
    pub pie_enabled: ::core::ffi::c_int,
    pub irqwork: work_struct,
    pub set_offset_nsec: ::core::ffi::c_ulong,
    pub features: [::core::ffi::c_ulong; BITS_TO_LONGS(RTC_FEATURE_CNT) as usize],
    pub range_min: time64_t,
    pub range_max: timeu64_t,
    pub alarm_offset_max: timeu64_t,
    pub start_secs: time64_t,
    pub offset_secs: time64_t,
    pub set_start_time: bool,
    #[cfg(feature = "CONFIG_RTC_INTF_DEV_UIE_EMUL")]
    pub uie_task: work_struct,
    #[cfg(feature = "CONFIG_RTC_INTF_DEV_UIE_EMUL")]
    pub uie_timer: timer_list,
    #[cfg(feature = "CONFIG_RTC_INTF_DEV_UIE_EMUL")]
    pub oldsecs: ::core::ffi::c_uint,
    #[cfg(feature = "CONFIG_RTC_INTF_DEV_UIE_EMUL")]
    pub uie_irq_active: ::core::ffi::c_uint,
    #[cfg(feature = "CONFIG_RTC_INTF_DEV_UIE_EMUL")]
    pub stop_uie_polling: ::core::ffi::c_uint,
    #[cfg(feature = "CONFIG_RTC_INTF_DEV_UIE_EMUL")]
    pub uie_task_active: ::core::ffi::c_uint,
    #[cfg(feature = "CONFIG_RTC_INTF_DEV_UIE_EMUL")]
    pub uie_timer_active: ::core::ffi::c_uint,
}

// The C container_of macro preserves the enclosing rtc_device pointer.
pub unsafe fn to_rtc_device(d: *mut device) -> *mut rtc_device {
    container_of!(d, rtc_device, dev)
}

#[inline]
pub unsafe fn rtc_lock(d: *mut rtc_device) { mutex_lock!(&mut (*d).ops_lock); }
#[inline]
pub unsafe fn rtc_unlock(d: *mut rtc_device) { mutex_unlock!(&mut (*d).ops_lock); }

pub const RTC_TIMESTAMP_BEGIN_0000: u64 = (-62167219200i64) as u64;
pub const RTC_TIMESTAMP_BEGIN_1900: i64 = -2208988800;
pub const RTC_TIMESTAMP_EPOCH_GPS: i64 = 315964800;
pub const RTC_TIMESTAMP_BEGIN_2000: i64 = 946684800;
pub const RTC_TIMESTAMP_END_2063: i64 = 2966371199;
pub const RTC_TIMESTAMP_END_2079: i64 = 3471292799;
pub const RTC_TIMESTAMP_END_2099: i64 = 4102444799;
pub const RTC_TIMESTAMP_END_2199: i64 = 7258118399;
pub const RTC_TIMESTAMP_END_9999: i64 = 253402300799;

#[inline]
pub unsafe fn devm_rtc_register_device(device: *mut rtc_device) -> ::core::ffi::c_int {
    __devm_rtc_register_device(THIS_MODULE, device)
}

unsafe extern "C" {
    pub static rtc_class: class;
    pub fn devm_rtc_device_register(dev: *mut device, name: *const ::core::ffi::c_char, ops: *const rtc_class_ops, owner: *mut module) -> *mut rtc_device;
    pub fn devm_rtc_allocate_device(dev: *mut device) -> *mut rtc_device;
    pub fn __devm_rtc_register_device(owner: *mut module, rtc: *mut rtc_device) -> ::core::ffi::c_int;
    pub fn rtc_read_time(rtc: *mut rtc_device, tm: *mut rtc_time) -> ::core::ffi::c_int;
    pub fn rtc_set_time(rtc: *mut rtc_device, tm: *mut rtc_time) -> ::core::ffi::c_int;
    pub fn __rtc_read_alarm(rtc: *mut rtc_device, alarm: *mut rtc_wkalrm) -> ::core::ffi::c_int;
    pub fn rtc_read_alarm(rtc: *mut rtc_device, alarm: *mut rtc_wkalrm) -> ::core::ffi::c_int;
    pub fn rtc_read_next_alarm(rtc: *mut rtc_device, alarm: *mut rtc_wkalrm) -> ::core::ffi::c_int;
    pub fn rtc_set_alarm(rtc: *mut rtc_device, alarm: *mut rtc_wkalrm) -> ::core::ffi::c_int;
    pub fn rtc_initialize_alarm(rtc: *mut rtc_device, alarm: *mut rtc_wkalrm) -> ::core::ffi::c_int;
    pub fn rtc_update_irq(rtc: *mut rtc_device, num: ::core::ffi::c_ulong, events: ::core::ffi::c_ulong);
    pub fn rtc_class_open(name: *const ::core::ffi::c_char) -> *mut rtc_device;
    pub fn rtc_class_close(rtc: *mut rtc_device);
    pub fn rtc_irq_set_state(rtc: *mut rtc_device, enabled: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn rtc_irq_set_freq(rtc: *mut rtc_device, freq: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn rtc_update_irq_enable(rtc: *mut rtc_device, enabled: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn rtc_alarm_irq_enable(rtc: *mut rtc_device, enabled: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn rtc_dev_update_irq_enable_emul(rtc: *mut rtc_device, enabled: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn rtc_handle_legacy_irq(rtc: *mut rtc_device, num: ::core::ffi::c_int, mode: ::core::ffi::c_int);
    pub fn rtc_aie_update_irq(rtc: *mut rtc_device);
    pub fn rtc_uie_update_irq(rtc: *mut rtc_device);
    pub fn rtc_pie_update_irq(timer: *mut hrtimer) -> hrtimer_restart;
    pub fn rtc_timer_init(timer: *mut rtc_timer, f: Option<unsafe extern "C" fn(*mut rtc_device)>, rtc: *mut rtc_device);
    pub fn rtc_timer_start(rtc: *mut rtc_device, timer: *mut rtc_timer, expires: ktime_t, period: ktime_t) -> ::core::ffi::c_int;
    pub fn rtc_timer_cancel(rtc: *mut rtc_device, timer: *mut rtc_timer);
    pub fn rtc_read_offset(rtc: *mut rtc_device, offset: *mut ::core::ffi::c_long) -> ::core::ffi::c_int;
    pub fn rtc_set_offset(rtc: *mut rtc_device, offset: ::core::ffi::c_long) -> ::core::ffi::c_int;
    pub fn rtc_timer_do_work(work: *mut work_struct);
}

#[inline]
pub const fn is_leap_year(year: ::core::ffi::c_uint) -> bool {
    (!(year % 4) && (year % 100 != 0)) || (year % 400 == 0)
}

#[inline]
pub unsafe fn rtc_bound_alarmtime(rtc: *mut rtc_device, requested: ktime_t) -> ktime_t {
    if (*rtc).alarm_offset_max != 0 && (*rtc).alarm_offset_max * MSEC_PER_SEC < ktime_to_ms(requested) {
        ms_to_ktime((*rtc).alarm_offset_max * MSEC_PER_SEC)
    } else { requested }
}

#[cfg(not(feature = "CONFIG_RTC_HCTOSYS_DEVICE"))]
pub const rtc_hctosys_ret: ::core::ffi::c_int = -ENODEV;

#[cfg(feature = "CONFIG_RTC_NVMEM")]
unsafe extern "C" { pub fn devm_rtc_nvmem_register(rtc: *mut rtc_device, nvmem_config: *mut nvmem_config) -> ::core::ffi::c_int; }
#[cfg(not(feature = "CONFIG_RTC_NVMEM"))]
#[inline] pub unsafe fn devm_rtc_nvmem_register(_: *mut rtc_device, _: *mut nvmem_config) -> ::core::ffi::c_int { 0 }

#[cfg(feature = "CONFIG_RTC_INTF_SYSFS")]
unsafe extern "C" {
    pub fn rtc_add_group(rtc: *mut rtc_device, grp: *const attribute_group) -> ::core::ffi::c_int;
    pub fn rtc_add_groups(rtc: *mut rtc_device, grps: *const *const attribute_group) -> ::core::ffi::c_int;
}
#[cfg(not(feature = "CONFIG_RTC_INTF_SYSFS"))]
#[inline] pub unsafe fn rtc_add_group(_: *mut rtc_device, _: *const attribute_group) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_RTC_INTF_SYSFS"))]
#[inline] pub unsafe fn rtc_add_groups(_: *mut rtc_device, _: *const *const attribute_group) -> ::core::ffi::c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
