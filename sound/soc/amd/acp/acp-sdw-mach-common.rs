// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2024 Advanced Micro Devices, Inc.

/*
 *  acp-sdw-mach-common - Common machine driver helper functions for
 *  legacy(No DSP) stack and SOF stack.
 */

use core::ffi::{c_char, c_int};

// Dependencies from linux/device.h and soc_amd_sdw_common.h are expected to be
// provided by the surrounding translated repository.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn get_acp63_cpu_pin_id(
    sdw_link_id: u32,
    be_id: c_int,
    cpu_pin_id: *mut c_int,
    dev: *mut device,
) -> c_int {
    match sdw_link_id {
        x if x == AMD_SDW0 => {
            match be_id {
                x if x == SOC_SDW_JACK_OUT_DAI_ID => {
                    unsafe {
                        *cpu_pin_id = ACP63_SW0_AUDIO0_TX;
                    }
                }
                x if x == SOC_SDW_JACK_IN_DAI_ID => {
                    unsafe {
                        *cpu_pin_id = ACP63_SW0_AUDIO0_RX;
                    }
                }
                x if x == SOC_SDW_AMP_OUT_DAI_ID => {
                    unsafe {
                        *cpu_pin_id = ACP63_SW0_AUDIO1_TX;
                    }
                }
                x if x == SOC_SDW_AMP_IN_DAI_ID => {
                    unsafe {
                        *cpu_pin_id = ACP63_SW0_AUDIO1_RX;
                    }
                }
                x if x == SOC_SDW_DMIC_DAI_ID => {
                    unsafe {
                        *cpu_pin_id = ACP63_SW0_AUDIO2_RX;
                    }
                }
                _ => {
                    unsafe {
                        dev_err(dev, c"Invalid be id:%d\n".as_ptr(), be_id);
                    }
                    return -EINVAL;
                }
            }
        }
        x if x == AMD_SDW1 => {
            match be_id {
                x if x == SOC_SDW_JACK_OUT_DAI_ID || x == SOC_SDW_AMP_OUT_DAI_ID => {
                    unsafe {
                        *cpu_pin_id = ACP63_SW1_AUDIO0_TX;
                    }
                }
                x if x == SOC_SDW_JACK_IN_DAI_ID
                    || x == SOC_SDW_AMP_IN_DAI_ID
                    || x == SOC_SDW_DMIC_DAI_ID =>
                {
                    unsafe {
                        *cpu_pin_id = ACP63_SW1_AUDIO0_RX;
                    }
                }
                _ => {
                    unsafe {
                        dev_err(dev, c"invalid be_id:%d\n".as_ptr(), be_id);
                    }
                    return -EINVAL;
                }
            }
        }
        _ => {
            unsafe {
                dev_err(dev, c"Invalid link id:%d\n".as_ptr(), sdw_link_id);
            }
            return -EINVAL;
        }
    }
    0
}
// EXPORT_SYMBOL_NS_GPL(get_acp63_cpu_pin_id, "SND_SOC_AMD_SDW_MACH");

#[no_mangle]
pub unsafe extern "C" fn get_acp70_cpu_pin_id(
    sdw_link_id: u32,
    be_id: c_int,
    cpu_pin_id: *mut c_int,
    dev: *mut device,
) -> c_int {
    match sdw_link_id {
        x if x == AMD_SDW0 || x == AMD_SDW1 => {
            match be_id {
                x if x == SOC_SDW_JACK_OUT_DAI_ID => {
                    unsafe {
                        *cpu_pin_id = ACP70_SW_AUDIO0_TX;
                    }
                }
                x if x == SOC_SDW_JACK_IN_DAI_ID => {
                    unsafe {
                        *cpu_pin_id = ACP70_SW_AUDIO0_RX;
                    }
                }
                x if x == SOC_SDW_AMP_OUT_DAI_ID => {
                    unsafe {
                        *cpu_pin_id = ACP70_SW_AUDIO1_TX;
                    }
                }
                x if x == SOC_SDW_AMP_IN_DAI_ID => {
                    unsafe {
                        *cpu_pin_id = ACP70_SW_AUDIO1_RX;
                    }
                }
                x if x == SOC_SDW_DMIC_DAI_ID => {
                    unsafe {
                        *cpu_pin_id = ACP70_SW_AUDIO2_RX;
                    }
                }
                _ => {
                    unsafe {
                        dev_err(dev, c"Invalid be id:%d\n".as_ptr(), be_id);
                    }
                    return -EINVAL;
                }
            }
        }
        _ => {
            return -EINVAL;
        }
    }
    unsafe {
        dev_dbg(
            dev,
            c"sdw_link_id:%d, be_id:%d, cpu_pin_id:%d\n".as_ptr(),
            sdw_link_id,
            be_id,
            *cpu_pin_id,
        );
    }
    0
}
// EXPORT_SYMBOL_NS_GPL(get_acp70_cpu_pin_id, "SND_SOC_AMD_SDW_MACH");

// MODULE_DESCRIPTION("AMD SoundWire Common Machine driver");
// MODULE_AUTHOR("Vijendar Mukunda <Vijendar.Mukunda@amd.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
