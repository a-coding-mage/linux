// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn show_cpuinfo(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let n = v as usize - 1;
    let isa = cpu_data[n].isa_level;
    let prid = cpu_data[n].processor_id;
    let version = cpu_data[n].processor_id & 0xff;
    let fp_version = cpu_data[n].fpu_vers;
    let mut freq: u64 = cpu_clock_freq;
    let mut bogomips: u64 = lpj_fine * cpu_clock_freq;

    // CONFIG_SMP conditional is preserved by the source-level check below.
    #[cfg(CONFIG_SMP)]
    if !cpu_online(n) {
        return 0;
    }

    freq /= 10000;
    bogomips /= const_clock_freq * (5000 / HZ);

    if n == 0 {
        seq_printf(m, "system type\t\t: %s\n\n", get_system_type());
    }

    seq_printf(m, "processor\t\t: %ld\n", n);
    seq_printf(m, "package\t\t\t: %d\n", cpu_data[n].package);
    seq_printf(m, "core\t\t\t: %d\n", cpu_data[n].core);
    seq_printf(m, "global_id\t\t: %d\n", cpu_data[n].global_id);
    seq_printf(m, "CPU Family\t\t: %s\n", __cpu_family[n]);
    seq_printf(m, "Model Name\t\t: %s\n", __cpu_full_name[n]);
    seq_printf(m, "PRID\t\t\t: %s (%08x)\n", id_to_core_name(prid), prid);
    seq_printf(m, "CPU Revision\t\t: 0x%02x\n", version);
    seq_printf(m, "FPU Revision\t\t: 0x%02x\n", fp_version);
    seq_printf(m, "CPU MHz\t\t\t: %u.%02u\n", (freq as u32) / 100, (freq as u32) % 100);
    seq_printf(m, "BogoMIPS\t\t: %u.%02u\n", (bogomips as u32) / 100, (bogomips as u32) % 100);
    seq_printf(m, "TLB Entries\t\t: %d\n", cpu_data[n].tlbsize);
    seq_printf(m, "Address Sizes\t\t: %d bits physical, %d bits virtual\n", cpu_pabits + 1, cpu_vabits + 1);

    seq_puts(m, "ISA\t\t\t:");
    if isa & LOONGARCH_CPU_ISA_LA32R != 0 { seq_puts(m, " loongarch32r"); }
    if isa & LOONGARCH_CPU_ISA_LA32S != 0 { seq_puts(m, " loongarch32s"); }
    if isa & LOONGARCH_CPU_ISA_LA64 != 0 { seq_puts(m, " loongarch64"); }
    seq_puts(m, "\n");

    seq_puts(m, "Features\t\t:");
    if cpu_has_cpucfg { seq_puts(m, " cpucfg"); }
    if cpu_has_lam { seq_puts(m, " lam"); }
    if cpu_has_lam_bh { seq_puts(m, " lam_bh"); }
    if cpu_has_scq { seq_puts(m, " scq"); }
    if cpu_has_ual { seq_puts(m, " ual"); }
    if cpu_has_fpu { seq_puts(m, " fpu"); }
    if cpu_has_lsx { seq_puts(m, " lsx"); }
    if cpu_has_lasx { seq_puts(m, " lasx"); }
    if cpu_has_crc32 { seq_puts(m, " crc32"); }
    if cpu_has_complex { seq_puts(m, " complex"); }
    if cpu_has_crypto { seq_puts(m, " crypto"); }
    if cpu_has_ptw { seq_puts(m, " ptw"); }
    if cpu_has_lspw { seq_puts(m, " lspw"); }
    if cpu_has_lvz { seq_puts(m, " lvz"); }
    if cpu_has_lbt_x86 { seq_puts(m, " lbt_x86"); }
    if cpu_has_lbt_arm { seq_puts(m, " lbt_arm"); }
    if cpu_has_lbt_mips { seq_puts(m, " lbt_mips"); }
    seq_puts(m, "\n");

    seq_printf(m, "Hardware Watchpoint\t: %s", str_yes_no(cpu_has_watch));
    if cpu_has_watch {
        seq_printf(m, ", iwatch count: %d, dwatch count: %d", cpu_data[n].watch_ireg_count, cpu_data[n].watch_dreg_count);
    }
    seq_puts(m, "\n\n");
    0
}

unsafe fn c_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let i = *pos as usize;
    if i < nr_cpu_ids { (i + 1) as *mut core::ffi::c_void } else { core::ptr::null_mut() }
}

unsafe fn c_next(m: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    *pos += 1;
    c_start(m, pos)
}

unsafe fn c_stop(_m: *mut seq_file, _v: *mut core::ffi::c_void) {}

pub static cpuinfo_op: seq_operations = seq_operations {
    start: Some(c_start),
    next: Some(c_next),
    stop: Some(c_stop),
    show: Some(show_cpuinfo),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
