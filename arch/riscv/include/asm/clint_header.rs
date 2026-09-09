/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Google, Inc
 */

// CONFIG_RISCV_M_MODE conditional preserved from the C header.
//
// This lives in the CLINT driver, but is accessed directly by timex.h to avoid
// any overhead when accessing the MMIO timer.
//
// The ISA defines mtime as a 64-bit memory-mapped register that increments at
// a constant frequency, but it doesn't define some other constraints we depend
// on (most notably ordering constraints, but also some simpler stuff like the
// memory layout).  Thus, this is called "clint_time_val" instead of something
// like "riscv_mtime", to signify that these non-ISA assumptions must hold.
//
// The declaration is applicable when CONFIG_RISCV_M_MODE is enabled.
#[cfg(CONFIG_RISCV_M_MODE)]
unsafe extern "C" {
    pub static mut clint_time_val: *mut u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
