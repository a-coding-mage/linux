/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding translation unit. */

pub const TRANSPORT_FLAG_PASSTHROUGH: u32 = 0x1;
/*
 * ALUA commands, state checks and setup operations are handled by the
 * backend module.
 */
pub const TRANSPORT_FLAG_PASSTHROUGH_ALUA: u32 = 0x2;
pub const TRANSPORT_FLAG_PASSTHROUGH_PGR: u32 = 0x4;

#[repr(C)]
pub struct block_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct target_backend_ops {
    pub name: [::std::os::raw::c_char; 16],
    pub inquiry_prod: [::std::os::raw::c_char; 16],
    pub inquiry_rev: [::std::os::raw::c_char; 4],
    pub owner: *mut module,

    pub transport_flags_default: u8,
    pub transport_flags_changeable: u8,

    pub attach_hba: Option<unsafe extern "C" fn(*mut se_hba, u32) -> ::std::os::raw::c_int>,
    pub detach_hba: Option<unsafe extern "C" fn(*mut se_hba)>,
    pub pmode_enable_hba: Option<unsafe extern "C" fn(*mut se_hba, ::std::os::raw::c_ulong) -> ::std::os::raw::c_int>,

    pub alloc_device: Option<unsafe extern "C" fn(*mut se_hba, *const ::std::os::raw::c_char) -> *mut se_device>,
    pub configure_device: Option<unsafe extern "C" fn(*mut se_device) -> ::std::os::raw::c_int>,
    pub destroy_device: Option<unsafe extern "C" fn(*mut se_device)>,
    pub free_device: Option<unsafe extern "C" fn(*mut se_device)>,
    pub plug_device: Option<unsafe extern "C" fn(*mut se_device) -> *mut se_dev_plug>,
    pub unplug_device: Option<unsafe extern "C" fn(*mut se_dev_plug)>,

    pub configure_unmap: Option<unsafe extern "C" fn(*mut se_device) -> bool>,
    pub set_configfs_dev_params: Option<unsafe extern "C" fn(*mut se_device, *const ::std::os::raw::c_char, ssize_t) -> ssize_t>,
    pub show_configfs_dev_params: Option<unsafe extern "C" fn(*mut se_device, *mut ::std::os::raw::c_char) -> ssize_t>,

    pub parse_cdb: Option<unsafe extern "C" fn(*mut se_cmd) -> sense_reason_t>,
    pub tmr_notify: Option<unsafe extern "C" fn(*mut se_device, tcm_tmreq_table, *mut list_head)>,
    pub get_device_type: Option<unsafe extern "C" fn(*mut se_device) -> u32>,
    pub get_blocks: Option<unsafe extern "C" fn(*mut se_device) -> sector_t>,
    pub get_alignment_offset_lbas: Option<unsafe extern "C" fn(*mut se_device) -> sector_t>,
    /* lbppbe = logical blocks per physical block exponent. see SBC-3 */
    pub get_lbppbe: Option<unsafe extern "C" fn(*mut se_device) -> ::std::os::raw::c_uint>,
    pub get_io_min: Option<unsafe extern "C" fn(*mut se_device) -> ::std::os::raw::c_uint>,
    pub get_io_opt: Option<unsafe extern "C" fn(*mut se_device) -> ::std::os::raw::c_uint>,
    pub get_sense_buffer: Option<unsafe extern "C" fn(*mut se_cmd) -> *mut u8>,
    pub get_write_cache: Option<unsafe extern "C" fn(*mut se_device) -> bool>,
    pub init_prot: Option<unsafe extern "C" fn(*mut se_device) -> ::std::os::raw::c_int>,
    pub format_prot: Option<unsafe extern "C" fn(*mut se_device) -> ::std::os::raw::c_int>,
    pub free_prot: Option<unsafe extern "C" fn(*mut se_device)>,

    pub tb_dev_attrib_attrs: *mut *mut configfs_attribute,
    pub tb_dev_action_attrs: *mut *mut configfs_attribute,
}

#[repr(C)]
pub struct exec_cmd_ops {
    pub execute_rw: Option<unsafe extern "C" fn(*mut se_cmd, *mut scatterlist, u32, dma_data_direction) -> sense_reason_t>,
    pub execute_sync_cache: Option<unsafe extern "C" fn(*mut se_cmd) -> sense_reason_t>,
    pub execute_write_same: Option<unsafe extern "C" fn(*mut se_cmd) -> sense_reason_t>,
    pub execute_unmap: Option<unsafe extern "C" fn(*mut se_cmd, sector_t, sector_t) -> sense_reason_t>,
    pub execute_pr_out: Option<unsafe extern "C" fn(*mut se_cmd, u8, u64, u64, u8, bool) -> sense_reason_t>,
    pub execute_pr_in: Option<unsafe extern "C" fn(*mut se_cmd, u8, *mut u8) -> sense_reason_t>,
}

extern "C" {
    pub fn transport_backend_register(ops: *const target_backend_ops) -> ::std::os::raw::c_int;
    pub fn target_backend_unregister(ops: *const target_backend_ops);
    pub fn target_complete_cmd(cmd: *mut se_cmd, status: u8);
    pub fn target_set_cmd_data_length(cmd: *mut se_cmd, length: ::std::os::raw::c_int);
    pub fn target_complete_cmd_with_sense(cmd: *mut se_cmd, status: u8, reason: sense_reason_t);
    pub fn target_complete_cmd_with_length(cmd: *mut se_cmd, status: u8, length: ::std::os::raw::c_int);
    pub fn transport_copy_sense_to_cmd(cmd: *mut se_cmd, sense: *mut u8);
    pub fn spc_parse_cdb(cmd: *mut se_cmd, size: *mut ::std::os::raw::c_uint) -> sense_reason_t;
    pub fn spc_emulate_report_luns(cmd: *mut se_cmd) -> sense_reason_t;
    pub fn spc_emulate_inquiry_std(cmd: *mut se_cmd, buf: *mut u8) -> sense_reason_t;
    pub fn spc_emulate_evpd_83(cmd: *mut se_cmd, buf: *mut u8) -> sense_reason_t;
    pub fn sbc_parse_cdb(cmd: *mut se_cmd, ops: *mut exec_cmd_ops) -> sense_reason_t;
    pub fn sbc_get_device_rev(dev: *mut se_device) -> u32;
    pub fn sbc_get_device_type(dev: *mut se_device) -> u32;
    pub fn sbc_get_write_same_sectors(cmd: *mut se_cmd) -> sector_t;
    pub fn sbc_dif_generate(cmd: *mut se_cmd);
    pub fn sbc_dif_verify(cmd: *mut se_cmd, a: sector_t, b: ::std::os::raw::c_uint, c: ::std::os::raw::c_uint, sg: *mut scatterlist, d: ::std::os::raw::c_int) -> sense_reason_t;
    pub fn sbc_dif_copy_prot(cmd: *mut se_cmd, a: ::std::os::raw::c_uint, b: bool, sg: *mut scatterlist, c: ::std::os::raw::c_int);
    pub fn transport_set_vpd_proto_id(vpd: *mut t10_vpd, buf: *mut u8);
    pub fn transport_set_vpd_assoc(vpd: *mut t10_vpd, buf: *mut u8) -> ::std::os::raw::c_int;
    pub fn transport_set_vpd_ident_type(vpd: *mut t10_vpd, buf: *mut u8) -> ::std::os::raw::c_int;
    pub fn transport_set_vpd_ident(vpd: *mut t10_vpd, buf: *mut u8) -> ::std::os::raw::c_int;
    pub fn transport_kmap_data_sg(cmd: *mut se_cmd) -> *mut ::std::ffi::c_void;
    pub fn transport_kunmap_data_sg(cmd: *mut se_cmd);
    pub fn transport_generic_map_mem_to_cmd(cmd: *mut se_cmd, a: *mut scatterlist, b: u32, c: *mut scatterlist, d: u32) -> sense_reason_t;
    pub fn target_lun_is_rdonly(cmd: *mut se_cmd) -> bool;
    pub fn passthrough_parse_cdb(cmd: *mut se_cmd, exec_cmd: Option<unsafe extern "C" fn(*mut se_cmd) -> sense_reason_t>) -> sense_reason_t;
    pub fn target_sense_desc_format(dev: *mut se_device) -> bool;
    pub fn target_to_linux_sector(dev: *mut se_device, lb: sector_t) -> sector_t;
    pub fn target_configure_unmap_from_bdev(attrib: *mut se_dev_attrib, bdev: *mut block_device) -> bool;
    pub fn target_configure_write_atomic_from_bdev(attrib: *mut se_dev_attrib, bdev: *mut block_device);
}

extern "C" {
    pub static mut sbc_attrib_attrs: *mut *mut configfs_attribute;
    pub static mut passthrough_attrib_attrs: *mut *mut configfs_attribute;
    pub static mut passthrough_pr_attrib_attrs: *mut *mut configfs_attribute;
}

#[inline]
pub unsafe fn target_dev_configured(se_dev: *mut se_device) -> bool {
    ((*se_dev).dev_flags & DF_CONFIGURED) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
