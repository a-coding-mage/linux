// SPDX-License-Identifier: GPL-2.0-only
/*
 * (HiSilicon's SoC based) flattened device tree enabled machine
 *
 * Copyright (c) 2012-2013 HiSilicon Ltd.
 * Copyright (c) 2012-2013 Linaro Ltd.
 *
 * Author: Haojian Zhuang <haojian.zhuang@linaro.org>
 */

// C dependencies: <linux/clocksource.h>, <linux/irqchip.h>,
// <asm/mach/arch.h>, and <asm/mach/map.h>.

const HI3620_SYSCTRL_PHYS_BASE: usize = 0xfc802000;
const HI3620_SYSCTRL_VIRT_BASE: usize = 0xfe802000;

#[repr(C)]
pub struct MapDesc {
    pub pfn: usize,
    pub virtual_address: usize,
    pub length: usize,
    pub type_: usize,
}

extern "C" {
    fn debug_ll_io_init();
    fn iotable_init(io_desc: *mut MapDesc, nr: usize);
    fn __phys_to_pfn(phys: usize) -> usize;
}

// MT_DEVICE is supplied by the architecture's map definitions.
extern "C" {
    static MT_DEVICE: usize;
}

/*
 * This table is only for optimization. Since ioremap() could always share
 * the same mapping if it's defined as static IO mapping.
 *
 * Without this table, system could also work. The cost is some virtual address
 * spaces wasted since ioremap() may be called multi times for the same
 * IO space.
 */
#[used]
static mut hi3620_io_desc: [MapDesc; 1] = [MapDesc {
    // sysctrl
    pfn: 0,
    virtual_address: HI3620_SYSCTRL_VIRT_BASE,
    length: 0x1000,
    type_: 0,
}];

#[inline]
unsafe fn init_hi3620_io_desc() {
    hi3620_io_desc[0].pfn = __phys_to_pfn(HI3620_SYSCTRL_PHYS_BASE);
    hi3620_io_desc[0].type_ = MT_DEVICE;
}

unsafe fn hi3620_map_io() {
    init_hi3620_io_desc();
    debug_ll_io_init();
    iotable_init(hi3620_io_desc.as_mut_ptr(), hi3620_io_desc.len());
}

static hi3xxx_compat: [*const u8; 2] = [
    b"hisilicon,hi3620-hi4511\0".as_ptr(),
    core::ptr::null(),
];

// Equivalent machine descriptor for DT_MACHINE_START(HI3620, ...).
#[repr(C)]
pub struct MachineDesc {
    pub name: *const u8,
    pub map_io: unsafe fn(),
    pub dt_compat: *const *const u8,
}

#[used]
static HI3620: MachineDesc = MachineDesc {
    name: b"Hisilicon Hi3620 (Flattened Device Tree)\0".as_ptr(),
    map_io: hi3620_map_io,
    dt_compat: hi3xxx_compat.as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
