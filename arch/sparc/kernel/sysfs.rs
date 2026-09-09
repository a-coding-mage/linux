// SPDX-License-Identifier: GPL-2.0
/* sysfs.c: Topology sysfs support code for sparc64.
 *
 * Copyright (C) 2007 David S. Miller <davem@davemloft.net>
 */

// Linux and architecture headers are supplied by the surrounding kernel.

static mut mmu_stats: PerCpu<hv_mmu_statistics> = DEFINE_PER_CPU_ALIGNED::<hv_mmu_statistics, 64>();

macro_rules! SHOW_MMUSTAT_ULONG {
    ($name:ident) => {
        unsafe fn show_$name(
            _dev: *mut device,
            _attr: *mut device_attribute,
            buf: *mut c_char,
        ) -> ssize_t {
            let p: *mut hv_mmu_statistics = &mut per_cpu!(mmu_stats, (*_dev).id);
            sprintf(buf, "%lu\n", (*p).$name)
        }
        static mut dev_attr_$name: device_attribute = DEVICE_ATTR!($name, 0o444, show_$name, None);
    };
}

SHOW_MMUSTAT_ULONG!(immu_tsb_hits_ctx0_8k_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_ticks_ctx0_8k_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_hits_ctx0_64k_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_ticks_ctx0_64k_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_hits_ctx0_4mb_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_ticks_ctx0_4mb_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_hits_ctx0_256mb_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_ticks_ctx0_256mb_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_hits_ctxnon0_8k_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_ticks_ctxnon0_8k_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_hits_ctxnon0_64k_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_ticks_ctxnon0_64k_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_hits_ctxnon0_4mb_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_ticks_ctxnon0_4mb_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_hits_ctxnon0_256mb_tte);
SHOW_MMUSTAT_ULONG!(immu_tsb_ticks_ctxnon0_256mb_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_hits_ctx0_8k_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_ticks_ctx0_8k_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_hits_ctx0_64k_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_ticks_ctx0_64k_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_hits_ctx0_4mb_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_ticks_ctx0_4mb_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_hits_ctx0_256mb_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_ticks_ctx0_256mb_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_hits_ctxnon0_8k_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_ticks_ctxnon0_8k_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_hits_ctxnon0_64k_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_ticks_ctxnon0_64k_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_hits_ctxnon0_4mb_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_ticks_ctxnon0_4mb_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_hits_ctxnon0_256mb_tte);
SHOW_MMUSTAT_ULONG!(dmmu_tsb_ticks_ctxnon0_256mb_tte);

static mut mmu_stat_attrs: [*mut attribute; 33] = [
    &mut dev_attr_immu_tsb_hits_ctx0_8k_tte.attr, &mut dev_attr_immu_tsb_ticks_ctx0_8k_tte.attr,
    &mut dev_attr_immu_tsb_hits_ctx0_64k_tte.attr, &mut dev_attr_immu_tsb_ticks_ctx0_64k_tte.attr,
    &mut dev_attr_immu_tsb_hits_ctx0_4mb_tte.attr, &mut dev_attr_immu_tsb_ticks_ctx0_4mb_tte.attr,
    &mut dev_attr_immu_tsb_hits_ctx0_256mb_tte.attr, &mut dev_attr_immu_tsb_ticks_ctx0_256mb_tte.attr,
    &mut dev_attr_immu_tsb_hits_ctxnon0_8k_tte.attr, &mut dev_attr_immu_tsb_ticks_ctxnon0_8k_tte.attr,
    &mut dev_attr_immu_tsb_hits_ctxnon0_64k_tte.attr, &mut dev_attr_immu_tsb_ticks_ctxnon0_64k_tte.attr,
    &mut dev_attr_immu_tsb_hits_ctxnon0_4mb_tte.attr, &mut dev_attr_immu_tsb_ticks_ctxnon0_4mb_tte.attr,
    &mut dev_attr_immu_tsb_hits_ctxnon0_256mb_tte.attr, &mut dev_attr_immu_tsb_ticks_ctxnon0_256mb_tte.attr,
    &mut dev_attr_dmmu_tsb_hits_ctx0_8k_tte.attr, &mut dev_attr_dmmu_tsb_ticks_ctx0_8k_tte.attr,
    &mut dev_attr_dmmu_tsb_hits_ctx0_64k_tte.attr, &mut dev_attr_dmmu_tsb_ticks_ctx0_64k_tte.attr,
    &mut dev_attr_dmmu_tsb_hits_ctx0_4mb_tte.attr, &mut dev_attr_dmmu_tsb_ticks_ctx0_4mb_tte.attr,
    &mut dev_attr_dmmu_tsb_hits_ctx0_256mb_tte.attr, &mut dev_attr_dmmu_tsb_ticks_ctx0_256mb_tte.attr,
    &mut dev_attr_dmmu_tsb_hits_ctxnon0_8k_tte.attr, &mut dev_attr_dmmu_tsb_ticks_ctxnon0_8k_tte.attr,
    &mut dev_attr_dmmu_tsb_hits_ctxnon0_64k_tte.attr, &mut dev_attr_dmmu_tsb_ticks_ctxnon0_64k_tte.attr,
    &mut dev_attr_dmmu_tsb_hits_ctxnon0_4mb_tte.attr, &mut dev_attr_dmmu_tsb_ticks_ctxnon0_4mb_tte.attr,
    &mut dev_attr_dmmu_tsb_hits_ctxnon0_256mb_tte.attr, &mut dev_attr_dmmu_tsb_ticks_ctxnon0_256mb_tte.attr,
    core::ptr::null_mut(),
];

static mut mmu_stat_group: attribute_group = attribute_group { attrs: mmu_stat_attrs.as_mut_ptr(), name: "mmu_stats\0".as_ptr() as *const c_char };

unsafe fn read_mmustat_enable(_data: *mut c_void) -> c_long {
    let mut ra: c_ulong = 0;
    sun4v_mmustat_info(&mut ra);
    (ra != 0) as c_long
}

unsafe fn write_mmustat_enable(data: *mut c_void) -> c_long {
    let val = data as *mut c_ulong;
    let ra = if *val { __pa(&mut per_cpu!(mmu_stats, smp_processor_id())) } else { 0 };
    let mut orig_ra: c_ulong = 0;
    sun4v_mmustat_conf(ra, &mut orig_ra)
}

unsafe fn show_mmustat_enable(s: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let val = work_on_cpu((*s).id, read_mmustat_enable, core::ptr::null_mut());
    sprintf(buf, "%lx\n", val)
}

unsafe fn store_mmustat_enable(s: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let mut val: c_ulong = 0;
    if sscanf(buf, "%lu\0".as_ptr() as *const c_char, &mut val) != 1 { return -EINVAL; }
    if work_on_cpu((*s).id, write_mmustat_enable, &mut val as *mut _ as *mut c_void) != 0 { return -EIO; }
    count as ssize_t
}

static mut dev_attr_mmustat_enable: device_attribute = DEVICE_ATTR!(mmustat_enable, 0o644, show_mmustat_enable, store_mmustat_enable);
static mut mmu_stats_supported: c_int = 0;

unsafe fn register_mmu_stats(s: *mut device) -> c_int {
    if mmu_stats_supported == 0 { return 0; }
    device_create_file(s, &mut dev_attr_mmustat_enable);
    sysfs_create_group(&mut (*s).kobj, &mut mmu_stat_group)
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn unregister_mmu_stats(s: *mut device) {
    if mmu_stats_supported == 0 { return; }
    sysfs_remove_group(&mut (*s).kobj, &mut mmu_stat_group);
    device_remove_file(s, &mut dev_attr_mmustat_enable);
}

macro_rules! SHOW_CPUDATA_ULONG_NAME {
    ($name:ident, $member:ident) => {
        unsafe fn show_$name(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
            let c: *mut cpuinfo_sparc = cpu_data((*dev).id);
            sprintf(buf, "%lu\n", (*c).$member)
        }
    };
}
macro_rules! SHOW_CPUDATA_UINT_NAME {
    ($name:ident, $member:ident) => {
        unsafe fn show_$name(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
            let c: *mut cpuinfo_sparc = cpu_data((*dev).id);
            sprintf(buf, "%u\n", (*c).$member)
        }
    };
}

SHOW_CPUDATA_ULONG_NAME!(clock_tick, clock_tick);
SHOW_CPUDATA_UINT_NAME!(l1_dcache_size, dcache_size);
SHOW_CPUDATA_UINT_NAME!(l1_dcache_line_size, dcache_line_size);
SHOW_CPUDATA_UINT_NAME!(l1_icache_size, icache_size);
SHOW_CPUDATA_UINT_NAME!(l1_icache_line_size, icache_line_size);
SHOW_CPUDATA_UINT_NAME!(l2_cache_size, ecache_size);
SHOW_CPUDATA_UINT_NAME!(l2_cache_line_size, ecache_line_size);

static mut cpu_core_attrs: [device_attribute; 7] = [
    __ATTR!(clock_tick, 0o444, show_clock_tick, None),
    __ATTR!(l1_dcache_size, 0o444, show_l1_dcache_size, None),
    __ATTR!(l1_dcache_line_size, 0o444, show_l1_dcache_line_size, None),
    __ATTR!(l1_icache_size, 0o444, show_l1_icache_size, None),
    __ATTR!(l1_icache_line_size, 0o444, show_l1_icache_line_size, None),
    __ATTR!(l2_cache_size, 0o444, show_l2_cache_size, None),
    __ATTR!(l2_cache_line_size, 0o444, show_l2_cache_line_size, None),
];

static mut cpu_devices: PerCpu<cpu> = DEFINE_PER_CPU::<cpu>();

unsafe fn register_cpu_online(cpu: c_uint) -> c_int {
    let c: *mut cpu = &mut per_cpu!(cpu_devices, cpu);
    let s: *mut device = &mut (*c).dev;
    for i in 0..ARRAY_SIZE!(cpu_core_attrs) { device_create_file(s, &mut cpu_core_attrs[i]); }
    register_mmu_stats(s);
    0
}

unsafe fn unregister_cpu_online(cpu: c_uint) -> c_int {
    #[cfg(CONFIG_HOTPLUG_CPU)] {
        let c: *mut cpu = &mut per_cpu!(cpu_devices, cpu);
        let s: *mut device = &mut (*c).dev;
        unregister_mmu_stats(s);
        for i in 0..ARRAY_SIZE!(cpu_core_attrs) { device_remove_file(s, &mut cpu_core_attrs[i]); }
    }
    0
}

unsafe fn check_mmu_stats() {
    let mut dummy1: c_ulong = 0;
    let err: c_ulong;
    if tlb_type != hypervisor { return; }
    err = sun4v_mmustat_info(&mut dummy1);
    if err == 0 { mmu_stats_supported = 1; }
}

unsafe fn topology_init() -> c_int {
    check_mmu_stats();
    for_each_possible_cpu!(cpu, {
        let c: *mut cpu = &mut per_cpu!(cpu_devices, cpu);
        register_cpu(c, cpu);
    });
    let ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "sparc/topology:online\0".as_ptr() as *const c_char, register_cpu_online, unregister_cpu_online);
    WARN_ON!(ret < 0);
    0
}

subsys_initcall!(topology_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
