/* Translated from cvmx-pciercx-defs.h.  The C bitfield layouts are represented
 * by their underlying 32-bit register value; field names and widths are kept
 * in the comments below because Rust has no portable C-style bitfields. */

#[inline(always)] pub const fn CVMX_PCIERCX_CFG001(_block_id: u64) -> u64 { 0x0000_0000_0000_0004 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG006(_block_id: u64) -> u64 { 0x18 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG008(_block_id: u64) -> u64 { 0x20 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG009(_block_id: u64) -> u64 { 0x24 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG010(_block_id: u64) -> u64 { 0x28 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG011(_block_id: u64) -> u64 { 0x2c }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG030(_block_id: u64) -> u64 { 0x78 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG031(_block_id: u64) -> u64 { 0x7c }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG032(_block_id: u64) -> u64 { 0x80 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG034(_block_id: u64) -> u64 { 0x88 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG035(_block_id: u64) -> u64 { 0x8c }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG040(_block_id: u64) -> u64 { 0xa0 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG066(_block_id: u64) -> u64 { 0x108 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG069(_block_id: u64) -> u64 { 0x114 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG070(_block_id: u64) -> u64 { 0x118 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG075(_block_id: u64) -> u64 { 0x12c }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG448(_block_id: u64) -> u64 { 0x700 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG452(_block_id: u64) -> u64 { 0x710 }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG455(_block_id: u64) -> u64 { 0x71c }
#[inline(always)] pub const fn CVMX_PCIERCX_CFG515(_block_id: u64) -> u64 { 0x80c }

macro_rules! pciercx_reg {
    ($u:ident, $s:ident, [$($fields:tt)*]) => {
        #[repr(C)] pub union $u { pub u32: u32, pub s: $s }
        #[repr(C)] #[derive(Copy, Clone)] pub struct $s { pub raw: u32 }
        /* fields: $($fields)* */
    };
}

pciercx_reg!(cvmx_pciercx_cfg001, cvmx_pciercx_cfg001_s,
    [dpe:1 sse:1 rma:1 rta:1 sta:1 devt:2 mdpe:1 fbb:1 reserved_22_22:1 m66:1 cl:1 i_stat:1 reserved_11_18:8 i_dis:1 fbbe:1 see:1 ids_wcc:1 per:1 vps:1 mwice:1 scse:1 me:1 msae:1 isae:1]);
pciercx_reg!(cvmx_pciercx_cfg006, cvmx_pciercx_cfg006_s, [slt:8 subbnum:8 sbnum:8 pbnum:8]);
pciercx_reg!(cvmx_pciercx_cfg008, cvmx_pciercx_cfg008_s, [ml_addr:12 reserved_16_19:4 mb_addr:12 reserved_0_3:4]);
pciercx_reg!(cvmx_pciercx_cfg009, cvmx_pciercx_cfg009_s, [lmem_limit:12 reserved_17_19:3 mem64b:1 lmem_base:12 reserved_1_3:3 mem64a:1]);
pciercx_reg!(cvmx_pciercx_cfg010, cvmx_pciercx_cfg010_s, [umem_base:32]);
pciercx_reg!(cvmx_pciercx_cfg011, cvmx_pciercx_cfg011_s, [umem_limit:32]);
pciercx_reg!(cvmx_pciercx_cfg030, cvmx_pciercx_cfg030_s, [reserved_22_31:10 tp:1 ap_d:1 ur_d:1 fe_d:1 nfe_d:1 ce_d:1 reserved_15_15:1 mrrs:3 ns_en:1 ap_en:1 pf_en:1 etf_en:1 mps:3 ro_en:1 ur_en:1 fe_en:1 nfe_en:1 ce_en:1]);
pciercx_reg!(cvmx_pciercx_cfg031, cvmx_pciercx_cfg031_s, [pnum:8 reserved_23_23:1 aspm:1 lbnc:1 dllarc:1 sderc:1 cpm:1 l1el:3 l0el:3 aslpms:2 mlw:6 mls:4]);
pciercx_reg!(cvmx_pciercx_cfg032, cvmx_pciercx_cfg032_s, [lab:1 lbm:1 dlla:1 scc:1 lt:1 reserved_26_26:1 nlw:6 ls:4 reserved_12_15:4 lab_int_enb:1 lbm_int_enb:1 hawd:1 ecpm:1 es:1 ccc:1 rl:1 ld:1 rcb:1 reserved_2_2:1 aslpc:2]);
pciercx_reg!(cvmx_pciercx_cfg034, cvmx_pciercx_cfg034_s, [reserved_25_31:7 dlls_c:1 emis:1 pds:1 mrlss:1 ccint_d:1 pd_c:1 mrls_c:1 pf_d:1 abp_d:1 reserved_13_15:3 dlls_en:1 emic:1 pcc:1 pic:1 aic:1 hpint_en:1 ccint_en:1 pd_en:1 mrls_en:1 pf_en:1 abp_en:1]);
pciercx_reg!(cvmx_pciercx_cfg035, cvmx_pciercx_cfg035_s, [reserved_17_31:15 crssv:1 reserved_5_15:11 crssve:1 pmeie:1 sefee:1 senfee:1 secee:1]);
pciercx_reg!(cvmx_pciercx_cfg040, cvmx_pciercx_cfg040_s, [reserved_22_31:10 ler:1 ep3s:1 ep2s:1 ep1s:1 eqc:1 cdl:1 cde:4 csos:1 emc:1 tm:3 sde:1 hasd:1 ec:1 tls:4]);
pciercx_reg!(cvmx_pciercx_cfg070, cvmx_pciercx_cfg070_s, [reserved_12_31:20 tplp:1 reserved_9_10:2 ce:1 cc:1 ge:1 gc:1 fep:5]);
pciercx_reg!(cvmx_pciercx_cfg075, cvmx_pciercx_cfg075_s, [reserved_3_31:29 fere:1 nfere:1 cere:1]);
pciercx_reg!(cvmx_pciercx_cfg448, cvmx_pciercx_cfg448_s, [rtl:16 rtltl:16]);
pciercx_reg!(cvmx_pciercx_cfg452, cvmx_pciercx_cfg452_s, [reserved_26_31:6 eccrc:1 reserved_22_24:3 lme:6 reserved_12_15:4 link_rate:4 flm:1 reserved_6_6:1 dllle:1 reserved_4_4:1 ra:1 le:1 sd:1 omr:1]);
pciercx_reg!(cvmx_pciercx_cfg455, cvmx_pciercx_cfg455_s, [m_cfg0_filt:1 m_io_filt:1 msg_ctrl:1 m_cpl_ecrc_filt:1 m_ecrc_filt:1 m_cpl_len_err:1 m_cpl_attr_err:1 m_cpl_tc_err:1 m_cpl_fun_err:1 m_cpl_rid_err:1 m_cpl_tag_err:1 m_lk_filt:1 m_cfg1_filt:1 m_bar_match:1 m_pois_filt:1 m_fun:1 dfcwt:1 reserved_11_14:4 skpiv:11]);
pciercx_reg!(cvmx_pciercx_cfg515, cvmx_pciercx_cfg515_s, [reserved_21_31:11 s_d_e:1 ctcrb:1 cpyts:1 dsc:1 le:9 n_fts:8]);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
