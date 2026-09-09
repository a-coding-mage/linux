// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Processor capabilities determination functions.
 *
 * Copyright (C) xxxx  the Anonymous
 * Copyright (C) 1994 - 2006 Ralf Baechle
 * Copyright (C) 2003, 2004  Maciej W. Rozycki
 * Copyright (C) 2001, 2004, 2011, 2012  MIPS Technologies, Inc.
 */

// Linux and architecture headers from the original translation provide the
// externally defined types, constants, functions, and globals used below.

/* Hardware capabilities */
pub static mut elf_hwcap: ::core::ffi::c_uint = 0;

pub unsafe fn check_bugs32() {}

/*
 * Probe whether cpu has config register by trying to play with
 * alternate cache bit and see whether it matters.
 * It's used by cpu_probe to distinguish between R3000A and R3081.
 */
#[inline]
unsafe fn cpu_has_confreg() -> ::core::ffi::c_int {
    #[cfg(CONFIG_CPU_R3000)]
    {
        let mut size1: ::core::ffi::c_ulong;
        let mut size2: ::core::ffi::c_ulong;
        let cfg: ::core::ffi::c_ulong = read_c0_conf();

        size1 = r3k_cache_size(ST0_ISC);
        write_c0_conf(cfg ^ R30XX_CONF_AC);
        size2 = r3k_cache_size(ST0_ISC);
        write_c0_conf(cfg);
        return (size1 != size2) as ::core::ffi::c_int;
    }
    #[cfg(not(CONFIG_CPU_R3000))]
    {
        0
    }
}

#[inline]
unsafe fn set_elf_platform(cpu: ::core::ffi::c_int, plat: *const ::core::ffi::c_char) {
    if cpu == 0 {
        __elf_platform = plat;
    }
}

pub static mut __cpu_name: [*const ::core::ffi::c_char; NR_CPUS] = [core::ptr::null(); NR_CPUS];
pub static mut __elf_platform: *const ::core::ffi::c_char = core::ptr::null();
pub static mut __elf_base_platform: *const ::core::ffi::c_char = core::ptr::null();

pub unsafe fn cpu_probe() {
    let c: *mut cpuinfo_mips = &raw mut current_cpu_data;
    let cpu: ::core::ffi::c_uint = smp_processor_id();

    /*
     * Set a default elf platform, cpu probe may later
     * overwrite it with a more precise value
     */
    set_elf_platform(cpu as ::core::ffi::c_int, c"mips".as_ptr());

    (*c).processor_id = PRID_IMP_UNKNOWN;
    (*c).fpu_id = FPIR_IMP_NONE;
    (*c).cputype = CPU_UNKNOWN;
    (*c).writecombine = _CACHE_UNCACHED;

    (*c).fpu_csr31 = FPU_CSR_RN;
    (*c).fpu_msk31 = FPU_CSR_RSVD | FPU_CSR_ABS2008 | FPU_CSR_NAN2008 |
        FPU_CSR_CONDX | FPU_CSR_FS;

    (*c).srsets = 1;

    (*c).processor_id = read_c0_prid();
    match (*c).processor_id & (PRID_COMP_MASK | PRID_IMP_MASK) {
        PRID_COMP_LEGACY | PRID_IMP_R2000 => {
            (*c).cputype = CPU_R2000;
            __cpu_name[cpu as usize] = c"R2000".as_ptr();
            (*c).options = MIPS_CPU_TLB | MIPS_CPU_3K_CACHE | MIPS_CPU_NOFPUEX;
            if __cpu_has_fpu() { (*c).options |= MIPS_CPU_FPU; }
            (*c).tlbsize = 64;
        }
        PRID_COMP_LEGACY | PRID_IMP_R3000 => {
            if ((*c).processor_id & PRID_REV_MASK) == PRID_REV_R3000A {
                if cpu_has_confreg() != 0 {
                    (*c).cputype = CPU_R3081E;
                    __cpu_name[cpu as usize] = c"R3081".as_ptr();
                } else {
                    (*c).cputype = CPU_R3000A;
                    __cpu_name[cpu as usize] = c"R3000A".as_ptr();
                }
            } else {
                (*c).cputype = CPU_R3000;
                __cpu_name[cpu as usize] = c"R3000".as_ptr();
            }
            (*c).options = MIPS_CPU_TLB | MIPS_CPU_3K_CACHE | MIPS_CPU_NOFPUEX;
            if __cpu_has_fpu() { (*c).options |= MIPS_CPU_FPU; }
            (*c).tlbsize = 64;
        }
        _ => {}
    }

    BUG_ON(__cpu_name[cpu as usize].is_null());
    BUG_ON((*c).cputype == CPU_UNKNOWN);
    BUG_ON(current_cpu_type() != (*c).cputype);

    if mips_fpu_disabled != 0 {
        (*c).options &= !MIPS_CPU_FPU;
    }

    if (*c).options & MIPS_CPU_FPU != 0 {
        cpu_set_fpu_opts(c);
    } else {
        cpu_set_nofpu_opts(c);
    }

    (*c).vmbits = 31;
    reserve_exception_space(0, 0x400);
}

pub unsafe fn cpu_report() {
    let c: *mut cpuinfo_mips = &raw mut current_cpu_data;

    pr_info!("CPU%d revision is: %08x (%s)\n", smp_processor_id(), (*c).processor_id, cpu_name_string());
    if (*c).options & MIPS_CPU_FPU != 0 {
        pr_info!("FPU revision is: %08x\n", (*c).fpu_id);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
