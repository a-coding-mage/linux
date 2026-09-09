/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * PRU-ICSS sub-system specific definitions
 *
 * Copyright (C) 2014-2020 Texas Instruments Incorporated - http://www.ti.com/
 *	Suman Anna <s-anna@ti.com>
 */

/* Linux header dependencies are supplied externally. */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pruss_gp_mux_sel {
    PRUSS_GP_MUX_SEL_GP,
    PRUSS_GP_MUX_SEL_ENDAT,
    PRUSS_GP_MUX_SEL_RESERVED,
    PRUSS_GP_MUX_SEL_SD,
    PRUSS_GP_MUX_SEL_MII2,
    PRUSS_GP_MUX_SEL_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pruss_gpi_mode {
    PRUSS_GPI_MODE_DIRECT,
    PRUSS_GPI_MODE_PARALLEL,
    PRUSS_GPI_MODE_28BIT_SHIFT,
    PRUSS_GPI_MODE_MII,
    PRUSS_GPI_MODE_MAX,
}

/**
 * enum pru_type - PRU core type identifier
 *
 * @PRU_TYPE_PRU: Programmable Real-time Unit
 * @PRU_TYPE_RTU: Auxiliary Programmable Real-Time Unit
 * @PRU_TYPE_TX_PRU: Transmit Programmable Real-Time Unit
 * @PRU_TYPE_MAX: just keep this one at the end
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pru_type {
    PRU_TYPE_PRU,
    PRU_TYPE_RTU,
    PRU_TYPE_TX_PRU,
    PRU_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pruss_mem {
    PRUSS_MEM_DRAM0 = 0,
    PRUSS_MEM_DRAM1,
    PRUSS_MEM_SHRD_RAM2,
    PRUSS_MEM_MAX,
}

/**
 * struct pruss_mem_region - PRUSS memory region structure
 * @va: kernel virtual address of the PRUSS memory region
 * @pa: physical (bus) address of the PRUSS memory region
 * @size: size of the PRUSS memory region
 */
#[repr(C)]
pub struct pruss_mem_region {
    pub va: *mut core::ffi::c_void,
    pub pa: phys_addr_t,
    pub size: usize,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rproc {
    _private: [u8; 0],
}

/**
 * struct pruss - PRUSS parent structure
 * @dev: pruss device pointer
 * @cfg_base: base iomap for CFG region
 * @cfg_regmap: regmap for config region
 * @mem_regions: data for each of the PRUSS memory regions
 * @mem_in_use: to indicate if memory resource is in use
 * @lock: mutex to serialize access to resources
 * @core_clk_mux: clk handle for PRUSS CORE_CLK_MUX
 * @iep_clk_mux: clk handle for PRUSS IEP_CLK_MUX
 */
#[repr(C)]
pub struct pruss {
    pub dev: *mut device,
    pub cfg_base: *mut core::ffi::c_void,
    pub cfg_regmap: *mut regmap,
    pub mem_regions: [pruss_mem_region; PRUSS_MEM_MAX as usize],
    pub mem_in_use: [*mut pruss_mem_region; PRUSS_MEM_MAX as usize],
    pub lock: mutex,
    pub core_clk_mux: *mut clk,
    pub iep_clk_mux: *mut clk,
}

/* CONFIG_TI_PRUSS declarations and fallback inline definitions. */
#[cfg(feature = "CONFIG_TI_PRUSS")]
extern "C" {
    pub fn pruss_get(rproc: *mut rproc) -> *mut pruss;
    pub fn pruss_put(pruss: *mut pruss);
    pub fn pruss_request_mem_region(pruss: *mut pruss, mem_id: pruss_mem,
                                    region: *mut pruss_mem_region) -> i32;
    pub fn pruss_release_mem_region(pruss: *mut pruss,
                                    region: *mut pruss_mem_region) -> i32;
    pub fn pruss_cfg_get_gpmux(pruss: *mut pruss, pru_id: enum_pruss_pru_id,
                               mux: *mut u8) -> i32;
    pub fn pruss_cfg_set_gpmux(pruss: *mut pruss, pru_id: enum_pruss_pru_id,
                               mux: u8) -> i32;
    pub fn pruss_cfg_gpimode(pruss: *mut pruss, pru_id: enum_pruss_pru_id,
                             mode: pruss_gpi_mode) -> i32;
    pub fn pruss_cfg_miirt_enable(pruss: *mut pruss, enable: bool) -> i32;
    pub fn pruss_cfg_xfr_enable(pruss: *mut pruss, pru_type: pru_type,
                                enable: bool) -> i32;
}

#[cfg(not(feature = "CONFIG_TI_PRUSS"))]
pub unsafe fn pruss_get(_rproc: *mut rproc) -> *mut pruss {
    ERR_PTR(-EOPNOTSUPP)
}

#[cfg(not(feature = "CONFIG_TI_PRUSS"))]
pub unsafe fn pruss_put(_pruss: *mut pruss) {}

#[cfg(not(feature = "CONFIG_TI_PRUSS"))]
pub unsafe fn pruss_request_mem_region(_pruss: *mut pruss, _mem_id: pruss_mem,
                                       _region: *mut pruss_mem_region) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_TI_PRUSS"))]
pub unsafe fn pruss_release_mem_region(_pruss: *mut pruss,
                                       _region: *mut pruss_mem_region) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_TI_PRUSS"))]
pub unsafe fn pruss_cfg_get_gpmux(_pruss: *mut pruss, _pru_id: enum_pruss_pru_id,
                                  _mux: *mut u8) -> i32 { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_TI_PRUSS"))]
pub unsafe fn pruss_cfg_set_gpmux(_pruss: *mut pruss, _pru_id: enum_pruss_pru_id,
                                  _mux: u8) -> i32 { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_TI_PRUSS"))]
pub unsafe fn pruss_cfg_gpimode(_pruss: *mut pruss, _pru_id: enum_pruss_pru_id,
                                _mode: pruss_gpi_mode) -> i32 { -EOPNOTSUPP }

#[cfg(not(feature = "CONFIG_TI_PRUSS"))]
pub unsafe fn pruss_cfg_miirt_enable(_pruss: *mut pruss, _enable: bool) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_TI_PRUSS"))]
pub unsafe fn pruss_cfg_xfr_enable(_pruss: *mut pruss, _pru_type: pru_type,
                                   _enable: bool) -> i32 { -EOPNOTSUPP }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
