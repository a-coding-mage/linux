// SPDX-License-Identifier: GPL-2.0

// Translated from the C implementation. Kernel headers and symbols are supplied
// by the surrounding Rust translation.

#[cfg(CONFIG_X86_VMX_FEATURE_NAMES)]
extern "C" {
    static x86_vmx_flags: *const *const core::ffi::c_char;
}

/*
 * Get CPU information for use by the procfs.
 */
unsafe fn show_cpuinfo_core(m: *mut seq_file, c: *mut cpuinfo_x86, cpu: c_uint) {
    #[cfg(CONFIG_SMP)]
    {
        seq_printf!(m, "physical id\t: %d\n", (*c).topo.pkg_id);
        seq_printf!(m, "siblings\t: %d\n", cpumask_weight(topology_core_cpumask(cpu)));
        seq_printf!(m, "core id\t\t: %d\n", (*c).topo.core_id);
        seq_printf!(m, "cpu cores\t: %d\n", (*c).booted_cores);
        seq_printf!(m, "apicid\t\t: %d\n", (*c).topo.apicid);
        seq_printf!(m, "initial apicid\t: %d\n", (*c).topo.initial_apicid);
    }
}

#[cfg(CONFIG_X86_32)]
unsafe fn show_cpuinfo_misc(m: *mut seq_file, c: *mut cpuinfo_x86) {
    seq_printf!(m,
        "fdiv_bug\t: %s\n";
        "f00f_bug\t: %s\n";
        "coma_bug\t: %s\n";
        "fpu\t\t: %s\n";
        "fpu_exception\t: %s\n";
        "cpuid level\t: %d\n";
        "wp\t\t: yes\n",
        str_yes_no(boot_cpu_has_bug(X86_BUG_FDIV)),
        str_yes_no(boot_cpu_has_bug(X86_BUG_F00F)),
        str_yes_no(boot_cpu_has_bug(X86_BUG_COMA)),
        str_yes_no(boot_cpu_has(X86_FEATURE_FPU)),
        str_yes_no(boot_cpu_has(X86_FEATURE_FPU)),
        (*c).cpuid_level);
}

#[cfg(not(CONFIG_X86_32))]
unsafe fn show_cpuinfo_misc(m: *mut seq_file, c: *mut cpuinfo_x86) {
    seq_printf!(m,
        "fpu\t\t: yes\n";
        "fpu_exception\t: yes\n";
        "cpuid level\t: %d\n";
        "wp\t\t: yes\n",
        (*c).cpuid_level);
}

unsafe fn show_cpuinfo(m: *mut seq_file, v: *mut core::ffi::c_void) -> c_int {
    let c = v as *mut cpuinfo_x86;
    let cpu = (*c).cpu_index;
    let mut i: c_int;

    seq_printf!(m,
        "processor\t: %u\n";
        "vendor_id\t: %s\n";
        "cpu family\t: %d\n";
        "model\t\t: %u\n";
        "model name\t: %s\n",
        cpu,
        if (*c).x86_vendor_id[0] != 0 { (*c).x86_vendor_id } else { "unknown" },
        (*c).x86,
        (*c).x86_model,
        if (*c).x86_model_id[0] != 0 { (*c).x86_model_id } else { "unknown" });

    if (*c).x86_stepping != 0 || (*c).cpuid_level >= 0 {
        seq_printf!(m, "stepping\t: %d\n", (*c).x86_stepping);
    } else {
        seq_puts!(m, "stepping\t: unknown\n");
    }
    if (*c).microcode != 0 { seq_printf!(m, "microcode\t: 0x%x\n", (*c).microcode); }

    if cpu_has(c, X86_FEATURE_TSC) {
        let freq = arch_freq_get_on_cpu(cpu);
        if freq < 0 { seq_puts!(m, "cpu MHz\t\t: Unknown\n"); }
        else { seq_printf!(m, "cpu MHz\t\t: %u.%03u\n", freq / 1000, freq % 1000); }
    }
    if (*c).x86_cache_size != 0 { seq_printf!(m, "cache size\t: %u KB\n", (*c).x86_cache_size); }

    show_cpuinfo_core(m, c, cpu);
    show_cpuinfo_misc(m, c);
    seq_puts!(m, "flags\t\t:");
    i = 0;
    while i < 32 * NCAPINTS {
        if cpu_has(c, i) && !x86_cap_flags[i as usize].is_null() { seq_printf!(m, " %s", x86_cap_flags[i as usize]); }
        i += 1;
    }

    #[cfg(CONFIG_X86_VMX_FEATURE_NAMES)]
    if cpu_has(c, X86_FEATURE_VMX) && (*c).vmx_capability[0] != 0 {
        seq_puts!(m, "\nvmx flags\t:");
        i = 0;
        while i < 32 * NVMXINTS {
            if test_bit(i, (*c).vmx_capability.as_mut_ptr() as *mut c_ulong) && !(*x86_vmx_flags.add(i as usize)).is_null() {
                seq_printf!(m, " %s", *x86_vmx_flags.add(i as usize));
            }
            i += 1;
        }
    }

    seq_puts!(m, "\nbugs\t\t:");
    i = 0;
    while i < 32 * NBUGINTS {
        let bug_bit = 32 * NCAPINTS + i;
        if cpu_has_bug(c, bug_bit) && !x86_bug_flags[i as usize].is_null() { seq_printf!(m, " %s", x86_bug_flags[i as usize]); }
        i += 1;
    }
    seq_printf!(m, "\nbogomips\t: %lu.%02lu\n", (*c).loops_per_jiffy / (500000 / HZ), ((*c).loops_per_jiffy / (5000 / HZ)) % 100);

    #[cfg(CONFIG_X86_64)]
    if (*c).x86_tlbsize > 0 { seq_printf!(m, "TLB size\t: %d 4K pages\n", (*c).x86_tlbsize); }
    seq_printf!(m, "clflush size\t: %u\n", (*c).x86_clflush_size);
    seq_printf!(m, "cache_alignment\t: %d\n", (*c).x86_cache_alignment);
    seq_printf!(m, "address sizes\t: %u bits physical, %u bits virtual\n", (*c).x86_phys_bits, (*c).x86_virt_bits);
    seq_puts!(m, "power management:");
    i = 0;
    while i < 32 {
        if (*c).x86_power & (1 << i) != 0 {
            if (i as usize) < x86_power_flags.len() && !x86_power_flags[i as usize].is_null() { seq_printf!(m, "%s%s", if *x86_power_flags[i as usize] != 0 { " " } else { "" }, x86_power_flags[i as usize]); }
            else { seq_printf!(m, " [%d]", i); }
        }
        i += 1;
    }
    seq_puts!(m, "\n\n");
    0
}

unsafe fn c_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    *pos = cpumask_next(*pos - 1, cpu_online_mask);
    if *pos < nr_cpu_ids { &mut cpu_data(*pos) as *mut _ as *mut core::ffi::c_void } else { core::ptr::null_mut() }
}
unsafe fn c_next(m: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void { *pos += 1; c_start(m, pos) }
unsafe fn c_stop(_m: *mut seq_file, _v: *mut core::ffi::c_void) {}

#[no_mangle]
pub static cpuinfo_op: seq_operations = seq_operations { start: Some(c_start), next: Some(c_next), stop: Some(c_stop), show: Some(show_cpuinfo) };

#[cfg(CONFIG_X86_USER_SHADOW_STACK)]
unsafe fn dump_x86_features(m: *mut seq_file, features: c_ulong) {
    if features & ARCH_SHSTK_SHSTK != 0 { seq_puts!(m, "shstk "); }
    if features & ARCH_SHSTK_WRSS != 0 { seq_puts!(m, "wrss "); }
}

#[cfg(CONFIG_X86_USER_SHADOW_STACK)]
pub unsafe fn arch_proc_pid_thread_features(m: *mut seq_file, task: *mut task_struct) {
    seq_puts!(m, "x86_Thread_features:\t"); dump_x86_features(m, (*task).thread.features); seq_putc!(m, '\n');
    seq_puts!(m, "x86_Thread_features_locked:\t"); dump_x86_features(m, (*task).thread.features_locked); seq_putc!(m, '\n');
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
