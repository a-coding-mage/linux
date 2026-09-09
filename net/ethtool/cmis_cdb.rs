// SPDX-License-Identifier: GPL-2.0-only

// External kernel/project declarations are supplied by the surrounding Rust translation.

pub fn ethtool_cmis_get_max_lpl_size(num_of_byte_octs: u8) -> u32 {
    8 * (1 + core::cmp::min(num_of_byte_octs, 15) as u32)
}

pub unsafe fn ethtool_cmis_cdb_compose_args(
    args: *mut ethtool_cmis_cdb_cmd_args,
    cmd: ethtool_cmis_cdb_cmd_id,
    lpl: *mut u8,
    lpl_len: u8,
    epl: *mut u8,
    epl_len: u16,
    max_duration: u16,
    read_write_len_ext: u8,
    msleep_pre_rpl: u16,
    rpl_exp_len: u8,
    flags: u8,
) {
    (*args).req.id = cpu_to_be16(cmd as u16);
    (*args).req.lpl_len = lpl_len;
    if !lpl.is_null() {
        core::ptr::copy_nonoverlapping(lpl, (*args).req.payload.as_mut_ptr(), lpl_len as usize);
    }
    if !epl.is_null() {
        (*args).req.epl_len = cpu_to_be16(epl_len);
        (*args).req.epl = epl;
    }
    (*args).max_duration = max_duration;
    (*args).read_write_len_ext = ethtool_cmis_get_max_lpl_size(read_write_len_ext) as u16;
    (*args).msleep_pre_rpl = msleep_pre_rpl;
    (*args).rpl_exp_len = rpl_exp_len;
    (*args).flags = flags;
    (*args).err_msg = core::ptr::null_mut();
}

pub unsafe fn ethtool_cmis_page_init(page_data: *mut ethtool_module_eeprom, page: u8, offset: u32, length: u32) {
    (*page_data).page = page;
    (*page_data).offset = offset;
    (*page_data).length = length;
    (*page_data).i2c_address = ETHTOOL_CMIS_CDB_PAGE_I2C_ADDR;
}

const CMIS_REVISION_PAGE: u8 = 0x00;
const CMIS_REVISION_OFFSET: u32 = 0x01;

#[repr(C)]
struct cmis_rev_rpl { rev: u8 }

unsafe fn cmis_rev_rpl_major(rpl: *mut cmis_rev_rpl) -> u8 { (*rpl).rev >> 4 }

unsafe fn cmis_rev_major_get(dev: *mut net_device, rev_major: *mut u8) -> i32 {
    let ops = (*dev).ethtool_ops;
    let mut page_data: ethtool_module_eeprom = core::mem::zeroed();
    let mut extack: netlink_ext_ack = core::mem::zeroed();
    let mut rpl: cmis_rev_rpl = core::mem::zeroed();
    ethtool_cmis_page_init(&mut page_data, CMIS_REVISION_PAGE, CMIS_REVISION_OFFSET, core::mem::size_of::<cmis_rev_rpl>() as u32);
    page_data.data = &mut rpl as *mut _ as *mut u8;
    let err = ((*ops).get_module_eeprom_by_page)(dev, &mut page_data, &mut extack);
    if err < 0 { if !extack._msg.is_null() { netdev_err(dev, extack._msg); } return err; }
    *rev_major = cmis_rev_rpl_major(&mut rpl);
    0
}

const CMIS_CDB_ADVERTISEMENT_PAGE: u8 = 0x01;
const CMIS_CDB_ADVERTISEMENT_OFFSET: u32 = 0xA3;

#[repr(C)]
struct cmis_cdb_advert_rpl { inst_supported: u8, read_write_len_ext: u8, resv1: u8, resv2: u8 }
unsafe fn cmis_cdb_advert_rpl_inst_supported(rpl: *mut cmis_cdb_advert_rpl) -> u8 { (*rpl).inst_supported >> 6 }

unsafe fn cmis_cdb_advertisement_get(cdb: *mut ethtool_cmis_cdb, dev: *mut net_device, ntf_params: *mut ethnl_module_fw_flash_ntf_params) -> i32 {
    let ops = (*dev).ethtool_ops;
    let mut page_data: ethtool_module_eeprom = core::mem::zeroed();
    let mut rpl: cmis_cdb_advert_rpl = core::mem::zeroed();
    let mut extack: netlink_ext_ack = core::mem::zeroed();
    ethtool_cmis_page_init(&mut page_data, CMIS_CDB_ADVERTISEMENT_PAGE, CMIS_CDB_ADVERTISEMENT_OFFSET, core::mem::size_of::<cmis_cdb_advert_rpl>() as u32);
    page_data.data = &mut rpl as *mut _ as *mut u8;
    let err = ((*ops).get_module_eeprom_by_page)(dev, &mut page_data, &mut extack);
    if err < 0 { if !extack._msg.is_null() { netdev_err(dev, extack._msg); } return err; }
    if cmis_cdb_advert_rpl_inst_supported(&mut rpl) == 0 { ethnl_module_fw_flash_ntf_err(dev, ntf_params, "CDB functionality is not supported", core::ptr::null_mut()); return -EOPNOTSUPP; }
    (*cdb).read_write_len_ext = rpl.read_write_len_ext;
    0
}

const CMIS_PASSWORD_ENTRY_PAGE: u8 = 0x00;
const CMIS_PASSWORD_ENTRY_OFFSET: u32 = 0x7A;
#[repr(C)] struct cmis_password_entry_pl { password: u32 }
#[repr(C)] struct cmis_cdb_query_status_pl { response_delay: u16 }
#[repr(C)] struct cmis_cdb_query_status_rpl { length: u8, status: u8 }

unsafe fn cmis_cdb_validate_password(cdb: *mut ethtool_cmis_cdb, dev: *mut net_device, params: *const ethtool_module_fw_flash_params, ntf_params: *mut ethnl_module_fw_flash_ntf_params) -> i32 {
    let ops = (*dev).ethtool_ops;
    let mut qs_pl: cmis_cdb_query_status_pl = core::mem::zeroed();
    let mut page_data: ethtool_module_eeprom = core::mem::zeroed();
    let mut args: ethtool_cmis_cdb_cmd_args = core::mem::zeroed();
    let mut pe_pl: cmis_password_entry_pl = core::mem::zeroed();
    let mut extack: netlink_ext_ack = core::mem::zeroed();
    ethtool_cmis_page_init(&mut page_data, CMIS_PASSWORD_ENTRY_PAGE, CMIS_PASSWORD_ENTRY_OFFSET, core::mem::size_of::<cmis_password_entry_pl>() as u32);
    page_data.data = &mut pe_pl as *mut _ as *mut u8;
    pe_pl = *(page_data.data as *const cmis_password_entry_pl);
    pe_pl.password = (*params).password;
    netdev_assert_locked_ops(dev);
    let mut err = ((*ops).set_module_eeprom_by_page)(dev, &mut page_data, &mut extack);
    if err < 0 { if !extack._msg.is_null() { netdev_err(dev, extack._msg); } return err; }
    ethtool_cmis_cdb_compose_args(&mut args, ETHTOOL_CMIS_CDB_CMD_QUERY_STATUS, &mut qs_pl as *mut _ as *mut u8, core::mem::size_of::<cmis_cdb_query_status_pl>() as u8, core::ptr::null_mut(), 0, 0, (*cdb).read_write_len_ext, 1000, core::mem::size_of::<cmis_cdb_query_status_rpl>() as u8, CDB_F_COMPLETION_VALID | CDB_F_STATUS_VALID);
    err = ethtool_cmis_cdb_execute_cmd(dev, &mut args);
    if err < 0 { ethnl_module_fw_flash_ntf_err(dev, ntf_params, "Query Status command failed", args.err_msg); return err; }
    let rpl = args.req.payload.as_ptr() as *const cmis_cdb_query_status_rpl;
    if (*rpl).length == 0 || (*rpl).status == 0 { ethnl_module_fw_flash_ntf_err(dev, ntf_params, "Password was not accepted", core::ptr::null_mut()); return -EINVAL; }
    0
}

pub unsafe fn ethtool_cmis_cdb_check_completion_flag(cmis_rev: u8, flags: *mut u8) { if cmis_rev >= 5 { *flags |= CDB_F_COMPLETION_VALID; } }

const CMIS_CDB_MODULE_FEATURES_RESV_DATA: usize = 34;
#[repr(C)] struct cmis_cdb_module_features_rpl { resv1: [u8; CMIS_CDB_MODULE_FEATURES_RESV_DATA], max_completion_time: u16 }
unsafe fn cmis_cdb_module_features_completion_time(rpl: *mut cmis_cdb_module_features_rpl) -> u16 { be16_to_cpu((*rpl).max_completion_time) }

unsafe fn cmis_cdb_module_features_get(cdb: *mut ethtool_cmis_cdb, dev: *mut net_device, ntf_params: *mut ethnl_module_fw_flash_ntf_params) -> i32 {
    let mut args: ethtool_cmis_cdb_cmd_args = core::mem::zeroed();
    let flags = CDB_F_STATUS_VALID;
    let mut flags_mut = flags;
    ethtool_cmis_cdb_check_completion_flag((*cdb).cmis_rev, &mut flags_mut);
    ethtool_cmis_cdb_compose_args(&mut args, ETHTOOL_CMIS_CDB_CMD_MODULE_FEATURES, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0, 0, (*cdb).read_write_len_ext, 1000, core::mem::size_of::<cmis_cdb_module_features_rpl>() as u8, flags_mut);
    let err = ethtool_cmis_cdb_execute_cmd(dev, &mut args);
    if err < 0 { ethnl_module_fw_flash_ntf_err(dev, ntf_params, "Module Features command failed", args.err_msg); return err; }
    (*cdb).max_completion_time = cmis_cdb_module_features_completion_time(args.req.payload.as_ptr() as *mut cmis_cdb_module_features_rpl);
    0
}

pub unsafe fn ethtool_cmis_cdb_init(dev: *mut net_device, params: *const ethtool_module_fw_flash_params, ntf_params: *mut ethnl_module_fw_flash_ntf_params) -> *mut ethtool_cmis_cdb {
    let cdb = kzalloc_obj::<ethtool_cmis_cdb>();
    if cdb.is_null() { return ERR_PTR(-ENOMEM); }
    let mut err = cmis_rev_major_get(dev, &mut (*cdb).cmis_rev);
    if err < 0 { ethtool_cmis_cdb_fini(cdb); return ERR_PTR(err); }
    if (*cdb).cmis_rev < 4 { ethnl_module_fw_flash_ntf_err(dev, ntf_params, "CMIS revision doesn't support module firmware flashing", core::ptr::null_mut()); ethtool_cmis_cdb_fini(cdb); return ERR_PTR(-EOPNOTSUPP); }
    err = cmis_cdb_advertisement_get(cdb, dev, ntf_params);
    if err < 0 { ethtool_cmis_cdb_fini(cdb); return ERR_PTR(err); }
    if (*params).password_valid { err = cmis_cdb_validate_password(cdb, dev, params, ntf_params); if err < 0 { ethtool_cmis_cdb_fini(cdb); return ERR_PTR(err); } }
    err = cmis_cdb_module_features_get(cdb, dev, ntf_params);
    if err < 0 { ethtool_cmis_cdb_fini(cdb); return ERR_PTR(err); }
    cdb
}

pub unsafe fn ethtool_cmis_cdb_fini(cdb: *mut ethtool_cmis_cdb) { kfree(cdb as *mut core::ffi::c_void); }

unsafe fn is_completed(data: u8) -> bool { data & 0x40 != 0 }
const CMIS_CDB_STATUS_SUCCESS: u8 = 0x01;
unsafe fn status_success(data: u8) -> bool { data == CMIS_CDB_STATUS_SUCCESS }
const CMIS_CDB_STATUS_FAIL: u8 = 0x40;
unsafe fn status_fail(data: u8) -> bool { data & CMIS_CDB_STATUS_FAIL != 0 }
#[repr(C)] struct cmis_wait_for_cond_rpl { state: u8 }

unsafe fn ethtool_cmis_module_poll(dev: *mut net_device, rpl: *mut cmis_wait_for_cond_rpl, offset: u32, cond_success: Option<unsafe fn(u8)->bool>, cond_fail: Option<unsafe fn(u8)->bool>) -> i32 {
    let ops = (*dev).ethtool_ops; let mut page_data: ethtool_module_eeprom = core::mem::zeroed(); let mut extack: netlink_ext_ack = core::mem::zeroed();
    ethtool_cmis_page_init(&mut page_data, 0, offset, core::mem::size_of::<cmis_wait_for_cond_rpl>() as u32); page_data.data = rpl as *mut u8;
    let err = ((*ops).get_module_eeprom_by_page)(dev, &mut page_data, &mut extack); if err < 0 { if !extack._msg.is_null() { netdev_err_once(dev, extack._msg); } return -EBUSY; }
    if cond_success.map_or(false, |f| f((*rpl).state)) { return 0; } if cond_fail.map_or(false, |f| f((*rpl).state)) { return -EIO; } -EBUSY
}

pub unsafe fn ethtool_cmis_wait_for_cond(dev: *mut net_device, flags: u8, flag: u8, max_duration: u16, offset: u32, cond_success: Option<unsafe fn(u8)->bool>, cond_fail: Option<unsafe fn(u8)->bool>, state: *mut u8) -> i32 {
    let mut rpl: cmis_wait_for_cond_rpl = core::mem::zeroed(); if flags & flag == 0 { return 0; } let max_duration = if max_duration == 0 { u16::MAX } else { max_duration }; let end = jiffies() + msecs_to_jiffies(max_duration as u64); let mut err;
    loop { err = ethtool_cmis_module_poll(dev, &mut rpl, offset, cond_success, cond_fail); if err != -EBUSY { break; } msleep(20); if !time_before(jiffies(), end) { err = ethtool_cmis_module_poll(dev, &mut rpl, offset, cond_success, cond_fail); if err == -EBUSY { err = -ETIMEDOUT; } break; } }
    *state = rpl.state; err
}

const CMIS_CDB_COMPLETION_FLAG_OFFSET: u32 = 0x08;
unsafe fn cmis_cdb_wait_for_completion(dev: *mut net_device, args: *mut ethtool_cmis_cdb_cmd_args) -> i32 { msleep((*args).msleep_pre_rpl); let mut flag = 0; let err = ethtool_cmis_wait_for_cond(dev, (*args).flags, CDB_F_COMPLETION_VALID, (*args).max_duration, CMIS_CDB_COMPLETION_FLAG_OFFSET, Some(is_completed), None, &mut flag); if err < 0 { (*args).err_msg = "Completion Flag did not set on time" as *const str as *mut u8; } err }

const CMIS_CDB_STATUS_OFFSET: u32 = 0x25;
unsafe fn cmis_cdb_status_fail_msg_get(status: u8, err_msg: *mut *mut u8) { *err_msg = match status { 0b10000001 => "CDB Status is in progress: Busy capturing command", 0b10000010 => "CDB Status is in progress: Busy checking/validating command", 0b10000011 => "CDB Status is in progress: Busy executing", 0b01000000 => "CDB status failed: no specific failure", 0b01000010 => "CDB status failed: Parameter range error or parameter not supported", 0b01000101 => "CDB status failed: CdbChkCode error", 0b01000110 => "CDB status failed: Password error", _ => "Unknown failure reason" } as *const str as *mut u8; }
unsafe fn cmis_cdb_wait_for_status(dev: *mut net_device, args: *mut ethtool_cmis_cdb_cmd_args) -> i32 { msleep((*args).msleep_pre_rpl); let mut status = 0; let err = ethtool_cmis_wait_for_cond(dev, (*args).flags, CDB_F_STATUS_VALID, (*args).max_duration, CMIS_CDB_STATUS_OFFSET, Some(status_success), Some(status_fail), &mut status); if err < 0 && (*args).err_msg.is_null() { cmis_cdb_status_fail_msg_get(status, &mut (*args).err_msg); } err }

const CMIS_CDB_REPLY_OFFSET: u32 = 0x86;
unsafe fn cmis_cdb_process_reply(dev: *mut net_device, page_data: *mut ethtool_module_eeprom, args: *mut ethtool_cmis_cdb_cmd_args) -> i32 { if (*args).rpl_exp_len == 0 { return 0; } let rpl_exp_len = (*args).rpl_exp_len + core::mem::size_of::<ethtool_cmis_cdb_rpl_hdr>() as u8; ethtool_cmis_page_init(page_data, ETHTOOL_CMIS_CDB_CMD_PAGE, CMIS_CDB_REPLY_OFFSET, rpl_exp_len as u32); (*page_data).data = kmalloc((*page_data).length, GFP_KERNEL); if (*page_data).data.is_null() { return -ENOMEM; } let mut extack: netlink_ext_ack = core::mem::zeroed(); let err = ((*(*dev).ethtool_ops).get_module_eeprom_by_page)(dev, page_data, &mut extack); if err < 0 { if !extack._msg.is_null() { netdev_err(dev, extack._msg); } kfree((*page_data).data as *mut _); return err; } let rpl = (*page_data).data as *const ethtool_cmis_cdb_rpl; if (*rpl).hdr.rpl_len != (*args).rpl_exp_len || (*rpl).hdr.rpl_chk_code == 0 { kfree((*page_data).data as *mut _); return -EIO; } (*args).req.lpl_len = (*rpl).hdr.rpl_len; core::ptr::copy_nonoverlapping((*rpl).payload.as_ptr(), (*args).req.payload.as_mut_ptr(), (*args).req.lpl_len as usize); kfree((*page_data).data as *mut _); 0 }

unsafe fn __ethtool_cmis_cdb_execute_cmd(dev: *mut net_device, page_data: *mut ethtool_module_eeprom, page: u8, offset: u32, length: u32, data: *const core::ffi::c_void) -> i32 { ethtool_cmis_page_init(page_data, page, offset, length); (*page_data).data = kmemdup(data, (*page_data).length, GFP_KERNEL); if (*page_data).data.is_null() { return -ENOMEM; } let mut extack: netlink_ext_ack = core::mem::zeroed(); netdev_assert_locked_ops(dev); let err = ((*(*dev).ethtool_ops).set_module_eeprom_by_page)(dev, page_data, &mut extack); if err < 0 && !extack._msg.is_null() { netdev_err(dev, extack._msg); } kfree((*page_data).data as *mut _); err }

const CMIS_CDB_EPL_PAGE_START: u8 = 0xA0; const CMIS_CDB_EPL_PAGE_END: u8 = 0xAF; const CMIS_CDB_EPL_FW_BLOCK_OFFSET_START: u16 = 128; const CMIS_CDB_EPL_FW_BLOCK_OFFSET_END: u16 = 255;
unsafe fn ethtool_cmis_cdb_execute_epl_cmd(dev: *mut net_device, args: *mut ethtool_cmis_cdb_cmd_args, page_data: *mut ethtool_module_eeprom) -> i32 { let epl_len = be16_to_cpu((*args).req.epl_len); let mut bytes_written = 0u32; let mut page = CMIS_CDB_EPL_PAGE_START; while page <= CMIS_CDB_EPL_PAGE_END && bytes_written < epl_len as u32 { let mut offset = CMIS_CDB_EPL_FW_BLOCK_OFFSET_START; while offset <= CMIS_CDB_EPL_FW_BLOCK_OFFSET_END && bytes_written < epl_len as u32 { let bytes_left = epl_len as u32 - bytes_written; let space_left = CMIS_CDB_EPL_FW_BLOCK_OFFSET_END - offset + 1; let bytes_to_write = core::cmp::min(bytes_left as u16, core::cmp::min(space_left, (*args).read_write_len_ext)); let err = __ethtool_cmis_cdb_execute_cmd(dev, page_data, page, offset as u32, bytes_to_write as u32, (*args).req.epl.add(bytes_written as usize) as *const _); if err < 0 { return err; } offset += bytes_to_write; bytes_written += bytes_to_write as u32; } page += 1; } 0 }

unsafe fn cmis_cdb_calc_checksum(data: *const u8, size: usize) -> u8 { let mut checksum = 0u8; for i in 0..size { checksum = checksum.wrapping_add(*data.add(i)); } !checksum }
const CMIS_CDB_CMD_ID_OFFSET: u32 = 0x80;
pub unsafe fn ethtool_cmis_cdb_execute_cmd(dev: *mut net_device, args: *mut ethtool_cmis_cdb_cmd_args) -> i32 { let mut page_data: ethtool_module_eeprom = core::mem::zeroed(); (*args).req.chk_code = cmis_cdb_calc_checksum(&(*args).req as *const _ as *const u8, core::mem::offset_of!(ethtool_cmis_cdb_request, epl)); if (*args).req.lpl_len as u16 > (*args).read_write_len_ext { (*args).err_msg = "LPL length is longer than CDB read write length extension allows" as *const str as *mut u8; return -EINVAL; } let mut offset = CMIS_CDB_CMD_ID_OFFSET + core::mem::offset_of!(ethtool_cmis_cdb_request, body) as u32; let mut err = __ethtool_cmis_cdb_execute_cmd(dev, &mut page_data, ETHTOOL_CMIS_CDB_CMD_PAGE, offset, core::mem::size_of_val(&(*args).req.body) as u32, &(*args).req.body as *const _ as *const _); if err < 0 { return err; } if (*args).req.epl_len != 0 { err = ethtool_cmis_cdb_execute_epl_cmd(dev, args, &mut page_data); if err < 0 { return err; } } offset = CMIS_CDB_CMD_ID_OFFSET + core::mem::offset_of!(ethtool_cmis_cdb_request, id) as u32; err = __ethtool_cmis_cdb_execute_cmd(dev, &mut page_data, ETHTOOL_CMIS_CDB_CMD_PAGE, offset, core::mem::size_of_val(&(*args).req.id) as u32, &(*args).req.id as *const _ as *const _); if err < 0 { return err; } err = cmis_cdb_wait_for_completion(dev, args); if err < 0 { return err; } err = cmis_cdb_wait_for_status(dev, args); if err < 0 { return err; } cmis_cdb_process_reply(dev, &mut page_data, args) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
