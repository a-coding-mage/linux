// SPDX-License-Identifier: GPL-2.0
//
// mt6797-afe-clk.c  --  Mediatek 6797 afe clock ctrl
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;

// Dependencies from <linux/clk.h>, "mt6797-afe-common.h", and
// "mt6797-afe-clk.h" are declared here and supplied by surrounding code.

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type gfp_t = u32;

#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut device,
    pub platform_priv: *mut c_void,
}

#[repr(C)]
pub struct mt6797_afe_private {
    pub clk: *mut *mut clk,
}

unsafe extern "C" {
    static GFP_KERNEL: gfp_t;
    static ENOMEM: c_int;

    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: gfp_t) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

const CLK_INFRA_SYS_AUD: usize = 0;
const CLK_INFRA_SYS_AUD_26M: usize = 1;
const CLK_TOP_MUX_AUD: usize = 2;
const CLK_TOP_MUX_AUD_BUS: usize = 3;
const CLK_TOP_SYSPLL3_D4: usize = 4;
const CLK_TOP_SYSPLL1_D4: usize = 5;
const CLK_CLK26M: usize = 6;
const CLK_NUM: usize = 7;

const AUD_CLKS: [*const c_char; CLK_NUM] = [
    b"infra_sys_audio_clk\0".as_ptr() as *const c_char,
    b"infra_sys_audio_26m\0".as_ptr() as *const c_char,
    b"top_mux_audio\0".as_ptr() as *const c_char,
    b"top_mux_aud_intbus\0".as_ptr() as *const c_char,
    b"top_sys_pll3_d4\0".as_ptr() as *const c_char,
    b"top_sys_pll1_d4\0".as_ptr() as *const c_char,
    b"top_clk26m_clk\0".as_ptr() as *const c_char,
];

const MT6797_INIT_CLOCK: *const c_char = b"mt6797_init_clock\0".as_ptr() as *const c_char;
const MT6797_AFE_ENABLE_CLOCK: *const c_char =
    b"mt6797_afe_enable_clock\0".as_ptr() as *const c_char;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt6797_init_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt6797_afe_private };
    let mut i: c_int;

    unsafe {
        (*afe_priv).clk = devm_kcalloc(
            (*afe).dev,
            CLK_NUM,
            size_of::<*mut clk>(),
            GFP_KERNEL,
        ) as *mut *mut clk;
    }
    if unsafe { (*afe_priv).clk.is_null() } {
        return unsafe { -ENOMEM };
    }

    i = 0;
    while i < CLK_NUM as c_int {
        unsafe {
            *(*afe_priv).clk.add(i as usize) = devm_clk_get((*afe).dev, AUD_CLKS[i as usize]);
            if IS_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void) {
                dev_err(
                    (*afe).dev,
                    b"%s(), devm_clk_get %s fail, ret %ld\n\0".as_ptr() as *const c_char,
                    MT6797_INIT_CLOCK,
                    AUD_CLKS[i as usize],
                    PTR_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void),
                );
                return PTR_ERR(*(*afe_priv).clk.add(i as usize) as *const c_void) as c_int;
            }
        }
        i += 1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt6797_afe_enable_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt6797_afe_private };
    let mut ret: c_int;

    ret = unsafe { clk_prepare_enable(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD)) };
    if ret != 0 {
        unsafe {
            dev_err(
                (*afe).dev,
                b"%s(), clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char,
                MT6797_AFE_ENABLE_CLOCK,
                AUD_CLKS[CLK_INFRA_SYS_AUD],
                ret,
            );
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD));
        }
        return 0;
    }

    ret = unsafe { clk_prepare_enable(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD_26M)) };
    if ret != 0 {
        unsafe {
            dev_err(
                (*afe).dev,
                b"%s(), clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char,
                MT6797_AFE_ENABLE_CLOCK,
                AUD_CLKS[CLK_INFRA_SYS_AUD_26M],
                ret,
            );
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD_26M));
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD));
        }
        return 0;
    }

    ret = unsafe { clk_prepare_enable(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD)) };
    if ret != 0 {
        unsafe {
            dev_err(
                (*afe).dev,
                b"%s(), clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char,
                MT6797_AFE_ENABLE_CLOCK,
                AUD_CLKS[CLK_TOP_MUX_AUD],
                ret,
            );
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD));
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD_26M));
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD));
        }
        return 0;
    }

    ret = unsafe {
        clk_set_parent(
            *(*afe_priv).clk.add(CLK_TOP_MUX_AUD),
            *(*afe_priv).clk.add(CLK_CLK26M),
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                (*afe).dev,
                b"%s(), clk_set_parent %s-%s fail %d\n\0".as_ptr() as *const c_char,
                MT6797_AFE_ENABLE_CLOCK,
                AUD_CLKS[CLK_TOP_MUX_AUD],
                AUD_CLKS[CLK_CLK26M],
                ret,
            );
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD));
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD_26M));
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD));
        }
        return 0;
    }

    ret = unsafe { clk_prepare_enable(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD_BUS)) };
    if ret != 0 {
        unsafe {
            dev_err(
                (*afe).dev,
                b"%s(), clk_prepare_enable %s fail %d\n\0".as_ptr() as *const c_char,
                MT6797_AFE_ENABLE_CLOCK,
                AUD_CLKS[CLK_TOP_MUX_AUD_BUS],
                ret,
            );
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD_BUS));
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD));
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD_26M));
            clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD));
        }
        return 0;
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mt6797_afe_disable_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv as *mut mt6797_afe_private };

    unsafe {
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD_BUS));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_TOP_MUX_AUD));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD_26M));
        clk_disable_unprepare(*(*afe_priv).clk.add(CLK_INFRA_SYS_AUD));
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
