// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2012 Freescale Semiconductor, Inc.
// Copyright 2012 Linaro Ltd.
// Copyright 2009 Pengutronix, Sascha Hauer <s.hauer@pengutronix.de>
//
// Initial development of this code was funded by
// Phytec Messtechnik GmbH, https://www.phytec.de

// C dependencies:
// linux/clk.h, linux/debugfs.h, linux/err.h, linux/io.h, linux/module.h,
// linux/of.h, linux/platform_device.h, linux/slab.h, "imx-audmux.h"

const DRIVER_NAME: &[u8] = b"imx-audmux\0";

static mut audmux_clk: *mut clk = core::ptr::null_mut();
static mut audmux_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut regcache: *mut u32 = core::ptr::null_mut();
static mut reg_max: u32 = 0;

const fn IMX_AUDMUX_V2_PTCR(x: u32) -> u32 {
    x.wrapping_mul(8)
}

const fn IMX_AUDMUX_V2_PDCR(x: u32) -> u32 {
    x.wrapping_mul(8).wrapping_add(4)
}

// Original C code is compiled only under CONFIG_DEBUG_FS.
#[cfg(CONFIG_DEBUG_FS)]
static mut audmux_debugfs_root: *mut dentry = core::ptr::null_mut();

// There is an annoying discontinuity in the SSI numbering with regard
// to the Linux number of the devices
#[cfg(CONFIG_DEBUG_FS)]
unsafe fn audmux_port_string(port: core::ffi::c_int) -> *const core::ffi::c_char {
    match port {
        MX31_AUDMUX_PORT1_SSI0 => b"imx-ssi.0\0".as_ptr() as *const core::ffi::c_char,
        MX31_AUDMUX_PORT2_SSI1 => b"imx-ssi.1\0".as_ptr() as *const core::ffi::c_char,
        MX31_AUDMUX_PORT3_SSI_PINS_3 => b"SSI3\0".as_ptr() as *const core::ffi::c_char,
        MX31_AUDMUX_PORT4_SSI_PINS_4 => b"SSI4\0".as_ptr() as *const core::ffi::c_char,
        MX31_AUDMUX_PORT5_SSI_PINS_5 => b"SSI5\0".as_ptr() as *const core::ffi::c_char,
        MX31_AUDMUX_PORT6_SSI_PINS_6 => b"SSI6\0".as_ptr() as *const core::ffi::c_char,
        _ => b"UNKNOWN\0".as_ptr() as *const core::ffi::c_char,
    }
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe extern "C" fn audmux_read_file(
    file: *mut file,
    user_buf: *mut core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> ssize_t {
    let mut ret: ssize_t;
    let buf: *mut core::ffi::c_char;
    let port: uintptr_t = (*file).private_data as uintptr_t;
    let pdcr: u32;
    let ptcr: u32;

    ret = clk_prepare_enable(audmux_clk) as ssize_t;
    if ret != 0 {
        return ret;
    }

    ptcr = readl(audmux_base.byte_add(IMX_AUDMUX_V2_PTCR(port as u32) as usize)) as u32;
    pdcr = readl(audmux_base.byte_add(IMX_AUDMUX_V2_PDCR(port as u32) as usize)) as u32;

    clk_disable_unprepare(audmux_clk);

    buf = kmalloc(PAGE_SIZE, GFP_KERNEL) as *mut core::ffi::c_char;
    if buf.is_null() {
        return -ENOMEM as ssize_t;
    }

    ret = sysfs_emit(
        buf,
        b"PDCR: %08x\nPTCR: %08x\n\0".as_ptr() as *const core::ffi::c_char,
        pdcr,
        ptcr,
    ) as ssize_t;

    if (ptcr & IMX_AUDMUX_V2_PTCR_TFSDIR) != 0 {
        ret += scnprintf(
            buf.offset(ret as isize),
            PAGE_SIZE - ret as usize,
            b"TxFS output from %s, \0".as_ptr() as *const core::ffi::c_char,
            audmux_port_string(((ptcr >> 27) & 0x7) as core::ffi::c_int),
        ) as ssize_t;
    } else {
        ret += scnprintf(
            buf.offset(ret as isize),
            PAGE_SIZE - ret as usize,
            b"TxFS input, \0".as_ptr() as *const core::ffi::c_char,
        ) as ssize_t;
    }

    if (ptcr & IMX_AUDMUX_V2_PTCR_TCLKDIR) != 0 {
        ret += scnprintf(
            buf.offset(ret as isize),
            PAGE_SIZE - ret as usize,
            b"TxClk output from %s\0".as_ptr() as *const core::ffi::c_char,
            audmux_port_string(((ptcr >> 22) & 0x7) as core::ffi::c_int),
        ) as ssize_t;
    } else {
        ret += scnprintf(
            buf.offset(ret as isize),
            PAGE_SIZE - ret as usize,
            b"TxClk input\0".as_ptr() as *const core::ffi::c_char,
        ) as ssize_t;
    }

    ret += scnprintf(
        buf.offset(ret as isize),
        PAGE_SIZE - ret as usize,
        b"\n\0".as_ptr() as *const core::ffi::c_char,
    ) as ssize_t;

    if (ptcr & IMX_AUDMUX_V2_PTCR_SYN) != 0 {
        ret += scnprintf(
            buf.offset(ret as isize),
            PAGE_SIZE - ret as usize,
            b"Port is symmetric\0".as_ptr() as *const core::ffi::c_char,
        ) as ssize_t;
    } else {
        if (ptcr & IMX_AUDMUX_V2_PTCR_RFSDIR) != 0 {
            ret += scnprintf(
                buf.offset(ret as isize),
                PAGE_SIZE - ret as usize,
                b"RxFS output from %s, \0".as_ptr() as *const core::ffi::c_char,
                audmux_port_string(((ptcr >> 17) & 0x7) as core::ffi::c_int),
            ) as ssize_t;
        } else {
            ret += scnprintf(
                buf.offset(ret as isize),
                PAGE_SIZE - ret as usize,
                b"RxFS input, \0".as_ptr() as *const core::ffi::c_char,
            ) as ssize_t;
        }

        if (ptcr & IMX_AUDMUX_V2_PTCR_RCLKDIR) != 0 {
            ret += scnprintf(
                buf.offset(ret as isize),
                PAGE_SIZE - ret as usize,
                b"RxClk output from %s\0".as_ptr() as *const core::ffi::c_char,
                audmux_port_string(((ptcr >> 12) & 0x7) as core::ffi::c_int),
            ) as ssize_t;
        } else {
            ret += scnprintf(
                buf.offset(ret as isize),
                PAGE_SIZE - ret as usize,
                b"RxClk input\0".as_ptr() as *const core::ffi::c_char,
            ) as ssize_t;
        }
    }

    ret += scnprintf(
        buf.offset(ret as isize),
        PAGE_SIZE - ret as usize,
        b"\nData received from %s\n\0".as_ptr() as *const core::ffi::c_char,
        audmux_port_string(((pdcr >> 13) & 0x7) as core::ffi::c_int),
    ) as ssize_t;

    ret = simple_read_from_buffer(user_buf, count, ppos, buf as *const core::ffi::c_void, ret as usize);

    kfree(buf as *const core::ffi::c_void);

    ret
}

#[cfg(CONFIG_DEBUG_FS)]
static audmux_debugfs_fops: file_operations = file_operations {
    open: Some(simple_open),
    read: Some(audmux_read_file),
    llseek: Some(default_llseek),
};

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn audmux_debugfs_init() {
    let mut i: uintptr_t;
    let mut buf: [core::ffi::c_char; 20] = [0; 20];

    audmux_debugfs_root = debugfs_create_dir(b"audmux\0".as_ptr() as *const core::ffi::c_char, core::ptr::null_mut());

    i = 0;
    while i < (MX31_AUDMUX_PORT7_SSI_PINS_7 + 1) as uintptr_t {
        snprintf(
            buf.as_mut_ptr(),
            core::mem::size_of_val(&buf),
            b"ssi%lu\0".as_ptr() as *const core::ffi::c_char,
            i,
        );
        debugfs_create_file(
            buf.as_ptr(),
            0o444,
            audmux_debugfs_root,
            i as *mut core::ffi::c_void,
            &audmux_debugfs_fops,
        );
        i += 1;
    }
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn audmux_debugfs_remove() {
    debugfs_remove_recursive(audmux_debugfs_root);
}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
unsafe fn audmux_debugfs_init() {}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline]
unsafe fn audmux_debugfs_remove() {}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum imx_audmux_type {
    IMX21_AUDMUX,
    IMX31_AUDMUX,
}

static mut audmux_type: imx_audmux_type = imx_audmux_type::IMX21_AUDMUX;

static imx_audmux_dt_ids: [of_device_id; 3] = [
    of_device_id {
        compatible: b"fsl,imx21-audmux\0".as_ptr() as *const core::ffi::c_char,
        data: imx_audmux_type::IMX21_AUDMUX as uintptr_t as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: b"fsl,imx31-audmux\0".as_ptr() as *const core::ffi::c_char,
        data: imx_audmux_type::IMX31_AUDMUX as uintptr_t as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, imx_audmux_dt_ids);

static port_mapping: [u8; 6] = [0x0, 0x4, 0x8, 0x10, 0x14, 0x1c];

#[no_mangle]
pub unsafe extern "C" fn imx_audmux_v1_configure_port(port: core::ffi::c_uint, pcr: core::ffi::c_uint) -> core::ffi::c_int {
    if audmux_type != imx_audmux_type::IMX21_AUDMUX {
        return -EINVAL;
    }

    if audmux_base.is_null() {
        return -ENOSYS;
    }

    if port as usize >= port_mapping.len() {
        return -EINVAL;
    }

    writel(pcr, audmux_base.byte_add(port_mapping[port as usize] as usize));

    0
}
// EXPORT_SYMBOL_GPL(imx_audmux_v1_configure_port);

#[no_mangle]
pub unsafe extern "C" fn imx_audmux_v2_configure_port(
    port: core::ffi::c_uint,
    ptcr: core::ffi::c_uint,
    pdcr: core::ffi::c_uint,
) -> core::ffi::c_int {
    let ret: core::ffi::c_int;

    if audmux_type != imx_audmux_type::IMX31_AUDMUX {
        return -EINVAL;
    }

    if audmux_base.is_null() {
        return -ENOSYS;
    }

    ret = clk_prepare_enable(audmux_clk);
    if ret != 0 {
        return ret;
    }

    writel(ptcr, audmux_base.byte_add(IMX_AUDMUX_V2_PTCR(port) as usize));
    writel(pdcr, audmux_base.byte_add(IMX_AUDMUX_V2_PDCR(port) as usize));

    clk_disable_unprepare(audmux_clk);

    0
}
// EXPORT_SYMBOL_GPL(imx_audmux_v2_configure_port);

unsafe fn imx_audmux_parse_dt_defaults(
    pdev: *mut platform_device,
    of_node: *mut device_node,
) -> core::ffi::c_int {
    let mut child: *mut device_node;

    child = core::ptr::null_mut();
    while {
        child = of_get_next_available_child(of_node, child);
        !child.is_null()
    } {
        let mut port: core::ffi::c_uint = 0;
        let mut ptcr: core::ffi::c_uint = 0;
        let mut pdcr: core::ffi::c_uint = 0;
        let mut pcr: core::ffi::c_uint = 0;
        let mut val: core::ffi::c_uint = 0;
        let mut ret: core::ffi::c_int;
        let mut i: core::ffi::c_int = 0;

        ret = of_property_read_u32(child, b"fsl,audmux-port\0".as_ptr() as *const core::ffi::c_char, &mut port);
        if ret != 0 {
            dev_warn(
                &mut (*pdev).dev,
                b"Failed to get fsl,audmux-port of child node \"%pOF\"\n\0".as_ptr() as *const core::ffi::c_char,
                child,
            );
            continue;
        }
        if !of_property_present(child, b"fsl,port-config\0".as_ptr() as *const core::ffi::c_char) {
            dev_warn(
                &mut (*pdev).dev,
                b"child node \"%pOF\" does not have property fsl,port-config\n\0".as_ptr() as *const core::ffi::c_char,
                child,
            );
            continue;
        }

        loop {
            ret = of_property_read_u32_index(
                child,
                b"fsl,port-config\0".as_ptr() as *const core::ffi::c_char,
                i as u32,
                &mut val,
            );
            if ret != 0 {
                break;
            }

            if audmux_type == imx_audmux_type::IMX31_AUDMUX {
                if i % 2 != 0 {
                    pdcr |= val;
                } else {
                    ptcr |= val;
                }
            } else {
                pcr |= val;
            }
            i += 1;
        }

        if ret != -EOVERFLOW {
            dev_err(
                &mut (*pdev).dev,
                b"Failed to read u32 at index %d of child %pOF\n\0".as_ptr() as *const core::ffi::c_char,
                i,
                child,
            );
            continue;
        }

        if audmux_type == imx_audmux_type::IMX31_AUDMUX {
            if i % 2 != 0 {
                dev_err(
                    &mut (*pdev).dev,
                    b"One pdcr value is missing in child node %pOF\n\0".as_ptr() as *const core::ffi::c_char,
                    child,
                );
                continue;
            }
            imx_audmux_v2_configure_port(port, ptcr, pdcr);
        } else {
            imx_audmux_v1_configure_port(port, pcr);
        }
    }

    0
}

unsafe extern "C" fn imx_audmux_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    audmux_base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(audmux_base) {
        return PTR_ERR(audmux_base) as core::ffi::c_int;
    }

    audmux_clk = devm_clk_get(&mut (*pdev).dev, b"audmux\0".as_ptr() as *const core::ffi::c_char);
    if IS_ERR(audmux_clk as *const core::ffi::c_void) {
        dev_dbg(
            &mut (*pdev).dev,
            b"cannot get clock: %ld\n\0".as_ptr() as *const core::ffi::c_char,
            PTR_ERR(audmux_clk as *const core::ffi::c_void),
        );
        audmux_clk = core::ptr::null_mut();
    }

    audmux_type = core::mem::transmute::<uintptr_t, imx_audmux_type>(
        of_device_get_match_data(&mut (*pdev).dev) as uintptr_t,
    );

    match audmux_type {
        imx_audmux_type::IMX31_AUDMUX => {
            audmux_debugfs_init();
            reg_max = 14;
        }
        imx_audmux_type::IMX21_AUDMUX => {
            reg_max = 6;
        }
        _ => {
            dev_err(
                &mut (*pdev).dev,
                b"unsupported version!\n\0".as_ptr() as *const core::ffi::c_char,
            );
            return -EINVAL;
        }
    }

    regcache = devm_kcalloc(
        &mut (*pdev).dev,
        reg_max as usize,
        core::mem::size_of::<u32>(),
        GFP_KERNEL,
    ) as *mut u32;
    if regcache.is_null() {
        return -ENOMEM;
    }

    imx_audmux_parse_dt_defaults(pdev, (*pdev).dev.of_node);

    0
}

unsafe extern "C" fn imx_audmux_remove(_pdev: *mut platform_device) {
    if audmux_type == imx_audmux_type::IMX31_AUDMUX {
        audmux_debugfs_remove();
    }
}

unsafe extern "C" fn imx_audmux_suspend(_dev: *mut device) -> core::ffi::c_int {
    let mut i: core::ffi::c_int;

    clk_prepare_enable(audmux_clk);

    i = 0;
    while i < reg_max as core::ffi::c_int {
        *regcache.offset(i as isize) = readl(audmux_base.byte_add((i * 4) as usize)) as u32;
        i += 1;
    }

    clk_disable_unprepare(audmux_clk);

    0
}

unsafe extern "C" fn imx_audmux_resume(_dev: *mut device) -> core::ffi::c_int {
    let mut i: core::ffi::c_int;

    clk_prepare_enable(audmux_clk);

    i = 0;
    while i < reg_max as core::ffi::c_int {
        writel(*regcache.offset(i as isize), audmux_base.byte_add((i * 4) as usize));
        i += 1;
    }

    clk_disable_unprepare(audmux_clk);

    0
}

static imx_audmux_pm: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(imx_audmux_suspend, imx_audmux_resume)
    suspend: Some(imx_audmux_suspend),
    resume: Some(imx_audmux_resume),
};

static mut imx_audmux_driver: platform_driver = platform_driver {
    probe: Some(imx_audmux_probe),
    remove: Some(imx_audmux_remove),
    driver: device_driver {
        name: DRIVER_NAME.as_ptr() as *const core::ffi::c_char,
        pm: pm_sleep_ptr(&imx_audmux_pm),
        of_match_table: imx_audmux_dt_ids.as_ptr(),
    },
};

unsafe extern "C" fn imx_audmux_init() -> core::ffi::c_int {
    platform_driver_register(&mut imx_audmux_driver)
}
// subsys_initcall(imx_audmux_init);

unsafe extern "C" fn imx_audmux_exit() {
    platform_driver_unregister(&mut imx_audmux_driver);
}
// module_exit(imx_audmux_exit);

// MODULE_DESCRIPTION("Freescale i.MX AUDMUX driver");
// MODULE_AUTHOR("Sascha Hauer <s.hauer@pengutronix.de>");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRIVER_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
