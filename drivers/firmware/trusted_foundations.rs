// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Trusted Foundations support for ARM CPUs
 *
 * Copyright (c) 2013, NVIDIA Corporation.
 */

// C dependencies supplied by the surrounding kernel translation unit.

const TF_CACHE_MAINT: u32 = 0xfffff100;
const TF_CACHE_ENABLE: u32 = 1;
const TF_CACHE_DISABLE: u32 = 2;
const TF_CACHE_REENABLE: u32 = 4;
const TF_SET_CPU_BOOT_ADDR_SMC: u32 = 0xfffff200;
const TF_CPU_PM: u32 = 0xfffffffc;
const TF_CPU_PM_S3: u32 = 0xffffffe3;
const TF_CPU_PM_S2: u32 = 0xffffffe6;
const TF_CPU_PM_S2_NO_MC_CLK: u32 = 0xffffffe5;
const TF_CPU_PM_S1: u32 = 0xffffffe4;
const TF_CPU_PM_S1_NOFLUSH_L2: u32 = 0xffffffe7;

static mut tf_idle_mode: usize = TF_PM_MODE_NONE;
static mut cpu_boot_addr: u64 = 0;

unsafe fn tf_generic_smc(type_: u32, arg1: u32, arg2: u32) {
    // The original ARM inline assembly performs the secure monitor call.
    core::arch::asm!(
        "stmfd sp!, {{r4-r11}}",
        "mov r3, #0",
        "mov r4, #0",
        "smc #0",
        "ldmfd sp!, {{r4-r11}}",
        in("r0") type_, in("r1") arg1, in("r2") arg2,
        lateout("r3") _, lateout("r12") _, lateout("lr") _,
        options(nostack)
    );
}

unsafe fn tf_set_cpu_boot_addr(_cpu: i32, boot_addr: u64) -> i32 {
    cpu_boot_addr = boot_addr;
    tf_generic_smc(TF_SET_CPU_BOOT_ADDR_SMC, cpu_boot_addr as u32, 0);
    0
}

unsafe fn tf_prepare_idle(mode: usize) -> i32 {
    match mode {
        TF_PM_MODE_LP0 => tf_generic_smc(TF_CPU_PM, TF_CPU_PM_S3, cpu_boot_addr as u32),
        TF_PM_MODE_LP1 => tf_generic_smc(TF_CPU_PM, TF_CPU_PM_S2, cpu_boot_addr as u32),
        TF_PM_MODE_LP1_NO_MC_CLK => tf_generic_smc(TF_CPU_PM, TF_CPU_PM_S2_NO_MC_CLK, cpu_boot_addr as u32),
        TF_PM_MODE_LP2 => tf_generic_smc(TF_CPU_PM, TF_CPU_PM_S1, cpu_boot_addr as u32),
        TF_PM_MODE_LP2_NOFLUSH_L2 => tf_generic_smc(TF_CPU_PM, TF_CPU_PM_S1_NOFLUSH_L2, cpu_boot_addr as u32),
        TF_PM_MODE_NONE => {}
        _ => return -22,
    }
    tf_idle_mode = mode;
    0
}

#[cfg(CONFIG_CACHE_L2X0)]
unsafe fn tf_cache_write_sec(val: u64, reg: u32) {
    let mut l2x0_way_mask: u32 = 0xff;
    if reg == L2X0_CTRL {
        if (l2x0_saved_regs.aux_ctrl & L310_AUX_CTRL_ASSOCIATIVITY_16) != 0 {
            l2x0_way_mask = 0xffff;
        }
        let enable_op = if tf_idle_mode == TF_PM_MODE_LP2 { TF_CACHE_REENABLE } else { TF_CACHE_ENABLE };
        if val == L2X0_CTRL_EN as u64 {
            tf_generic_smc(TF_CACHE_MAINT, enable_op, l2x0_saved_regs.aux_ctrl);
        } else {
            tf_generic_smc(TF_CACHE_MAINT, TF_CACHE_DISABLE, l2x0_way_mask);
        }
    }
}

#[cfg(CONFIG_CACHE_L2X0)]
unsafe fn tf_init_cache() -> i32 {
    outer_cache.write_sec = Some(tf_cache_write_sec);
    0
}

// The surrounding translation supplies firmware_ops, trusted_foundations_ops,
// register_firmware_ops, firmware_ops, and the platform data definitions.
static trusted_foundations_ops: firmware_ops = firmware_ops {
    set_cpu_boot_addr: Some(tf_set_cpu_boot_addr),
    prepare_idle: Some(tf_prepare_idle),
    #[cfg(CONFIG_CACHE_L2X0)]
    l2x0_init: Some(tf_init_cache),
};

pub unsafe fn register_trusted_foundations(pd: *mut trusted_foundations_platform_data) {
    let _ = pd;
    register_firmware_ops(&trusted_foundations_ops);
}

pub unsafe fn of_register_trusted_foundations() {
    let node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"tlm,trusted-foundations\0".as_ptr() as *const i8);
    if node.is_null() { return; }
    let mut pdata: trusted_foundations_platform_data = core::mem::zeroed();
    if of_property_read_u32(node, b"tlm,version-major\0".as_ptr() as *const i8, &mut pdata.version_major) != 0 {
        panic!("Trusted Foundation: missing version-major property\n");
    }
    if of_property_read_u32(node, b"tlm,version-minor\0".as_ptr() as *const i8, &mut pdata.version_minor) != 0 {
        panic!("Trusted Foundation: missing version-minor property\n");
    }
    register_trusted_foundations(&mut pdata);
}

pub unsafe fn trusted_foundations_registered() -> bool {
    firmware_ops == &trusted_foundations_ops
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
