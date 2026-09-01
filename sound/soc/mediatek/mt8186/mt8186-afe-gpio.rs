// SPDX-License-Identifier: GPL-2.0
//
// mt8186-afe-gpio.c  --  Mediatek 8186 afe gpio ctrl
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>

// C dependencies:
// #include <linux/pinctrl/consumer.h>
// #include "mt8186-afe-common.h"
// #include "mt8186-afe-gpio.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pinctrl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pinctrl_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

extern "C" {
    fn devm_pinctrl_get(dev: *mut device) -> *mut pinctrl;
    fn pinctrl_lookup_state(
        p: *mut pinctrl,
        name: *const c_char,
    ) -> *mut pinctrl_state;
    fn pinctrl_select_state(
        p: *mut pinctrl,
        state: *mut pinctrl_state,
    ) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

extern "C" {
    static MT8186_DAI_ADDA: c_int;
    static MT8186_DAI_I2S_0: c_int;
    static MT8186_DAI_I2S_1: c_int;
    static MT8186_DAI_I2S_2: c_int;
    static MT8186_DAI_I2S_3: c_int;
    static MT8186_DAI_TDM_IN: c_int;
    static MT8186_DAI_PCM: c_int;
}

const EINVAL: c_int = 22;
const EIO: c_int = 5;

static mut aud_pinctrl: *mut pinctrl = ptr::null_mut();

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum mt8186_afe_gpio {
    MT8186_AFE_GPIO_CLK_MOSI_OFF,
    MT8186_AFE_GPIO_CLK_MOSI_ON,
    MT8186_AFE_GPIO_CLK_MISO_OFF,
    MT8186_AFE_GPIO_CLK_MISO_ON,
    MT8186_AFE_GPIO_DAT_MISO_OFF,
    MT8186_AFE_GPIO_DAT_MISO_ON,
    MT8186_AFE_GPIO_DAT_MOSI_OFF,
    MT8186_AFE_GPIO_DAT_MOSI_ON,
    MT8186_AFE_GPIO_I2S0_OFF,
    MT8186_AFE_GPIO_I2S0_ON,
    MT8186_AFE_GPIO_I2S1_OFF,
    MT8186_AFE_GPIO_I2S1_ON,
    MT8186_AFE_GPIO_I2S2_OFF,
    MT8186_AFE_GPIO_I2S2_ON,
    MT8186_AFE_GPIO_I2S3_OFF,
    MT8186_AFE_GPIO_I2S3_ON,
    MT8186_AFE_GPIO_TDM_OFF,
    MT8186_AFE_GPIO_TDM_ON,
    MT8186_AFE_GPIO_PCM_OFF,
    MT8186_AFE_GPIO_PCM_ON,
    MT8186_AFE_GPIO_GPIO_NUM,
}

#[repr(C)]
struct audio_gpio_attr {
    name: *const c_char,
    gpio_prepare: bool,
    gpioctrl: *mut pinctrl_state,
}

static mut aud_gpios: [audio_gpio_attr; mt8186_afe_gpio::MT8186_AFE_GPIO_GPIO_NUM as usize] = [
    audio_gpio_attr {
        name: b"aud_clk_mosi_off\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_clk_mosi_on\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_clk_miso_off\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_clk_miso_on\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_dat_miso_off\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_dat_miso_on\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_dat_mosi_off\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_dat_mosi_on\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_gpio_i2s0_off\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_gpio_i2s0_on\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_gpio_i2s1_off\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_gpio_i2s1_on\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_gpio_i2s2_off\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_gpio_i2s2_on\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_gpio_i2s3_off\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_gpio_i2s3_on\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_gpio_tdm_off\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_gpio_tdm_on\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_gpio_pcm_off\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
    audio_gpio_attr {
        name: b"aud_gpio_pcm_on\0".as_ptr() as *const c_char,
        gpio_prepare: false,
        gpioctrl: ptr::null_mut(),
    },
];

// static DEFINE_MUTEX(gpio_request_mutex);
static mut gpio_request_mutex: mutex = mutex { _private: [] };

#[no_mangle]
pub unsafe extern "C" fn mt8186_afe_gpio_init(dev: *mut device) -> c_int {
    let mut i: c_int;
    let mut j: c_int;
    let mut ret: c_int;

    aud_pinctrl = devm_pinctrl_get(dev);
    if IS_ERR(aud_pinctrl as *const c_void) {
        ret = PTR_ERR(aud_pinctrl as *const c_void);
        dev_err(
            dev,
            b"%s(), ret %d, cannot get aud_pinctrl!\n\0".as_ptr() as *const c_char,
            b"mt8186_afe_gpio_init\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    i = 0;
    while (i as usize) < aud_gpios.len() {
        aud_gpios[i as usize].gpioctrl =
            pinctrl_lookup_state(aud_pinctrl, aud_gpios[i as usize].name);
        if IS_ERR(aud_gpios[i as usize].gpioctrl as *const c_void) {
            ret = PTR_ERR(aud_gpios[i as usize].gpioctrl as *const c_void);
            dev_dbg(
                dev,
                b"%s(), pinctrl_lookup_state %s fail, ret %d\n\0".as_ptr() as *const c_char,
                b"mt8186_afe_gpio_init\0".as_ptr() as *const c_char,
                aud_gpios[i as usize].name,
                ret,
            );
        } else {
            aud_gpios[i as usize].gpio_prepare = true;
        }
        i += 1;
    }

    /* gpio status init */
    i = MT8186_DAI_ADDA;
    while i <= MT8186_DAI_TDM_IN {
        j = 0;
        while j <= 1 {
            mt8186_afe_gpio_request(dev, false, i, j);
            j += 1;
        }
        i += 1;
    }

    0
}
// EXPORT_SYMBOL_GPL(mt8186_afe_gpio_init);

unsafe fn mt8186_afe_gpio_select(dev: *mut device, type_: mt8186_afe_gpio) -> c_int {
    let mut ret: c_int = 0;

    if (type_ as c_int) < 0
        || (type_ as c_int) >= mt8186_afe_gpio::MT8186_AFE_GPIO_GPIO_NUM as c_int
    {
        dev_dbg(
            dev,
            b"%s(), error, invalid gpio type %d\n\0".as_ptr() as *const c_char,
            b"mt8186_afe_gpio_select\0".as_ptr() as *const c_char,
            type_ as c_int,
        );
        return -EINVAL;
    }

    if !aud_gpios[type_ as usize].gpio_prepare {
        dev_dbg(
            dev,
            b"%s(), error, gpio type %d not prepared\n\0".as_ptr() as *const c_char,
            b"mt8186_afe_gpio_select\0".as_ptr() as *const c_char,
            type_ as c_int,
        );
        return -EIO;
    }

    ret = pinctrl_select_state(aud_pinctrl, aud_gpios[type_ as usize].gpioctrl);
    if ret != 0 {
        dev_dbg(
            dev,
            b"%s(), error, can not set gpio type %d\n\0".as_ptr() as *const c_char,
            b"mt8186_afe_gpio_select\0".as_ptr() as *const c_char,
            type_ as c_int,
        );
        return ret;
    }

    0
}

unsafe fn mt8186_afe_gpio_adda_dl(dev: *mut device, enable: bool) -> c_int {
    let mut ret: c_int;

    if enable {
        ret = mt8186_afe_gpio_select(dev, mt8186_afe_gpio::MT8186_AFE_GPIO_CLK_MOSI_ON);
        if ret != 0 {
            dev_dbg(
                dev,
                b"%s(), MOSI CLK ON select fail!\n\0".as_ptr() as *const c_char,
                b"mt8186_afe_gpio_adda_dl\0".as_ptr() as *const c_char,
            );
            return ret;
        }

        ret = mt8186_afe_gpio_select(dev, mt8186_afe_gpio::MT8186_AFE_GPIO_DAT_MOSI_ON);
        if ret != 0 {
            dev_dbg(
                dev,
                b"%s(), MOSI DAT ON select fail!\n\0".as_ptr() as *const c_char,
                b"mt8186_afe_gpio_adda_dl\0".as_ptr() as *const c_char,
            );
            return ret;
        }
    } else {
        ret = mt8186_afe_gpio_select(dev, mt8186_afe_gpio::MT8186_AFE_GPIO_DAT_MOSI_OFF);
        if ret != 0 {
            dev_dbg(
                dev,
                b"%s(), MOSI DAT OFF select fail!\n\0".as_ptr() as *const c_char,
                b"mt8186_afe_gpio_adda_dl\0".as_ptr() as *const c_char,
            );
            return ret;
        }

        ret = mt8186_afe_gpio_select(dev, mt8186_afe_gpio::MT8186_AFE_GPIO_CLK_MOSI_OFF);
        if ret != 0 {
            dev_dbg(
                dev,
                b"%s(), MOSI CLK ON select fail!\n\0".as_ptr() as *const c_char,
                b"mt8186_afe_gpio_adda_dl\0".as_ptr() as *const c_char,
            );
            return ret;
        }
    }

    0
}

unsafe fn mt8186_afe_gpio_adda_ul(dev: *mut device, enable: bool) -> c_int {
    let mut ret: c_int;

    if enable {
        ret = mt8186_afe_gpio_select(dev, mt8186_afe_gpio::MT8186_AFE_GPIO_CLK_MISO_ON);
        if ret != 0 {
            dev_dbg(
                dev,
                b"%s(), MISO CLK ON select fail!\n\0".as_ptr() as *const c_char,
                b"mt8186_afe_gpio_adda_ul\0".as_ptr() as *const c_char,
            );
            return ret;
        }

        ret = mt8186_afe_gpio_select(dev, mt8186_afe_gpio::MT8186_AFE_GPIO_DAT_MISO_ON);
        if ret != 0 {
            dev_dbg(
                dev,
                b"%s(), MISO DAT ON select fail!\n\0".as_ptr() as *const c_char,
                b"mt8186_afe_gpio_adda_ul\0".as_ptr() as *const c_char,
            );
            return ret;
        }
    } else {
        ret = mt8186_afe_gpio_select(dev, mt8186_afe_gpio::MT8186_AFE_GPIO_DAT_MISO_OFF);
        if ret != 0 {
            dev_dbg(
                dev,
                b"%s(), MISO DAT OFF select fail!\n\0".as_ptr() as *const c_char,
                b"mt8186_afe_gpio_adda_ul\0".as_ptr() as *const c_char,
            );
            return ret;
        }

        ret = mt8186_afe_gpio_select(dev, mt8186_afe_gpio::MT8186_AFE_GPIO_CLK_MISO_OFF);
        if ret != 0 {
            dev_dbg(
                dev,
                b"%s(), MISO CLK OFF select fail!\n\0".as_ptr() as *const c_char,
                b"mt8186_afe_gpio_adda_ul\0".as_ptr() as *const c_char,
            );
            return ret;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8186_afe_gpio_request(
    dev: *mut device,
    enable: bool,
    dai: c_int,
    uplink: c_int,
) -> c_int {
    let sel: mt8186_afe_gpio;
    let mut ret: c_int = -EINVAL;

    mutex_lock(&mut gpio_request_mutex);

    if dai == MT8186_DAI_ADDA {
        if uplink != 0 {
            ret = mt8186_afe_gpio_adda_ul(dev, enable);
        } else {
            ret = mt8186_afe_gpio_adda_dl(dev, enable);
        }
        mutex_unlock(&mut gpio_request_mutex);
        return ret;
    } else if dai == MT8186_DAI_I2S_0 {
        sel = if enable {
            mt8186_afe_gpio::MT8186_AFE_GPIO_I2S0_ON
        } else {
            mt8186_afe_gpio::MT8186_AFE_GPIO_I2S0_OFF
        };
    } else if dai == MT8186_DAI_I2S_1 {
        sel = if enable {
            mt8186_afe_gpio::MT8186_AFE_GPIO_I2S1_ON
        } else {
            mt8186_afe_gpio::MT8186_AFE_GPIO_I2S1_OFF
        };
    } else if dai == MT8186_DAI_I2S_2 {
        sel = if enable {
            mt8186_afe_gpio::MT8186_AFE_GPIO_I2S2_ON
        } else {
            mt8186_afe_gpio::MT8186_AFE_GPIO_I2S2_OFF
        };
    } else if dai == MT8186_DAI_I2S_3 {
        sel = if enable {
            mt8186_afe_gpio::MT8186_AFE_GPIO_I2S3_ON
        } else {
            mt8186_afe_gpio::MT8186_AFE_GPIO_I2S3_OFF
        };
    } else if dai == MT8186_DAI_TDM_IN {
        sel = if enable {
            mt8186_afe_gpio::MT8186_AFE_GPIO_TDM_ON
        } else {
            mt8186_afe_gpio::MT8186_AFE_GPIO_TDM_OFF
        };
    } else if dai == MT8186_DAI_PCM {
        sel = if enable {
            mt8186_afe_gpio::MT8186_AFE_GPIO_PCM_ON
        } else {
            mt8186_afe_gpio::MT8186_AFE_GPIO_PCM_OFF
        };
    } else {
        dev_dbg(
            dev,
            b"%s(), invalid dai %d\n\0".as_ptr() as *const c_char,
            b"mt8186_afe_gpio_request\0".as_ptr() as *const c_char,
            dai,
        );
        mutex_unlock(&mut gpio_request_mutex);
        return ret;
    }

    ret = mt8186_afe_gpio_select(dev, sel);
    mutex_unlock(&mut gpio_request_mutex);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
