// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Support for Ingenic SoCs
 *
 * Copyright (C) 2009-2010, Lars-Peter Clausen <lars@metafoo.de>
 * Copyright (C) 2011, Maarten ter Huurne <maarten@treewalker.org>
 * Copyright (C) 2020 Paul Cercueil <paul@crapouillou.net>
 */

// Kernel and architecture dependencies are supplied by other translation units.

unsafe fn ingenic_get_system_type(machtype: usize) -> *mut u8 {
    match machtype {
        MACH_INGENIC_X2100 => b"X2100\0" as *const u8 as *mut u8,
        MACH_INGENIC_X2000H => b"X2000H\0" as *const u8 as *mut u8,
        MACH_INGENIC_X2000E => b"X2000E\0" as *const u8 as *mut u8,
        MACH_INGENIC_X2000 => b"X2000\0" as *const u8 as *mut u8,
        MACH_INGENIC_X1830 => b"X1830\0" as *const u8 as *mut u8,
        MACH_INGENIC_X1000E => b"X1000E\0" as *const u8 as *mut u8,
        MACH_INGENIC_X1000 => b"X1000\0" as *const u8 as *mut u8,
        MACH_INGENIC_JZ4780 => b"JZ4780\0" as *const u8 as *mut u8,
        MACH_INGENIC_JZ4775 => b"JZ4775\0" as *const u8 as *mut u8,
        MACH_INGENIC_JZ4770 => b"JZ4770\0" as *const u8 as *mut u8,
        MACH_INGENIC_JZ4760B => b"JZ4760B\0" as *const u8 as *mut u8,
        MACH_INGENIC_JZ4760 => b"JZ4760\0" as *const u8 as *mut u8,
        MACH_INGENIC_JZ4755 => b"JZ4755\0" as *const u8 as *mut u8,
        MACH_INGENIC_JZ4750 => b"JZ4750\0" as *const u8 as *mut u8,
        MACH_INGENIC_JZ4725B => b"JZ4725B\0" as *const u8 as *mut u8,
        MACH_INGENIC_JZ4730 => b"JZ4730\0" as *const u8 as *mut u8,
        _ => b"JZ4740\0" as *const u8 as *mut u8,
    }
}

const INGENIC_CGU_BASE: usize = 0x10000000;
const JZ4750_CGU_CPCCR_ECS: u32 = 1 << 30;
const JZ4760_CGU_CPCCR_ECS: u32 = 1 << 31;

unsafe fn ingenic_force_12M_ext(fdt: *const core::ffi::c_void, mask: u32) {
    let offset = fdt_path_offset(fdt, b"/ext\0".as_ptr() as *const i8);
    if offset < 0 { return; }
    let prop = fdt_getprop(fdt, offset, b"clock-frequency\0".as_ptr() as *const i8, core::ptr::null_mut());
    if prop.is_null() { return; }
    let use_div = u32::from_be(*(prop as *const u32)) >= 16000000;
    let cgu = ioremap(INGENIC_CGU_BASE, 0x4);
    if cgu.is_null() { return; }
    let mut cpccr = ioread32(cgu);
    if use_div { cpccr |= mask; } else { cpccr &= !mask; }
    iowrite32(cpccr, cgu);
    iounmap(cgu);
}

unsafe fn ingenic_fixup_fdt(fdt: *const core::ffi::c_void, match_data: *const core::ffi::c_void) -> *const core::ffi::c_void {
    if fdt_node_check_compatible(fdt, 0, b"qi,lb60\0".as_ptr() as *const i8) == 0 && fdt_path_offset(fdt, b"/memory\0".as_ptr() as *const i8) < 0 {
        early_init_dt_add_memory_arch(0, SZ_32M);
    }
    mips_machtype = match_data as usize;
    system_type = ingenic_get_system_type(mips_machtype);
    match mips_machtype {
        MACH_INGENIC_JZ4750 | MACH_INGENIC_JZ4755 => ingenic_force_12M_ext(fdt, JZ4750_CGU_CPCCR_ECS),
        MACH_INGENIC_JZ4760 => ingenic_force_12M_ext(fdt, JZ4760_CGU_CPCCR_ECS),
        _ => {}
    }
    fdt
}

#[repr(C)]
struct OfDeviceId { compatible: *const i8, data: *const core::ffi::c_void }

static ingenic_of_match: &[OfDeviceId] = &[
    OfDeviceId { compatible: b"ingenic,jz4730\0".as_ptr() as *const i8, data: MACH_INGENIC_JZ4730 as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,jz4740\0".as_ptr() as *const i8, data: MACH_INGENIC_JZ4740 as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,jz4725b\0".as_ptr() as *const i8, data: MACH_INGENIC_JZ4725B as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,jz4750\0".as_ptr() as *const i8, data: MACH_INGENIC_JZ4750 as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,jz4755\0".as_ptr() as *const i8, data: MACH_INGENIC_JZ4755 as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,jz4760\0".as_ptr() as *const i8, data: MACH_INGENIC_JZ4760 as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,jz4760b\0".as_ptr() as *const i8, data: MACH_INGENIC_JZ4760B as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,jz4770\0".as_ptr() as *const i8, data: MACH_INGENIC_JZ4770 as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,jz4775\0".as_ptr() as *const i8, data: MACH_INGENIC_JZ4775 as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,jz4780\0".as_ptr() as *const i8, data: MACH_INGENIC_JZ4780 as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,x1000\0".as_ptr() as *const i8, data: MACH_INGENIC_X1000 as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,x1000e\0".as_ptr() as *const i8, data: MACH_INGENIC_X1000E as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,x1830\0".as_ptr() as *const i8, data: MACH_INGENIC_X1830 as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,x2000\0".as_ptr() as *const i8, data: MACH_INGENIC_X2000 as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,x2000e\0".as_ptr() as *const i8, data: MACH_INGENIC_X2000E as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,x2000h\0".as_ptr() as *const i8, data: MACH_INGENIC_X2000H as *const core::ffi::c_void },
    OfDeviceId { compatible: b"ingenic,x2100\0".as_ptr() as *const i8, data: MACH_INGENIC_X2100 as *const core::ffi::c_void },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe fn ingenic_wait_instr() { core::arch::asm!(".set push", ".set mips3", "wait", ".set pop"); }
unsafe fn ingenic_halt() -> ! { loop { ingenic_wait_instr(); } }
unsafe fn ingenic_pm_enter(_state: suspend_state_t) -> i32 { ingenic_wait_instr(); 0 }

#[repr(C)]
struct PlatformSuspendOps { valid: Option<unsafe extern "C" fn(suspend_state_t) -> bool>, enter: Option<unsafe extern "C" fn(suspend_state_t) -> i32> }
static ingenic_pm_ops: PlatformSuspendOps = PlatformSuspendOps { valid: Some(suspend_valid_only_mem), enter: Some(ingenic_pm_enter) };

unsafe fn ingenic_pm_init() -> i32 {
    if boot_cpu_type() == CPU_XBURST {
        if IS_ENABLED_CONFIG_PM_SLEEP { suspend_set_ops(&ingenic_pm_ops); }
        _machine_halt = Some(ingenic_halt);
    }
    0
}

// late_initcall(ingenic_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
