// SPDX-License-Identifier: GPL-2.0
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// C dependencies supplied by the surrounding kernel translation unit.

pub static mut efi_system_table: u64 = 0;
pub static mut loongson_sysconf: loongson_system_configuration = loongson_system_configuration {
    cpuname: core::ptr::null_mut(),
};

pub unsafe fn init_environ() {
    let efi_boot: i32 = fw_arg0;
    let cmdline: *mut core::ffi::c_char =
        early_memremap_ro(fw_arg1, COMMAND_LINE_SIZE) as *mut core::ffi::c_char;

    if efi_boot != 0 {
        set_bit(EFI_BOOT, &raw mut efi.flags);
    } else {
        clear_bit(EFI_BOOT, &raw mut efi.flags);
    }

    strscpy(boot_command_line, cmdline, COMMAND_LINE_SIZE);
    strscpy(init_command_line, cmdline, COMMAND_LINE_SIZE);
    early_memunmap(cmdline as *mut core::ffi::c_void, COMMAND_LINE_SIZE);

    efi_system_table = fw_arg2;
}

unsafe fn init_cpu_fullname() -> i32 {
    let mut cpu: i32;
    let ret: i32;
    let mut cpuname: *mut core::ffi::c_char;
    let mut model: *const core::ffi::c_char = core::ptr::null();

    /* Parsing cpuname from DTS model property */
    ret = of_property_read_string(of_root, b"model\0".as_ptr() as *const core::ffi::c_char, &mut model);
    if ret == 0 {
        cpuname = kstrdup(model, GFP_KERNEL);
        if cpuname.is_null() {
            return -ENOMEM;
        }
        loongson_sysconf.cpuname = strsep(&mut cpuname, b" \0".as_ptr() as *const core::ffi::c_char);
    }

    if !loongson_sysconf.cpuname.is_null()
        && strncmp(
            loongson_sysconf.cpuname,
            b"Loongson\0".as_ptr() as *const core::ffi::c_char,
            8,
        ) == 0
    {
        cpu = 0;
        while cpu < NR_CPUS {
            __cpu_full_name[cpu as usize] = loongson_sysconf.cpuname;
            cpu += 1;
        }
    }
    0
}

unsafe fn fdt_cpu_clk_init() -> i32 {
    let mut clk: *mut clk;
    let np: *mut device_node;

    np = of_get_cpu_node(0, core::ptr::null_mut());
    if np.is_null() {
        return -ENODEV;
    }

    clk = of_clk_get(np, 0);
    of_node_put(np);
    cpu_clock_freq = 200 * 1000 * 1000;

    if IS_ERR(clk) {
        pr_warn!("No valid CPU clock freq, assume 200MHz.\n");
        return -ENODEV;
    }

    cpu_clock_freq = clk_get_rate(clk);
    clk_put(clk);

    0
}

unsafe fn boardinfo_show(
    kobj: *mut kobject,
    attr: *mut kobj_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    sysfs_emit(
        buf,
        b"BIOS Information\nVendor\t\t\t: %s\nVersion\t\t\t: %s\nROM Size\t\t: %d KB\nRelease Date\t\t: %s\n\nBoard Information\nManufacturer\t\t: %s\nBoard Name\t\t: %s\nFamily\t\t\t: LOONGSON64\n\n\0".as_ptr()
            as *const core::ffi::c_char,
        b_info.bios_vendor,
        b_info.bios_version,
        b_info.bios_size,
        b_info.bios_release_date,
        b_info.board_vendor,
        b_info.board_name,
    )
}

static mut boardinfo_attr: kobj_attribute = __ATTR!(boardinfo, 0o444, boardinfo_show, None);

unsafe fn boardinfo_init() -> i32 {
    let loongson_kobj: *mut kobject;

    loongson_kobj = kobject_create_and_add(b"loongson\0".as_ptr() as *const core::ffi::c_char, firmware_kobj);
    if loongson_kobj.is_null() {
        return -ENOMEM;
    }

    sysfs_create_file(loongson_kobj, &raw mut boardinfo_attr.attr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
