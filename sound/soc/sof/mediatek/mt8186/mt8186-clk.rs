// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright(c) 2022 Mediatek Corporation. All rights reserved.
//
// Author: Allen-KH Cheng <allen-kh.cheng@mediatek.com>
//         Tinghan Shen <tinghan.shen@mediatek.com>
//
// Hardware interface for mt8186 DSP clock

// C dependencies:
// #include <linux/clk.h>
// #include <linux/io.h>
// #include "../../sof-audio.h"
// #include "../../ops.h"
// #include "../adsp_helper.h"
// #include "mt8186.h"
// #include "mt8186-clk.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct snd_sof_dev {
    pub pdata: *mut snd_sof_pdata,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut c_void,
}

#[repr(C)]
pub struct adsp_priv {
    pub clk: *mut *mut clk,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

const ADSP_CLK_MAX: usize = 2;
const CLK_TOP_AUDIODSP: usize = 0;
const CLK_TOP_ADSP_BUS: usize = 1;

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;

const DSP_REG_BAR: u32 = 0;
const ADSP_CK_EN: u32 = 0;
const ADSP_UART_CTRL: u32 = 0;
const UART_EN: u32 = 0;
const DMA_EN: u32 = 0;
const TIMER_EN: u32 = 0;
const COREDBG_EN: u32 = 0;
const CORE_CLK_EN: u32 = 0;
const UART_BCLK_CG: u32 = 0;
const UART_RSTN: u32 = 0;

static ADSP_CLKS: [*const c_char; ADSP_CLK_MAX] = [
    b"audiodsp\0".as_ptr() as *const c_char,
    b"adsp_bus\0".as_ptr() as *const c_char,
];

extern "C" {
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_int) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_sof_dsp_write(sdev: *mut snd_sof_dev, bar: u32, offset: u32, value: u32);
}

#[no_mangle]
pub unsafe extern "C" fn mt8186_adsp_init_clock(sdev: *mut snd_sof_dev) -> c_int {
    let priv_ = (*(*sdev).pdata).hw_pdata as *mut adsp_priv;
    let dev = (*sdev).dev;
    let mut i: c_int;

    (*priv_).clk = devm_kcalloc(
        dev,
        ADSP_CLK_MAX,
        core::mem::size_of::<*mut clk>(),
        GFP_KERNEL,
    ) as *mut *mut clk;
    if (*priv_).clk.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < ADSP_CLK_MAX as c_int {
        *(*priv_).clk.add(i as usize) = devm_clk_get(dev, ADSP_CLKS[i as usize]);

        if IS_ERR(*(*priv_).clk.add(i as usize) as *const c_void) {
            return PTR_ERR(*(*priv_).clk.add(i as usize) as *const c_void);
        }

        i += 1;
    }

    0
}

unsafe extern "C" fn adsp_enable_all_clock(sdev: *mut snd_sof_dev) -> c_int {
    let priv_ = (*(*sdev).pdata).hw_pdata as *mut adsp_priv;
    let dev = (*sdev).dev;
    let mut ret: c_int;

    ret = clk_prepare_enable(*(*priv_).clk.add(CLK_TOP_AUDIODSP));
    if ret != 0 {
        dev_err(
            dev,
            b"%s clk_prepare_enable(audiodsp) fail %d\n\0".as_ptr() as *const c_char,
            b"adsp_enable_all_clock\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = clk_prepare_enable(*(*priv_).clk.add(CLK_TOP_ADSP_BUS));
    if ret != 0 {
        dev_err(
            dev,
            b"%s clk_prepare_enable(adsp_bus) fail %d\n\0".as_ptr() as *const c_char,
            b"adsp_enable_all_clock\0".as_ptr() as *const c_char,
            ret,
        );
        clk_disable_unprepare(*(*priv_).clk.add(CLK_TOP_AUDIODSP));
        return ret;
    }

    0
}

unsafe extern "C" fn adsp_disable_all_clock(sdev: *mut snd_sof_dev) {
    let priv_ = (*(*sdev).pdata).hw_pdata as *mut adsp_priv;

    clk_disable_unprepare(*(*priv_).clk.add(CLK_TOP_ADSP_BUS));
    clk_disable_unprepare(*(*priv_).clk.add(CLK_TOP_AUDIODSP));
}

#[no_mangle]
pub unsafe extern "C" fn mt8186_adsp_clock_on(sdev: *mut snd_sof_dev) -> c_int {
    let dev = (*sdev).dev;
    let mut ret: c_int;

    ret = adsp_enable_all_clock(sdev);
    if ret != 0 {
        dev_err(
            dev,
            b"failed to adsp_enable_clock: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }
    snd_sof_dsp_write(
        sdev,
        DSP_REG_BAR,
        ADSP_CK_EN,
        UART_EN | DMA_EN | TIMER_EN | COREDBG_EN | CORE_CLK_EN,
    );
    snd_sof_dsp_write(
        sdev,
        DSP_REG_BAR,
        ADSP_UART_CTRL,
        UART_BCLK_CG | UART_RSTN,
    );

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8186_adsp_clock_off(sdev: *mut snd_sof_dev) {
    snd_sof_dsp_write(sdev, DSP_REG_BAR, ADSP_CK_EN, 0);
    snd_sof_dsp_write(sdev, DSP_REG_BAR, ADSP_UART_CTRL, 0);
    adsp_disable_all_clock(sdev);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
