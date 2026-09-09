// SPDX-License-Identifier: GPL-2.0
/* sysfs entries for device PM */
// Kernel headers and symbols are supplied by the surrounding translation unit.

pub static power_group_name: &[u8] = b"power\0";
static ctrl_auto: &[u8] = b"auto\0";
static ctrl_on: &[u8] = b"on\0";

unsafe fn control_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t {
    sysfs_emit(buf, b"%s\n\0".as_ptr() as *const i8,
        if (*dev).power.runtime_auto { ctrl_auto.as_ptr() } else { ctrl_on.as_ptr() })
}
unsafe fn control_store(dev: *mut device, _attr: *mut device_attribute, buf: *const i8, mut n: usize) -> ssize_t {
    device_lock(dev);
    if sysfs_streq(buf, ctrl_auto.as_ptr() as *const i8) { pm_runtime_allow(dev); }
    else if sysfs_streq(buf, ctrl_on.as_ptr() as *const i8) { pm_runtime_forbid(dev); }
    else { n = (-EINVAL) as usize; }
    device_unlock(dev); n as ssize_t
}

unsafe fn runtime_active_time_show(dev: *mut device, _a: *mut device_attribute, buf: *mut i8) -> ssize_t {
    let mut tmp = pm_runtime_active_time(dev); tmp /= NSEC_PER_MSEC; sysfs_emit(buf, b"%llu\n\0".as_ptr() as *const i8, tmp)
}
unsafe fn runtime_suspended_time_show(dev: *mut device, _a: *mut device_attribute, buf: *mut i8) -> ssize_t {
    let mut tmp = pm_runtime_suspended_time(dev); tmp /= NSEC_PER_MSEC; sysfs_emit(buf, b"%llu\n\0".as_ptr() as *const i8, tmp)
}
unsafe fn runtime_status_show(dev: *mut device, _a: *mut device_attribute, buf: *mut i8) -> ssize_t {
    let output = if (*dev).power.runtime_error { b"error\0".as_ptr() } else if (*dev).power.disable_depth != 0 { b"unsupported\0".as_ptr() } else { match (*dev).power.runtime_status { RPM_SUSPENDED=>b"suspended\0".as_ptr(), RPM_SUSPENDING=>b"suspending\0".as_ptr(), RPM_RESUMING=>b"resuming\0".as_ptr(), RPM_ACTIVE=>b"active\0".as_ptr(), _=>return -EIO } }; sysfs_emit(buf,b"%s\n\0".as_ptr() as *const i8,output)
}
unsafe fn autosuspend_delay_ms_show(dev:*mut device,_a:*mut device_attribute,buf:*mut i8)->ssize_t { if !(*dev).power.use_autosuspend{return -EIO} sysfs_emit(buf,b"%d\n\0".as_ptr() as *const i8,(*dev).power.autosuspend_delay) }
unsafe fn autosuspend_delay_ms_store(dev:*mut device,_a:*mut device_attribute,buf:*const i8,n:usize)->ssize_t { let mut delay: c_long=0; if !(*dev).power.use_autosuspend{return -EIO} if kstrtol(buf,10,&mut delay)!=0 || delay != delay as i32 as c_long{return -EINVAL} device_lock(dev); pm_runtime_set_autosuspend_delay(dev,delay); device_unlock(dev); n as ssize_t }
unsafe fn pm_qos_resume_latency_us_show(dev:*mut device,_a:*mut device_attribute,buf:*mut i8)->ssize_t { let mut v=dev_pm_qos_requested_resume_latency(dev); if v==0{return sysfs_emit(buf,b"n/a\n\0".as_ptr() as *const i8)} if v==PM_QOS_RESUME_LATENCY_NO_CONSTRAINT{v=0} sysfs_emit(buf,b"%d\n\0".as_ptr() as *const i8,v) }
unsafe fn pm_qos_resume_latency_us_store(dev:*mut device,_a:*mut device_attribute,buf:*const i8,n:usize)->ssize_t { let mut v=0i32; if kstrtos32(buf,0,&mut v)==0 {if v<0||v==PM_QOS_RESUME_LATENCY_NO_CONSTRAINT{return -EINVAL} if v==0{v=PM_QOS_RESUME_LATENCY_NO_CONSTRAINT}} else if sysfs_streq(buf,b"n/a\0".as_ptr() as *const i8){v=0}else{return -EINVAL} let r=dev_pm_qos_update_request((*dev).power.qos.resume_latency_req,v);if r<0{r}else{n as ssize_t} }
unsafe fn pm_qos_latency_tolerance_us_show(dev:*mut device,_a:*mut device_attribute,buf:*mut i8)->ssize_t {let v=dev_pm_qos_get_user_latency_tolerance(dev);if v<0{return sysfs_emit(buf,b"auto\n\0".as_ptr() as *const i8)}if v==PM_QOS_LATENCY_ANY{return sysfs_emit(buf,b"any\n\0".as_ptr() as *const i8)}sysfs_emit(buf,b"%d\n\0".as_ptr() as *const i8,v)}
unsafe fn pm_qos_latency_tolerance_us_store(dev:*mut device,_a:*mut device_attribute,buf:*const i8,n:usize)->ssize_t {let mut v=0i32;if kstrtos32(buf,0,&mut v)==0{if v<0{return -EINVAL}}else if sysfs_streq(buf,b"auto\0".as_ptr() as *const i8){v=PM_QOS_LATENCY_TOLERANCE_NO_CONSTRAINT}else if sysfs_streq(buf,b"any\0".as_ptr() as *const i8){v=PM_QOS_LATENCY_ANY}else{return -EINVAL}let r=dev_pm_qos_update_user_latency_tolerance(dev,v);if r<0{r}else{n as ssize_t}}
unsafe fn pm_qos_no_power_off_show(dev:*mut device,_a:*mut device_attribute,buf:*mut i8)->ssize_t{sysfs_emit(buf,b"%d\n\0".as_ptr() as *const i8,if dev_pm_qos_requested_flags(dev)&PM_QOS_FLAG_NO_POWER_OFF!=0{1}else{0})}
unsafe fn pm_qos_no_power_off_store(dev:*mut device,_a:*mut device_attribute,buf:*const i8,n:usize)->ssize_t{let mut v=0;if kstrtoint(buf,0,&mut v)!=0||v<0||v>1{return -EINVAL}let r=dev_pm_qos_update_flags(dev,PM_QOS_FLAG_NO_POWER_OFF,v);if r<0{r}else{n as ssize_t}}

#[cfg(feature="CONFIG_PM_SLEEP")]
static _enabled:&[u8]=b"enabled\0"; #[cfg(feature="CONFIG_PM_SLEEP")] static _disabled:&[u8]=b"disabled\0";
#[cfg(feature="CONFIG_PM_SLEEP")]
unsafe fn wakeup_show(dev:*mut device,_a:*mut device_attribute,buf:*mut i8)->ssize_t{sysfs_emit(buf,b"%s\n\0".as_ptr() as *const i8,if device_can_wakeup(dev){if device_may_wakeup(dev){_enabled.as_ptr()}else{_disabled.as_ptr()}}else{b"\0".as_ptr()})}
#[cfg(feature="CONFIG_PM_SLEEP")]
unsafe fn wakeup_store(dev:*mut device,_a:*mut device_attribute,buf:*const i8,n:usize)->ssize_t{if !device_can_wakeup(dev){return -EINVAL}if sysfs_streq(buf,_enabled.as_ptr() as *const i8){device_set_wakeup_enable(dev,1)}else if sysfs_streq(buf,_disabled.as_ptr() as *const i8){device_set_wakeup_enable(dev,0)}else{return -EINVAL}n as ssize_t}
#[cfg(feature="CONFIG_PM_SLEEP")]
unsafe fn wakeup_count_show(dev:*mut device,_a:*mut device_attribute,buf:*mut i8)->ssize_t{let mut v=0u64;let mut ok=false;spin_lock_irq(&mut (*dev).power.lock);if !(*dev).power.wakeup.is_null(){v=(*(*dev).power.wakeup).wakeup_count;ok=true}spin_unlock_irq(&mut (*dev).power.lock);if ok{sysfs_emit(buf,b"%lu\n\0".as_ptr() as *const i8,v)}else{sysfs_emit(buf,b"\n\0".as_ptr() as *const i8)}}
#[cfg(feature="CONFIG_PM_SLEEP")]
unsafe fn wakeup_active_count_show(dev:*mut device,a:*mut device_attribute,b:*mut i8)->ssize_t{wakeup_count_show(dev,a,b)}
#[cfg(feature="CONFIG_PM_SLEEP")]
unsafe fn wakeup_abort_count_show(dev:*mut device,a:*mut device_attribute,b:*mut i8)->ssize_t{wakeup_count_show(dev,a,b)}
#[cfg(feature="CONFIG_PM_SLEEP")]
unsafe fn wakeup_expire_count_show(dev:*mut device,a:*mut device_attribute,b:*mut i8)->ssize_t{wakeup_count_show(dev,a,b)}
#[cfg(feature="CONFIG_PM_SLEEP")]
unsafe fn wakeup_active_show(dev:*mut device,_a:*mut device_attribute,buf:*mut i8)->ssize_t{let mut v=0u32;let mut ok=false;spin_lock_irq(&mut (*dev).power.lock);if !(*dev).power.wakeup.is_null(){v=(*(*dev).power.wakeup).active;ok=true}spin_unlock_irq(&mut (*dev).power.lock);if ok{sysfs_emit(buf,b"%u\n\0".as_ptr() as *const i8,v)}else{sysfs_emit(buf,b"\n\0".as_ptr() as *const i8)}}
#[cfg(feature="CONFIG_PM_SLEEP")]
unsafe fn dpm_sysfs_wakeup_change_owner(dev:*mut device,k:kuid_t,g:kgid_t)->c_int{if !(*dev).power.wakeup.is_null()&&!(*(*dev).power.wakeup).dev.is_null(){device_change_owner((*dev).power.wakeup.dev,k,g)}else{0}}
#[cfg(not(feature="CONFIG_PM_SLEEP"))] unsafe fn dpm_sysfs_wakeup_change_owner(_:*mut device,_:kuid_t,_:kgid_t)->c_int{0}

unsafe fn dpm_sysfs_add_impl(dev:*mut device)->c_int{if device_pm_not_required(dev){return 0}let mut r=sysfs_create_group(&mut (*dev).kobj,pm_attr_group());if r!=0{return r}if !pm_runtime_has_no_callbacks(dev){r=sysfs_merge_group(&mut (*dev).kobj,pm_runtime_attr_group());if r!=0{sysfs_remove_group(&mut (*dev).kobj,pm_attr_group());return r}}if device_can_wakeup(dev){r=sysfs_merge_group(&mut (*dev).kobj,pm_wakeup_attr_group());if r!=0{sysfs_unmerge_group(&mut (*dev).kobj,pm_runtime_attr_group());sysfs_remove_group(&mut (*dev).kobj,pm_attr_group());return r}}r=pm_wakeup_source_sysfs_add(dev);if r!=0{sysfs_unmerge_group(&mut (*dev).kobj,pm_wakeup_attr_group());sysfs_unmerge_group(&mut (*dev).kobj,pm_runtime_attr_group());sysfs_remove_group(&mut (*dev).kobj,pm_attr_group())}r}

// Attribute-group objects are supplied by the kernel binding layer.
extern "C" { fn pm_attr_group()->*mut attribute_group; fn pm_runtime_attr_group()->*mut attribute_group; fn pm_wakeup_attr_group()->*mut attribute_group; fn pm_runtime_has_no_callbacks(dev:*mut device)->bool; fn pm_wakeup_source_sysfs_add(dev:*mut device)->c_int; }

// The following declarations preserve the C attribute objects and exported entry points.
extern "C" {
    fn dpm_sysfs_add(dev:*mut device)->c_int; fn dpm_sysfs_change_owner(dev:*mut device,kuid:kuid_t,kgid:kgid_t)->c_int;
    fn wakeup_sysfs_add(dev:*mut device)->c_int; fn wakeup_sysfs_remove(dev:*mut device);
    fn pm_qos_sysfs_add_resume_latency(dev:*mut device)->c_int; fn pm_qos_sysfs_remove_resume_latency(dev:*mut device);
    fn pm_qos_sysfs_add_flags(dev:*mut device)->c_int; fn pm_qos_sysfs_remove_flags(dev:*mut device);
    fn pm_qos_sysfs_add_latency_tolerance(dev:*mut device)->c_int; fn pm_qos_sysfs_remove_latency_tolerance(dev:*mut device);
    fn rpm_sysfs_remove(dev:*mut device); fn dpm_sysfs_remove(dev:*mut device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
