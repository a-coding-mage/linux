// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP2+ common Clock Management (CM) IP block functions
 *
 * Copyright (C) 2012 Texas Instruments, Inc.
 * Paul Walmsley
 *
 * XXX This code should eventually be moved to a CM driver.
 */

// Linux and SoC header dependencies are supplied by the surrounding tree.

/* cm_ll_data: function pointers to SoC-specific implementations of
 * common CM functions
 */
static mut NULL_CM_LL_DATA: cm_ll_data = cm_ll_data::default();
static mut CM_LL_DATA: *const cm_ll_data = &raw const NULL_CM_LL_DATA;

/* cm_base: base virtual address of the CM IP block */
pub static mut cm_base: omap_domain_base = omap_domain_base::default();

/* cm2_base: base virtual address of the CM2 IP block (OMAP44xx only) */
pub static mut cm2_base: omap_domain_base = omap_domain_base::default();

const CM_NO_CLOCKS: u32 = 0x1;
const CM_SINGLE_INSTANCE: u32 = 0x2;

pub unsafe fn cm_split_idlest_reg(
    idlest_reg: *mut clk_omap_reg,
    prcm_inst: *mut i16,
    idlest_reg_id: *mut u8,
) -> i32 {
    let data = &*CM_LL_DATA;
    let f = match data.split_idlest_reg {
        Some(f) => f,
        None => {
            WARN_ONCE!(1, "cm: cm_split_idlest_reg: no low-level function defined\n");
            return -EINVAL;
        }
    };
    let ret = f(idlest_reg, prcm_inst, idlest_reg_id);
    *prcm_inst -= cm_base.offset;
    ret
}

pub unsafe fn omap_cm_wait_module_ready(
    part: u8, prcm_mod: i16, idlest_reg: u16, idlest_shift: u8,
) -> i32 {
    let data = &*CM_LL_DATA;
    let f = match data.wait_module_ready {
        Some(f) => f,
        None => {
            WARN_ONCE!(1, "cm: omap_cm_wait_module_ready: no low-level function defined\n");
            return -EINVAL;
        }
    };
    f(part, prcm_mod, idlest_reg, idlest_shift)
}

pub unsafe fn omap_cm_wait_module_idle(
    part: u8, prcm_mod: i16, idlest_reg: u16, idlest_shift: u8,
) -> i32 {
    let data = &*CM_LL_DATA;
    let f = match data.wait_module_idle {
        Some(f) => f,
        None => {
            WARN_ONCE!(1, "cm: omap_cm_wait_module_idle: no low-level function defined\n");
            return -EINVAL;
        }
    };
    f(part, prcm_mod, idlest_reg, idlest_shift)
}

pub unsafe fn omap_cm_module_enable(mode: u8, part: u8, inst: u16, clkctrl_offs: u16) -> i32 {
    let data = &*CM_LL_DATA;
    let f = match data.module_enable {
        Some(f) => f,
        None => {
            WARN_ONCE!(1, "cm: omap_cm_module_enable: no low-level function defined\n");
            return -EINVAL;
        }
    };
    f(mode, part, inst, clkctrl_offs);
    0
}

pub unsafe fn omap_cm_module_disable(part: u8, inst: u16, clkctrl_offs: u16) -> i32 {
    let data = &*CM_LL_DATA;
    let f = match data.module_disable {
        Some(f) => f,
        None => {
            WARN_ONCE!(1, "cm: omap_cm_module_disable: no low-level function defined\n");
            return -EINVAL;
        }
    };
    f(part, inst, clkctrl_offs);
    0
}

pub unsafe fn omap_cm_xlate_clkctrl(part: u8, inst: u16, clkctrl_offs: u16) -> u32 {
    let data = &*CM_LL_DATA;
    let f = match data.xlate_clkctrl {
        Some(f) => f,
        None => {
            WARN_ONCE!(1, "cm: omap_cm_xlate_clkctrl: no low-level function defined\n");
            return 0;
        }
    };
    f(part, inst, clkctrl_offs)
}

pub unsafe fn cm_register(cld: *const cm_ll_data) -> i32 {
    if cld.is_null() { return -EINVAL; }
    if CM_LL_DATA != &raw const NULL_CM_LL_DATA { return -EEXIST; }
    CM_LL_DATA = cld;
    0
}

pub unsafe fn cm_unregister(cld: *const cm_ll_data) -> i32 {
    if cld.is_null() || CM_LL_DATA != cld { return -EINVAL; }
    CM_LL_DATA = &raw const NULL_CM_LL_DATA;
    0
}

#[cfg(any(CONFIG_ARCH_OMAP4, CONFIG_SOC_OMAP5, CONFIG_SOC_DRA7XX))]
static mut cm_data: omap_prcm_init_data = omap_prcm_init_data {
    index: TI_CLKM_CM,
    init: Some(omap4_cm_init),
    ..omap_prcm_init_data::default()
};
#[cfg(any(CONFIG_ARCH_OMAP4, CONFIG_SOC_OMAP5, CONFIG_SOC_DRA7XX))]
static mut cm2_data: omap_prcm_init_data = omap_prcm_init_data {
    index: TI_CLKM_CM2,
    init: Some(omap4_cm_init),
    ..omap_prcm_init_data::default()
};
#[cfg(CONFIG_ARCH_OMAP2)]
static mut omap2_prcm_data: omap_prcm_init_data = omap_prcm_init_data {
    index: TI_CLKM_CM,
    init: Some(omap2xxx_cm_init),
    flags: CM_NO_CLOCKS | CM_SINGLE_INSTANCE,
    ..omap_prcm_init_data::default()
};
#[cfg(CONFIG_ARCH_OMAP3)]
static mut omap3_cm_data: omap_prcm_init_data = omap_prcm_init_data {
    index: TI_CLKM_CM,
    init: Some(omap3xxx_cm_init),
    flags: CM_SINGLE_INSTANCE,
    offset: -OMAP3430_IVA2_MOD,
    ..omap_prcm_init_data::default()
};
#[cfg(any(CONFIG_SOC_AM33XX, CONFIG_SOC_TI81XX))]
static mut am3_prcm_data: omap_prcm_init_data = omap_prcm_init_data {
    index: TI_CLKM_CM,
    flags: CM_NO_CLOCKS | CM_SINGLE_INSTANCE,
    init: Some(am33xx_cm_init),
    ..omap_prcm_init_data::default()
};
#[cfg(CONFIG_SOC_AM43XX)]
static mut am4_prcm_data: omap_prcm_init_data = omap_prcm_init_data {
    index: TI_CLKM_CM,
    flags: CM_NO_CLOCKS | CM_SINGLE_INSTANCE,
    init: Some(omap4_cm_init),
    ..omap_prcm_init_data::default()
};

// The device-tree match table contains the CONFIG_* gated entries from the C
// source; its bindings and linker-visible initialization are external here.

pub unsafe fn omap2_cm_base_init() -> i32 {
    // for_each_matching_node_and_match(): device-tree iteration and resource
    // mapping are provided by the surrounding kernel bindings.
    todo!("translate kernel device-tree iteration and iomapping dependencies")
}

pub unsafe fn omap_cm_init() -> i32 {
    // for_each_matching_node_and_match(): device-tree iteration is supplied
    // by the surrounding kernel bindings.
    todo!("translate kernel device-tree clock-provider dependencies")
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
