// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD ACPI support for ACPI2platform device.
 *
 * Copyright (c) 2014,2015 AMD Corporation.
 * Authors: Ken Xue <Ken.Xue@amd.com>
 *	Wu, Jeff <Jeff.Wu@amd.com>
 */

// Linux kernel dependencies are supplied by the surrounding crate.

struct ApdPrivateData;

/**
 * struct apd_device_desc - a descriptor for apd device
 * @fixed_clk_rate: fixed rate input clock source for acpi device;
 *			0 means no fixed rate input clock source
 * @properties: build-in properties of the device such as UART
 * @setup: a hook routine to set device resource during create platform device
 *
 * Device description defined as acpi_device_id.driver_data
 */
#[repr(C)]
struct ApdDeviceDesc {
    fixed_clk_rate: u32,
    properties: *mut PropertyEntry,
    setup: Option<unsafe extern "C" fn(*mut ApdPrivateData) -> i32>,
}

#[repr(C)]
struct ApdPrivateData {
    clk: *mut Clk,
    adev: *mut AcpiDevice,
    dev_desc: *const ApdDeviceDesc,
}

#[cfg(any(CONFIG_X86_AMD_PLATFORM_DEVICE, CONFIG_ARM64))]
const fn apd_addr(desc: &'static ApdDeviceDesc) -> usize {
    desc as *const ApdDeviceDesc as usize
}

#[cfg(any(CONFIG_X86_AMD_PLATFORM_DEVICE, CONFIG_ARM64))]
unsafe extern "C" fn acpi_apd_setup(pdata: *mut ApdPrivateData) -> i32 {
    let dev_desc = (*pdata).dev_desc;
    let mut clk: *mut Clk;

    if (*dev_desc).fixed_clk_rate != 0 {
        clk = clk_register_fixed_rate(
            &mut (*(*pdata).adev).dev,
            dev_name(&mut (*(*pdata).adev).dev),
            core::ptr::null(),
            0,
            (*dev_desc).fixed_clk_rate,
        );
        clk_register_clkdev(clk, core::ptr::null(), dev_name(&mut (*(*pdata).adev).dev));
        (*pdata).clk = clk;
    }

    0
}

#[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)]
unsafe extern "C" fn fch_misc_setup(pdata: *mut ApdPrivateData) -> i32 {
    let adev = (*pdata).adev;
    let mut obj: *const AcpiObject = core::ptr::null();
    let mut clkdev: *mut PlatformDevice;
    let mut clk_data: *mut FchClkData;
    let mut rentry: *mut ResourceEntry;
    let mut resource_list: ListHead = core::mem::zeroed();
    let mut ret: i32;

    clk_data = devm_kzalloc(&mut (*adev).dev, core::mem::size_of::<FchClkData>(), GFP_KERNEL);
    if clk_data.is_null() {
        return -ENOMEM;
    }

    INIT_LIST_HEAD(&mut resource_list);
    ret = acpi_dev_get_memory_resources(adev, &mut resource_list);
    if ret < 0 {
        return -ENOENT;
    }

    if acpi_dev_get_property(adev, c"clk-name".as_ptr(), ACPI_TYPE_STRING, &mut obj) == 0 {
        (*clk_data).name = devm_kzalloc(
            &mut (*adev).dev,
            (*obj).string.length as usize,
            GFP_KERNEL,
        ) as *mut i8;
        if (*clk_data).name.is_null() {
            return -ENOMEM;
        }

        strscpy((*clk_data).name, (*obj).string.pointer, (*obj).string.length as usize);
    } else {
        /* Set default name to mclk if entry missing in firmware */
        (*clk_data).name = c"mclk".as_ptr() as *mut i8;
    }

    list_for_each_entry!(rentry, &mut resource_list, node);
    {
        (*clk_data).base = devm_ioremap(
            &mut (*adev).dev,
            (*(*rentry).res).start,
            resource_size((*rentry).res),
        );
    }
    if (*clk_data).base.is_null() {
        return -ENOMEM;
    }

    acpi_dev_free_resource_list(&mut resource_list);

    clkdev = platform_device_register_data(
        &mut (*adev).dev,
        c"clk-fch".as_ptr(),
        PLATFORM_DEVID_NONE,
        clk_data as *const core::ffi::c_void,
        core::mem::size_of::<FchClkData>(),
    );
    PTR_ERR_OR_ZERO(clkdev)
}

#[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)]
static CZ_I2C_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 133 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)]
static WT_I2C_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 150 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)]
static WT_I3C_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 125 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };

#[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)]
static mut UART_PROPERTIES: [PropertyEntry; 4] = [
    PROPERTY_ENTRY_U32!(c"reg-io-width", 4),
    PROPERTY_ENTRY_U32!(c"reg-shift", 2),
    PROPERTY_ENTRY_BOOL!(c"snps,uart-16550-compatible"),
    PropertyEntry::default(),
];

#[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)]
static CZ_UART_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 48 * HZ_PER_MHZ, properties: unsafe { UART_PROPERTIES.as_mut_ptr() }, setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)]
static FCH_MISC_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 0, properties: core::ptr::null_mut(), setup: Some(fch_misc_setup) };

#[cfg(CONFIG_ARM64)]
static XGENE_I2C_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 100 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_ARM64)]
static VULCAN_SPI_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 133 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_ARM64)]
static HIP07_I2C_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 200 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_ARM64)]
static HIP08_I2C_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 250 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_ARM64)]
static HIP08_LITE_I2C_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 125 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_ARM64)]
static THUNDERX2_I2C_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 125 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_ARM64)]
static NXP_I2C_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 350 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_ARM64)]
static HIP08_SPI_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 250 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_ARM64)]
static LECA_SPI_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 400 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_ARM64)]
static LECA_I2C_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 250 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };
#[cfg(CONFIG_ARM64)]
static HJMC_I2C_DESC: ApdDeviceDesc = ApdDeviceDesc { fixed_clk_rate: 200 * HZ_PER_MHZ, properties: core::ptr::null_mut(), setup: Some(acpi_apd_setup) };

unsafe extern "C" fn acpi_apd_create_device(adev: *mut AcpiDevice, id: *const AcpiDeviceId) -> i32 {
    let dev_desc = (*id).driver_data as *const ApdDeviceDesc;
    let pdata: *mut ApdPrivateData;
    let pdev: *mut PlatformDevice;
    let mut ret: i32;

    if dev_desc.is_null() {
        pdev = acpi_create_platform_device(adev, core::ptr::null_mut());
        return if IS_ERR_OR_NULL(pdev) { PTR_ERR(pdev) } else { 1 };
    }

    pdata = kzalloc(core::mem::size_of::<ApdPrivateData>(), GFP_KERNEL) as *mut ApdPrivateData;
    if pdata.is_null() {
        return -ENOMEM;
    }

    (*pdata).adev = adev;
    (*pdata).dev_desc = dev_desc;

    if let Some(setup) = (*dev_desc).setup {
        ret = setup(pdata);
        if ret != 0 {
            kfree(pdata as *mut core::ffi::c_void);
            return ret;
        }
    }

    (*adev).driver_data = pdata as *mut core::ffi::c_void;
    pdev = acpi_create_platform_device(adev, (*dev_desc).properties);
    if !IS_ERR_OR_NULL(pdev) {
        return 1;
    }

    ret = PTR_ERR(pdev);
    (*adev).driver_data = core::ptr::null_mut();
    kfree(pdata as *mut core::ffi::c_void);
    ret
}

static ACPI_APD_DEVICE_IDS: [AcpiDeviceId; 23] = [
    #[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)] AcpiDeviceId { id: *c"AMD0010", driver_data: apd_addr(&CZ_I2C_DESC) },
    #[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)] AcpiDeviceId { id: *c"AMD0020", driver_data: apd_addr(&CZ_UART_DESC) },
    #[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)] AcpiDeviceId { id: *c"AMD0030", driver_data: 0 },
    #[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)] AcpiDeviceId { id: *c"AMD0040", driver_data: apd_addr(&FCH_MISC_DESC) },
    #[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)] AcpiDeviceId { id: *c"AMDI0010", driver_data: apd_addr(&WT_I2C_DESC) },
    #[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)] AcpiDeviceId { id: *c"AMDI0015", driver_data: apd_addr(&WT_I3C_DESC) },
    #[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)] AcpiDeviceId { id: *c"AMDI0019", driver_data: apd_addr(&WT_I2C_DESC) },
    #[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)] AcpiDeviceId { id: *c"AMDI0020", driver_data: apd_addr(&CZ_UART_DESC) },
    #[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)] AcpiDeviceId { id: *c"AMDI0022", driver_data: apd_addr(&CZ_UART_DESC) },
    #[cfg(CONFIG_X86_AMD_PLATFORM_DEVICE)] AcpiDeviceId { id: *c"HYGO0010", driver_data: apd_addr(&WT_I2C_DESC) },
    #[cfg(CONFIG_ARM64)] AcpiDeviceId { id: *c"APMC0D0F", driver_data: apd_addr(&XGENE_I2C_DESC) },
    #[cfg(CONFIG_ARM64)] AcpiDeviceId { id: *c"BRCM900D", driver_data: apd_addr(&VULCAN_SPI_DESC) },
    #[cfg(CONFIG_ARM64)] AcpiDeviceId { id: *c"CAV900D", driver_data: apd_addr(&VULCAN_SPI_DESC) },
    #[cfg(CONFIG_ARM64)] AcpiDeviceId { id: *c"CAV9007", driver_data: apd_addr(&THUNDERX2_I2C_DESC) },
    #[cfg(CONFIG_ARM64)] AcpiDeviceId { id: *c"HISI02A1", driver_data: apd_addr(&HIP07_I2C_DESC) },
    #[cfg(CONFIG_ARM64)] AcpiDeviceId { id: *c"HISI02A2", driver_data: apd_addr(&HIP08_I2C_DESC) },
    #[cfg(CONFIG_ARM64)] AcpiDeviceId { id: *c"HISI02A3", driver_data: apd_addr(&HIP08_LITE_I2C_DESC) },
    #[cfg(CONFIG_ARM64)] AcpiDeviceId { id: *c"HISI0173", driver_data: apd_addr(&HIP08_SPI_DESC) },
    #[cfg(CONFIG_ARM64)] AcpiDeviceId { id: *c"HJMC3001", driver_data: apd_addr(&HJMC_I2C_DESC) },
    #[cfg(CONFIG_ARM64)] AcpiDeviceId { id: *c"LECA0002", driver_data: apd_addr(&LECA_SPI_DESC) },
    #[cfg(CONFIG_ARM64)] AcpiDeviceId { id: *c"LECA0003", driver_data: apd_addr(&LECA_I2C_DESC) },
    #[cfg(CONFIG_ARM64)] AcpiDeviceId { id: *c"NXP0001", driver_data: apd_addr(&NXP_I2C_DESC) },
    AcpiDeviceId::default(),
];

static mut APD_HANDLER: AcpiScanHandler = AcpiScanHandler {
    ids: ACPI_APD_DEVICE_IDS.as_ptr(),
    attach: Some(acpi_apd_create_device),
};

pub unsafe extern "C" fn acpi_apd_init() {
    acpi_scan_add_handler(&mut APD_HANDLER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
