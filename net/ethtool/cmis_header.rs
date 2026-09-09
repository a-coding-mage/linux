/* SPDX-License-Identifier: GPL-2.0-only */

pub const ETHTOOL_CMIS_CDB_LPL_MAX_PL_LENGTH: usize = 120;
pub const ETHTOOL_CMIS_CDB_EPL_MAX_PL_LENGTH: usize = 2048;
pub const ETHTOOL_CMIS_CDB_CMD_PAGE: u8 = 0x9f;
pub const ETHTOOL_CMIS_CDB_PAGE_I2C_ADDR: u8 = 0x50;

/**
 * struct ethtool_cmis_cdb - CDB commands parameters
 * @cmis_rev: CMIS revision major.
 * @read_write_len_ext: Allowable additional number of byte octets to the LPL
 *                      in a READ or a WRITE CDB commands.
 * @max_completion_time:  Maximum CDB command completion time in msec.
 */
#[repr(C)]
pub struct ethtool_cmis_cdb {
    pub cmis_rev: u8,
    pub read_write_len_ext: u8,
    pub max_completion_time: u16,
}

#[repr(i32)]
pub enum ethtool_cmis_cdb_cmd_id {
    ETHTOOL_CMIS_CDB_CMD_QUERY_STATUS = 0x0000,
    ETHTOOL_CMIS_CDB_CMD_MODULE_FEATURES = 0x0040,
    ETHTOOL_CMIS_CDB_CMD_FW_MANAGMENT_FEATURES = 0x0041,
    ETHTOOL_CMIS_CDB_CMD_START_FW_DOWNLOAD = 0x0101,
    ETHTOOL_CMIS_CDB_CMD_WRITE_FW_BLOCK_LPL = 0x0103,
    ETHTOOL_CMIS_CDB_CMD_WRITE_FW_BLOCK_EPL = 0x0104,
    ETHTOOL_CMIS_CDB_CMD_COMPLETE_FW_DOWNLOAD = 0x0107,
    ETHTOOL_CMIS_CDB_CMD_RUN_FW_IMAGE = 0x0109,
    ETHTOOL_CMIS_CDB_CMD_COMMIT_FW_IMAGE = 0x010a,
}

/**
 * struct ethtool_cmis_cdb_request - CDB commands request fields as decribed in
 *                              the CMIS standard
 * @id: Command ID.
 * @epl_len: EPL memory length.
 * @lpl_len: LPL memory length.
 * @chk_code: Check code for the previous field and the payload.
 * @resv1: Added to match the CMIS standard request continuity.
 * @resv2: Added to match the CMIS standard request continuity.
 * @payload: Payload for the CDB commands.
 * @epl: Extended payload for the CDB commands.
 */
#[repr(C)]
pub struct ethtool_cmis_cdb_request {
    pub id: u16,
    pub epl_len: u16,
    pub lpl_len: u8,
    pub chk_code: u8,
    pub resv1: u8,
    pub resv2: u8,
    pub payload: [u8; ETHTOOL_CMIS_CDB_LPL_MAX_PL_LENGTH],
    pub epl: *mut u8, /* Everything above this field checksummed. */
}

pub const CDB_F_COMPLETION_VALID: u8 = 1 << 0;
pub const CDB_F_STATUS_VALID: u8 = 1 << 1;
pub const CDB_F_MODULE_STATE_VALID: u8 = 1 << 2;

/** CDB commands execution arguments. */
#[repr(C)]
pub struct ethtool_cmis_cdb_cmd_args {
    pub req: ethtool_cmis_cdb_request,
    pub max_duration: u16,
    pub msleep_pre_rpl: u16,
    pub read_write_len_ext: u8,
    pub rpl_exp_len: u8,
    pub flags: u8,
    pub err_msg: *mut ::std::os::raw::c_char,
}

#[repr(C)]
pub struct ethtool_cmis_cdb_rpl_hdr {
    pub rpl_len: u8,
    pub rpl_chk_code: u8,
}

#[repr(C)]
pub struct ethtool_cmis_cdb_rpl {
    pub hdr: ethtool_cmis_cdb_rpl_hdr,
    pub payload: [u8; ETHTOOL_CMIS_CDB_LPL_MAX_PL_LENGTH],
}

extern "C" {
    pub fn ethtool_cmis_get_max_lpl_size(num_of_byte_octs: u8) -> u32;

    pub fn ethtool_cmis_cdb_compose_args(
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
    );

    pub fn ethtool_cmis_cdb_check_completion_flag(cmis_rev: u8, flags: *mut u8);

    pub fn ethtool_cmis_page_init(
        page_data: *mut ethtool_module_eeprom,
        page: u8,
        offset: u32,
        length: u32,
    );

    pub fn ethtool_cmis_cdb_init(
        dev: *mut net_device,
        params: *const ethtool_module_fw_flash_params,
        ntf_params: *mut ethnl_module_fw_flash_ntf_params,
    ) -> *mut ethtool_cmis_cdb;
    pub fn ethtool_cmis_cdb_fini(cdb: *mut ethtool_cmis_cdb);

    pub fn ethtool_cmis_wait_for_cond(
        dev: *mut net_device,
        flags: u8,
        flag: u8,
        max_duration: u16,
        offset: u32,
        cond_success: Option<unsafe extern "C" fn(u8) -> bool>,
        cond_fail: Option<unsafe extern "C" fn(u8) -> bool>,
        state: *mut u8,
    ) -> ::std::os::raw::c_int;

    pub fn ethtool_cmis_cdb_execute_cmd(
        dev: *mut net_device,
        args: *mut ethtool_cmis_cdb_cmd_args,
    ) -> ::std::os::raw::c_int;
}

/* External types supplied by the surrounding ethtool/kernel translation. */
pub enum ethtool_module_eeprom {}
pub enum net_device {}
pub enum ethtool_module_fw_flash_params {}
pub enum ethnl_module_fw_flash_ntf_params {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
