// SPDX-License-Identifier: GPL-2.0

#[cfg(target_arch = "x86_64")]
use usdt::USDT;

#[cfg(target_arch = "x86_64")]
// Include usdt.h with defined USDT_NOP macro to use single
// nop instruction.
// C equivalent before including usdt.h: #define USDT_NOP .byte 0x90
// C function attribute preserved as intent: __attribute__((aligned(16))).
pub unsafe extern "C" fn usdt_1() {
    USDT!(optimized_attach, usdt_1);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
