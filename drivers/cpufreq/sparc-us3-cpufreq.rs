// SPDX-License-Identifier: GPL-2.0-only
/* us3_cpufreq.c: UltraSPARC-III cpu frequency support
 *
 * Copyright (C) 2003 David S. Miller (davem@redhat.com)
 *
 * Many thanks to Dominik Brodowski for fixing up the cpufreq
 * infrastructure in order to make this driver easier to implement.
 */

// Linux and architecture headers provide the declarations referenced below.

#[repr(C)]
struct us3_freq_percpu_info {
    table: [cpufreq_frequency_table; 4],
}

/* Indexed by cpu number. */
static mut us3_freq_table: *mut us3_freq_percpu_info = core::ptr::null_mut();

/* UltraSPARC-III has three dividers: 1, 2, and 32.  These are controlled
 * in the Safari config register.
 */
const SAFARI_CFG_DIV_1:    u64 = 0x0000000000000000;
const SAFARI_CFG_DIV_2:    u64 = 0x0000000040000000;
const SAFARI_CFG_DIV_32:   u64 = 0x0000000080000000;
const SAFARI_CFG_DIV_MASK: u64 = 0x00000000C0000000;

unsafe fn read_safari_cfg(arg: *mut core::ffi::c_void) {
    let val = arg as *mut u64;
    let ret: u64;
    core::arch::asm!(
        "ldxa [%%g0] {asi}, {ret}",
        asi = const ASI_SAFARI_CONFIG,
        ret = lateout(reg) ret,
    );
    *val = ret;
}

unsafe fn update_safari_cfg(arg: *mut core::ffi::c_void) {
    let new_bits = arg as *mut u64;
    let mut reg: u64 = 0;

    read_safari_cfg((&mut reg as *mut u64).cast());
    reg &= !SAFARI_CFG_DIV_MASK;
    reg |= *new_bits;

    core::arch::asm!(
        "stxa {reg}, [%%g0] {asi}\n\t",
        "membar #Sync",
        reg = in(reg) reg,
        asi = const ASI_SAFARI_CONFIG,
        options(nostack),
    );
}

unsafe fn get_current_freq(cpu: u32, safari_cfg: u64) -> u64 {
    let clock_tick = sparc64_get_clock_tick(cpu) / 1000;
    match safari_cfg & SAFARI_CFG_DIV_MASK {
        SAFARI_CFG_DIV_1 => clock_tick / 1,
        SAFARI_CFG_DIV_2 => clock_tick / 2,
        SAFARI_CFG_DIV_32 => clock_tick / 32,
        _ => BUG(),
    }
}

unsafe fn us3_freq_get(cpu: u32) -> u32 {
    let mut reg: u64 = 0;
    if smp_call_function_single(cpu, Some(read_safari_cfg), (&mut reg as *mut u64).cast(), 1) != 0 {
        return 0;
    }
    get_current_freq(cpu, reg) as u32
}

unsafe fn us3_freq_target(policy: *mut cpufreq_policy, index: u32) -> i32 {
    let cpu = (*policy).cpu;
    let mut new_bits: u64;
    let mut new_freq = sparc64_get_clock_tick(cpu) / 1000;

    match index {
        0 => { new_bits = SAFARI_CFG_DIV_1; new_freq /= 1; }
        1 => { new_bits = SAFARI_CFG_DIV_2; new_freq /= 2; }
        2 => { new_bits = SAFARI_CFG_DIV_32; new_freq /= 32; }
        _ => BUG(),
    }
    let _ = new_freq;
    smp_call_function_single(cpu, Some(update_safari_cfg), (&mut new_bits as *mut u64).cast(), 1)
}

unsafe fn us3_freq_cpu_init(policy: *mut cpufreq_policy) -> i32 {
    let cpu = (*policy).cpu;
    let clock_tick = sparc64_get_clock_tick(cpu) / 1000;
    let table = &mut (*us3_freq_table.add(cpu as usize)).table[0] as *mut cpufreq_frequency_table;

    (*table.add(0)).driver_data = 0;
    (*table.add(0)).frequency = clock_tick / 1;
    (*table.add(1)).driver_data = 1;
    (*table.add(1)).frequency = clock_tick / 2;
    (*table.add(2)).driver_data = 2;
    (*table.add(2)).frequency = clock_tick / 32;
    (*table.add(3)).driver_data = 0;
    (*table.add(3)).frequency = CPUFREQ_TABLE_END;

    (*policy).cpuinfo.transition_latency = 0;
    (*policy).cur = clock_tick as u32;
    (*policy).freq_table = table;
    0
}

unsafe fn us3_freq_cpu_exit(policy: *mut cpufreq_policy) {
    us3_freq_target(policy, 0);
}

static mut cpufreq_us3_driver: cpufreq_driver = cpufreq_driver {
    name: b"UltraSPARC-III\0".as_ptr() as *const _,
    init: Some(us3_freq_cpu_init),
    verify: Some(cpufreq_generic_frequency_table_verify),
    target_index: Some(us3_freq_target),
    get: Some(us3_freq_get),
    exit: Some(us3_freq_cpu_exit),
};

unsafe fn us3_freq_init() -> i32 {
    let mut manuf: u64;
    let mut impl_: u64;
    let ver: u64;
    let mut ret: i32;

    if tlb_type != cheetah && tlb_type != cheetah_plus { return -ENODEV; }

    core::arch::asm!("rdpr %ver, {ver}", ver = lateout(reg) ver);
    manuf = (ver >> 48) & 0xffff;
    impl_ = (ver >> 32) & 0xffff;

    if manuf == CHEETAH_MANUF &&
       (impl_ == CHEETAH_IMPL || impl_ == CHEETAH_PLUS_IMPL ||
        impl_ == JAGUAR_IMPL || impl_ == PANTHER_IMPL) {
        us3_freq_table = kzalloc_objs::<us3_freq_percpu_info>(NR_CPUS);
        if us3_freq_table.is_null() { return -ENOMEM; }

        ret = cpufreq_register_driver(&mut cpufreq_us3_driver);
        if ret != 0 { kfree(us3_freq_table.cast()); }
        return ret;
    }
    -ENODEV
}

unsafe fn us3_freq_exit() {
    cpufreq_unregister_driver(&mut cpufreq_us3_driver);
    kfree(us3_freq_table.cast());
}

// MODULE_AUTHOR("David S. Miller <davem@redhat.com>");
// MODULE_DESCRIPTION("cpufreq driver for UltraSPARC-III");
// MODULE_LICENSE("GPL");
// module_init(us3_freq_init);
// module_exit(us3_freq_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
