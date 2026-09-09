// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2013 MundoReader S.L.
 * Author: Heiko Stuebner <heiko@sntech.de>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

static mut scu_base_addr: *mut core::ffi::c_void = core::ptr::null_mut();
static mut sram_base_addr: *mut core::ffi::c_void = core::ptr::null_mut();
static mut ncores: i32 = 0;

const PMU_PWRDN_CON: u32 = 0x08;
const PMU_PWRDN_ST: u32 = 0x0c;
const PMU_PWRDN_SCU: i32 = 4;

static mut pmu: *mut regmap = core::ptr::null_mut();
static mut has_pmu: bool = true;
static mut cpu_rstc: [*mut reset_control; 4] = [core::ptr::null_mut(); 4];

unsafe fn pmu_power_domain_is_on(pd: i32) -> i32 {
    let mut val: u32 = 0;
    let ret = regmap_read(pmu, PMU_PWRDN_ST, &mut val);
    if ret < 0 {
        return ret;
    }
    if (val & (1u32 << pd)) == 0 { 1 } else { 0 }
}

unsafe fn rockchip_get_core_reset(cpu: i32) -> *mut reset_control {
    let dev = get_cpu_device(cpu);
    let np = if !dev.is_null() {
        (*dev).of_node
    } else {
        of_get_cpu_node(cpu, core::ptr::null_mut())
    };
    of_reset_control_get_exclusive(np, core::ptr::null())
}

unsafe fn pmu_set_power_domain(pd: i32, on: bool) -> i32 {
    let val: u32 = if on { 0 } else { 1u32 << pd };
    let rstc = if (pd as usize) < cpu_rstc.len() {
        cpu_rstc[pd as usize]
    } else {
        ERR_PTR(-EINVAL)
    };
    if IS_ERR(rstc) && read_cpuid_part() != ARM_CPU_PART_CORTEX_A9 {
        pr_err!("%s: could not get reset control for core %d\n", __func__, pd);
        return PTR_ERR(rstc);
    }
    if !IS_ERR(rstc) && !on {
        reset_control_assert(rstc);
    }
    if has_pmu {
        let mut ret = regmap_update_bits(pmu, PMU_PWRDN_CON, 1u32 << pd, val);
        if ret < 0 {
            pr_err!("%s: could not update power domain\n", __func__);
            return ret;
        }
        ret = -1;
        while ret != on as i32 {
            ret = pmu_power_domain_is_on(pd);
            if ret < 0 {
                pr_err!("%s: could not read power domain state\n", __func__);
                return ret;
            }
        }
    }
    if !IS_ERR(rstc) && on {
        reset_control_deassert(rstc);
    }
    0
}

/* Handling of CPU cores */
unsafe fn rockchip_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    if sram_base_addr.is_null() || (has_pmu && pmu.is_null()) {
        pr_err!("%s: sram or pmu missing for cpu boot\n", __func__);
        return -ENXIO;
    }
    if cpu as i32 >= ncores {
        pr_err!("%s: cpu %d outside maximum number of cpus %d\n", __func__, cpu, ncores);
        return -ENXIO;
    }
    let ret = pmu_set_power_domain(cpu as i32, true);
    if ret < 0 { return ret; }
    if read_cpuid_part() != ARM_CPU_PART_CORTEX_A9 {
        mdelay(1);
        writel(__pa_symbol(secondary_startup), (sram_base_addr as *mut u8).add(8) as _);
        writel(0xDEADBEAF, (sram_base_addr as *mut u8).add(4) as _);
        dsb_sev();
    }
    0
}

unsafe fn rockchip_smp_prepare_sram(node: *mut device_node) -> i32 {
    let trampoline_sz = (&rockchip_secondary_trampoline_end as *const _ as usize)
        - (&rockchip_secondary_trampoline as *const _ as usize);
    let mut res = resource { _private: [] };
    let ret = of_address_to_resource(node, 0, &mut res);
    if ret < 0 {
        pr_err!("%s: could not get address for node %pOF\n", __func__, node);
        return ret;
    }
    let rsize = resource_size(&res);
    if rsize < trampoline_sz as _ {
        pr_err!("%s: reserved block with size 0x%x is too small for trampoline size 0x%x\n", __func__, rsize, trampoline_sz);
        return -EINVAL;
    }
    rockchip_boot_fn = __pa_symbol(secondary_startup);
    memcpy_toio(sram_base_addr, &rockchip_secondary_trampoline as *const _ as _, trampoline_sz);
    flush_cache_all();
    outer_clean_range(0, trampoline_sz as _);
    dsb_sev();
    0
}

static rockchip_pmu_regmap_config: regmap_config = regmap_config {
    name: "rockchip-pmu",
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
};

unsafe fn rockchip_smp_prepare_pmu() -> i32 {
    let mut node: *mut device_node;
    let mut pmu_base: *mut core::ffi::c_void;
    node = of_find_node_by_path("/cpus");
    pmu = syscon_regmap_lookup_by_phandle(node, "rockchip,pmu");
    of_node_put(node);
    if !IS_ERR(pmu) { return 0; }
    pmu = syscon_regmap_lookup_by_compatible("rockchip,rk3066-pmu");
    if !IS_ERR(pmu) { return 0; }
    pmu = core::ptr::null_mut();
    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "rockchip,rk3066-pmu");
    if node.is_null() { pr_err!("%s: could not find pmu dt node\n", __func__); return -ENODEV; }
    pmu_base = of_iomap(node, 0);
    of_node_put(node);
    if pmu_base.is_null() { pr_err!("%s: could not map pmu registers\n", __func__); return -ENOMEM; }
    pmu = regmap_init_mmio(core::ptr::null_mut(), pmu_base, &rockchip_pmu_regmap_config);
    if IS_ERR(pmu) {
        let ret = PTR_ERR(pmu);
        iounmap(pmu_base);
        pmu = core::ptr::null_mut();
        pr_err!("%s: regmap init failed\n", __func__);
        return ret;
    }
    0
}

unsafe fn rockchip_smp_prepare_cpus(_max_cpus: u32) {
    let mut node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "rockchip,rk3066-smp-sram");
    if node.is_null() { pr_err!("%s: could not find sram dt node\n", __func__); return; }
    sram_base_addr = of_iomap(node, 0);
    if sram_base_addr.is_null() { pr_err!("%s: could not map sram registers\n", __func__); of_node_put(node); return; }
    if has_pmu && rockchip_smp_prepare_pmu() != 0 { of_node_put(node); return; }
    if read_cpuid_part() == ARM_CPU_PART_CORTEX_A9 {
        pmu_set_power_domain(PMU_PWRDN_SCU, true);
        of_node_put(node);
        node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "arm,cortex-a9-scu");
        if node.is_null() { pr_err!("%s: missing scu\n", __func__); return; }
        scu_base_addr = of_iomap(node, 0);
        if scu_base_addr.is_null() { pr_err!("%s: could not map scu registers\n", __func__); of_node_put(node); return; }
        ncores = scu_get_core_count(scu_base_addr);
        pr_err!("%s: ncores %d\n", __func__, ncores);
        scu_enable(scu_base_addr);
    } else {
        let l2ctlr: u32;
        core::arch::asm!("mrc p15, 1, {0}, c9, c0, 2", out(reg) l2ctlr);
        ncores = (((l2ctlr >> 24) & 0x3) + 1) as i32;
    }
    for i in 0..ncores { cpu_rstc[i as usize] = rockchip_get_core_reset(i); }
    for i in 1..ncores { pmu_set_power_domain(i, false); }
    if read_cpuid_part() == ARM_CPU_PART_CORTEX_A9 && rockchip_smp_prepare_sram(node) != 0 { of_node_put(node); return; }
    of_node_put(node);
}

unsafe fn rk3036_smp_prepare_cpus(max_cpus: u32) { has_pmu = false; rockchip_smp_prepare_cpus(max_cpus); }

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn rockchip_cpu_kill(cpu: u32) -> i32 { mdelay(1); pmu_set_power_domain(cpu as i32, false); 1 }

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn rockchip_cpu_die(_cpu: u32) -> ! { v7_exit_coherency_flush(louis); loop { cpu_do_idle(); } }

static rk3036_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(rk3036_smp_prepare_cpus),
    smp_boot_secondary: Some(rockchip_boot_secondary),
};

static rockchip_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: Some(rockchip_smp_prepare_cpus),
    smp_boot_secondary: Some(rockchip_boot_secondary),
};

CPU_METHOD_OF_DECLARE!(rk3036_smp, "rockchip,rk3036-smp", &rk3036_smp_ops);
CPU_METHOD_OF_DECLARE!(rk3066_smp, "rockchip,rk3066-smp", &rockchip_smp_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
