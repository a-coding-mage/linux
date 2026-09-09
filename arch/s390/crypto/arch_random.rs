// SPDX-License-Identifier: GPL-2.0
/*
 * s390 arch random implementation.
 *
 * Copyright IBM Corp. 2017, 2020
 * Author(s): Harald Freudenberger
 */

// The following names are supplied by the surrounding kernel translation.

/// Equivalent of DEFINE_STATIC_KEY_FALSE(s390_arch_random_available).
pub static s390_arch_random_available: StaticKeyFalse = StaticKeyFalse::new();

pub static s390_arch_random_counter: AtomicI64 = AtomicI64::new(0);

// Equivalent of EXPORT_SYMBOL(s390_arch_random_counter).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
