// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the Linux ethtool/netlink implementation are
// intentionally referenced here rather than reimplemented.

#[repr(C)]
pub struct eeprom_req_info {
    pub base: ethnl_req_info,
    pub offset: u32,
    pub length: u32,
    pub page: u8,
    pub bank: u8,
    pub i2c_address: u8,
}

#[repr(C)]
pub struct eeprom_reply_data {
    pub base: ethnl_reply_data,
    pub length: u32,
    pub data: *mut u8,
}

unsafe fn module_eeprom_reqinfo(p: *const ethnl_req_info) -> *mut eeprom_req_info {
    p as *mut eeprom_req_info
}

unsafe fn module_eeprom_repdata(p: *const ethnl_reply_data) -> *mut eeprom_reply_data {
    p as *mut eeprom_reply_data
}

unsafe fn fallback_set_params(
    request: *mut eeprom_req_info,
    modinfo: *mut ethtool_modinfo,
    eeprom: *mut ethtool_eeprom,
) -> i32 {
    let mut offset = (*request).offset;
    let length = (*request).length;

    if (*request).page != 0 {
        offset = (*request).page as u32 * ETH_MODULE_EEPROM_PAGE_LEN + offset;
    }

    if (*modinfo).type_ == ETH_MODULE_SFF_8472 && (*request).i2c_address == 0x51 {
        offset += ETH_MODULE_EEPROM_PAGE_LEN * 2;
    }

    if offset >= (*modinfo).eeprom_len || length > (*modinfo).eeprom_len - offset {
        return -EINVAL;
    }

    (*eeprom).cmd = ETHTOOL_GMODULEEEPROM;
    (*eeprom).len = length;
    (*eeprom).offset = offset;
    0
}

unsafe fn eeprom_fallback(
    request: *mut eeprom_req_info,
    reply: *mut eeprom_reply_data,
) -> i32 {
    let dev = (*reply).base.dev;
    let mut modinfo: ethtool_modinfo = core::mem::zeroed();
    let mut eeprom: ethtool_eeprom = core::mem::zeroed();
    let data: *mut u8;
    let err: i32;

    modinfo.cmd = ETHTOOL_GMODULEINFO;
    err = ethtool_get_module_info_call(dev, &mut modinfo);
    if err < 0 { return err; }

    err = fallback_set_params(request, &mut modinfo, &mut eeprom);
    if err < 0 { return err; }

    data = kzalloc(eeprom.len as usize, GFP_KERNEL);
    if data.is_null() { return -ENOMEM; }
    err = ethtool_get_module_eeprom_call(dev, &mut eeprom, data);
    if err < 0 {
        kfree(data);
        return err;
    }

    (*reply).data = data;
    (*reply).length = eeprom.len;
    0
}

unsafe fn get_module_eeprom_by_page(
    dev: *mut net_device,
    page_data: *mut ethtool_module_eeprom,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let ops = (*dev).ethtool_ops;

    if (*(*dev).ethtool).module_fw_flash_in_progress {
        NL_SET_ERR_MSG(extack, "Module firmware flashing is in progress");
        return -EBUSY;
    }
    if !(*dev).sfp_bus.is_null() {
        return sfp_get_module_eeprom_by_page((*dev).sfp_bus, page_data, extack);
    }
    if !(*ops).get_module_eeprom_by_page.is_none() {
        return ((*ops).get_module_eeprom_by_page.unwrap())(dev, page_data, extack);
    }
    -EOPNOTSUPP
}

unsafe fn eeprom_prepare_data(
    req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    info: *const genl_info,
) -> i32 {
    let reply = module_eeprom_repdata(reply_base);
    let request = module_eeprom_reqinfo(req_base);
    let mut page_data: ethtool_module_eeprom = core::mem::zeroed();
    let dev = (*reply_base).dev;
    let ret: i32;

    page_data.offset = (*request).offset;
    page_data.length = (*request).length;
    page_data.i2c_address = (*request).i2c_address;
    page_data.page = (*request).page;
    page_data.bank = (*request).bank;
    page_data.data = kmalloc(page_data.length as usize, GFP_KERNEL);
    if page_data.data.is_null() { return -ENOMEM; }

    ret = ethnl_ops_begin(dev);
    if ret != 0 { kfree(page_data.data); return ret; }
    ret = get_module_eeprom_by_page(dev, &mut page_data, (*info).extack);
    if ret < 0 {
        if ret == -EOPNOTSUPP { ret = eeprom_fallback(request, reply); }
        ethnl_ops_complete(dev);
        kfree(page_data.data);
        return ret;
    }
    (*reply).length = ret as u32;
    (*reply).data = page_data.data;
    ethnl_ops_complete(dev);
    0
}

unsafe fn eeprom_parse_request(
    req_info: *mut ethnl_req_info,
    info: *const genl_info,
    tb: *mut *mut nlattr,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let request = module_eeprom_reqinfo(req_info);
    if GENL_REQ_ATTR_CHECK(info, ETHTOOL_A_MODULE_EEPROM_OFFSET) != 0
        || GENL_REQ_ATTR_CHECK(info, ETHTOOL_A_MODULE_EEPROM_LENGTH) != 0
        || GENL_REQ_ATTR_CHECK(info, ETHTOOL_A_MODULE_EEPROM_PAGE) != 0
        || GENL_REQ_ATTR_CHECK(info, ETHTOOL_A_MODULE_EEPROM_I2C_ADDRESS) != 0 { return -EINVAL; }

    (*request).i2c_address = nla_get_u8(*tb.add(ETHTOOL_A_MODULE_EEPROM_I2C_ADDRESS as usize));
    (*request).offset = nla_get_u32(*tb.add(ETHTOOL_A_MODULE_EEPROM_OFFSET as usize));
    (*request).length = nla_get_u32(*tb.add(ETHTOOL_A_MODULE_EEPROM_LENGTH as usize));
    (*request).page = nla_get_u8(*tb.add(ETHTOOL_A_MODULE_EEPROM_PAGE as usize));
    if (*request).page != 0 && (*request).offset < ETH_MODULE_EEPROM_PAGE_LEN {
        NL_SET_ERR_MSG_ATTR(*tb.add(ETHTOOL_A_MODULE_EEPROM_PAGE as usize), "reading from lower half page is allowed for page 0 only");
        return -EINVAL;
    }
    if ((*request).offset < ETH_MODULE_EEPROM_PAGE_LEN && (*request).offset + (*request).length > ETH_MODULE_EEPROM_PAGE_LEN)
        || (*request).offset + (*request).length > ETH_MODULE_EEPROM_PAGE_LEN * 2 {
        NL_SET_ERR_MSG_ATTR(*tb.add(ETHTOOL_A_MODULE_EEPROM_LENGTH as usize), "reading cross page boundary is illegal");
        return -EINVAL;
    }
    if !(*tb.add(ETHTOOL_A_MODULE_EEPROM_BANK as usize)).is_null() {
        (*request).bank = nla_get_u8(*tb.add(ETHTOOL_A_MODULE_EEPROM_BANK as usize));
    }
    0
}

unsafe fn eeprom_reply_size(req_base: *const ethnl_req_info, _reply_base: *const ethnl_reply_data) -> i32 {
    nla_total_size(core::mem::size_of::<u8>() as i32 * (*module_eeprom_reqinfo(req_base)).length as i32)
}

unsafe fn eeprom_fill_reply(skb: *mut sk_buff, _req_base: *const ethnl_req_info, reply_base: *const ethnl_reply_data) -> i32 {
    let reply = module_eeprom_repdata(reply_base);
    nla_put(skb, ETHTOOL_A_MODULE_EEPROM_DATA, (*reply).length, (*reply).data)
}

unsafe fn eeprom_cleanup_data(reply_base: *mut ethnl_reply_data) {
    kfree((*module_eeprom_repdata(reply_base)).data);
}

pub static ethnl_module_eeprom_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_MODULE_EEPROM_GET,
    reply_cmd: ETHTOOL_MSG_MODULE_EEPROM_GET_REPLY,
    hdr_attr: ETHTOOL_A_MODULE_EEPROM_HEADER,
    req_info_size: core::mem::size_of::<eeprom_req_info>(),
    reply_data_size: core::mem::size_of::<eeprom_reply_data>(),
    parse_request: Some(eeprom_parse_request),
    prepare_data: Some(eeprom_prepare_data),
    reply_size: Some(eeprom_reply_size),
    fill_reply: Some(eeprom_fill_reply),
    cleanup_data: Some(eeprom_cleanup_data),
};

pub static ethnl_module_eeprom_get_policy: [nla_policy; ETHTOOL_A_MODULE_EEPROM_I2C_ADDRESS as usize + 1] = [
    nla_policy::nested(ethnl_header_policy),
    nla_policy::max(NLA_U32, ETH_MODULE_EEPROM_PAGE_LEN * 2 - 1),
    nla_policy::range(NLA_U32, 1, ETH_MODULE_EEPROM_PAGE_LEN),
    nla_policy { type_: NLA_U8 },
    nla_policy { type_: NLA_U8 },
    nla_policy::range(NLA_U8, 0, ETH_MODULE_MAX_I2C_ADDRESS),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
