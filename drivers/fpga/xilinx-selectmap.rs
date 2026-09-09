// SPDX-License-Identifier: GPL-2.0-only
/*
 * Xilinx Spartan6 and 7 Series SelectMAP interface driver
 *
 * (C) 2024 Charles Perry <charles.perry@savoirfairelinux.com>
 *
 * Manage Xilinx FPGA firmware loaded over the SelectMAP configuration
 * interface.
 */

// Dependencies supplied by xilinx-core.h and the Linux kernel headers remain
// external to this translation unit.

#[repr(C)]
pub struct xilinx_selectmap_conf {
    pub core: xilinx_fpga_core,
    pub base: *mut core::ffi::c_void,
}

// External types and functions supplied by the included kernel interfaces.
#[repr(C)]
pub struct xilinx_fpga_core {
    pub dev: *mut device,
    pub write: Option<unsafe extern "C" fn(*mut xilinx_fpga_core, *const core::ffi::c_char, usize) -> i32>,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    _private: [u8; 0],
}

unsafe extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
        res: *mut *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn is_err(ptr: *const core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *const core::ffi::c_void) -> i64;
    fn dev_err_probe(
        dev: *mut device,
        err: i32,
        fmt: *const core::ffi::c_char,
        ...,
    ) -> i32;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const core::ffi::c_char,
        flags: u32,
    ) -> *mut gpio_desc;
    fn writeb(value: u8, addr: *mut core::ffi::c_void);
    fn xilinx_core_probe(core: *mut xilinx_fpga_core) -> i32;
}

const GFP_KERNEL: u32 = 0;
const GPIOD_OUT_HIGH: u32 = 0;
const ENOMEM: i32 = 12;

#[inline]
unsafe fn to_xilinx_selectmap_conf(obj: *mut xilinx_fpga_core) -> *mut xilinx_selectmap_conf {
    obj as *mut xilinx_selectmap_conf
}

unsafe extern "C" fn xilinx_selectmap_write(
    core: *mut xilinx_fpga_core,
    buf: *const core::ffi::c_char,
    count: usize,
) -> i32 {
    let conf = to_xilinx_selectmap_conf(core);

    for i in 0..count {
        writeb(*buf.add(i) as u8, (*conf).base);
    }

    0
}

unsafe extern "C" fn xilinx_selectmap_probe(pdev: *mut platform_device) -> i32 {
    let conf = devm_kzalloc(
        // The platform device begins with its embedded device in the kernel
        // representation; this access is supplied by the external ABI.
        pdev as *mut device,
        core::mem::size_of::<xilinx_selectmap_conf>(),
        GFP_KERNEL,
    ) as *mut xilinx_selectmap_conf;
    if conf.is_null() {
        return -ENOMEM;
    }

    (*conf).core.dev = pdev as *mut device;
    (*conf).core.write = Some(xilinx_selectmap_write);

    let base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if is_err(base) {
        return dev_err_probe(
            (*conf).core.dev,
            ptr_err(base) as i32,
            b"ioremap error\0".as_ptr() as *const core::ffi::c_char,
        );
    }
    (*conf).base = base;

    /* CSI_B is active low */
    let gpio = devm_gpiod_get_optional(
        (*conf).core.dev,
        b"csi\0".as_ptr() as *const core::ffi::c_char,
        GPIOD_OUT_HIGH,
    );
    if is_err(gpio as *mut core::ffi::c_void) {
        return dev_err_probe(
            (*conf).core.dev,
            ptr_err(gpio as *mut core::ffi::c_void) as i32,
            b"Failed to get CSI_B gpio\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    /* RDWR_B is active low */
    let gpio = devm_gpiod_get_optional(
        (*conf).core.dev,
        b"rdwr\0".as_ptr() as *const core::ffi::c_char,
        GPIOD_OUT_HIGH,
    );
    if is_err(gpio as *mut core::ffi::c_void) {
        return dev_err_probe(
            (*conf).core.dev,
            ptr_err(gpio as *mut core::ffi::c_void) as i32,
            b"Failed to get RDWR_B gpio\0".as_ptr() as *const core::ffi::c_char,
        );
    }

    xilinx_core_probe(&mut (*conf).core)
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

#[repr(C)]
pub struct driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

static xlnx_selectmap_of_match: [of_device_id; 5] = [
    of_device_id { compatible: b"xlnx,fpga-xc7s-selectmap\0".as_ptr() as *const _, _private: [] }, // Spartan-7
    of_device_id { compatible: b"xlnx,fpga-xc7a-selectmap\0".as_ptr() as *const _, _private: [] }, // Artix-7
    of_device_id { compatible: b"xlnx,fpga-xc7k-selectmap\0".as_ptr() as *const _, _private: [] }, // Kintex-7
    of_device_id { compatible: b"xlnx,fpga-xc7v-selectmap\0".as_ptr() as *const _, _private: [] }, // Virtex-7
    of_device_id { compatible: core::ptr::null(), _private: [] },
];

#[no_mangle]
pub static mut xilinx_selectmap_driver: platform_driver = platform_driver {
    driver: driver {
        name: b"xilinx-selectmap\0".as_ptr() as *const _,
        of_match_table: xlnx_selectmap_of_match.as_ptr(),
    },
    probe: Some(xilinx_selectmap_probe),
};

// module_platform_driver(xilinx_selectmap_driver);
// MODULE_DEVICE_TABLE(of, xlnx_selectmap_of_match);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Charles Perry <charles.perry@savoirfairelinux.com>");
// MODULE_DESCRIPTION("Load Xilinx FPGA firmware over SelectMap");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
