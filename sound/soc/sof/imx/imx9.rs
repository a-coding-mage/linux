// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
/*
 * Copyright 2025 NXP
 */

// C dependencies:
// #include <linux/firmware/imx/sm.h>
// #include "imx-common.h"

const IMX95_M7_CPU_ID: u32 = 0x1;
const IMX95_M7_LM_ID: u32 = 0x1;

static mut imx95_dai: [snd_soc_dai_driver; 1] = [IMX_SOF_DAI_DRV_ENTRY_BIDIR!("sai3", 1, 32)];

static mut sof_imx9_ops: snd_sof_dsp_ops = unsafe { core::mem::zeroed() };

unsafe extern "C" {
    static sof_imx_ops: snd_sof_dsp_ops;
    static sof_of_pm: dev_pm_ops;

    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize)
        -> *mut core::ffi::c_void;
    fn get_chip_info(sdev: *mut snd_sof_dev) -> *const imx_chip_info;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn platform_get_resource_byname(
        dev: *mut platform_device,
        ty: u32,
        name: *const core::ffi::c_char,
    ) -> *mut resource;
    fn dev_err_probe(
        dev: *mut device,
        err: core::ffi::c_int,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn scmi_imx_lmm_reset_vector_set(
        lm_id: u32,
        cpu_id: u32,
        flags: u32,
        addr: resource_size_t,
    ) -> core::ffi::c_int;
    fn scmi_imx_lmm_operation(
        lm_id: u32,
        operation: u32,
        flags: u32,
    ) -> core::ffi::c_int;
    fn sof_of_probe(pdev: *mut platform_device) -> core::ffi::c_int;
    fn sof_of_remove(pdev: *mut platform_device);
}

unsafe extern "C" fn imx95_ops_init(sdev: *mut snd_sof_dev) -> core::ffi::c_int {
    /* first copy from template */
    memcpy(
        &raw mut sof_imx9_ops as *mut core::ffi::c_void,
        &raw const sof_imx_ops as *const core::ffi::c_void,
        core::mem::size_of::<snd_sof_dsp_ops>(),
    );

    /* ... and finally set DAI driver */
    sof_imx9_ops.drv = (*get_chip_info(sdev)).drv;
    sof_imx9_ops.num_drv = (*get_chip_info(sdev)).num_drv;

    0
}

unsafe extern "C" fn imx95_chip_probe(sdev: *mut snd_sof_dev) -> core::ffi::c_int {
    let pdev: *mut platform_device;
    let res: *mut resource;

    pdev = to_platform_device((*sdev).dev);

    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, c"sram".as_ptr());
    if res.is_null() {
        return dev_err_probe(
            (*sdev).dev,
            -ENODEV,
            c"failed to fetch SRAM region\n".as_ptr(),
        );
    }

    scmi_imx_lmm_reset_vector_set(IMX95_M7_LM_ID, IMX95_M7_CPU_ID, 0, (*res).start)
}

unsafe extern "C" fn imx95_core_kick(_sdev: *mut snd_sof_dev) -> core::ffi::c_int {
    scmi_imx_lmm_operation(IMX95_M7_LM_ID, SCMI_IMX_LMM_BOOT, 0)
}

unsafe extern "C" fn imx95_core_shutdown(_sdev: *mut snd_sof_dev) -> core::ffi::c_int {
    scmi_imx_lmm_operation(
        IMX95_M7_LM_ID,
        SCMI_IMX_LMM_SHUTDOWN,
        SCMI_IMX_LMM_OP_FORCEFUL,
    )
}

static imx95_chip_ops: imx_chip_ops = imx_chip_ops {
    probe: Some(imx95_chip_probe),
    core_kick: Some(imx95_core_kick),
    core_shutdown: Some(imx95_core_shutdown),
};

static mut imx95_memory_regions: [imx_memory_info; 2] = [
    imx_memory_info {
        name: c"sram".as_ptr(),
        reserved: false,
    },
    imx_memory_info::default(),
];

static imx95_chip_info: imx_chip_info = imx_chip_info {
    ipc_info: imx_ipc_info {
        boot_mbox_offset: 0x6001000,
        window_offset: 0x6000000,
    },
    has_dma_reserved: true,
    memory: unsafe { imx95_memory_regions.as_ptr() as *mut imx_memory_info },
    drv: unsafe { imx95_dai.as_ptr() as *mut snd_soc_dai_driver },
    num_drv: ARRAY_SIZE!(imx95_dai),
    ops: &imx95_chip_ops,
};

static mut sof_imx9_machs: [snd_sof_of_mach; 2] = [
    snd_sof_of_mach {
        compatible: c"fsl,imx95-19x19-evk".as_ptr(),
        sof_tplg_filename: c"sof-imx95-wm8962.tplg".as_ptr(),
        drv_name: c"asoc-audio-graph-card2".as_ptr(),
    },
    snd_sof_of_mach::default(),
];

IMX_SOF_DEV_DESC!(
    imx95,
    sof_imx9_machs,
    &imx95_chip_info,
    &raw mut sof_imx9_ops,
    imx95_ops_init
);

static sof_of_imx9_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"fsl,imx95-cm7-sof".as_ptr(),
        data: &IMX_SOF_DEV_DESC_NAME!(imx95) as *const _ as *const core::ffi::c_void,
    },
    of_device_id::default(),
];
MODULE_DEVICE_TABLE!(of, sof_of_imx9_ids);

static mut snd_sof_of_imx9_driver: platform_driver = platform_driver {
    probe: Some(sof_of_probe),
    remove: Some(sof_of_remove),
    driver: device_driver {
        name: c"sof-audio-of-imx9".as_ptr(),
        pm: pm_ptr!(&sof_of_pm),
        of_match_table: sof_of_imx9_ids.as_ptr(),
    },
};
module_platform_driver!(snd_sof_of_imx9_driver);

MODULE_LICENSE!("Dual BSD/GPL");
MODULE_DESCRIPTION!("SOF driver for imx9 platforms");
MODULE_AUTHOR!("Laurentiu Mihalcea <laurentiu.mihalcea@nxp.com>");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
