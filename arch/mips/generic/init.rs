// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

use core::ffi::{c_char, c_void};

// Kernel headers and build-time configuration are supplied by external dependencies.

#[repr(C)]
pub struct of_device_id {
    pub data: *const c_void,
}

#[repr(C)]
pub struct mips_machine {
    pub fdt: *const c_void,
    pub detect: Option<unsafe extern "C" fn() -> bool>,
    pub fixup_fdt: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> *const c_void>,
    pub measure_hpt_freq: Option<unsafe extern "C" fn() -> u64>,
}

#[repr(C)]
pub struct mips_fdt_fixup {
    pub apply: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub description: *const c_char,
}

extern "C" {
    static mut fw_arg0: i32;
    static mut fw_arg1: usize;
    static mut cpu_has_counter: bool;
    static mut mips_hpt_frequency: u64;

    fn get_fdt() -> *const c_void;
    fn fdt_check_header(fdt: *const c_void) -> i32;
    fn mips_machine_is_compatible(
        machine: *const mips_machine,
        fdt: *const c_void,
    ) -> *const of_device_id;
    fn fw_init_cmdline();
    fn __dt_setup_arch(fdt: *const c_void);
    fn unflatten_and_copy_device_tree();
    fn mips_cpc_probe();
    fn register_cps_smp_ops() -> bool;
    fn register_vsmp_smp_ops() -> bool;
    fn register_up_smp_ops();
    fn fdt_open_into(fdt_in: *const c_void, fdt_out: *mut c_void, size: usize) -> i32;
    fn fdt_pack(fdt: *mut c_void) -> i32;
    fn of_clk_init(data: *const c_void);
    fn of_get_cpu_node(cpu: i32, thread: *const c_void) -> *mut device_node;
    fn of_clk_get(node: *mut device_node, index: i32) -> *mut clk;
    fn clk_get_rate(clock: *mut clk) -> u64;
    fn clk_put(clock: *mut clk);
    fn ptr_err(ptr: *mut clk) -> isize;
    fn boot_cpu_type() -> i32;
    fn timer_probe();
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn mips_cpu_irq_init();
    fn irqchip_init();
    fn pr_err(format: *const c_char, ...);
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

static mut FDT: *const c_void = core::ptr::null();
static mut MACH: *const mips_machine = core::ptr::null();
static mut MACH_MATCH_DATA: *const c_void = core::ptr::null();

pub unsafe extern "C" fn prom_init() {
    plat_get_fdt();
    assert!(!FDT.is_null());
}

pub unsafe extern "C" fn plat_get_fdt() -> *mut c_void {
    if !FDT.is_null() {
        // Already set up
        return FDT as *mut c_void;
    }

    FDT = get_fdt();
    if !FDT.is_null() && fdt_check_header(FDT) == 0 {
        // Search registered machines for one compatible with the device tree.
        // The C for_each_mips_machine() iteration is provided by the kernel.
        for check_mach in mips_machines() {
            let match_ = mips_machine_is_compatible(check_mach, FDT);
            if !match_.is_null() {
                MACH = check_mach;
                MACH_MATCH_DATA = (*match_).data;
                break;
            }
        }
    } else if cfg!(feature = "CONFIG_LEGACY_BOARDS") {
        // Legacy-board configuration is a build-time kernel condition.
        for check_mach in mips_machines() {
            let detect = (*check_mach).detect;
            if detect.is_none() || !detect.unwrap()() {
                continue;
            }
            MACH = check_mach;
        }

        assert!(!MACH.is_null());
        FDT = (*MACH).fdt;
    }
    FDT as *mut c_void
}

#[cfg(feature = "CONFIG_RELOCATABLE")]
pub unsafe extern "C" fn plat_fdt_relocated(new_location: *mut c_void) {
    // Reset the cached FDT after relocation and update the UHI argument.
    FDT = core::ptr::null();
    if fw_arg0 == -2 {
        fw_arg1 = new_location as usize;
    }
}

pub unsafe extern "C" fn plat_mem_setup() {
    if !MACH.is_null() {
        if let Some(fixup_fdt) = (*MACH).fixup_fdt {
            FDT = fixup_fdt(FDT, MACH_MATCH_DATA);
        }
    }
    fw_init_cmdline();
    __dt_setup_arch(FDT);
}

pub unsafe extern "C" fn device_tree_init() {
    unflatten_and_copy_device_tree();
    mips_cpc_probe();
    if !register_cps_smp_ops() {
        return;
    }
    if !register_vsmp_smp_ops() {
        return;
    }
    register_up_smp_ops();
}

pub unsafe extern "C" fn apply_mips_fdt_fixups(
    fdt_out: *mut c_void,
    fdt_out_size: usize,
    fdt_in: *const c_void,
    mut fixups: *const mips_fdt_fixup,
) -> i32 {
    let mut err = fdt_open_into(fdt_in, fdt_out, fdt_out_size);
    if err != 0 {
        return err;
    }
    while !(*fixups).apply.is_none() {
        err = (*fixups).apply.unwrap()(fdt_out);
        if err != 0 {
            return err;
        }
        fixups = fixups.add(1);
    }
    err = fdt_pack(fdt_out);
    err
}

pub unsafe extern "C" fn plat_time_init() {
    let mut np: *mut device_node;
    let clk: *mut clk;
    of_clk_init(core::ptr::null());
    if !cpu_has_counter {
        mips_hpt_frequency = 0;
    } else if !MACH.is_null() && (*MACH).measure_hpt_freq.is_some() {
        mips_hpt_frequency = (*MACH).measure_hpt_freq.unwrap()();
    } else {
        np = of_get_cpu_node(0, core::ptr::null());
        if np.is_null() {
            return;
        }
        clk = of_clk_get(np, 0);
        if (clk as *mut c_void as usize) >= usize::MAX - 4096 {
            return;
        }
        mips_hpt_frequency = clk_get_rate(clk);
        clk_put(clk);
        match boot_cpu_type() {
            0x20 | 0x25 => {}
            _ => mips_hpt_frequency /= 2,
        }
    }
    timer_probe();
}

pub unsafe extern "C" fn arch_init_irq() {
    let intc_node = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"mti,cpu-interrupt-controller\0".as_ptr() as *const c_char,
    );
    if !intc_node.is_null() {
        mips_cpu_irq_init();
    }
    of_node_put(intc_node);
    irqchip_init();
}

extern "C" {
    fn mips_machines() -> &'static [*const mips_machine];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
