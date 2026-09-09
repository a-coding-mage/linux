/* SPDX-License-Identifier: GPL-2.0 */

pub unsafe extern "C" fn mpc85xx_common_publish_devices() -> ::core::ffi::c_int;

// CONFIG_CPM2
#[cfg(feature = "CONFIG_CPM2")]
pub unsafe extern "C" fn mpc85xx_cpm2_pic_init();

#[cfg(not(feature = "CONFIG_CPM2"))]
#[inline]
pub unsafe extern "C" fn mpc85xx_cpm2_pic_init() {}

// CONFIG_QUICC_ENGINE
#[cfg(feature = "CONFIG_QUICC_ENGINE")]
pub unsafe extern "C" fn mpc85xx_qe_par_io_init();

#[cfg(not(feature = "CONFIG_QUICC_ENGINE"))]
#[inline]
pub unsafe extern "C" fn mpc85xx_qe_par_io_init() {}

// CONFIG_PPC_I8259
#[cfg(feature = "CONFIG_PPC_I8259")]
pub unsafe extern "C" fn mpc85xx_8259_init();

#[cfg(not(feature = "CONFIG_PPC_I8259"))]
#[inline]
pub unsafe extern "C" fn mpc85xx_8259_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
