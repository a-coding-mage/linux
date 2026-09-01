// SPDX-License-Identifier: GPL-2.0-only
/*
 * sst_acpi.c - SST (LPE) driver init file for ACPI enumeration.
 *
 * Copyright (c) 2013, Intel Corporation.
 *
 *  Authors:	Ramesh Babu K V <Ramesh.Babu@intel.com>
 *  Authors:	Omair Mohammed Abdullah <omair.m.abdullah@intel.com>
 */

/* C include dependencies translated as external Rust dependencies:
 * linux/module.h, linux/fs.h, linux/interrupt.h, linux/slab.h,
 * linux/string.h, linux/io.h, linux/platform_device.h, linux/firmware.h,
 * linux/pm_qos.h, linux/dmi.h, linux/acpi.h, asm/platform_sst_audio.h,
 * sound/core.h, sound/intel-dsp-config.h, sound/soc.h,
 * sound/compress_driver.h, acpi/acbuffer.h, acpi/platform/acenv.h,
 * acpi/platform/aclinux.h, acpi/actypes.h, acpi/acpi_bus.h,
 * sound/soc-acpi.h, sound/soc-acpi-intel-match.h,
 * ../sst-mfld-platform.h, ../../common/soc-intel-quirks.h, sst.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

/* LPE viewpoint addresses */
const SST_BYT_IRAM_PHY_START: c_uint = 0xff2c0000;
const SST_BYT_IRAM_PHY_END: c_uint = 0xff2d4000;
const SST_BYT_DRAM_PHY_START: c_uint = 0xff300000;
const SST_BYT_DRAM_PHY_END: c_uint = 0xff320000;
const SST_BYT_IMR_VIRT_START: c_uint = 0xc0000000; /* virtual addr in LPE */
const SST_BYT_IMR_VIRT_END: c_uint = 0xc01fffff;
const SST_BYT_SHIM_PHY_ADDR: c_uint = 0xff340000;
const SST_BYT_MBOX_PHY_ADDR: c_uint = 0xff344000;
const SST_BYT_DMA0_PHY_ADDR: c_uint = 0xff298000;
const SST_BYT_DMA1_PHY_ADDR: c_uint = 0xff29c000;
const SST_BYT_SSP0_PHY_ADDR: c_uint = 0xff2a0000;
const SST_BYT_SSP2_PHY_ADDR: c_uint = 0xff2a2000;

const BYT_FW_MOD_TABLE_OFFSET: c_uint = 0x80000;
const BYT_FW_MOD_TABLE_SIZE: c_uint = 0x100;
const BYT_FW_MOD_OFFSET: c_uint = BYT_FW_MOD_TABLE_OFFSET + BYT_FW_MOD_TABLE_SIZE;

const IORESOURCE_MEM: c_uint = 0;
const EIO: c_int = 5;
const ENODEV: c_int = 19;
const SND_INTEL_DSP_DRIVER_ANY: c_int = 0;
const SND_INTEL_DSP_DRIVER_SST: c_int = 0;

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct resource {
    pub start: c_uint,
    pub end: c_uint,
}

#[repr(C)]
pub struct sst_info {
    pub use_elf: bool,
    pub max_streams: c_uint,
    pub iram_start: c_uint,
    pub iram_end: c_uint,
    pub iram_use: bool,
    pub dram_start: c_uint,
    pub dram_end: c_uint,
    pub dram_use: bool,
    pub imr_start: c_uint,
    pub imr_end: c_uint,
    pub imr_use: bool,
    pub mailbox_start: c_uint,
    pub num_probes: c_uint,
    pub lpe_viewpt_rqd: bool,
}

#[repr(C)]
pub struct sst_ipc_info {
    pub ipc_offset: c_uint,
    pub mbox_recv_off: c_uint,
}

#[repr(C)]
pub struct sst_lib_dnld_info {
    pub mod_base: c_uint,
    pub mod_end: c_uint,
    pub mod_table_offset: c_uint,
    pub mod_table_size: c_uint,
    pub mod_ddr_dnld: bool,
}

#[repr(C)]
pub struct sst_res_info {
    pub shim_offset: c_uint,
    pub shim_size: c_uint,
    pub shim_phy_addr: c_uint,
    pub ssp0_offset: c_uint,
    pub ssp0_size: c_uint,
    pub dma0_offset: c_uint,
    pub dma0_size: c_uint,
    pub dma1_offset: c_uint,
    pub dma1_size: c_uint,
    pub iram_offset: c_uint,
    pub iram_size: c_uint,
    pub dram_offset: c_uint,
    pub dram_size: c_uint,
    pub mbox_offset: c_uint,
    pub mbox_size: c_uint,
    pub acpi_lpe_res_index: c_uint,
    pub acpi_ddr_index: c_uint,
    pub acpi_ipc_irq_index: c_uint,
}

#[repr(C)]
pub struct sst_platform_info {
    pub probe_data: *const sst_info,
    pub ipc_info: *const sst_ipc_info,
    pub lib_info: *const sst_lib_dnld_info,
    pub res_info: *const sst_res_info,
    pub platform: *const c_char,
    pub streams_lost_on_suspend: bool,
}

#[repr(C)]
pub struct mach_params {
    pub acpi_ipc_irq_index: c_uint,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub pdata: *mut sst_platform_info,
    pub mach_params: mach_params,
    pub drv_name: *const c_char,
    pub fw_filename: *const c_char,
}

#[repr(C)]
pub struct intel_sst_drv {
    pub dev: *mut device,
    pub pdata: *mut sst_platform_info,
    pub info: sst_info,
    pub iram_base: c_uint,
    pub iram_end: c_uint,
    pub iram: *mut c_void,
    pub dram_base: c_uint,
    pub dram_end: c_uint,
    pub dram: *mut c_void,
    pub shim_phy_add: c_uint,
    pub shim: *mut c_void,
    pub mailbox_add: c_uint,
    pub mailbox: *mut c_void,
    pub ddr_base: c_uint,
    pub ddr_end: c_uint,
    pub ddr: *mut c_void,
    pub irq_num: c_int,
    pub firmware_name: *mut c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 9],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub acpi_match_table: *const acpi_device_id,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

unsafe extern "C" {
    static snd_soc_acpi_intel_baytrail_machines: snd_soc_acpi_mach;
    static snd_soc_acpi_intel_cherrytrail_machines: snd_soc_acpi_mach;
    static intel_sst_pm: c_void;

    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn platform_get_resource(
        pdev: *mut platform_device,
        type_: c_uint,
        num: c_uint,
    ) -> *mut resource;
    fn resource_size(res: *mut resource) -> c_uint;
    fn devm_ioremap(dev: *mut device, offset: c_uint, size: c_uint) -> *mut c_void;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn acpi_match_device(
        ids: *const acpi_device_id,
        dev: *mut device,
    ) -> *const acpi_device_id;
    fn snd_intel_acpi_dsp_driver_probe(dev: *mut device, hid: *const c_char) -> c_int;
    fn snd_soc_acpi_find_machine(mach: *mut snd_soc_acpi_mach) -> *mut snd_soc_acpi_mach;
    fn soc_intel_is_byt() -> bool;
    fn soc_intel_is_byt_cr(pdev: *mut platform_device) -> bool;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn kstrtouint(s: *const c_char, base: c_uint, res: *mut c_uint) -> c_int;
    fn sst_alloc_drv_context(
        ctx: *mut *mut intel_sst_drv,
        dev: *mut device,
        dev_id: c_uint,
    ) -> c_int;
    fn platform_device_register_data(
        parent: *mut device,
        name: *const c_char,
        id: c_int,
        data: *const c_void,
        size: usize,
    ) -> *mut platform_device;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn sst_context_init(ctx: *mut intel_sst_drv) -> c_int;
    fn sst_configure_runtime_pm(ctx: *mut intel_sst_drv);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn sst_context_cleanup(ctx: *mut intel_sst_drv);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

static BYT_FWPARSE_INFO: sst_info = sst_info {
    use_elf: false,
    max_streams: 25,
    iram_start: SST_BYT_IRAM_PHY_START,
    iram_end: SST_BYT_IRAM_PHY_END,
    iram_use: true,
    dram_start: SST_BYT_DRAM_PHY_START,
    dram_end: SST_BYT_DRAM_PHY_END,
    dram_use: true,
    imr_start: SST_BYT_IMR_VIRT_START,
    imr_end: SST_BYT_IMR_VIRT_END,
    imr_use: true,
    mailbox_start: SST_BYT_MBOX_PHY_ADDR,
    num_probes: 0,
    lpe_viewpt_rqd: true,
};

static BYT_IPC_INFO: sst_ipc_info = sst_ipc_info {
    ipc_offset: 0,
    mbox_recv_off: 0x400,
};

static BYT_LIB_DNLD_INFO: sst_lib_dnld_info = sst_lib_dnld_info {
    mod_base: SST_BYT_IMR_VIRT_START,
    mod_end: SST_BYT_IMR_VIRT_END,
    mod_table_offset: BYT_FW_MOD_TABLE_OFFSET,
    mod_table_size: BYT_FW_MOD_TABLE_SIZE,
    mod_ddr_dnld: false,
};

static BYT_RVP_RES_INFO: sst_res_info = sst_res_info {
    shim_offset: 0x140000,
    shim_size: 0x000100,
    shim_phy_addr: SST_BYT_SHIM_PHY_ADDR,
    ssp0_offset: 0xa0000,
    ssp0_size: 0x1000,
    dma0_offset: 0x98000,
    dma0_size: 0x4000,
    dma1_offset: 0x9c000,
    dma1_size: 0x4000,
    iram_offset: 0x0c0000,
    iram_size: 0x14000,
    dram_offset: 0x100000,
    dram_size: 0x28000,
    mbox_offset: 0x144000,
    mbox_size: 0x1000,
    acpi_lpe_res_index: 0,
    acpi_ddr_index: 2,
    acpi_ipc_irq_index: 5,
};

/* BYTCR has different BIOS from BYT */
static BYTCR_RES_INFO: sst_res_info = sst_res_info {
    shim_offset: 0x140000,
    shim_size: 0x000100,
    shim_phy_addr: SST_BYT_SHIM_PHY_ADDR,
    ssp0_offset: 0xa0000,
    ssp0_size: 0x1000,
    dma0_offset: 0x98000,
    dma0_size: 0x4000,
    dma1_offset: 0x9c000,
    dma1_size: 0x4000,
    iram_offset: 0x0c0000,
    iram_size: 0x14000,
    dram_offset: 0x100000,
    dram_size: 0x28000,
    mbox_offset: 0x144000,
    mbox_size: 0x1000,
    acpi_lpe_res_index: 0,
    acpi_ddr_index: 2,
    acpi_ipc_irq_index: 0,
};

/* For "LPE0F28" ACPI device found on some Android factory OS models */
static LPE8086_RES_INFO: sst_res_info = sst_res_info {
    shim_offset: 0x140000,
    shim_size: 0x000100,
    shim_phy_addr: SST_BYT_SHIM_PHY_ADDR,
    ssp0_offset: 0xa0000,
    ssp0_size: 0x1000,
    dma0_offset: 0x98000,
    dma0_size: 0x4000,
    dma1_offset: 0x9c000,
    dma1_size: 0x4000,
    iram_offset: 0x0c0000,
    iram_size: 0x14000,
    dram_offset: 0x100000,
    dram_size: 0x28000,
    mbox_offset: 0x144000,
    mbox_size: 0x1000,
    acpi_lpe_res_index: 1,
    acpi_ddr_index: 0,
    acpi_ipc_irq_index: 0,
};

static mut BYT_RVP_PLATFORM_DATA: sst_platform_info = sst_platform_info {
    probe_data: &BYT_FWPARSE_INFO,
    ipc_info: &BYT_IPC_INFO,
    lib_info: &BYT_LIB_DNLD_INFO,
    res_info: &BYT_RVP_RES_INFO,
    platform: b"sst-mfld-platform\0".as_ptr() as *const c_char,
    streams_lost_on_suspend: true,
};

/* Cherryview (Cherrytrail and Braswell) uses same mrfld dpcm fw as Baytrail,
 * so pdata is same as Baytrail, minus the streams_lost_on_suspend quirk.
 */
static mut CHV_PLATFORM_DATA: sst_platform_info = sst_platform_info {
    probe_data: &BYT_FWPARSE_INFO,
    ipc_info: &BYT_IPC_INFO,
    lib_info: &BYT_LIB_DNLD_INFO,
    res_info: &BYT_RVP_RES_INFO,
    platform: b"sst-mfld-platform\0".as_ptr() as *const c_char,
    streams_lost_on_suspend: false,
};

unsafe extern "C" fn sst_platform_get_resources(ctx: *mut intel_sst_drv) -> c_int {
    let mut rsrc: *mut resource;
    let pdev: *mut platform_device = to_platform_device((*ctx).dev);

    /* All ACPI resource request here */
    /* Get Shim addr */
    rsrc = platform_get_resource(
        pdev,
        IORESOURCE_MEM,
        (*(*(*ctx).pdata).res_info).acpi_lpe_res_index,
    );
    if rsrc.is_null() {
        dev_err((*ctx).dev, b"Invalid SHIM base from IFWI\n\0".as_ptr() as *const c_char);
        return -EIO;
    }
    dev_info(
        (*ctx).dev,
        b"LPE base: %#x size:%#x\0".as_ptr() as *const c_char,
        (*rsrc).start as c_uint,
        resource_size(rsrc) as c_uint,
    );

    (*ctx).iram_base = (*rsrc)
        .start
        .wrapping_add((*(*(*ctx).pdata).res_info).iram_offset);
    (*ctx).iram_end = (*ctx)
        .iram_base
        .wrapping_add((*(*(*ctx).pdata).res_info).iram_size)
        .wrapping_sub(1);
    dev_info((*ctx).dev, b"IRAM base: %#x\0".as_ptr() as *const c_char, (*ctx).iram_base);
    (*ctx).iram = devm_ioremap(
        (*ctx).dev,
        (*ctx).iram_base,
        (*(*(*ctx).pdata).res_info).iram_size,
    );
    if (*ctx).iram.is_null() {
        dev_err((*ctx).dev, b"unable to map IRAM\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    (*ctx).dram_base = (*rsrc)
        .start
        .wrapping_add((*(*(*ctx).pdata).res_info).dram_offset);
    (*ctx).dram_end = (*ctx)
        .dram_base
        .wrapping_add((*(*(*ctx).pdata).res_info).dram_size)
        .wrapping_sub(1);
    dev_info((*ctx).dev, b"DRAM base: %#x\0".as_ptr() as *const c_char, (*ctx).dram_base);
    (*ctx).dram = devm_ioremap(
        (*ctx).dev,
        (*ctx).dram_base,
        (*(*(*ctx).pdata).res_info).dram_size,
    );
    if (*ctx).dram.is_null() {
        dev_err((*ctx).dev, b"unable to map DRAM\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    (*ctx).shim_phy_add = (*rsrc)
        .start
        .wrapping_add((*(*(*ctx).pdata).res_info).shim_offset);
    dev_info(
        (*ctx).dev,
        b"SHIM base: %#x\0".as_ptr() as *const c_char,
        (*ctx).shim_phy_add,
    );
    (*ctx).shim = devm_ioremap(
        (*ctx).dev,
        (*ctx).shim_phy_add,
        (*(*(*ctx).pdata).res_info).shim_size,
    );
    if (*ctx).shim.is_null() {
        dev_err((*ctx).dev, b"unable to map SHIM\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    /* reassign physical address to LPE viewpoint address */
    (*ctx).shim_phy_add = (*(*(*ctx).pdata).res_info).shim_phy_addr;

    /* Get mailbox addr */
    (*ctx).mailbox_add = (*rsrc)
        .start
        .wrapping_add((*(*(*ctx).pdata).res_info).mbox_offset);
    dev_info(
        (*ctx).dev,
        b"Mailbox base: %#x\0".as_ptr() as *const c_char,
        (*ctx).mailbox_add,
    );
    (*ctx).mailbox = devm_ioremap(
        (*ctx).dev,
        (*ctx).mailbox_add,
        (*(*(*ctx).pdata).res_info).mbox_size,
    );
    if (*ctx).mailbox.is_null() {
        dev_err((*ctx).dev, b"unable to map mailbox\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    /* reassign physical address to LPE viewpoint address */
    (*ctx).mailbox_add = (*ctx).info.mailbox_start;

    rsrc = platform_get_resource(
        pdev,
        IORESOURCE_MEM,
        (*(*(*ctx).pdata).res_info).acpi_ddr_index,
    );
    if rsrc.is_null() {
        dev_err((*ctx).dev, b"Invalid DDR base from IFWI\n\0".as_ptr() as *const c_char);
        return -EIO;
    }
    (*ctx).ddr_base = (*rsrc).start;
    (*ctx).ddr_end = (*rsrc).end;
    dev_info((*ctx).dev, b"DDR base: %#x\0".as_ptr() as *const c_char, (*ctx).ddr_base);
    (*ctx).ddr = devm_ioremap((*ctx).dev, (*ctx).ddr_base, resource_size(rsrc));
    if (*ctx).ddr.is_null() {
        dev_err((*ctx).dev, b"unable to map DDR\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    /* Find the IRQ */
    (*ctx).irq_num = platform_get_irq(pdev, (*(*(*ctx).pdata).res_info).acpi_ipc_irq_index);
    if (*ctx).irq_num <= 0 {
        return if (*ctx).irq_num < 0 { (*ctx).irq_num } else { -EIO };
    }

    0
}

unsafe extern "C" fn sst_acpi_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let mut ret: c_int = 0;
    let mut ctx: *mut intel_sst_drv = ptr::null_mut();
    let id: *const acpi_device_id;
    let mut mach: *mut snd_soc_acpi_mach;
    let mdev: *mut platform_device;
    let plat_dev: *mut platform_device;
    let pdata: *mut sst_platform_info;
    let mut dev_id: c_uint = 0;

    id = acpi_match_device((*(*dev).driver).acpi_match_table, dev);
    if id.is_null() {
        return -ENODEV;
    }

    ret = snd_intel_acpi_dsp_driver_probe(dev, (*id).id.as_ptr());
    if ret != SND_INTEL_DSP_DRIVER_ANY && ret != SND_INTEL_DSP_DRIVER_SST {
        dev_dbg(
            dev,
            b"SST ACPI driver not selected, aborting probe\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }

    dev_dbg(dev, b"for %s\n\0".as_ptr() as *const c_char, (*id).id.as_ptr());

    mach = (*id).driver_data as *mut snd_soc_acpi_mach;
    mach = snd_soc_acpi_find_machine(mach);
    if mach.is_null() {
        dev_err(dev, b"No matching machine driver found\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    if soc_intel_is_byt() {
        (*mach).pdata = &raw mut BYT_RVP_PLATFORM_DATA;
    } else {
        (*mach).pdata = &raw mut CHV_PLATFORM_DATA;
    }
    pdata = (*mach).pdata;

    if strcmp((*id).id.as_ptr(), b"LPE0F28\0".as_ptr() as *const c_char) == 0 {
        let rsrc: *mut resource;

        /* Use regular BYT SST PCI VID:PID */
        dev_id = 0x80860F28;
        BYT_RVP_PLATFORM_DATA.res_info = &LPE8086_RES_INFO;

        /*
         * The "LPE0F28" ACPI device has separate IO-mem resources for:
         * DDR, SHIM, MBOX, IRAM, DRAM, CFG
         * None of which covers the entire LPE base address range.
         * lpe8086_res_info.acpi_lpe_res_index points to the SHIM.
         * Patch this to cover the entire base address range as expected
         * by sst_platform_get_resources().
         */
        rsrc = platform_get_resource(pdev, IORESOURCE_MEM, (*(*pdata).res_info).acpi_lpe_res_index);
        if rsrc.is_null() {
            dev_err(dev, b"Invalid SHIM base\n\0".as_ptr() as *const c_char);
            return -EIO;
        }
        (*rsrc).start = (*rsrc).start.wrapping_sub((*(*pdata).res_info).shim_offset);
        (*rsrc).end = (*rsrc).start.wrapping_add(0x200000).wrapping_sub(1);
    } else {
        ret = kstrtouint((*id).id.as_ptr(), 16, &mut dev_id);
        if ret < 0 {
            dev_err(
                dev,
                b"Unique device id conversion error: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        if soc_intel_is_byt_cr(pdev) {
            BYT_RVP_PLATFORM_DATA.res_info = &BYTCR_RES_INFO;
        }
    }

    dev_dbg(dev, b"ACPI device id: %x\n\0".as_ptr() as *const c_char, dev_id);

    ret = sst_alloc_drv_context(&mut ctx, dev, dev_id);
    if ret < 0 {
        return ret;
    }

    /* update machine parameters */
    (*mach).mach_params.acpi_ipc_irq_index = (*(*pdata).res_info).acpi_ipc_irq_index;

    plat_dev = platform_device_register_data(dev, (*pdata).platform, -1, ptr::null(), 0);
    if IS_ERR(plat_dev as *const c_void) {
        dev_err(
            dev,
            b"Failed to create machine device: %s\n\0".as_ptr() as *const c_char,
            (*pdata).platform,
        );
        return PTR_ERR(plat_dev as *const c_void);
    }

    /*
     * Create platform device for sst machine driver,
     * pass machine info as pdata
     */
    mdev = platform_device_register_data(
        dev,
        (*mach).drv_name,
        -1,
        mach as *const c_void,
        core::mem::size_of::<snd_soc_acpi_mach>(),
    );
    if IS_ERR(mdev as *const c_void) {
        dev_err(
            dev,
            b"Failed to create machine device: %s\n\0".as_ptr() as *const c_char,
            (*mach).drv_name,
        );
        return PTR_ERR(mdev as *const c_void);
    }

    /* Fill sst platform data */
    (*ctx).pdata = pdata;
    strscpy((*ctx).firmware_name, (*mach).fw_filename);

    ret = sst_platform_get_resources(ctx);
    if ret != 0 {
        return ret;
    }

    ret = sst_context_init(ctx);
    if ret < 0 {
        return ret;
    }

    sst_configure_runtime_pm(ctx);
    platform_set_drvdata(pdev, ctx as *mut c_void);
    ret
}

/**
* sst_acpi_remove - remove function
*
* @pdev:	platform device structure
*
* This function is called by OS when a device is unloaded
* This frees the interrupt etc
*/
unsafe extern "C" fn sst_acpi_remove(pdev: *mut platform_device) {
    let ctx: *mut intel_sst_drv;

    ctx = platform_get_drvdata(pdev) as *mut intel_sst_drv;
    sst_context_cleanup(ctx);
    platform_set_drvdata(pdev, ptr::null_mut());
}

static SST_ACPI_IDS: [acpi_device_id; 4] = [
    acpi_device_id {
        id: [
            b'L' as c_char,
            b'P' as c_char,
            b'E' as c_char,
            b'0' as c_char,
            b'F' as c_char,
            b'2' as c_char,
            b'8' as c_char,
            0,
            0,
        ],
        driver_data: unsafe { &snd_soc_acpi_intel_baytrail_machines as *const _ as c_ulong },
    },
    acpi_device_id {
        id: [
            b'8' as c_char,
            b'0' as c_char,
            b'8' as c_char,
            b'6' as c_char,
            b'0' as c_char,
            b'F' as c_char,
            b'2' as c_char,
            b'8' as c_char,
            0,
        ],
        driver_data: unsafe { &snd_soc_acpi_intel_baytrail_machines as *const _ as c_ulong },
    },
    acpi_device_id {
        id: [
            b'8' as c_char,
            b'0' as c_char,
            b'8' as c_char,
            b'6' as c_char,
            b'2' as c_char,
            b'2' as c_char,
            b'A' as c_char,
            b'8' as c_char,
            0,
        ],
        driver_data: unsafe { &snd_soc_acpi_intel_cherrytrail_machines as *const _ as c_ulong },
    },
    acpi_device_id {
        id: [0; 9],
        driver_data: 0,
    },
];

/* MODULE_DEVICE_TABLE(acpi, sst_acpi_ids); */

static mut SST_ACPI_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"intel_sst_acpi\0".as_ptr() as *const c_char,
        acpi_match_table: SST_ACPI_IDS.as_ptr(),
        pm: unsafe { &intel_sst_pm as *const _ as *const c_void },
    },
    probe: Some(sst_acpi_probe),
    remove: Some(sst_acpi_remove),
};

/* module_platform_driver(sst_acpi_driver); */

/* MODULE_DESCRIPTION("Intel (R) SST(R) Audio Engine ACPI Driver"); */
/* MODULE_AUTHOR("Ramesh Babu K V"); */
/* MODULE_AUTHOR("Omair Mohammed Abdullah"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("sst"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
