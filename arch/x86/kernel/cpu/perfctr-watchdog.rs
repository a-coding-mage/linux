// SPDX-License-Identifier: GPL-2.0
/*
 * local apic based NMI watchdog for various CPUs.
 *
 * This file also handles reservation of performance counters for coordination
 * with other users.
 *
 * Note that these events normally don't tick when the CPU idles. This means
 * the frequency varies with CPU load.
 *
 * Original code for K7/P6 written by Keith Owens
 */

// Linux kernel dependencies and build-time configuration are supplied by the
// surrounding translation unit.

/* This number is calculated from Intel's MSR_P4_CRU_ESCR5 register and its
 * offset from MSR_P4_BSU_ESCR0. It will be the max for all platforms (for now).
 */
const NMI_MAX_COUNTER_BITS: usize = 66;

/*
 * perfctr_nmi_owner tracks the ownership of the perfctr registers:
 * evtsel_nmi_owner tracks the ownership of the event selection
 * - different performance counters/event selection may be reserved for
 *   different subsystems; this reservation system just tries to coordinate
 *   things a little
 */
static mut PERFCTR_NMI_OWNER: [usize; (NMI_MAX_COUNTER_BITS + usize::BITS as usize - 1) / usize::BITS as usize] =
    [0; (NMI_MAX_COUNTER_BITS + usize::BITS as usize - 1) / usize::BITS as usize];
static mut EVNTSEL_NMI_OWNER: [usize; (NMI_MAX_COUNTER_BITS + usize::BITS as usize - 1) / usize::BITS as usize] =
    [0; (NMI_MAX_COUNTER_BITS + usize::BITS as usize - 1) / usize::BITS as usize];

/* External kernel symbols and constants are supplied by other translated files. */
extern "C" {
    static boot_cpu_data: CpuData;
    fn cpu_has(cpu: *const CpuData, feature: u32) -> bool;
    fn test_and_set_bit(bit: u32, addr: *mut usize) -> bool;
    fn clear_bit(bit: u32, addr: *mut usize);
}

#[repr(C)]
struct CpuData {
    x86_vendor: u32,
    x86: u32,
}

#[inline]
unsafe fn nmi_perfctr_msr_to_bit(msr: u32) -> u32 {
    /* returns the bit offset of the performance counter register */
    match boot_cpu_data.x86_vendor {
        X86_VENDOR_HYGON | X86_VENDOR_AMD => {
            if msr >= MSR_F15H_PERF_CTR {
                return (msr - MSR_F15H_PERF_CTR) >> 1;
            }
            msr - MSR_K7_PERFCTR0
        }
        X86_VENDOR_INTEL => {
            if cpu_has(&boot_cpu_data, X86_FEATURE_ARCH_PERFMON) {
                return msr - MSR_ARCH_PERFMON_PERFCTR0;
            }
            match boot_cpu_data.x86 {
                6 => msr - MSR_P6_PERFCTR0,
                11 => msr - MSR_KNC_PERFCTR0,
                15 => msr - MSR_P4_BPU_PERFCTR0,
                _ => 0,
            }
        }
        X86_VENDOR_ZHAOXIN | X86_VENDOR_CENTAUR => msr - MSR_ARCH_PERFMON_PERFCTR0,
        _ => 0,
    }
}

#[inline]
unsafe fn nmi_evntsel_msr_to_bit(msr: u32) -> u32 {
    /* returns the bit offset of the event selection register */
    match boot_cpu_data.x86_vendor {
        X86_VENDOR_HYGON | X86_VENDOR_AMD => {
            if msr >= MSR_F15H_PERF_CTL {
                return (msr - MSR_F15H_PERF_CTL) >> 1;
            }
            msr - MSR_K7_EVNTSEL0
        }
        X86_VENDOR_INTEL => {
            if cpu_has(&boot_cpu_data, X86_FEATURE_ARCH_PERFMON) {
                return msr - MSR_ARCH_PERFMON_EVENTSEL0;
            }
            match boot_cpu_data.x86 {
                6 => msr - MSR_P6_EVNTSEL0,
                11 => msr - MSR_KNC_EVNTSEL0,
                15 => msr - MSR_P4_BSU_ESCR0,
                _ => 0,
            }
        }
        X86_VENDOR_ZHAOXIN | X86_VENDOR_CENTAUR => msr - MSR_ARCH_PERFMON_EVENTSEL0,
        _ => 0,
    }
}

pub unsafe fn reserve_perfctr_nmi(msr: u32) -> i32 {
    let counter = nmi_perfctr_msr_to_bit(msr);
    /* register not managed by the allocator? */
    if counter as usize > NMI_MAX_COUNTER_BITS {
        return 1;
    }
    if !test_and_set_bit(counter, PERFCTR_NMI_OWNER.as_mut_ptr()) {
        return 1;
    }
    0
}

pub unsafe fn release_perfctr_nmi(msr: u32) {
    let counter = nmi_perfctr_msr_to_bit(msr);
    /* register not managed by the allocator? */
    if counter as usize > NMI_MAX_COUNTER_BITS {
        return;
    }
    clear_bit(counter, PERFCTR_NMI_OWNER.as_mut_ptr());
}

pub unsafe fn reserve_evntsel_nmi(msr: u32) -> i32 {
    let counter = nmi_evntsel_msr_to_bit(msr);
    /* register not managed by the allocator? */
    if counter as usize > NMI_MAX_COUNTER_BITS {
        return 1;
    }
    if !test_and_set_bit(counter, EVNTSEL_NMI_OWNER.as_mut_ptr()) {
        return 1;
    }
    0
}

pub unsafe fn release_evntsel_nmi(msr: u32) {
    let counter = nmi_evntsel_msr_to_bit(msr);
    /* register not managed by the allocator? */
    if counter as usize > NMI_MAX_COUNTER_BITS {
        return;
    }
    clear_bit(counter, EVNTSEL_NMI_OWNER.as_mut_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
