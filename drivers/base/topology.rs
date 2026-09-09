// SPDX-License-Identifier: GPL-2.0+
/*
 * driver/base/topology.c - Populate sysfs with cpu topology information
 *
 * Written by: Zhang Yanmin, Intel Corporation
 *
 * Copyright (C) 2006, Intel Corp.
 *
 * All rights reserved.
 */

// Kernel headers and symbols referenced by this translation are supplied by
// the surrounding kernel crate.

macro_rules! define_id_show_func {
    ($name:ident, $fmt:expr, $topology:ident) => {
        unsafe fn $name##_show(
            dev: *mut device,
            attr: *mut device_attribute,
            buf: *mut c_char,
        ) -> ssize_t {
            sysfs_emit(buf, concat!($fmt, "\n"), $topology((*dev).id))
        }
    };
}

macro_rules! define_siblings_read_func {
    ($name:ident, $topology:ident) => {
        unsafe fn $name##_read(
            file: *mut file,
            kobj: *mut kobject,
            attr: *const bin_attribute,
            buf: *mut c_char,
            off: loff_t,
            count: usize,
        ) -> ssize_t {
            let dev = kobj_to_dev(kobj);
            let mut mask: cpumask_var_t = core::ptr::null_mut();
            let mut n: ssize_t;
            if !alloc_cpumask_var(&mut mask, GFP_KERNEL) {
                return -ENOMEM;
            }
            cpumask_copy(mask, $topology((*dev).id));
            n = cpumap_print_bitmask_to_buf(buf, mask, off, count);
            free_cpumask_var(mask);
            n
        }

        unsafe fn $name##_list_read(
            file: *mut file,
            kobj: *mut kobject,
            attr: *const bin_attribute,
            buf: *mut c_char,
            off: loff_t,
            count: usize,
        ) -> ssize_t {
            let dev = kobj_to_dev(kobj);
            let mut mask: cpumask_var_t = core::ptr::null_mut();
            let mut n: ssize_t;
            if !alloc_cpumask_var(&mut mask, GFP_KERNEL) {
                return -ENOMEM;
            }
            cpumask_copy(mask, $topology((*dev).id));
            n = cpumap_print_list_to_buf(buf, mask, off, count);
            free_cpumask_var(mask);
            n
        }
    };
}

define_id_show_func!(physical_package_id, "%d", topology_physical_package_id);
// static DEVICE_ATTR_RO(physical_package_id);

#[cfg(feature = "TOPOLOGY_DIE_SYSFS")]
define_id_show_func!(die_id, "%d", topology_die_id);
#[cfg(feature = "TOPOLOGY_CLUSTER_SYSFS")]
define_id_show_func!(cluster_id, "%d", topology_cluster_id);
define_id_show_func!(core_id, "%d", topology_core_id);
define_id_show_func!(ppin, "0x%llx", topology_ppin);

define_siblings_read_func!(thread_siblings, topology_sibling_cpumask);
define_siblings_read_func!(core_cpus, topology_sibling_cpumask);
define_siblings_read_func!(core_siblings, topology_core_cpumask);
#[cfg(feature = "TOPOLOGY_CLUSTER_SYSFS")]
define_siblings_read_func!(cluster_cpus, topology_cluster_cpumask);
#[cfg(feature = "TOPOLOGY_DIE_SYSFS")]
define_siblings_read_func!(die_cpus, topology_die_cpumask);
define_siblings_read_func!(package_cpus, topology_core_cpumask);
#[cfg(feature = "TOPOLOGY_BOOK_SYSFS")]
define_id_show_func!(book_id, "%d", topology_book_id);
#[cfg(feature = "TOPOLOGY_BOOK_SYSFS")]
define_siblings_read_func!(book_siblings, topology_book_cpumask);
#[cfg(feature = "TOPOLOGY_DRAWER_SYSFS")]
define_id_show_func!(drawer_id, "%d", topology_drawer_id);
#[cfg(feature = "TOPOLOGY_DRAWER_SYSFS")]
define_siblings_read_func!(drawer_siblings, topology_drawer_cpumask);

// BIN_ATTR_RO and DEVICE_ATTR_RO declarations are represented by the
// corresponding kernel attribute objects supplied by the surrounding crate.
static BIN_ATTRS: [*const bin_attribute; 1] = [core::ptr::null()];
static mut DEFAULT_ATTRS: [*mut attribute; 1] = [core::ptr::null_mut()];

unsafe fn topology_is_visible(kobj: *mut kobject, attr: *mut attribute, unused: i32) -> umode_t {
    if attr == core::ptr::null_mut() && topology_ppin((*kobj_to_dev(kobj)).id) == 0 {
        return 0;
    }
    (*attr).mode
}

static TOPOLOGY_ATTR_GROUP: attribute_group = attribute_group {
    attrs: unsafe { DEFAULT_ATTRS.as_ptr() },
    bin_attrs: BIN_ATTRS.as_ptr(),
    is_visible: Some(topology_is_visible),
    name: b"topology\0".as_ptr() as *const c_char,
};

unsafe fn topology_add_dev(cpu: c_uint) -> c_int {
    let dev = get_cpu_device(cpu);
    sysfs_create_group(&mut (*dev).kobj, &TOPOLOGY_ATTR_GROUP)
}

unsafe fn topology_remove_dev(cpu: c_uint) -> c_int {
    let dev = get_cpu_device(cpu);
    sysfs_remove_group(&mut (*dev).kobj, &TOPOLOGY_ATTR_GROUP);
    0
}

unsafe fn topology_sysfs_init() -> c_int {
    cpuhp_setup_state(
        CPUHP_TOPOLOGY_PREPARE,
        b"base/topology:prepare\0".as_ptr() as *const c_char,
        Some(topology_add_dev),
        Some(topology_remove_dev),
    )
}

// device_initcall(topology_sysfs_init);

#[no_mangle]
pub static mut cpu_scale: per_cpu<unsigned_long> = per_cpu::new(SCHED_CAPACITY_SCALE);

pub unsafe fn topology_set_cpu_scale(cpu: c_uint, capacity: c_ulong) {
    *per_cpu_ptr(&mut cpu_scale, cpu) = capacity;
}

unsafe fn cpu_capacity_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let cpu = container_of!(dev, cpu, dev);
    sysfs_emit(buf, "%lu\n", topology_get_cpu_scale((*cpu).dev.id))
}

unsafe fn cpu_capacity_sysctl_add(cpu: c_uint) -> c_int {
    let cpu_dev = get_cpu_device(cpu);
    if cpu_dev.is_null() {
        return -ENOENT;
    }
    device_create_file(cpu_dev, &DEV_ATTR_CPU_CAPACITY);
    0
}

unsafe fn cpu_capacity_sysctl_remove(cpu: c_uint) -> c_int {
    let cpu_dev = get_cpu_device(cpu);
    if cpu_dev.is_null() {
        return -ENOENT;
    }
    device_remove_file(cpu_dev, &DEV_ATTR_CPU_CAPACITY);
    0
}

unsafe fn register_cpu_capacity_sysctl() -> c_int {
    cpuhp_setup_state(
        CPUHP_AP_ONLINE_DYN,
        b"topology/cpu-capacity\0".as_ptr() as *const c_char,
        Some(cpu_capacity_sysctl_add),
        Some(cpu_capacity_sysctl_remove),
    );
    0
}

// subsys_initcall(register_cpu_capacity_sysctl);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
