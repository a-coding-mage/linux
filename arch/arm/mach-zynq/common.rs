// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file contains common code that is intended to be used across
 * boards so that it's not replicated.
 *
 * Copyright (C) 2011 Xilinx
 */

// C dependencies supplied by the surrounding kernel translation.

const ZYNQ_DEVCFG_MCTRL: usize = 0x80;
const ZYNQ_DEVCFG_PS_VERSION_SHIFT: u32 = 28;
const ZYNQ_DEVCFG_PS_VERSION_MASK: u32 = 0xF;

static mut zynq_scu_base: *mut core::ffi::c_void = core::ptr::null_mut();

/// zynq_memory_init - Initialize special memory
///
/// We need to stop things allocating the low memory as DMA can't work in
/// the 1st 512K of memory.
unsafe fn zynq_memory_init() {
    if __pa(PAGE_OFFSET) == 0 {
        memblock_reserve(__pa(PAGE_OFFSET), 0x80000);
    }
}

static mut zynq_cpuidle_device: platform_device = platform_device {
    name: "cpuidle-zynq",
};

/// zynq_get_revision - Get Zynq silicon revision
///
/// Return: Silicon version or -1 otherwise
unsafe fn zynq_get_revision() -> i32 {
    let np: *mut device_node = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        "xlnx,zynq-devcfg-1.0",
    );
    if np.is_null() {
        pr_err("%s: no devcfg node found\n", "zynq_get_revision");
        return -1;
    }

    let zynq_devcfg_base = of_iomap(np, 0);
    of_node_put(np);
    if zynq_devcfg_base.is_null() {
        pr_err("%s: Unable to map I/O memory\n", "zynq_get_revision");
        return -1;
    }

    let mut revision: u32 = readl(zynq_devcfg_base.add(ZYNQ_DEVCFG_MCTRL));
    revision >>= ZYNQ_DEVCFG_PS_VERSION_SHIFT;
    revision &= ZYNQ_DEVCFG_PS_VERSION_MASK;

    iounmap(zynq_devcfg_base);
    revision as i32
}

unsafe fn zynq_init_late() {
    zynq_core_pm_init();
    zynq_pm_late_init();
}

/// zynq_init_machine - System specific initialization, intended to be
/// called from board specific initialization.
unsafe fn zynq_init_machine() {
    let mut soc_dev_attr: *mut soc_device_attribute = kzalloc_obj();
    let mut parent: *mut device = core::ptr::null_mut();
    if soc_dev_attr.is_null() {
        of_platform_default_populate(core::ptr::null_mut(), core::ptr::null_mut(), parent);
        platform_device_register(&raw mut zynq_cpuidle_device);
        return;
    }

    system_rev = zynq_get_revision();
    (*soc_dev_attr).family = kasprintf(GFP_KERNEL, "Xilinx Zynq");
    (*soc_dev_attr).revision = kasprintf(GFP_KERNEL, "0x%x", system_rev);
    (*soc_dev_attr).soc_id = kasprintf(GFP_KERNEL, "0x%x", zynq_slcr_get_device_id());

    let soc_dev = soc_device_register(soc_dev_attr);
    if IS_ERR(soc_dev) {
        kfree((*soc_dev_attr).family);
        kfree((*soc_dev_attr).revision);
        kfree((*soc_dev_attr).soc_id);
        kfree(soc_dev_attr);
    } else {
        parent = soc_device_to_device(soc_dev);
    }

    // Finished with the static registrations now; fill in the missing devices.
    of_platform_default_populate(core::ptr::null_mut(), core::ptr::null_mut(), parent);
    platform_device_register(&raw mut zynq_cpuidle_device);
}

unsafe fn zynq_timer_init() {
    zynq_clock_init();
    of_clk_init(core::ptr::null_mut());
    timer_probe();
}

static mut zynq_cortex_a9_scu_map: map_desc = map_desc {
    length: SZ_256,
    type_: MT_DEVICE,
    pfn: 0,
    virtual_: 0,
};

unsafe fn zynq_scu_map_io() {
    let base: usize = scu_a9_get_base();
    zynq_cortex_a9_scu_map.pfn = __phys_to_pfn(base);
    // Expected address is in vmalloc area that's why simple assign here.
    zynq_cortex_a9_scu_map.virtual_ = base;
    iotable_init(&raw mut zynq_cortex_a9_scu_map, 1);
    zynq_scu_base = base as *mut core::ffi::c_void;
    BUG_ON(zynq_scu_base.is_null());
}

/// zynq_map_io - Create memory mappings needed for early I/O.
unsafe fn zynq_map_io() {
    debug_ll_io_init();
    zynq_scu_map_io();
}

unsafe fn zynq_irq_init() {
    zynq_early_slcr_init();
    irqchip_init();
}

static zynq_dt_match: [*const core::ffi::c_char; 2] = [
    c"xlnx,zynq-7000".as_ptr(),
    core::ptr::null(),
];

// DT_MACHINE_START(XILINX_EP107, "Xilinx Zynq Platform")
// 64KB way size, 8-way associativity, parity disabled.
static XILINX_EP107: machine_desc = machine_desc {
    l2c_aux_val: 0x00400000,
    l2c_aux_mask: 0xffbfffff,
    smp: smp_ops(zynq_smp_ops),
    map_io: Some(zynq_map_io),
    init_irq: Some(zynq_irq_init),
    init_machine: Some(zynq_init_machine),
    init_late: Some(zynq_init_late),
    init_time: Some(zynq_timer_init),
    dt_compat: zynq_dt_match.as_ptr(),
    reserve: Some(zynq_memory_init),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
