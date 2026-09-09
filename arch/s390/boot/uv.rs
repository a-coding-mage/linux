// SPDX-License-Identifier: GPL-2.0
// C dependencies: <asm/uv.h>, <asm/boot_data.h>, <asm/facility.h>,
// <asm/sections.h>, "boot.h", and "uv.h".

use core::ffi::c_void;

#[repr(C)]
pub struct UvCbHeader {
    pub cmd: u16,
    pub len: u16,
    pub rc: u16,
}

#[repr(C)]
pub struct UvCbQui {
    pub header: UvCbHeader,
    pub inst_calls_list: [u8; 32],
    pub uv_base_stor_len: u64,
    pub conf_base_phys_stor_len: u64,
    pub conf_base_virt_stor_len: u64,
    pub conf_virt_var_stor_len: u64,
    pub cpu_stor_len: u64,
    pub max_guest_stor_addr: u64,
    pub max_num_sec_conf: u64,
    pub max_guest_cpu_id: u16,
    pub uv_feature_indications: u64,
    pub supp_se_hdr_versions: u8,
    pub supp_se_hdr_pcf: u64,
    pub conf_dump_storage_state_len: u64,
    pub conf_dump_finalize_len: u64,
    pub supp_att_req_hdr_ver: u8,
    pub supp_att_pflags: u64,
    pub supp_add_secret_req_ver: u8,
    pub supp_add_secret_pcf: u64,
    pub supp_secret_types: u64,
    pub max_assoc_secrets: u64,
    pub max_retr_secrets: u64,
}

#[repr(C)]
pub struct UvInfo {
    pub inst_calls_list: [u8; 32],
    pub uv_base_stor_len: u64,
    pub guest_base_stor_len: u64,
    pub guest_virt_base_stor_len: u64,
    pub guest_virt_var_stor_len: u64,
    pub guest_cpu_stor_len: u64,
    pub max_sec_stor_addr: usize,
    pub max_num_sec_conf: u64,
    pub max_guest_cpu_id: u16,
    pub uv_feature_indications: u64,
    pub supp_se_hdr_ver: u8,
    pub supp_se_hdr_pcf: u64,
    pub conf_dump_storage_state_len: u64,
    pub conf_dump_finalize_len: u64,
    pub supp_att_req_hdr_ver: u8,
    pub supp_att_pflags: u64,
    pub supp_add_secret_req_ver: u8,
    pub supp_add_secret_pcf: u64,
    pub supp_secret_types: u64,
    pub max_assoc_secrets: u64,
    pub max_retr_secrets: u64,
}

extern "C" {
    pub static mut prot_virt_guest: i32;
    pub static mut prot_virt_host: i32;
    pub static mut uv_info: UvInfo;
    pub static oldmem_data: OldmemData;
    pub static mut ipl_block_valid: bool;

    fn test_facility(facility: u32) -> bool;
    fn uv_call(func: u64, uv_cb: u64) -> i32;
    fn test_bit_inv(bit: usize, addr: *mut usize) -> bool;
    fn is_prot_virt_host() -> bool;
    fn is_prot_virt_guest() -> bool;
    fn is_ipl_block_dump() -> bool;
}

#[repr(C)]
pub struct OldmemData {
    pub start: u64,
}

pub const UVC_CMD_QUI: u16 = 0x01;
pub const UVC_RC_MORE_DATA: u16 = 0x0100;
pub const BIT_UVC_CMD_SET_SHARED_ACCESS: usize = 0;
pub const BIT_UVC_CMD_REMOVE_SHARED_ACCESS: usize = 1;
pub const PAGE_SIZE: usize = 4096;

pub unsafe fn uv_query_info() {
    let mut uvcb = UvCbQui {
        header: UvCbHeader { cmd: UVC_CMD_QUI, len: core::mem::size_of::<UvCbQui>() as u16, rc: 0 },
        inst_calls_list: [0; 32],
        uv_base_stor_len: 0, conf_base_phys_stor_len: 0, conf_base_virt_stor_len: 0,
        conf_virt_var_stor_len: 0, cpu_stor_len: 0, max_guest_stor_addr: 0,
        max_num_sec_conf: 0, max_guest_cpu_id: 0, uv_feature_indications: 0,
        supp_se_hdr_versions: 0, supp_se_hdr_pcf: 0, conf_dump_storage_state_len: 0,
        conf_dump_finalize_len: 0, supp_att_req_hdr_ver: 0, supp_att_pflags: 0,
        supp_add_secret_req_ver: 0, supp_add_secret_pcf: 0, supp_secret_types: 0,
        max_assoc_secrets: 0, max_retr_secrets: 0,
    };

    if !test_facility(158) { return; }
    if uv_call(0, (&mut uvcb as *mut UvCbQui) as u64) != 0 && uvcb.header.rc != UVC_RC_MORE_DATA { return; }

    // CONFIG_KVM conditional from the C source.
    if cfg!(feature = "CONFIG_KVM") {
        uv_info.inst_calls_list.copy_from_slice(&uvcb.inst_calls_list);
        uv_info.uv_base_stor_len = uvcb.uv_base_stor_len;
        uv_info.guest_base_stor_len = uvcb.conf_base_phys_stor_len;
        uv_info.guest_virt_base_stor_len = uvcb.conf_base_virt_stor_len;
        uv_info.guest_virt_var_stor_len = uvcb.conf_virt_var_stor_len;
        uv_info.guest_cpu_stor_len = uvcb.cpu_stor_len;
        uv_info.max_sec_stor_addr = (uvcb.max_guest_stor_addr + PAGE_SIZE as u64 - 1) & !(PAGE_SIZE as u64 - 1);
        uv_info.max_num_sec_conf = uvcb.max_num_sec_conf;
        uv_info.max_guest_cpu_id = uvcb.max_guest_cpu_id;
        uv_info.uv_feature_indications = uvcb.uv_feature_indications;
        uv_info.supp_se_hdr_ver = uvcb.supp_se_hdr_versions;
        uv_info.supp_se_hdr_pcf = uvcb.supp_se_hdr_pcf;
        uv_info.conf_dump_storage_state_len = uvcb.conf_dump_storage_state_len;
        uv_info.conf_dump_finalize_len = uvcb.conf_dump_finalize_len;
        uv_info.supp_att_req_hdr_ver = uvcb.supp_att_req_hdr_ver;
        uv_info.supp_att_pflags = uvcb.supp_att_pflags;
        uv_info.supp_add_secret_req_ver = uvcb.supp_add_secret_req_ver;
        uv_info.supp_add_secret_pcf = uvcb.supp_add_secret_pcf;
        uv_info.supp_secret_types = uvcb.supp_secret_types;
        uv_info.max_assoc_secrets = uvcb.max_assoc_secrets;
        uv_info.max_retr_secrets = uvcb.max_retr_secrets;
    }
    if test_bit_inv(BIT_UVC_CMD_SET_SHARED_ACCESS, uvcb.inst_calls_list.as_mut_ptr() as *mut usize)
        && test_bit_inv(BIT_UVC_CMD_REMOVE_SHARED_ACCESS, uvcb.inst_calls_list.as_mut_ptr() as *mut usize) {
        prot_virt_guest = 1;
    }
}

pub unsafe fn adjust_to_uv_max(mut limit: usize) -> usize {
    if is_prot_virt_host() && uv_info.max_sec_stor_addr != 0 { limit = core::cmp::min(limit, uv_info.max_sec_stor_addr); }
    limit
}

unsafe fn is_prot_virt_host_capable() -> i32 {
    if !is_prot_virt_host() || is_prot_virt_guest() || !test_facility(158) || oldmem_data.start != 0 || (ipl_block_valid && is_ipl_block_dump()) { return 0; }
    1
}

pub unsafe fn sanitize_prot_virt_host() { prot_virt_host = is_prot_virt_host_capable(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
