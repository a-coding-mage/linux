/* SPDX-License-Identifier: GPL-2.0-or-later */

// Translated from the PowerPC paravirt header. C preprocessor configuration
// conditions are represented with Rust cfg attributes where applicable.

#[cfg(feature = "CONFIG_PPC_SPLPAR")]
extern "C" {
    static shared_processor: StaticKey;
}

#[cfg(feature = "CONFIG_PPC_SPLPAR")]
#[inline]
fn is_shared_processor() -> bool {
    unsafe { static_branch_unlikely(&shared_processor) }
}

#[cfg(feature = "CONFIG_PARAVIRT_TIME_ACCOUNTING")]
extern "C" {
    fn pseries_paravirt_steal_clock(cpu: i32) -> u64;
}

#[cfg(all(feature = "CONFIG_PPC_SPLPAR", feature = "CONFIG_PARAVIRT_TIME_ACCOUNTING"))]
#[inline]
fn paravirt_steal_clock(cpu: i32) -> u64 {
    unsafe { pseries_paravirt_steal_clock(cpu) }
}

#[cfg(feature = "CONFIG_PPC_SPLPAR")]
#[inline]
fn yield_count_of(cpu: i32) -> u32 {
    let yield_count: Be32 = unsafe { core::ptr::read_volatile(&lppaca_of(cpu).yield_count) };
    be32_to_cpu(yield_count)
}

#[cfg(feature = "CONFIG_PPC_SPLPAR")]
#[inline]
fn yield_to_preempted(cpu: i32, yield_count: u32) {
    unsafe {
        plpar_hcall_norets_notrace(
            H_CONFER,
            get_hard_smp_processor_id(cpu),
            yield_count,
        );
    }
}

#[cfg(feature = "CONFIG_PPC_SPLPAR")]
#[inline]
fn prod_cpu(cpu: i32) {
    unsafe { plpar_hcall_norets_notrace(H_PROD, get_hard_smp_processor_id(cpu)) }
}

#[cfg(feature = "CONFIG_PPC_SPLPAR")]
#[inline]
fn yield_to_any() {
    unsafe { plpar_hcall_norets_notrace(H_CONFER, -1, 0) }
}

#[cfg(feature = "CONFIG_PPC_SPLPAR")]
#[inline]
fn is_vcpu_idle(vcpu: i32) -> bool {
    unsafe { lppaca_of(vcpu).idle }
}

#[cfg(feature = "CONFIG_PPC_SPLPAR")]
#[inline]
fn vcpu_is_dispatched(vcpu: i32) -> bool {
    // An odd yield_count means yielded; an even value means executing.
    (yield_count_of(vcpu) & 1) == 0
}

#[cfg(not(feature = "CONFIG_PPC_SPLPAR"))]
#[inline]
fn is_shared_processor() -> bool {
    false
}

#[cfg(not(feature = "CONFIG_PPC_SPLPAR"))]
#[inline]
fn yield_count_of(_cpu: i32) -> u32 {
    0
}

#[cfg(not(feature = "CONFIG_PPC_SPLPAR"))]
extern "C" {
    fn ___bad_yield_to_preempted();
}

#[cfg(not(feature = "CONFIG_PPC_SPLPAR"))]
#[inline]
fn yield_to_preempted(_cpu: i32, _yield_count: u32) {
    unsafe { ___bad_yield_to_preempted() } // This would be a bug
}

#[cfg(not(feature = "CONFIG_PPC_SPLPAR"))]
extern "C" {
    fn ___bad_yield_to_any();
}

#[cfg(not(feature = "CONFIG_PPC_SPLPAR"))]
#[inline]
fn yield_to_any() {
    unsafe { ___bad_yield_to_any() } // This would be a bug
}

#[cfg(not(feature = "CONFIG_PPC_SPLPAR"))]
extern "C" {
    fn ___bad_prod_cpu();
}

#[cfg(not(feature = "CONFIG_PPC_SPLPAR"))]
#[inline]
fn prod_cpu(_cpu: i32) {
    unsafe { ___bad_prod_cpu() } // This would be a bug
}

#[cfg(not(feature = "CONFIG_PPC_SPLPAR"))]
#[inline]
fn is_vcpu_idle(_vcpu: i32) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_PPC_SPLPAR"))]
#[inline]
fn vcpu_is_dispatched(_vcpu: i32) -> bool {
    true
}

#[inline]
fn vcpu_is_preempted(cpu: i32) -> bool {
    // The dispatch/yield bit alone is an imperfect indicator of whether the
    // hypervisor has dispatched cpu to run on a physical processor.
    if !is_shared_processor() {
        return false;
    }

    if vcpu_is_dispatched(cpu) {
        return false;
    }

    if !is_vcpu_idle(cpu) {
        return true;
    }

    #[cfg(feature = "CONFIG_PPC_SPLPAR")]
    {
        if unsafe { !is_kvm_guest() } {
            let mut first_cpu: i32;
            let mut i: i32;

            first_cpu = unsafe { cpu_first_thread_sibling(raw_smp_processor_id()) };
            if unsafe { cpu_first_thread_sibling(cpu) } == first_cpu {
                return false;
            }

            first_cpu = unsafe { cpu_first_thread_sibling(cpu) };
            i = first_cpu;
            while i < first_cpu + unsafe { threads_per_core } {
                if i == cpu {
                    i += 1;
                    continue;
                } else if vcpu_is_dispatched(i) {
                    return false;
                } else if !is_vcpu_idle(i) {
                    return true;
                }
                i += 1;
            }
        }
    }

    false
}

#[inline]
fn pv_is_native_spin_unlock() -> bool {
    !is_shared_processor()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
