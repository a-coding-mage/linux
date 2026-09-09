/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright © 2019 Macronix
 * Author: Miquèl Raynal <miquel.raynal@bootlin.com>
 *
 * Header for the Macronix external ECC engine.
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct mxic_ecc_engine {
    _private: [u8; 0],
}

/* The following opaque types and aliases are supplied by other headers:
 * nand_ecc_engine_ops, nand_ecc_engine, platform_device, dma_addr_t.
 */

/* Build-time condition preserved from:
 * IS_ENABLED(CONFIG_MTD_NAND_ECC_MXIC) && IS_REACHABLE(CONFIG_MTD_NAND_CORE)
 */
#[cfg(all(feature = "CONFIG_MTD_NAND_ECC_MXIC", feature = "CONFIG_MTD_NAND_CORE"))]
extern "C" {
    pub fn mxic_ecc_get_pipelined_ops() -> *const nand_ecc_engine_ops;
    pub fn mxic_ecc_get_pipelined_engine(
        spi_pdev: *mut platform_device,
    ) -> *mut nand_ecc_engine;
    pub fn mxic_ecc_put_pipelined_engine(eng: *mut nand_ecc_engine);
    pub fn mxic_ecc_process_data_pipelined(
        eng: *mut nand_ecc_engine,
        direction: ::core::ffi::c_uint,
        dirmap: dma_addr_t,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(all(feature = "CONFIG_MTD_NAND_ECC_MXIC", feature = "CONFIG_MTD_NAND_CORE")))]
pub unsafe fn mxic_ecc_get_pipelined_ops() -> *const nand_ecc_engine_ops {
    core::ptr::null()
}

#[cfg(not(all(feature = "CONFIG_MTD_NAND_ECC_MXIC", feature = "CONFIG_MTD_NAND_CORE")))]
pub unsafe fn mxic_ecc_get_pipelined_engine(
    _spi_pdev: *mut platform_device,
) -> *mut nand_ecc_engine {
    /* ERR_PTR(-EOPNOTSUPP), with EOPNOTSUPP == 95. */
    (-95isize) as *mut nand_ecc_engine
}

#[cfg(not(all(feature = "CONFIG_MTD_NAND_ECC_MXIC", feature = "CONFIG_MTD_NAND_CORE")))]
pub unsafe fn mxic_ecc_put_pipelined_engine(_eng: *mut nand_ecc_engine) {}

#[cfg(not(all(feature = "CONFIG_MTD_NAND_ECC_MXIC", feature = "CONFIG_MTD_NAND_CORE")))]
pub unsafe fn mxic_ecc_process_data_pipelined(
    _eng: *mut nand_ecc_engine,
    _direction: ::core::ffi::c_uint,
    _dirmap: dma_addr_t,
) -> ::core::ffi::c_int {
    -95
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
