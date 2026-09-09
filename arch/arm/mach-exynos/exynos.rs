// SPDX-License-Identifier: GPL-2.0
//
// Samsung Exynos Flattened Device Tree enabled machine
//
// Copyright (c) 2010-2014 Samsung Electronics Co., Ltd.
//		http://www.samsung.com

// Linux kernel headers and the local common header are supplied by other
// translated units.

const S3C_ADDR_BASE: usize = 0xF6000000;
#[inline]
unsafe fn S3C_ADDR(x: usize) -> *mut core::ffi::c_void {
    (S3C_ADDR_BASE + x) as *mut core::ffi::c_void
}
#[allow(non_upper_case_globals)]
static S5P_VA_CHIPID: *mut core::ffi::c_void = S3C_ADDR(0x02000000);

// CONFIG_ARM_EXYNOS_CPUIDLE selects the platform_data initializer here.
#[repr(C)]
static mut exynos_cpuidle: platform_device = platform_device {
    name: "exynos_cpuidle" as *const str as *const core::ffi::c_char,
    #[cfg(CONFIG_ARM_EXYNOS_CPUIDLE)]
    dev: device { platform_data: exynos_enter_aftr as *const core::ffi::c_void },
    id: -1,
};

static mut sysram_base_addr: *mut core::ffi::c_void = core::ptr::null_mut();
static mut sysram_base_phys: phys_addr_t = 0;
static mut sysram_ns_base_addr: *mut core::ffi::c_void = core::ptr::null_mut();

static mut exynos_cpu_id: usize = 0;
static mut exynos_cpu_rev: u32 = 0;

pub unsafe fn exynos_rev() -> u32 {
    exynos_cpu_rev
}

pub unsafe fn exynos_sysram_init() {
    // for_each_compatible_node_scoped(node, NULL, "samsung,exynos4210-sysram")
    // is a kernel scoped device-tree iterator.
    for node in compatible_nodes("samsung,exynos4210-sysram") {
        let mut res: resource = core::mem::zeroed();
        if !of_device_is_available(node) {
            continue;
        }
        of_address_to_resource(node, 0, &mut res);
        sysram_base_addr = ioremap(res.start, resource_size(&res));
        sysram_base_phys = res.start;
        break;
    }

    for node in compatible_nodes("samsung,exynos4210-sysram-ns") {
        if !of_device_is_available(node) {
            continue;
        }
        sysram_ns_base_addr = of_iomap(node, 0);
        break;
    }
}

unsafe fn exynos_fdt_map_chipid(node: usize, _uname: *const core::ffi::c_char,
                                _depth: i32, _data: *mut core::ffi::c_void) -> i32 {
    let mut iodesc: map_desc = core::mem::zeroed();
    let mut len: i32 = 0;
    if !of_flat_dt_is_compatible(node, "samsung,exynos4210-chipid") {
        return 0;
    }
    let reg = of_get_flat_dt_prop(node, "reg", &mut len);
    if reg.is_null() || len != (core::mem::size_of::<usize>() * 2) as i32 {
        return 0;
    }
    iodesc.pfn = __phys_to_pfn(be32_to_cpu(*reg));
    iodesc.length = be32_to_cpu(*reg.add(1)) - 1;
    iodesc.virtual_ = S5P_VA_CHIPID as usize;
    iodesc.type_ = MT_DEVICE;
    iotable_init(&mut iodesc, 1);
    1
}

unsafe fn exynos_init_io() {
    debug_ll_io_init();
    of_scan_flat_dt(Some(exynos_fdt_map_chipid), core::ptr::null_mut());
    // detect cpu id and rev.
    exynos_cpu_id = readl_relaxed(S5P_VA_CHIPID) as usize;
    exynos_cpu_rev = (exynos_cpu_id & 0xFF) as u32;
    pr_info!("Samsung CPU ID: 0x{:08x}\n", exynos_cpu_id);
}

pub unsafe fn exynos_set_delayed_reset_assertion(enable: bool) {
    if of_machine_is_compatible("samsung,exynos4") {
        let mut core_id = 0;
        while core_id < num_possible_cpus() {
            let mut tmp = pmu_raw_readl(EXYNOS_ARM_CORE_OPTION(core_id));
            if enable {
                tmp |= S5P_USE_DELAYED_RESET_ASSERTION;
            } else {
                tmp &= !S5P_USE_DELAYED_RESET_ASSERTION;
            }
            pmu_raw_writel(tmp, EXYNOS_ARM_CORE_OPTION(core_id));
            core_id += 1;
        }
    }
}

// Apparently, these SoCs are not able to wake-up from suspend using the PMU.
static exynos_dt_pmu_match: [of_device_id; 3] = [
    of_device_id { compatible: "samsung,exynos5260-pmu" },
    of_device_id { compatible: "samsung,exynos5410-pmu" },
    of_device_id { compatible: core::ptr::null() },
];

unsafe fn exynos_map_pmu() {
    let np = of_find_matching_node(core::ptr::null_mut(), exynos_dt_pmu_match.as_ptr());
    if !np.is_null() {
        pmu_base_addr = of_iomap(np, 0);
    }
    of_node_put(np);
}

unsafe fn exynos_init_irq() {
    irqchip_init();
    // platsmp.c needs the PMU base address before the device tree is unflattened.
    exynos_map_pmu();
}

unsafe fn exynos_dt_machine_init() {
    if !IS_ENABLED(CONFIG_SMP) {
        exynos_sysram_init();
    }
    // CONFIG_SMP and CONFIG_ARM_EXYNOS_CPUIDLE conditional block.
    if (of_machine_is_compatible("samsung,exynos4210") ||
        of_machine_is_compatible("samsung,exynos3250")) {
        exynos_cpuidle.dev.platform_data = &cpuidle_coupled_exynos_data;
    }
    if of_machine_is_compatible("samsung,exynos4210") ||
       of_machine_is_compatible("samsung,exynos4212") ||
       (of_machine_is_compatible("samsung,exynos4412") &&
        (of_machine_is_compatible("samsung,trats2") ||
         of_machine_is_compatible("samsung,midas") ||
         of_machine_is_compatible("samsung,p4note"))) ||
       of_machine_is_compatible("samsung,exynos3250") ||
       of_machine_is_compatible("samsung,exynos5250") {
        platform_device_register(&mut exynos_cpuidle);
    }
}

static exynos_dt_compat: [*const core::ffi::c_char; 11] = [
    "samsung,exynos3", "samsung,exynos3250", "samsung,exynos4",
    "samsung,exynos4210", "samsung,exynos4212", "samsung,exynos4412",
    "samsung,exynos5", "samsung,exynos5250", "samsung,exynos5260",
    "samsung,exynos5420", core::ptr::null(),
];

unsafe fn exynos_dt_fixup() {
    // Some versions of uboot pass garbage entries in the memory node.
    of_fdt_limit_memory(8);
}

// DT_MACHINE_START(EXYNOS_DT, "Samsung Exynos (Flattened Device Tree)")
// .l2c_aux_val = 0x08400000, .l2c_aux_mask = 0xf60fffff,
// .smp = smp_ops(exynos_smp_ops), .map_io = exynos_init_io,
// .init_early = exynos_firmware_init, .init_irq = exynos_init_irq,
// .init_machine = exynos_dt_machine_init, .init_late = exynos_pm_init,
// .dt_compat = exynos_dt_compat, .dt_fixup = exynos_dt_fixup, MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
