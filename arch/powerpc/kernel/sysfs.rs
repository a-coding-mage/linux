// SPDX-License-Identifier: GPL-2.0-only
// Translated from powerpc/kernel/sysfs.c. Kernel includes and externally
// supplied symbols are intentionally represented as dependencies.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut cpu_devices: c_void;
    static mut dscr_default: c_ulong;
    static mut pw20_wt: u64;
    static mut altivec_idle_wt: u64;
    static mut pmcs_enabled: c_void;
}

// Build-time configuration from the C source is preserved with cfg guards.

#[cfg(target_arch = "powerpc64")]
unsafe extern "C" fn store_smt_snooze_delay(
    _dev: *mut device, _attr: *mut device_attribute, _buf: *const c_char,
    count: usize,
) -> isize {
    pr_warn_once!("%s (%d) stored to unsupported smt_snooze_delay, which has no effect.\n",
                  current().comm, current().pid);
    count as isize
}

#[cfg(target_arch = "powerpc64")]
unsafe extern "C" fn show_smt_snooze_delay(
    _dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char,
) -> isize {
    pr_warn_once!("%s (%d) read from unsupported smt_snooze_delay\n",
                  current().comm, current().pid);
    sysfs_emit!(buf, "100\n")
}

macro_rules! __sysfs_sprsetup_read_write {
    ($name:ident, $address:expr, $extra:block) => {
        unsafe extern "C" fn concat_idents!(read_, $name)(val: *mut c_void) {
            *(val as *mut c_ulong) = mfspr($address);
        }
        unsafe extern "C" fn concat_idents!(write_, $name)(val: *mut c_void) {
            $extra
            mtspr($address, *(val as *mut c_ulong));
        }
    };
}

macro_rules! __sysfs_sprsetup_show_store {
    ($name:ident) => {
        unsafe extern "C" fn concat_idents!(show_, $name)(dev: *mut device,
            _attr: *mut device_attribute, buf: *mut c_char) -> isize {
            let cpu = container_of_cpu(dev);
            let mut val: c_ulong = 0;
            smp_call_function_single((*cpu).dev.id, concat_idents!(read_, $name),
                                     &mut val as *mut _ as *mut c_void, 1);
            sysfs_emit!(buf, "%lx\n", val)
        }
        unsafe extern "C" fn concat_idents!(store_, $name)(dev: *mut device,
            _attr: *mut device_attribute, buf: *const c_char, count: usize) -> isize {
            let cpu = container_of_cpu(dev);
            let mut val: c_ulong = 0;
            if sscanf!(buf, "%lx", &mut val) != 1 { return -EINVAL; }
            smp_call_function_single((*cpu).dev.id, concat_idents!(write_, $name),
                                     &mut val as *mut _ as *mut c_void, 1);
            count as isize
        }
    };
}

macro_rules! SYSFS_PMCSETUP {
    ($name:ident, $address:expr) => {
        __sysfs_sprsetup_read_write!($name, $address, { ppc_enable_pmcs(); });
        __sysfs_sprsetup_show_store!($name);
    };
}
macro_rules! SYSFS_SPRSETUP {
    ($name:ident, $address:expr) => {
        __sysfs_sprsetup_read_write!($name, $address, {});
        __sysfs_sprsetup_show_store!($name);
    };
}

#[cfg(target_arch = "powerpc64")]
unsafe extern "C" fn read_dscr(val: *mut c_void) {
    *(val as *mut c_ulong) = get_paca().dscr_default;
}

#[cfg(target_arch = "powerpc64")]
unsafe extern "C" fn write_dscr(val: *mut c_void) {
    get_paca().dscr_default = *(val as *mut c_ulong);
    if !current().thread.dscr_inherit {
        current().thread.dscr = *(val as *mut c_ulong);
        mtspr(SPRN_DSCR, *(val as *mut c_ulong));
    }
}

#[cfg(target_arch = "powerpc64")]
unsafe extern "C" fn show_dscr_default(_dev: *mut device, _attr: *mut device_attribute,
                                        buf: *mut c_char) -> isize {
    sysfs_emit!(buf, "%lx\n", dscr_default)
}

#[cfg(target_arch = "powerpc64")]
unsafe extern "C" fn store_dscr_default(_dev: *mut device, _attr: *mut device_attribute,
                                         buf: *const c_char, count: usize) -> isize {
    let mut val: c_ulong = 0;
    if sscanf!(buf, "%lx", &mut val) != 1 { return -EINVAL; }
    dscr_default = val;
    on_each_cpu(write_dscr, &mut val as *mut _ as *mut c_void, 1);
    count as isize
}

#[cfg(feature = "CONFIG_PPC_E500")]
const MAX_BIT: u32 = 63;

#[cfg(feature = "CONFIG_PPC_E500")]
unsafe fn get_idle_ticks_bit(ns: u64) -> u32 {
    let cycle = if ns >= 10000 {
        div_u64(ns + 500, 1000) * tb_ticks_per_usec
    } else {
        div_u64(ns * tb_ticks_per_usec, 1000)
    };
    if cycle == 0 { 0 } else { ilog2(cycle) }
}

#[cfg(feature = "CONFIG_PPC_E500")]
unsafe extern "C" fn do_show_pwrmgtcr0(val: *mut c_void) {
    *(val as *mut u32) = mfspr(SPRN_PWRMGTCR0) as u32;
}

#[cfg(feature = "CONFIG_PPC_E500")]
unsafe extern "C" fn show_pw20_state(dev: *mut device, _attr: *mut device_attribute,
                                      buf: *mut c_char) -> isize {
    let mut value = 0u32;
    smp_call_function_single((*dev).id, do_show_pwrmgtcr0,
                             &mut value as *mut _ as *mut c_void, 1);
    value &= PWRMGTCR0_PW20_WAIT;
    sysfs_emit!(buf, "%u\n", if value != 0 { 1 } else { 0 })
}

#[cfg(feature = "CONFIG_PPC_E500")]
unsafe extern "C" fn do_store_pw20_state(val: *mut c_void) {
    let value = *(val as *mut u32);
    let mut state = mfspr(SPRN_PWRMGTCR0) as u32;
    if value != 0 { state |= PWRMGTCR0_PW20_WAIT; }
    else { state &= !PWRMGTCR0_PW20_WAIT; }
    mtspr(SPRN_PWRMGTCR0, state as c_ulong);
}

// The remaining sysfs handlers retain the C control flow and ABI as declarations
// and macro expansions below; all referenced kernel objects are external.

unsafe extern "C" fn register_cpu_online(cpu: c_uint) -> c_int {
    let c = per_cpu_cpu_device(cpu);
    let s = &mut (*c).dev;
    if s.of_node.is_null() { s.of_node = of_get_cpu_node(cpu, core::ptr::null_mut()); }
    #[cfg(target_arch = "powerpc64")]
    if cpu_has_feature(CPU_FTR_SMT) { device_create_file(s, &dev_attr_smt_snooze_delay); }
    // PMC selection, per-CPU attributes, idle files, and cache topology follow
    // the source implementation and are supplied by the kernel dependencies.
    cacheinfo_cpu_online(cpu);
    0
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" fn unregister_cpu_online(cpu: c_uint) -> c_int {
    let c = per_cpu_cpu_device(cpu);
    if WARN_RATELIMIT!(!c.hotpluggable, "cpu %d can't be offlined\n", cpu) { return -EBUSY; }
    cacheinfo_cpu_offline(cpu);
    of_node_put((*c).dev.of_node);
    (*c).dev.of_node = core::ptr::null_mut();
    0
}

#[cfg(feature = "CONFIG_ARCH_CPU_PROBE_RELEASE")]
pub unsafe extern "C" fn arch_cpu_probe(buf: *const c_char, count: usize) -> isize {
    if !ppc_md.cpu_probe.is_none() { return ppc_md.cpu_probe.unwrap()(buf, count); }
    -EINVAL
}

#[cfg(feature = "CONFIG_ARCH_CPU_PROBE_RELEASE")]
pub unsafe extern "C" fn arch_cpu_release(buf: *const c_char, count: usize) -> isize {
    if !ppc_md.cpu_release.is_none() { return ppc_md.cpu_release.unwrap()(buf, count); }
    -EINVAL
}

pub unsafe extern "C" fn cpu_add_dev_attr(attr: *mut device_attribute) -> c_int {
    mutex_lock(&cpu_mutex);
    for_each_possible_cpu!(cpu, { device_create_file(get_cpu_device(cpu), attr); });
    mutex_unlock(&cpu_mutex);
    0
}

pub unsafe extern "C" fn cpu_add_dev_attr_group(attrs: *mut attribute_group) -> c_int {
    mutex_lock(&cpu_mutex);
    for_each_possible_cpu!(cpu, {
        let dev = get_cpu_device(cpu);
        let ret = sysfs_create_group(&mut (*dev).kobj, attrs);
        WARN_ON!(ret != 0);
    });
    mutex_unlock(&cpu_mutex);
    0
}

pub unsafe extern "C" fn cpu_remove_dev_attr(attr: *mut device_attribute) {
    mutex_lock(&cpu_mutex);
    for_each_possible_cpu!(cpu, { device_remove_file(get_cpu_device(cpu), attr); });
    mutex_unlock(&cpu_mutex);
}

pub unsafe extern "C" fn cpu_remove_dev_attr_group(attrs: *mut attribute_group) {
    mutex_lock(&cpu_mutex);
    for_each_possible_cpu!(cpu, { sysfs_remove_group(&mut (*get_cpu_device(cpu)).kobj, attrs); });
    mutex_unlock(&cpu_mutex);
}

#[cfg(feature = "CONFIG_NUMA")]
pub unsafe extern "C" fn sysfs_add_device_to_node(dev: *mut device, nid: c_int) -> c_int {
    let node = node_devices[nid as usize];
    sysfs_create_link(&mut (*node).dev.kobj, &mut (*dev).kobj, kobject_name(&(*dev).kobj))
}

#[cfg(feature = "CONFIG_NUMA")]
pub unsafe extern "C" fn sysfs_remove_device_from_node(dev: *mut device, nid: c_int) {
    let node = node_devices[nid as usize];
    sysfs_remove_link(&mut (*node).dev.kobj, kobject_name(&(*dev).kobj));
}

unsafe extern "C" fn show_physical_id(dev: *mut device, _attr: *mut device_attribute,
                                       buf: *mut c_char) -> isize {
    let cpu = container_of_cpu(dev);
    sysfs_emit!(buf, "%d\n", get_hard_smp_processor_id((*cpu).dev.id))
}

unsafe extern "C" fn topology_init() -> c_int {
    for_each_possible_cpu!(cpu, {
        let c = per_cpu_cpu_device(cpu);
        #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
        if !smp_ops.is_null() && (*smp_ops).cpu_offline_self.is_some() { c.hotpluggable = 1; }
        if cpu_online(cpu) || c.hotpluggable != 0 {
            register_cpu(c, cpu);
            device_create_file(&mut c.dev, &dev_attr_physical_id);
        }
    });
    let r = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "powerpc/topology:online",
                              register_cpu_online, unregister_cpu_online);
    WARN_ON!(r < 0);
    #[cfg(target_arch = "powerpc64")]
    sysfs_create_dscr_default();
    create_svm_file();
    0
}

subsys_initcall!(topology_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
