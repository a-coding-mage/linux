/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * MTK SDG1 ECC controller
 *
 * Copyright (c) 2016 Mediatek
 * Authors:    Xiaolei Li        <xiaolei.li@mediatek.com>
 *             Jorge Ramirez-Ortiz       <jorge.ramirez-ortiz@linaro.org>
 */

// C header guard omitted.
// Dependency: linux/types.h

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mtk_ecc_mode {
    ECC_DMA_MODE = 0,
    ECC_NFI_MODE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mtk_ecc_operation {
    ECC_ENCODE = 0,
    ECC_DECODE = 1,
}

pub enum device_node {}
pub enum mtk_ecc {}

#[repr(C)]
pub struct mtk_ecc_stats {
    pub corrected: u32,
    pub bitflips: u32,
    pub failed: u32,
}

#[repr(C)]
pub struct mtk_ecc_config {
    pub op: mtk_ecc_operation,
    pub mode: mtk_ecc_mode,
    pub addr: dma_addr_t,
    pub strength: u32,
    pub sectors: u32,
    pub len: u32,
}

extern "C" {
    pub fn mtk_ecc_encode(
        ecc: *mut mtk_ecc,
        config: *mut mtk_ecc_config,
        data: *mut u8,
        len: u32,
    ) -> i32;
    pub fn mtk_ecc_get_stats(
        ecc: *mut mtk_ecc,
        stats: *mut mtk_ecc_stats,
        n: i32,
    );
    pub fn mtk_ecc_wait_done(ecc: *mut mtk_ecc, op: mtk_ecc_operation) -> i32;
    pub fn mtk_ecc_enable(ecc: *mut mtk_ecc, config: *mut mtk_ecc_config) -> i32;
    pub fn mtk_ecc_disable(ecc: *mut mtk_ecc);
    pub fn mtk_ecc_adjust_strength(ecc: *mut mtk_ecc, p: *mut u32);
    pub fn mtk_ecc_get_parity_bits(ecc: *mut mtk_ecc) -> u32;

    pub fn of_mtk_ecc_get(node: *mut device_node) -> *mut mtk_ecc;
    pub fn mtk_ecc_release(ecc: *mut mtk_ecc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
