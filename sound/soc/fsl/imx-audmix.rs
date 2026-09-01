// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2017 NXP
 *
 * The code contained herein is licensed under the GNU General Public
 * License. You may obtain a copy of the GNU General Public License
 * Version 2 or later at the following locations:
 *
 * https://www.opensource.org/licenses/gpl-license.html
 * https://www.gnu.org/copyleft/gpl.html
 */

// Dependencies from the original C includes:
// linux/module.h, linux/of_platform.h, linux/clk.h, sound/soc.h,
// sound/soc-dapm.h, fsl_sai.h, fsl_audmix.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct imx_audmix {
    pub pdev: *mut platform_device,
    pub card: snd_soc_card,
    pub audmix_pdev: *mut platform_device,
    pub out_pdev: *mut platform_device,
    pub num_dai: c_int,
    pub dai: *mut snd_soc_dai_link,
    pub num_dai_conf: c_int,
    pub dai_conf: *mut snd_soc_codec_conf,
    pub num_dapm_routes: c_int,
    pub dapm_routes: *mut snd_soc_dapm_route,
}

unsafe fn imx_audmix_fe_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = unsafe { (*substream).runtime };
    let ret: c_int;

    ret = unsafe {
        snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_CHANNELS, 1, 8)
    };
    if ret < 0 {
        return ret;
    }

    unsafe {
        snd_pcm_hw_constraint_mask64(runtime, SNDRV_PCM_HW_PARAM_FORMAT, FSL_AUDMIX_FORMATS)
    }
}

unsafe fn imx_audmix_fe_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let dev = unsafe { (*(*rtd).card).dev };
    let tx = unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK };
    let mut fmt: c_uint = SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_NB_NF;
    let channels: u32 = unsafe { params_channels(params) };
    let mut ret: c_int;
    let dir: c_int;

    /* For playback the AUDMIX is consumer, and for record is provider */
    fmt |= if tx {
        SND_SOC_DAIFMT_BP_FP
    } else {
        SND_SOC_DAIFMT_BC_FC
    };
    dir = if tx {
        SND_SOC_CLOCK_OUT
    } else {
        SND_SOC_CLOCK_IN
    };

    /* set DAI configuration */
    ret = unsafe { snd_soc_dai_set_fmt(snd_soc_rtd_to_cpu(rtd, 0), fmt) };
    if ret != 0 {
        unsafe {
            dev_err(
                dev,
                c"failed to set cpu dai fmt: %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    ret = unsafe { snd_soc_dai_set_sysclk(snd_soc_rtd_to_cpu(rtd, 0), FSL_SAI_CLK_MAST1, 0, dir) };
    if ret != 0 {
        unsafe {
            dev_err(
                dev,
                c"failed to set cpu sysclk: %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    /*
     * Per datasheet, AUDMIX expects 8 slots and 32 bits
     * for every slot in TDM mode.
     */
    ret = unsafe {
        snd_soc_dai_set_tdm_slot(
            snd_soc_rtd_to_cpu(rtd, 0),
            BIT(channels) - 1,
            BIT(channels) - 1,
            8,
            32,
        )
    };
    if ret != 0 {
        unsafe {
            dev_err(
                dev,
                c"failed to set cpu dai tdm slot: %d\n".as_ptr(),
                ret,
            );
        }
    }

    ret
}

unsafe fn imx_audmix_be_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let dev = unsafe { (*(*rtd).card).dev };
    let tx = unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK };
    let mut fmt: c_uint = SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_NB_NF;
    let ret: c_int;

    if !tx {
        return 0;
    }

    /* For playback the AUDMIX is consumer */
    fmt |= SND_SOC_DAIFMT_BC_FC;

    /* set AUDMIX DAI configuration */
    ret = unsafe { snd_soc_dai_set_fmt(snd_soc_rtd_to_cpu(rtd, 0), fmt) };
    if ret != 0 {
        unsafe {
            dev_err(
                dev,
                c"failed to set AUDMIX DAI fmt: %d\n".as_ptr(),
                ret,
            );
        }
    }

    ret
}

static imx_audmix_fe_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(imx_audmix_fe_startup),
    hw_params: Some(imx_audmix_fe_hw_params),
};

static imx_audmix_be_ops: snd_soc_ops = snd_soc_ops {
    startup: None,
    hw_params: Some(imx_audmix_be_hw_params),
};

static name: [[*const c_char; 3]; 4] = [
    [
        c"HiFi-AUDMIX-FE-0".as_ptr(),
        c"HiFi-AUDMIX-FE-1".as_ptr(),
        c"HiFi-AUDMIX-FE-2".as_ptr(),
    ],
    [c"sai-tx".as_ptr(), c"sai-tx".as_ptr(), c"sai-rx".as_ptr()],
    [
        c"AUDMIX-Playback-0".as_ptr(),
        c"AUDMIX-Playback-1".as_ptr(),
        c"SAI-Capture".as_ptr(),
    ],
    [
        c"SAI-Playback".as_ptr(),
        c"SAI-Playback".as_ptr(),
        c"AUDMIX-Capture-0".as_ptr(),
    ],
];

unsafe fn imx_audmix_probe(pdev: *mut platform_device) -> c_int {
    let np = unsafe { (*pdev).dev.of_node };
    let mut audmix_np: *mut device_node = ptr::null_mut();
    let mut out_cpu_np: *mut device_node = ptr::null_mut();
    let mut audmix_pdev: *mut platform_device = ptr::null_mut();
    let mut cpu_pdev: *mut platform_device;
    let mut args: of_phandle_args = unsafe { core::mem::zeroed() };
    let priv_: *mut imx_audmix;
    let mut i: c_int;
    let mut num_dai: c_int;
    let mut ret: c_int;
    let fe_name_pref = c"HiFi-AUDMIX-FE-".as_ptr();
    let mut be_name: *mut c_char;
    let mut dai_name: *mut c_char;

    if unsafe { !(*pdev).dev.parent.is_null() } {
        audmix_np = unsafe { (*(*pdev).dev.parent).of_node };
    } else {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                c"Missing parent device.\n".as_ptr(),
            );
        }
        return -EINVAL;
    }

    if audmix_np.is_null() {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                c"Missing DT node for parent device.\n".as_ptr(),
            );
        }
        return -EINVAL;
    }

    audmix_pdev = unsafe { of_find_device_by_node(audmix_np) };
    if audmix_pdev.is_null() {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                c"Missing AUDMIX platform device for %s\n".as_ptr(),
                (*np).full_name,
            );
        }
        return -EINVAL;
    }
    unsafe {
        put_device(&mut (*audmix_pdev).dev);
    }

    num_dai = unsafe { of_count_phandle_with_args(audmix_np, c"dais".as_ptr(), ptr::null()) };
    if num_dai != FSL_AUDMIX_MAX_DAIS {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                c"Need 2 dais to be provided for %s\n".as_ptr(),
                (*audmix_np).full_name,
            );
        }
        return -EINVAL;
    }

    priv_ = unsafe {
        devm_kzalloc(
            &mut (*pdev).dev,
            size_of::<imx_audmix>(),
            GFP_KERNEL,
        ) as *mut imx_audmix
    };
    if priv_.is_null() {
        return -ENOMEM;
    }

    num_dai += 1;
    unsafe {
        (*priv_).num_dai = 2 * num_dai;
        (*priv_).dai = devm_kcalloc(
            &mut (*pdev).dev,
            (*priv_).num_dai as usize,
            size_of::<snd_soc_dai_link>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link;
    }
    if unsafe { (*priv_).dai.is_null() } {
        return -ENOMEM;
    }

    unsafe {
        (*priv_).num_dai_conf = num_dai;
        (*priv_).dai_conf = devm_kcalloc(
            &mut (*pdev).dev,
            (*priv_).num_dai_conf as usize,
            size_of::<snd_soc_codec_conf>(),
            GFP_KERNEL,
        ) as *mut snd_soc_codec_conf;
    }
    if unsafe { (*priv_).dai_conf.is_null() } {
        return -ENOMEM;
    }

    unsafe {
        (*priv_).num_dapm_routes = num_dai;
        (*priv_).dapm_routes = devm_kcalloc(
            &mut (*pdev).dev,
            (*priv_).num_dapm_routes as usize,
            size_of::<snd_soc_dapm_route>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dapm_route;
    }
    if unsafe { (*priv_).dapm_routes.is_null() } {
        return -ENOMEM;
    }

    i = 0;
    while i < num_dai {
        let dlc: *mut snd_soc_dai_link_component;

        /* for CPU x 2 */
        dlc = unsafe {
            devm_kcalloc(
                &mut (*pdev).dev,
                2,
                size_of::<snd_soc_dai_link_component>(),
                GFP_KERNEL,
            ) as *mut snd_soc_dai_link_component
        };
        if dlc.is_null() {
            return -ENOMEM;
        }

        if i == num_dai - 1 {
            ret = unsafe {
                of_parse_phandle_with_args(audmix_np, c"dais".as_ptr(), ptr::null(), 0, &mut args)
            };
        } else {
            ret = unsafe {
                of_parse_phandle_with_args(audmix_np, c"dais".as_ptr(), ptr::null(), i, &mut args)
            };
        }
        if ret < 0 {
            unsafe {
                dev_err(
                    &mut (*pdev).dev,
                    c"of_parse_phandle_with_args failed\n".as_ptr(),
                );
            }
            return ret;
        }

        cpu_pdev = unsafe { of_find_device_by_node(args.np) };
        if cpu_pdev.is_null() {
            unsafe {
                dev_err(
                    &mut (*pdev).dev,
                    c"failed to find SAI platform device\n".as_ptr(),
                );
            }
            return -EINVAL;
        }
        unsafe {
            put_device(&mut (*cpu_pdev).dev);
        }

        dai_name = unsafe {
            devm_kasprintf(
                &mut (*pdev).dev,
                GFP_KERNEL,
                c"%s%s".as_ptr(),
                fe_name_pref,
                (*args.np).full_name,
            )
        };
        if dai_name.is_null() {
            return -ENOMEM;
        }

        unsafe {
            dev_info((*pdev).dev.parent, c"DAI FE name:%s\n".as_ptr(), dai_name);
        }

        if i == num_dai - 1 {
            out_cpu_np = args.np;
        }

        /*
         * CPU == Platform
         * platform is using soc-generic-dmaengine-pcm
         */
        unsafe {
            (*(*priv_).dai.add(i as usize)).cpus = dlc.add(0);
            (*(*priv_).dai.add(i as usize)).platforms = dlc.add(0);
            (*(*priv_).dai.add(i as usize)).codecs = &snd_soc_dummy_dlc as *const _ as *mut _;

            (*(*priv_).dai.add(i as usize)).num_cpus = 1;
            (*(*priv_).dai.add(i as usize)).num_codecs = 1;
            (*(*priv_).dai.add(i as usize)).num_platforms = 1;
            (*(*priv_).dai.add(i as usize)).name = name[0][i as usize];
            (*(*priv_).dai.add(i as usize)).stream_name = c"HiFi-AUDMIX-FE".as_ptr();
            (*(*(*priv_).dai.add(i as usize)).cpus).of_node = args.np;
            (*(*(*priv_).dai.add(i as usize)).cpus).dai_name = name[1][i as usize];

            (*(*priv_).dai.add(i as usize)).dynamic = 1;
            if i == num_dai - 1 {
                (*(*priv_).dai.add(i as usize)).capture_only = 1;
            } else {
                (*(*priv_).dai.add(i as usize)).playback_only = 1;
            }
            (*(*priv_).dai.add(i as usize)).ignore_pmdown_time = 1;
            (*(*priv_).dai.add(i as usize)).ops = &imx_audmix_fe_ops;
        }

        /* Add AUDMIX Backend */
        be_name = unsafe {
            devm_kasprintf(
                &mut (*pdev).dev,
                GFP_KERNEL,
                c"audmix-%d".as_ptr(),
                i,
            )
        };
        if be_name.is_null() {
            return -ENOMEM;
        }

        unsafe {
            (*(*priv_).dai.add((num_dai + i) as usize)).cpus = dlc.add(1);
            (*(*priv_).dai.add((num_dai + i) as usize)).codecs =
                &snd_soc_dummy_dlc as *const _ as *mut _;

            (*(*priv_).dai.add((num_dai + i) as usize)).num_cpus = 1;
            (*(*priv_).dai.add((num_dai + i) as usize)).num_codecs = 1;

            (*(*priv_).dai.add((num_dai + i) as usize)).name = be_name;
            (*(*(*priv_).dai.add((num_dai + i) as usize)).cpus).of_node = audmix_np;
            (*(*(*priv_).dai.add((num_dai + i) as usize)).cpus).dai_name = be_name;
            (*(*priv_).dai.add((num_dai + i) as usize)).no_pcm = 1;
            if i == num_dai - 1 {
                (*(*priv_).dai.add((num_dai + i) as usize)).capture_only = 1;
            } else {
                (*(*priv_).dai.add((num_dai + i) as usize)).playback_only = 1;
            }
            (*(*priv_).dai.add((num_dai + i) as usize)).ignore_pmdown_time = 1;
            (*(*priv_).dai.add((num_dai + i) as usize)).ops = &imx_audmix_be_ops;

            (*(*priv_).dai_conf.add(i as usize)).dlc.of_node = args.np;
            (*(*priv_).dai_conf.add(i as usize)).name_prefix = dai_name;
        }

        if i == num_dai - 1 {
            unsafe {
                (*(*priv_).dapm_routes.add(i as usize)).sink = devm_kasprintf(
                    &mut (*pdev).dev,
                    GFP_KERNEL,
                    c"%s %s".as_ptr(),
                    dai_name,
                    name[2][i as usize],
                );
            }
            if unsafe { (*(*priv_).dapm_routes.add(i as usize)).sink.is_null() } {
                return -ENOMEM;
            }

            unsafe {
                (*(*priv_).dapm_routes.add(i as usize)).source = name[3][i as usize];
            }
        } else {
            unsafe {
                (*(*priv_).dapm_routes.add(i as usize)).source = devm_kasprintf(
                    &mut (*pdev).dev,
                    GFP_KERNEL,
                    c"%s %s".as_ptr(),
                    dai_name,
                    name[3][i as usize],
                );
            }
            if unsafe { (*(*priv_).dapm_routes.add(i as usize)).source.is_null() } {
                return -ENOMEM;
            }

            unsafe {
                (*(*priv_).dapm_routes.add(i as usize)).sink = name[2][i as usize];
            }
        }

        i += 1;
    }

    cpu_pdev = unsafe { of_find_device_by_node(out_cpu_np) };
    if cpu_pdev.is_null() {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                c"failed to find SAI platform device\n".as_ptr(),
            );
        }
        return -EINVAL;
    }
    unsafe {
        put_device(&mut (*cpu_pdev).dev);
    }

    unsafe {
        (*priv_).audmix_pdev = audmix_pdev;
        (*priv_).out_pdev = cpu_pdev;

        (*priv_).card.dai_link = (*priv_).dai;
        (*priv_).card.num_links = (*priv_).num_dai;
        (*priv_).card.codec_conf = (*priv_).dai_conf;
        (*priv_).card.num_configs = (*priv_).num_dai_conf;
        (*priv_).card.dapm_routes = (*priv_).dapm_routes;
        (*priv_).card.num_dapm_routes = (*priv_).num_dapm_routes;
        (*priv_).card.dev = &mut (*pdev).dev;
        (*priv_).card.owner = THIS_MODULE;
        (*priv_).card.name = c"imx-audmix".as_ptr();

        platform_set_drvdata(pdev, &mut (*priv_).card as *mut _ as *mut c_void);
        snd_soc_card_set_drvdata(&mut (*priv_).card, priv_ as *mut c_void);

        ret = devm_snd_soc_register_card(&mut (*pdev).dev, &mut (*priv_).card);
    }
    if ret != 0 {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                c"snd_soc_register_card failed\n".as_ptr(),
            );
        }
        return ret;
    }

    ret
}

static mut imx_audmix_driver: platform_driver = platform_driver {
    probe: Some(imx_audmix_probe),
    driver: device_driver {
        name: c"imx-audmix".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops as *const _ },
    },
};

// module_platform_driver(imx_audmix_driver);
// MODULE_DESCRIPTION("NXP AUDMIX ASoC machine driver");
// MODULE_AUTHOR("Viorel Suman <viorel.suman@nxp.com>");
// MODULE_ALIAS("platform:imx-audmix");
// MODULE_LICENSE("GPL v2");

extern "C" {
    static snd_soc_dummy_dlc: snd_soc_dai_link_component;
    static snd_soc_pm_ops: dev_pm_ops;
    static THIS_MODULE: *mut module;

    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        min: c_uint,
        max: c_uint,
    ) -> c_int;
    fn snd_pcm_hw_constraint_mask64(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        mask: u64,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn params_channels(params: *mut snd_pcm_hw_params) -> u32;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn of_find_device_by_node(np: *mut device_node) -> *mut platform_device;
    fn put_device(dev: *mut device);
    fn of_count_phandle_with_args(
        np: *mut device_node,
        list_name: *const c_char,
        cells_name: *const c_char,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, gfp: c_uint) -> *mut c_void;
    fn of_parse_phandle_with_args(
        np: *mut device_node,
        list_name: *const c_char,
        cells_name: *const c_char,
        index: c_int,
        out_args: *mut of_phandle_args,
    ) -> c_int;
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const FSL_AUDMIX_FORMATS: u64 = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const FSL_SAI_CLK_MAST1: c_int = 0;
const FSL_AUDMIX_MAX_DAIS: c_int = 2;
const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const fn BIT(n: u32) -> c_uint {
    1u32 << n
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    pub full_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub owner: *mut module,
    pub name: *const c_char,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_int,
    pub dapm_routes: *mut snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_cpus: c_int,
    pub num_codecs: c_int,
    pub num_platforms: c_int,
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dynamic: c_uint,
    pub capture_only: c_uint,
    pub playback_only: c_uint,
    pub ignore_pmdown_time: c_uint,
    pub ops: *const snd_soc_ops,
    pub no_pcm: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_phandle_args {
    pub np: *mut device_node,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
