// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007 Lemote, Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Zhangjin Wu, wuzhangjin@gmail.com
 */

// Kernel and architecture declarations are supplied by the surrounding tree.

const NOTIFY_DONE: i32 = 0;

unsafe fn firmware_restart(_unusedd: *mut sys_off_data) -> i32 {
    let fw_restart: unsafe extern "C" fn() = core::mem::transmute(loongson_sysconf.restart_addr);
    fw_restart();
    NOTIFY_DONE
}

unsafe fn firmware_poweroff(_unused: *mut sys_off_data) -> i32 {
    let fw_poweroff: unsafe extern "C" fn() = core::mem::transmute(loongson_sysconf.poweroff_addr);
    fw_poweroff();
    NOTIFY_DONE
}

// CONFIG_KEXEC_CORE
// 0X80000000~0X80200000 is safe
const MAX_ARGS: usize = 64;
const KEXEC_CTRL_CODE: usize = 0xFFFFFFFF80100000;
const KEXEC_ARGV_ADDR: usize = 0xFFFFFFFF80108000;
const KEXEC_ARGV_SIZE: usize = COMMAND_LINE_SIZE;
const KEXEC_ENVP_SIZE: usize = 4800;

static mut kexec_argc: i32 = 0;
static mut kdump_argc: i32 = 0;
static mut kexec_argv: *mut core::ffi::c_void = core::ptr::null_mut();
static mut kdump_argv: *mut core::ffi::c_void = core::ptr::null_mut();
static mut kexec_envp: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn loongson_kexec_prepare(image: *mut kimage) -> i32 {
    let mut argc: usize = 0;
    let argv: *mut u32;
    let mut str_: *mut u8;
    let mut ptr: *mut u8;
    let bootloader = b"kexec\0";

    if (*image).type_ == KEXEC_TYPE_DEFAULT {
        argv = kexec_argv as *mut u32;
    } else {
        argv = kdump_argv as *mut u32;
    }

    *argv.add(argc) = (KEXEC_ARGV_ADDR + KEXEC_ARGV_SIZE / 2) as u32;
    argc += 1;

    for i in 0..(*image).nr_segments {
        let segment = (*image).segment.add(i);
        if strncmp(bootloader.as_ptr(), (*segment).buf as *const u8, strlen(bootloader.as_ptr())) == 0 {
            let mut offt: i32;
            str_ = argv as *mut u8;
            str_ = str_.add(KEXEC_ARGV_SIZE / 2);
            memcpy(str_, (*segment).buf, KEXEC_ARGV_SIZE / 2);
            ptr = strchr(str_, b' ' as i32);

            while !ptr.is_null() && argc < MAX_ARGS {
                *ptr = 0;
                if *ptr.add(1) != b' ' {
                    offt = (ptr.offset_from(str_) + 1) as i32;
                    *argv.add(argc) = (KEXEC_ARGV_ADDR + KEXEC_ARGV_SIZE / 2 + offt as usize) as u32;
                    argc += 1;
                }
                ptr = strchr(ptr.add(1), b' ' as i32);
            }
            break;
        }
    }

    if (*image).type_ == KEXEC_TYPE_DEFAULT { kexec_argc = argc as i32; } else { kdump_argc = argc as i32; }
    (*image).control_code_page = virt_to_page(KEXEC_CTRL_CODE as *mut core::ffi::c_void);
    0
}

unsafe fn loongson_kexec_shutdown() {
    // CONFIG_SMP: bring all possible CPUs online and set secondary_kexec_args[0].
    kexec_args[0] = kexec_argc;
    kexec_args[1] = fw_arg1;
    kexec_args[2] = fw_arg2;
    memcpy(fw_arg1 as *mut core::ffi::c_void, kexec_argv, KEXEC_ARGV_SIZE);
    memcpy(fw_arg2 as *mut core::ffi::c_void, kexec_envp, KEXEC_ENVP_SIZE);
}

unsafe fn loongson_crash_shutdown(regs: *mut pt_regs) {
    default_machine_crash_shutdown(regs);
    kexec_args[0] = kdump_argc;
    kexec_args[1] = fw_arg1;
    kexec_args[2] = fw_arg2;
    memcpy(fw_arg1 as *mut core::ffi::c_void, kdump_argv, KEXEC_ARGV_SIZE);
    memcpy(fw_arg2 as *mut core::ffi::c_void, kexec_envp, KEXEC_ENVP_SIZE);
}

unsafe fn mips_reboot_setup() -> i32 {
    if loongson_sysconf.restart_addr != 0 {
        register_sys_off_handler(SYS_OFF_MODE_RESTART, SYS_OFF_PRIO_FIRMWARE, firmware_restart, core::ptr::null_mut());
    }
    if loongson_sysconf.poweroff_addr != 0 {
        register_sys_off_handler(SYS_OFF_MODE_POWER_OFF, SYS_OFF_PRIO_FIRMWARE, firmware_poweroff, core::ptr::null_mut());
    }

    kexec_argv = kmalloc(KEXEC_ARGV_SIZE, GFP_KERNEL);
    if WARN_ON(kexec_argv.is_null()) { return -12; }
    kdump_argv = kmalloc(KEXEC_ARGV_SIZE, GFP_KERNEL);
    if WARN_ON(kdump_argv.is_null()) { return -12; }
    kexec_envp = kmalloc(KEXEC_ENVP_SIZE, GFP_KERNEL);
    if WARN_ON(kexec_envp.is_null()) { return -12; }

    fw_arg1 = KEXEC_ARGV_ADDR;
    memcpy(kexec_envp, fw_arg2 as *mut core::ffi::c_void, KEXEC_ENVP_SIZE);
    _machine_kexec_prepare = Some(loongson_kexec_prepare);
    _machine_kexec_shutdown = Some(loongson_kexec_shutdown);
    _machine_crash_shutdown = Some(loongson_crash_shutdown);
    0
}

// arch_initcall(mips_reboot_setup)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
