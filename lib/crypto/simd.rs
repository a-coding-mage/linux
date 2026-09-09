// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SIMD testing utility functions
 *
 * Copyright 2024 Google LLC
 */

// C dependency: <crypto/internal/simd.h>
// DEFINE_PER_CPU(bool, crypto_simd_disabled_for_test);
// The kernel per-CPU storage mechanism is represented here by the exported
// mutable global; per-CPU placement is supplied by the surrounding kernel
// integration.
#[no_mangle]
pub static mut crypto_simd_disabled_for_test: bool = false;

// EXPORT_PER_CPU_SYMBOL_GPL(crypto_simd_disabled_for_test);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
