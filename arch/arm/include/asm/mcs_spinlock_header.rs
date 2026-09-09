/* SPDX-License-Identifier: GPL-2.0 */

// The original declarations are conditional on CONFIG_SMP.
// Include the following items when CONFIG_SMP is enabled.

// Dependency intent from <asm/spinlock.h> is preserved; the referenced
// synchronization primitives are supplied by the surrounding translation.

/* MCS spin-locking. */
macro_rules! arch_mcs_spin_lock_contended {
    ($lock:expr) => {{
        // Ensure prior stores are observed before we enter wfe.
        smp_mb!();
        while !(smp_load_acquire!($lock)) {
            wfe!();
        }
    }};
}

macro_rules! arch_mcs_spin_unlock_contended {
    ($lock:expr) => {{
        smp_store_release!($lock, 1);
        dsb_sev!();
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
