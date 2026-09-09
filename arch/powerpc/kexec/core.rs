// SPDX-License-Identifier: GPL-2.0-only
/*
 * Code to handle transition of Linux booting another kernel.
 *
 * Copyright (C) 2002-2003 Eric Biederman  <ebiederm@xmission.com>
 * GameCube/ppc32 port Copyright (C) 2004 Albert Herranz
 * Copyright (C) 2005 IBM Corporation.
 */

// C headers and build-time configuration are supplied by the surrounding kernel.

#[cfg(feature = "CONFIG_CRASH_DUMP")]
pub unsafe extern "C" fn machine_crash_shutdown(regs: *mut pt_regs) {
    default_machine_crash_shutdown(regs);
}

pub unsafe extern "C" fn machine_kexec_cleanup(_image: *mut kimage) {
}

/*
 * Do not allocate memory (or fail in any way) in machine_kexec().
 * We are past the point of no return, committed to rebooting now.
 */
pub unsafe extern "C" fn machine_kexec(image: *mut kimage) {
    let save_ftrace_enabled: i32 = __ftrace_enabled_save();
    this_cpu_disable_ftrace();

    if ppc_md.machine_kexec != 0 {
        ((*ppc_md.machine_kexec)(image));
    } else {
        default_machine_kexec(image);
    }

    this_cpu_enable_ftrace();
    __ftrace_enabled_restore(save_ftrace_enabled);

    /* Fall back to normal restart if we're still alive. */
    machine_restart(core::ptr::null());
    loop {}
}

#[cfg(feature = "CONFIG_CRASH_RESERVE")]
static mut crashk_cma_size: u64 = 0;

#[cfg(feature = "CONFIG_CRASH_RESERVE")]
unsafe extern "C" fn get_crash_base(crash_base: u64) -> u64 {
    #[cfg(not(feature = "CONFIG_NONSTATIC_KERNEL"))]
    {
        if crash_base != KDUMP_KERNELBASE as u64 {
            printk(c"Crash kernel location must be 0x%x\0");
        }
        return KDUMP_KERNELBASE as u64;
    }

    #[cfg(feature = "CONFIG_NONSTATIC_KERNEL")]
    {
        let mut crash_base = crash_base;
        if crash_base == 0 {
            #[cfg(feature = "CONFIG_PPC64")]
            {
                /* On LPAR place the crash kernel in the middle of the RMA. */
                if firmware_has_feature(FW_FEATURE_LPAR) {
                    crash_base = core::cmp::min(ppc64_rma_size / 2, SZ_512M as u64);
                } else {
                    crash_base = core::cmp::min(ppc64_rma_size / 2, SZ_128M as u64);
                }
            }
            #[cfg(not(feature = "CONFIG_PPC64"))]
            {
                crash_base = KDUMP_KERNELBASE as u64;
            }
        }

        let crash_base_align = PAGE_ALIGN(crash_base);
        if crash_base != crash_base_align {
            pr_warn(c"Crash kernel base must be aligned to 0x%lx\0", PAGE_SIZE);
        }
        crash_base_align
    }
}

#[cfg(feature = "CONFIG_CRASH_RESERVE")]
pub unsafe extern "C" fn arch_reserve_crashkernel() {
    let mut crash_size: u64 = 0;
    let mut crash_base: u64 = 0;
    let mut crash_end: u64;
    let mut kernel_start: u64;
    let mut kernel_size: u64;
    let total_mem_sz = if memory_limit != 0 { memory_limit } else { memblock_phys_mem_size() };
    let ret = parse_crashkernel(boot_command_line, total_mem_sz, &mut crash_size,
        &mut crash_base, core::ptr::null_mut(), &mut crashk_cma_size, core::ptr::null_mut());
    if ret != 0 { return; }
    crash_base = get_crash_base(crash_base);
    crash_end = crash_base + crash_size - 1;
    kernel_start = __pa(_stext);
    kernel_size = _end - _stext;
    if kernel_start + kernel_size > crash_base && kernel_start <= crash_end {
        pr_warn(c"Crash kernel can not overlap current kernel\0");
        return;
    }
    reserve_crashkernel_generic(crash_size, crash_base, 0, false);
}

#[cfg(feature = "CONFIG_CRASH_RESERVE")]
pub unsafe extern "C" fn kdump_cma_reserve() {
    if crashk_cma_size != 0 { reserve_crashkernel_cma(crashk_cma_size); }
}

#[cfg(feature = "CONFIG_CRASH_RESERVE")]
pub unsafe extern "C" fn overlaps_crashkernel(start: c_ulong, size: c_ulong) -> i32 {
    if start + size > crashk_res.start && start <= crashk_res.end { 1 } else { 0 }
}

// Values exported to the second kernel via the device tree.
#[cfg(feature = "CONFIG_CRASH_RESERVE")]
static mut crashk_base: __be_word = 0;
#[cfg(feature = "CONFIG_CRASH_RESERVE")]
static mut crashk_size: __be_word = 0;
#[cfg(feature = "CONFIG_CRASH_RESERVE")]
static mut mem_limit: __be_word = 0;

#[cfg(feature = "CONFIG_CRASH_RESERVE")]
static mut crashk_base_prop: property = property {
    name: c"linux,crashkernel-base",
    length: core::mem::size_of::<__be_word>() as u32,
    value: unsafe { &raw mut crashk_base as *mut _ as *mut c_void },
};
#[cfg(feature = "CONFIG_CRASH_RESERVE")]
static mut crashk_size_prop: property = property {
    name: c"linux,crashkernel-size",
    length: core::mem::size_of::<__be_word>() as u32,
    value: unsafe { &raw mut crashk_size as *mut _ as *mut c_void },
};
#[cfg(feature = "CONFIG_CRASH_RESERVE")]
static mut memory_limit_prop: property = property {
    name: c"linux,memory-limit",
    length: core::mem::size_of::<__be_word>() as u32,
    value: unsafe { &raw mut mem_limit as *mut _ as *mut c_void },
};

#[cfg(feature = "CONFIG_CRASH_RESERVE")]
unsafe fn export_crashk_values(node: *mut device_node) {
    of_remove_property(node, of_find_property(node, c"linux,crashkernel-base\0", core::ptr::null_mut()));
    of_remove_property(node, of_find_property(node, c"linux,crashkernel-size\0", core::ptr::null_mut()));
    if crashk_res.start != 0 {
        crashk_base = cpu_to_be_ulong(crashk_res.start);
        of_add_property(node, &mut crashk_base_prop);
        crashk_size = cpu_to_be_ulong(resource_size(&crashk_res));
        of_add_property(node, &mut crashk_size_prop);
    }
    mem_limit = cpu_to_be_ulong(memory_limit);
    of_update_property(node, &mut memory_limit_prop);
}

static mut kernel_end: __be_word = 0;

static mut kernel_end_prop: property = property {
    name: c"linux,kernel-end",
    length: core::mem::size_of::<__be_word>() as u32,
    value: unsafe { &raw mut kernel_end as *mut _ as *mut c_void },
};

unsafe extern "C" fn kexec_setup() -> i32 {
    let node = of_find_node_by_path(c"/chosen\0");
    if node.is_null() { return -ENOENT; }
    of_remove_property(node, of_find_property(node, kernel_end_prop.name, core::ptr::null_mut()));
    kernel_end = cpu_to_be_ulong(__pa(_end));
    of_add_property(node, &mut kernel_end_prop);
    #[cfg(feature = "CONFIG_CRASH_RESERVE")]
    export_crashk_values(node);
    of_node_put(node);
    0
}

late_initcall!(kexec_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
