// SPDX-License-Identifier: GPL-2.0-only

// Kernel and local header dependencies are supplied by the surrounding translation.

#[repr(C)]
pub struct module_req_info {
    pub base: ethnl_req_info,
}

#[repr(C)]
pub struct module_reply_data {
    pub base: ethnl_reply_data,
    pub power: ethtool_module_power_mode_params,
}

#[inline]
unsafe fn module_repdata(reply_base: *mut ethnl_reply_data) -> *mut module_reply_data {
    container_of!(reply_base, module_reply_data, base)
}

pub static ethnl_module_get_policy: [nla_policy; ETHTOOL_A_MODULE_HEADER as usize + 1] = [
    nla_policy_nested!(ETHTOOL_A_MODULE_HEADER, ethnl_header_policy),
];

unsafe fn module_get_power_mode(
    dev: *mut net_device,
    data: *mut module_reply_data,
    extack: *mut netlink_ext_ack,
) -> c_int {
    let ops = (*dev).ethtool_ops;
    if (*ops).get_module_power_mode.is_none() {
        return 0;
    }
    if (*(*dev).ethtool).module_fw_flash_in_progress {
        NL_SET_ERR_MSG!(extack, "Module firmware flashing is in progress");
        return -EBUSY;
    }
    ((*ops).get_module_power_mode.unwrap())(dev, &mut (*data).power, extack)
}

unsafe fn module_prepare_data(
    _req_base: *const ethnl_req_info,
    reply_base: *mut ethnl_reply_data,
    info: *const genl_info,
) -> c_int {
    let data = module_repdata(reply_base);
    let dev = (*reply_base).dev;
    let mut ret = ethnl_ops_begin(dev);
    if ret < 0 { return ret; }
    ret = module_get_power_mode(dev, data, (*info).extack);
    ethnl_ops_complete(dev);
    ret
}

unsafe fn module_reply_size(
    _req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> c_int {
    let data = module_repdata(reply_base as *mut ethnl_reply_data);
    let mut len = 0;
    if (*data).power.policy != 0 { len += nla_total_size(core::mem::size_of::<u8>()); }
    if (*data).power.mode != 0 { len += nla_total_size(core::mem::size_of::<u8>()); }
    len
}

unsafe fn module_fill_reply(
    skb: *mut sk_buff,
    _req_base: *const ethnl_req_info,
    reply_base: *const ethnl_reply_data,
) -> c_int {
    let data = module_repdata(reply_base as *mut ethnl_reply_data);
    if (*data).power.policy != 0 && nla_put_u8(skb, ETHTOOL_A_MODULE_POWER_MODE_POLICY, (*data).power.policy) != 0 { return -EMSGSIZE; }
    if (*data).power.mode != 0 && nla_put_u8(skb, ETHTOOL_A_MODULE_POWER_MODE, (*data).power.mode) != 0 { return -EMSGSIZE; }
    0
}

pub static ethnl_module_set_policy: [nla_policy; ETHTOOL_A_MODULE_POWER_MODE_POLICY as usize + 1] = [
    nla_policy_nested!(ETHTOOL_A_MODULE_HEADER, ethnl_header_policy),
    nla_policy_range!(ETHTOOL_A_MODULE_POWER_MODE_POLICY, NLA_U8, ETHTOOL_MODULE_POWER_MODE_POLICY_HIGH, ETHTOOL_MODULE_POWER_MODE_POLICY_AUTO),
];

unsafe fn ethnl_set_module_validate(req_info: *mut ethnl_req_info, info: *mut genl_info) -> c_int {
    let ops = (*(*req_info).dev).ethtool_ops;
    let tb = (*info).attrs;
    if (*tb.add(ETHTOOL_A_MODULE_POWER_MODE_POLICY as usize)).is_null() { return 0; }
    if (*ops).get_module_power_mode.is_none() || (*ops).set_module_power_mode.is_none() {
        NL_SET_ERR_MSG_ATTR!((*info).extack, *tb.add(ETHTOOL_A_MODULE_POWER_MODE_POLICY as usize), "Setting power mode policy is not supported by this device");
        return -EOPNOTSUPP;
    }
    1
}

unsafe fn ethnl_set_module(req_info: *mut ethnl_req_info, info: *mut genl_info) -> c_int {
    let dev = (*req_info).dev;
    let ops = (*dev).ethtool_ops;
    if (*(*dev).ethtool).module_fw_flash_in_progress { NL_SET_ERR_MSG!((*info).extack, "Module firmware flashing is in progress"); return -EBUSY; }
    let tb = (*info).attrs;
    let mut power: ethtool_module_power_mode_params = core::mem::zeroed();
    let mut power_new: ethtool_module_power_mode_params = core::mem::zeroed();
    power_new.policy = nla_get_u8(*tb.add(ETHTOOL_A_MODULE_POWER_MODE_POLICY as usize));
    let mut ret = ((*ops).get_module_power_mode.unwrap())(dev, &mut power, (*info).extack);
    if ret < 0 { return ret; }
    if power_new.policy == power.policy { return 0; }
    ret = ((*ops).set_module_power_mode.unwrap())(dev, &mut power_new, (*info).extack);
    if ret < 0 { ret } else { 1 }
}

pub static ethnl_module_request_ops: ethnl_request_ops = ethnl_request_ops {
    request_cmd: ETHTOOL_MSG_MODULE_GET, reply_cmd: ETHTOOL_MSG_MODULE_GET_REPLY,
    hdr_attr: ETHTOOL_A_MODULE_HEADER, req_info_size: core::mem::size_of::<module_req_info>(),
    reply_data_size: core::mem::size_of::<module_reply_data>(), prepare_data: Some(module_prepare_data),
    reply_size: Some(module_reply_size), fill_reply: Some(module_fill_reply),
    set_validate: Some(ethnl_set_module_validate), set: Some(ethnl_set_module),
    set_ntf_cmd: ETHTOOL_MSG_MODULE_NTF,
};

pub static ethnl_module_fw_flash_act_policy: [nla_policy; ETHTOOL_A_MODULE_FW_FLASH_PASSWORD as usize + 1] = [
    nla_policy_nested!(ETHTOOL_A_MODULE_FW_FLASH_HEADER, ethnl_header_policy),
    nla_policy_type!(ETHTOOL_A_MODULE_FW_FLASH_FILE_NAME, NLA_NUL_STRING),
    nla_policy_type!(ETHTOOL_A_MODULE_FW_FLASH_PASSWORD, NLA_U32),
];

static mut module_fw_flash_work_list: list_head = list_head::new();
static mut module_fw_flash_work_list_lock: spinlock_t = spinlock_t::new();

unsafe fn module_flash_fw_work_list_add(module_fw: *mut ethtool_module_fw_flash, info: *mut genl_info) -> c_int {
    spin_lock!(&raw mut module_fw_flash_work_list_lock);
    list_for_each_entry!(work, module_fw_flash_work_list, ethtool_module_fw_flash, list, {
        if (*work).fw_update.ntf_params.portid == (*info).snd_portid && (*work).fw_update.dev == (*module_fw).fw_update.dev {
            spin_unlock!(&raw mut module_fw_flash_work_list_lock); return -EALREADY;
        }
    });
    list_add_tail!(&mut (*module_fw).list, &raw mut module_fw_flash_work_list);
    spin_unlock!(&raw mut module_fw_flash_work_list_lock); 0
}

unsafe fn module_flash_fw_work_list_del(list: *mut list_head) { spin_lock!(&raw mut module_fw_flash_work_list_lock); list_del!(list); spin_unlock!(&raw mut module_fw_flash_work_list_lock); }

unsafe extern "C" fn module_flash_fw_work(work: *mut work_struct) {
    let module_fw = container_of!(work, ethtool_module_fw_flash, work);
    let dev = (*module_fw).fw_update.dev;
    netdev_lock_ops(dev); ethtool_cmis_fw_update(&mut (*module_fw).fw_update); netdev_unlock_ops(dev);
    module_flash_fw_work_list_del(&mut (*module_fw).list);
    rtnl_lock(); netdev_lock_ops(dev); (*(*dev).ethtool).module_fw_flash_in_progress = false; netdev_unlock_ops(dev); rtnl_unlock();
    netdev_put(dev, &mut (*module_fw).dev_tracker); release_firmware((*module_fw).fw_update.fw); kfree(module_fw as *mut c_void);
}

const MODULE_EEPROM_PHYS_ID_PAGE: u32 = 0;
const MODULE_EEPROM_PHYS_ID_I2C_ADDR: u8 = 0x50;

unsafe fn module_flash_fw_work_init(module_fw: *mut ethtool_module_fw_flash, dev: *mut net_device, extack: *mut netlink_ext_ack) -> c_int {
    let ops = (*dev).ethtool_ops; let mut page_data: ethtool_module_eeprom = core::mem::zeroed(); let mut phys_id: u8 = 0;
    page_data.page = MODULE_EEPROM_PHYS_ID_PAGE; page_data.offset = SFP_PHYS_ID; page_data.length = core::mem::size_of::<u8>() as u32; page_data.i2c_address = MODULE_EEPROM_PHYS_ID_I2C_ADDR; page_data.data = &mut phys_id;
    let err = ((*ops).get_module_eeprom_by_page.unwrap())(dev, &mut page_data, extack); if err < 0 { return err; }
    match phys_id { SFF8024_ID_QSFP_DD | SFF8024_ID_OSFP | SFF8024_ID_DSFP | SFF8024_ID_QSFP_PLUS_CMIS | SFF8024_ID_SFP_DD_CMIS | SFF8024_ID_SFP_PLUS_CMIS => INIT_WORK!(&mut (*module_fw).work, module_flash_fw_work), _ => { NL_SET_ERR_MSG!(extack, "Module type does not support firmware flashing"); return -EOPNOTSUPP; } }
    0
}

pub unsafe extern "C" fn ethnl_module_fw_flash_sock_destroy(sk_priv: *mut ethnl_sock_priv) {
    spin_lock!(&raw mut module_fw_flash_work_list_lock);
    list_for_each_entry!(work, module_fw_flash_work_list, ethtool_module_fw_flash, list, { if (*work).fw_update.ntf_params.portid == (*sk_priv).portid && dev_net((*work).fw_update.dev) == (*sk_priv).net { (*work).fw_update.ntf_params.closed_sock = true; } });
    spin_unlock!(&raw mut module_fw_flash_work_list_lock);
}

unsafe fn module_flash_fw_schedule(dev: *mut net_device, file_name: *const c_char, params: *mut ethtool_module_fw_flash_params, skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let module_fw = kzalloc_obj!(*module_fw);
    if module_fw.is_null() { return -ENOMEM; }
    let fw_update = &mut (*module_fw).fw_update;
    (*fw_update).params = *params;
    let mut err = request_firmware_direct(&mut (*fw_update).fw, file_name, &(*dev).dev);
    if err != 0 { NL_SET_ERR_MSG!((*info).extack, "Failed to request module firmware image"); kfree(module_fw as *mut c_void); return err; }
    err = module_flash_fw_work_init(module_fw, dev, (*info).extack);
    if err < 0 { release_firmware((*fw_update).fw); kfree(module_fw as *mut c_void); return err; }
    (*fw_update).dev = dev; (*fw_update).ntf_params.portid = (*info).snd_portid; (*fw_update).ntf_params.seq = (*info).snd_seq; (*fw_update).ntf_params.closed_sock = false;
    err = ethnl_sock_priv_set(skb, dev_net(dev), (*fw_update).ntf_params.portid, ETHTOOL_SOCK_TYPE_MODULE_FW_FLASH);
    if err < 0 { release_firmware((*fw_update).fw); kfree(module_fw as *mut c_void); return err; }
    err = module_flash_fw_work_list_add(module_fw, info);
    if err < 0 { release_firmware((*fw_update).fw); kfree(module_fw as *mut c_void); return err; }
    (*(*dev).ethtool).module_fw_flash_in_progress = true; netdev_hold(dev, &mut (*module_fw).dev_tracker, GFP_KERNEL); schedule_work(&mut (*module_fw).work); 0
}

unsafe fn module_flash_fw(dev: *mut net_device, tb: *mut *mut nlattr, skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let mut params: ethtool_module_fw_flash_params = core::mem::zeroed();
    if GENL_REQ_ATTR_CHECK!(info, ETHTOOL_A_MODULE_FW_FLASH_FILE_NAME) { return -EINVAL; }
    let file_name = nla_data(*tb.add(ETHTOOL_A_MODULE_FW_FLASH_FILE_NAME as usize));
    let attr = *tb.add(ETHTOOL_A_MODULE_FW_FLASH_PASSWORD as usize);
    if !attr.is_null() { params.password = cpu_to_be32(nla_get_u32(attr)); params.password_valid = true; }
    module_flash_fw_schedule(dev, file_name, &mut params, skb, info)
}

unsafe fn ethnl_module_fw_flash_validate(dev: *mut net_device, extack: *mut netlink_ext_ack) -> c_int {
    let devlink_port = (*dev).devlink_port; let ops = (*dev).ethtool_ops;
    if (*ops).set_module_eeprom_by_page.is_none() || (*ops).get_module_eeprom_by_page.is_none() { NL_SET_ERR_MSG!(extack, "Flashing module firmware is not supported by this device"); return -EOPNOTSUPP; }
    if (*ops).reset.is_none() { NL_SET_ERR_MSG!(extack, "Reset module is not supported by this device, so flashing is not permitted"); return -EOPNOTSUPP; }
    if (*(*dev).ethtool).module_fw_flash_in_progress { NL_SET_ERR_MSG!(extack, "Module firmware flashing already in progress"); return -EBUSY; }
    if (*dev).flags & IFF_UP != 0 { NL_SET_ERR_MSG!(extack, "Netdevice is up, so flashing is not permitted"); return -EBUSY; }
    if !devlink_port.is_null() && (*devlink_port).attrs.split { NL_SET_ERR_MSG!(extack, "Can't perform firmware flashing on a split port"); return -EOPNOTSUPP; }
    0
}

pub unsafe extern "C" fn ethnl_act_module_fw_flash(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    let mut req_info: ethnl_req_info = core::mem::zeroed(); let tb = (*info).attrs; let mut ret = ethnl_parse_header_dev_get(&mut req_info, *tb.add(ETHTOOL_A_MODULE_FW_FLASH_HEADER as usize), genl_info_net(info), (*info).extack, true);
    if ret < 0 { return ret; } let dev = req_info.dev; netdev_lock_ops_compat(dev); ret = ethnl_ops_begin(dev);
    if ret >= 0 { ret = ethnl_module_fw_flash_validate(dev, (*info).extack); if ret >= 0 { ret = module_flash_fw(dev, tb, skb, info); } ethnl_ops_complete(dev); }
    netdev_unlock_ops_compat(dev); ethnl_parse_header_dev_put(&mut req_info); ret
}

unsafe fn ethnl_module_fw_flash_ntf_put_err(skb: *mut sk_buff, err_msg: *mut c_char, sub_err_msg: *mut c_char) -> c_int {
    if err_msg.is_null() { return 0; }
    let err_len = strlen(err_msg); let sub_len = if sub_err_msg.is_null() { 0 } else { strlen(sub_err_msg) };
    let total_len = err_len + 2 + if sub_err_msg.is_null() { 0 } else { sub_len + 2 };
    let attr = nla_reserve(skb, ETHTOOL_A_MODULE_FW_FLASH_STATUS_MSG, total_len); if attr.is_null() { return -ENOMEM; }
    if sub_err_msg.is_null() { sprintf!(nla_data(attr), "%s.", err_msg); } else { sprintf!(nla_data(attr), "%s, %s.", err_msg, sub_err_msg); } 0
}

unsafe fn ethnl_module_fw_flash_ntf(dev: *mut net_device, status: ethtool_module_fw_flash_status, ntf_params: *mut ethnl_module_fw_flash_ntf_params, err_msg: *mut c_char, sub_err_msg: *mut c_char, done: u64, total: u64) {
    if (*ntf_params).closed_sock { return; }
    let skb = genlmsg_new(NLMSG_GOODSIZE, GFP_KERNEL); if skb.is_null() { return; }
    let hdr = ethnl_unicast_put(skb, (*ntf_params).portid, { (*ntf_params).seq += 1; (*ntf_params).seq }, ETHTOOL_MSG_MODULE_FW_FLASH_NTF); if hdr.is_null() { nlmsg_free(skb); return; }
    if ethnl_fill_reply_header(skb, dev, ETHTOOL_A_MODULE_FW_FLASH_HEADER) < 0 || nla_put_u32(skb, ETHTOOL_A_MODULE_FW_FLASH_STATUS, status) != 0 || ethnl_module_fw_flash_ntf_put_err(skb, err_msg, sub_err_msg) < 0 || nla_put_uint(skb, ETHTOOL_A_MODULE_FW_FLASH_DONE, done) != 0 || nla_put_uint(skb, ETHTOOL_A_MODULE_FW_FLASH_TOTAL, total) != 0 { nlmsg_free(skb); return; }
    genlmsg_end(skb, hdr); genlmsg_unicast(dev_net(dev), skb, (*ntf_params).portid);
}

pub unsafe extern "C" fn ethnl_module_fw_flash_ntf_err(dev: *mut net_device, params: *mut ethnl_module_fw_flash_ntf_params, err_msg: *mut c_char, sub_err_msg: *mut c_char) { ethnl_module_fw_flash_ntf(dev, ETHTOOL_MODULE_FW_FLASH_STATUS_ERROR, params, err_msg, sub_err_msg, 0, 0); }
pub unsafe extern "C" fn ethnl_module_fw_flash_ntf_start(dev: *mut net_device, params: *mut ethnl_module_fw_flash_ntf_params) { ethnl_module_fw_flash_ntf(dev, ETHTOOL_MODULE_FW_FLASH_STATUS_STARTED, params, core::ptr::null_mut(), core::ptr::null_mut(), 0, 0); }
pub unsafe extern "C" fn ethnl_module_fw_flash_ntf_complete(dev: *mut net_device, params: *mut ethnl_module_fw_flash_ntf_params) { ethnl_module_fw_flash_ntf(dev, ETHTOOL_MODULE_FW_FLASH_STATUS_COMPLETED, params, core::ptr::null_mut(), core::ptr::null_mut(), 0, 0); }
pub unsafe extern "C" fn ethnl_module_fw_flash_ntf_in_progress(dev: *mut net_device, params: *mut ethnl_module_fw_flash_ntf_params, done: u64, total: u64) { ethnl_module_fw_flash_ntf(dev, ETHTOOL_MODULE_FW_FLASH_STATUS_IN_PROGRESS, params, core::ptr::null_mut(), core::ptr::null_mut(), done, total); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
