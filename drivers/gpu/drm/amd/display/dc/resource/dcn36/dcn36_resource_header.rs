/* SPDX-License-Identifier: MIT */
/* Copyright 2025 Advanced Micro Devices, Inc. */

// C dependency: #include "core_types.h"

extern "C" {
    pub static mut dcn3_6_ip: _vcs_dpi_ip_params_st;
    pub static mut dcn3_6_soc: _vcs_dpi_soc_bounding_box_st;
}

// Equivalent of:
// #define TO_DCN36_RES_POOL(pool) container_of(pool, struct dcn36_resource_pool, base)
#[macro_export]
macro_rules! TO_DCN36_RES_POOL {
    ($pool:expr) => {
        container_of!($pool, dcn36_resource_pool, base)
    };
}

#[repr(C)]
pub struct dcn36_resource_pool {
    pub base: resource_pool,
}

extern "C" {
    pub fn dcn36_create_resource_pool(
        init_data: *const dc_init_data,
        dc: *mut dc,
    ) -> *mut resource_pool;
}

// Hardware sequencer register list. SR, SRII, OTG, and the register symbols
// are supplied by the translated register definitions and build environment.
#[macro_export]
macro_rules! HWSEQ_DCN36_REG_LIST {
    () => {
        SR!(DCHUBBUB_GLOBAL_TIMER_CNTL),
        SR!(DCHUBBUB_ARB_HOSTVM_CNTL),
        SR!(DIO_MEM_PWR_CTRL),
        SR!(ODM_MEM_PWR_CTRL3),
        SR!(MMHUBBUB_MEM_PWR_CNTL),
        SR!(DCCG_GATE_DISABLE_CNTL),
        SR!(DCCG_GATE_DISABLE_CNTL2),
        SR!(DCCG_GATE_DISABLE_CNTL4),
        SR!(DCCG_GATE_DISABLE_CNTL5),
        SR!(DCFCLK_CNTL),
        SR!(DC_MEM_GLOBAL_PWR_REQ_CNTL),
        SRII!(PIXEL_RATE_CNTL, OTG, 0),
        SRII!(PIXEL_RATE_CNTL, OTG, 1),
        SRII!(PIXEL_RATE_CNTL, OTG, 2),
        SRII!(PIXEL_RATE_CNTL, OTG, 3),
        SRII!(PHYPLL_PIXEL_RATE_CNTL, OTG, 0),
        SRII!(PHYPLL_PIXEL_RATE_CNTL, OTG, 1),
        SRII!(PHYPLL_PIXEL_RATE_CNTL, OTG, 2),
        SRII!(PHYPLL_PIXEL_RATE_CNTL, OTG, 3),
        SR!(MICROSECOND_TIME_BASE_DIV),
        SR!(MILLISECOND_TIME_BASE_DIV),
        SR!(DISPCLK_FREQ_CHANGE_CNTL),
        SR!(RBBMIF_TIMEOUT_DIS),
        SR!(RBBMIF_TIMEOUT_DIS_2),
        SR!(DCHUBBUB_CRC_CTRL),
        SR!(DPP_TOP0_DPP_CRC_CTRL),
        SR!(MPC_CRC_CTRL),
        SR!(DOMAIN0_PG_CONFIG),
        SR!(DOMAIN1_PG_CONFIG),
        SR!(DOMAIN2_PG_CONFIG),
        SR!(DOMAIN3_PG_CONFIG),
        SR!(DOMAIN16_PG_CONFIG),
        SR!(DOMAIN17_PG_CONFIG),
        SR!(DOMAIN18_PG_CONFIG),
        SR!(DOMAIN19_PG_CONFIG),
        SR!(DOMAIN0_PG_STATUS),
        SR!(DOMAIN1_PG_STATUS),
        SR!(DOMAIN2_PG_STATUS),
        SR!(DOMAIN3_PG_STATUS),
        SR!(DOMAIN16_PG_STATUS),
        SR!(DOMAIN17_PG_STATUS),
        SR!(DOMAIN18_PG_STATUS),
        SR!(DOMAIN19_PG_STATUS),
        SR!(DC_IP_REQUEST_CNTL),
        SR!(AZALIA_AUDIO_DTO),
        SR!(AZALIA_CONTROLLER_CLOCK_GATING),
        SR!(HPO_TOP_HW_CONTROL),
        SR!(DMU_CLK_CNTL)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
