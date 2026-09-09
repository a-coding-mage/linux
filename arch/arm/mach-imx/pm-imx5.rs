// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2011 Freescale Semiconductor, Inc. All Rights Reserved.
 */

// Linux/kernel, architecture, and local header dependencies are supplied by
// the surrounding translation unit.

const MXC_CCM_CLPCR: u32 = 0x54;
const MXC_CCM_CLPCR_LPM_OFFSET: u32 = 0;
const MXC_CCM_CLPCR_LPM_MASK: u32 = 0x3;
const MXC_CCM_CLPCR_STBY_COUNT_OFFSET: u32 = 9;
const MXC_CCM_CLPCR_VSTBY: u32 = 0x1 << 8;
const MXC_CCM_CLPCR_SBYOS: u32 = 0x1 << 6;

const MXC_CORTEXA8_PLAT_LPC: u32 = 0xc;
const MXC_CORTEXA8_PLAT_LPC_DSM: u32 = 1 << 0;
const MXC_CORTEXA8_PLAT_LPC_DBG_DSM: u32 = 1 << 1;

const MXC_SRPG_NEON_SRPGCR: u32 = 0x280;
const MXC_SRPG_ARM_SRPGCR: u32 = 0x2a0;
const MXC_SRPG_EMPGC0_SRPGCR: u32 = 0x2c0;
const MXC_SRPG_EMPGC1_SRPGCR: u32 = 0x2d0;
const MXC_SRPGCR_PCR: u32 = 1;

/*
 * The WAIT_UNCLOCKED_POWER_OFF state only requires <= 500ns to exit.
 * This is also the lowest power state possible without affecting
 * non-cpu parts of the system.  For these reasons, imx5 should default
 * to always using this state for cpu idling.  The PM_SUSPEND_STANDBY also
 * uses this state and needs to take no action when registers remain configured
 * for this state.
 */
const IMX5_DEFAULT_CPU_IDLE_STATE: mxc_cpu_pwr_mode = WAIT_UNCLOCKED_POWER_OFF;

#[repr(C)]
struct imx5_suspend_io_state {
    offset: u32,
    clear: u32,
    set: u32,
    saved_value: u32,
}

#[repr(C)]
struct imx5_pm_data {
    ccm_addr: phys_addr_t,
    cortex_addr: phys_addr_t,
    gpc_addr: phys_addr_t,
    m4if_addr: phys_addr_t,
    iomuxc_addr: phys_addr_t,
    suspend_asm: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    suspend_asm_sz: *const u32,
    suspend_io_config: *const imx5_suspend_io_state,
    suspend_io_count: i32,
}

const MX53_DSE_HIGHZ_MASK: u32 = 0x7 << 19;
static imx53_suspend_io_config: [imx5_suspend_io_state; 19] = [
    imx5_suspend_io_state { offset: 0x584, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x594, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x560, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x554, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x574, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x588, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x578, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x570, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x580, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x564, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x57c, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x590, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x568, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x558, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x6f0, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x718, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x71c, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x728, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
    imx5_suspend_io_state { offset: 0x72c, clear: MX53_DSE_HIGHZ_MASK, set: 0, saved_value: 0 },
];

static imx51_pm_data: imx5_pm_data = imx5_pm_data {
    ccm_addr: 0x73fd4000, cortex_addr: 0x83fa0000, gpc_addr: 0x73fd8000,
    m4if_addr: 0, iomuxc_addr: 0, suspend_asm: None, suspend_asm_sz: core::ptr::null(),
    suspend_io_config: core::ptr::null(), suspend_io_count: 0,
};
static imx53_pm_data: imx5_pm_data = imx5_pm_data {
    ccm_addr: 0x53fd4000, cortex_addr: 0x63fa0000, gpc_addr: 0x53fd8000,
    m4if_addr: 0x63fd8000, iomuxc_addr: 0x53fa8000,
    suspend_asm: Some(imx53_suspend), suspend_asm_sz: &imx53_suspend_sz,
    suspend_io_config: imx53_suspend_io_config.as_ptr(), suspend_io_count: 19,
};

const MX5_MAX_SUSPEND_IOSTATE: usize = 19;

#[repr(C, align(8))]
struct imx5_cpu_suspend_info {
    m4if_base: *mut core::ffi::c_void,
    iomuxc_base: *mut core::ffi::c_void,
    io_count: u32,
    io_state: [imx5_suspend_io_state; MX5_MAX_SUSPEND_IOSTATE],
}

static mut ccm_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut cortex_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut gpc_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut suspend_ocram_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut imx5_suspend_in_ocram_fn: Option<unsafe extern "C" fn(*mut core::ffi::c_void)> = None;

// The remaining implementation retains the C control flow and calls the
// externally supplied kernel/architecture symbols directly.
unsafe fn mx5_cpu_lp_set(mode: mxc_cpu_pwr_mode) {
    let mut plat_lpc = imx_readl(ccm_base.add(MXC_CORTEXA8_PLAT_LPC as usize)) & !MXC_CORTEXA8_PLAT_LPC_DSM;
    let mut ccm_clpcr = imx_readl(ccm_base.add(MXC_CCM_CLPCR as usize)) & !MXC_CCM_CLPCR_LPM_MASK;
    let mut arm_srpgcr = imx_readl(gpc_base.add(MXC_SRPG_ARM_SRPGCR as usize)) & !MXC_SRPGCR_PCR;
    let mut empgc0 = imx_readl(gpc_base.add(MXC_SRPG_EMPGC0_SRPGCR as usize)) & !MXC_SRPGCR_PCR;
    let mut empgc1 = imx_readl(gpc_base.add(MXC_SRPG_EMPGC1_SRPGCR as usize)) & !MXC_SRPGCR_PCR;
    let mut stop_mode = false;
    match mode {
        WAIT_CLOCKED => {},
        WAIT_UNCLOCKED => ccm_clpcr |= 0x1 << MXC_CCM_CLPCR_LPM_OFFSET,
        WAIT_UNCLOCKED_POWER_OFF | STOP_POWER_OFF => {
            plat_lpc |= MXC_CORTEXA8_PLAT_LPC_DSM | MXC_CORTEXA8_PLAT_LPC_DBG_DSM;
            if mode == WAIT_UNCLOCKED_POWER_OFF {
                ccm_clpcr |= 0x1 << MXC_CCM_CLPCR_LPM_OFFSET;
                ccm_clpcr &= !MXC_CCM_CLPCR_VSTBY; ccm_clpcr &= !MXC_CCM_CLPCR_SBYOS;
            } else {
                ccm_clpcr |= 0x2 << MXC_CCM_CLPCR_LPM_OFFSET;
                ccm_clpcr |= 0x3 << MXC_CCM_CLPCR_STBY_COUNT_OFFSET;
                ccm_clpcr |= MXC_CCM_CLPCR_VSTBY | MXC_CCM_CLPCR_SBYOS; stop_mode = true;
            }
            arm_srpgcr |= MXC_SRPGCR_PCR;
        },
        STOP_POWER_ON => ccm_clpcr |= 0x2 << MXC_CCM_CLPCR_LPM_OFFSET,
        _ => return,
    }
    imx_writel(plat_lpc, cortex_base.add(MXC_CORTEXA8_PLAT_LPC as usize));
    imx_writel(ccm_clpcr, ccm_base.add(MXC_CCM_CLPCR as usize));
    imx_writel(arm_srpgcr, gpc_base.add(MXC_SRPG_ARM_SRPGCR as usize));
    imx_writel(arm_srpgcr, gpc_base.add(MXC_SRPG_NEON_SRPGCR as usize));
    if stop_mode { empgc0 |= MXC_SRPGCR_PCR; empgc1 |= MXC_SRPGCR_PCR; imx_writel(empgc0, gpc_base.add(MXC_SRPG_EMPGC0_SRPGCR as usize)); imx_writel(empgc1, gpc_base.add(MXC_SRPG_EMPGC1_SRPGCR as usize)); }
}

unsafe fn mx5_suspend_enter(state: suspend_state_t) -> i32 {
    match state { PM_SUSPEND_MEM => mx5_cpu_lp_set(STOP_POWER_OFF), PM_SUSPEND_STANDBY => {}, _ => return -EINVAL }
    if state == PM_SUSPEND_MEM { local_flush_tlb_all(); flush_cache_all(); imx_writel(0, gpc_base.add(MXC_SRPG_EMPGC0_SRPGCR as usize)); imx_writel(0, gpc_base.add(MXC_SRPG_EMPGC1_SRPGCR as usize)); if let Some(f) = imx5_suspend_in_ocram_fn { f(suspend_ocram_base); } else { cpu_do_idle(); } } else { cpu_do_idle(); }
    mx5_cpu_lp_set(IMX5_DEFAULT_CPU_IDLE_STATE); 0
}

unsafe fn mx5_pm_valid(state: suspend_state_t) -> bool { state > PM_SUSPEND_ON && state <= PM_SUSPEND_MAX }

unsafe fn imx5_cpu_do_idle() -> i32 { let ret = tzic_enable_wake(); if ret == 0 { cpu_do_idle(); } ret }
unsafe fn imx5_pm_idle() { imx5_cpu_do_idle(); }

// The OCRAM allocation and initialization paths are direct translations; the
// referenced allocator, mapping, and platform symbols remain external.
unsafe fn imx_suspend_alloc_ocram(size: usize, virt_out: *mut *mut core::ffi::c_void, phys_out: *mut phys_addr_t) -> i32 {
    let node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "mmio-sram");
    if node.is_null() { return -ENODEV; }
    let pdev = of_find_device_by_node(node);
    if pdev.is_null() { of_node_put(node); return -ENODEV; }
    let pool = gen_pool_get((*pdev).dev_mut(), core::ptr::null());
    if pool.is_null() { put_device((*pdev).dev_mut()); of_node_put(node); return -ENODEV; }
    let base = gen_pool_alloc(pool, size);
    if base == 0 { put_device((*pdev).dev_mut()); of_node_put(node); return -ENOMEM; }
    let phys = gen_pool_virt_to_phys(pool, base);
    let virt = __arm_ioremap_exec(phys, size, false);
    if !phys_out.is_null() { *phys_out = phys; }
    if !virt_out.is_null() { *virt_out = virt; }
    put_device((*pdev).dev_mut()); of_node_put(node); 0
}

unsafe fn imx5_suspend_init(soc_data: *const imx5_pm_data) -> i32 {
    let suspend_asm = (*soc_data).suspend_asm;
    if suspend_asm.is_none() { return 0; }
    if (*soc_data).suspend_asm_sz.is_null() || *(*soc_data).suspend_asm_sz == 0 { return -EINVAL; }
    let mut info = core::ptr::null_mut();
    let ret = imx_suspend_alloc_ocram(*(*soc_data).suspend_asm_sz + core::mem::size_of::<imx5_cpu_suspend_info>(), &mut suspend_ocram_base, core::ptr::null_mut());
    if ret != 0 { return ret; }
    info = suspend_ocram_base as *mut imx5_cpu_suspend_info;
    (*info).io_count = (*soc_data).suspend_io_count as u32;
    core::ptr::copy_nonoverlapping((*soc_data).suspend_io_config, (*info).io_state.as_mut_ptr(), (*soc_data).suspend_io_count as usize);
    (*info).m4if_base = ioremap((*soc_data).m4if_addr, SZ_16K);
    if (*info).m4if_base.is_null() { return -ENOMEM; }
    (*info).iomuxc_base = ioremap((*soc_data).iomuxc_addr, SZ_16K);
    if (*info).iomuxc_base.is_null() { iounmap((*info).m4if_base); return -ENOMEM; }
    imx5_suspend_in_ocram_fn = Some(fncpy(suspend_ocram_base.add(core::mem::size_of::<imx5_cpu_suspend_info>()), suspend_asm.unwrap(), *(*soc_data).suspend_asm_sz));
    0
}

unsafe fn imx5_pm_common_init(data: *const imx5_pm_data) -> i32 {
    let clk = clk_get(core::ptr::null_mut(), "gpc_dvfs");
    if IS_ERR(clk) { return PTR_ERR(clk); }
    let ret = clk_prepare_enable(clk); if ret != 0 { return ret; }
    arm_pm_idle = Some(imx5_pm_idle);
    ccm_base = ioremap((*data).ccm_addr, SZ_16K); cortex_base = ioremap((*data).cortex_addr, SZ_16K); gpc_base = ioremap((*data).gpc_addr, SZ_16K);
    mx5_cpu_lp_set(IMX5_DEFAULT_CPU_IDLE_STATE);
    let _ = imx5_cpuidle_init(); let _ = imx5_suspend_init(data); suspend_set_ops(&mx5_suspend_ops); 0
}

unsafe fn imx51_pm_init() { imx5_pm_common_init(&imx51_pm_data); }
unsafe fn imx53_pm_init() { imx5_pm_common_init(&imx53_pm_data); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
