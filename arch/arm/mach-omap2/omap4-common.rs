// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP4 specific common source file.
 *
 * Copyright (C) 2010 Texas Instruments, Inc.
 * Author:
 *	Santosh Shilimkar <santosh.shilimkar@ti.com>
 */

// C includes and symbols supplied by the surrounding kernel are external dependencies.

#[cfg(CONFIG_CACHE_L2X0)]
static mut L2CACHE_BASE: *mut core::ffi::c_void = core::ptr::null_mut();
static mut SAR_RAM_BASE: *mut core::ffi::c_void = core::ptr::null_mut();
static mut GIC_DIST_BASE_ADDR: *mut u8 = core::ptr::null_mut();
static mut TWD_BASE: *mut u8 = core::ptr::null_mut();

const IRQ_LOCALTIMER: u32 = 29;

#[cfg(CONFIG_OMAP_INTERCONNECT_BARRIER)]
const OMAP4_DRAM_BARRIER_VA: usize = 0xfe600000;

#[cfg(CONFIG_OMAP_INTERCONNECT_BARRIER)]
static mut DRAM_SYNC: *mut u8 = core::ptr::null_mut();
#[cfg(CONFIG_OMAP_INTERCONNECT_BARRIER)]
static mut SRAM_SYNC: *mut u8 = core::ptr::null_mut();
#[cfg(CONFIG_OMAP_INTERCONNECT_BARRIER)]
static mut DRAM_SYNC_PADDR: phys_addr_t = 0;
#[cfg(CONFIG_OMAP_INTERCONNECT_BARRIER)]
static mut DRAM_SYNC_SIZE: u32 = 0;

#[cfg(CONFIG_OMAP_INTERCONNECT_BARRIER)]
unsafe fn omap4_mb() {
    if !DRAM_SYNC.is_null() {
        writel_relaxed(0, DRAM_SYNC);
    }
}

#[cfg(CONFIG_OMAP_INTERCONNECT_BARRIER)]
pub unsafe fn omap_interconnect_sync() {
    if !DRAM_SYNC.is_null() && !SRAM_SYNC.is_null() {
        writel_relaxed(readl_relaxed(DRAM_SYNC), DRAM_SYNC);
        writel_relaxed(readl_relaxed(SRAM_SYNC), SRAM_SYNC);
        isb();
    }
}

#[cfg(CONFIG_OMAP_INTERCONNECT_BARRIER)]
unsafe fn omap4_sram_init() -> i32 {
    let mut np: *mut device_node;
    let sram_pool: *mut gen_pool;

    if !soc_is_omap44xx() && !soc_is_omap54xx() {
        return 0;
    }
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"ti,omap4-mpu".as_ptr());
    if np.is_null() {
        pr_warn!("{}:Unable to allocate sram needed to handle errata I688\n", __func__);
    }
    sram_pool = of_gen_pool_get(np, c"sram".as_ptr(), 0);
    if sram_pool.is_null() {
        pr_warn!("{}:Unable to get sram pool needed to handle errata I688\n", __func__);
    } else {
        SRAM_SYNC = gen_pool_alloc(sram_pool, PAGE_SIZE) as *mut u8;
    }
    of_node_put(np);
    0
}

#[cfg(CONFIG_OMAP_INTERCONNECT_BARRIER)]
pub unsafe fn omap_barrier_reserve_memblock() {
    DRAM_SYNC_SIZE = ALIGN(PAGE_SIZE, SZ_1M);
    DRAM_SYNC_PADDR = arm_memblock_steal(DRAM_SYNC_SIZE, SZ_1M);
}

#[cfg(CONFIG_OMAP_INTERCONNECT_BARRIER)]
pub unsafe fn omap_barriers_init() {
    let mut dram_io_desc: [map_desc; 1] = core::mem::zeroed();
    dram_io_desc[0].virtual_ = OMAP4_DRAM_BARRIER_VA;
    dram_io_desc[0].pfn = __phys_to_pfn(DRAM_SYNC_PADDR);
    dram_io_desc[0].length = DRAM_SYNC_SIZE as usize;
    dram_io_desc[0].type_ = MT_MEMORY_RW_SO;
    iotable_init(dram_io_desc.as_mut_ptr(), ARRAY_SIZE(&dram_io_desc));
    DRAM_SYNC = dram_io_desc[0].virtual_ as *mut u8;
    pr_info!("OMAP4: Map %pa to %p for dram barrier\n", &DRAM_SYNC_PADDR, DRAM_SYNC);
    soc_mb = Some(omap4_mb);
}

pub unsafe fn gic_dist_disable() {
    if !GIC_DIST_BASE_ADDR.is_null() {
        writel_relaxed(0x0, GIC_DIST_BASE_ADDR.add(GIC_DIST_CTRL as usize));
    }
}

pub unsafe fn gic_dist_enable() {
    if !GIC_DIST_BASE_ADDR.is_null() {
        writel_relaxed(0x1, GIC_DIST_BASE_ADDR.add(GIC_DIST_CTRL as usize));
    }
}

pub unsafe fn gic_dist_disabled() -> bool {
    (readl_relaxed(GIC_DIST_BASE_ADDR.add(GIC_DIST_CTRL as usize)) & 0x1) == 0
}

pub unsafe fn gic_timer_retrigger() {
    let twd_int = readl_relaxed(TWD_BASE.add(TWD_TIMER_INTSTAT as usize));
    let gic_int = readl_relaxed(GIC_DIST_BASE_ADDR.add(GIC_DIST_PENDING_SET as usize));
    let mut twd_ctrl = readl_relaxed(TWD_BASE.add(TWD_TIMER_CONTROL as usize));

    if twd_int != 0 && (gic_int & BIT(IRQ_LOCALTIMER)) == 0 {
        /* The local timer interrupt got lost while the distributor was disabled. */
        pr_warn!("{}: lost localtimer interrupt\n", __func__);
        writel_relaxed(1, TWD_BASE.add(TWD_TIMER_INTSTAT as usize));
        if (twd_ctrl & TWD_TIMER_CONTROL_PERIODIC) == 0 {
            writel_relaxed(1, TWD_BASE.add(TWD_TIMER_COUNTER as usize));
            twd_ctrl |= TWD_TIMER_CONTROL_ENABLE;
            writel_relaxed(twd_ctrl, TWD_BASE.add(TWD_TIMER_CONTROL as usize));
        }
    }
}

#[cfg(CONFIG_CACHE_L2X0)]
pub unsafe fn omap4_get_l2cache_base() -> *mut core::ffi::c_void { L2CACHE_BASE }

#[cfg(CONFIG_CACHE_L2X0)]
pub unsafe fn omap4_l2c310_write_sec(val: c_ulong, reg: c_uint) {
    let smc_op: c_uint;
    match reg {
        L2X0_CTRL => smc_op = OMAP4_MON_L2X0_CTRL_INDEX,
        L2X0_AUX_CTRL => smc_op = OMAP4_MON_L2X0_AUXCTRL_INDEX,
        L2X0_DEBUG_CTRL => smc_op = OMAP4_MON_L2X0_DBG_CTRL_INDEX,
        L310_PREFETCH_CTRL => smc_op = OMAP4_MON_L2X0_PREFETCH_INDEX,
        L310_POWER_CTRL => { pr_info_once!("OMAP L2C310: ROM does not support power control setting\n"); return; }
        _ => { WARN_ONCE!(true, "OMAP L2C310: ignoring write to reg 0x%x\n", reg); return; }
    }
    omap_smc1(smc_op, val);
}

#[cfg(CONFIG_CACHE_L2X0)]
pub unsafe fn omap_l2_cache_init() -> i32 {
    L2CACHE_BASE = ioremap(OMAP44XX_L2CACHE_BASE, SZ_4K);
    if WARN_ON(L2CACHE_BASE.is_null()) { return -ENOMEM; }
    0
}

pub unsafe fn omap4_get_sar_ram_base() -> *mut core::ffi::c_void { SAR_RAM_BASE }

pub unsafe fn omap4_sar_ram_init() {
    let sar_base: c_ulong;
    if cpu_is_omap44xx() { sar_base = OMAP44XX_SAR_RAM_BASE; }
    else if soc_is_omap54xx() { sar_base = OMAP54XX_SAR_RAM_BASE; }
    else { return; }
    SAR_RAM_BASE = ioremap(sar_base, SZ_16K);
    if WARN_ON(SAR_RAM_BASE.is_null()) { return; }
}

static mut INTC_MATCH: [of_device_id; 3] = [
    of_device_id { compatible: c"ti,omap4-wugen-mpu".as_ptr() },
    of_device_id { compatible: c"ti,omap5-wugen-mpu".as_ptr() },
    of_device_id::default(),
];
static mut INTC_NODE: *mut device_node = core::ptr::null_mut();

pub unsafe fn omap_gic_of_init() {
    let mut np: *mut device_node;
    INTC_NODE = of_find_matching_node(core::ptr::null_mut(), INTC_MATCH.as_ptr());
    if WARN_ON(INTC_NODE.is_null()) {
        pr_err!("No WUGEN found in DT, system will misbehave.\n");
        pr_err!("UPDATE YOUR DEVICE TREE!\n");
    }
    if !cpu_is_omap446x() { irqchip_init(); return; }
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"arm,cortex-a9-gic".as_ptr());
    GIC_DIST_BASE_ADDR = of_iomap(np, 0);
    of_node_put(np);
    WARN_ON(GIC_DIST_BASE_ADDR.is_null());
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"arm,cortex-a9-twd-timer".as_ptr());
    TWD_BASE = of_iomap(np, 0);
    of_node_put(np);
    WARN_ON(TWD_BASE.is_null());
    irqchip_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
