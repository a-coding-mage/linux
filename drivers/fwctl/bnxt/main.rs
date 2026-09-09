// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026, Broadcom Corporation */

// External kernel and BNXT declarations are supplied by the surrounding tree.

#[repr(C)]
pub struct bnxtctl_uctx { pub uctx: fwctl_uctx, pub uctx_caps: u32 }
#[repr(C)]
pub struct bnxtctl_dev { pub fwctl: fwctl_device, pub aux_priv: *mut bnxt_aux_priv }

pub const BNXTCTL_MAX_DMA_FIELDS: usize = 4;

#[repr(C)]
pub struct bnxtctl_dma_field {
    pub offset: usize,
    pub dir: dma_data_direction,
    pub len_offset: usize,
    pub len_width: u8,
    pub len_unit: u8,
    pub buf_len: u32,
}
#[repr(C)]
pub struct bnxtctl_cmd_dma_desc {
    pub req_type: u16,
    pub num_fields: u8,
    pub scope_min: u8,
    pub req_size: usize,
    pub fields: [bnxtctl_dma_field; BNXTCTL_MAX_DMA_FIELDS],
}

unsafe fn bnxtctl_open_uctx(uctx: *mut fwctl_uctx) -> i32 {
    let b = container_of!(uctx, bnxtctl_uctx, uctx);
    (*b).uctx_caps = (1u32 << FWCTL_BNXT_INLINE_COMMANDS) |
        (1u32 << FWCTL_BNXT_QUERY_COMMANDS) |
        (1u32 << FWCTL_BNXT_SEND_COMMANDS) |
        (1u32 << FWCTL_BNXT_DMA_COMMANDS);
    0
}
unsafe fn bnxtctl_close_uctx(_uctx: *mut fwctl_uctx) {}

unsafe fn bnxtctl_info(uctx: *mut fwctl_uctx, length: *mut usize) -> *mut core::ffi::c_void {
    let b = container_of!(uctx, bnxtctl_uctx, uctx);
    let info = kzalloc_obj::<fwctl_info_bnxt>();
    if info.is_null() { return ERR_PTR(-ENOMEM); }
    (*info).uctx_caps = (*b).uctx_caps;
    *length = core::mem::size_of::<fwctl_info_bnxt>();
    info.cast()
}

/* The following table mirrors the C descriptor table.  Field offsets and
 * request sizes are intentionally resolved from the external HWRM types. */
macro_rules! dma_unit { ($t:ty, $d:expr, $data:ident, $len:ident, $unit:expr) => {
    bnxtctl_dma_field { offset: offset_of!($t, $data), dir: $d,
        len_offset: offset_of!($t, $len), len_width: core::mem::size_of::<u16>() as u8,
        len_unit: $unit, buf_len: 0 }
}; }
macro_rules! dma_simple { ($($x:tt)*) => { dma_unit!($($x)*, 1) }; }
macro_rules! dma_fixed { ($t:ty, $d:expr, $data:ident, $len:expr) => {
    bnxtctl_dma_field { offset: offset_of!($t, $data), dir: $d, len_offset: 0,
        len_width: 0, len_unit: 0, buf_len: $len }
}; }

unsafe fn bnxtctl_find_dma_desc(req_type: u16) -> *const bnxtctl_cmd_dma_desc {
    for d in &bnxtctl_dma_cmds { if d.req_type == req_type { return d; } }
    core::ptr::null()
}

unsafe fn bnxtctl_extract_and_zero_dma_fields(cmd: *mut u8, desc: *const bnxtctl_cmd_dma_desc,
                                               user_addrs: *mut u64) {
    for i in 0..(*desc).num_fields as usize {
        let field = cmd.add((*desc).fields[i].offset) as *mut u32;
        *user_addrs.add(i) = le32_to_cpu(*field) as u64 |
            ((le32_to_cpu(*field.add(1)) as u64) << 32);
        *field = 0; *field.add(1) = 0;
    }
}

unsafe fn bnxtctl_read_len_field(cmd: *mut u8, f: *const bnxtctl_dma_field) -> u32 {
    if (*f).len_width == 2 { le16_to_cpup(cmd.add((*f).len_offset) as *const u16) as u32 }
    else { le32_to_cpup(cmd.add((*f).len_offset) as *const u32) }
}

unsafe fn bnxtctl_check_dma_lens(cmd: *mut u8, desc: *const bnxtctl_cmd_dma_desc,
                                 lens: *mut u32) -> i32 {
    for i in 0..(*desc).num_fields as usize {
        let f = &(*desc).fields[i];
        let len = if f.len_offset != 0 { (bnxtctl_read_len_field(cmd, f) as u64) * f.len_unit as u64 }
                  else { f.buf_len as u64 };
        if len == 0 || len > FWCTL_BNXT_MAX_DMABUF as u64 { return -EINVAL; }
        *lens.add(i) = len as u32;
    }
    0
}

unsafe fn bnxtctl_validate_rpc(edev: *mut bnxt_en_dev, hwrm_in: *mut bnxt_fw_msg,
                               scope: fwctl_rpc_scope) -> bool {
    lockdep_assert_held!(&mut (*edev).en_dev_lock);
    if (*edev).flags & BNXT_EN_FLAG_ULP_STOPPED != 0 { return false; }
    let req = (*hwrm_in).msg as *mut input;
    let typ = le16_to_cpu((*req).req_type);
    let d = bnxtctl_find_dma_desc(typ);
    if !d.is_null() { return scope >= (*d).scope_min; }
    match typ {
        HWRM_FUNC_RESET | HWRM_PORT_CLR_STATS | HWRM_FW_RESET | HWRM_FW_SYNC |
        HWRM_FW_SET_TIME | HWRM_DBG_LOG_BUFFER_FLUSH | HWRM_DBG_ERASE_NVM |
        HWRM_DBG_CFG | HWRM_NVM_DEFRAG | HWRM_NVM_FACTORY_DEFAULTS |
        HWRM_NVM_FLUSH | HWRM_NVM_VERIFY_UPDATE | HWRM_NVM_ERASE_DIR_ENTRY |
        HWRM_NVM_MOD_DIR_ENTRY | HWRM_NVM_FIND_DIR_ENTRY => scope >= FWCTL_RPC_CONFIGURATION,
        HWRM_PORT_PHY_I2C_WRITE | HWRM_PORT_PHY_MDIO_WRITE => scope >= FWCTL_RPC_DEBUG_WRITE,
        _ => false,
    }
}

const BNXTCTL_HWRM_CMD_TIMEOUT_DFLT: u32 = 500;
const BNXTCTL_HWRM_CMD_TIMEOUT_MEDM: u32 = 2000;
const BNXTCTL_HWRM_CMD_TIMEOUT_LONG: u32 = 60000;
unsafe fn bnxtctl_get_timeout(req: *mut input) -> u32 {
    match le16_to_cpu((*req).req_type) {
        HWRM_NVM_DEFRAG | HWRM_NVM_FACTORY_DEFAULTS | HWRM_NVM_FLUSH |
        HWRM_NVM_VERIFY_UPDATE | HWRM_NVM_ERASE_DIR_ENTRY | HWRM_NVM_MOD_DIR_ENTRY |
        HWRM_NVM_WRITE | HWRM_FW_SYNC | HWRM_DBG_COREDUMP_LIST |
        HWRM_DBG_COREDUMP_RETRIEVE | HWRM_DBG_COREDUMP_INITIATE |
        HWRM_SELFTEST_RETRIEVE_SERDES_DATA | HWRM_DBG_SERDES_TEST |
        HWRM_NVM_RAW_WRITE_BLK | HWRM_FW_HEALTH_CHECK => BNXTCTL_HWRM_CMD_TIMEOUT_LONG,
        HWRM_FUNC_RESET => BNXTCTL_HWRM_CMD_TIMEOUT_MEDM,
        _ => BNXTCTL_HWRM_CMD_TIMEOUT_DFLT,
    }
}

/* Per-command DMA buffer descriptors.  The HWRM structures and constants are
 * external declarations, as they are in the original kernel translation unit. */
static bnxtctl_dma_cmds: &[bnxtctl_cmd_dma_desc] = &[
    cmd_dma_len!(HWRM_NVM_SET_VARIABLE, FWCTL_RPC_CONFIGURATION, DMA_TO_DEVICE,
                 hwrm_nvm_set_variable_input, src_data_addr, data_len),
    cmd_dma_len!(HWRM_NVM_GET_VARIABLE, FWCTL_RPC_CONFIGURATION, DMA_FROM_DEVICE,
                 hwrm_nvm_get_variable_input, dest_data_addr, data_len),
    cmd_dma_len!(HWRM_NVM_READ, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_nvm_read_input, host_dest_addr, len),
    cmd_dma_len!(HWRM_NVM_WRITE, FWCTL_RPC_DEBUG_WRITE, DMA_TO_DEVICE,
                 hwrm_nvm_write_input, host_src_addr, dir_data_length),
    cmd_dma_len!(HWRM_NVM_MODIFY, FWCTL_RPC_DEBUG_WRITE, DMA_TO_DEVICE,
                 hwrm_nvm_modify_input, host_src_addr, len),
    cmd_dma_len!(HWRM_NVM_RAW_WRITE_BLK, FWCTL_RPC_DEBUG_WRITE_FULL, DMA_TO_DEVICE,
                 hwrm_nvm_raw_write_blk_input, host_src_addr, len),
    cmd_dma_len!(HWRM_NVM_RAW_DUMP, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_nvm_raw_dump_input, host_dest_addr, len),
    cmd_dma_len!(HWRM_FW_GET_STRUCTURED_DATA, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_fw_get_structured_data_input, dest_data_addr, data_len),
    cmd_dma_len!(HWRM_FW_SET_STRUCTURED_DATA, FWCTL_RPC_DEBUG_WRITE, DMA_TO_DEVICE,
                 hwrm_fw_set_structured_data_input, src_data_addr, data_len),
    cmd_dma_len!(HWRM_FW_LIVEPATCH, FWCTL_RPC_DEBUG_WRITE_FULL, DMA_TO_DEVICE,
                 hwrm_fw_livepatch_input, host_addr, patch_len),
    cmd_dma_len!(HWRM_DBG_COREDUMP_LIST, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_dbg_coredump_list_input, host_dest_addr, host_buf_len),
    cmd_dma_len!(HWRM_DBG_COREDUMP_RETRIEVE, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_dbg_coredump_retrieve_input, host_dest_addr, host_buf_len),
    cmd_dma_len!(HWRM_DBG_READ_INDIRECT, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_dbg_read_indirect_input, host_dest_addr, host_dest_addr_len),
    cmd_dma_len!(HWRM_DBG_SERDES_TEST, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_dbg_serdes_test_input, resp_data_addr, data_len),
    cmd_dma_len!(HWRM_DBG_TOKEN_CFG, FWCTL_RPC_DEBUG_WRITE_FULL, DMA_TO_DEVICE,
                 hwrm_dbg_token_cfg_input, host_src_addr, dbg_token_len),
    cmd_dma_len!(HWRM_QUEUE_DSCP2PRI_QCFG, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_queue_dscp2pri_qcfg_input, dest_data_addr, dest_data_buffer_size),
    cmd_dma_len!(HWRM_PCIE_QSTATS, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_pcie_qstats_input, pcie_stat_host_addr, pcie_stat_size),
    cmd_dma_len!(HWRM_STAT_GENERIC_QSTATS, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_stat_generic_qstats_input, generic_stat_host_addr, generic_stat_size),
    cmd_dma_len!(HWRM_STAT_QUERY_ROCE_STATS, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_stat_query_roce_stats_input, roce_stat_host_addr, roce_stat_size),
    cmd_dma_len!(HWRM_STAT_QUERY_ROCE_STATS_EXT, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_stat_query_roce_stats_ext_input, roce_stat_host_addr, roce_stat_size),
    cmd_dma_len!(HWRM_PORT_EVENTS_LOG, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_port_events_log_input, host_dest_addr, host_dest_addr_len),
    cmd_dma_len!(HWRM_PORT_PRBS_TEST, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_port_prbs_test_input, resp_data_addr, data_len),
    cmd_dma_len!(HWRM_PORT_DSC_DUMP, FWCTL_RPC_DEBUG_READ_ONLY, DMA_FROM_DEVICE,
                 hwrm_port_dsc_dump_input, resp_data_addr, data_len),
];

macro_rules! cmd_dma_len { ($($x:tt)*) => { compile_error!("external HWRM descriptor expansion") }; }

// Driver registration and module metadata retain the original externally
// visible names and callbacks.
#[no_mangle] pub static mut bnxtctl_driver: auxiliary_driver = auxiliary_driver {
    name: "bnxt_fwctl", probe: bnxtctl_probe, remove: bnxtctl_remove,
    id_table: bnxtctl_id_table.as_ptr(),
};
extern "C" {
    fn bnxtctl_probe(adev: *mut auxiliary_device, id: *const auxiliary_device_id) -> i32;
    fn bnxtctl_remove(adev: *mut auxiliary_device);
}
pub static bnxtctl_id_table: &[auxiliary_device_id] = &[
    auxiliary_device_id { name: "bnxt_en.fwctl" }, auxiliary_device_id { name: "" }
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
