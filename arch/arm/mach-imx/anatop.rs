// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2013-2015 Freescale Semiconductor, Inc.
 * Copyright 2017-2018 NXP.
 */

// Dependency intent preserved from the C translation unit:
// linux/err.h, linux/io.h, linux/of.h, linux/of_address.h,
// linux/mfd/syscon.h, linux/regmap.h, common.h, and hardware.h.

use core::ffi::c_void;

#[repr(C)]
pub struct Regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

type U32 = u32;

const REG_SET: U32 = 0x4;
const REG_CLR: U32 = 0x8;

const ANADIG_REG_2P5: U32 = 0x130;
const ANADIG_REG_CORE: U32 = 0x140;
const ANADIG_ANA_MISC0: U32 = 0x150;
const ANADIG_DIGPROG: u16 = 0x260;
const ANADIG_DIGPROG_IMX6SL: u16 = 0x280;
const ANADIG_DIGPROG_IMX7D: u16 = 0x800;

const SRC_SBMR2: U32 = 0x1c;

const BM_ANADIG_REG_2P5_ENABLE_WEAK_LINREG: U32 = 0x40000;
const BM_ANADIG_REG_2P5_ENABLE_PULLDOWN: U32 = 0x8;
const BM_ANADIG_REG_CORE_FET_ODRIVE: U32 = 0x20000000;
const BM_ANADIG_ANA_MISC0_STOP_MODE_CONFIG: U32 = 0x1000;
/* Below MISC0_DISCON_HIGH_SNVS is only for i.MX6SL */
const BM_ANADIG_ANA_MISC0_DISCON_HIGH_SNVS: U32 = 0x2000;

static mut anatop: *mut Regmap = core::ptr::null_mut();

extern "C" {
    fn regmap_read(map: *mut Regmap, reg: U32, val: *mut U32) -> c_void;
    fn regmap_write(map: *mut Regmap, reg: U32, val: U32) -> c_void;
    fn imx_mmdc_get_ddr_type() -> i32;
    fn cpu_is_imx6sl() -> bool;
    fn of_find_compatible_node(from: *mut DeviceNode, type_: *const u8, compatible: *const u8) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: i32) -> *mut c_void;
    fn of_device_is_compatible(node: *mut DeviceNode, compatible: *const u8) -> bool;
    fn of_node_put(node: *mut DeviceNode);
    fn readl_relaxed(addr: *const c_void) -> U32;
    fn iounmap(addr: *mut c_void);
    fn syscon_regmap_lookup_by_compatible(compatible: *const u8) -> *mut Regmap;
    fn mxc_set_cpu_type(cpu_type: U32);
    fn imx_set_soc_revision(revision: u32);
}

const IMX_DDR_TYPE_LPDDR2: i32 = 0; // Supplied by hardware.h.
const MXC_CPU_IMX6ULL: U32 = 0; // Supplied by hardware.h.
const MXC_CPU_IMX6ULZ: U32 = 0; // Supplied by hardware.h.

unsafe fn imx_anatop_enable_weak2p5(enable: bool) {
    let mut reg: U32;
    let mut val: U32 = 0;

    regmap_read(anatop, ANADIG_ANA_MISC0, &mut val);

    /* can only be enabled when stop_mode_config is clear. */
    reg = ANADIG_REG_2P5;
    reg += if enable && (val & BM_ANADIG_ANA_MISC0_STOP_MODE_CONFIG) == 0 {
        REG_SET
    } else {
        REG_CLR
    };
    regmap_write(anatop, reg, BM_ANADIG_REG_2P5_ENABLE_WEAK_LINREG);
}

unsafe fn imx_anatop_enable_fet_odrive(enable: bool) {
    regmap_write(
        anatop,
        ANADIG_REG_CORE + if enable { REG_SET } else { REG_CLR },
        BM_ANADIG_REG_CORE_FET_ODRIVE,
    );
}

#[inline]
unsafe fn imx_anatop_enable_2p5_pulldown(enable: bool) {
    regmap_write(
        anatop,
        ANADIG_REG_2P5 + if enable { REG_SET } else { REG_CLR },
        BM_ANADIG_REG_2P5_ENABLE_PULLDOWN,
    );
}

#[inline]
unsafe fn imx_anatop_disconnect_high_snvs(enable: bool) {
    regmap_write(
        anatop,
        ANADIG_ANA_MISC0 + if enable { REG_SET } else { REG_CLR },
        BM_ANADIG_ANA_MISC0_DISCON_HIGH_SNVS,
    );
}

pub unsafe fn imx_anatop_pre_suspend() {
    if imx_mmdc_get_ddr_type() == IMX_DDR_TYPE_LPDDR2 {
        imx_anatop_enable_2p5_pulldown(true);
    } else {
        imx_anatop_enable_weak2p5(true);
    }

    imx_anatop_enable_fet_odrive(true);

    if cpu_is_imx6sl() {
        imx_anatop_disconnect_high_snvs(true);
    }
}

pub unsafe fn imx_anatop_post_resume() {
    if imx_mmdc_get_ddr_type() == IMX_DDR_TYPE_LPDDR2 {
        imx_anatop_enable_2p5_pulldown(false);
    } else {
        imx_anatop_enable_weak2p5(false);
    }

    imx_anatop_enable_fet_odrive(false);

    if cpu_is_imx6sl() {
        imx_anatop_disconnect_high_snvs(false);
    }
}

pub unsafe fn imx_init_revision_from_anatop() {
    let np: *mut DeviceNode;
    let mut src_np: *mut DeviceNode;
    let anatop_base: *mut c_void;
    let mut revision: u32;
    let digprog: U32;
    let mut offset: u16 = ANADIG_DIGPROG;
    let major_part: u8;
    let minor_part: u8;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,imx6q-anatop\0".as_ptr());
    anatop_base = of_iomap(np, 0);
    // WARN_ON(!anatop_base);
    if of_device_is_compatible(np, b"fsl,imx6sl-anatop\0".as_ptr()) {
        offset = ANADIG_DIGPROG_IMX6SL;
    }
    if of_device_is_compatible(np, b"fsl,imx7d-anatop\0".as_ptr()) {
        offset = ANADIG_DIGPROG_IMX7D;
    }
    digprog = readl_relaxed(anatop_base.add(offset as usize));
    iounmap(anatop_base);

    /*
     * On i.MX7D digprog value match linux version format, so
     * it needn't map again and we can use register value directly.
     */
    if of_device_is_compatible(np, b"fsl,imx7d-anatop\0".as_ptr()) {
        revision = digprog & 0xff;
    } else {
        /*
         * MAJOR: [15:8], the major silicon revison;
         * MINOR: [7: 0], the minor silicon revison;
         *
         * please refer to the i.MX RM for the detailed
         * silicon revison bit define.
         * format the major part and minor part to match the
         * linux kernel soc version format.
         */
        major_part = ((digprog >> 8) & 0xf) as u8;
        minor_part = (digprog & 0xf) as u8;
        revision = (((major_part as u32 + 1) << 4) | minor_part as u32) as u32;

        if (digprog >> 16) == MXC_CPU_IMX6ULL {
            let src_base: *mut c_void;
            let sbmr2: U32;

            src_np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,imx6ul-src\0".as_ptr());
            src_base = of_iomap(src_np, 0);
            of_node_put(src_np);
            // WARN_ON(!src_base);
            sbmr2 = readl_relaxed(src_base.add(SRC_SBMR2 as usize));
            iounmap(src_base);

            /* src_sbmr2 bit 6 is to identify if it is i.MX6ULZ */
            if sbmr2 & (1 << 6) != 0 {
                // digprog &= ~(0xff << 16);
                // digprog |= (MXC_CPU_IMX6ULZ << 16);
                // The C object is mutated here; preserve that behavior below.
                let mut digprog_mut = digprog & !(0xff << 16);
                digprog_mut |= MXC_CPU_IMX6ULZ << 16;
                of_node_put(np);
                mxc_set_cpu_type((digprog_mut >> 16) & 0xff);
                imx_set_soc_revision(revision);
                return;
            }
        }
    }
    of_node_put(np);

    mxc_set_cpu_type((digprog >> 16) & 0xff);
    imx_set_soc_revision(revision);
}

pub unsafe fn imx_anatop_init() {
    anatop = syscon_regmap_lookup_by_compatible(b"fsl,imx6q-anatop\0".as_ptr());
    // IS_ERR(anatop) / pr_err(...) are supplied by linux/err.h and the kernel logging layer.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
