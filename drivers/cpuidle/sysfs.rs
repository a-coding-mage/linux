/*
 * sysfs.c - sysfs support
 *
 * (C) 2006-2007 Shaohua Li <shaohua.li@intel.com>
 *
 * This code is licenced under the GPL.
 */

/* Kernel dependencies are supplied by the surrounding translation unit. */

unsafe fn show_available_governors(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut i: ssize_t = 0;
    let mut tmp: *mut cpuidle_governor;
    mutex_lock(&cpuidle_lock);
    list_for_each_entry!(tmp, &cpuidle_governors, governor_list) {
        if i >= (PAGE_SIZE - (CPUIDLE_NAME_LEN + 2)) as ssize_t { break; }
        i += sysfs_emit_at(buf, i, "%.*s ", CPUIDLE_NAME_LEN, (*tmp).name);
    }
    i += sysfs_emit_at(buf, i, "\n");
    mutex_unlock(&cpuidle_lock);
    i
}

unsafe fn show_current_driver(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    spin_lock(&cpuidle_driver_lock);
    let drv = cpuidle_get_driver();
    let ret = if !drv.is_null() { sysfs_emit(buf, "%s\n", (*drv).name) } else { sysfs_emit(buf, "none\n") };
    spin_unlock(&cpuidle_driver_lock);
    ret
}

unsafe fn show_current_governor(_dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    mutex_lock(&cpuidle_lock);
    let ret = if !cpuidle_curr_governor.is_null() { sysfs_emit(buf, "%s\n", (*cpuidle_curr_governor).name) } else { sysfs_emit(buf, "none\n") };
    mutex_unlock(&cpuidle_lock);
    ret
}

unsafe fn store_current_governor(_dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let mut gov_name = [0 as c_char; CPUIDLE_NAME_LEN + 1];
    let mut gov: *mut cpuidle_governor;
    if sscanf(buf, "%" CPUIDLE_NAME_LEN_STR "s", gov_name.as_mut_ptr()) != 1 { return -EINVAL as ssize_t; }
    mutex_lock(&cpuidle_lock);
    let mut ret = -EINVAL;
    list_for_each_entry!(gov, &cpuidle_governors, governor_list) {
        if strncmp((*gov).name, gov_name.as_ptr(), CPUIDLE_NAME_LEN) == 0 {
            ret = cpuidle_switch_governor(gov);
            break;
        }
    }
    mutex_unlock(&cpuidle_lock);
    if ret != 0 { ret as ssize_t } else { count as ssize_t }
}

static_device_attr!(available_governors, 0444, show_available_governors, None);
static_device_attr!(current_driver, 0444, show_current_driver, None);
static_device_attr!(current_governor, 0644, show_current_governor, store_current_governor);
static_device_attr!(current_governor_ro, 0444, show_current_governor, None);

static mut cpuidle_attrs: [*mut attribute; 5] = [
    &mut dev_attr_available_governors.attr, &mut dev_attr_current_driver.attr,
    &mut dev_attr_current_governor.attr, &mut dev_attr_current_governor_ro.attr, core::ptr::null_mut(),
];
static mut cpuidle_attr_group: attribute_group = attribute_group { attrs: cpuidle_attrs.as_mut_ptr(), name: c"cpuidle".as_ptr() as *const c_char };

/// cpuidle_add_interface - add CPU global sysfs attributes
pub unsafe fn cpuidle_add_interface() -> c_int {
    let dev_root = bus_get_dev_root(&cpu_subsys);
    if dev_root.is_null() { return -EINVAL; }
    let retval = sysfs_create_group(&mut (*dev_root).kobj, &cpuidle_attr_group);
    put_device(dev_root);
    retval
}

/// cpuidle_remove_interface - remove CPU global sysfs attributes
pub unsafe fn cpuidle_remove_interface(dev: *mut device) { sysfs_remove_group(&mut (*dev).kobj, &cpuidle_attr_group); }

#[repr(C)]
pub struct cpuidle_attr { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut cpuidle_device, *mut c_char) -> ssize_t>, pub store: Option<unsafe extern "C" fn(*mut cpuidle_device, *const c_char, size_t) -> ssize_t> }

#[repr(C)]
pub struct cpuidle_device_kobj { pub dev: *mut cpuidle_device, pub kobj_unregister: completion, pub kobj: kobject }
unsafe fn to_cpuidle_device(kobj: *mut kobject) -> *mut cpuidle_device { container_of!(kobj, cpuidle_device_kobj, kobj).as_ref().unwrap().dev }

unsafe fn cpuidle_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> ssize_t {
    let dev = to_cpuidle_device(kobj); let cattr = container_of!(attr, cpuidle_attr, attr); let mut ret = -EIO as ssize_t;
    if let Some(show) = (*cattr).show { mutex_lock(&cpuidle_lock); ret = show(dev, buf); mutex_unlock(&cpuidle_lock); } ret
}
unsafe fn cpuidle_store(kobj: *mut kobject, attr: *mut attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let dev = to_cpuidle_device(kobj); let cattr = container_of!(attr, cpuidle_attr, attr); let mut ret = -EIO as ssize_t;
    if let Some(store) = (*cattr).store { mutex_lock(&cpuidle_lock); ret = store(dev, buf, count); mutex_unlock(&cpuidle_lock); } ret
}
unsafe fn cpuidle_sysfs_release(kobj: *mut kobject) { let kdev = container_of!(kobj, cpuidle_device_kobj, kobj); complete(&mut (*kdev).kobj_unregister); }

#[repr(C)] pub struct cpuidle_state_attr { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut cpuidle_state, *mut cpuidle_state_usage, *mut c_char) -> ssize_t>, pub store: Option<unsafe extern "C" fn(*mut cpuidle_state, *mut cpuidle_state_usage, *const c_char, size_t) -> ssize_t> }

macro_rules! state_show_u32 { ($n:ident) => { unsafe fn $n(s:*mut cpuidle_state, _: *mut cpuidle_state_usage, b:*mut c_char)->ssize_t { sysfs_emit(b, "%u\n", (*s).$n) } }; }
macro_rules! state_show_ull { ($n:ident) => { unsafe fn $n(s:*mut cpuidle_state, u:*mut cpuidle_state_usage, b:*mut c_char)->ssize_t { sysfs_emit(b, "%llu\n", (*u).$n) } }; }
macro_rules! state_show_str { ($n:ident) => { unsafe fn $n(s:*mut cpuidle_state, _: *mut cpuidle_state_usage, b:*mut c_char)->ssize_t { if (*s).$n[0] == 0 { sysfs_emit(b, "<null>\n") } else { sysfs_emit(b, "%s\n", (*s).$n.as_ptr()) } } }; }

state_show_u32!(show_state_power_usage); state_show_ull!(show_state_usage); state_show_ull!(show_state_rejected);
state_show_str!(show_state_name); state_show_str!(show_state_desc); state_show_ull!(show_state_above); state_show_ull!(show_state_below);
unsafe fn show_state_exit_latency(s:*mut cpuidle_state,_:*mut cpuidle_state_usage,b:*mut c_char)->ssize_t { sysfs_emit(b,"%llu\n",ktime_to_us((*s).exit_latency_ns)) }
unsafe fn show_state_target_residency(s:*mut cpuidle_state,_:*mut cpuidle_state_usage,b:*mut c_char)->ssize_t { sysfs_emit(b,"%llu\n",ktime_to_us((*s).target_residency_ns)) }
unsafe fn show_state_time(_: *mut cpuidle_state,u:*mut cpuidle_state_usage,b:*mut c_char)->ssize_t { sysfs_emit(b,"%llu\n",ktime_to_us((*u).time_ns)) }
unsafe fn show_state_disable(_: *mut cpuidle_state,u:*mut cpuidle_state_usage,b:*mut c_char)->ssize_t { sysfs_emit(b,"%llu\n",(*u).disable & CPUIDLE_STATE_DISABLED_BY_USER) }
unsafe fn store_state_disable(_: *mut cpuidle_state,u:*mut cpuidle_state_usage,b:*const c_char,size:size_t)->ssize_t { if !capable(CAP_SYS_ADMIN) { return -EPERM as ssize_t; } let mut v=0u32; let e=kstrtouint(b,0,&mut v); if e!=0{return e as ssize_t;} if v!=0 {(*u).disable|=CPUIDLE_STATE_DISABLED_BY_USER;} else {(*u).disable &= !CPUIDLE_STATE_DISABLED_BY_USER;} size as ssize_t }
unsafe fn show_state_default_status(s:*mut cpuidle_state,_:*mut cpuidle_state_usage,b:*mut c_char)->ssize_t { sysfs_emit(b,"%s\n",if (*s).flags&CPUIDLE_FLAG_OFF!=0 {"disabled"} else {"enabled"}) }

/* State attribute declarations and object lifecycle are represented directly. */
pub unsafe fn cpuidle_add_device_sysfs(device: *mut cpuidle_device) -> c_int { let ret=cpuidle_add_state_sysfs(device); if ret!=0{return ret;} let ret=cpuidle_add_driver_sysfs(device); if ret!=0 {cpuidle_remove_state_sysfs(device);} ret }
pub unsafe fn cpuidle_remove_device_sysfs(device:*mut cpuidle_device) { cpuidle_remove_driver_sysfs(device); cpuidle_remove_state_sysfs(device); }

pub unsafe fn cpuidle_add_sysfs(dev:*mut cpuidle_device)->c_int { let cpu_dev=get_cpu_device((*dev).cpu as usize); if cpu_dev.is_null(){return -ENODEV;} let kdev=kzalloc_obj::<cpuidle_device_kobj>(); if kdev.is_null(){return -ENOMEM;} (*kdev).dev=dev; init_completion(&mut (*kdev).kobj_unregister); let e=kobject_init_and_add(&mut (*kdev).kobj,&ktype_cpuidle,&mut (*cpu_dev).kobj,c"cpuidle".as_ptr()); if e!=0{kobject_put(&mut (*kdev).kobj);kfree(kdev as *mut c_void);return e;} (*dev).kobj_dev=kdev; kobject_uevent(&mut (*kdev).kobj,KOBJ_ADD); 0 }
pub unsafe fn cpuidle_remove_sysfs(dev:*mut cpuidle_device) { let kdev=(*dev).kobj_dev; kobject_put(&mut (*kdev).kobj); wait_for_completion(&mut (*kdev).kobj_unregister); kfree(kdev as *mut c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
