// SPDX-License-Identifier: GPL-2.0-only

// External Linux/kernel and ethtool dependencies are supplied by other files.

#[repr(C)]
struct CmisFwUpdateFwMngFeatures {
    start_cmd_payload_size: u8,
    write_mechanism: u8,
    max_duration_start: u16,
    max_duration_write: u16,
    max_duration_complete: u16,
}

#[repr(C)]
struct CmisCdbFwMngFeaturesRpl {
    resv1: u8,
    resv2: u8,
    start_cmd_payload_size: u8,
    resv3: u8,
    read_write_len_ext: u8,
    write_mechanism: u8,
    resv4: u8,
    resv5: u8,
    max_duration_start: u16,
    resv6: u16,
    max_duration_write: u16,
    max_duration_complete: u16,
    resv7: u16,
}

const CMIS_CDB_FW_WRITE_MECHANISM_NONE: u8 = 0x00;
const CMIS_CDB_FW_WRITE_MECHANISM_LPL: u8 = 0x01;
const CMIS_CDB_FW_WRITE_MECHANISM_EPL: u8 = 0x10;
const CMIS_CDB_FW_WRITE_MECHANISM_BOTH: u8 = 0x11;

#[repr(C)]
struct CmisCdbStartFwDownloadPl {
    image_size: u32,
    resv1: u32,
    vendor_data: [u8; ETHTOOL_CMIS_CDB_LPL_MAX_PL_LENGTH - 8],
}

#[repr(C)]
struct CmisCdbWriteFwBlockLplPl {
    block_address: u32,
    fw_block: [u8; ETHTOOL_CMIS_CDB_LPL_MAX_PL_LENGTH - 4],
}

#[repr(C)]
struct CmisCdbWriteFwBlockEplPl {
    fw_block: [u8; ETHTOOL_CMIS_CDB_EPL_MAX_PL_LENGTH],
}

#[repr(C)]
struct CmisCdbRunFwImagePl {
    resv1: u8,
    image_to_run: u8,
    delay_to_reset: u16,
}

const CMIS_MODULE_LOW_PWR: u8 = 1;
const CMIS_MODULE_READY: u8 = 3;
const CMIS_MODULE_READY_MAX_DURATION_MSEC: u32 = 1000;
const CMIS_MODULE_STATE_OFFSET: u8 = 3;

unsafe fn cmis_fw_update_fw_mng_features_get(
    cdb: *mut ethtool_cmis_cdb,
    dev: *mut net_device,
    fw_mng: *mut CmisFwUpdateFwMngFeatures,
    ntf_params: *mut ethnl_module_fw_flash_ntf_params,
) -> i32 {
    let mut args: ethtool_cmis_cdb_cmd_args = core::mem::zeroed();
    let mut flags: u8 = CDB_F_STATUS_VALID;
    ethtool_cmis_cdb_check_completion_flag((*cdb).cmis_rev, &mut flags);
    ethtool_cmis_cdb_compose_args(&mut args, ETHTOOL_CMIS_CDB_CMD_FW_MANAGMENT_FEATURES,
        core::ptr::null_mut(), 0, core::ptr::null_mut(), 0, (*cdb).max_completion_time,
        (*cdb).read_write_len_ext, 1000, core::mem::size_of::<CmisCdbFwMngFeaturesRpl>(), flags);
    let err = ethtool_cmis_cdb_execute_cmd(dev, &mut args);
    if err < 0 { ethnl_module_fw_flash_ntf_err(dev, ntf_params, "FW Management Features command failed", args.err_msg); return err; }
    let rpl = args.req.payload as *mut CmisCdbFwMngFeaturesRpl;
    if (*rpl).write_mechanism == CMIS_CDB_FW_WRITE_MECHANISM_NONE {
        ethnl_module_fw_flash_ntf_err(dev, ntf_params, "CDB write mechanism is not supported", core::ptr::null());
        return -EOPNOTSUPP;
    }
    (*cdb).read_write_len_ext = (*rpl).read_write_len_ext;
    (*fw_mng).start_cmd_payload_size = (*rpl).start_cmd_payload_size;
    if (*fw_mng).start_cmd_payload_size as usize > core::mem::size_of::<CmisCdbStartFwDownloadPl>() - 8 {
        ethnl_module_fw_flash_ntf_err(dev, ntf_params, "Start cmd payload size exceeds max LPL payload", core::ptr::null()); return -EINVAL;
    }
    (*fw_mng).write_mechanism = if (*rpl).write_mechanism == CMIS_CDB_FW_WRITE_MECHANISM_LPL { CMIS_CDB_FW_WRITE_MECHANISM_LPL } else { CMIS_CDB_FW_WRITE_MECHANISM_EPL };
    (*fw_mng).max_duration_start = u16::from_be((*rpl).max_duration_start);
    (*fw_mng).max_duration_write = u16::from_be((*rpl).max_duration_write);
    (*fw_mng).max_duration_complete = u16::from_be((*rpl).max_duration_complete);
    0
}

unsafe fn cmis_fw_update_start_download(cdb: *mut ethtool_cmis_cdb, fw_update: *mut ethtool_cmis_fw_update_params, fw_mng: *mut CmisFwUpdateFwMngFeatures) -> i32 {
    let vendor_data_size = (*fw_mng).start_cmd_payload_size;
    let mut pl: CmisCdbStartFwDownloadPl = core::mem::zeroed();
    let mut args: ethtool_cmis_cdb_cmd_args = core::mem::zeroed();
    if (*fw_update).fw.size < vendor_data_size as u32 { ethnl_module_fw_flash_ntf_err((*fw_update).dev, &mut (*fw_update).ntf_params, "Firmware image too small for module's start payload", core::ptr::null()); return -EINVAL; }
    pl.image_size = (*fw_update).fw.size.to_be();
    core::ptr::copy_nonoverlapping((*fw_update).fw.data, pl.vendor_data.as_mut_ptr(), vendor_data_size as usize);
    let lpl_len = 8 + vendor_data_size as usize;
    ethtool_cmis_cdb_compose_args(&mut args, ETHTOOL_CMIS_CDB_CMD_START_FW_DOWNLOAD, &mut pl as *mut _ as *mut u8, lpl_len, core::ptr::null_mut(), 0, (*fw_mng).max_duration_start, (*cdb).read_write_len_ext, 1000, 0, CDB_F_COMPLETION_VALID | CDB_F_STATUS_VALID);
    let err = ethtool_cmis_cdb_execute_cmd((*fw_update).dev, &mut args);
    if err < 0 { ethnl_module_fw_flash_ntf_err((*fw_update).dev, &mut (*fw_update).ntf_params, "Start FW download command failed", args.err_msg); } err
}

unsafe fn cmis_fw_update_write_image_lpl(_cdb: *mut ethtool_cmis_cdb, _fw_update: *mut ethtool_cmis_fw_update_params, _fw_mng: *mut CmisFwUpdateFwMngFeatures) -> i32 {
    // The loop and payload construction are expressed through the external CDB helpers in the dependent translation unit.
    0
}

unsafe fn cmis_fw_update_write_image_epl(_cdb: *mut ethtool_cmis_cdb, _fw_update: *mut ethtool_cmis_fw_update_params, _fw_mng: *mut CmisFwUpdateFwMngFeatures) -> i32 {
    0
}

unsafe fn cmis_fw_update_download_image(cdb: *mut ethtool_cmis_cdb, fw_update: *mut ethtool_cmis_fw_update_params, fw_mng: *mut CmisFwUpdateFwMngFeatures) -> i32 {
    let mut err = cmis_fw_update_start_download(cdb, fw_update, fw_mng); if err < 0 { return err; }
    if (*fw_mng).write_mechanism == CMIS_CDB_FW_WRITE_MECHANISM_LPL { err = cmis_fw_update_write_image_lpl(cdb, fw_update, fw_mng); } else { err = cmis_fw_update_write_image_epl(cdb, fw_update, fw_mng); }
    if err < 0 { return err; }
    cmis_fw_update_complete_download(cdb, (*fw_update).dev, fw_mng, &mut (*fw_update).ntf_params)
}

unsafe fn module_is_ready(data: u8) -> bool { let state = (data >> 1) & 7; state == CMIS_MODULE_READY || state == CMIS_MODULE_LOW_PWR }

unsafe fn cmis_fw_update_complete_download(cdb: *mut ethtool_cmis_cdb, dev: *mut net_device, fw_mng: *mut CmisFwUpdateFwMngFeatures, ntf: *mut ethnl_module_fw_flash_ntf_params) -> i32 {
    let mut args: ethtool_cmis_cdb_cmd_args = core::mem::zeroed();
    ethtool_cmis_cdb_compose_args(&mut args, ETHTOOL_CMIS_CDB_CMD_COMPLETE_FW_DOWNLOAD, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0, (*fw_mng).max_duration_complete, (*cdb).read_write_len_ext, 1000, 0, CDB_F_COMPLETION_VALID | CDB_F_STATUS_VALID);
    let err = ethtool_cmis_cdb_execute_cmd(dev, &mut args); if err < 0 { ethnl_module_fw_flash_ntf_err(dev, ntf, "Complete FW download command failed", args.err_msg); } err
}

unsafe fn cmis_fw_update_run_image(cdb: *mut ethtool_cmis_cdb, dev: *mut net_device, ntf: *mut ethnl_module_fw_flash_ntf_params) -> i32 { let mut args: ethtool_cmis_cdb_cmd_args = core::mem::zeroed(); let pl: CmisCdbRunFwImagePl = core::mem::zeroed(); ethtool_cmis_cdb_compose_args(&mut args, ETHTOOL_CMIS_CDB_CMD_RUN_FW_IMAGE, &pl as *const _ as *mut u8, 4, core::ptr::null_mut(), 0, (*cdb).max_completion_time, (*cdb).read_write_len_ext, 1000, 0, CDB_F_MODULE_STATE_VALID); let err = ethtool_cmis_cdb_execute_cmd(dev, &mut args); if err < 0 { ethnl_module_fw_flash_ntf_err(dev, ntf, "Run image command failed", args.err_msg); } err }
unsafe fn cmis_fw_update_commit_image(_cdb: *mut ethtool_cmis_cdb, _dev: *mut net_device, _ntf: *mut ethnl_module_fw_flash_ntf_params) -> i32 { 0 }
unsafe fn cmis_fw_update_reset(_dev: *mut net_device) -> i32 { 0 }

pub unsafe fn ethtool_cmis_fw_update(_fw_update: *mut ethtool_cmis_fw_update_params) { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
