// SPDX-License-Identifier: GPL-2.0
/* CPU subsystem support. Direct Rust translation; Linux dependencies are external. */

// C includes and preprocessor-provided Linux definitions are supplied by other files.

static mut CPU_SYS_DEVICES: PerCpu<*mut device> = DEFINE_PER_CPU!();

unsafe fn cpu_subsys_match(dev: *mut device, drv: *const device_driver) -> c_int {
    /* ACPI style match is the only one that may succeed. */
    if acpi_driver_match_device(dev, drv) != 0 { 1 } else { 0 }
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn change_cpu_under_node(cpu: *mut cpu, from_nid: c_uint, to_nid: c_uint) {
    let cpuid = (*cpu).dev.id;
    unregister_cpu_under_node(cpuid, from_nid);
    register_cpu_under_node(cpuid, to_nid);
    (*cpu).node_id = to_nid;
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn cpu_subsys_online(dev: *mut device) -> c_int {
    let cpu = container_of!(dev, cpu, dev);
    let cpuid = (*dev).id;
    let from_nid = cpu_to_node(cpuid);
    if from_nid == NUMA_NO_NODE { return -ENODEV; }
    let mut retries = 0;
    loop {
        let ret = cpu_device_up(dev);
        /* Retry transient hotplug disablement with an exponentially increasing delay. */
        if ret == -EBUSY {
            retries += 1;
            if retries > 5 { return ret; }
            msleep(10 * (1 << retries));
            continue;
        }
        let to_nid = cpu_to_node(cpuid);
        if from_nid != to_nid { change_cpu_under_node(cpu, from_nid, to_nid); }
        return ret;
    }
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn cpu_subsys_offline(dev: *mut device) -> c_int { cpu_device_down(dev) }

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe fn unregister_cpu(cpu: *mut cpu) {
    let logical_cpu = (*cpu).dev.id;
    set_cpu_enabled(logical_cpu, false);
    unregister_cpu_under_node(logical_cpu, cpu_to_node(logical_cpu));
    device_unregister(&mut (*cpu).dev);
    per_cpu!(&mut CPU_SYS_DEVICES, logical_cpu) = core::ptr::null_mut();
}

#[cfg(all(CONFIG_HOTPLUG_CPU, CONFIG_ARCH_CPU_PROBE_RELEASE))]
unsafe fn cpu_probe_store(dev: *mut device, attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let ret = lock_device_hotplug_sysfs();
    if ret != 0 { return ret as ssize_t; }
    let cnt = arch_cpu_probe(buf, count);
    unlock_device_hotplug();
    cnt
}

#[cfg(all(CONFIG_HOTPLUG_CPU, CONFIG_ARCH_CPU_PROBE_RELEASE))]
unsafe fn cpu_release_store(dev: *mut device, attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let ret = lock_device_hotplug_sysfs();
    if ret != 0 { return ret as ssize_t; }
    let cnt = arch_cpu_release(buf, count);
    unlock_device_hotplug();
    cnt
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn crash_notes_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let cpu = container_of!(dev, cpu, dev);
    let cpunum = (*cpu).dev.id;
    let addr = per_cpu_ptr_to_phys(per_cpu_ptr(crash_notes, cpunum));
    sysfs_emit(buf, "%llx\n", addr)
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn crash_notes_size_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, "%zu\n", core::mem::size_of::<note_buf_t>())
}

static COMMON_CPU_ATTR_GROUPS: [*const attribute_group; 1] = [core::ptr::null()];
static HOTPLUGABLE_CPU_ATTR_GROUPS: [*const attribute_group; 1] = [core::ptr::null()];

/* Print cpu online, possible, present, and system maps. */
#[repr(C)]
struct cpu_attr { attr: device_attribute, map: *const cpumask }

unsafe fn show_cpus_attr(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let ca = container_of!(attr, cpu_attr, attr);
    sysfs_emit(buf, "%*pbl\n", cpumask_pr_args((*ca).map))
}

static mut CPU_ATTRS: [cpu_attr; 3] = [
    CPU_ATTR!(online, &__cpu_online_mask, show_cpus_attr),
    CPU_ATTR!(possible, &__cpu_possible_mask, show_cpus_attr),
    CPU_ATTR!(present, &__cpu_present_mask, show_cpus_attr),
];

unsafe fn print_cpus_kernel_max(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, "%d\n", NR_CPUS - 1)
}

pub static mut total_cpus: c_uint = 0;

unsafe fn print_cpus_offline(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut len = 0;
    let offline = alloc_cpumask_var(GFP_KERNEL);
    if offline.is_null() { return -ENOMEM as ssize_t; }
    cpumask_andnot(offline, cpu_possible_mask, cpu_online_mask);
    len += sysfs_emit_at(buf, len, "%*pbl", cpumask_pr_args(offline));
    free_cpumask_var(offline);
    if total_cpus != 0 && nr_cpu_ids < total_cpus {
        len += sysfs_emit_at(buf, len, ",");
        if nr_cpu_ids == total_cpus - 1 { len += sysfs_emit_at(buf, len, "%u", nr_cpu_ids); }
        else { len += sysfs_emit_at(buf, len, "%u-%d", nr_cpu_ids, total_cpus - 1); }
    }
    len += sysfs_emit_at(buf, len, "\n");
    len
}

unsafe fn print_cpus_enabled(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(cpu_enabled_mask))
}

unsafe fn print_cpus_isolated(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let isolated = alloc_cpumask_var(GFP_KERNEL);
    if isolated.is_null() { return -ENOMEM as ssize_t; }
    cpumask_andnot(isolated, cpu_possible_mask, housekeeping_cpumask(HK_TYPE_DOMAIN_BOOT));
    let len = sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(isolated));
    free_cpumask_var(isolated);
    len
}

unsafe fn housekeeping_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let hk_mask = housekeeping_cpumask(HK_TYPE_KERNEL_NOISE);
    if housekeeping_enabled(HK_TYPE_KERNEL_NOISE) != 0 { sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(hk_mask)) }
    else { sysfs_emit(buf, "\n") }
}

#[cfg(CONFIG_NO_HZ_FULL)]
unsafe fn nohz_full_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    if cpumask_available(tick_nohz_full_mask) != 0 { sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(tick_nohz_full_mask)) }
    else { sysfs_emit(buf, "\n") }
}

#[cfg(CONFIG_CRASH_HOTPLUG)]
unsafe fn crash_hotplug_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, "%d\n", crash_check_hotplug_support())
}

unsafe fn cpu_device_release(dev: *mut device) { /* statically allocated CPU devices */ }

#[cfg(CONFIG_GENERIC_CPU_AUTOPROBE)]
unsafe fn print_cpu_modalias(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let mut len = sysfs_emit_at(buf, 0, "cpu:type:" CPU_FEATURE_TYPEFMT ":feature:", CPU_FEATURE_TYPEVAL);
    for i in 0..MAX_CPU_FEATURES {
        if cpu_have_feature(i) != 0 {
            if len + core::mem::size_of::<&str>() as isize >= PAGE_SIZE { WARN!(1, "CPU features overflow page\n"); break; }
            len += sysfs_emit_at(buf, len, ",%04X", i);
        }
    }
    len + sysfs_emit_at(buf, len, "\n")
}

#[cfg(CONFIG_GENERIC_CPU_AUTOPROBE)]
unsafe fn cpu_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    let buf = kzalloc(PAGE_SIZE, GFP_KERNEL);
    if !buf.is_null() { print_cpu_modalias(core::ptr::null_mut(), core::ptr::null_mut(), buf); add_uevent_var(env, "MODALIAS=%s", buf); kfree(buf); }
    0
}

pub static cpu_subsys: bus_type = bus_type {
    name: "cpu", dev_name: "cpu", match_: Some(cpu_subsys_match),
    #[cfg(CONFIG_HOTPLUG_CPU)] online: Some(cpu_subsys_online),
    #[cfg(CONFIG_HOTPLUG_CPU)] offline: Some(cpu_subsys_offline),
    #[cfg(CONFIG_GENERIC_CPU_AUTOPROBE)] uevent: Some(cpu_uevent),
};

pub unsafe fn register_cpu(cpu: *mut cpu, num: c_int) -> c_int {
    (*cpu).node_id = cpu_to_node(num);
    core::ptr::write_bytes(&mut (*cpu).dev, 0, 1);
    (*cpu).dev.id = num; (*cpu).dev.bus = &cpu_subsys; (*cpu).dev.release = Some(cpu_device_release);
    dev_assign_offline_disabled(&mut (*cpu).dev, !(*cpu).hotpluggable);
    dev_assign_offline(&mut (*cpu).dev, !cpu_online(num));
    (*cpu).dev.of_node = of_get_cpu_node(num, core::ptr::null_mut());
    (*cpu).dev.groups = if (*cpu).hotpluggable { HOTPLUGABLE_CPU_ATTR_GROUPS.as_ptr() } else { COMMON_CPU_ATTR_GROUPS.as_ptr() };
    let error = device_register(&mut (*cpu).dev);
    if error != 0 { put_device(&mut (*cpu).dev); return error; }
    per_cpu!(&mut CPU_SYS_DEVICES, num) = &mut (*cpu).dev;
    register_cpu_under_node(num, cpu_to_node(num));
    dev_pm_qos_expose_latency_limit(&mut (*cpu).dev, PM_QOS_RESUME_LATENCY_NO_CONSTRAINT);
    set_cpu_enabled(num, true); 0
}

pub unsafe fn get_cpu_device(cpu: c_uint) -> *mut device {
    if cpu < nr_cpu_ids && cpu_possible(cpu) != 0 { per_cpu!(&CPU_SYS_DEVICES, cpu) } else { core::ptr::null_mut() }
}

unsafe fn device_create_release(dev: *mut device) { kfree(dev); }

unsafe fn __cpu_device_create(parent: *mut device, drvdata: *mut c_void, groups: *const *const attribute_group, fmt: *const c_char, args: va_list) -> *mut device {
    let dev = kzalloc_obj::<device>();
    if dev.is_null() { return ERR_PTR(-ENOMEM); }
    device_initialize(dev); (*dev).parent = parent; (*dev).groups = groups; (*dev).release = Some(device_create_release); device_set_pm_not_required(dev); dev_set_drvdata(dev, drvdata);
    let retval = kobject_set_name_vargs(&mut (*dev).kobj, fmt, args);
    if retval != 0 { put_device(dev); return ERR_PTR(retval); }
    let retval = device_add(dev);
    if retval != 0 { put_device(dev); return ERR_PTR(retval); }
    dev
}

pub unsafe fn cpu_device_create(parent: *mut device, drvdata: *mut c_void, groups: *const *const attribute_group, fmt: *const c_char, mut args: ...) -> *mut device {
    __cpu_device_create(parent, drvdata, groups, fmt, args)
}

pub unsafe fn cpu_is_hotpluggable(cpu: c_uint) -> bool {
    let dev = get_cpu_device(cpu);
    !dev.is_null() && (*container_of!(dev, cpu, dev)).hotpluggable && tick_nohz_cpu_hotpluggable(cpu)
}

#[cfg(CONFIG_GENERIC_CPU_DEVICES)]
pub static mut cpu_devices: PerCpu<cpu> = DEFINE_PER_CPU!();
#[cfg(CONFIG_GENERIC_CPU_DEVICES)]
pub unsafe fn arch_cpu_is_hotpluggable(cpu: c_int) -> bool { false }
#[cfg(CONFIG_GENERIC_CPU_DEVICES)]
pub unsafe fn arch_register_cpu(cpu: c_int) -> c_int { let c = per_cpu!(&mut cpu_devices, cpu); (*c).hotpluggable = arch_cpu_is_hotpluggable(cpu); register_cpu(c, cpu) }
#[cfg(all(CONFIG_GENERIC_CPU_DEVICES, CONFIG_HOTPLUG_CPU))]
pub unsafe fn arch_unregister_cpu(num: c_int) { unregister_cpu(per_cpu!(&mut cpu_devices, num)); }

unsafe fn cpu_dev_register_generic() {
    if !IS_ENABLED!(CONFIG_GENERIC_CPU_DEVICES) { return; }
    for_each_present_cpu!(i, { let ret = arch_register_cpu(i); if ret != 0 && ret != -EPROBE_DEFER { pr_warn!("register_cpu {} failed ({})\n", i, ret); } });
}

#[cfg(CONFIG_GENERIC_CPU_VULNERABILITIES)]
unsafe fn cpu_show_not_affected(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { sysfs_emit(buf, "Not affected\n") }

#[cfg(CONFIG_GENERIC_CPU_VULNERABILITIES)]
unsafe fn cpu_register_vulnerabilities() {
    let dev = bus_get_dev_root(&cpu_subsys);
    if !dev.is_null() { if sysfs_create_group(&mut (*dev).kobj, &cpu_root_vulnerabilities_group) != 0 { pr_err!("Unable to register CPU vulnerabilities\n"); } put_device(dev); }
}
#[cfg(not(CONFIG_GENERIC_CPU_VULNERABILITIES))]
unsafe fn cpu_register_vulnerabilities() {}

pub unsafe fn cpu_dev_init() {
    if subsys_system_register(&cpu_subsys, cpu_root_attr_groups.as_ptr()) != 0 { panic!("Failed to register CPU subsystem"); }
    cpu_dev_register_generic(); cpu_register_vulnerabilities();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
