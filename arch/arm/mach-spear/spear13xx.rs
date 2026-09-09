// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-spear13xx/spear13xx.c
 *
 * SPEAr13XX machines common source file
 *
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

// C preprocessor dependency intent: pr_fmt(fmt) = "SPEAr13xx: " fmt.

extern "C" {
    fn writel_relaxed(value: u32, address: usize);
    fn l2x0_init(base: usize, tag_latency: u32, data_latency: u32);
    fn iotable_init(desc: *mut map_desc, size: usize);
    fn of_machine_is_compatible(compatible: *const u8) -> bool;
    fn spear1310_clk_init(misc_base: usize, ras_base: usize);
    fn spear1340_clk_init(misc_base: usize);
    fn clk_get_sys(dev_id: *const u8, con_id: *const u8) -> *mut clk;
    fn clk_get(dev_id: *const u8, con_id: *const u8) -> *mut clk;
    fn is_err(ptr: *mut clk) -> bool;
    fn clk_set_parent(child: *mut clk, parent: *mut clk) -> i32;
    fn clk_put(clk: *mut clk);
    fn spear_setup_of_timer();
    fn timer_probe();
    fn bug();
}

// External definitions supplied by the architecture headers.
#[repr(C)]
pub struct map_desc {
    pub virtual_: usize,
    pub pfn: usize,
    pub length: usize,
    pub type_: u32,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

extern "C" {
    static VA_L2CC_BASE: usize;
    static VA_PERIP_GRP2_BASE: usize;
    static VA_PERIP_GRP1_BASE: usize;
    static VA_A9SM_AND_MPMC_BASE: usize;
    static VA_MISC_BASE: usize;
    static VA_SPEAR1310_RAS_BASE: usize;
    static L2CC_BASE: usize;
    static PERIP_GRP2_BASE: usize;
    static PERIP_GRP1_BASE: usize;
    static A9SM_AND_MPMC_BASE: usize;
    static L310_PREFETCH_CTRL: usize;
    static L310_TAG_LATENCY_CTRL: usize;
    static L310_DATA_LATENCY_CTRL: usize;
    static SZ_16M: usize;
    static SZ_4K: usize;
    static MT_DEVICE: u32;
}

/*
 * Following will create 16MB static virtual/physical mappings
 * PHYSICAL                VIRTUAL
 * 0xB3000000              0xF9000000
 * 0xE0000000              0xFD000000
 * 0xEC000000              0xFC000000
 * 0xED000000              0xFB000000
 */
static mut spear13xx_io_desc: [map_desc; 4] = [
    map_desc {
        virtual_: 0, // VA_PERIP_GRP2_BASE
        pfn: 0, // __phys_to_pfn(PERIP_GRP2_BASE)
        length: 0, // SZ_16M
        type_: 0, // MT_DEVICE
    },
    map_desc {
        virtual_: 0, // VA_PERIP_GRP1_BASE
        pfn: 0, // __phys_to_pfn(PERIP_GRP1_BASE)
        length: 0, // SZ_16M
        type_: 0, // MT_DEVICE
    },
    map_desc {
        virtual_: 0, // VA_A9SM_AND_MPMC_BASE
        pfn: 0, // __phys_to_pfn(A9SM_AND_MPMC_BASE)
        length: 0, // SZ_16M
        type_: 0, // MT_DEVICE
    },
    map_desc {
        virtual_: 0, // VA_L2CC_BASE
        pfn: 0, // __phys_to_pfn(L2CC_BASE)
        length: 0, // SZ_4K
        type_: 0, // MT_DEVICE
    },
];

pub unsafe fn spear13xx_l2x0_init() {
    // IS_ENABLED(CONFIG_CACHE_L2X0) is a build-time kernel configuration condition.
    if !cfg!(feature = "CONFIG_CACHE_L2X0") {
        return;
    }

    writel_relaxed(0x06, VA_L2CC_BASE + L310_PREFETCH_CTRL);

    /*
     * Program following latencies in order to make
     * SPEAr1340 work at 600 MHz
     */
    writel_relaxed(0x221, VA_L2CC_BASE + L310_TAG_LATENCY_CTRL);
    writel_relaxed(0x441, VA_L2CC_BASE + L310_DATA_LATENCY_CTRL);
    l2x0_init(VA_L2CC_BASE, 0x30a00001, 0xfe0fffff);
}

/* This will create static memory mapping for selected devices */
pub unsafe fn spear13xx_map_io() {
    iotable_init(spear13xx_io_desc.as_mut_ptr(), spear13xx_io_desc.len());
}

unsafe fn spear13xx_clk_init() {
    if of_machine_is_compatible(b"st,spear1310\0".as_ptr()) {
        spear1310_clk_init(VA_MISC_BASE, VA_SPEAR1310_RAS_BASE);
    } else if of_machine_is_compatible(b"st,spear1340\0".as_ptr()) {
        spear1340_clk_init(VA_MISC_BASE);
    } else {
        // pr_err("%s: Unknown machine\n", __func__);
    }
}

pub unsafe fn spear13xx_timer_init() {
    let pclk_name = *b"osc_24m_clk\0";
    let mut gpt_clk: *mut clk;
    let mut pclk: *mut clk;

    spear13xx_clk_init();

    /* get the system timer clock */
    gpt_clk = clk_get_sys(b"gpt0\0".as_ptr(), core::ptr::null());
    if is_err(gpt_clk) {
        // pr_err("%s:couldn't get clk for gpt\n", __func__);
        bug();
    }

    /* get the suitable parent clock for timer*/
    pclk = clk_get(core::ptr::null(), pclk_name.as_ptr());
    if is_err(pclk) {
        // pr_err("%s:couldn't get %s as parent for gpt\n", __func__, pclk_name);
        bug();
    }

    clk_set_parent(gpt_clk, pclk);
    clk_put(gpt_clk);
    clk_put(pclk);

    spear_setup_of_timer();
    timer_probe();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
