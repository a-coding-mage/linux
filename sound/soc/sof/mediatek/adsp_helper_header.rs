/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (c) 2021 MediaTek Corporation. All rights reserved.
 */

// Depends on Linux/firmware MediaTek ADSP IPC declarations from:
// <linux/firmware/mediatek/mtk-adsp-ipc.h>

/*
 * Global important adsp data structure.
 */
#[repr(C)]
pub struct mtk_adsp_chip_info {
    pub pa_sram: phys_addr_t,
    pub pa_dram: phys_addr_t, /* adsp dram physical base */
    pub pa_cfgreg: phys_addr_t,
    pub sramsize: u32,
    pub dramsize: u32,
    pub cfgregsize: u32,
    pub va_sram: *mut core::ffi::c_void, /* corresponding to pa_sram */
    pub va_dram: *mut core::ffi::c_void, /* corresponding to pa_dram */
    pub va_cfgreg: *mut core::ffi::c_void,
    pub adsp_bootup_addr: phys_addr_t,
    pub dram_offset: core::ffi::c_int, /*dram offset between system and dsp view*/

    pub pa_secreg: phys_addr_t,
    pub secregsize: u32,
    pub va_secreg: *mut core::ffi::c_void,

    pub pa_busreg: phys_addr_t,
    pub busregsize: u32,
    pub va_busreg: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct adsp_priv {
    pub dev: *mut device,
    pub sdev: *mut snd_sof_dev,
    pub dsp_ipc: *mut mtk_adsp_ipc,
    pub ipc_dev: *mut platform_device,
    pub adsp: *mut mtk_adsp_chip_info,
    pub clk: *mut *mut clk,
    pub ap2adsp_addr: Option<unsafe extern "C" fn(addr: u32, data: *mut core::ffi::c_void) -> u32>,
    pub adsp2ap_addr: Option<unsafe extern "C" fn(addr: u32, data: *mut core::ffi::c_void) -> u32>,

    pub private_data: *mut core::ffi::c_void,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
