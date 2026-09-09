/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Emulated 1-byte and 2-byte cmpxchg operations for architectures
 * lacking direct support for these sizes.  These are implemented in terms
 * of 4-byte cmpxchg operations.
 *
 * Copyright (C) 2024 Paul E. McKenney.
 */

// C header guard: __LINUX_CMPXCHG_EMU_H

extern "C" {
    pub fn cmpxchg_emu_u8(p: *mut u8, old: usize, new: usize) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
