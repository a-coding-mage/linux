// SPDX-License-Identifier: GPL-2.0
/*
 * JZ47xx SoCs TCU Operating System Timer driver
 *
 * Copyright (C) 2016 Maarten ter Huurne <maarten@treewalker.org>
 * Copyright (C) 2020 Paul Cercueil <paul@crapouillou.net>
 */

// Translated from the Linux kernel implementation; referenced kernel symbols
// and types are supplied by external dependencies.

const TCU_OST_TCSR_MASK: u32 = 0xffc0;
const TCU_OST_TCSR_CNT_MD: u32 = 1 << 15;

const TCU_OST_CHANNEL: u32 = 15;

/*
 * The TCU_REG_OST_CNT{L,R} from <linux/mfd/ingenic-tcu.h> are only for the
 * regmap; these are for use with the __iomem pointer.
 */
const OST_REG_CNTL: usize = 0x4;
const OST_REG_CNTH: usize = 0x8;

#[repr(C)]
pub struct ingenic_ost_soc_info {
    pub is64bit: bool,
}

#[repr(C)]
pub struct ingenic_ost {
    pub regs: *mut core::ffi::c_void,
    pub clk: *mut clk,

    pub cs: clocksource,
}

static mut ingenic_ost: *mut ingenic_ost = core::ptr::null_mut();

extern "C" {
    type clk;
    type device;
    type platform_device;
    type regmap;

    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn device_get_match_data(dev: *mut device) -> *const core::ffi::c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn device_node_to_regmap(node: *mut core::ffi::c_void) -> *mut regmap;
    fn devm_clk_get_enabled(dev: *mut device, id: *const u8) -> *mut clk;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn clk_get_rate(clk: *mut clk) -> usize;
    fn clocksource_register_hz(cs: *mut clocksource, hz: usize) -> i32;
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: usize);
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn clk_disable(clk: *mut clk);
    fn clk_enable(clk: *mut clk) -> i32;
}

#[repr(C)]
pub struct clocksource {
    pub name: *const u8,
    pub rating: i32,
    pub flags: u32,
    pub mask: u64,
    pub read: unsafe extern "C" fn(*mut clocksource) -> u64,
}

const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1 << 2;
const TCU_REG_OST_CNTL: u32 = 0;
const TCU_REG_OST_CNTH: u32 = 0;
const TCU_REG_OST_TCSR: u32 = 0;
const TCU_REG_TESR: u32 = 0;

unsafe extern "C" fn ingenic_ost_read_cntl() -> u64 {
    /* Read using __iomem pointer instead of regmap to avoid locking */
    readl((*ingenic_ost).regs.add(OST_REG_CNTL) as *mut core::ffi::c_void) as u64
}

unsafe extern "C" fn ingenic_ost_read_cnth() -> u64 {
    /* Read using __iomem pointer instead of regmap to avoid locking */
    readl((*ingenic_ost).regs.add(OST_REG_CNTH) as *mut core::ffi::c_void) as u64
}

unsafe extern "C" fn ingenic_ost_clocksource_readl(cs: *mut clocksource) -> u64 {
    let _ = cs;
    ingenic_ost_read_cntl()
}

unsafe extern "C" fn ingenic_ost_clocksource_readh(cs: *mut clocksource) -> u64 {
    let _ = cs;
    ingenic_ost_read_cnth()
}

unsafe extern "C" fn ingenic_ost_probe(pdev: *mut platform_device) -> i32 {
    let soc_info = device_get_match_data(pdev as *mut device)
        as *const ingenic_ost_soc_info;
    if soc_info.is_null() {
        return -22;
    }

    let ost = devm_kzalloc(pdev as *mut device, core::mem::size_of::<ingenic_ost>(), 0)
        as *mut ingenic_ost;
    if ost.is_null() {
        return -12;
    }

    ingenic_ost = ost;

    (*ost).regs = devm_platform_ioremap_resource(pdev, 0);
    let map = device_node_to_regmap(core::ptr::null_mut());
    let clk = devm_clk_get_enabled(pdev as *mut device, b"ost\0".as_ptr());
    (*ost).clk = clk;

    /* Clear counter high/low registers */
    if (*soc_info).is64bit {
        regmap_write(map, TCU_REG_OST_CNTL, 0);
    }
    regmap_write(map, TCU_REG_OST_CNTH, 0);

    /* Don't reset counter at compare value. */
    regmap_update_bits(
        map,
        TCU_REG_OST_TCSR,
        TCU_OST_TCSR_MASK,
        TCU_OST_TCSR_CNT_MD,
    );

    let rate = clk_get_rate((*ost).clk);

    /* Enable OST TCU channel */
    regmap_write(map, TCU_REG_TESR, 1 << TCU_OST_CHANNEL);

    let cs = &mut (*ost).cs;
    cs.name = b"ingenic-ost\0".as_ptr();
    cs.rating = 320;
    cs.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    cs.mask = u32::MAX as u64;

    if (*soc_info).is64bit {
        cs.read = ingenic_ost_clocksource_readl;
    } else {
        cs.read = ingenic_ost_clocksource_readh;
    }

    let err = clocksource_register_hz(cs, rate);
    if err != 0 {
        return err;
    }

    if (*soc_info).is64bit {
        sched_clock_register(ingenic_ost_read_cntl, 32, rate);
    } else {
        sched_clock_register(ingenic_ost_read_cnth, 32, rate);
    }

    0
}

unsafe extern "C" fn ingenic_ost_suspend(dev: *mut device) -> i32 {
    let ost = dev_get_drvdata(dev) as *mut ingenic_ost;

    clk_disable((*ost).clk);

    0
}

unsafe extern "C" fn ingenic_ost_resume(dev: *mut device) -> i32 {
    let ost = dev_get_drvdata(dev) as *mut ingenic_ost;

    clk_enable((*ost).clk)
}

#[repr(C)]
pub struct dev_pm_ops {
    /* _noirq: We want the OST clock to be gated last / ungated first */
    pub suspend_noirq: unsafe extern "C" fn(*mut device) -> i32,
    pub resume_noirq: unsafe extern "C" fn(*mut device) -> i32,
}

static ingenic_ost_pm_ops: dev_pm_ops = dev_pm_ops {
    .suspend_noirq: ingenic_ost_suspend,
    .resume_noirq: ingenic_ost_resume,
};

static jz4725b_ost_soc_info: ingenic_ost_soc_info = ingenic_ost_soc_info {
    is64bit: false,
};

static jz4760b_ost_soc_info: ingenic_ost_soc_info = ingenic_ost_soc_info {
    is64bit: true,
};

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
    pub data: *const core::ffi::c_void,
}

static ingenic_ost_of_match: &[of_device_id] = &[
    of_device_id { compatible: b"ingenic,jz4725b-ost\0".as_ptr(), data: &jz4725b_ost_soc_info as *const _ as *const _, },
    of_device_id { compatible: b"ingenic,jz4760b-ost\0".as_ptr(), data: &jz4760b_ost_soc_info as *const _ as *const _, },
    of_device_id { compatible: b"ingenic,jz4770-ost\0".as_ptr(), data: &jz4760b_ost_soc_info as *const _ as *const _, },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null(), },
];

#[repr(C)]
pub struct platform_driver {
    pub name: *const u8,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

static ingenic_ost_driver: platform_driver = platform_driver {
    name: b"ingenic-ost\0".as_ptr(),
    pm: &ingenic_ost_pm_ops,
    of_match_table: ingenic_ost_of_match.as_ptr(),
};

extern "C" {
    fn builtin_platform_driver_probe(driver: *const platform_driver, probe: unsafe extern "C" fn(*mut platform_device) -> i32);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
