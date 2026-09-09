/*
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of this
 * archive for more details.
 *
 * Copyright (C) 2000 - 2001 by Kanoj Sarcar (kanoj@sgi.com)
 * Copyright (C) 2000 - 2001 by Silicon Graphics, Inc.
 * Copyright (C) 2000, 2001, 2002 Ralf Baechle
 * Copyright (C) 2000, 2001 Broadcom Corporation
 */

/* C header guard: __ASM_SMP_OPS_H */
/* Dependency: linux/errno.h */

#[cfg(CONFIG_SMP)]
/* Dependency: linux/cpumask.h */

#[cfg(CONFIG_SMP)]
pub struct task_struct;

#[cfg(CONFIG_SMP)]
#[repr(C)]
pub struct plat_smp_ops {
    pub send_ipi_single: Option<unsafe extern "C" fn(cpu: ::core::ffi::c_int, action: u32)>,
    pub send_ipi_mask: Option<unsafe extern "C" fn(mask: *const cpumask, action: u32)>,
    pub init_secondary: Option<unsafe extern "C" fn()>,
    pub smp_finish: Option<unsafe extern "C" fn()>,
    pub boot_secondary: Option<unsafe extern "C" fn(cpu: ::core::ffi::c_int, idle: *mut task_struct) -> ::core::ffi::c_int>,
    pub smp_setup: Option<unsafe extern "C" fn()>,
    pub prepare_cpus: Option<unsafe extern "C" fn(max_cpus: u32)>,
    pub prepare_boot_cpu: Option<unsafe extern "C" fn()>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_disable: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cpu_die: Option<unsafe extern "C" fn(cpu: u32)>,
    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub cleanup_dead_cpu: Option<unsafe extern "C" fn(cpu: u32)>,
    #[cfg(CONFIG_KEXEC_CORE)]
    pub kexec_nonboot_cpu: Option<unsafe extern "C" fn()>,
}

#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn register_smp_ops(ops: *const plat_smp_ops);
}

#[cfg(CONFIG_SMP)]
#[inline]
pub unsafe fn plat_smp_setup() {
    /* private */
    extern "C" {
        static mp_ops: *const plat_smp_ops;
    }

    ((*mp_ops).smp_setup.expect("smp_setup function pointer"))();
}

#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn mips_smp_send_ipi_single(cpu: ::core::ffi::c_int, action: u32);
    pub fn mips_smp_send_ipi_mask(mask: *const cpumask, action: u32);
}

#[cfg(not(CONFIG_SMP))]
pub struct plat_smp_ops;

#[cfg(not(CONFIG_SMP))]
#[inline]
pub fn plat_smp_setup() {
    /* UP, nothing to do ... */
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub fn register_smp_ops(_ops: *const plat_smp_ops) {
}

#[inline]
pub unsafe fn register_up_smp_ops() -> ::core::ffi::c_int {
    #[cfg(CONFIG_SMP_UP)]
    {
        extern "C" {
            static up_smp_ops: plat_smp_ops;
        }

        register_smp_ops(&up_smp_ops);
        return 0;
    }
    #[cfg(not(CONFIG_SMP_UP))]
    {
        return -ENODEV;
    }
}

#[inline]
pub unsafe fn register_vsmp_smp_ops() -> ::core::ffi::c_int {
    #[cfg(CONFIG_MIPS_MT_SMP)]
    {
        extern "C" {
            static vsmp_smp_ops: plat_smp_ops;
            static cpu_has_mipsmt: bool;
        }

        if !cpu_has_mipsmt {
            return -ENODEV;
        }

        register_smp_ops(&vsmp_smp_ops);
        return 0;
    }
    #[cfg(not(CONFIG_MIPS_MT_SMP))]
    {
        return -ENODEV;
    }
}

#[cfg(CONFIG_MIPS_CPS)]
extern "C" {
    pub fn register_cps_smp_ops() -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_MIPS_CPS))]
#[inline]
pub fn register_cps_smp_ops() -> ::core::ffi::c_int {
    -ENODEV
}

/* ENODEV is supplied by linux/errno.h; cpumask is supplied by linux/cpumask.h. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
