/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2022 Intel Corporation */
/* Translated from icp_qat_fw_comp.h; dependency symbols are supplied externally. */

#[repr(C)]
pub enum icp_qat_fw_comp_cmd_id { ICP_QAT_FW_COMP_CMD_STATIC = 0, ICP_QAT_FW_COMP_CMD_DYNAMIC = 1, ICP_QAT_FW_COMP_CMD_DECOMPRESS = 2, ICP_QAT_FW_COMP_CMD_ZSTD_COMPRESS = 10, ICP_QAT_FW_COMP_CMD_ZSTD_DECOMPRESS = 11, ICP_QAT_FW_COMP_CMD_DELIMITER }
#[repr(C)]
pub enum icp_qat_fw_comp_20_cmd_id { ICP_QAT_FW_COMP_20_CMD_LZ4_COMPRESS = 3, ICP_QAT_FW_COMP_20_CMD_LZ4_DECOMPRESS = 4, ICP_QAT_FW_COMP_20_CMD_LZ4S_COMPRESS = 5, ICP_QAT_FW_COMP_20_CMD_LZ4S_DECOMPRESS = 6, ICP_QAT_FW_COMP_20_CMD_RESERVED_7 = 7, ICP_QAT_FW_COMP_20_CMD_RESERVED_8 = 8, ICP_QAT_FW_COMP_20_CMD_RESERVED_9 = 9, ICP_QAT_FW_COMP_23_CMD_ZSTD_COMPRESS = 10, ICP_QAT_FW_COMP_23_CMD_ZSTD_DECOMPRESS = 11, ICP_QAT_FW_COMP_20_CMD_DELIMITER }

macro_rules! c { ($n:ident, $v:expr) => { pub const $n: u32 = $v; }; }
c!(ICP_QAT_FW_COMP_STATELESS_SESSION, 0); c!(ICP_QAT_FW_COMP_STATEFUL_SESSION, 1); c!(ICP_QAT_FW_COMP_NOT_AUTO_SELECT_BEST, 0); c!(ICP_QAT_FW_COMP_AUTO_SELECT_BEST, 1); c!(ICP_QAT_FW_COMP_NOT_ENH_AUTO_SELECT_BEST, 0); c!(ICP_QAT_FW_COMP_ENH_AUTO_SELECT_BEST, 1); c!(ICP_QAT_FW_COMP_NOT_DISABLE_TYPE0_ENH_AUTO_SELECT_BEST, 0); c!(ICP_QAT_FW_COMP_DISABLE_TYPE0_ENH_AUTO_SELECT_BEST, 1); c!(ICP_QAT_FW_COMP_DISABLE_SECURE_RAM_USED_AS_INTMD_BUF, 1); c!(ICP_QAT_FW_COMP_ENABLE_SECURE_RAM_USED_AS_INTMD_BUF, 0);
c!(ICP_QAT_FW_COMP_SESSION_TYPE_BITPOS, 2); c!(ICP_QAT_FW_COMP_SESSION_TYPE_MASK, 1); c!(ICP_QAT_FW_COMP_AUTO_SELECT_BEST_BITPOS, 3); c!(ICP_QAT_FW_COMP_AUTO_SELECT_BEST_MASK, 1); c!(ICP_QAT_FW_COMP_ENHANCED_AUTO_SELECT_BEST_BITPOS, 4); c!(ICP_QAT_FW_COMP_ENHANCED_AUTO_SELECT_BEST_MASK, 1); c!(ICP_QAT_FW_COMP_RET_DISABLE_TYPE0_HEADER_DATA_BITPOS, 5); c!(ICP_QAT_FW_COMP_RET_DISABLE_TYPE0_HEADER_DATA_MASK, 1); c!(ICP_QAT_FW_COMP_DISABLE_SECURE_RAM_AS_INTMD_BUF_BITPOS, 7); c!(ICP_QAT_FW_COMP_DISABLE_SECURE_RAM_AS_INTMD_BUF_MASK, 1); c!(ICP_QAT_FW_COMP_AUTO_SELECT_BEST_MAX_VALUE, 0xffff_ffff);

#[inline] pub const fn ICP_QAT_FW_COMP_FLAGS_BUILD(sesstype:u32, autoselect:u32, enhanced_asb:u32, ret_uncomp:u32, secure_ram:u32)->u32 { ((sesstype&1)<<2)|((autoselect&1)<<3)|((enhanced_asb&1)<<4)|((ret_uncomp&1)<<5)|((secure_ram&1)<<7) }
#[inline] pub const fn ICP_QAT_FW_COMP_SESSION_TYPE_GET(flags:u32)->u32 {(flags>>2)&1} #[inline] pub const fn ICP_QAT_FW_COMP_SESSION_TYPE_SET(flags:u32,val:u32)->u32 {(flags&!(1<<2))|((val&1)<<2)}
#[inline] pub const fn ICP_QAT_FW_COMP_AUTO_SELECT_BEST_GET(flags:u32)->u32 {(flags>>3)&1} #[inline] pub const fn ICP_QAT_FW_COMP_EN_ASB_GET(flags:u32)->u32 {(flags>>4)&1} #[inline] pub const fn ICP_QAT_FW_COMP_RET_UNCOMP_GET(flags:u32)->u32 {(flags>>5)&1} #[inline] pub const fn ICP_QAT_FW_COMP_SECURE_RAM_USE_GET(flags:u32)->u32 {(flags>>7)&1}

#[repr(C)] pub union icp_qat_fw_comp_req_hdr_cd_pars_u_s { pub content_desc_addr:u64, pub content_desc_resrvd1:u16, pub content_desc_params_sz:u8, pub content_desc_hdr_resrvd2:u8, pub content_desc_resrvd3:u32 }
#[repr(C)] pub struct icp_qat_fw_comp_req_hdr_cd_pars_s { pub content_desc_addr:u64, pub content_desc_resrvd1:u16, pub content_desc_params_sz:u8, pub content_desc_hdr_resrvd2:u8, pub content_desc_resrvd3:u32 }
#[repr(C)] pub struct icp_qat_fw_comp_req_hdr_cd_pars_sl { pub comp_slice_cfg_word:[u32; ICP_QAT_FW_NUM_LONGWORDS_2 as usize], pub content_desc_resrvd4:u32 }
#[repr(C)] pub union icp_qat_fw_comp_req_hdr_cd_pars_union { pub s: icp_qat_fw_comp_req_hdr_cd_pars_s, pub sl: icp_qat_fw_comp_req_hdr_cd_pars_sl }
#[repr(C)] pub struct icp_qat_fw_comp_req_hdr_cd_pars { pub u: icp_qat_fw_comp_req_hdr_cd_pars_union }

#[repr(C)] pub union icp_qat_fw_comp_req_params_crc { pub legacy: icp_qat_fw_comp_req_params_crc_legacy, pub crc_data_addr:u64 }
#[repr(C)] pub struct icp_qat_fw_comp_req_params_crc_legacy { pub initial_crc32:u32, pub initial_adler:u32 }
#[repr(C)] pub struct icp_qat_fw_comp_req_params { pub comp_len:u32, pub out_buffer_sz:u32, pub crc:icp_qat_fw_comp_req_params_crc, pub req_par_flags:u32, pub rsrvd:u32 }
#[inline] pub const fn ICP_QAT_FW_COMP_REQ_PARAM_FLAGS_BUILD(sop:u32,eop:u32,bfinal:u32,cnv:u32,cnvnr:u32,cnvdfx:u32,crc:u32,xxhash_acc:u32,cnv_error_type:u32,append_crc:u32,drop_data:u32,partial_decomp:u32)->u32 { (sop&1)|((eop&1)<<1)|((bfinal&1)<<6)|((cnv&1)<<16)|((cnvnr&1)<<17)|((cnvdfx&1)<<18)|((crc&1)<<19)|((xxhash_acc&1)<<20)|((cnv_error_type&7)<<21)|((append_crc&1)<<24)|((drop_data&1)<<25)|((partial_decomp&1)<<27) }

macro_rules! bit_consts { ($($n:ident=$v:expr),* $(,)?) => { $(pub const $n:u32=$v;)* }; }
bit_consts! { ICP_QAT_FW_COMP_NOT_SOP=0, ICP_QAT_FW_COMP_SOP=1, ICP_QAT_FW_COMP_NOT_EOP=0, ICP_QAT_FW_COMP_EOP=1, ICP_QAT_FW_COMP_NOT_BFINAL=0, ICP_QAT_FW_COMP_BFINAL=1, ICP_QAT_FW_COMP_NO_CNV=0, ICP_QAT_FW_COMP_CNV=1, ICP_QAT_FW_COMP_NO_CNV_RECOVERY=0, ICP_QAT_FW_COMP_CNV_RECOVERY=1, ICP_QAT_FW_COMP_NO_CNV_DFX=0, ICP_QAT_FW_COMP_CNV_DFX=1, ICP_QAT_FW_COMP_CRC_MODE_LEGACY=0, ICP_QAT_FW_COMP_CRC_MODE_E2E=1, ICP_QAT_FW_COMP_NO_XXHASH_ACC=0, ICP_QAT_FW_COMP_XXHASH_ACC=1, ICP_QAT_FW_COMP_APPEND_CRC=1, ICP_QAT_FW_COMP_NO_APPEND_CRC=0, ICP_QAT_FW_COMP_DROP_DATA=1, ICP_QAT_FW_COMP_NO_DROP_DATA=0, ICP_QAT_FW_COMP_PARTIAL_DECOMPRESS=1, ICP_QAT_FW_COMP_NO_PARTIAL_DECOMPRESS=0 }
bit_consts! { ICP_QAT_FW_COMP_SOP_BITPOS=0, ICP_QAT_FW_COMP_SOP_MASK=1, ICP_QAT_FW_COMP_EOP_BITPOS=1, ICP_QAT_FW_COMP_EOP_MASK=1, ICP_QAT_FW_COMP_BFINAL_BITPOS=6, ICP_QAT_FW_COMP_BFINAL_MASK=1, ICP_QAT_FW_COMP_CNV_BITPOS=16, ICP_QAT_FW_COMP_CNV_MASK=1, ICP_QAT_FW_COMP_CNVNR_BITPOS=17, ICP_QAT_FW_COMP_CNVNR_MASK=1, ICP_QAT_FW_COMP_CNV_DFX_BITPOS=18, ICP_QAT_FW_COMP_CNV_DFX_MASK=1, ICP_QAT_FW_COMP_CRC_MODE_BITPOS=19, ICP_QAT_FW_COMP_CRC_MODE_MASK=1, ICP_QAT_FW_COMP_XXHASH_ACC_MODE_BITPOS=20, ICP_QAT_FW_COMP_XXHASH_ACC_MODE_MASK=1, ICP_QAT_FW_COMP_CNV_ERROR_BITPOS=21, ICP_QAT_FW_COMP_CNV_ERROR_MASK=7, ICP_QAT_FW_COMP_CNV_ERROR_NONE=0, ICP_QAT_FW_COMP_CNV_ERROR_CHECKSUM=1, ICP_QAT_FW_COMP_CNV_ERROR_DCPR_OBC_DIFF=2, ICP_QAT_FW_COMP_CNV_ERROR_DCPR=3, ICP_QAT_FW_COMP_CNV_ERROR_XLT=4, ICP_QAT_FW_COMP_CNV_ERROR_DCPR_IBC_DIFF=5, ICP_QAT_FW_COMP_APPEND_CRC_BITPOS=24, ICP_QAT_FW_COMP_APPEND_CRC_MASK=1, ICP_QAT_FW_COMP_DROP_DATA_BITPOS=25, ICP_QAT_FW_COMP_DROP_DATA_MASK=1, ICP_QAT_FW_COMP_PARTIAL_DECOMP_BITPOS=27, ICP_QAT_FW_COMP_PARTIAL_DECOMP_MASK=1 }

macro_rules! get_set { ($g:ident,$s:ident,$p:expr,$m:expr) => { #[inline] pub const fn $g(f:u32)->u32 {(f>>$p)&$m} #[inline] pub const fn $s(f:u32,v:u32)->u32 {(f&!($m<<$p))|((v&$m)<<$p)} }; }
get_set!(ICP_QAT_FW_COMP_SOP_GET,ICP_QAT_FW_COMP_SOP_SET,0,1); get_set!(ICP_QAT_FW_COMP_EOP_GET,ICP_QAT_FW_COMP_EOP_SET,1,1); get_set!(ICP_QAT_FW_COMP_BFINAL_GET,ICP_QAT_FW_COMP_BFINAL_SET,6,1); #[inline] pub const fn ICP_QAT_FW_COMP_CNV_GET(f:u32)->u32 {(f>>16)&1}
#[inline] pub const fn ICP_QAT_FW_COMP_CNVNR_GET(f:u32)->u32 {(f>>17)&1} #[inline] pub const fn ICP_QAT_FW_COMP_CNV_DFX_GET(f:u32)->u32 {(f>>18)&1} #[inline] pub const fn ICP_QAT_FW_COMP_CRC_MODE_GET(f:u32)->u32 {(f>>19)&1} get_set!(ICP_QAT_FW_COMP_XXHASH_ACC_MODE_GET,ICP_QAT_FW_COMP_XXHASH_ACC_MODE_SET,20,1); get_set!(ICP_QAT_FW_COMP_CNV_ERROR_TYPE_GET,ICP_QAT_FW_COMP_CNV_ERROR_TYPE_SET,21,7);
#[inline] pub const fn ICP_QAT_FW_COMP_CNV_DFX_SET(f:u32,v:u32)->u32 {(f&!(1<<18))|((v&1)<<18)}

#[repr(C)] pub struct icp_qat_fw_xlt_req_params { pub inter_buff_ptr:u64 }
#[repr(C)] pub struct icp_qat_fw_comp_cd_hdr { pub ram_bank_flags:u16,pub comp_cfg_offset:u8,pub next_curr_id:u8,pub resrvd:u32,pub comp_state_addr:u64,pub ram_banks_addr:u64 }
pub const COMP_CPR_INITIAL_CRC:u32=0; pub const COMP_CPR_INITIAL_ADLER:u32=1;
#[repr(C)] pub struct icp_qat_fw_xlt_cd_hdr { pub resrvd1:u16,pub resrvd2:u8,pub next_curr_id:u8,pub resrvd3:u32 }

#[repr(C)] pub union icp_qat_fw_comp_req_u1 { pub xlt_pars:icp_qat_fw_xlt_req_params,pub resrvd1:[u32;ICP_QAT_FW_NUM_LONGWORDS_2 as usize],pub partial_decompress:icp_qat_fw_comp_req_partial_decompress }
#[repr(C)] pub struct icp_qat_fw_comp_req_partial_decompress { pub partial_decompress_length:u32,pub partial_decompress_offset:u32 }
#[repr(C)] pub union icp_qat_fw_comp_req_u3 { pub resrvd2:[u32;ICP_QAT_FW_NUM_LONGWORDS_2 as usize],pub asb_threshold:icp_qat_fw_comp_req_asb_threshold }
#[repr(C)] pub struct icp_qat_fw_comp_req_asb_threshold { pub asb_value:u32,pub reserved:u32 }
#[repr(C)] pub union icp_qat_fw_comp_req_u2 { pub xlt_cd_ctrl:icp_qat_fw_xlt_cd_hdr,pub resrvd3:[u32;ICP_QAT_FW_NUM_LONGWORDS_2 as usize] }
#[repr(C)] pub struct icp_qat_fw_comp_req { pub comn_hdr:icp_qat_fw_comn_req_hdr,pub cd_pars:icp_qat_fw_comp_req_hdr_cd_pars,pub comn_mid:icp_qat_fw_comn_req_mid,pub comp_pars:icp_qat_fw_comp_req_params,pub u1:icp_qat_fw_comp_req_u1,pub u3:icp_qat_fw_comp_req_u3,pub comp_cd_ctrl:icp_qat_fw_comp_cd_hdr,pub u2:icp_qat_fw_comp_req_u2 }

/* The remaining request/response structures retain their C layout and refer to external common QAT types. */
#[repr(C)] pub struct icp_qat_fw_comp_state { pub rd8_counter:u32,pub status_flags:u32,pub in_counter:u32,pub out_counter:u32,pub intermediate_state:u64,pub lobc:u32,pub replaybc:u32,pub pcrc64_poly:u64,pub crc32:u32,pub adler_xxhash32:u32,pub pcrc64_xorout:u64,pub out_buf_size:u32,pub in_buf_size:u32,pub in_pcrc64:u64,pub out_pcrc64:u64,pub lobs:u32,pub libc:u32,pub reserved:u64,pub xxhash_state:[u32;4],pub cleartext:[u32;4] }
#[repr(C)] pub struct icp_qat_fw_resp_comp_pars { pub input_byte_counter:u32,pub output_byte_counter:u32,pub crc:icp_qat_fw_resp_comp_pars_crc }
#[repr(C)] pub union icp_qat_fw_resp_comp_pars_crc { pub legacy:icp_qat_fw_resp_comp_pars_legacy,pub resrvd:[u32;ICP_QAT_FW_NUM_LONGWORDS_2 as usize] }
#[repr(C)] pub struct icp_qat_fw_resp_comp_pars_legacy { pub curr_crc32:u32,pub curr_adler_32:u32 }
#[repr(C)] pub struct icp_qat_fw_comp_resp { pub comn_resp:icp_qat_fw_comn_resp_hdr,pub opaque_data:u64,pub comp_resp_pars:icp_qat_fw_resp_comp_pars }

bit_consts! { QAT_FW_COMP_BANK_FLAG_MASK=1, QAT_FW_COMP_BANK_I_BITPOS=8, QAT_FW_COMP_BANK_H_BITPOS=7, QAT_FW_COMP_BANK_G_BITPOS=6, QAT_FW_COMP_BANK_F_BITPOS=5, QAT_FW_COMP_BANK_E_BITPOS=4, QAT_FW_COMP_BANK_D_BITPOS=3, QAT_FW_COMP_BANK_C_BITPOS=2, QAT_FW_COMP_BANK_B_BITPOS=1, QAT_FW_COMP_BANK_A_BITPOS=0 }
#[repr(C)] pub enum icp_qat_fw_comp_bank_enabled { ICP_QAT_FW_COMP_BANK_DISABLED=0, ICP_QAT_FW_COMP_BANK_ENABLED=1, ICP_QAT_FW_COMP_BANK_DELIMITER=2 }
#[inline] pub const fn ICP_QAT_FW_COMP_RAM_FLAGS_BUILD(i:u32,h:u32,g:u32,f:u32,e:u32,d:u32,c_:u32,b:u32,a:u32)->u32 {(i&1)<<8|(h&1)<<7|(g&1)<<6|(f&1)<<5|(e&1)<<4|(d&1)<<3|(c_&1)<<2|(b&1)<<1|(a&1)}
#[repr(C)] pub union icp_qat_fw_comp_crc_data_struct_adler_xxhash_u { pub adler:u32,pub xxhash:u32 }
#[repr(C)] pub struct icp_qat_fw_comp_crc_data_struct { pub crc32:u32,pub adler_xxhash_u:icp_qat_fw_comp_crc_data_struct_adler_xxhash_u,pub cpr_in_crc_lo:u32,pub cpr_in_crc_hi:u32,pub cpr_out_crc_lo:u32,pub cpr_out_crc_hi:u32,pub xlt_in_crc_lo:u32,pub xlt_in_crc_hi:u32,pub xlt_out_crc_lo:u32,pub xlt_out_crc_hi:u32,pub prog_crc_poly_lo:u32,pub prog_crc_poly_hi:u32,pub xor_out_lo:u32,pub xor_out_hi:u32,pub append_crc_lo:u32,pub append_crc_hi:u32 }
#[repr(C)] pub struct xxhash_acc_state_buff { pub in_counter:u32,pub out_counter:u32,pub xxhash_state:[u32;4],pub clear_txt:[u32;4] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
