/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

pub const ICP_QAT_AC_895XCC_DEV_TYPE: u32 = 0x00400000;
pub const ICP_QAT_AC_C62X_DEV_TYPE: u32 = 0x01000000;
pub const ICP_QAT_AC_C3XXX_DEV_TYPE: u32 = 0x02000000;
pub const ICP_QAT_AC_4XXX_A_DEV_TYPE: u32 = 0x08000000;
pub const ICP_QAT_AC_6XXX_DEV_TYPE: u32 = 0x80000000;
pub const ICP_QAT_UCLO_MAX_AE: usize = 17;
pub const ICP_QAT_UCLO_MAX_CTX: usize = 8;
pub const ICP_QAT_UCLO_MAX_UIMAGE: usize = ICP_QAT_UCLO_MAX_AE * ICP_QAT_UCLO_MAX_CTX;
pub const ICP_QAT_UCLO_MAX_USTORE: u32 = 0x4000;
pub const ICP_QAT_UCLO_MAX_XFER_REG: u32 = 128;
pub const ICP_QAT_UCLO_MAX_GPR_REG: u32 = 128;
pub const ICP_QAT_UCLO_MAX_LMEM_REG: u32 = 1024;
pub const ICP_QAT_UCLO_MAX_LMEM_REG_2X: u32 = 1280;
pub const ICP_QAT_UCLO_AE_ALL_CTX: u32 = 0xff;
pub const ICP_QAT_UOF_OBJID_LEN: usize = 8;
pub const ICP_QAT_UOF_FID: u32 = 0xc6c2;
pub const ICP_QAT_UOF_MAJVER: u32 = 0x4;
pub const ICP_QAT_UOF_MINVER: u32 = 0x11;
pub const ICP_QAT_UOF_OBJS: &[u8] = b"UOF_OBJS\0";
pub const ICP_QAT_UOF_STRT: &[u8] = b"UOF_STRT\0";
pub const ICP_QAT_UOF_IMAG: &[u8] = b"UOF_IMAG\0";
pub const ICP_QAT_UOF_IMEM: &[u8] = b"UOF_IMEM\0";
pub const ICP_QAT_UOF_LOCAL_SCOPE: u32 = 1;
pub const ICP_QAT_UOF_INIT_EXPR: u32 = 0;
pub const ICP_QAT_UOF_INIT_REG: u32 = 1;
pub const ICP_QAT_UOF_INIT_REG_CTX: u32 = 2;
pub const ICP_QAT_UOF_INIT_EXPR_ENDIAN_SWAP: u32 = 3;
pub const ICP_QAT_SUOF_OBJ_ID_LEN: usize = 8;
pub const ICP_QAT_SUOF_FID: u32 = 0x53554f46;
pub const ICP_QAT_SUOF_MAJVER: u32 = 0;
pub const ICP_QAT_SUOF_MINVER: u32 = 1;
pub const ICP_QAT_SUOF_OBJ_NAME_LEN: usize = 128;
pub const ICP_QAT_MOF_OBJ_ID_LEN: usize = 8;
pub const ICP_QAT_MOF_OBJ_CHUNKID_LEN: usize = 8;
pub const ICP_QAT_MOF_FID: u32 = 0x00666f6d;
pub const ICP_QAT_MOF_MAJVER: u32 = 0;
pub const ICP_QAT_MOF_MINVER: u32 = 1;
pub const ICP_QAT_MOF_SYM_OBJS: &[u8] = b"SYM_OBJS\0";
pub const ICP_QAT_SUOF_OBJS: &[u8] = b"SUF_OBJS\0";
pub const ICP_QAT_SUOF_IMAG: &[u8] = b"SUF_IMAG\0";
pub const ICP_QAT_SIMG_AE_INIT_SEQ_LEN: usize = 50 * core::mem::size_of::<u64>();
pub const DSS_FWSK_MODULUS_LEN: usize = 384;
pub const DSS_FWSK_EXPONENT_LEN: usize = 4;
pub const DSS_FWSK_PADDING_LEN: usize = 380;
pub const DSS_SIGNATURE_LEN: usize = 384;
pub const CSS_FWSK_MODULUS_LEN: usize = 256;
pub const CSS_FWSK_EXPONENT_LEN: usize = 4;
pub const CSS_FWSK_PADDING_LEN: usize = 252;
pub const CSS_SIGNATURE_LEN: usize = 256;
#[macro_export] macro_rules! ICP_QAT_CSS_FWSK_MODULUS_LEN { ($h:expr) => (if unsafe { (*(*$h).chip_info).css_3k } { DSS_FWSK_MODULUS_LEN } else { CSS_FWSK_MODULUS_LEN }) }
#[macro_export] macro_rules! ICP_QAT_CSS_FWSK_EXPONENT_LEN { ($h:expr) => (if unsafe { (*(*$h).chip_info).css_3k } { DSS_FWSK_EXPONENT_LEN } else { CSS_FWSK_EXPONENT_LEN }) }
#[macro_export] macro_rules! ICP_QAT_CSS_FWSK_PAD_LEN { ($h:expr) => (if unsafe { (*(*$h).chip_info).css_3k } { DSS_FWSK_PADDING_LEN } else { CSS_FWSK_PADDING_LEN }) }
#[macro_export] macro_rules! ICP_QAT_CSS_FWSK_PUB_LEN { ($h:expr) => (ICP_QAT_CSS_FWSK_MODULUS_LEN!($h) + ICP_QAT_CSS_FWSK_EXPONENT_LEN!($h) + ICP_QAT_CSS_FWSK_PAD_LEN!($h)) }
#[macro_export] macro_rules! ICP_QAT_CSS_SIGNATURE_LEN { ($h:expr) => (if unsafe { (*(*$h).chip_info).css_3k } { DSS_SIGNATURE_LEN } else { CSS_SIGNATURE_LEN }) }
#[macro_export] macro_rules! ICP_QAT_AE_IMG_OFFSET { ($h:expr) => (core::mem::size_of::<icp_qat_css_hdr>() + ICP_QAT_CSS_FWSK_MODULUS_LEN!($h) + ICP_QAT_CSS_FWSK_EXPONENT_LEN!($h) + ICP_QAT_CSS_SIGNATURE_LEN!($h)) }
pub const ICP_QAT_CSS_RSA4K_MAX_IMAGE_LEN: u32 = 0x40000;
pub const ICP_QAT_CSS_RSA3K_MAX_IMAGE_LEN: u32 = 0x30000;
pub const ICP_QAT_DUALSIGN_OPAQUE_HDR_LEN: usize = 12;
pub const ICP_QAT_DUALSIGN_OPAQUE_HDR_ALIGN_LEN: usize = 16;
pub const ICP_QAT_DUALSIGN_OPAQUE_DATA_LEN: usize = 3540;
pub const ICP_QAT_DUALSIGN_XMSS_PUBKEY_LEN: usize = 64;
pub const ICP_QAT_DUALSIGN_XMSS_SIG_LEN: usize = 2692;
pub const ICP_QAT_DUALSIGN_XMSS_SIG_ALIGN_LEN: usize = 2696;
pub const ICP_QAT_DUALSIGN_MISC_INFO_LEN: usize = 16;
pub const ICP_QAT_DUALSIGN_FW_TYPE_LEN: usize = 7;
pub const ICP_QAT_DUALSIGN_MODULE_TYPE: u32 = 0x14;
pub const ICP_QAT_DUALSIGN_HDR_LEN: u32 = 0x375;
pub const ICP_QAT_DUALSIGN_HDR_VER: u32 = 0x40001;
pub const ICP_QAT_DUALSIGN_HDR_LEN_OFFSET: u32 = 4;
pub const ICP_QAT_DUALSIGN_HDR_VER_OFFSET: u32 = 8;

#[macro_export] macro_rules! ICP_QAT_CTX_MODE { ($x:expr) => (($x) & 0xf) }
#[macro_export] macro_rules! ICP_QAT_NN_MODE { ($x:expr) => ((($x) >> 0x4) & 0xf) }
#[macro_export] macro_rules! ICP_QAT_SHARED_USTORE_MODE { ($x:expr) => ((($x) >> 0xb) & 0x1) }
#[macro_export] macro_rules! RELOADABLE_CTX_SHARED_MODE { ($x:expr) => ((($x) >> 0xc) & 0x1) }
#[macro_export] macro_rules! ICP_QAT_LOC_MEM0_MODE { ($x:expr) => ((($x) >> 0x8) & 0x1) }
#[macro_export] macro_rules! ICP_QAT_LOC_MEM1_MODE { ($x:expr) => ((($x) >> 0x9) & 0x1) }
#[macro_export] macro_rules! ICP_QAT_LOC_MEM2_MODE { ($x:expr) => ((($x) >> 0x6) & 0x1) }
#[macro_export] macro_rules! ICP_QAT_LOC_MEM3_MODE { ($x:expr) => ((($x) >> 0x7) & 0x1) }
#[macro_export] macro_rules! ICP_QAT_LOC_TINDEX_MODE { ($x:expr) => ((($x) >> 0xe) & 0x1) }

#[repr(u32)] pub enum icp_qat_uof_mem_region { ICP_QAT_UOF_SRAM_REGION=0, ICP_QAT_UOF_LMEM_REGION=3, ICP_QAT_UOF_UMEM_REGION=5 }
#[repr(u32)] pub enum icp_qat_uof_regtype { ICP_NO_DEST=0, ICP_GPA_REL=1, ICP_GPA_ABS=2, ICP_GPB_REL=3, ICP_GPB_ABS=4, ICP_SR_REL=5, ICP_SR_RD_REL=6, ICP_SR_WR_REL=7, ICP_SR_ABS=8, ICP_SR_RD_ABS=9, ICP_SR_WR_ABS=10, ICP_DR_REL=19, ICP_DR_RD_REL=20, ICP_DR_WR_REL=21, ICP_DR_ABS=22, ICP_DR_RD_ABS=23, ICP_DR_WR_ABS=24, ICP_LMEM=26, ICP_LMEM0=27, ICP_LMEM1=28, ICP_NEIGH_REL=31, ICP_LMEM2=61, ICP_LMEM3=62 }
#[repr(u32)] pub enum icp_qat_css_fwtype { CSS_AE_FIRMWARE=0, CSS_MMP_FIRMWARE=1 }

#[repr(C)] pub struct icp_qat_uclo_page { pub encap_page:*mut icp_qat_uclo_encap_page, pub region:*mut icp_qat_uclo_region, pub flags:u32 }
#[repr(C)] pub struct icp_qat_uclo_region { pub loaded:*mut icp_qat_uclo_page, pub page:*mut icp_qat_uclo_page }
#[repr(C)] pub struct icp_qat_uclo_aeslice { pub region:*mut icp_qat_uclo_region, pub page:*mut icp_qat_uclo_page, pub cur_page:[*mut icp_qat_uclo_page; ICP_QAT_UCLO_MAX_CTX], pub encap_image:*mut icp_qat_uclo_encapme, pub ctx_mask_assigned:u32, pub new_uaddr:[u32; ICP_QAT_UCLO_MAX_CTX] }
#[repr(C)] pub struct icp_qat_uclo_aedata { pub slice_num:u32, pub eff_ustore_size:u32, pub ae_slices:[icp_qat_uclo_aeslice; ICP_QAT_UCLO_MAX_CTX] }
#[repr(C)] pub struct icp_qat_uof_encap_obj { pub beg_uof:*mut i8, pub obj_hdr:*mut icp_qat_uof_objhdr, pub chunk_hdr:*mut icp_qat_uof_chunkhdr, pub var_mem_seg:*mut icp_qat_uof_varmem_seg }
#[repr(C)] pub struct icp_qat_uclo_encap_uwblock { pub start_addr:u32, pub words_num:u32, pub micro_words:u64 }
#[repr(C)] pub struct icp_qat_uclo_encap_page { pub def_page:u32, pub page_region:u32, pub beg_addr_v:u32, pub beg_addr_p:u32, pub micro_words_num:u32, pub uwblock_num:u32, pub uwblock:*mut icp_qat_uclo_encap_uwblock }
#[repr(C)] pub struct icp_qat_uclo_encapme { pub img_ptr:*mut icp_qat_uof_image, pub page:*mut icp_qat_uclo_encap_page, pub ae_reg_num:u32, pub ae_reg:*mut icp_qat_uof_ae_reg, pub init_regsym_num:u32, pub init_regsym:*mut icp_qat_uof_init_regsym, pub sbreak_num:u32, pub sbreak:*mut icp_qat_uof_sbreak, pub uwords_num:u32 }
#[repr(C)] pub struct icp_qat_uclo_init_mem_table { pub entry_num:u32, pub init_mem:*mut icp_qat_uof_initmem }
#[repr(C)] pub struct icp_qat_uclo_objhdr { pub file_buff:*mut i8, pub checksum:u32, pub size:u32 }
#[repr(C)] pub struct icp_qat_uof_strtable { pub table_len:u32, pub reserved:u32, pub strings:u64 }
#[repr(C)] pub struct icp_qat_uclo_objhandle { pub prod_type:u32, pub prod_rev:u32, pub obj_hdr:*mut icp_qat_uclo_objhdr, pub encap_uof_obj:icp_qat_uof_encap_obj, pub str_table:icp_qat_uof_strtable, pub ae_uimage:[icp_qat_uclo_encapme; ICP_QAT_UCLO_MAX_UIMAGE], pub ae_data:[icp_qat_uclo_aedata; ICP_QAT_UCLO_MAX_AE], pub init_mem_tab:icp_qat_uclo_init_mem_table, pub lm_init_tab:[*mut icp_qat_uof_batch_init; ICP_QAT_UCLO_MAX_AE], pub umem_init_tab:[*mut icp_qat_uof_batch_init; ICP_QAT_UCLO_MAX_AE], pub uimage_num:i32, pub uword_in_bytes:i32, pub global_inited:i32, pub ae_num:u32, pub ustore_phy_size:u32, pub obj_buf:*mut core::ffi::c_void, pub uword_buf:*mut u64 }
#[repr(C)] pub struct icp_qat_uof_uword_block { pub start_addr:u32, pub words_num:u32, pub uword_offset:u32, pub reserved:u32 }
#[repr(C)] pub struct icp_qat_uof_filehdr { pub file_id:u16, pub reserved1:u16, pub min_ver:i8, pub maj_ver:i8, pub reserved2:u16, pub max_chunks:u16, pub num_chunks:u16 }
#[repr(C)] pub struct icp_qat_uof_filechunkhdr { pub chunk_id:[i8; ICP_QAT_UOF_OBJID_LEN], pub checksum:u32, pub offset:u32, pub size:u32 }
#[repr(C)] pub struct icp_qat_uof_objhdr { pub ac_dev_type:u32, pub min_cpu_ver:u16, pub max_cpu_ver:u16, pub max_chunks:i16, pub num_chunks:i16, pub reserved1:u32, pub reserved2:u32 }
#[repr(C)] pub struct icp_qat_uof_chunkhdr { pub chunk_id:[i8; ICP_QAT_UOF_OBJID_LEN], pub offset:u32, pub size:u32 }
#[repr(C)] pub struct icp_qat_uof_memvar_attr { pub offset_in_byte:u32, pub value:u32 }
#[repr(C)] pub struct icp_qat_uof_initmem { pub sym_name:u32, pub region:i8, pub scope:i8, pub reserved1:u16, pub addr:u32, pub num_in_bytes:u32, pub val_attr_num:u32 }
#[repr(C)] pub struct icp_qat_uof_init_regsym { pub sym_name:u32, pub init_type:i8, pub value_type:i8, pub reg_type:i8, pub ctx:u8, pub reg_addr:u32, pub value:u32 }
#[repr(C)] pub struct icp_qat_uof_varmem_seg { pub sram_base:u32,pub sram_size:u32,pub sram_alignment:u32,pub sdram_base:u32,pub sdram_size:u32,pub sdram_alignment:u32,pub sdram1_base:u32,pub sdram1_size:u32,pub sdram1_alignment:u32,pub scratch_base:u32,pub scratch_size:u32,pub scratch_alignment:u32 }
#[repr(C)] pub struct icp_qat_uof_gtid { pub tool_id:[i8; ICP_QAT_UOF_OBJID_LEN], pub tool_ver:i32, pub reserved1:u32, pub reserved2:u32 }
#[repr(C)] pub struct icp_qat_uof_sbreak { pub page_num:u32,pub virt_uaddr:u32,pub sbreak_type:u8,pub reg_type:u8,pub reserved1:u16,pub addr_offset:u32,pub reg_addr:u32 }
#[repr(C)] pub struct icp_qat_uof_code_page { pub page_region:u32,pub page_num:u32,pub def_page:u8,pub reserved2:u8,pub reserved1:u16,pub beg_addr_v:u32,pub beg_addr_p:u32,pub neigh_reg_tab_offset:u32,pub uc_var_tab_offset:u32,pub imp_var_tab_offset:u32,pub imp_expr_tab_offset:u32,pub code_area_offset:u32 }
#[repr(C)] pub struct icp_qat_uof_image { pub img_name:u32,pub ae_assigned:u32,pub ctx_assigned:u32,pub ac_dev_type:u32,pub entry_address:u32,pub fill_pattern:[u32;2],pub reloadable_size:u32,pub sensitivity:u8,pub reserved:u8,pub ae_mode:u16,pub max_ver:u16,pub min_ver:u16,pub image_attrib:u16,pub reserved2:u16,pub page_region_num:u16,pub numpages:u16,pub reg_tab_offset:u32,pub init_reg_sym_tab:u32,pub sbreak_tab:u32,pub app_metadata:u32 }
#[repr(C)] pub struct icp_qat_uof_objtable { pub entry_num:u32 }
#[repr(C)] pub struct icp_qat_uof_ae_reg { pub name:u32,pub vis_name:u32,pub type_:u16,pub addr:u16,pub access_mode:u16,pub visible:u8,pub reserved1:u8,pub ref_count:u16,pub reserved2:u16,pub xo_id:u32 }
#[repr(C)] pub struct icp_qat_uof_code_area { pub micro_words_num:u32,pub uword_block_tab:u32 }
#[repr(C)] pub struct icp_qat_uof_batch_init { pub ae:u32,pub addr:u32,pub value:*mut u32,pub size:u32,pub next:*mut icp_qat_uof_batch_init }
#[repr(C)] pub struct icp_qat_suof_img_hdr { pub simg_buf:*mut i8,pub simg_len:usize,pub css_header:*mut i8,pub css_simg:*mut i8,pub simg_size:usize,pub ae_num:u32,pub ae_mask:u32,pub fw_type:u32,pub simg_name:usize,pub appmeta_data:usize }
#[repr(C)] pub struct icp_qat_suof_img_tbl { pub num_simgs:u32,pub simg_hdr:*mut icp_qat_suof_img_hdr }
#[repr(C)] pub struct icp_qat_suof_handle { pub file_id:u32,pub check_sum:u32,pub min_ver:i8,pub maj_ver:i8,pub fw_type:i8,pub suof_buf:*mut i8,pub suof_size:u32,pub sym_str:*mut i8,pub sym_size:u32,pub img_table:icp_qat_suof_img_tbl }
#[repr(C)] pub struct icp_qat_fw_auth_desc { pub img_len:u32,pub ae_mask:u32,pub css_hdr_high:u32,pub css_hdr_low:u32,pub img_high:u32,pub img_low:u32,pub signature_high:u32,pub signature_low:u32,pub fwsk_pub_high:u32,pub fwsk_pub_low:u32,pub img_ae_mode_data_high:u32,pub img_ae_mode_data_low:u32,pub img_ae_init_data_high:u32,pub img_ae_init_data_low:u32,pub img_ae_insts_high:u32,pub img_ae_insts_low:u32,pub cpp_mask:u32,pub reserved:u32,pub xmss_pubkey_high:u32,pub xmss_pubkey_low:u32,pub xmss_sig_high:u32,pub xmss_sig_low:u32,pub reserved2:[u32;2] }
#[repr(C)] pub struct icp_qat_auth_chunk { pub fw_auth_desc:icp_qat_fw_auth_desc,pub chunk_size:u64,pub chunk_bus_addr:u64 }
#[repr(C)] pub struct icp_qat_css_hdr { pub module_type:u32,pub header_len:u32,pub header_ver:u32,pub module_id:u32,pub module_vendor:u32,pub date:u32,pub size:u32,pub key_size:u32,pub module_size:u32,pub exponent_size:u32,pub fw_type:u32,pub reserved:[u32;21] }
#[repr(C)] pub struct icp_qat_simg_ae_mode { pub file_id:u32,pub maj_ver:u16,pub min_ver:u16,pub dev_type:u32,pub devmax_ver:u16,pub devmin_ver:u16,pub ae_mask:u32,pub ctx_enables:u32,pub fw_type:i8,pub ctx_mode:i8,pub nn_mode:i8,pub lm0_mode:i8,pub lm1_mode:i8,pub scs_mode:i8,pub lm2_mode:i8,pub lm3_mode:i8,pub tindex_mode:i8,pub reserved:[u8;7],pub simg_name:[i8;256],pub appmeta_data:[i8;256] }
#[repr(C)] pub struct icp_qat_suof_filehdr { pub file_id:u32,pub check_sum:u32,pub min_ver:i8,pub maj_ver:i8,pub fw_type:i8,pub reserved:i8,pub max_chunks:u16,pub num_chunks:u16 }
#[repr(C)] pub struct icp_qat_suof_chunk_hdr { pub chunk_id:[i8;ICP_QAT_SUOF_OBJ_ID_LEN],pub offset:u64,pub size:u64 }
#[repr(C)] pub struct icp_qat_suof_strtable { pub tab_length:u32,pub strings:u32 }
#[repr(C)] pub struct icp_qat_suof_objhdr { pub img_length:u32,pub reserved:u32 }
#[repr(C)] pub struct icp_qat_mof_file_hdr { pub file_id:u32,pub checksum:u32,pub min_ver:i8,pub maj_ver:i8,pub reserved:u16,pub max_chunks:u16,pub num_chunks:u16 }
#[repr(C)] pub struct icp_qat_mof_chunkhdr { pub chunk_id:[i8;ICP_QAT_MOF_OBJ_ID_LEN],pub offset:u64,pub size:u64 }
#[repr(C)] pub struct icp_qat_mof_str_table { pub tab_len:u32,pub strings:u32 }
#[repr(C)] pub struct icp_qat_mof_obj_hdr { pub max_chunks:u16,pub num_chunks:u16,pub reserved:u32 }
#[repr(C)] pub struct icp_qat_mof_obj_chunkhdr { pub chunk_id:[i8;ICP_QAT_MOF_OBJ_CHUNKID_LEN],pub offset:u64,pub size:u64,pub name:u32,pub reserved:u32 }
#[repr(C)] pub struct icp_qat_mof_objhdr { pub obj_name:*mut i8,pub obj_buf:*mut i8,pub obj_size:u32 }
#[repr(C)] pub struct icp_qat_mof_table { pub num_objs:u32,pub obj_hdr:*mut icp_qat_mof_objhdr }
#[repr(C)] pub struct icp_qat_mof_handle { pub file_id:u32,pub checksum:u32,pub min_ver:i8,pub maj_ver:i8,pub mof_buf:*mut i8,pub mof_size:u32,pub sym_str:*mut i8,pub sym_size:u32,pub uobjs_hdr:*mut i8,pub sobjs_hdr:*mut i8,pub obj_table:icp_qat_mof_table }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
