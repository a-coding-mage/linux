// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright(c) 2021 Mediatek Corporation. All rights reserved.
//
// Author: YC Hung <yc.hung@mediatek.com>
//
// Hardware interface for mt8195 DSP clock

// Dependencies from the original C includes:
// linux/clk.h, linux/io.h, linux/string_choices.h, mt8195.h,
// mt8195-clk.h, ../adsp_helper.h, ../../sof-audio.h

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static ADSP_CLK_MAX: usize;
    static CLK_TOP_ADSP: usize;
    static CLK_TOP_CLK26M: usize;
    static CLK_TOP_AUDIO_LOCAL_BUS: usize;
    static CLK_TOP_MAINPLL_D7_D2: usize;
    static CLK_SCP_ADSP_AUDIODSP: usize;
    static CLK_TOP_AUDIO_H: usize;
    static GFP_KERNEL: c_int;
    static ENOMEM: c_int;

    fn devm_kcalloc(
        dev: *mut device,
        n: usize,
        size: usize,
        flags: c_int,
    ) -> *mut *mut clk;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn str_on_off(on: bool) -> *const c_char;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub pdata: *mut snd_sof_pdata,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub hw_pdata: *mut adsp_priv,
}

#[repr(C)]
pub struct adsp_priv {
    pub clk: *mut *mut clk,
}

static mut adsp_clks: [*const c_char; 6] = [
    b"adsp_sel\0".as_ptr() as *const c_char,
    b"clk26m_ck\0".as_ptr() as *const c_char,
    b"audio_local_bus\0".as_ptr() as *const c_char,
    b"mainpll_d7_d2\0".as_ptr() as *const c_char,
    b"scp_adsp_audiodsp\0".as_ptr() as *const c_char,
    b"audio_h\0".as_ptr() as *const c_char,
];

#[no_mangle]
pub unsafe extern "C" fn mt8195_adsp_init_clock(sdev: *mut snd_sof_dev) -> c_int {
    let dev = (*sdev).dev;
    let priv_0 = (*(*sdev).pdata).hw_pdata;
    let mut i: c_int;

    (*priv_0).clk = devm_kcalloc(
        dev,
        ADSP_CLK_MAX,
        core::mem::size_of::<*mut clk>(),
        GFP_KERNEL,
    );

    if (*priv_0).clk.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < ADSP_CLK_MAX as c_int {
        *(*priv_0).clk.add(i as usize) = devm_clk_get(dev, adsp_clks[i as usize]);
        if IS_ERR(*(*priv_0).clk.add(i as usize) as *const c_void) {
            return PTR_ERR(*(*priv_0).clk.add(i as usize) as *const c_void);
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn adsp_enable_all_clock(sdev: *mut snd_sof_dev) -> c_int {
    let dev = (*sdev).dev;
    let priv_0 = (*(*sdev).pdata).hw_pdata;
    let mut ret: c_int;

    ret = clk_prepare_enable(*(*priv_0).clk.add(CLK_TOP_MAINPLL_D7_D2));
    if ret != 0 {
        dev_err(
            dev,
            b"%s clk_prepare_enable(mainpll_d7_d2) fail %d\n\0".as_ptr() as *const c_char,
            b"adsp_enable_all_clock\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = clk_prepare_enable(*(*priv_0).clk.add(CLK_TOP_ADSP));
    if ret != 0 {
        dev_err(
            dev,
            b"%s clk_prepare_enable(adsp_sel) fail %d\n\0".as_ptr() as *const c_char,
            b"adsp_enable_all_clock\0".as_ptr() as *const c_char,
            ret,
        );
        clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_MAINPLL_D7_D2));
        return ret;
    }

    ret = clk_prepare_enable(*(*priv_0).clk.add(CLK_TOP_AUDIO_LOCAL_BUS));
    if ret != 0 {
        dev_err(
            dev,
            b"%s clk_prepare_enable(audio_local_bus) fail %d\n\0".as_ptr() as *const c_char,
            b"adsp_enable_all_clock\0".as_ptr() as *const c_char,
            ret,
        );
        clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_ADSP));
        clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_MAINPLL_D7_D2));
        return ret;
    }

    ret = clk_prepare_enable(*(*priv_0).clk.add(CLK_SCP_ADSP_AUDIODSP));
    if ret != 0 {
        dev_err(
            dev,
            b"%s clk_prepare_enable(scp_adsp_audiodsp) fail %d\n\0".as_ptr() as *const c_char,
            b"adsp_enable_all_clock\0".as_ptr() as *const c_char,
            ret,
        );
        clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_AUDIO_LOCAL_BUS));
        clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_ADSP));
        clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_MAINPLL_D7_D2));
        return ret;
    }

    ret = clk_prepare_enable(*(*priv_0).clk.add(CLK_TOP_AUDIO_H));
    if ret != 0 {
        dev_err(
            dev,
            b"%s clk_prepare_enable(audio_h) fail %d\n\0".as_ptr() as *const c_char,
            b"adsp_enable_all_clock\0".as_ptr() as *const c_char,
            ret,
        );
        clk_disable_unprepare(*(*priv_0).clk.add(CLK_SCP_ADSP_AUDIODSP));
        clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_AUDIO_LOCAL_BUS));
        clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_ADSP));
        clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_MAINPLL_D7_D2));
        return ret;
    }

    0
}

unsafe extern "C" fn adsp_disable_all_clock(sdev: *mut snd_sof_dev) {
    let priv_0 = (*(*sdev).pdata).hw_pdata;

    clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_AUDIO_H));
    clk_disable_unprepare(*(*priv_0).clk.add(CLK_SCP_ADSP_AUDIODSP));
    clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_AUDIO_LOCAL_BUS));
    clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_ADSP));
    clk_disable_unprepare(*(*priv_0).clk.add(CLK_TOP_MAINPLL_D7_D2));
}

unsafe extern "C" fn adsp_default_clk_init(sdev: *mut snd_sof_dev, enable: bool) -> c_int {
    let dev = (*sdev).dev;
    let priv_0 = (*(*sdev).pdata).hw_pdata;
    let mut ret: c_int;

    dev_dbg(
        dev,
        b"%s: %s\n\0".as_ptr() as *const c_char,
        b"adsp_default_clk_init\0".as_ptr() as *const c_char,
        str_on_off(enable),
    );

    if enable {
        ret = clk_set_parent(
            *(*priv_0).clk.add(CLK_TOP_ADSP),
            *(*priv_0).clk.add(CLK_TOP_CLK26M),
        );
        if ret != 0 {
            dev_err(
                dev,
                b"failed to set dsp_sel to clk26m: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        ret = clk_set_parent(
            *(*priv_0).clk.add(CLK_TOP_AUDIO_LOCAL_BUS),
            *(*priv_0).clk.add(CLK_TOP_MAINPLL_D7_D2),
        );
        if ret != 0 {
            dev_err(
                dev,
                b"set audio_local_bus failed %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        ret = clk_set_parent(
            *(*priv_0).clk.add(CLK_TOP_AUDIO_H),
            *(*priv_0).clk.add(CLK_TOP_CLK26M),
        );
        if ret != 0 {
            dev_err(
                dev,
                b"set audio_h_sel failed %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        ret = adsp_enable_all_clock(sdev);
        if ret != 0 {
            dev_err(
                dev,
                b"failed to adsp_enable_clock: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
    } else {
        adsp_disable_all_clock(sdev);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn adsp_clock_on(sdev: *mut snd_sof_dev) -> c_int {
    /* Open ADSP clock */
    adsp_default_clk_init(sdev, true)
}

#[no_mangle]
pub unsafe extern "C" fn adsp_clock_off(sdev: *mut snd_sof_dev) -> c_int {
    /* Close ADSP clock */
    adsp_default_clk_init(sdev, false)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
