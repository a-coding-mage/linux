// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2022, The Linux Foundation. All rights reserved.

// C dependencies:
// linux/cleanup.h, linux/export.h, linux/module.h, linux/init.h, linux/of.h,
// linux/platform_device.h, linux/pm_domain.h, linux/pm_runtime.h
// "lpass-macro-common.h"

static lpass_codec_mutex: DEFINE_MUTEX = DEFINE_MUTEX::new();
static mut lpass_codec_version: lpass_codec_version = 0 as lpass_codec_version;

pub unsafe extern "C" fn lpass_macro_pds_init(dev: *mut device) -> *mut lpass_macro {
    let mut l_pds: *mut lpass_macro;
    let mut ret: c_int;

    if !of_property_present((*dev).of_node, c_str!("power-domains")) {
        return core::ptr::null_mut();
    }

    l_pds = devm_kzalloc(
        dev,
        core::mem::size_of::<lpass_macro>(),
        GFP_KERNEL,
    ) as *mut lpass_macro;
    if l_pds.is_null() {
        return ERR_PTR(-ENOMEM) as *mut lpass_macro;
    }

    (*l_pds).macro_pd = dev_pm_domain_attach_by_name(dev, c_str!("macro"));
    if IS_ERR_OR_NULL((*l_pds).macro_pd) {
        ret = if !(*l_pds).macro_pd.is_null() {
            PTR_ERR((*l_pds).macro_pd)
        } else {
            -ENODATA
        };
        return ERR_PTR(ret) as *mut lpass_macro;
    }

    ret = pm_runtime_resume_and_get((*l_pds).macro_pd);
    if ret < 0 {
        dev_pm_domain_detach((*l_pds).macro_pd, false);
        return ERR_PTR(ret) as *mut lpass_macro;
    }

    (*l_pds).dcodec_pd = dev_pm_domain_attach_by_name(dev, c_str!("dcodec"));
    if IS_ERR_OR_NULL((*l_pds).dcodec_pd) {
        ret = if !(*l_pds).dcodec_pd.is_null() {
            PTR_ERR((*l_pds).dcodec_pd)
        } else {
            -ENODATA
        };
        pm_runtime_put((*l_pds).macro_pd);
        dev_pm_domain_detach((*l_pds).macro_pd, false);
        return ERR_PTR(ret) as *mut lpass_macro;
    }

    ret = pm_runtime_resume_and_get((*l_pds).dcodec_pd);
    if ret < 0 {
        dev_pm_domain_detach((*l_pds).dcodec_pd, false);
        pm_runtime_put((*l_pds).macro_pd);
        dev_pm_domain_detach((*l_pds).macro_pd, false);
        return ERR_PTR(ret) as *mut lpass_macro;
    }

    l_pds
}
EXPORT_SYMBOL_GPL!(lpass_macro_pds_init);

pub unsafe extern "C" fn lpass_macro_pds_exit(pds: *mut lpass_macro) {
    if !pds.is_null() {
        pm_runtime_put((*pds).macro_pd);
        dev_pm_domain_detach((*pds).macro_pd, false);
        pm_runtime_put((*pds).dcodec_pd);
        dev_pm_domain_detach((*pds).dcodec_pd, false);
    }
}
EXPORT_SYMBOL_GPL!(lpass_macro_pds_exit);

pub unsafe extern "C" fn lpass_macro_set_codec_version(version: lpass_codec_version) {
    let _guard = lpass_codec_mutex.lock();
    lpass_codec_version = version;
}
EXPORT_SYMBOL_GPL!(lpass_macro_set_codec_version);

pub unsafe extern "C" fn lpass_macro_get_codec_version() -> lpass_codec_version {
    let _guard = lpass_codec_mutex.lock();

    lpass_codec_version
}
EXPORT_SYMBOL_GPL!(lpass_macro_get_codec_version);

MODULE_DESCRIPTION!("Common macro driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
