/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */

// C includes removed:
// <linux/clk.h>
// <linux/of_platform.h>
// <sound/sof/xtensa.h>
// "../sof-of-dev.h"
// "../ops.h"

pub const EXCEPT_MAX_HDR_SIZE: u32 = 0x400;
pub const IMX8_STACK_DUMP_SIZE: u32 = 32;

// chip_info refers to the data stored in struct sof_dev_desc's chip_info
#[macro_export]
macro_rules! get_chip_info {
    ($sdev:expr) => {
        ((*(*(*($sdev)).pdata).desc).chip_info as *const imx_chip_info)
    };
}

// chip_pdata refers to the data stored in struct imx_common_data's chip_pdata
#[macro_export]
macro_rules! get_chip_pdata {
    ($sdev:expr) => {
        (*((*(*($sdev)).pdata).hw_pdata as *mut imx_common_data)).chip_pdata
    };
}

/*
 * can be used if:
 *      1) The only supported IPC version is IPC3.
 *      2) The default paths/FW name match values below.
 *
 * otherwise, just explicitly declare the structure
 */
#[macro_export]
macro_rules! IMX_SOF_DEV_DESC {
    ($mach_name:ident, $of_machs:expr, $mach_chip_info:expr, $mach_ops:expr, $mach_ops_init:expr) => {
        static mut sof_of_$mach_name##_desc: sof_dev_desc = sof_dev_desc {
            of_machines: $of_machs,
            chip_info: $mach_chip_info,
            ipc_supported_mask: BIT(SOF_IPC_TYPE_3),
            ipc_default: SOF_IPC_TYPE_3,
            default_fw_path: {
                let mut a = Default::default();
                a[SOF_IPC_TYPE_3 as usize] = c"imx/sof".as_ptr();
                a
            },
            default_tplg_path: {
                let mut a = Default::default();
                a[SOF_IPC_TYPE_3 as usize] = c"imx/sof-tplg".as_ptr();
                a
            },
            default_fw_filename: {
                let mut a = Default::default();
                a[SOF_IPC_TYPE_3 as usize] = concat!("sof-", stringify!($mach_name), ".ri\0").as_ptr() as *const ::core::ffi::c_char;
                a
            },
            ops: $mach_ops,
            ops_init: $mach_ops_init,
        };
    };
}

// to be used alongside IMX_SOF_DEV_DESC()
#[macro_export]
macro_rules! IMX_SOF_DEV_DESC_NAME {
    ($mach_name:ident) => {
        sof_of_$mach_name##_desc
    };
}

/*
 * dai driver entry w/ playback and capture caps. If one direction is missing
 * then set the channels to 0.
 */
#[macro_export]
macro_rules! IMX_SOF_DAI_DRV_ENTRY {
    ($dai_name:expr, $pb_cmin:expr, $pb_cmax:expr, $cap_cmin:expr, $cap_cmax:expr) => {
        snd_soc_dai_driver {
            name: $dai_name,
            playback: snd_soc_pcm_stream {
                channels_min: $pb_cmin,
                channels_max: $pb_cmax,
                ..Default::default()
            },
            capture: snd_soc_pcm_stream {
                channels_min: $cap_cmin,
                channels_max: $cap_cmax,
                ..Default::default()
            },
            ..Default::default()
        }
    };
}

// use if playback and capture have the same min/max channel count
#[macro_export]
macro_rules! IMX_SOF_DAI_DRV_ENTRY_BIDIR {
    ($dai_name:expr, $cmin:expr, $cmax:expr) => {
        IMX_SOF_DAI_DRV_ENTRY!($dai_name, $cmin, $cmax, $cmin, $cmax)
    };
}

#[repr(C)]
pub struct imx_ipc_info {
    // true if core is able to write a panic code to the debug box
    pub has_panic_code: bool,
    // offset to mailbox in which firmware initially writes FW_READY
    pub boot_mbox_offset: ::core::ffi::c_int,
    // offset to region at which the mailboxes start
    pub window_offset: ::core::ffi::c_int,
}

#[repr(C)]
pub struct imx_chip_ops {
    // called after clocks and PDs are enabled
    pub probe: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int>,
    // used directly by the SOF core
    pub core_kick: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int>,
    // called during suspend()/remove() before clocks are disabled
    pub core_shutdown: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int>,
    // used directly by the SOF core
    pub core_reset: Option<unsafe extern "C" fn(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct imx_memory_info {
    pub name: *const ::core::ffi::c_char,
    pub reserved: bool,
}

#[repr(C)]
pub struct imx_chip_info {
    pub ipc_info: imx_ipc_info,
    // does the chip have a reserved memory region for DMA?
    pub has_dma_reserved: bool,
    pub memory: *mut imx_memory_info,
    pub drv: *mut snd_soc_dai_driver,
    pub num_drv: ::core::ffi::c_int,
    // optional
    pub ops: *const imx_chip_ops,
}

#[repr(C)]
pub struct imx_common_data {
    pub ipc_dev: *mut platform_device,
    pub ipc_handle: *mut imx_dsp_ipc,
    // core may have no clocks
    pub clks: *mut clk_bulk_data,
    pub clk_num: ::core::ffi::c_int,
    // core may have no PDs
    pub pd_list: *mut dev_pm_domain_list,
    pub chip_pdata: *mut ::core::ffi::c_void,
}

#[inline]
pub unsafe fn imx_chip_core_kick(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int {
    let ops: *const imx_chip_ops = (*get_chip_info!(sdev)).ops;

    if !ops.is_null() {
        if let Some(core_kick) = (*ops).core_kick {
            return core_kick(sdev);
        }
    }

    0
}

#[inline]
pub unsafe fn imx_chip_core_shutdown(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int {
    let ops: *const imx_chip_ops = (*get_chip_info!(sdev)).ops;

    if !ops.is_null() {
        if let Some(core_shutdown) = (*ops).core_shutdown {
            return core_shutdown(sdev);
        }
    }

    0
}

#[inline]
pub unsafe fn imx_chip_core_reset(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int {
    let ops: *const imx_chip_ops = (*get_chip_info!(sdev)).ops;

    if !ops.is_null() {
        if let Some(core_reset) = (*ops).core_reset {
            return core_reset(sdev);
        }
    }

    0
}

#[inline]
pub unsafe fn imx_chip_probe(sdev: *mut snd_sof_dev) -> ::core::ffi::c_int {
    let ops: *const imx_chip_ops = (*get_chip_info!(sdev)).ops;

    if !ops.is_null() {
        if let Some(probe) = (*ops).probe {
            return probe(sdev);
        }
    }

    0
}

extern "C" {
    pub fn imx8_get_registers(
        sdev: *mut snd_sof_dev,
        xoops: *mut sof_ipc_dsp_oops_xtensa,
        panic_info: *mut sof_ipc_panic_info,
        stack: *mut u32,
        stack_words: usize,
    );

    pub fn imx8_dump(sdev: *mut snd_sof_dev, flags: u32);

    pub static sof_imx_ops: snd_sof_dsp_ops;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
