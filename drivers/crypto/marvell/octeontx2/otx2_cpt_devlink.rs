// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2021 Marvell. */

// Translated from otx2_cpt_devlink.c. Definitions supplied by the kernel and
// other source files remain external dependencies.

unsafe fn otx2_cpt_dl_egrp_create(
    dl: *mut devlink,
    _id: u32,
    ctx: *mut devlink_param_gset_ctx,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    let cpt_dl = devlink_priv(dl);
    let cptpf = (*cpt_dl).cptpf;

    otx2_cpt_dl_custom_egrp_create(cptpf, ctx)
}

unsafe fn otx2_cpt_dl_egrp_delete(
    dl: *mut devlink,
    _id: u32,
    ctx: *mut devlink_param_gset_ctx,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    let cpt_dl = devlink_priv(dl);
    let cptpf = (*cpt_dl).cptpf;

    otx2_cpt_dl_custom_egrp_delete(cptpf, ctx)
}

unsafe fn otx2_cpt_dl_uc_info(
    _dl: *mut devlink,
    _id: u32,
    ctx: *mut devlink_param_gset_ctx,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    (*ctx).val.vstr[0] = 0;
    0
}

unsafe fn otx2_cpt_dl_t106_mode_get(
    dl: *mut devlink,
    _id: u32,
    ctx: *mut devlink_param_gset_ctx,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    let cpt_dl = devlink_priv(dl);
    let cptpf = (*cpt_dl).cptpf;
    let pdev = (*cptpf).pdev;
    let mut reg_val: u64 = 0;

    otx2_cpt_read_af_reg(
        &mut (*cptpf).afpf_mbox,
        pdev,
        CPT_AF_CTL,
        &mut reg_val,
        BLKADDR_CPT0,
    );
    (*ctx).val.vu8 = ((reg_val >> 18) & 0x1) as u8;
    0
}

unsafe fn otx2_cpt_dl_t106_mode_set(
    dl: *mut devlink,
    _id: u32,
    ctx: *mut devlink_param_gset_ctx,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    let cpt_dl = devlink_priv(dl);
    let cptpf = (*cpt_dl).cptpf;
    let pdev = (*cptpf).pdev;
    let mut reg_val: u64 = 0;

    if (*cptpf).enabled_vfs != 0 || (*cptpf).eng_grps.is_grps_created {
        return -EPERM;
    }

    if cpt_feature_sgv2(pdev) {
        otx2_cpt_read_af_reg(
            &mut (*cptpf).afpf_mbox,
            pdev,
            CPT_AF_CTL,
            &mut reg_val,
            BLKADDR_CPT0,
        );
        reg_val &= !(0x1u64 << 18);
        reg_val |= (((*ctx).val.vu8 as u64) & 0x1) << 18;
        return otx2_cpt_write_af_reg(
            &mut (*cptpf).afpf_mbox,
            pdev,
            CPT_AF_CTL,
            reg_val,
            BLKADDR_CPT0,
        );
    }

    0
}

#[repr(i32)]
enum otx2_cpt_dl_param_id {
    OTX2_CPT_DEVLINK_PARAM_ID_BASE = DEVLINK_PARAM_GENERIC_ID_MAX,
    OTX2_CPT_DEVLINK_PARAM_ID_EGRP_CREATE,
    OTX2_CPT_DEVLINK_PARAM_ID_EGRP_DELETE,
    OTX2_CPT_DEVLINK_PARAM_ID_T106_MODE,
}

static otx2_cpt_dl_params: [devlink_param; 3] = [
    DEVLINK_PARAM_DRIVER!(
        OTX2_CPT_DEVLINK_PARAM_ID_EGRP_CREATE,
        "egrp_create",
        DEVLINK_PARAM_TYPE_STRING,
        BIT(DEVLINK_PARAM_CMODE_RUNTIME),
        otx2_cpt_dl_uc_info,
        otx2_cpt_dl_egrp_create,
        None,
    ),
    DEVLINK_PARAM_DRIVER!(
        OTX2_CPT_DEVLINK_PARAM_ID_EGRP_DELETE,
        "egrp_delete",
        DEVLINK_PARAM_TYPE_STRING,
        BIT(DEVLINK_PARAM_CMODE_RUNTIME),
        otx2_cpt_dl_uc_info,
        otx2_cpt_dl_egrp_delete,
        None,
    ),
    DEVLINK_PARAM_DRIVER!(
        OTX2_CPT_DEVLINK_PARAM_ID_T106_MODE,
        "t106_mode",
        DEVLINK_PARAM_TYPE_U8,
        BIT(DEVLINK_PARAM_CMODE_RUNTIME),
        otx2_cpt_dl_t106_mode_get,
        otx2_cpt_dl_t106_mode_set,
        None,
    ),
];

unsafe fn otx2_cpt_dl_info_firmware_version_put(
    req: *mut devlink_info_req,
    grp: *mut otx2_cpt_eng_grp_info,
    ver_name: *const i8,
    eng_type: i32,
) -> i32 {
    let mut eng: *mut otx2_cpt_engs_rsvd;

    for i in 0..OTX2_CPT_MAX_ENGINE_GROUPS {
        eng = find_engines_by_type(&mut *grp.add(i as usize), eng_type);
        if !eng.is_null() {
            return devlink_info_version_running_put(req, ver_name, (*(*eng).ucode).ver_str);
        }
    }

    0
}

unsafe fn otx2_cpt_devlink_info_get(
    dl: *mut devlink,
    req: *mut devlink_info_req,
    _extack: *mut netlink_ext_ack,
) -> i32 {
    let cpt_dl = devlink_priv(dl);
    let cptpf = (*cpt_dl).cptpf;
    let mut err: i32;

    err = otx2_cpt_dl_info_firmware_version_put(
        req, (*cptpf).eng_grps.grp, c"fw.ae".as_ptr(), OTX2_CPT_AE_TYPES,
    );
    if err != 0 { return err; }
    err = otx2_cpt_dl_info_firmware_version_put(
        req, (*cptpf).eng_grps.grp, c"fw.se".as_ptr(), OTX2_CPT_SE_TYPES,
    );
    if err != 0 { return err; }
    otx2_cpt_dl_info_firmware_version_put(
        req, (*cptpf).eng_grps.grp, c"fw.ie".as_ptr(), OTX2_CPT_IE_TYPES,
    )
}

static otx2_cpt_devlink_ops: devlink_ops = devlink_ops {
    info_get: Some(otx2_cpt_devlink_info_get),
};

unsafe fn otx2_cpt_register_dl(cptpf: *mut otx2_cptpf_dev) -> i32 {
    let dev = &mut (*(*cptpf).pdev).dev;
    let dl = devlink_alloc(&otx2_cpt_devlink_ops, core::mem::size_of::<otx2_cpt_devlink>(), dev);
    if dl.is_null() {
        dev_warn(dev, "devlink_alloc failed\n");
        return -ENOMEM;
    }
    let cpt_dl = devlink_priv(dl);
    (*cpt_dl).dl = dl;
    (*cpt_dl).cptpf = cptpf;
    (*cptpf).dl = dl;
    let ret = devlink_params_register(dl, otx2_cpt_dl_params.as_ptr(), otx2_cpt_dl_params.len());
    if ret != 0 {
        dev_err(dev, "devlink params register failed with error %d", ret);
        devlink_free(dl);
        return ret;
    }
    devlink_register(dl);
    0
}

unsafe fn otx2_cpt_unregister_dl(cptpf: *mut otx2_cptpf_dev) {
    let dl = (*cptpf).dl;
    if dl.is_null() { return; }
    devlink_unregister(dl);
    devlink_params_unregister(dl, otx2_cpt_dl_params.as_ptr(), otx2_cpt_dl_params.len());
    devlink_free(dl);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
