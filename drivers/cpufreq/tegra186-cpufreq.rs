// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017, NVIDIA CORPORATION. All rights reserved
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const TEGRA186_NUM_CLUSTERS: usize = 2;
const fn edvd_offset_a57(core: usize) -> usize { (SZ_64K * 6) + (0x20 + core * 0x4) }
const fn edvd_offset_denver(core: usize) -> usize { (SZ_64K * 7) + (0x20 + core * 0x4) }
const EDVD_CORE_VOLT_FREQ_F_SHIFT: u32 = 0;
const EDVD_CORE_VOLT_FREQ_F_MASK: u32 = 0xffff;
const EDVD_CORE_VOLT_FREQ_V_SHIFT: u32 = 16;

#[repr(C)]
struct tegra186_cpufreq_cpu { bpmp_cluster_id: u32, edvd_offset: u32 }

static TEGRA186_CPUS: [tegra186_cpufreq_cpu; 6] = [
    // CPU0 - A57 Cluster
    tegra186_cpufreq_cpu { bpmp_cluster_id: 1, edvd_offset: edvd_offset_a57(0) as u32 },
    // CPU1 - Denver Cluster
    tegra186_cpufreq_cpu { bpmp_cluster_id: 0, edvd_offset: edvd_offset_denver(0) as u32 },
    // CPU2 - Denver Cluster
    tegra186_cpufreq_cpu { bpmp_cluster_id: 0, edvd_offset: edvd_offset_denver(1) as u32 },
    // CPU3 - A57 Cluster
    tegra186_cpufreq_cpu { bpmp_cluster_id: 1, edvd_offset: edvd_offset_a57(1) as u32 },
    // CPU4 - A57 Cluster
    tegra186_cpufreq_cpu { bpmp_cluster_id: 1, edvd_offset: edvd_offset_a57(2) as u32 },
    // CPU5 - A57 Cluster
    tegra186_cpufreq_cpu { bpmp_cluster_id: 1, edvd_offset: edvd_offset_a57(3) as u32 },
];

#[repr(C)]
struct tegra186_cpufreq_cluster {
    bpmp_lut: *mut cpufreq_frequency_table,
    ref_clk_khz: u32,
    div: u32,
}

#[repr(C)]
struct tegra186_cpufreq_data {
    regs: *mut core::ffi::c_void,
    cpus: *const tegra186_cpufreq_cpu,
    icc_dram_bw_scaling: bool,
    clusters: [tegra186_cpufreq_cluster; TEGRA186_NUM_CLUSTERS],
}

unsafe fn tegra_cpufreq_set_bw(policy: *mut cpufreq_policy, freq_khz: usize) -> i32 {
    let data = cpufreq_get_driver_data() as *mut tegra186_cpufreq_data;
    let dev = get_cpu_device((*policy).cpu);
    if dev.is_null() { return -ENODEV; }
    let opp = dev_pm_opp_find_freq_exact(dev, freq_khz * HZ_PER_KHZ, true);
    if IS_ERR(opp) { return PTR_ERR(opp); }
    let ret = dev_pm_opp_set_opp(dev, opp);
    if ret != 0 { (*data).icc_dram_bw_scaling = false; }
    ret
}

unsafe fn tegra_cpufreq_init_cpufreq_table(
    policy: *mut cpufreq_policy, bpmp_lut: *mut cpufreq_frequency_table,
    opp_table: *mut *mut cpufreq_frequency_table) -> i32 {
    let data = cpufreq_get_driver_data() as *mut tegra186_cpufreq_data;
    let mut freq_table: *mut cpufreq_frequency_table = core::ptr::null_mut();
    let mut pos: *mut cpufreq_frequency_table = core::ptr::null_mut();
    let cpu_dev = get_cpu_device((*policy).cpu);
    if cpu_dev.is_null() { pr_err!("%s: failed to get cpu%d device\n", "tegra_cpufreq_init_cpufreq_table", (*policy).cpu); return -ENODEV; }
    let mut ret = dev_pm_opp_of_add_table_indexed(cpu_dev, 0);
    if ret != 0 { dev_err!(cpu_dev, "Invalid or empty opp table in device tree\n"); (*data).icc_dram_bw_scaling = false; return ret; }
    let max_opps = dev_pm_opp_get_opp_count(cpu_dev);
    if max_opps <= 0 { dev_err!(cpu_dev, "Failed to add OPPs\n"); return max_opps; }
    let mut rate: usize = 0;
    loop { let opp = dev_pm_opp_find_freq_ceil(cpu_dev, &mut rate); if IS_ERR(opp) { break; } dev_pm_opp_disable(cpu_dev, rate); rate += 1; }
    freq_table = kzalloc_objs::<cpufreq_frequency_table>((max_opps + 1) as usize);
    if freq_table.is_null() { return -ENOMEM; }
    let mut j = 0;
    cpufreq_for_each_valid_entry!(pos, bpmp_lut, {
        let opp = dev_pm_opp_find_freq_exact(cpu_dev, (*pos).frequency * HZ_PER_KHZ, false);
        if !IS_ERR(opp) { ret = dev_pm_opp_enable(cpu_dev, (*pos).frequency * HZ_PER_KHZ); if ret < 0 { return ret; } (*freq_table.add(j)).driver_data = (*pos).driver_data; (*freq_table.add(j)).frequency = (*pos).frequency; j += 1; }
    });
    (*freq_table.add(j)).driver_data = (*pos).driver_data;
    (*freq_table.add(j)).frequency = CPUFREQ_TABLE_END;
    *opp_table = freq_table;
    dev_pm_opp_set_sharing_cpus(cpu_dev, (*policy).cpus);
    tegra_cpufreq_set_bw(policy, (*freq_table.add(j - 1)).frequency);
    ret
}

unsafe fn tegra186_cpufreq_init(policy: *mut cpufreq_policy) -> i32 {
    let data = cpufreq_get_driver_data() as *mut tegra186_cpufreq_data;
    let cluster = (*data).cpus.add((*policy).cpu).as_ref().unwrap().bpmp_cluster_id;
    let mut freq_table = core::ptr::null_mut();
    (*policy).cpuinfo.transition_latency = 300 * 1000; (*policy).driver_data = core::ptr::null_mut();
    for cpu in 0..TEGRA186_CPUS.len() { if (*data).cpus.add(cpu).as_ref().unwrap().bpmp_cluster_id == cluster { cpumask_set_cpu(cpu as u32, (*policy).cpus); } }
    let bpmp_lut = (*data).clusters[cluster as usize].bpmp_lut;
    if (*data).icc_dram_bw_scaling && tegra_cpufreq_init_cpufreq_table(policy, bpmp_lut, &mut freq_table) == 0 { (*policy).freq_table = freq_table; return 0; }
    (*data).icc_dram_bw_scaling = false; (*policy).freq_table = bpmp_lut; pr_info!("OPP tables missing from DT, EMC frequency scaling disabled\n"); 0
}

unsafe fn tegra186_cpufreq_set_target(policy: *mut cpufreq_policy, index: u32) -> i32 {
    let data = cpufreq_get_driver_data() as *mut tegra186_cpufreq_data;
    let tbl = (*policy).freq_table.add(index as usize); let edvd_val = (*tbl).driver_data;
    for_each_cpu!(cpu, (*policy).cpus, { let off = (*data).cpus.add(cpu as usize).as_ref().unwrap().edvd_offset; writel(edvd_val, ((*data).regs as *mut u8).add(off as usize)); });
    if (*data).icc_dram_bw_scaling { tegra_cpufreq_set_bw(policy, (*tbl).frequency); } 0
}

unsafe fn tegra186_cpufreq_get(cpu: u32) -> u32 {
    let policy = cpufreq_cpu_get(cpu); if policy.is_null() { return 0; }
    let data = cpufreq_get_driver_data() as *mut tegra186_cpufreq_data;
    let c = (*data).cpus.add((*policy).cpu).as_ref().unwrap();
    let ndiv = readl(((*data).regs as *mut u8).add(c.edvd_offset as usize)) & EDVD_CORE_VOLT_FREQ_F_MASK;
    let cluster = &(*data).clusters[c.bpmp_cluster_id as usize]; (cluster.ref_clk_khz * ndiv) / cluster.div
}

unsafe fn tegra_cpufreq_bpmp_read_lut(pdev: *mut platform_device, bpmp: *mut tegra_bpmp,
    cluster: *mut tegra186_cpufreq_cluster, cluster_id: u32, num_rates: *mut i32) -> *mut cpufreq_frequency_table {
    let mut phys = 0; let virt = dma_alloc_coherent((*bpmp).dev, core::mem::size_of::<cpu_vhint_data>(), &mut phys, GFP_KERNEL);
    if virt.is_null() { return ERR_PTR(-ENOMEM); }
    let data = virt as *mut cpu_vhint_data; let mut req: mrq_cpu_vhint_request = core::mem::zeroed(); req.addr = phys; req.cluster_id = cluster_id;
    let mut msg: tegra_bpmp_message = core::mem::zeroed(); msg.mrq = MRQ_CPU_VHINT; msg.tx.data = &mut req as *mut _ as *mut _; msg.tx.size = core::mem::size_of_val(&req);
    let err = tegra_bpmp_transfer(bpmp, &mut msg); if err != 0 { dma_free_coherent((*bpmp).dev, core::mem::size_of::<cpu_vhint_data>(), virt, phys); return ERR_PTR(err); }
    if msg.rx.ret != 0 { dma_free_coherent((*bpmp).dev, core::mem::size_of::<cpu_vhint_data>(), virt, phys); return ERR_PTR(-EINVAL); }
    *num_rates = 0;
    for i in data.as_ref().unwrap().vfloor..=data.as_ref().unwrap().vceil { let n = (*data).ndiv[i as usize]; if n < (*data).ndiv_min || n > (*data).ndiv_max || (i > 0 && n == (*data).ndiv[(i-1) as usize]) { continue; } *num_rates += 1; }
    let table = devm_kcalloc(&mut (*pdev).dev, (*num_rates + 1) as usize, core::mem::size_of::<cpufreq_frequency_table>(), GFP_KERNEL);
    if table.is_null() { dma_free_coherent((*bpmp).dev, core::mem::size_of::<cpu_vhint_data>(), virt, phys); return ERR_PTR(-ENOMEM); }
    (*cluster).ref_clk_khz = (*data).ref_clk_hz / 1000; (*cluster).div = (*data).pdiv * (*data).mdiv;
    let mut j = 0usize;
    for i in (*data).vfloor..=(*data).vceil { let n = (*data).ndiv[i as usize]; if n < (*data).ndiv_min || n > (*data).ndiv_max || (i > 0 && n == (*data).ndiv[(i-1) as usize]) { continue; } let p = table.add(j); (*p).driver_data = ((i as u32) << EDVD_CORE_VOLT_FREQ_V_SHIFT) | ((n as u32) << EDVD_CORE_VOLT_FREQ_F_SHIFT); (*p).frequency = ((*cluster).ref_clk_khz * n as u32) / (*cluster).div; j += 1; }
    (*table.add(j)).frequency = CPUFREQ_TABLE_END; dma_free_coherent((*bpmp).dev, core::mem::size_of::<cpu_vhint_data>(), virt, phys); table
}

unsafe fn tegra186_cpufreq_probe(pdev: *mut platform_device) -> i32 {
    let data = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<tegra186_cpufreq_data>(), GFP_KERNEL) as *mut tegra186_cpufreq_data;
    if data.is_null() { return -ENOMEM; } (*data).cpus = TEGRA186_CPUS.as_ptr();
    let bpmp = tegra_bpmp_get(&mut (*pdev).dev); if IS_ERR(bpmp) { return PTR_ERR(bpmp); }
    (*data).regs = devm_platform_ioremap_resource(pdev, 0); if IS_ERR((*data).regs) { let e = PTR_ERR((*data).regs); tegra_bpmp_put(bpmp); return e; }
    let mut num_rates = 0; for i in 0..TEGRA186_NUM_CLUSTERS { let c = &mut (*data).clusters[i]; c.bpmp_lut = tegra_cpufreq_bpmp_read_lut(pdev, bpmp, c, i as u32, &mut num_rates); if IS_ERR(c.bpmp_lut) || num_rates == 0 { tegra_bpmp_put(bpmp); return if IS_ERR(c.bpmp_lut) { PTR_ERR(c.bpmp_lut) } else { -EINVAL }; } }
    tegra186_cpufreq_driver.driver_data = data as *mut _; let cpu_dev = get_cpu_device(0); if cpu_dev.is_null() { tegra_bpmp_put(bpmp); return -EPROBE_DEFER; }
    if dev_pm_opp_of_get_opp_desc_node(cpu_dev) && dev_pm_opp_of_find_icc_paths(cpu_dev, core::ptr::null_mut()) == 0 { (*data).icc_dram_bw_scaling = true; }
    let err = cpufreq_register_driver(&mut tegra186_cpufreq_driver); tegra_bpmp_put(bpmp); err
}

unsafe fn tegra186_cpufreq_remove(_pdev: *mut platform_device) { cpufreq_unregister_driver(&mut tegra186_cpufreq_driver); }

// Device-tree match table, platform-driver registration, and module metadata.
static TEGRA186_CPUFREQ_OF_MATCH: &[of_device_id] = &[of_device_id { compatible: b"nvidia,tegra186-ccplex-cluster\0".as_ptr() as *const _ }, of_device_id { compatible: core::ptr::null() }];

static mut tegra186_cpufreq_driver: cpufreq_driver = cpufreq_driver {
    name: b"tegra186\0".as_ptr() as *const _,
    flags: CPUFREQ_HAVE_GOVERNOR_PER_POLICY | CPUFREQ_NEED_INITIAL_FREQ_CHECK,
    get: Some(tegra186_cpufreq_get), verify: Some(cpufreq_generic_frequency_table_verify),
    target_index: Some(tegra186_cpufreq_set_target), init: Some(tegra186_cpufreq_init),
    driver_data: core::ptr::null_mut(),
};

static mut TEGRA186_CPUFREQ_PLATFORM_DRIVER: platform_driver = platform_driver {
    driver: device_driver { name: b"tegra186-cpufreq\0".as_ptr() as *const _, of_match_table: TEGRA186_CPUFREQ_OF_MATCH.as_ptr() },
    probe: Some(tegra186_cpufreq_probe), remove: Some(tegra186_cpufreq_remove),
};

// Equivalent of module_platform_driver(tegra186_cpufreq_platform_driver).
// MODULE_AUTHOR("Mikko Perttunen <mperttunen@nvidia.com>");
// MODULE_DESCRIPTION("NVIDIA Tegra186 cpufreq driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
