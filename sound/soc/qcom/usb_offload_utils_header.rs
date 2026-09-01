/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.
 */

use core::ffi::c_int;

// Dependency intent from C: #include <sound/soc.h>
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

// C conditional intent: #if IS_ENABLED(CONFIG_SND_SOC_QCOM_OFFLOAD_UTILS)
#[cfg(CONFIG_SND_SOC_QCOM_OFFLOAD_UTILS)]
unsafe extern "C" {
    pub fn qcom_snd_usb_offload_jack_setup(
        rtd: *mut snd_soc_pcm_runtime,
        jack: *mut snd_soc_jack,
        jack_setup: *mut bool,
    ) -> c_int;

    pub fn qcom_snd_usb_offload_jack_remove(
        rtd: *mut snd_soc_pcm_runtime,
        jack_setup: *mut bool,
    ) -> c_int;
}

// C conditional intent: #else
#[cfg(not(CONFIG_SND_SOC_QCOM_OFFLOAD_UTILS))]
pub unsafe extern "C" fn qcom_snd_usb_offload_jack_setup(
    _rtd: *mut snd_soc_pcm_runtime,
    _jack: *mut snd_soc_jack,
    _jack_setup: *mut bool,
) -> c_int {
    -ENODEV
}

#[cfg(not(CONFIG_SND_SOC_QCOM_OFFLOAD_UTILS))]
pub unsafe extern "C" fn qcom_snd_usb_offload_jack_remove(
    _rtd: *mut snd_soc_pcm_runtime,
    _jack_setup: *mut bool,
) -> c_int {
    -ENODEV
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
