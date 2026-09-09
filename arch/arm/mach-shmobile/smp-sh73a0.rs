// SPDX-License-Identifier: GPL-2.0
/*
 * SMP support for R-Mobile / SH-Mobile - sh73a0 portion
 *
 * Copyright (C) 2010  Magnus Damm
 * Copyright (C) 2010  Takashi Yoshii
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

const CPG_BASE2: usize = 0xe6151000;
const WUPCR: usize = 0x10; // System-CPU Wake Up Control Register
const SRESCR: usize = 0x18; // System-CPU Software Reset Control Register
const PSTR: usize = 0x40; // System-CPU Power Status Register

const SYSC_BASE: usize = 0xe6180000;
const SBAR: usize = 0x20; // SYS Boot Address Register

const AP_BASE: usize = 0xe6f10000;
const APARMBAREA: usize = 0x20; // Address Translation Area Register

const SH73A0_SCU_BASE: usize = 0xf0000000;

unsafe fn sh73a0_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    let lcpu: u32 = cpu_logical_map(cpu);
    let cpg2: *mut core::ffi::c_void = ioremap(CPG_BASE2, PAGE_SIZE);

    if ((readl(cpg2.byte_add(PSTR)) >> (4 * lcpu)) & 3) == 3 {
        writel(1u32 << lcpu, cpg2.byte_add(WUPCR)); // wake up
    } else {
        writel(1u32 << lcpu, cpg2.byte_add(SRESCR)); // reset
    }
    iounmap(cpg2);
    0
}

unsafe fn sh73a0_smp_prepare_cpus(max_cpus: u32) {
    let ap: *mut core::ffi::c_void;
    let sysc: *mut core::ffi::c_void;

    if request_mem_region(0, SZ_4K, "Boot Area\0".as_ptr() as *const i8).is_null() {
        pr_err!("Failed to request boot area\n");
        return;
    }

    // Map the reset vector (in headsmp.S)
    ap = ioremap(AP_BASE, PAGE_SIZE);
    sysc = ioremap(SYSC_BASE, PAGE_SIZE);
    writel(0, ap.byte_add(APARMBAREA)); // 4k
    writel(__pa(shmobile_boot_vector), sysc.byte_add(SBAR));
    iounmap(sysc);
    iounmap(ap);

    // setup sh73a0 specific SCU bits
    shmobile_smp_scu_prepare_cpus(SH73A0_SCU_BASE, max_cpus);
}

// CONFIG_HOTPLUG_CPU conditionally supplies the additional operation members.
pub const sh73a0_smp_ops: smp_operations = smp_operations {
    smp_prepare_cpus: sh73a0_smp_prepare_cpus,
    smp_boot_secondary: sh73a0_boot_secondary,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    cpu_can_disable: shmobile_smp_cpu_can_disable,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    cpu_die: shmobile_smp_scu_cpu_die,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    cpu_kill: shmobile_smp_scu_cpu_kill,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
