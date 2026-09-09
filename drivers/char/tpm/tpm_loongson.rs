// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Loongson Technology Corporation Limited. */

// Dependencies supplied by the kernel and by tpm.h are intentionally left as
// external Rust items.

use core::ffi::c_void;

#[repr(C)]
pub struct tpm_loongson_cmd {
    pub cmd_id: u32,
    pub data_off: u32,
    pub data_len: u32,
    pub pad: [u32; 5],
}

#[repr(C)]
pub struct loongson_se_engine {
    pub command_ret: *mut tpm_loongson_cmd,
    pub data_buffer: *mut u8,
    pub command: *mut tpm_loongson_cmd,
    pub buffer_size: usize,
    pub buffer_off: u32,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct tpm_chip {
    pub dev: device,
    pub flags: u32,
}

#[repr(C)]
pub struct tpm_class_ops {
    pub flags: u32,
    pub recv: Option<unsafe extern "C" fn(*mut tpm_chip, *mut u8, usize) -> i32>,
    pub send: Option<unsafe extern "C" fn(*mut tpm_chip, *mut u8, usize, usize) -> i32>,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const u8,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: platform_driver_driver,
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn loongson_se_init_engine(parent: *mut device, engine: u32) -> *mut loongson_se_engine;
    fn loongson_se_send_engine_cmd(engine: *mut loongson_se_engine) -> i32;
    fn tpmm_chip_alloc(dev: *mut device, ops: *const tpm_class_ops) -> *mut tpm_chip;
    fn tpm_chip_register(chip: *mut tpm_chip) -> i32;
    fn ptr_err(ptr: *mut tpm_chip) -> i32;
}

extern "C" {
    static TPM_OPS_AUTO_STARTUP: u32;
    static TPM_CHIP_FLAG_TPM2: u32;
    static TPM_CHIP_FLAG_IRQ: u32;
    static SE_ENGINE_TPM: u32;
    static SE_CMD_TPM: u32;
}

const EIO: i32 = 5;
const E2BIG: i32 = 7;
const ENODEV: i32 = 19;

unsafe extern "C" fn tpm_loongson_recv(
    chip: *mut tpm_chip,
    buf: *mut u8,
    count: usize,
) -> i32 {
    let tpm_engine = dev_get_drvdata(&mut (*chip).dev as *mut device)
        as *mut loongson_se_engine;
    let cmd_ret = (*tpm_engine).command_ret;

    if (*cmd_ret).data_len as usize > count {
        return -EIO;
    }

    core::ptr::copy_nonoverlapping(
        (*tpm_engine).data_buffer,
        buf,
        (*cmd_ret).data_len as usize,
    );

    (*cmd_ret).data_len as i32
}

unsafe extern "C" fn tpm_loongson_send(
    chip: *mut tpm_chip,
    buf: *mut u8,
    _bufsiz: usize,
    count: usize,
) -> i32 {
    let tpm_engine = dev_get_drvdata(&mut (*chip).dev as *mut device)
        as *mut loongson_se_engine;
    let cmd = (*tpm_engine).command;

    if count > (*tpm_engine).buffer_size {
        return -E2BIG;
    }

    (*cmd).data_len = count as u32;
    core::ptr::copy_nonoverlapping(buf, (*tpm_engine).data_buffer, count);

    loongson_se_send_engine_cmd(tpm_engine)
}

static TPM_LOONGSON_OPS: tpm_class_ops = tpm_class_ops {
    flags: unsafe { TPM_OPS_AUTO_STARTUP },
    recv: Some(tpm_loongson_recv),
    send: Some(tpm_loongson_send),
};

unsafe extern "C" fn tpm_loongson_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let tpm_engine = loongson_se_init_engine(dev, unsafe { SE_ENGINE_TPM });
    if tpm_engine.is_null() {
        return -ENODEV;
    }

    let cmd = (*tpm_engine).command;
    (*cmd).cmd_id = unsafe { SE_CMD_TPM };
    (*cmd).data_off = (*tpm_engine).buffer_off;

    let chip = tpmm_chip_alloc(dev, &TPM_LOONGSON_OPS);
    if chip.is_null() {
        return ptr_err(chip);
    }
    (*chip).flags = unsafe { TPM_CHIP_FLAG_TPM2 | TPM_CHIP_FLAG_IRQ };
    dev_set_drvdata(&mut (*chip).dev as *mut device, tpm_engine as *mut c_void);

    tpm_chip_register(chip)
}

static mut TPM_LOONGSON: platform_driver = platform_driver {
    probe: Some(tpm_loongson_probe),
    driver: platform_driver_driver {
        name: b"tpm_loongson\0".as_ptr(),
    },
};

// module_platform_driver(tpm_loongson);
// MODULE_ALIAS("platform:tpm_loongson");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Loongson TPM driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
