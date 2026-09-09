/* Translated from cvmx-l2c-defs.h. */

/* Dependency: <uapi/asm/bitfield.h>. */

macro_rules! CVMX_L2C_DBG { () => { CVMX_ADD_IO_SEG(0x0001180080000030u64) }; }
macro_rules! CVMX_L2C_CFG { () => { CVMX_ADD_IO_SEG(0x0001180080000000u64) }; }
macro_rules! CVMX_L2C_CTL { () => { CVMX_ADD_IO_SEG(0x0001180080800000u64) }; }
macro_rules! CVMX_L2C_ERR_TDTX { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180080A007E0u64) + (($block_id) & 3) * 0x40000u64 }; }
macro_rules! CVMX_L2C_ERR_TTGX { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180080A007E8u64) + (($block_id) & 3) * 0x40000u64 }; }
macro_rules! CVMX_L2C_LCKBASE { () => { CVMX_ADD_IO_SEG(0x0001180080000058u64) }; }
macro_rules! CVMX_L2C_LCKOFF { () => { CVMX_ADD_IO_SEG(0x0001180080000060u64) }; }
macro_rules! CVMX_L2C_PFCTL { () => { CVMX_ADD_IO_SEG(0x0001180080000090u64) }; }
macro_rules! CVMX_L2C_PFCX { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001180080000098u64) + (($offset) & 3) * 8 }; }
macro_rules! CVMX_L2C_PFC0 { () => { CVMX_L2C_PFCX!(0) }; }
macro_rules! CVMX_L2C_PFC1 { () => { CVMX_L2C_PFCX!(1) }; }
macro_rules! CVMX_L2C_PFC2 { () => { CVMX_L2C_PFCX!(2) }; }
macro_rules! CVMX_L2C_PFC3 { () => { CVMX_L2C_PFCX!(3) }; }
macro_rules! CVMX_L2C_SPAR0 { () => { CVMX_ADD_IO_SEG(0x0001180080000068u64) }; }
macro_rules! CVMX_L2C_SPAR1 { () => { CVMX_ADD_IO_SEG(0x0001180080000070u64) }; }
macro_rules! CVMX_L2C_SPAR2 { () => { CVMX_ADD_IO_SEG(0x0001180080000078u64) }; }
macro_rules! CVMX_L2C_SPAR3 { () => { CVMX_ADD_IO_SEG(0x0001180080000080u64) }; }
macro_rules! CVMX_L2C_SPAR4 { () => { CVMX_ADD_IO_SEG(0x0001180080000088u64) }; }
macro_rules! CVMX_L2C_TADX_PFCX { ($offset:expr, $block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180080A00400u64) + ((($offset) & 3) + (($block_id) & 7) * 0x8000u64) * 8 }; }
macro_rules! CVMX_L2C_TADX_PFC0 { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180080A00400u64) + (($block_id) & 3) * 0x40000u64 }; }
macro_rules! CVMX_L2C_TADX_PFC1 { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180080A00408u64) + (($block_id) & 3) * 0x40000u64 }; }
macro_rules! CVMX_L2C_TADX_PFC2 { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180080A00410u64) + (($block_id) & 3) * 0x40000u64 }; }
macro_rules! CVMX_L2C_TADX_PFC3 { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180080A00418u64) + (($block_id) & 3) * 0x40000u64 }; }
macro_rules! CVMX_L2C_TADX_PRF { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001180080A00008u64) + (($offset) & 7) * 0x40000u64 }; }
macro_rules! CVMX_L2C_TADX_TAG { ($block_id:expr) => { CVMX_ADD_IO_SEG(0x0001180080A00010u64) + (($block_id) & 3) * 0x40000u64 }; }
macro_rules! CVMX_L2C_WPAR_IOBX { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001180080840200u64) + (($offset) & 1) * 8 }; }
macro_rules! CVMX_L2C_WPAR_PPX { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001180080840000u64) + (($offset) & 31) * 8 }; }

macro_rules! l2c_union { ($u:ident, $s:ident, { $($field:ident : $ty:ty),* $(,)? }) => {
    #[repr(C)] pub union $u { pub u64: u64, pub s: $s }
    #[repr(C)] pub struct $s { $(pub $field: $ty),* }
} }

l2c_union!(cvmx_l2c_err_tdtx, cvmx_l2c_err_tdtx_s, { dbe:u64, sbe:u64, vdbe:u64, vsbe:u64, syn:u64, reserved_22_49:u64, wayidx:u64, reserved_2_3:u64, type_:u64 });
l2c_union!(cvmx_l2c_err_ttgx, cvmx_l2c_err_ttgx_s, { dbe:u64, sbe:u64, noway:u64, reserved_56_60:u64, syn:u64, reserved_22_49:u64, wayidx:u64, reserved_2_6:u64, type_:u64 });
l2c_union!(cvmx_l2c_cfg, cvmx_l2c_cfg_s, { reserved_20_63:u64, bstrun:u64, lbist:u64, xor_bank:u64, dpres1:u64, dpres0:u64, dfill_dis:u64, fpexp:u64, fpempty:u64, fpen:u64, idxalias:u64, mwf_crd:u64, rsp_arb_mode:u64, rfb_arb_mode:u64, lrf_arb_mode:u64 });
l2c_union!(cvmx_l2c_ctl, cvmx_l2c_ctl_s, { reserved_30_63:u64, sepcmt:u64, rdf_fast:u64, disstgl2i:u64, l2dfsbe:u64, l2dfdbe:u64, discclk:u64, maxvab:u64, maxlfb:u64, rsp_arb_mode:u64, xmc_arb_mode:u64, ef_ena:u64, ef_cnt:u64, vab_thresh:u64, disecc:u64, disidxalias:u64 });
l2c_union!(cvmx_l2c_dbg, cvmx_l2c_dbg_s, { reserved_15_63:u64, lfb_enum:u64, lfb_dmp:u64, ppnum:u64, set:u64, finv:u64, l2d:u64, l2t:u64 });
l2c_union!(cvmx_l2c_pfctl, cvmx_l2c_pfctl_s, { reserved_36_63:u64, cnt3rdclr:u64, cnt2rdclr:u64, cnt1rdclr:u64, cnt0rdclr:u64, cnt3ena:u64, cnt3clr:u64, cnt3sel:u64, cnt2ena:u64, cnt2clr:u64, cnt2sel:u64, cnt1ena:u64, cnt1clr:u64, cnt1sel:u64, cnt0ena:u64, cnt0clr:u64, cnt0sel:u64 });
l2c_union!(cvmx_l2c_tadx_prf, cvmx_l2c_tadx_prf_s, { reserved_32_63:u64, cnt3sel:u64, cnt2sel:u64, cnt1sel:u64, cnt0sel:u64 });
l2c_union!(cvmx_l2c_tadx_tag, cvmx_l2c_tadx_tag_s, { reserved_46_63:u64, ecc:u64, reserved_36_39:u64, tag:u64, reserved_4_16:u64, use_:u64, valid:u64, dirty:u64, lock:u64 });
l2c_union!(cvmx_l2c_lckbase, cvmx_l2c_lckbase_s, { reserved_31_63:u64, lck_base:u64, reserved_1_3:u64, lck_ena:u64 });
l2c_union!(cvmx_l2c_lckoff, cvmx_l2c_lckoff_s, { reserved_10_63:u64, lck_offset:u64 });

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
