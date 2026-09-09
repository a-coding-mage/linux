// SPDX-License-Identifier: GPL-2.0-only

// C dependencies supplied by the surrounding ethtool translation.

#[repr(C)]
pub struct wol_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct wol_reply_data {
    pub base: ethnl_reply_data,
    pub wol: ethtool_wolinfo,
    pub show_sopass: bool,
}

// Equivalent to container_of(__reply_base, struct wol_reply_data, base).
#[inline]
unsafe fn wol_repdata(reply_base: *const ethnl_reply_data) -> *mut wol_reply_data {
    reply_base.cast::<wol_reply_data>()
}

pub static ethnl_wol_get_policy: [nla_policy; ETHTOOL_A_WOL_HEADER + 1] = [
    // ETHTOOL_A_WOL_HEADER: NLA_POLICY_NESTED(ethnl_header_policy)
    nla_policy::default(),
];

unsafe fn wol_prepare_data(
    _req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    info: *const genl_info,
) -> i32 {
    let data = &mut *wol_repdata(reply_base);
    let dev = (*reply_base).dev;

    if (*(*dev).ethtool_ops).get_wol.is_none() {
        return -EOPNOTSUPP;
    }

    let ret = ethnl_ops_begin(dev);
    if ret < 0 {
        return ret;
    }
    ((*(*dev).ethtool_ops).get_wol.unwrap())(dev, &mut data.wol);
    ethnl_ops_complete(dev);
    // Do not include password in notifications.
    data.show_sopass = !genl_info_is_ntf(info)
        && (data.wol.supported & WAKE_MAGICSECURE) != 0;

    0
}

unsafe fn wol_reply_size(
    req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> i32 {
    let compact = ((*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;
    let data = &*wol_repdata(reply_base);

    let mut len = ethnl_bitset32_size(
        &data.wol.wolopts,
        &data.wol.supported,
        WOL_MODE_COUNT,
        wol_mode_names,
        compact,
    );
    if len < 0 {
        return len;
    }
    if data.show_sopass {
        len += nla_total_size(core::mem::size_of_val(&data.wol.sopass)) as i32;
    }
    len
}

unsafe fn wol_fill_reply(
    skb: *mut sk_buff,
    req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> i32 {
    let compact = ((*req_base).flags & ETHTOOL_FLAG_COMPACT_BITSETS) != 0;
    let data = &*wol_repdata(reply_base);

    let ret = ethnl_put_bitset32(
        skb,
        ETHTOOL_A_WOL_MODES,
        &data.wol.wolopts,
        &data.wol.supported,
        WOL_MODE_COUNT,
        wol_mode_names,
        compact,
    );
    if ret < 0 {
        return ret;
    }
    if data.show_sopass
        && nla_put(
            skb,
            ETHTOOL_A_WOL_SOPASS,
            core::mem::size_of_val(&data.wol.sopass),
            data.wol.sopass.as_ptr().cast(),
        ) != 0
    {
        return -EMSGSIZE;
    }

    0
}

// WOL_SET

pub static ethnl_wol_set_policy: [nla_policy; ETHTOOL_A_WOL_SOPASS + 1] = [
    nla_policy::default(),
];

unsafe fn ethnl_set_wol_validate(
    req_info: *mut ethnl_req_info,
    _info: *mut genl_info,
) -> i32 {
    let ops = (*(*req_info).dev).ethtool_ops;
    if (*ops).get_wol.is_some() && (*ops).set_wol.is_some() {
        1
    } else {
        -EOPNOTSUPP
    }
}

unsafe fn ethnl_set_wol(req_info: *mut ethnl_req_info, info: *mut genl_info) -> i32 {
    let mut wol = ethtool_wolinfo {
        cmd: ETHTOOL_GWOL,
        ..core::mem::zeroed()
    };
    let dev = (*req_info).dev;
    let tb = (*info).attrs;
    let mut modified = false;

    ((*(*dev).ethtool_ops).get_wol.unwrap())(dev, &mut wol);
    let mut ret = ethnl_update_bitset32(
        &mut wol.wolopts,
        WOL_MODE_COUNT,
        *tb.add(ETHTOOL_A_WOL_MODES),
        wol_mode_names,
        (*info).extack,
        &mut modified,
    );
    if ret < 0 {
        return ret;
    }
    if (wol.wolopts & !wol.supported) != 0 {
        NL_SET_ERR_MSG_ATTR(
            (*info).extack,
            *tb.add(ETHTOOL_A_WOL_MODES),
            "cannot enable unsupported WoL mode",
        );
        return -EINVAL;
    }
    let sopass_attr = *tb.add(ETHTOOL_A_WOL_SOPASS);
    if !sopass_attr.is_null() {
        if (wol.supported & WAKE_MAGICSECURE) == 0 {
            NL_SET_ERR_MSG_ATTR(
                (*info).extack,
                sopass_attr,
                "magicsecure not supported, cannot set password",
            );
            return -EINVAL;
        }
        ethnl_update_binary(
            wol.sopass.as_mut_ptr(),
            core::mem::size_of_val(&wol.sopass),
            sopass_attr,
            &mut modified,
        );
    }

    if !modified {
        return 0;
    }
    ret = ((*(*dev).ethtool_ops).set_wol.unwrap())(dev, &mut wol);
    if ret != 0 {
        return ret;
    }
    (*dev).ethtool.as_mut().unwrap().wol_enabled = (wol.wolopts != 0) as _;
    1
}

pub static ethnl_wol_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_WOL_GET,
    reply_cmd: ETHTOOL_MSG_WOL_GET_REPLY,
    hdr_attr: ETHTOOL_A_WOL_HEADER,
    req_info_size: core::mem::size_of::<wol_req_info>(),
    reply_data_size: core::mem::size_of::<wol_reply_data>(),
    prepare_data: Some(wol_prepare_data),
    reply_size: Some(wol_reply_size),
    fill_reply: Some(wol_fill_reply),
    set_validate: Some(ethnl_set_wol_validate),
    set: Some(ethnl_set_wol),
    set_ntf_cmd: ETHTOOL_MSG_WOL_NTF,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
