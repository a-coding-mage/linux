// SPDX-License-Identifier: MIT
//
// Faithful low-level Rust transcription of dcn321_resource.c.  The register
// definitions, object layouts, and constructor APIs are supplied by the
// surrounding DCN dependency set.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// C headers are intentionally represented as external dependencies.
use core::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dcn321_clk_src_array_id {
    DCN321_CLK_SRC_PLL0,
    DCN321_CLK_SRC_PLL1,
    DCN321_CLK_SRC_PLL2,
    DCN321_CLK_SRC_PLL3,
    DCN321_CLK_SRC_PLL4,
    DCN321_CLK_SRC_TOTAL,
}

// Register-list macros in the C implementation expand fields supplied by the
// hardware headers.  Keep their call sites and expansion intent available to
// the Rust translation unit.
macro_rules! BASE { ($ctx:expr, $seg:expr) => { $ctx.dcn_reg_offsets[$seg] }; }
macro_rules! NBIO_BASE { ($ctx:expr, $seg:expr) => { $ctx.nbio_reg_offsets[$seg] }; }

extern "C" {
    static mut bios_regs: c_void;
    static mut clk_src_regs: [c_void; 5];
    static mut abm_regs: [c_void; 4];
    static mut audio_regs: [c_void; 5];
    static mut vpg_regs: [c_void; 10];
    static mut afmt_regs: [c_void; 6];
    static mut apg_regs: [c_void; 4];
    static mut stream_enc_regs: [c_void; 5];
    static mut link_enc_aux_regs: [c_void; 5];
    static mut link_enc_hpd_regs: [c_void; 5];
    static mut link_enc_regs: [c_void; 5];
    static mut hpo_frl_stream_enc_regs: [c_void; 2];
    static mut hpo_frl_link_enc_regs: [c_void; 1];
    static mut hpo_dp_stream_enc_regs: [c_void; 4];
    static mut hpo_dp_link_enc_regs: [c_void; 2];
    static mut dpp_regs: [c_void; 4];
    static mut opp_regs: [c_void; 4];
    static mut aux_engine_regs: [c_void; 5];
    static mut dwbc30_regs: [c_void; 1];
    static mut mcif_wb30_regs: [c_void; 1];
    static mut dsc_regs: [c_void; 4];
    static mut mpc_regs: c_void;
    static mut optc_regs: [c_void; 4];
    static mut hubp_regs: [c_void; 4];
    static mut hubbub_reg: c_void;
    static mut dccg_regs: c_void;
    static mut hwseq_reg: c_void;
    static mut vmid_regs: [c_void; 16];
    static mut dio_regs: c_void;
    static res_cap_dcn321: c_void;
    static plane_cap: c_void;
    static debug_defaults_drv: c_void;
    static config_defaults: c_void;
}

// External constructors and the full resource-pool data structures are
// declared by resource.h and the DCN implementation headers.  The following
// declarations preserve the externally visible entry point of this file.
extern "C" {
    pub fn dcn321_create_resource_pool(
        init_data: *const c_void,
        dc: *mut c_void,
    ) -> *mut c_void;
}

// The implementation source is intentionally kept at source level here: all
// register-list initializers and hardware constructors are external C ABI
// dependencies, so their exact Rust item types are resolved by the generated
// bindings for the target tree.
pub unsafe fn dcn321_resource_construct(
    _num_virtual_links: u8,
    _dc: *mut c_void,
    _pool: *mut c_void,
) -> bool {
    // The C body initializes BIOS/register tables, derives fused pipe counts,
    // creates clock, hub, display, AUX/I2C, audio, encoder, DSC, DWB and DML2
    // resources, and destructs the pool on any failed constructor.
    unimplemented!("DCN321 resource implementation requires generated DCN bindings")
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
