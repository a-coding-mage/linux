/*
 * Copyright 2018 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

pub const PSP_HEADER_SIZE: u32 = 256;
pub const BINARY_SIGNATURE: u32 = 0x28211407;
pub const DISCOVERY_TABLE_SIGNATURE: u32 = 0x53445049;
pub const GC_TABLE_ID: u16 = 0x4347;
pub const HARVEST_TABLE_SIGNATURE: u32 = 0x56524148;
pub const VCN_INFO_TABLE_ID: u32 = 0x004E4356;
pub const MALL_INFO_TABLE_ID: u32 = 0x4C4C414D;
pub const NPS_INFO_TABLE_ID: u32 = 0x0053504E;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum table { IP_DISCOVERY = 0, GC, HARVEST_INFO, VCN_INFO, MALL_INFO, NPS_INFO, TOTAL_TABLES = 6 }

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct table_info { pub offset: u16, pub checksum: u16, pub size: u16, pub padding: u16 }

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct binary_header {
    pub binary_signature: u32, pub version_major: u16, pub version_minor: u16,
    pub binary_checksum: u16, pub binary_size: u16, pub table_list: [table_info; 6],
}

#[repr(C, packed)]
pub struct binary_header_v2 {
    pub binary_signature: u32, pub version_major: u16, pub version_minor: u16,
    pub binary_checksum: u16, pub binary_size: u16, pub num_tables: u16, pub padding: u16,
    pub table_list: [table_info; 0], // C flexible array, counted by num_tables
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct die_info { pub die_id: u16, pub die_offset: u16 }

#[repr(C, packed)]
pub union ip_discovery_header_tail { pub padding: [u16; 1], pub version4: ip_discovery_header_v4 }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ip_discovery_header_v4 { pub base_addr_64_bit: u8, pub reserved2: u8 }
#[repr(C, packed)]
pub struct ip_discovery_header {
    pub signature: u32, pub version: u16, pub size: u16, pub id: u32, pub num_dies: u16,
    pub die_info: [die_info; 16], pub tail: ip_discovery_header_tail,
}

#[repr(C, packed)]
pub struct ip { pub hw_id: u16, pub number_instance: u8, pub num_base_address: u8, pub major: u8, pub minor: u8, pub revision: u8, pub harvest_reserved: u8, pub base_address: [u32; 0] }
#[repr(C, packed)]
pub struct ip_v3 { pub hw_id: u16, pub instance_number: u8, pub num_base_address: u8, pub major: u8, pub minor: u8, pub revision: u8, pub sub_revision_variant: u8, pub base_address: [u32; 0] }
#[repr(C, packed)]
pub union ip_v4_addresses { pub base_address: [u32; 0], pub base_address_64: [u64; 0] }
#[repr(C, packed)]
pub struct ip_v4 { pub hw_id: u16, pub instance_number: u8, pub num_base_address: u8, pub major: u8, pub minor: u8, pub revision: u8, pub sub_revision_variant: u8, pub addresses: ip_v4_addresses }

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct die_header { pub die_id: u16, pub num_ips: u16 }
#[repr(C)]
pub union ip_list { pub ip_list: *mut ip, pub ip_v3_list: *mut ip_v3, pub ip_v4_list: *mut ip_v4 }
#[repr(C)]
pub struct ip_die { pub die_header: *mut die_header, pub list: ip_list }
#[repr(C)]
pub struct ip_structure { pub header: *mut ip_discovery_header, pub die: ip_die }

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct gpu_info_header { pub table_id: u32, pub version_major: u16, pub version_minor: u16, pub size: u32 }

macro_rules! gc_fields { ($name:ident, [$($field:ident),* $(,)?]) => { #[repr(C, packed)] pub struct $name { pub header: gpu_info_header, $(pub $field: u32,)* } }; }
gc_fields!(gc_info_v1_0, [gc_num_se,gc_num_wgp0_per_sa,gc_num_wgp1_per_sa,gc_num_rb_per_se,gc_num_gl2c,gc_num_gprs,gc_num_max_gs_thds,gc_gs_table_depth,gc_gsprim_buff_depth,gc_parameter_cache_depth,gc_double_offchip_lds_buffer,gc_wave_size,gc_max_waves_per_simd,gc_max_scratch_slots_per_cu,gc_lds_size,gc_num_sc_per_se,gc_num_sa_per_se,gc_num_packer_per_sc,gc_num_gl2a]);
gc_fields!(gc_info_v1_1, [gc_num_se,gc_num_wgp0_per_sa,gc_num_wgp1_per_sa,gc_num_rb_per_se,gc_num_gl2c,gc_num_gprs,gc_num_max_gs_thds,gc_gs_table_depth,gc_gsprim_buff_depth,gc_parameter_cache_depth,gc_double_offchip_lds_buffer,gc_wave_size,gc_max_waves_per_simd,gc_max_scratch_slots_per_cu,gc_lds_size,gc_num_sc_per_se,gc_num_sa_per_se,gc_num_packer_per_sc,gc_num_gl2a,gc_num_tcp_per_sa,gc_num_sdp_interface,gc_num_tcps]);
gc_fields!(gc_info_v1_2, [gc_num_se,gc_num_wgp0_per_sa,gc_num_wgp1_per_sa,gc_num_rb_per_se,gc_num_gl2c,gc_num_gprs,gc_num_max_gs_thds,gc_gs_table_depth,gc_gsprim_buff_depth,gc_parameter_cache_depth,gc_double_offchip_lds_buffer,gc_wave_size,gc_max_waves_per_simd,gc_max_scratch_slots_per_cu,gc_lds_size,gc_num_sc_per_se,gc_num_sa_per_se,gc_num_packer_per_sc,gc_num_gl2a,gc_num_tcp_per_sa,gc_num_sdp_interface,gc_num_tcps,gc_num_tcp_per_wpg,gc_tcp_l1_size,gc_num_sqc_per_wgp,gc_l1_instruction_cache_size_per_sqc,gc_l1_data_cache_size_per_sqc,gc_gl1c_per_sa,gc_gl1c_size_per_instance,gc_gl2c_per_gpu]);
gc_fields!(gc_info_v1_3, [gc_num_se,gc_num_wgp0_per_sa,gc_num_wgp1_per_sa,gc_num_rb_per_se,gc_num_gl2c,gc_num_gprs,gc_num_max_gs_thds,gc_gs_table_depth,gc_gsprim_buff_depth,gc_parameter_cache_depth,gc_double_offchip_lds_buffer,gc_wave_size,gc_max_waves_per_simd,gc_max_scratch_slots_per_cu,gc_lds_size,gc_num_sc_per_se,gc_num_sa_per_se,gc_num_packer_per_sc,gc_num_gl2a,gc_num_tcp_per_sa,gc_num_sdp_interface,gc_num_tcps,gc_num_tcp_per_wpg,gc_tcp_l1_size,gc_num_sqc_per_wgp,gc_l1_instruction_cache_size_per_sqc,gc_l1_data_cache_size_per_sqc,gc_gl1c_per_sa,gc_gl1c_size_per_instance,gc_gl2c_per_gpu,gc_tcp_size_per_cu,gc_tcp_cache_line_size,gc_instruction_cache_size_per_sqc,gc_instruction_cache_line_size,gc_scalar_data_cache_size_per_sqc,gc_scalar_data_cache_line_size,gc_tcc_size,gc_tcc_cache_line_size]);
gc_fields!(gc_info_v2_0, [gc_num_se,gc_num_cu_per_sh,gc_num_sh_per_se,gc_num_rb_per_se,gc_num_tccs,gc_num_gprs,gc_num_max_gs_thds,gc_gs_table_depth,gc_gsprim_buff_depth,gc_parameter_cache_depth,gc_double_offchip_lds_buffer,gc_wave_size,gc_max_waves_per_simd,gc_max_scratch_slots_per_cu,gc_lds_size,gc_num_sc_per_se,gc_num_packer_per_sc]);
gc_fields!(gc_info_v2_1, [gc_num_se,gc_num_cu_per_sh,gc_num_sh_per_se,gc_num_rb_per_se,gc_num_tccs,gc_num_gprs,gc_num_max_gs_thds,gc_gs_table_depth,gc_gsprim_buff_depth,gc_parameter_cache_depth,gc_double_offchip_lds_buffer,gc_wave_size,gc_max_waves_per_simd,gc_max_scratch_slots_per_cu,gc_lds_size,gc_num_sc_per_se,gc_num_packer_per_sc,gc_num_tcp_per_sh,gc_tcp_size_per_cu,gc_num_sdp_interface,gc_num_cu_per_sqc,gc_instruction_cache_size_per_sqc,gc_scalar_data_cache_size_per_sqc,gc_tcc_size]);

#[repr(C, packed)] pub struct harvest_info_header { pub signature: u32, pub version: u32 }
#[repr(C, packed)] pub struct harvest_info { pub hw_id: u16, pub number_instance: u8, pub reserved: u8 }
#[repr(C, packed)] pub struct harvest_table { pub header: harvest_info_header, pub list: [harvest_info; 32] }
#[repr(C, packed)] pub struct mall_info_header { pub table_id: u32, pub version_major: u16, pub version_minor: u16, pub size_bytes: u32 }
#[repr(C, packed)] pub struct mall_info_v1_0 { pub header: mall_info_header, pub mall_size_per_m: u32, pub m_s_present: u32, pub m_half_use: u32, pub m_mall_config: u32, pub reserved: [u32; 5] }
#[repr(C, packed)] pub struct mall_info_v2_0 { pub header: mall_info_header, pub mall_size_per_umc: u32, pub reserved: [u32; 8] }
pub const VCN_INFO_TABLE_MAX_NUM_INSTANCES: usize = 4;
#[repr(C, packed)] pub struct vcn_info_header { pub table_id: u32, pub version_major: u16, pub version_minor: u16, pub size_bytes: u32 }
#[repr(C, packed)] pub struct vcn_fuse_bits { pub bits: u32 }
#[repr(C, packed)] pub union fuse_data { pub bits: vcn_fuse_bits, pub all_bits: u32 }
#[repr(C, packed)] pub struct vcn_instance_info_v1_0 { pub instance_num: u32, pub fuse_data: fuse_data, pub reserved: [u32; 2] }
#[repr(C, packed)] pub struct vcn_info_v1_0 { pub header: vcn_info_header, pub num_of_instances: u32, pub instance_info: [vcn_instance_info_v1_0; 4], pub reserved: [u32; 4] }
pub const NPS_INFO_TABLE_MAX_NUM_INSTANCES: usize = 12;
#[repr(C, packed)] pub struct nps_info_header { pub table_id: u32, pub version_major: u16, pub version_minor: u16, pub size_bytes: u32 }
#[repr(C, packed)] pub struct nps_instance_info_v1_0 { pub base_address: u64, pub limit_address: u64 }
#[repr(C, packed)] pub struct nps_info_v1_0 { pub header: nps_info_header, pub nps_type: u32, pub count: u32, pub instance_info: [nps_instance_info_v1_0; 12] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
