// SPDX-License-Identifier: GPL-2.0-only
/* Record and handle CPU attributes. */

// C headers and configuration-dependent declarations are supplied by the
// surrounding kernel translation unit.

static mut CPU_DATA: PerCpu<cpuinfo_arm64> = DEFINE_PER_CPU!();
static mut BOOT_CPU_DATA: cpuinfo_arm64 = cpuinfo_arm64::zeroed();

#[inline]
unsafe fn icache_policy_str(l1ip: i32) -> &'static str {
    match l1ip {
        CTR_EL0_L1Ip_VIPT => "VIPT",
        CTR_EL0_L1Ip_PIPT => "PIPT",
        _ => "RESERVED/UNKNOWN",
    }
}

static mut __ICACHE_FLAGS: c_ulong = 0;

static HWCAP_STR: &[Option<&str>] = &[
    Some("fp"), Some("asimd"), Some("evtstrm"), Some("aes"), Some("pmull"),
    Some("sha1"), Some("sha2"), Some("crc32"), Some("atomics"), Some("fphp"),
    Some("asimdhp"), Some("cpuid"), Some("asimdrdm"), Some("jscvt"), Some("fcma"),
    Some("lrcpc"), Some("dcpop"), Some("sha3"), Some("sm3"), Some("sm4"),
    Some("asimddp"), Some("sha512"), Some("sve"), Some("asimdfhm"), Some("dit"),
    Some("uscat"), Some("ilrcpc"), Some("flagm"), Some("ssbs"), Some("sb"),
    Some("paca"), Some("pacg"), Some("gcs"), Some("ls64"), Some("dcpodp"),
    Some("sve2"), Some("sveaes"), Some("svepmull"), Some("svebitperm"), Some("svesha3"),
    Some("svesm4"), Some("flagm2"), Some("frint"), Some("svei8mm"), Some("svef32mm"),
    Some("svef64mm"), Some("svebf16"), Some("i8mm"), Some("bf16"), Some("dgh"),
    Some("rng"), Some("bti"), Some("mte"), Some("ecv"), Some("afp"), Some("rpres"),
    Some("mte3"), Some("sme"), Some("smei16i64"), Some("smef64f64"), Some("smei8i32"),
    Some("smef16f32"), Some("smeb16f32"), Some("smef32f32"), Some("smefa64"), Some("wfxt"),
    Some("ebf16"), Some("sveebf16"), Some("cssc"), Some("rprfm"), Some("sve2p1"),
    Some("sme2"), Some("sme2p1"), Some("smei16i32"), Some("smebi32i32"), Some("smeb16b16"),
    Some("smef16f16"), Some("mops"), Some("hbc"), Some("sveb16b16"), Some("lrcpc3"),
    Some("lse128"), Some("fpmr"), Some("lut"), Some("faminmax"), Some("f8cvt"),
    Some("f8fma"), Some("f8dp4"), Some("f8dp2"), Some("f8e4m3"), Some("f8e5m2"),
    Some("smelutv2"), Some("smef8f16"), Some("smef8f32"), Some("smesf8fma"), Some("smesf8dp4"),
    Some("smesf8dp2"), Some("poe"), Some("cmpbr"), Some("fprcvt"), Some("f8mm8"),
    Some("f8mm4"), Some("svef16mm"), Some("sveeltperm"), Some("sveaes2"), Some("svebfscale"),
    Some("sve2p2"), Some("sme2p2"), Some("smesbitperm"), Some("smeaes"), Some("smesfexpa"),
    Some("smestmop"), Some("smesmop4"), Some("mtefar"), Some("mtestoreonly"), Some("lsfe"),
    Some("sveb16mm"), Some("sve2p3"), Some("smelut6"), Some("sme2p3"), Some("f16mm"),
    Some("f16f32dot"), Some("f16f32mm"), Some("svelut6"),
];

unsafe fn c_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let cpu = (*m).index;
    let compat = personality(current().personality) == PER_LINUX32;
    let cpuinfo = _v as *mut cpuinfo_arm64;
    let midr = (*cpuinfo).reg_midr;
    seq_printf(m, "processor\t: %d\n", cpu);
    if compat { seq_printf(m, "model name\t: ARMv8 Processor rev %d (%s)\n", MIDR_REVISION(midr), COMPAT_ELF_PLATFORM); }
    seq_printf(m, "BogoMIPS\t: %lu.%02lu\n", loops_per_jiffy / (500000UL / HZ), (loops_per_jiffy / (5000UL / HZ)) % 100);
    seq_puts(m, "Features\t:");
    if !compat {
        for j in 0..HWCAP_STR.len() { if cpu_have_feature(j) { if let Some(s) = HWCAP_STR[j] { seq_printf(m, " %s", s); } } }
    }
    seq_puts(m, "\n");
    seq_printf(m, "CPU implementer\t: 0x%02x\n", MIDR_IMPLEMENTOR(midr));
    seq_puts(m, "CPU architecture: 8\n");
    seq_printf(m, "CPU variant\t: 0x%x\n", MIDR_VARIANT(midr));
    seq_printf(m, "CPU part\t: 0x%03x\n", MIDR_PARTNUM(midr));
    seq_printf(m, "CPU revision\t: %d\n\n", MIDR_REVISION(midr));
    0
}

unsafe fn c_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    *pos = cpumask_next(*pos - 1, cpu_online_mask);
    if *pos < nr_cpu_ids { &mut per_cpu!(CPU_DATA, *pos) as *mut _ as *mut c_void } else { core::ptr::null_mut() }
}
unsafe fn c_next(m: *mut seq_file, _v: *mut c_void, pos: *mut loff_t) -> *mut c_void { *pos += 1; c_start(m, pos) }
unsafe fn c_stop(_m: *mut seq_file, _v: *mut c_void) {}

pub static CPUINFO_OP: seq_operations = seq_operations { start: Some(c_start), next: Some(c_next), stop: Some(c_stop), show: Some(c_show) };

unsafe fn __cpuinfo_store_cpu_32bit(info: *mut cpuinfo_32bit) {
    (*info).reg_id_dfr0 = read_cpuid(ID_DFR0_EL1); (*info).reg_id_dfr1 = read_cpuid(ID_DFR1_EL1);
    (*info).reg_id_isar0 = read_cpuid(ID_ISAR0_EL1); (*info).reg_id_isar1 = read_cpuid(ID_ISAR1_EL1);
    (*info).reg_id_isar2 = read_cpuid(ID_ISAR2_EL1); (*info).reg_id_isar3 = read_cpuid(ID_ISAR3_EL1);
    (*info).reg_id_isar4 = read_cpuid(ID_ISAR4_EL1); (*info).reg_id_isar5 = read_cpuid(ID_ISAR5_EL1);
    (*info).reg_id_isar6 = read_cpuid(ID_ISAR6_EL1); (*info).reg_id_mmfr0 = read_cpuid(ID_MMFR0_EL1);
    (*info).reg_id_mmfr1 = read_cpuid(ID_MMFR1_EL1); (*info).reg_id_mmfr2 = read_cpuid(ID_MMFR2_EL1);
    (*info).reg_id_mmfr3 = read_cpuid(ID_MMFR3_EL1); (*info).reg_id_mmfr4 = read_cpuid(ID_MMFR4_EL1);
    (*info).reg_id_mmfr5 = read_cpuid(ID_MMFR5_EL1); (*info).reg_id_pfr0 = read_cpuid(ID_PFR0_EL1);
    (*info).reg_id_pfr1 = read_cpuid(ID_PFR1_EL1); (*info).reg_id_pfr2 = read_cpuid(ID_PFR2_EL1);
    (*info).reg_mvfr0 = read_cpuid(MVFR0_EL1); (*info).reg_mvfr1 = read_cpuid(MVFR1_EL1);
    (*info).reg_mvfr2 = read_cpuid(MVFR2_EL1);
}

unsafe fn cpuid_cpu_online(cpu: c_uint) -> c_int {
    let info = per_cpu_ptr(&raw mut CPU_DATA, cpu);
    let dev = get_cpu_device(cpu); if dev.is_null() { return -ENODEV; }
    let mut rc = kobject_add(&mut (*info).kobj, &mut (*dev).kobj, c"regs".as_ptr());
    if rc != 0 { return rc; }
    rc = sysfs_create_group(&mut (*info).kobj, &cpuregs_attr_group);
    if rc != 0 { kobject_del(&mut (*info).kobj); }
    if system_supports_sme() { rc = sysfs_merge_group(&mut (*info).kobj, &sme_cpuregs_attr_group); }
    rc
}
unsafe fn cpuid_cpu_offline(cpu: c_uint) -> c_int {
    let info = per_cpu_ptr(&raw mut CPU_DATA, cpu); let dev = get_cpu_device(cpu);
    if dev.is_null() { return -ENODEV; }
    if !(*info).kobj.parent.is_null() { sysfs_remove_group(&mut (*info).kobj, &cpuregs_attr_group); kobject_del(&mut (*info).kobj); }
    0
}
unsafe fn cpuinfo_regs_init() -> c_int {
    for_each_possible_cpu!(cpu, { let info = per_cpu_ptr(&raw mut CPU_DATA, cpu); kobject_init(&mut (*info).kobj, &cpuregs_kobj_type); });
    let ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, c"arm64/cpuinfo:online".as_ptr(), cpuid_cpu_online, cpuid_cpu_offline);
    if ret < 0 { pr_err!("cpuinfo: failed to register hotplug callbacks.\n"); return ret; } 0
}

unsafe fn cpuinfo_detect_icache_policy(info: *mut cpuinfo_arm64) {
    let cpu = smp_processor_id();
    let l1ip = CTR_L1IP((*info).reg_ctr);
    if l1ip != CTR_EL0_L1Ip_PIPT { set_bit(ICACHEF_ALIASING, &mut __ICACHE_FLAGS); }
    pr_info!("Detected %s I-cache on CPU%d\n", icache_policy_str(l1ip), cpu);
}

unsafe fn __cpuinfo_store_cpu(info: *mut cpuinfo_arm64) {
    (*info).reg_cntfrq = arch_timer_get_cntfrq();
    (*info).reg_ctr = read_cpuid_effective_cachetype();
    (*info).reg_dczid = read_cpuid(DCZID_EL0);
    (*info).reg_midr = read_cpuid_id();
    (*info).reg_revidr = read_cpuid(REVIDR_EL1);
    (*info).reg_aidr = read_cpuid(AIDR_EL1);
    (*info).reg_id_aa64dfr0 = read_cpuid(ID_AA64DFR0_EL1);
    (*info).reg_id_aa64dfr1 = read_cpuid(ID_AA64DFR1_EL1);
    (*info).reg_id_aa64isar0 = read_cpuid(ID_AA64ISAR0_EL1);
    (*info).reg_id_aa64isar1 = read_cpuid(ID_AA64ISAR1_EL1);
    (*info).reg_id_aa64isar2 = read_cpuid(ID_AA64ISAR2_EL1);
    (*info).reg_id_aa64isar3 = read_cpuid(ID_AA64ISAR3_EL1);
    (*info).reg_id_aa64mmfr0 = read_cpuid(ID_AA64MMFR0_EL1);
    (*info).reg_id_aa64mmfr1 = read_cpuid(ID_AA64MMFR1_EL1);
    (*info).reg_id_aa64mmfr2 = read_cpuid(ID_AA64MMFR2_EL1);
    (*info).reg_id_aa64mmfr3 = read_cpuid(ID_AA64MMFR3_EL1);
    (*info).reg_id_aa64mmfr4 = read_cpuid(ID_AA64MMFR4_EL1);
    (*info).reg_id_aa64pfr0 = read_cpuid(ID_AA64PFR0_EL1);
    (*info).reg_id_aa64pfr1 = read_cpuid(ID_AA64PFR1_EL1);
    (*info).reg_id_aa64pfr2 = read_cpuid(ID_AA64PFR2_EL1);
    (*info).reg_id_aa64zfr0 = read_cpuid(ID_AA64ZFR0_EL1);
    (*info).reg_id_aa64smfr0 = read_cpuid(ID_AA64SMFR0_EL1);
    (*info).reg_id_aa64fpfr0 = read_cpuid(ID_AA64FPFR0_EL1);
    if id_aa64pfr1_mte((*info).reg_id_aa64pfr1) { (*info).reg_gmid = read_cpuid(GMID_EL1); }
    if id_aa64pfr0_32bit_el0((*info).reg_id_aa64pfr0) { __cpuinfo_store_cpu_32bit(&mut (*info).aarch32); }
    if IS_ENABLED!(CONFIG_ARM64_SME) && id_aa64pfr1_sme((*info).reg_id_aa64pfr1) { (*info).reg_smidr = read_cpuid(SMIDR_EL1) & !SMIDR_EL1_SMPS; }
    cpuinfo_detect_icache_policy(info);
}

pub unsafe fn cpuinfo_store_cpu() {
    let info = this_cpu_ptr(&raw mut CPU_DATA);
    __cpuinfo_store_cpu(info);
    update_cpu_features(smp_processor_id(), info, &raw mut BOOT_CPU_DATA);
}

pub unsafe fn cpuinfo_store_boot_cpu() {
    let info = per_cpu_ptr(&raw mut CPU_DATA, 0);
    __cpuinfo_store_cpu(info);
    BOOT_CPU_DATA = *info;
    init_cpu_features(&raw mut BOOT_CPU_DATA);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
