// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Framebuffer device registration for TI OMAP platforms
 *
 * Copyright (C) 2006 Nokia Corporation
 * Author: Imre Deak <imre.deak@nokia.com>
 */

// Kernel headers and local OMAP declarations are supplied by the surrounding
// translation unit.

#[cfg(CONFIG_OMAP2_VRFB)]
/*
 * The first memory resource is the register region for VRFB,
 * the rest are VRFB virtual memory areas for each VRFB context.
 */
static OMAP2_VRFB_RESOURCES: [struct_resource; 5] = [
    DEFINE_RES_MEM_NAMED!(0x68008000u32, 0x40, "vrfb-regs"),
    DEFINE_RES_MEM_NAMED!(0x70000000u32, 0x4000000, "vrfb-area-0"),
    DEFINE_RES_MEM_NAMED!(0x74000000u32, 0x4000000, "vrfb-area-1"),
    DEFINE_RES_MEM_NAMED!(0x78000000u32, 0x4000000, "vrfb-area-2"),
    DEFINE_RES_MEM_NAMED!(0x7c000000u32, 0x4000000, "vrfb-area-3"),
];

#[cfg(CONFIG_OMAP2_VRFB)]
static OMAP3_VRFB_RESOURCES: [struct_resource; 12] = [
    DEFINE_RES_MEM_NAMED!(0x6C000180u32, 0xc0, "vrfb-regs"),
    DEFINE_RES_MEM_NAMED!(0x70000000u32, 0x4000000, "vrfb-area-0"),
    DEFINE_RES_MEM_NAMED!(0x74000000u32, 0x4000000, "vrfb-area-1"),
    DEFINE_RES_MEM_NAMED!(0x78000000u32, 0x4000000, "vrfb-area-2"),
    DEFINE_RES_MEM_NAMED!(0x7c000000u32, 0x4000000, "vrfb-area-3"),
    DEFINE_RES_MEM_NAMED!(0xe0000000u32, 0x4000000, "vrfb-area-4"),
    DEFINE_RES_MEM_NAMED!(0xe4000000u32, 0x4000000, "vrfb-area-5"),
    DEFINE_RES_MEM_NAMED!(0xe8000000u32, 0x4000000, "vrfb-area-6"),
    DEFINE_RES_MEM_NAMED!(0xec000000u32, 0x4000000, "vrfb-area-7"),
    DEFINE_RES_MEM_NAMED!(0xf0000000u32, 0x4000000, "vrfb-area-8"),
    DEFINE_RES_MEM_NAMED!(0xf4000000u32, 0x4000000, "vrfb-area-9"),
    DEFINE_RES_MEM_NAMED!(0xf8000000u32, 0x4000000, "vrfb-area-10"),
    DEFINE_RES_MEM_NAMED!(0xfc000000u32, 0x4000000, "vrfb-area-11"),
];

#[cfg(CONFIG_OMAP2_VRFB)]
pub unsafe extern "C" fn omap_init_vrfb() -> i32 {
    let mut res: *const struct_resource;
    let mut num_res: u32;

    if cpu_is_omap24xx() {
        res = OMAP2_VRFB_RESOURCES.as_ptr();
        num_res = OMAP2_VRFB_RESOURCES.len() as u32;
    } else if cpu_is_omap34xx() {
        res = OMAP3_VRFB_RESOURCES.as_ptr();
        num_res = OMAP3_VRFB_RESOURCES.len() as u32;
    } else {
        return 0;
    }

    let pdev = platform_device_register_resndata(
        core::ptr::null_mut(),
        "omapvrfb".as_ptr() as *const i8,
        -1,
        res,
        num_res,
        core::ptr::null(),
        0,
    );

    PTR_ERR_OR_ZERO(pdev)
}

#[cfg(not(CONFIG_OMAP2_VRFB))]
pub unsafe extern "C" fn omap_init_vrfb() -> i32 { 0 }

#[cfg(IS_ENABLED_CONFIG_FB_OMAP2)]
static mut OMAP_FB_DMA_MASK: u64 = !(0u32) as u64;

#[cfg(IS_ENABLED_CONFIG_FB_OMAP2)]
static mut OMAPFB_CONFIG: omapfb_platform_data = omapfb_platform_data_init!();

#[cfg(IS_ENABLED_CONFIG_FB_OMAP2)]
static mut OMAP_FB_DEVICE: platform_device = platform_device {
    name: "omapfb".as_ptr() as *const i8,
    id: -1,
    dev: device {
        dma_mask: core::ptr::addr_of_mut!(OMAP_FB_DMA_MASK),
        coherent_dma_mask: DMA_BIT_MASK!(32),
        platform_data: core::ptr::addr_of_mut!(OMAPFB_CONFIG) as *mut _,
    },
    num_resources: 0,
};

#[cfg(IS_ENABLED_CONFIG_FB_OMAP2)]
pub unsafe extern "C" fn omap_init_fb() -> i32 {
    platform_device_register(core::ptr::addr_of_mut!(OMAP_FB_DEVICE))
}

#[cfg(not(IS_ENABLED_CONFIG_FB_OMAP2))]
pub unsafe extern "C" fn omap_init_fb() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
