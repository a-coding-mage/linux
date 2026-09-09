/*
 * Rust translation of cvmx-pow-defs.h.  C bit-fields are retained as raw
 * 64-bit register values; their field names and widths are documented by the
 * corresponding source declarations and are selected by the hardware ABI.
 */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

// Dependency supplied by the surrounding OCTEON register definitions.
extern "C" {
    fn CVMX_ADD_IO_SEG(address: u64) -> u64;
}

macro_rules! cvmx_reg { ($name:ident, $addr:expr) => {
    #[inline] pub unsafe fn $name() -> u64 { CVMX_ADD_IO_SEG($addr) }
};}
macro_rules! cvmx_reg_indexed { ($name:ident, $addr:expr, $mask:expr) => {
    #[inline] pub unsafe fn $name(offset: u64) -> u64 {
        CVMX_ADD_IO_SEG($addr) + (offset & $mask) * 8
    }
};}

cvmx_reg!(CVMX_POW_BIST_STAT, 0x00016700000003F8u64);
cvmx_reg!(CVMX_POW_DS_PC, 0x0001670000000398u64);
cvmx_reg!(CVMX_POW_ECC_ERR, 0x0001670000000218u64);
cvmx_reg!(CVMX_POW_INT_CTL, 0x0001670000000220u64);
cvmx_reg_indexed!(CVMX_POW_IQ_CNTX, 0x0001670000000340u64, 7);
cvmx_reg!(CVMX_POW_IQ_COM_CNT, 0x0001670000000388u64);
cvmx_reg!(CVMX_POW_IQ_INT, 0x0001670000000238u64);
cvmx_reg!(CVMX_POW_IQ_INT_EN, 0x0001670000000240u64);
cvmx_reg_indexed!(CVMX_POW_IQ_THRX, 0x00016700000003A0u64, 7);
cvmx_reg!(CVMX_POW_NOS_CNT, 0x0001670000000228u64);
cvmx_reg!(CVMX_POW_NW_TIM, 0x0001670000000210u64);
cvmx_reg!(CVMX_POW_PF_RST_MSK, 0x0001670000000230u64);
cvmx_reg_indexed!(CVMX_POW_PP_GRP_MSKX, 0x0001670000000000u64, 15);
cvmx_reg_indexed!(CVMX_POW_QOS_RNDX, 0x00016700000001C0u64, 7);
cvmx_reg_indexed!(CVMX_POW_QOS_THRX, 0x0001670000000180u64, 7);
cvmx_reg!(CVMX_POW_TS_PC, 0x0001670000000390u64);
cvmx_reg!(CVMX_POW_WA_COM_PC, 0x0001670000000380u64);
cvmx_reg_indexed!(CVMX_POW_WA_PCX, 0x0001670000000300u64, 7);
cvmx_reg!(CVMX_POW_WQ_INT, 0x0001670000000200u64);
cvmx_reg_indexed!(CVMX_POW_WQ_INT_CNTX, 0x0001670000000100u64, 15);
cvmx_reg!(CVMX_POW_WQ_INT_PC, 0x0001670000000208u64);
cvmx_reg_indexed!(CVMX_POW_WQ_INT_THRX, 0x0001670000000080u64, 15);
cvmx_reg_indexed!(CVMX_POW_WS_PCX, 0x0001670000000280u64, 15);
cvmx_reg!(CVMX_SSO_WQ_INT, 0x0001670000001000u64);
cvmx_reg!(CVMX_SSO_WQ_IQ_DIS, 0x0001670000001010u64);
cvmx_reg!(CVMX_SSO_WQ_INT_PC, 0x0001670000001020u64);
cvmx_reg_indexed!(CVMX_SSO_PPX_GRP_MSK, 0x0001670000006000u64, 31);
cvmx_reg_indexed!(CVMX_SSO_WQ_INT_THRX, 0x0001670000007000u64, 63);

#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_bits { pub bits: u64 }

macro_rules! cvmx_union { ($u:ident; $($field:ident),+ $(,)?) => {
    #[repr(C)] pub union $u { pub u64: u64, $(pub $field: cvmx_bits,)+ }
};}

// The following variants correspond one-for-one to the C bit-field structs.
cvmx_union!(cvmx_pow_bist_stat; s, cn30xx, cn31xx, cn38xx, cn52xx, cn56xx, cn61xx, cn63xx, cn66xx);
cvmx_union!(cvmx_pow_ds_pc; s);
cvmx_union!(cvmx_pow_ecc_err; s, cn31xx);
cvmx_union!(cvmx_pow_int_ctl; s);
cvmx_union!(cvmx_pow_iq_cntx; s);
cvmx_union!(cvmx_pow_iq_com_cnt; s);
cvmx_union!(cvmx_pow_iq_int; s);
cvmx_union!(cvmx_pow_iq_int_en; s);
cvmx_union!(cvmx_pow_iq_thrx; s);
cvmx_union!(cvmx_pow_nos_cnt; s, cn30xx, cn31xx, cn52xx, cn63xx);
cvmx_union!(cvmx_pow_nw_tim; s);
cvmx_union!(cvmx_pow_pf_rst_msk; s);
cvmx_union!(cvmx_pow_pp_grp_mskx; s, cn30xx);
cvmx_union!(cvmx_pow_qos_rndx; s);
cvmx_union!(cvmx_pow_qos_thrx; s, cn30xx, cn31xx, cn52xx, cn63xx);
cvmx_union!(cvmx_pow_ts_pc; s);
cvmx_union!(cvmx_pow_wa_com_pc; s);
cvmx_union!(cvmx_pow_wa_pcx; s);
cvmx_union!(cvmx_pow_wq_int; s);
cvmx_union!(cvmx_pow_wq_int_cntx; s, cn30xx, cn31xx, cn52xx, cn63xx);
cvmx_union!(cvmx_pow_wq_int_pc; s);
cvmx_union!(cvmx_pow_wq_int_thrx; s, cn30xx, cn31xx, cn52xx, cn63xx);
cvmx_union!(cvmx_pow_ws_pcx; s);
cvmx_union!(cvmx_sso_wq_int_thrx; s);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
