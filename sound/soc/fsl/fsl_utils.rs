// SPDX-License-Identifier: GPL-2.0
//
// Freescale ALSA SoC Machine driver utility
//
// Author: Timur Tabi <timur@freescale.com>
//
// Copyright 2010 Freescale Semiconductor, Inc.

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

const EINVAL: c_int = 22;
const DAI_NAME_SIZE: usize = 32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: u64,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub platforms: *mut snd_soc_dai_link_component,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *mut c_int,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

unsafe extern "C" {
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_device_is_compatible(device: *mut device_node, compat: *const c_char) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn of_address_to_resource(
        dev: *mut device_node,
        index: c_int,
        r: *mut resource,
    ) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn of_get_property(
        node: *mut device_node,
        name: *const c_char,
        lenp: *mut c_int,
    ) -> *const u32;
    fn be32_to_cpup(p: *const u32) -> u32;
    fn of_get_parent(node: *mut device_node) -> *mut device_node;

    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn clk_get_parent(clk: *mut clk) -> *mut clk;
    fn clk_is_match(p: *mut clk, q: *mut clk) -> bool;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn clk_get_rate(clk: *mut clk) -> c_ulonglong;

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn snd_soc_get_xr_sx(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn snd_soc_put_xr_sx(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn snd_soc_get_enum_double(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn snd_soc_put_enum_double(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn snd_soc_get_volsw(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn snd_soc_put_volsw(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
}

/**
 * fsl_asoc_get_dma_channel - determine the dma channel for a SSI node
 *
 * @ssi_np: pointer to the SSI device tree node
 * @name: name of the phandle pointing to the dma channel
 * @dai: ASoC DAI link pointer to be filled with platform_name
 * @dma_channel_id: dma channel id to be returned
 * @dma_id: dma id to be returned
 *
 * This function determines the dma and channel id for given SSI node.  It
 * also discovers the platform_name for the ASoC DAI link.
 */
#[no_mangle]
pub unsafe extern "C" fn fsl_asoc_get_dma_channel(
    ssi_np: *mut device_node,
    name: *const c_char,
    dai: *mut snd_soc_dai_link,
    dma_channel_id: *mut c_uint,
    dma_id: *mut c_uint,
) -> c_int {
    let mut res = resource { start: 0 };
    let dma_channel_np: *mut device_node;
    let dma_np: *mut device_node;
    let mut iprop: *const u32;
    let ret: c_int;

    dma_channel_np = of_parse_phandle(ssi_np, name, 0);
    if dma_channel_np.is_null() {
        return -EINVAL;
    }

    if !of_device_is_compatible(dma_channel_np, c"fsl,ssi-dma-channel".as_ptr()).ne(&0) {
        of_node_put(dma_channel_np);
        return -EINVAL;
    }

    /* Determine the dev_name for the device_node.  This code mimics the
     * behavior of of_device_make_bus_id(). We need this because ASoC uses
     * the dev_name() of the device to match the platform (DMA) device with
     * the CPU (SSI) device.  It's all ugly and hackish, but it works (for
     * now).
     *
     * dai->platform name should already point to an allocated buffer.
     */
    ret = of_address_to_resource(dma_channel_np, 0, &mut res);
    if ret != 0 {
        of_node_put(dma_channel_np);
        return ret;
    }
    snprintf(
        (*(*dai).platforms).name as *mut c_char,
        DAI_NAME_SIZE,
        c"%llx.%pOFn".as_ptr(),
        res.start as c_ulonglong,
        dma_channel_np,
    );

    iprop = of_get_property(dma_channel_np, c"cell-index".as_ptr(), core::ptr::null_mut());
    if iprop.is_null() {
        of_node_put(dma_channel_np);
        return -EINVAL;
    }
    *dma_channel_id = be32_to_cpup(iprop);

    dma_np = of_get_parent(dma_channel_np);
    iprop = of_get_property(dma_np, c"cell-index".as_ptr(), core::ptr::null_mut());
    if iprop.is_null() {
        of_node_put(dma_np);
        of_node_put(dma_channel_np);
        return -EINVAL;
    }
    *dma_id = be32_to_cpup(iprop);

    of_node_put(dma_np);
    of_node_put(dma_channel_np);

    0
}

/**
 * fsl_asoc_get_pll_clocks - get two PLL clock source
 *
 * @dev: device pointer
 * @pll8k_clk: PLL clock pointer for 8kHz
 * @pll11k_clk: PLL clock pointer for 11kHz
 *
 * This function get two PLL clock source
 */
#[no_mangle]
pub unsafe extern "C" fn fsl_asoc_get_pll_clocks(
    dev: *mut device,
    pll8k_clk: *mut *mut clk,
    pll11k_clk: *mut *mut clk,
) {
    *pll8k_clk = devm_clk_get(dev, c"pll8k".as_ptr());
    if IS_ERR(*pll8k_clk as *const c_void) {
        *pll8k_clk = core::ptr::null_mut();
    }

    *pll11k_clk = devm_clk_get(dev, c"pll11k".as_ptr());
    if IS_ERR(*pll11k_clk as *const c_void) {
        *pll11k_clk = core::ptr::null_mut();
    }
}

/**
 * fsl_asoc_reparent_pll_clocks - set clock parent if necessary
 *
 * @dev: device pointer
 * @clk: root clock pointer
 * @pll8k_clk: PLL clock pointer for 8kHz
 * @pll11k_clk: PLL clock pointer for 11kHz
 * @ratio: target requency for root clock
 *
 * This function set root clock parent according to the target ratio
 */
#[no_mangle]
pub unsafe extern "C" fn fsl_asoc_reparent_pll_clocks(
    dev: *mut device,
    clk: *mut clk,
    pll8k_clk: *mut clk,
    pll11k_clk: *mut clk,
    ratio: u64,
) {
    let mut p: *mut clk;
    let mut pll: *mut clk = core::ptr::null_mut();
    let mut npll: *mut clk = core::ptr::null_mut();
    let reparent: bool;
    let ret: c_int;

    if clk.is_null() || pll8k_clk.is_null() || pll11k_clk.is_null() {
        return;
    }

    p = clk;
    while !p.is_null() && !pll8k_clk.is_null() && !pll11k_clk.is_null() {
        let pp = clk_get_parent(p);

        if clk_is_match(pp, pll8k_clk) || clk_is_match(pp, pll11k_clk) {
            pll = pp;
            break;
        }
        p = pp;
    }

    npll = if ratio % 8000 != 0 { pll11k_clk } else { pll8k_clk };
    reparent = !pll.is_null() && !clk_is_match(pll, npll);

    if reparent {
        ret = clk_set_parent(p, npll);
        if ret < 0 {
            dev_warn(dev, c"failed to set parent:%d\n".as_ptr(), ret);
        }
    }
}

/**
 * fsl_asoc_constrain_rates - constrain rates according to clocks
 *
 * @target_constr: target constraint
 * @original_constr: original constraint
 * @pll8k_clk: PLL clock pointer for 8kHz
 * @pll11k_clk: PLL clock pointer for 11kHz
 * @ext_clk: External clock pointer
 * @target_rates: target rates array
 *
 * This function constrain rates according to clocks
 */
#[no_mangle]
pub unsafe extern "C" fn fsl_asoc_constrain_rates(
    target_constr: *mut snd_pcm_hw_constraint_list,
    original_constr: *const snd_pcm_hw_constraint_list,
    pll8k_clk: *mut clk,
    pll11k_clk: *mut clk,
    ext_clk: *mut clk,
    target_rates: *mut c_int,
) {
    let mut i: c_uint;
    let mut j: usize;
    let mut k: isize = 0;
    let mut clk_rate: [u64; 3] = [0; 3];

    *target_constr = *original_constr;
    if !pll8k_clk.is_null() || !pll11k_clk.is_null() || !ext_clk.is_null() {
        (*target_constr).list = target_rates;
        (*target_constr).count = 0;
        i = 0;
        while i < (*original_constr).count {
            clk_rate[0] = clk_get_rate(pll8k_clk) as u64;
            clk_rate[1] = clk_get_rate(pll11k_clk) as u64;
            clk_rate[2] = clk_get_rate(ext_clk) as u64;
            j = 0;
            while j < 3 {
                let rate = *((*original_constr).list).offset(i as isize);
                if clk_rate[j] != 0 && clk_rate[j] % rate as u64 == 0 {
                    *target_rates.offset(k) = rate;
                    k += 1;
                    (*target_constr).count += 1;
                    break;
                }
                j += 1;
            }
            i += 1;
        }

        /* protection for if there is no proper rate found*/
        if (*target_constr).count == 0 {
            *target_constr = *original_constr;
        }
    }
}

/*
 * Below functions are used by mixer interface to avoid accessing registers
 * which are volatile at pm runtime suspend state (cache_only is enabled).
 */
#[no_mangle]
pub unsafe extern "C" fn fsl_asoc_get_xr_sx(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let mut ret: c_int = 0;

    ret = pm_runtime_resume_and_get((*component).dev);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_get_xr_sx(kcontrol, ucontrol);

    pm_runtime_put_autosuspend((*component).dev);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn fsl_asoc_put_xr_sx(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let mut ret: c_int = 0;

    ret = pm_runtime_resume_and_get((*component).dev);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_put_xr_sx(kcontrol, ucontrol);
    /*
     * As this function only used by the SNDRV_CTL_ELEM_ACCESS_VOLATILE
     * case. return 0 to avoid control event notification.
     */
    if ret > 0 {
        ret = 0;
    }

    pm_runtime_put_autosuspend((*component).dev);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn fsl_asoc_get_enum_double(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let mut ret: c_int = 0;

    ret = pm_runtime_resume_and_get((*component).dev);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_get_enum_double(kcontrol, ucontrol);

    pm_runtime_put_autosuspend((*component).dev);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn fsl_asoc_put_enum_double(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let mut ret: c_int = 0;

    ret = pm_runtime_resume_and_get((*component).dev);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_put_enum_double(kcontrol, ucontrol);
    /*
     * As this function only used by the SNDRV_CTL_ELEM_ACCESS_VOLATILE
     * case. return 0 to avoid control event notification.
     */
    if ret > 0 {
        ret = 0;
    }

    pm_runtime_put_autosuspend((*component).dev);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn fsl_asoc_get_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let mut ret: c_int = 0;

    ret = pm_runtime_resume_and_get((*component).dev);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_get_volsw(kcontrol, ucontrol);

    pm_runtime_put_autosuspend((*component).dev);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn fsl_asoc_put_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let mut ret: c_int = 0;

    ret = pm_runtime_resume_and_get((*component).dev);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_put_volsw(kcontrol, ucontrol);
    /*
     * As this function only used by the SNDRV_CTL_ELEM_ACCESS_VOLATILE
     * case. return 0 to avoid control event notification.
     */
    if ret > 0 {
        ret = 0;
    }

    pm_runtime_put_autosuspend((*component).dev);

    ret
}

// MODULE_AUTHOR("Timur Tabi <timur@freescale.com>");
// MODULE_DESCRIPTION("Freescale ASoC utility code");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
