/* Translated from cvmx-uctlx-defs.h. */

/* CVMX_ADD_IO_SEG is supplied by the surrounding platform bindings. */
macro_rules! cvmx_uctlx_reg {
    ($name:ident, $addr:expr) => {
        #[inline]
        pub fn $name(_block_id: u64) -> u64 { CVMX_ADD_IO_SEG!($addr) }
    };
}

cvmx_uctlx_reg!(CVMX_UCTLX_BIST_STATUS, 0x000118006F0000A0u64);
cvmx_uctlx_reg!(CVMX_UCTLX_CLK_RST_CTL, 0x000118006F000000u64);
cvmx_uctlx_reg!(CVMX_UCTLX_EHCI_CTL, 0x000118006F000080u64);
cvmx_uctlx_reg!(CVMX_UCTLX_EHCI_FLA, 0x000118006F0000A8u64);
cvmx_uctlx_reg!(CVMX_UCTLX_ERTO_CTL, 0x000118006F000090u64);
cvmx_uctlx_reg!(CVMX_UCTLX_IF_ENA, 0x000118006F000030u64);
cvmx_uctlx_reg!(CVMX_UCTLX_INT_ENA, 0x000118006F000028u64);
cvmx_uctlx_reg!(CVMX_UCTLX_INT_REG, 0x000118006F000020u64);
cvmx_uctlx_reg!(CVMX_UCTLX_OHCI_CTL, 0x000118006F000088u64);
cvmx_uctlx_reg!(CVMX_UCTLX_ORTO_CTL, 0x000118006F000098u64);
cvmx_uctlx_reg!(CVMX_UCTLX_PPAF_WM, 0x000118006F000038u64);
cvmx_uctlx_reg!(CVMX_UCTLX_UPHY_CTL_STATUS, 0x000118006F000008u64);

#[inline]
pub fn CVMX_UCTLX_UPHY_PORTX_CTL_STATUS(offset: u64, block_id: u64) -> u64 {
    CVMX_ADD_IO_SEG!(0x000118006F000010u64) + (((offset & 1) + ((block_id & 0) * 0)) * 8)
}

/* C bit-fields are represented by their containing register word.  The field
 * order and widths are retained below as documentation; accessors in the
 * platform bindings may decode the corresponding masks. */
macro_rules! cvmx_uctlx_union {
    ($union:ident, $bits:ident, [$($field:ident : $width:expr),* $(,)?]) => {
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct $bits { pub u64: u64 }
        #[repr(C)]
        pub union $union { pub u64: u64, pub s: $bits }
        $(const _: usize = $width;)*
    };
}

cvmx_uctlx_union!(cvmx_uctlx_bist_status, cvmx_uctlx_bist_status_s,
    [reserved_6_63:58, data_bis:1, desc_bis:1, erbm_bis:1, orbm_bis:1, wrbm_bis:1, ppaf_bis:1]);
cvmx_uctlx_union!(cvmx_uctlx_clk_rst_ctl, cvmx_uctlx_clk_rst_ctl_s,
    [reserved_25_63:39, clear_bist:1, start_bist:1, ehci_sm:1, ohci_clkcktrst:1, ohci_sm:1, ohci_susp_lgcy:1, app_start_clk:1, o_clkdiv_rst:1, h_clkdiv_byp:1, h_clkdiv_rst:1, h_clkdiv_en:1, o_clkdiv_en:1, h_div:4, p_refclk_sel:2, p_refclk_div:2, reserved_4_4:1, p_com_on:1, p_por:1, p_prst:1, hrst:1]);
cvmx_uctlx_union!(cvmx_uctlx_ehci_ctl, cvmx_uctlx_ehci_ctl_s,
    [reserved_20_63:44, desc_rbm:1, reg_nb:1, l2c_dc:1, l2c_bc:1, l2c_0pag:1, l2c_stt:1, l2c_buff_emod:2, l2c_desc_emod:2, inv_reg_a2:1, ehci_64b_addr_en:1, l2c_addr_msb:8]);
cvmx_uctlx_union!(cvmx_uctlx_ehci_fla, cvmx_uctlx_ehci_fla_s, [reserved_6_63:58, fla:6]);
cvmx_uctlx_union!(cvmx_uctlx_erto_ctl, cvmx_uctlx_erto_ctl_s, [reserved_32_63:32, to_val:27, reserved_0_4:5]);
cvmx_uctlx_union!(cvmx_uctlx_if_ena, cvmx_uctlx_if_ena_s, [reserved_1_63:63, en:1]);
cvmx_uctlx_union!(cvmx_uctlx_int_ena, cvmx_uctlx_int_ena_s, [reserved_8_63:56, ec_ovf_e:1, oc_ovf_e:1, wb_pop_e:1, wb_psh_f:1, cf_psh_f:1, or_psh_f:1, er_psh_f:1, pp_psh_f:1]);
cvmx_uctlx_union!(cvmx_uctlx_int_reg, cvmx_uctlx_int_reg_s, [reserved_8_63:56, ec_ovf_e:1, oc_ovf_e:1, wb_pop_e:1, wb_psh_f:1, cf_psh_f:1, or_psh_f:1, er_psh_f:1, pp_psh_f:1]);
cvmx_uctlx_union!(cvmx_uctlx_ohci_ctl, cvmx_uctlx_ohci_ctl_s, [reserved_19_63:45, reg_nb:1, l2c_dc:1, l2c_bc:1, l2c_0pag:1, l2c_stt:1, l2c_buff_emod:2, l2c_desc_emod:2, inv_reg_a2:1, reserved_8_8:1, l2c_addr_msb:8]);
cvmx_uctlx_union!(cvmx_uctlx_orto_ctl, cvmx_uctlx_orto_ctl_s, [reserved_32_63:32, to_val:24, reserved_0_7:8]);
cvmx_uctlx_union!(cvmx_uctlx_ppaf_wm, cvmx_uctlx_ppaf_wm_s, [reserved_5_63:59, wm:5]);
cvmx_uctlx_union!(cvmx_uctlx_uphy_ctl_status, cvmx_uctlx_uphy_ctl_status_s, [reserved_10_63:54, bist_done:1, bist_err:1, hsbist:1, fsbist:1, lsbist:1, siddq:1, vtest_en:1, uphy_bist:1, bist_en:1, ate_reset:1]);
cvmx_uctlx_union!(cvmx_uctlx_uphy_portx_ctl_status, cvmx_uctlx_uphy_portx_ctl_status_s, [reserved_43_63:21, tdata_out:4, txbiststuffenh:1, txbiststuffen:1, dmpulldown:1, dppulldown:1, vbusvldext:1, portreset:1, txhsvxtune:2, txvreftune:4, txrisetune:1, txpreemphasistune:1, txfslstune:4, sqrxtune:3, compdistune:3, loop_en:1, tclk:1, tdata_sel:1, taddr_in:4, tdata_in:8]);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
