/* SPDX-License-Identifier: GPL-2.0-or-later */
// Translated from asm/idle.h.
// Dependencies supplied by the surrounding PowerPC environment:
// asm/runlatch.h and asm/paca.h.

// CONFIG_PPC_PSERIES conditional preserved from the source header.
#[cfg(CONFIG_PPC_PSERIES)]
extern "C" {
    pub static mut idle_spurr_cycles: u64;
    pub static mut idle_entry_purr_snap: u64;
    pub static mut idle_entry_spurr_snap: u64;
}

#[cfg(CONFIG_PPC_PSERIES)]
#[inline(always)]
pub unsafe fn snapshot_purr_idle_entry() {
    *(this_cpu_ptr(&raw mut idle_entry_purr_snap)) = mfspr(SPRN_PURR);
}

#[cfg(CONFIG_PPC_PSERIES)]
#[inline(always)]
pub unsafe fn snapshot_spurr_idle_entry() {
    *(this_cpu_ptr(&raw mut idle_entry_spurr_snap)) = mfspr(SPRN_SPURR);
}

#[cfg(CONFIG_PPC_PSERIES)]
#[inline(always)]
pub unsafe fn update_idle_purr_accounting() {
    let mut wait_cycles: u64;
    let in_purr: u64 = *this_cpu_ptr(&raw mut idle_entry_purr_snap);

    wait_cycles = be64_to_cpu((*get_lppaca()).wait_state_cycles);
    wait_cycles = wait_cycles.wrapping_add(mfspr(SPRN_PURR).wrapping_sub(in_purr));
    (*get_lppaca()).wait_state_cycles = cpu_to_be64(wait_cycles);
}

#[cfg(CONFIG_PPC_PSERIES)]
#[inline(always)]
pub unsafe fn update_idle_spurr_accounting() {
    let idle_spurr_cycles_ptr: *mut u64 = this_cpu_ptr(&raw mut idle_spurr_cycles);
    let in_spurr: u64 = *this_cpu_ptr(&raw mut idle_entry_spurr_snap);

    *idle_spurr_cycles_ptr = (*idle_spurr_cycles_ptr)
        .wrapping_add(mfspr(SPRN_SPURR).wrapping_sub(in_spurr));
}

#[cfg(CONFIG_PPC_PSERIES)]
#[inline(always)]
pub unsafe fn pseries_idle_prolog() {
    ppc64_runlatch_off();
    snapshot_purr_idle_entry();
    snapshot_spurr_idle_entry();
    /*
     * Indicate to the HV that we are idle. Now would be
     * a good time to find other work to dispatch.
     */
    (*get_lppaca()).idle = 1;
}

#[cfg(CONFIG_PPC_PSERIES)]
#[inline(always)]
pub unsafe fn pseries_idle_epilog() {
    update_idle_purr_accounting();
    update_idle_spurr_accounting();
    (*get_lppaca()).idle = 0;
    ppc64_runlatch_on();
}

#[cfg(CONFIG_PPC_PSERIES)]
#[inline]
pub unsafe fn read_this_idle_purr() -> u64 {
    /*
     * If we are reading from an idle context, update the
     * idle-purr cycles corresponding to the last idle period.
     * Since the idle context is not yet over, take a fresh
     * snapshot of the idle-purr.
     */
    if unlikely((*get_lppaca()).idle == 1) {
        update_idle_purr_accounting();
        snapshot_purr_idle_entry();
    }

    be64_to_cpu((*get_lppaca()).wait_state_cycles)
}

#[cfg(CONFIG_PPC_PSERIES)]
#[inline]
pub unsafe fn read_this_idle_spurr() -> u64 {
    /*
     * If we are reading from an idle context, update the
     * idle-spurr cycles corresponding to the last idle period.
     * Since the idle context is not yet over, take a fresh
     * snapshot of the idle-spurr.
     */
    if (*get_lppaca()).idle == 1 {
        update_idle_spurr_accounting();
        snapshot_spurr_idle_entry();
    }

    *this_cpu_ptr(&raw mut idle_spurr_cycles)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
