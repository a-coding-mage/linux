// SPDX-License-Identifier: GPL-2.0-or-later
/* KVM paravirtual clock driver. A clocksource implementation
 * Copyright (C) 2008 Glauber de Oliveira Costa, Red Hat Inc.
 */

// Kernel dependencies supplied by the surrounding translation unit.

static mut kvmclock: i32 = 1;
static mut kvmclock_vsyscall: i32 = 1;
static mut msr_kvm_system_time: i32 = 0;
static mut msr_kvm_wall_clock: i32 = 0;
static mut kvm_sched_clock_offset: u64 = 0;

unsafe fn parse_no_kvmclock(_arg: *mut i8) -> i32 {
    kvmclock = 0;
    0
}

unsafe fn parse_no_kvmclock_vsyscall(_arg: *mut i8) -> i32 {
    kvmclock_vsyscall = 0;
    0
}

// HVC_BOOT_ARRAY_SIZE = PAGE_SIZE / sizeof(struct pvclock_vsyscall_time_info)
const HVC_BOOT_ARRAY_SIZE: usize = PAGE_SIZE / core::mem::size_of::<pvclock_vsyscall_time_info>();

static mut hv_clock_boot: [pvclock_vsyscall_time_info; HVC_BOOT_ARRAY_SIZE] =
    [pvclock_vsyscall_time_info::ZERO; HVC_BOOT_ARRAY_SIZE];
static mut wall_clock: pvclock_wall_clock = pvclock_wall_clock::ZERO;
static mut hvclock_mem: *mut pvclock_vsyscall_time_info = core::ptr::null_mut();

// DEFINE_PER_CPU(struct pvclock_vsyscall_time_info *, hv_clock_per_cpu)
extern "C" {
    static mut hv_clock_per_cpu: *mut pvclock_vsyscall_time_info;
}

unsafe fn kvm_get_wallclock(now: *mut timespec64) {
    wrmsrq(msr_kvm_wall_clock, slow_virt_to_phys(&raw mut wall_clock));
    preempt_disable();
    pvclock_read_wallclock(&raw mut wall_clock, this_cpu_pvti(), now);
    preempt_enable();
}

unsafe fn kvm_set_wallclock(_now: *const timespec64) -> i32 { -ENODEV }

unsafe fn kvm_clock_read() -> u64 {
    preempt_disable_notrace();
    let ret = pvclock_clocksource_read_nowd(this_cpu_pvti());
    preempt_enable_notrace();
    ret
}

unsafe fn kvm_clock_get_cycles(_cs: *mut clocksource) -> u64 { kvm_clock_read() }

unsafe fn kvm_clock_get_cycles_snapshot(
    _cs: *mut clocksource,
    chs: *mut clocksource_hw_snapshot,
) -> u64 {
    let src: *mut pvclock_vcpu_time_info = this_cpu_pvti();
    let (ret, tsc);
    preempt_disable_notrace();
    loop {
        let version = pvclock_read_begin(src);
        tsc = rdtsc_ordered();
        ret = __pvclock_read_cycles(src, tsc);
        if !pvclock_read_retry(src, version) { break; }
    }
    preempt_enable_notrace();
    (*chs).hw_cycles = tsc;
    (*chs).hw_csid = CSID_X86_TSC;
    ret
}

unsafe fn kvm_sched_clock_read() -> u64 {
    pvclock_clocksource_read_nowd(this_cpu_pvti()).wrapping_sub(kvm_sched_clock_offset)
}

unsafe fn kvm_sched_clock_init(stable: bool) {
    if !stable { clear_sched_clock_stable(); }
    kvm_sched_clock_offset = kvm_clock_read();
    paravirt_set_sched_clock(kvm_sched_clock_read);
    pr_info!("kvm-clock: using sched offset of %llu cycles", kvm_sched_clock_offset);
}

unsafe fn kvm_get_tsc_khz() -> c_ulong {
    setup_force_cpu_cap(X86_FEATURE_TSC_KNOWN_FREQ);
    pvclock_tsc_khz(this_cpu_pvti())
}

unsafe fn kvm_get_preset_lpj() {
    let khz = kvm_get_tsc_khz();
    let mut lpj = (khz as u64).wrapping_mul(1000);
    lpj /= HZ as u64;
    preset_lpj = lpj;
}

pub unsafe fn kvm_check_and_clear_guest_paused() -> bool {
    let src = this_cpu_hvclock();
    if src.is_null() { return false; }
    if ((*src).pvti.flags & PVCLOCK_GUEST_STOPPED) != 0 {
        (*src).pvti.flags &= !PVCLOCK_GUEST_STOPPED;
        pvclock_touch_watchdogs();
        return true;
    }
    false
}

unsafe fn kvm_cs_enable(_cs: *mut clocksource) -> i32 {
    vclocks_set_used(VDSO_CLOCKMODE_PVCLOCK);
    0
}

static mut kvm_clock: clocksource = clocksource {
    name: b"kvm-clock\0".as_ptr() as *const i8,
    read: Some(kvm_clock_get_cycles),
    read_snapshot: Some(kvm_clock_get_cycles_snapshot),
    rating: 400,
    mask: u64::MAX,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    id: CSID_X86_KVM_CLK,
    enable: Some(kvm_cs_enable),
    ..clocksource::ZERO
};

unsafe fn kvm_register_clock(txt: *mut i8) {
    let src = this_cpu_hvclock();
    if src.is_null() { return; }
    let pa = slow_virt_to_phys(&raw mut (*src).pvti) | 0x01u64;
    wrmsrq(msr_kvm_system_time, pa);
    pr_debug!("kvm-clock: cpu %d, msr %llx, %s", smp_processor_id(), pa, txt);
}

unsafe fn kvm_save_sched_clock_state() {}

unsafe fn kvm_restore_sched_clock_state() {
    kvm_register_clock(b"primary cpu clock, resume\0".as_ptr() as *mut i8);
}

#[cfg(CONFIG_X86_LOCAL_APIC)]
unsafe fn kvm_setup_secondary_clock() {
    kvm_register_clock(b"secondary cpu clock\0".as_ptr() as *mut i8);
}

pub unsafe fn kvmclock_disable() {
    if msr_kvm_system_time != 0 { native_write_msr(msr_kvm_system_time, 0); }
}

unsafe fn kvmclock_init_mem() {
    let ncpus: usize;
    let order: u32;
    let p: *mut page;
    let mut r: i32;
    if HVC_BOOT_ARRAY_SIZE >= num_possible_cpus() { return; }
    ncpus = num_possible_cpus() - HVC_BOOT_ARRAY_SIZE;
    order = get_order((ncpus * core::mem::size_of::<*mut pvclock_vsyscall_time_info>()) as c_ulong);
    p = alloc_pages(GFP_KERNEL, order);
    if p.is_null() { pr_warn!("%s: failed to alloc %d pages", __func__, 1u32 << order); return; }
    hvclock_mem = page_address(p);
    if cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) {
        r = set_memory_decrypted(hvclock_mem as c_ulong, 1u64 << order);
        if r != 0 { __free_pages(p, order); hvclock_mem = core::ptr::null_mut(); pr_warn!("kvmclock: set_memory_decrypted() failed. Disabling\n"); return; }
    }
    memset(hvclock_mem as *mut _, 0, PAGE_SIZE << order);
}

unsafe fn kvm_setup_vsyscall_timeinfo() -> i32 {
    if !kvm_para_available() || kvmclock == 0 || nopv { return 0; }
    kvmclock_init_mem();
    #[cfg(CONFIG_X86_64)]
    if !per_cpu(hv_clock_per_cpu, 0).is_null() && kvmclock_vsyscall != 0 {
        let flags = pvclock_read_flags(&raw mut hv_clock_boot[0].pvti);
        if (flags & PVCLOCK_TSC_STABLE_BIT) == 0 { return 0; }
        kvm_clock.vdso_clock_mode = VDSO_CLOCKMODE_PVCLOCK;
    }
    0
}

unsafe fn kvmclock_setup_percpu(cpu: u32) -> i32 {
    let mut p = per_cpu(hv_clock_per_cpu, cpu);
    if cpu == 0 || (!p.is_null() && p != per_cpu(hv_clock_per_cpu, 0)) { return 0; }
    if (cpu as usize) < HVC_BOOT_ARRAY_SIZE { p = &raw mut hv_clock_boot[cpu as usize]; }
    else if !hvclock_mem.is_null() { p = hvclock_mem.add(cpu as usize - HVC_BOOT_ARRAY_SIZE); }
    else { return -ENOMEM; }
    per_cpu(hv_clock_per_cpu, cpu) = p;
    if !p.is_null() { 0 } else { -ENOMEM }
}

pub unsafe fn kvmclock_init() {
    if !kvm_para_available() || kvmclock == 0 { return; }
    if kvm_para_has_feature(KVM_FEATURE_CLOCKSOURCE2) { msr_kvm_system_time = MSR_KVM_SYSTEM_TIME_NEW; msr_kvm_wall_clock = MSR_KVM_WALL_CLOCK_NEW; }
    else if kvm_para_has_feature(KVM_FEATURE_CLOCKSOURCE) { msr_kvm_system_time = MSR_KVM_SYSTEM_TIME; msr_kvm_wall_clock = MSR_KVM_WALL_CLOCK; }
    else { return; }
    if cpuhp_setup_state(CPUHP_BP_PREPARE_DYN, b"kvmclock:setup_percpu\0".as_ptr() as *const i8, kvmclock_setup_percpu, None) < 0 { return; }
    pr_info!("kvm-clock: Using msrs %x and %x", msr_kvm_system_time, msr_kvm_wall_clock);
    this_cpu_write(hv_clock_per_cpu, &raw mut hv_clock_boot[0]);
    kvm_register_clock(b"primary cpu clock\0".as_ptr() as *mut i8);
    pvclock_set_pvti_cpu0_va(&raw mut hv_clock_boot);
    if kvm_para_has_feature(KVM_FEATURE_CLOCKSOURCE_STABLE_BIT) { pvclock_set_flags(PVCLOCK_TSC_STABLE_BIT); }
    let flags = pvclock_read_flags(&raw mut hv_clock_boot[0].pvti);
    kvm_sched_clock_init((flags & PVCLOCK_TSC_STABLE_BIT) != 0);
    x86_platform.calibrate_tsc = Some(kvm_get_tsc_khz);
    x86_platform.calibrate_cpu = Some(kvm_get_tsc_khz);
    x86_platform.get_wallclock = Some(kvm_get_wallclock);
    x86_platform.set_wallclock = Some(kvm_set_wallclock);
    x86_platform.save_sched_clock_state = Some(kvm_save_sched_clock_state);
    x86_platform.restore_sched_clock_state = Some(kvm_restore_sched_clock_state);
    kvm_get_preset_lpj();
    if boot_cpu_has(X86_FEATURE_CONSTANT_TSC) && boot_cpu_has(X86_FEATURE_NONSTOP_TSC) && !check_tsc_unstable() { kvm_clock.rating = 299; }
    clocksource_register_hz(&raw mut kvm_clock, NSEC_PER_SEC);
    pv_info.name = b"KVM\0".as_ptr() as *const i8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
