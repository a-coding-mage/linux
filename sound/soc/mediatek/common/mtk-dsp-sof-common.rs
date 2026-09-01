// SPDX-License-Identifier: GPL-2.0
/*
 * mtk-dsp-sof-common.c  --  MediaTek dsp sof common ctrl
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Chunxu Li <chunxu.li@mediatek.com>
 */

/*
 * Translated from the implementation source. C header dependencies:
 * "mtk-dsp-sof-common.h"
 * "mtk-soc-card.h"
 *
 * The Linux/ASoC types, constants, list helpers, allocation helpers, logging
 * helpers, DAPM iteration helpers, and exported-symbol machinery referenced
 * below are supplied by external bindings in the final repository context.
 */

extern "C" {
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut core::ffi::c_void;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dai_stream_active(cpu_dai: *mut snd_soc_dai, stream: core::ffi::c_int) -> core::ffi::c_int;
    fn snd_soc_rtdcom_lookup(
        rtd: *mut snd_soc_pcm_runtime,
        name: *const core::ffi::c_char,
    ) -> *mut snd_soc_component;
    fn snd_soc_dai_get_widget(
        dai: *mut snd_soc_dai,
        stream: core::ffi::c_int,
    ) -> *mut snd_soc_dapm_widget;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn devm_kzalloc(
        dev: *mut device,
        size: usize,
        flags: core::ffi::c_uint,
    ) -> *mut core::ffi::c_void;
    fn devm_kcalloc(
        dev: *mut device,
        n: usize,
        size: usize,
        flags: core::ffi::c_uint,
    ) -> *mut core::ffi::c_void;
    fn of_property_count_strings(np: *mut device_node, propname: *const core::ffi::c_char) -> core::ffi::c_int;
    fn of_property_read_string_index(
        np: *mut device_node,
        propname: *const core::ffi::c_char,
        index: core::ffi::c_int,
        output: *mut *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn strcmp(s1: *const core::ffi::c_char, s2: *const core::ffi::c_char) -> core::ffi::c_int;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn memset(dst: *mut core::ffi::c_void, c: core::ffi::c_int, n: usize) -> *mut core::ffi::c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

const GFP_KERNEL: core::ffi::c_uint = 0;
const ENOMEM: core::ffi::c_int = 12;
const EINVAL: core::ffi::c_int = 22;
const SNDRV_PCM_STREAM_PLAYBACK: core::ffi::c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: core::ffi::c_int = 1;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

pub type BeHwParamsFixup = Option<
    unsafe extern "C" fn(
        rtd: *mut snd_soc_pcm_runtime,
        params: *mut snd_pcm_hw_params,
    ) -> core::ffi::c_int,
>;

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: core::ffi::c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const core::ffi::c_char,
    pub stream_name: *const core::ffi::c_char,
    pub no_pcm: bool,
    pub be_hw_params_fixup: BeHwParamsFixup,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_component {
    pub driver: *mut snd_soc_component_driver,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub be_hw_params_fixup: BeHwParamsFixup,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const core::ffi::c_char,
    pub control: *const core::ffi::c_char,
    pub source: *const core::ffi::c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_path {
    pub source: *mut snd_soc_dapm_widget,
    pub sink: *mut snd_soc_dapm_widget,
}

#[repr(C)]
pub struct snd_soc_dpcm {
    pub fe: *mut snd_soc_pcm_runtime,
    pub be: *mut snd_soc_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_soc_card_data {
    pub sof_priv: *const mtk_sof_priv,
    pub sof_dai_link_list: list_head,
}

#[repr(C)]
pub struct mtk_sof_priv {
    pub num_streams: core::ffi::c_int,
    pub conn_streams: *const sof_conn_stream,
    pub sof_dai_link_fixup: BeHwParamsFixup,
}

#[repr(C)]
pub struct sof_conn_stream {
    pub normal_link: *const core::ffi::c_char,
    pub sof_link: *const core::ffi::c_char,
    pub stream_dir: core::ffi::c_int,
    pub sof_dma: *const core::ffi::c_char,
}

#[repr(C)]
pub struct mtk_dai_link {
    pub list: list_head,
    pub name: *const core::ffi::c_char,
    pub be_hw_params_fixup: BeHwParamsFixup,
}

unsafe fn for_each_card_rtds<F>(_card: *mut snd_soc_card, _f: F)
where
    F: FnMut(*mut snd_soc_pcm_runtime),
{
    todo!("external for_each_card_rtds iteration supplied by ASoC bindings")
}

unsafe fn for_each_card_prelinks<F>(_card: *mut snd_soc_card, _f: F)
where
    F: FnMut(core::ffi::c_int, *mut snd_soc_dai_link),
{
    todo!("external for_each_card_prelinks iteration supplied by ASoC bindings")
}

unsafe fn for_each_rtd_cpu_dais<F>(_rtd: *mut snd_soc_pcm_runtime, _f: F)
where
    F: FnMut(core::ffi::c_int, *mut snd_soc_dai),
{
    todo!("external for_each_rtd_cpu_dais iteration supplied by ASoC bindings")
}

unsafe fn for_each_pcm_streams<F>(_f: F)
where
    F: FnMut(core::ffi::c_int),
{
    todo!("external for_each_pcm_streams iteration supplied by ASoC bindings")
}

unsafe fn for_each_dpcm_fe<F>(_rtd: *mut snd_soc_pcm_runtime, _stream: core::ffi::c_int, _f: F)
where
    F: FnMut(*mut snd_soc_dpcm),
{
    todo!("external for_each_dpcm_fe iteration supplied by ASoC bindings")
}

unsafe fn for_each_dpcm_be<F>(_fe: *mut snd_soc_pcm_runtime, _stream: core::ffi::c_int, _f: F)
where
    F: FnMut(*mut snd_soc_dpcm),
{
    todo!("external for_each_dpcm_be iteration supplied by ASoC bindings")
}

unsafe fn list_for_each_mtk_dai_link<F>(_head: *mut list_head, _f: F)
where
    F: FnMut(*mut mtk_dai_link),
{
    todo!("external list_for_each_entry(dai_link, head, list) supplied by Linux list bindings")
}

unsafe fn snd_soc_dapm_widget_for_each_sink_path<F>(_widget: *mut snd_soc_dapm_widget, _f: F)
where
    F: FnMut(*mut snd_soc_dapm_path),
{
    todo!("external snd_soc_dapm_widget_for_each_sink_path iteration supplied by ASoC bindings")
}

unsafe fn snd_soc_dapm_widget_for_each_source_path<F>(_widget: *mut snd_soc_dapm_widget, _f: F)
where
    F: FnMut(*mut snd_soc_dapm_path),
{
    todo!("external snd_soc_dapm_widget_for_each_source_path iteration supplied by ASoC bindings")
}

/* fixup the BE DAI link to match any values from topology */
#[no_mangle]
pub unsafe extern "C" fn mtk_sof_dai_link_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> core::ffi::c_int {
    let card = (*rtd).card;
    let soc_card_data = snd_soc_card_get_drvdata(card) as *mut mtk_soc_card_data;
    let sof_priv = (*soc_card_data).sof_priv;
    let mut ret: core::ffi::c_int = 0;

    for i in 0..(*sof_priv).num_streams {
        let mut runtime: *mut snd_soc_pcm_runtime = core::ptr::null_mut();
        let mut sof_dai_link: *mut snd_soc_dai_link = core::ptr::null_mut();
        let conn = (*sof_priv).conn_streams.add(i as usize);

        if !(*conn).normal_link.is_null() && strcmp((*(*rtd).dai_link).name, (*conn).normal_link) != 0 {
            continue;
        }

        let mut found_runtime = false;
        for_each_card_rtds(card, |candidate_runtime| {
            if found_runtime {
                return;
            }

            if strcmp((*(*candidate_runtime).dai_link).name, (*conn).sof_link) != 0 {
                return;
            }

            runtime = candidate_runtime;
            let mut found_cpu_dai = false;
            for_each_rtd_cpu_dais(candidate_runtime, |_, cpu_dai| {
                if found_cpu_dai {
                    return;
                }

                if snd_soc_dai_stream_active(cpu_dai, (*conn).stream_dir) > 0 {
                    sof_dai_link = (*candidate_runtime).dai_link;
                    found_cpu_dai = true;
                }
            });
            found_runtime = true;
        });

        if !sof_dai_link.is_null() {
            if let Some(be_hw_params_fixup) = (*sof_dai_link).be_hw_params_fixup {
                ret = be_hw_params_fixup(runtime, params);
            }
        }

        break;
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn mtk_sof_card_probe(card: *mut snd_soc_card) -> core::ffi::c_int {
    let soc_card_data = snd_soc_card_get_drvdata(card) as *mut mtk_soc_card_data;

    /* Set stream_name to help sof bind widgets */
    for_each_card_prelinks(card, |_, dai_link| {
        if (*dai_link).no_pcm && (*dai_link).stream_name.is_null() && !(*dai_link).name.is_null() {
            (*dai_link).stream_name = (*dai_link).name;
        }
    });

    INIT_LIST_HEAD(core::ptr::addr_of_mut!((*soc_card_data).sof_dai_link_list));

    0
}

unsafe fn mtk_sof_find_tplg_be(rtd: *mut snd_soc_pcm_runtime) -> *mut snd_soc_pcm_runtime {
    let card = (*rtd).card;
    let soc_card_data = snd_soc_card_get_drvdata(card) as *mut mtk_soc_card_data;
    let sof_priv = (*soc_card_data).sof_priv;
    let mut found_be: *mut snd_soc_pcm_runtime = core::ptr::null_mut();

    for_each_pcm_streams(|stream| {
        if !found_be.is_null() {
            return;
        }

        let mut fe: *mut snd_soc_pcm_runtime = core::ptr::null_mut();
        for_each_dpcm_fe(rtd, stream, |dpcm| {
            if fe.is_null() {
                fe = (*dpcm).fe;
            }
        });

        if fe.is_null() {
            return;
        }

        for_each_dpcm_be(fe, stream, |dpcm| {
            if !found_be.is_null() {
                return;
            }

            let be = (*dpcm).be;
            if be == rtd {
                return;
            }

            for i in 0..(*sof_priv).num_streams {
                let conn = (*sof_priv).conn_streams.add(i as usize);

                if strcmp((*(*be).dai_link).name, (*conn).sof_link) == 0 {
                    found_be = be;
                    break;
                }
            }
        });
    });

    found_be
}

/* fixup the BE DAI link to match any values from topology */
unsafe extern "C" fn mtk_sof_check_tplg_be_dai_link_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> core::ffi::c_int {
    let card = (*rtd).card;
    let soc_card_data = snd_soc_card_get_drvdata(card) as *mut mtk_soc_card_data;
    let sof_priv = (*soc_card_data).sof_priv;
    let sof_be: *mut snd_soc_pcm_runtime;
    let mut ret: core::ffi::c_int = 0;

    sof_be = mtk_sof_find_tplg_be(rtd);
    if !sof_be.is_null() {
        if let Some(sof_dai_link_fixup) = (*sof_priv).sof_dai_link_fixup {
            ret = sof_dai_link_fixup(rtd, params);
        } else if let Some(be_hw_params_fixup) = (*(*sof_be).dai_link).be_hw_params_fixup {
            ret = be_hw_params_fixup(sof_be, params);
        }
    } else {
        list_for_each_mtk_dai_link(
            core::ptr::addr_of_mut!((*soc_card_data).sof_dai_link_list),
            |dai_link| {
                if strcmp((*dai_link).name, (*(*rtd).dai_link).name) == 0 {
                    if let Some(be_hw_params_fixup) = (*dai_link).be_hw_params_fixup {
                        ret = be_hw_params_fixup(rtd, params);
                    }
                }
            },
        );
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn mtk_sof_card_late_probe(card: *mut snd_soc_card) -> core::ffi::c_int {
    let dapm = snd_soc_card_to_dapm(card);
    let mut sof_comp: *mut snd_soc_component = core::ptr::null_mut();
    let soc_card_data = snd_soc_card_get_drvdata(card) as *mut mtk_soc_card_data;
    let sof_priv = (*soc_card_data).sof_priv;

    /* 1. find sof component */
    for_each_card_rtds(card, |rtd| {
        if sof_comp.is_null() {
            sof_comp = snd_soc_rtdcom_lookup(rtd, b"sof-audio-component\0".as_ptr() as *const _);
        }
    });

    if sof_comp.is_null() {
        dev_info((*card).dev, b"probe without sof-audio-component\n\0".as_ptr() as *const _);
        return 0;
    }

    /* 2. overwrite all BE fixups, and backup the existing fixup */
    let mut alloc_failed = false;
    for_each_card_prelinks(card, |_, dai_link| {
        if alloc_failed {
            return;
        }

        if (*dai_link).be_hw_params_fixup.is_some() {
            let mtk_dai_link = devm_kzalloc(
                (*card).dev,
                core::mem::size_of::<mtk_dai_link>(),
                GFP_KERNEL,
            ) as *mut mtk_dai_link;
            if mtk_dai_link.is_null() {
                alloc_failed = true;
                return;
            }

            (*mtk_dai_link).be_hw_params_fixup = (*dai_link).be_hw_params_fixup;
            (*mtk_dai_link).name = (*dai_link).name;

            list_add(
                core::ptr::addr_of_mut!((*mtk_dai_link).list),
                core::ptr::addr_of_mut!((*soc_card_data).sof_dai_link_list),
            );
        }

        if (*dai_link).no_pcm {
            (*dai_link).be_hw_params_fixup = Some(mtk_sof_check_tplg_be_dai_link_fixup);
        }
    });
    if alloc_failed {
        return -ENOMEM;
    }

    /* 3. add route path and SOF_BE fixup callback */
    for i in 0..(*sof_priv).num_streams {
        let conn = (*sof_priv).conn_streams.add(i as usize);
        let mut sof_rtd: *mut snd_soc_pcm_runtime = core::ptr::null_mut();

        for_each_card_rtds(card, |rtd| {
            if sof_rtd.is_null() && strcmp((*(*rtd).dai_link).name, (*conn).sof_link) == 0 {
                sof_rtd = rtd;
            }
        });

        if !sof_rtd.is_null() {
            for_each_rtd_cpu_dais(sof_rtd, |_, cpu_dai| {
                let mut route: snd_soc_dapm_route = core::mem::zeroed();
                let widget = snd_soc_dai_get_widget(cpu_dai, (*conn).stream_dir);

                memset(
                    core::ptr::addr_of_mut!(route) as *mut core::ffi::c_void,
                    0,
                    core::mem::size_of::<snd_soc_dapm_route>(),
                );
                if (*conn).stream_dir == SNDRV_PCM_STREAM_CAPTURE && !widget.is_null() {
                    snd_soc_dapm_widget_for_each_sink_path(widget, |p| {
                        route.source = (*conn).sof_dma;
                        route.sink = (*(*p).sink).name;
                        snd_soc_dapm_add_routes(dapm, core::ptr::addr_of!(route), 1);
                    });
                } else if (*conn).stream_dir == SNDRV_PCM_STREAM_PLAYBACK && !widget.is_null() {
                    snd_soc_dapm_widget_for_each_source_path(widget, |p| {
                        route.source = (*(*p).source).name;
                        route.sink = (*conn).sof_dma;
                        snd_soc_dapm_add_routes(dapm, core::ptr::addr_of!(route), 1);
                    });
                } else {
                    dev_err((*cpu_dai).dev, b"stream dir and widget not pair\n\0".as_ptr() as *const _);
                }
            });

            /* overwrite SOF BE fixup */
            (*(*sof_rtd).dai_link).be_hw_params_fixup = (*(*sof_comp).driver).be_hw_params_fixup;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mtk_sof_dailink_parse_of(
    dev: *mut device,
    card: *mut snd_soc_card,
    propname: *const core::ffi::c_char,
) -> core::ffi::c_int {
    let np = (*dev).of_node;
    let mut dai_name: *const core::ffi::c_char = core::ptr::null();
    let mut parsed_num_links: core::ffi::c_int = 0;

    let num_links = of_property_count_strings(np, b"mediatek,dai-link\0".as_ptr() as *const _);
    if num_links < 0 || num_links > (*card).num_links {
        dev_dbg(dev, b"number of dai-link is invalid\n\0".as_ptr() as *const _);
        return -EINVAL;
    }

    let parsed_dai_link = devm_kcalloc(
        dev,
        num_links as usize,
        core::mem::size_of::<snd_soc_dai_link>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link;
    if parsed_dai_link.is_null() {
        return -ENOMEM;
    }

    for i in 0..num_links {
        let ret = of_property_read_string_index(np, propname, i, core::ptr::addr_of_mut!(dai_name));
        if ret != 0 {
            dev_dbg(
                dev,
                b"ASoC: Property '%s' index %d could not be read: %d\n\0".as_ptr() as *const _,
                propname,
                i,
                ret,
            );
            return ret;
        }
        dev_dbg(dev, b"ASoC: Property get dai_name:%s\n\0".as_ptr() as *const _, dai_name);
        for_each_card_prelinks(card, |_, dai_link| {
            if strcmp(dai_name, (*dai_link).name) == 0 {
                memcpy(
                    parsed_dai_link.add(parsed_num_links as usize) as *mut core::ffi::c_void,
                    dai_link as *const core::ffi::c_void,
                    core::mem::size_of::<snd_soc_dai_link>(),
                );
                parsed_num_links += 1;
            }
        });
    }

    if parsed_num_links != num_links {
        return -EINVAL;
    }

    (*card).dai_link = parsed_dai_link;
    (*card).num_links = parsed_num_links;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
