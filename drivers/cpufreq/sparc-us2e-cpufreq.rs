// SPDX-License-Identifier: GPL-2.0-only
/* us2e_cpufreq.c: UltraSPARC-IIe cpu frequency support
 *
 * Copyright (C) 2003 David S. Miller (davem@redhat.com)
 *
 * Many thanks to Dominik Brodowski for fixing up the cpufreq
 * infrastructure in order to make this driver easier to implement.
 */

// Kernel and architecture dependencies supplied by other translation units.

#[repr(C)]
struct us2e_freq_percpu_info {
    table: [cpufreq_frequency_table; 6],
}

/* Indexed by cpu number. */
static mut us2e_freq_table: *mut us2e_freq_percpu_info = core::ptr::null_mut();

const HBIRD_MEM_CNTL0_ADDR: usize = 0x1fe0000f010;
const HBIRD_ESTAR_MODE_ADDR: usize = 0x1fe0000f080;

/* UltraSPARC-IIe has five dividers: 1, 2, 4, 6, and 8.  These are controlled
 * in the ESTAR mode control register.
 */
const ESTAR_MODE_DIV_1: usize = 0x0000000000000000;
const ESTAR_MODE_DIV_2: usize = 0x0000000000000001;
const ESTAR_MODE_DIV_4: usize = 0x0000000000000003;
const ESTAR_MODE_DIV_6: usize = 0x0000000000000002;
const ESTAR_MODE_DIV_8: usize = 0x0000000000000004;
const ESTAR_MODE_DIV_MASK: usize = 0x0000000000000007;

const MCTRL0_SREFRESH_ENAB: usize = 0x0000000000010000;
const MCTRL0_REFR_COUNT_MASK: usize = 0x0000000000007f00;
const MCTRL0_REFR_COUNT_SHIFT: usize = 8;
const MCTRL0_REFR_INTERVAL: usize = 7800;
const MCTRL0_REFR_CLKS_P_CNT: usize = 64;

extern "C" {
    fn udelay(usecs: usize);
    fn sparc64_get_clock_tick(cpu: u32) -> usize;
    fn smp_processor_id() -> u32;
    fn smp_call_function_single(cpu: u32, func: unsafe extern "C" fn(*mut core::ffi::c_void), info: *mut core::ffi::c_void, wait: i32) -> i32;
    fn BUG() -> !;
}

#[repr(C)]
struct cpufreq_frequency_table {
    driver_data: u32,
    frequency: u32,
}

#[repr(C)]
struct cpufreq_policy {
    cpu: u32,
    cpuinfo: cpufreq_cpuinfo,
    cur: u32,
    freq_table: *mut cpufreq_frequency_table,
}

#[repr(C)]
struct cpufreq_cpuinfo {
    transition_latency: u32,
}

const CPUFREQ_TABLE_END: u32 = u32::MAX;

unsafe fn read_hbreg(addr: usize) -> usize {
    let ret: usize;
    core::arch::asm!(
        "ldxa [{addr}] %asi, {ret}",
        addr = in(reg) addr,
        ret = lateout(reg) ret,
        options(nostack, preserves_flags)
    );
    ret
}

unsafe fn write_hbreg(addr: usize, val: usize) {
    core::arch::asm!(
        "stxa {val}, [{addr}] %asi\n\tmembar #Sync",
        val = in(reg) val,
        addr = in(reg) addr,
        options(nostack)
    );
    if addr == HBIRD_ESTAR_MODE_ADDR {
        /* Need to wait 16 clock cycles for the PLL to lock.  */
        udelay(1);
    }
}

unsafe fn self_refresh_ctl(enable: i32) {
    let mut mctrl = read_hbreg(HBIRD_MEM_CNTL0_ADDR);
    if enable != 0 {
        mctrl |= MCTRL0_SREFRESH_ENAB;
    } else {
        mctrl &= !MCTRL0_SREFRESH_ENAB;
    }
    write_hbreg(HBIRD_MEM_CNTL0_ADDR, mctrl);
    let _ = read_hbreg(HBIRD_MEM_CNTL0_ADDR);
}

unsafe fn frob_mem_refresh(cpu_slowing_down: i32, clock_tick: usize, old_divisor: usize, divisor: usize) {
    let mut refr_count = clock_tick * MCTRL0_REFR_INTERVAL;
    refr_count /= MCTRL0_REFR_CLKS_P_CNT * divisor * 1_000_000_000usize;

    let mut mctrl = read_hbreg(HBIRD_MEM_CNTL0_ADDR);
    let old_refr_count = (mctrl & MCTRL0_REFR_COUNT_MASK) >> MCTRL0_REFR_COUNT_SHIFT;
    mctrl &= !MCTRL0_REFR_COUNT_MASK;
    mctrl |= refr_count << MCTRL0_REFR_COUNT_SHIFT;
    write_hbreg(HBIRD_MEM_CNTL0_ADDR, mctrl);
    mctrl = read_hbreg(HBIRD_MEM_CNTL0_ADDR);

    if cpu_slowing_down != 0 && (mctrl & MCTRL0_SREFRESH_ENAB) == 0 {
        /* We have to wait for both refresh counts (old
         * and new) to go to zero.
         */
        let usecs = (MCTRL0_REFR_CLKS_P_CNT * (refr_count + old_refr_count) * 1_000_000usize * old_divisor) / clock_tick;
        udelay(usecs + 1);
    }
}

unsafe fn us2e_transition(mut estar: usize, new_bits: usize, clock_tick: usize, old_divisor: usize, divisor: usize) {
    estar &= !ESTAR_MODE_DIV_MASK;
    /* This is based upon the state transition diagram in the IIe manual.  */
    if old_divisor == 2 && divisor == 1 {
        self_refresh_ctl(0);
        write_hbreg(HBIRD_ESTAR_MODE_ADDR, estar | new_bits);
        frob_mem_refresh(0, clock_tick, old_divisor, divisor);
    } else if old_divisor == 1 && divisor == 2 {
        frob_mem_refresh(1, clock_tick, old_divisor, divisor);
        write_hbreg(HBIRD_ESTAR_MODE_ADDR, estar | new_bits);
        self_refresh_ctl(1);
    } else if old_divisor == 1 && divisor > 2 {
        us2e_transition(estar, ESTAR_MODE_DIV_2, clock_tick, 1, 2);
        us2e_transition(estar, new_bits, clock_tick, 2, divisor);
    } else if old_divisor > 2 && divisor == 1 {
        us2e_transition(estar, ESTAR_MODE_DIV_2, clock_tick, old_divisor, 2);
        us2e_transition(estar, new_bits, clock_tick, 2, divisor);
    } else if old_divisor < divisor {
        frob_mem_refresh(0, clock_tick, old_divisor, divisor);
        write_hbreg(HBIRD_ESTAR_MODE_ADDR, estar | new_bits);
    } else if old_divisor > divisor {
        write_hbreg(HBIRD_ESTAR_MODE_ADDR, estar | new_bits);
        frob_mem_refresh(1, clock_tick, old_divisor, divisor);
    } else {
        BUG();
    }
}

unsafe fn index_to_estar_mode(index: u32) -> usize {
    match index { 0 => ESTAR_MODE_DIV_1, 1 => ESTAR_MODE_DIV_2, 2 => ESTAR_MODE_DIV_4, 3 => ESTAR_MODE_DIV_6, 4 => ESTAR_MODE_DIV_8, _ => BUG() }
}

unsafe fn index_to_divisor(index: u32) -> usize {
    match index { 0 => 1, 1 => 2, 2 => 4, 3 => 6, 4 => 8, _ => BUG() }
}

unsafe fn estar_to_divisor(estar: usize) -> usize {
    match estar & ESTAR_MODE_DIV_MASK { ESTAR_MODE_DIV_1 => 1, ESTAR_MODE_DIV_2 => 2, ESTAR_MODE_DIV_4 => 4, ESTAR_MODE_DIV_6 => 6, ESTAR_MODE_DIV_8 => 8, _ => BUG() }
}

unsafe extern "C" fn __us2e_freq_get(arg: *mut core::ffi::c_void) {
    *(arg as *mut usize) = read_hbreg(HBIRD_ESTAR_MODE_ADDR);
}

unsafe fn us2e_freq_get(cpu: u32) -> u32 {
    let clock_tick = sparc64_get_clock_tick(cpu) / 1000;
    let mut estar = 0usize;
    if smp_call_function_single(cpu, __us2e_freq_get, (&mut estar as *mut usize).cast(), 1) != 0 { return 0; }
    (clock_tick / estar_to_divisor(estar)) as u32
}

unsafe extern "C" fn __us2e_freq_target(arg: *mut core::ffi::c_void) {
    let cpu = smp_processor_id();
    let index = *(arg as *mut u32);
    let clock_tick = sparc64_get_clock_tick(cpu) / 1000;
    let new_bits = index_to_estar_mode(index);
    let divisor = index_to_divisor(index);
    let _new_freq = clock_tick / divisor;
    let estar = read_hbreg(HBIRD_ESTAR_MODE_ADDR);
    let old_divisor = estar_to_divisor(estar);
    if old_divisor != divisor { us2e_transition(estar, new_bits, clock_tick * 1000, old_divisor, divisor); }
}

unsafe fn us2e_freq_target(policy: *mut cpufreq_policy, index: u32) -> i32 {
    smp_call_function_single((*policy).cpu, __us2e_freq_target, (&index as *const u32 as *mut u32).cast(), 1)
}

unsafe fn us2e_freq_cpu_init(policy: *mut cpufreq_policy) -> i32 {
    let cpu = (*policy).cpu;
    let clock_tick = sparc64_get_clock_tick(cpu) / 1000;
    let table = &mut (*us2e_freq_table.add(cpu as usize)).table[0] as *mut cpufreq_frequency_table;
    (*table.add(0)).driver_data = 0; (*table.add(0)).frequency = clock_tick as u32 / 1;
    (*table.add(1)).driver_data = 1; (*table.add(1)).frequency = clock_tick as u32 / 2;
    (*table.add(2)).driver_data = 2; (*table.add(2)).frequency = clock_tick as u32 / 4;
    (*table.add(2)).driver_data = 3; (*table.add(2)).frequency = clock_tick as u32 / 6;
    (*table.add(2)).driver_data = 4; (*table.add(2)).frequency = clock_tick as u32 / 8;
    (*table.add(2)).driver_data = 5; (*table.add(3)).frequency = CPUFREQ_TABLE_END;
    (*policy).cpuinfo.transition_latency = 0;
    (*policy).cur = clock_tick as u32;
    (*policy).freq_table = table;
    0
}

unsafe fn us2e_freq_cpu_exit(policy: *mut cpufreq_policy) { us2e_freq_target(policy, 0); }

#[repr(C)]
struct cpufreq_driver {
    name: *const u8,
    init: Option<unsafe fn(*mut cpufreq_policy) -> i32>,
    verify: Option<unsafe fn(*mut cpufreq_policy) -> i32>,
    target_index: Option<unsafe fn(*mut cpufreq_policy, u32) -> i32>,
    get: Option<unsafe fn(u32) -> u32>,
    exit: Option<unsafe fn(*mut cpufreq_policy)>,
}

static mut cpufreq_us2e_driver: cpufreq_driver = cpufreq_driver {
    name: b"UltraSPARC-IIe\0".as_ptr(),
    init: Some(us2e_freq_cpu_init),
    verify: None,
    target_index: Some(us2e_freq_target),
    get: Some(us2e_freq_get),
    exit: Some(us2e_freq_cpu_exit),
};

extern "C" {
    static mut tlb_type: i32;
    static spitfire: i32;
    static NR_CPUS: usize;
    fn kzalloc_objs<T>(count: usize) -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn cpufreq_register_driver(driver: *mut cpufreq_driver) -> i32;
    fn cpufreq_unregister_driver(driver: *mut cpufreq_driver);
}

unsafe fn us2e_freq_init() -> i32 {
    let mut ver: usize;
    core::arch::asm!("rdpr %ver, {ver}", ver = lateout(reg) ver, options(nostack, preserves_flags));
    let manuf = (ver >> 48) & 0xffff;
    let impl_ = (ver >> 32) & 0xffff;
    if tlb_type != spitfire { return -19; }
    if manuf == 0x17 && impl_ == 0x13 {
        us2e_freq_table = kzalloc_objs::<us2e_freq_percpu_info>(NR_CPUS);
        if us2e_freq_table.is_null() { return -12; }
        let ret = cpufreq_register_driver(&mut cpufreq_us2e_driver);
        if ret != 0 { kfree(us2e_freq_table.cast()); }
        return ret;
    }
    -19
}

unsafe fn us2e_freq_exit() {
    cpufreq_unregister_driver(&mut cpufreq_us2e_driver);
    kfree(us2e_freq_table.cast());
}

// MODULE_AUTHOR("David S. Miller <davem@redhat.com>");
// MODULE_DESCRIPTION("cpufreq driver for UltraSPARC-IIe");
// MODULE_LICENSE("GPL");
// module_init(us2e_freq_init);
// module_exit(us2e_freq_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
