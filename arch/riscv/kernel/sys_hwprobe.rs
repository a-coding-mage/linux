// SPDX-License-Identifier: GPL-2.0-only
/*
 * The hwprobe interface, for allowing userspace to probe to see which features
 * are supported by the hardware. See Documentation/arch/riscv/hwprobe.rst.
 */

macro_rules! ext_key {
    ($isa_arg:expr, $ext:ident, $pv:expr, $missing:expr) => {
        if unsafe { __riscv_isa_extension_available($isa_arg, RISCV_ISA_EXT_$ext) } {
            $pv |= RISCV_HWPROBE_EXT_$ext;
        } else {
            $missing |= RISCV_HWPROBE_EXT_$ext;
        }
    };
}

unsafe fn hwprobe_arch_id(pair: *mut riscv_hwprobe, cpus: *const cpumask) {
    let mut id: u64 = !0;
    let mut first = true;
    let mut cpu: i32;

    if (*pair).key != RISCV_HWPROBE_KEY_MVENDORID
        && (*pair).key != RISCV_HWPROBE_KEY_MIMPID
        && (*pair).key != RISCV_HWPROBE_KEY_MARCHID
    {
        (*pair).value = id;
        return;
    }

    for_each_cpu!(cpu, cpus, {
        let cpu_id: u64 = match (*pair).key {
            RISCV_HWPROBE_KEY_MVENDORID => riscv_cached_mvendorid(cpu),
            RISCV_HWPROBE_KEY_MIMPID => riscv_cached_mimpid(cpu),
            RISCV_HWPROBE_KEY_MARCHID => riscv_cached_marchid(cpu),
            _ => 0,
        };
        if first { id = cpu_id; first = false; }
        if id != cpu_id { id = !0; break; }
    });
    (*pair).value = id;
}

unsafe fn hwprobe_isa_ext0(pair: *mut riscv_hwprobe, cpus: *const cpumask) {
    let mut missing: u64 = 0;
    (*pair).value = 0;
    if has_fpu() { (*pair).value |= RISCV_HWPROBE_IMA_FD; }
    if riscv_isa_extension_available(core::ptr::null(), C) { (*pair).value |= RISCV_HWPROBE_IMA_C; }
    if has_vector() && riscv_isa_extension_available(core::ptr::null(), V) { (*pair).value |= RISCV_HWPROBE_IMA_V; }

    for_each_cpu!(cpu, cpus, {
        let isainfo = &hart_isa[cpu as usize];
        ext_key!(isainfo.isa, ZAAMO, (*pair).value, missing); ext_key!(isainfo.isa, ZABHA, (*pair).value, missing);
        ext_key!(isainfo.isa, ZACAS, (*pair).value, missing); ext_key!(isainfo.isa, ZALASR, (*pair).value, missing);
        ext_key!(isainfo.isa, ZALRSC, (*pair).value, missing); ext_key!(isainfo.isa, ZAWRS, (*pair).value, missing);
        ext_key!(isainfo.isa, ZBA, (*pair).value, missing); ext_key!(isainfo.isa, ZBB, (*pair).value, missing);
        ext_key!(isainfo.isa, ZBC, (*pair).value, missing); ext_key!(isainfo.isa, ZBKB, (*pair).value, missing);
        ext_key!(isainfo.isa, ZBKC, (*pair).value, missing); ext_key!(isainfo.isa, ZBKX, (*pair).value, missing);
        ext_key!(isainfo.isa, ZBS, (*pair).value, missing); ext_key!(isainfo.isa, ZCA, (*pair).value, missing);
        ext_key!(isainfo.isa, ZCB, (*pair).value, missing); ext_key!(isainfo.isa, ZCLSD, (*pair).value, missing);
        ext_key!(isainfo.isa, ZCMOP, (*pair).value, missing); ext_key!(isainfo.isa, ZICBOM, (*pair).value, missing);
        ext_key!(isainfo.isa, ZICBOP, (*pair).value, missing); ext_key!(isainfo.isa, ZICBOZ, (*pair).value, missing);
        ext_key!(isainfo.isa, ZICFILP, (*pair).value, missing); ext_key!(isainfo.isa, ZICNTR, (*pair).value, missing);
        ext_key!(isainfo.isa, ZICOND, (*pair).value, missing); ext_key!(isainfo.isa, ZIHINTNTL, (*pair).value, missing);
        ext_key!(isainfo.isa, ZIHINTPAUSE, (*pair).value, missing); ext_key!(isainfo.isa, ZIHPM, (*pair).value, missing);
        ext_key!(isainfo.isa, ZILSD, (*pair).value, missing); ext_key!(isainfo.isa, ZIMOP, (*pair).value, missing);
        ext_key!(isainfo.isa, ZKND, (*pair).value, missing); ext_key!(isainfo.isa, ZKNE, (*pair).value, missing);
        ext_key!(isainfo.isa, ZKNH, (*pair).value, missing); ext_key!(isainfo.isa, ZKSED, (*pair).value, missing);
        ext_key!(isainfo.isa, ZKSH, (*pair).value, missing); ext_key!(isainfo.isa, ZKT, (*pair).value, missing);
        ext_key!(isainfo.isa, ZTSO, (*pair).value, missing);
        if has_vector() {
            ext_key!(isainfo.isa, ZVBB, (*pair).value, missing); ext_key!(isainfo.isa, ZVBC, (*pair).value, missing);
            ext_key!(isainfo.isa, ZVE32F, (*pair).value, missing); ext_key!(isainfo.isa, ZVE32X, (*pair).value, missing);
            ext_key!(isainfo.isa, ZVE64D, (*pair).value, missing); ext_key!(isainfo.isa, ZVE64F, (*pair).value, missing);
            ext_key!(isainfo.isa, ZVE64X, (*pair).value, missing); ext_key!(isainfo.isa, ZVFBFMIN, (*pair).value, missing);
            ext_key!(isainfo.isa, ZVFBFWMA, (*pair).value, missing); ext_key!(isainfo.isa, ZVFH, (*pair).value, missing);
            ext_key!(isainfo.isa, ZVFHMIN, (*pair).value, missing); ext_key!(isainfo.isa, ZVKB, (*pair).value, missing);
            ext_key!(isainfo.isa, ZVKG, (*pair).value, missing); ext_key!(isainfo.isa, ZVKNED, (*pair).value, missing);
            ext_key!(isainfo.isa, ZVKNHA, (*pair).value, missing); ext_key!(isainfo.isa, ZVKNHB, (*pair).value, missing);
            ext_key!(isainfo.isa, ZVKSED, (*pair).value, missing); ext_key!(isainfo.isa, ZVKSH, (*pair).value, missing);
            ext_key!(isainfo.isa, ZVKT, (*pair).value, missing);
        }
        ext_key!(isainfo.isa, ZCD, (*pair).value, missing); ext_key!(isainfo.isa, ZCF, (*pair).value, missing);
        ext_key!(isainfo.isa, ZFA, (*pair).value, missing); ext_key!(isainfo.isa, ZFBFMIN, (*pair).value, missing);
        ext_key!(isainfo.isa, ZFH, (*pair).value, missing); ext_key!(isainfo.isa, ZFHMIN, (*pair).value, missing);
        if IS_ENABLED!(CONFIG_RISCV_ISA_SUPM) { ext_key!(isainfo.isa, SUPM, (*pair).value, missing); }
    });
    (*pair).value &= !missing;
}

unsafe fn hwprobe_isa_ext1(pair: *mut riscv_hwprobe, cpus: *const cpumask) {
    let mut missing: u64 = 0; (*pair).value = 0;
    for_each_cpu!(cpu, cpus, {
        let isainfo = &hart_isa[cpu as usize];
        ext_key!(isainfo.isa, ZICFISS, (*pair).value, missing); ext_key!(isainfo.isa, ZICCLSM, (*pair).value, missing);
        ext_key!(isainfo.isa, ZICCAMOA, (*pair).value, missing); ext_key!(isainfo.isa, ZICCIF, (*pair).value, missing);
        ext_key!(isainfo.isa, ZICCRSE, (*pair).value, missing); ext_key!(isainfo.isa, ZA64RS, (*pair).value, missing);
    });
    (*pair).value &= !missing;
}

unsafe fn hwprobe_ext0_has(cpus: *const cpumask, ext: u64) -> bool { let mut pair = riscv_hwprobe::default(); hwprobe_isa_ext0(&mut pair, cpus); (pair.value & ext) != 0 }

// CONFIG_RISCV_PROBE_UNALIGNED_ACCESS selects the probing implementation.
unsafe fn hwprobe_misaligned(_cpus: *const cpumask) -> u64 {
    if IS_ENABLED!(CONFIG_RISCV_EFFICIENT_UNALIGNED_ACCESS) { return RISCV_HWPROBE_MISALIGNED_SCALAR_FAST; }
    if IS_ENABLED!(CONFIG_RISCV_EMULATED_UNALIGNED_ACCESS) && unaligned_ctl_available() { return RISCV_HWPROBE_MISALIGNED_SCALAR_EMULATED; }
    RISCV_HWPROBE_MISALIGNED_SCALAR_SLOW
}

unsafe fn hwprobe_vec_misaligned(_cpus: *const cpumask) -> u64 {
    if IS_ENABLED!(CONFIG_RISCV_EFFICIENT_VECTOR_UNALIGNED_ACCESS) { return RISCV_HWPROBE_MISALIGNED_VECTOR_FAST; }
    if IS_ENABLED!(CONFIG_RISCV_SLOW_VECTOR_UNALIGNED_ACCESS) { return RISCV_HWPROBE_MISALIGNED_VECTOR_SLOW; }
    RISCV_HWPROBE_MISALIGNED_VECTOR_UNKNOWN
}

unsafe fn hwprobe_one_pair(pair: *mut riscv_hwprobe, cpus: *const cpumask) {
    match (*pair).key {
        RISCV_HWPROBE_KEY_MVENDORID | RISCV_HWPROBE_KEY_MARCHID | RISCV_HWPROBE_KEY_MIMPID => hwprobe_arch_id(pair, cpus),
        RISCV_HWPROBE_KEY_BASE_BEHAVIOR => (*pair).value = RISCV_HWPROBE_BASE_BEHAVIOR_IMA,
        RISCV_HWPROBE_KEY_IMA_EXT_0 => hwprobe_isa_ext0(pair, cpus),
        RISCV_HWPROBE_KEY_IMA_EXT_1 => hwprobe_isa_ext1(pair, cpus),
        RISCV_HWPROBE_KEY_CPUPERF_0 | RISCV_HWPROBE_KEY_MISALIGNED_SCALAR_PERF => (*pair).value = hwprobe_misaligned(cpus),
        RISCV_HWPROBE_KEY_MISALIGNED_VECTOR_PERF => (*pair).value = hwprobe_vec_misaligned(cpus),
        RISCV_HWPROBE_KEY_ZICBOZ_BLOCK_SIZE => { (*pair).value = if hwprobe_ext0_has(cpus, RISCV_HWPROBE_EXT_ZICBOZ) { riscv_cboz_block_size } else { 0 }; },
        RISCV_HWPROBE_KEY_ZICBOM_BLOCK_SIZE => { (*pair).value = if hwprobe_ext0_has(cpus, RISCV_HWPROBE_EXT_ZICBOM) { riscv_cbom_block_size } else { 0 }; },
        RISCV_HWPROBE_KEY_ZICBOP_BLOCK_SIZE => { (*pair).value = if hwprobe_ext0_has(cpus, RISCV_HWPROBE_EXT_ZICBOP) { riscv_cbop_block_size } else { 0 }; },
        RISCV_HWPROBE_KEY_HIGHEST_VIRT_ADDRESS => (*pair).value = user_max_virt_addr(),
        RISCV_HWPROBE_KEY_TIME_CSR_FREQ => (*pair).value = riscv_timebase,
        RISCV_HWPROBE_KEY_VENDOR_EXT_SIFIVE_0 => hwprobe_isa_vendor_ext_sifive_0(pair, cpus),
        RISCV_HWPROBE_KEY_VENDOR_EXT_THEAD_0 => hwprobe_isa_vendor_ext_thead_0(pair, cpus),
        RISCV_HWPROBE_KEY_VENDOR_EXT_MIPS_0 => hwprobe_isa_vendor_ext_mips_0(pair, cpus),
        _ => { (*pair).key = -1; (*pair).value = 0; }
    }
}

unsafe fn hwprobe_get_values(pairs: *mut riscv_hwprobe, pair_count: usize, cpusetsize: usize, cpus_user: *mut usize, flags: u32) -> i32 {
    let mut cpus = cpumask::default();
    if flags != 0 { return -EINVAL; }
    cpumask_clear(&mut cpus);
    if cpusetsize == 0 && cpus_user.is_null() { cpumask_copy(&mut cpus, cpu_online_mask); }
    else {
        let size = core::cmp::min(cpusetsize, cpumask_size());
        if copy_from_user(&mut cpus, cpus_user, size) != 0 { return -EFAULT; }
        cpumask_and(&mut cpus, &cpus, cpu_online_mask);
        if cpumask_empty(&cpus) { return -EINVAL; }
    }
    for out in 0..pair_count {
        let mut pair = riscv_hwprobe::default();
        if get_user(&mut pair.key, &(*pairs.add(out)).key) != 0 { return -EFAULT; }
        pair.value = 0; hwprobe_one_pair(&mut pair, &cpus);
        if put_user(pair.key, &mut (*pairs.add(out)).key) != 0 || put_user(pair.value, &mut (*pairs.add(out)).value) != 0 { return -EFAULT; }
    }
    0
}

unsafe fn hwprobe_get_cpus(pairs: *const riscv_hwprobe, pair_count: usize, cpusetsize: usize, cpus_user: *mut usize, flags: u32) -> i32 {
    if flags != RISCV_HWPROBE_WHICH_CPUS || cpusetsize == 0 || cpus_user.is_null() { return -EINVAL; }
    let size = core::cmp::min(cpusetsize, cpumask_size());
    let mut cpus = cpumask::default(); let mut one_cpu = cpumask::default();
    cpumask_clear(&mut cpus);
    if copy_from_user(&mut cpus, cpus_user, size) != 0 { return -EFAULT; }
    if cpumask_empty(&cpus) { cpumask_copy(&mut cpus, cpu_online_mask); }
    cpumask_and(&mut cpus, &cpus, cpu_online_mask); cpumask_clear(&mut one_cpu);
    let mut clear_all = false;
    for i in 0..pair_count {
        let mut pair = riscv_hwprobe::default();
        if copy_from_user(&mut pair, pairs.add(i), core::mem::size_of::<riscv_hwprobe>()) != 0 { return -EFAULT; }
        if !riscv_hwprobe_key_is_valid(pair.key) { clear_all = true; pair = riscv_hwprobe { key: -1, value: 0 }; if copy_to_user(pairs.add(i) as *mut _, &pair, core::mem::size_of::<riscv_hwprobe>()) != 0 { return -EFAULT; } }
        if clear_all { continue; }
        let mut tmp = riscv_hwprobe { key: pair.key, value: 0 };
        for_each_cpu!(cpu, &cpus, { cpumask_set_cpu(cpu, &mut one_cpu); hwprobe_one_pair(&mut tmp, &one_cpu); if !riscv_hwprobe_pair_cmp(&tmp, &pair) { cpumask_clear_cpu(cpu, &mut cpus); } cpumask_clear_cpu(cpu, &mut one_cpu); });
    }
    if clear_all { cpumask_clear(&mut cpus); }
    if copy_to_user(cpus_user, &cpus, size) != 0 { return -EFAULT; } 0
}

unsafe fn do_riscv_hwprobe(pairs: *mut riscv_hwprobe, pair_count: usize, cpusetsize: usize, cpus: *mut usize, flags: u32) -> i32 {
    if flags & RISCV_HWPROBE_WHICH_CPUS != 0 { hwprobe_get_cpus(pairs, pair_count, cpusetsize, cpus, flags) } else { hwprobe_get_values(pairs, pair_count, cpusetsize, cpus, flags) }
}

// CONFIG_MMU: initialize vDSO data for the all-CPUs case.
unsafe fn init_hwprobe_vdso_data() -> i32 {
    let avd = vdso_k_arch_data; let mut id_bitsmash = 0u64;
    for key in 0..=RISCV_HWPROBE_MAX_KEY { let mut pair = riscv_hwprobe { key, value: 0 }; hwprobe_one_pair(&mut pair, cpu_online_mask); WARN_ON_ONCE!(pair.key < 0); avd.all_cpu_hwprobe_values[key as usize] = pair.value; if key <= RISCV_HWPROBE_KEY_MIMPID { id_bitsmash |= pair.value; } }
    avd.homogeneous_cpus = id_bitsmash != 0 && id_bitsmash != !0; 0
}

pub unsafe fn riscv_hwprobe(pairs: *mut riscv_hwprobe, pair_count: usize, cpusetsize: usize, cpus: *mut usize, flags: u32) -> i32 {
    do_riscv_hwprobe(pairs, pair_count, cpusetsize, cpus, flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
