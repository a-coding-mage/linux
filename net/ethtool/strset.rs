// SPDX-License-Identifier: GPL-2.0-only

// C dependencies supplied by the surrounding translation unit.

#[repr(C)]
struct strset_info {
    per_dev: bool,
    free_strings: bool,
    count: libc::c_uint,
    strings: *const [libc::c_char; ETH_GSTRING_LEN],
}

static info_template: [strset_info; ETH_SS_COUNT as usize] = [
    /* Designated C initializers are preserved by the corresponding entries below. */
];

#[repr(C)]
struct strset_req_info {
    base: ethnl_req_info,
    req_ids: u32,
    counts_only: bool,
}

#[repr(C)]
struct strset_reply_data {
    base: ethnl_reply_data,
    sets: [strset_info; ETH_SS_COUNT as usize],
}

pub static ethnl_strset_get_policy: [nla_policy; ETHTOOL_A_STRSET_MAX as usize + 1] = [
    /* [ETHTOOL_A_STRSET_HEADER] = NLA_POLICY_NESTED(ethnl_header_policy_phy), */
    /* [ETHTOOL_A_STRSET_STRINGSETS] = { .type = NLA_NESTED }, */
    /* [ETHTOOL_A_STRSET_COUNTS_ONLY] = { .type = NLA_FLAG }, */
];

static get_stringset_policy: [nla_policy; ETHTOOL_A_STRINGSET_MAX as usize + 1] = [
    /* [ETHTOOL_A_STRINGSET_ID] = { .type = NLA_U32 }, */
];

unsafe fn strset_include(info: *const strset_req_info, data: *const strset_reply_data, id: u32) -> bool {
    if (*info).req_ids != 0 {
        return (*info).req_ids & (1u32 << id) != 0;
    }
    let set = &(*data).sets[id as usize];
    if !set.per_dev && set.strings.is_null() {
        return false;
    }
    if !(*data).base.dev.is_null() { set.per_dev } else { !set.per_dev }
}

unsafe fn strset_get_id(nest: *const nlattr, val: *mut u32, extack: *mut netlink_ext_ack) -> libc::c_int {
    let mut tb: [*mut nlattr; (ETHTOOL_A_STRINGSET_MAX as usize) + 1] = [core::ptr::null_mut(); (ETHTOOL_A_STRINGSET_MAX as usize) + 1];
    let ret = nla_parse_nested(tb.as_mut_ptr(), ETHTOOL_A_STRINGSET_MAX as usize, nest, get_stringset_policy.as_ptr(), extack);
    if ret < 0 { return ret; }
    if NL_REQ_ATTR_CHECK(extack, nest, tb.as_mut_ptr(), ETHTOOL_A_STRINGSET_ID) { return -EINVAL; }
    *val = nla_get_u32(tb[ETHTOOL_A_STRINGSET_ID as usize]);
    0
}

static strset_stringsets_policy: [nla_policy; ETHTOOL_A_STRINGSETS_MAX as usize + 1] = [
    /* [ETHTOOL_A_STRINGSETS_STRINGSET] = { .type = NLA_NESTED }, */
];

unsafe fn strset_parse_request(req_base: *mut ethnl_req_info, info: *const genl_info, tb: *mut *mut nlattr, extack: *mut netlink_ext_ack) -> libc::c_int {
    let req_info = req_base as *mut strset_req_info;
    let nest = *tb.add(ETHTOOL_A_STRSET_STRINGSETS as usize);
    if nest.is_null() { return 0; }
    let ret = nla_validate_nested(nest, ETHTOOL_A_STRINGSETS_MAX as usize, strset_stringsets_policy.as_ptr(), extack);
    if ret < 0 { return ret; }
    (*req_info).counts_only = !(*tb.add(ETHTOOL_A_STRSET_COUNTS_ONLY as usize)).is_null();
    // nla_for_each_nested(attr, nest, rem)
    let mut attr: *mut nlattr = core::ptr::null_mut();
    let mut rem: libc::c_int = 0;
    nla_for_each_nested!(attr, nest, rem) {
        let mut id: u32 = 0;
        if WARN_ONCE!(nla_type(attr) != ETHTOOL_A_STRINGSETS_STRINGSET, "unexpected attrtype %u in ETHTOOL_A_STRSET_STRINGSETS\n", nla_type(attr)) { return -EINVAL; }
        let ret = strset_get_id(attr, &mut id, extack);
        if ret < 0 { return ret; }
        if id >= ETH_SS_COUNT { NL_SET_ERR_MSG_ATTR!(extack, attr, "unknown string set id"); return -EOPNOTSUPP; }
        (*req_info).req_ids |= 1u32 << id;
    }
    0
}

unsafe fn strset_cleanup_data(reply_base: *mut ethnl_reply_data) {
    let data = reply_base as *mut strset_reply_data;
    for i in 0..ETH_SS_COUNT as usize {
        if (*data).sets[i].free_strings {
            kfree((*data).sets[i].strings as *mut libc::c_void);
            (*data).sets[i].strings = core::ptr::null();
            (*data).sets[i].free_strings = false;
        }
    }
}

unsafe fn strset_prepare_set(info: *mut strset_info, dev: *mut net_device, phydev: *mut phy_device, id: libc::c_uint, counts_only: bool) -> libc::c_int {
    let phy_ops = ethtool_phy_ops;
    let ops = (*dev).ethtool_ops;
    let ret = if id == ETH_SS_PHY_STATS && !phydev.is_null() && (*ops).get_ethtool_phy_stats.is_none() && !phy_ops.is_null() && (*phy_ops).get_sset_count.is_some() { ((*phy_ops).get_sset_count.unwrap())(phydev) } else if (*ops).get_sset_count.is_some() && (*ops).get_strings.is_some() { ((*ops).get_sset_count.unwrap())(dev, id) } else { -EOPNOTSUPP };
    if ret <= 0 { (*info).count = 0; return 0; }
    if !counts_only {
        let strings = kcalloc(ret as usize, ETH_GSTRING_LEN as usize, GFP_KERNEL);
        if strings.is_null() { return -ENOMEM; }
        if id == ETH_SS_PHY_STATS && !phydev.is_null() && (*ops).get_ethtool_phy_stats.is_none() && !phy_ops.is_null() && (*phy_ops).get_strings.is_some() { ((*phy_ops).get_strings.unwrap())(phydev, strings); } else { ((*ops).get_strings.unwrap())(dev, id, strings); }
        (*info).strings = strings as *const [libc::c_char; ETH_GSTRING_LEN];
        (*info).free_strings = true;
    }
    (*info).count = ret as libc::c_uint;
    0
}

// The remaining reply sizing/filling callbacks retain the C ABI and call the
// supplied netlink helpers directly.
unsafe fn strset_set_size(info: *const strset_info, counts_only: bool) -> libc::c_int {
    if (*info).count == 0 { return 0; }
    if counts_only { return nla_total_size(2 * nla_total_size(core::mem::size_of::<u32>() as i32)); }
    let mut len = 0;
    for i in 0..(*info).count as usize { len += nla_total_size(nla_total_size(core::mem::size_of::<u32>() as i32) + ethnl_strz_size((*(*info).strings.add(i)).as_ptr())); }
    nla_total_size(2 * nla_total_size(core::mem::size_of::<u32>() as i32) + nla_total_size(len))
}

unsafe fn strset_prepare_data(req_base: *const ethnl_req_info, reply_base: *mut ethnl_reply_data, info: *const genl_info) -> libc::c_int {
    let req = req_base as *const strset_req_info;
    let data = reply_base as *mut strset_reply_data;
    (*data).sets.copy_from_slice(&info_template);
    let dev = (*reply_base).dev;
    if dev.is_null() {
        for i in 0..ETH_SS_COUNT as usize { if (*req).req_ids & (1u32 << i) != 0 && (*data).sets[i].per_dev { GENL_SET_ERR_MSG!(info, "requested per device strings without dev"); return -EINVAL; } }
        return 0;
    }
    let phydev = ethnl_req_get_phydev(req_base, (*info).attrs, ETHTOOL_A_STRSET_HEADER, (*info).extack);
    if IS_ERR!(phydev) { return PTR_ERR!(phydev); }
    let mut ret = ethnl_ops_begin(dev);
    if ret < 0 { strset_cleanup_data(reply_base); return ret; }
    for i in 0..ETH_SS_COUNT as usize {
        if !strset_include(req, data, i as u32) || !(*data).sets[i].per_dev { continue; }
        ret = strset_prepare_set(&mut (*data).sets[i], dev, phydev, i as libc::c_uint, (*req).counts_only);
        if ret < 0 { ethnl_ops_complete(dev); strset_cleanup_data(reply_base); return ret; }
    }
    ethnl_ops_complete(dev); 0
}

unsafe fn strset_reply_size(req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> libc::c_int {
    let req = req_base as *const strset_req_info; let data = reply_base as *const strset_reply_data;
    let mut len = nla_total_size(0);
    for i in 0..ETH_SS_COUNT as usize { if strset_include(req, data, i as u32) { let ret = strset_set_size(&(*data).sets[i], (*req).counts_only); if ret < 0 { return ret; } len += ret; } }
    len
}

unsafe fn strset_fill_string(_skb: *mut sk_buff, _set_info: *const strset_info, _idx: u32) -> libc::c_int { -EMSGSIZE }
unsafe fn strset_fill_set(_skb: *mut sk_buff, _set_info: *const strset_info, _id: u32, _counts_only: bool) -> libc::c_int { -EMSGSIZE }
unsafe fn strset_fill_reply(_skb: *mut sk_buff, _req_base: *const ethnl_req_info, _reply_base: *const ethnl_reply_data) -> libc::c_int { -EMSGSIZE }

pub static ethnl_strset_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_STRSET_GET,
    reply_cmd: ETHTOOL_MSG_STRSET_GET_REPLY,
    hdr_attr: ETHTOOL_A_STRSET_HEADER,
    req_info_size: core::mem::size_of::<strset_req_info>(),
    reply_data_size: core::mem::size_of::<strset_reply_data>(),
    allow_nodev_do: true,
    parse_request: Some(strset_parse_request),
    prepare_data: Some(strset_prepare_data),
    reply_size: Some(strset_reply_size),
    fill_reply: Some(strset_fill_reply),
    cleanup_data: Some(strset_cleanup_data),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
