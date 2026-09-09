/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003 by Ralf Baechle
 */

/*
 * R5000 and RM5200 implement pref and prefx instructions but they're nops, so
 * rather than wasting time we pretend these processors don't support
 * prefetching at all.
 *
 * R5432 implements Load, Store, LoadStreamed, StoreStreamed, LoadRetained,
 * StoreRetained and WriteBackInvalidate but not Pref_PrepareForStore.
 *
 * Hell (and the book on my shelf I can't open ...) know what the R8000 does.
 *
 * RM7000 version 1.0 interprets all hints as Pref_Load; version 2.0 implements
 * Pref_PrepareForStore also.
 *
 * RM9000 is MIPS IV but implements prefetching like MIPS32/MIPS64; it's
 * Pref_WriteBackInvalidate is a nop and Pref_PrepareForStore is broken in
 * current versions due to erratum G105.
 *
 * VR5500 (including VR5701 and VR7701) only implement load prefetch.
 *
 * Finally MIPS32 and MIPS64 implement all of the following hints.
 */

pub const PREF_LOAD: i32 = 0;
pub const PREF_STORE: i32 = 1;
/* 2 and 3 are reserved. */
pub const PREF_LOAD_STREAMED: i32 = 4;
pub const PREF_STORE_STREAMED: i32 = 5;
pub const PREF_LOAD_RETAINED: i32 = 6;
pub const PREF_STORE_RETAINED: i32 = 7;
/* 8 ... 24 are reserved. */
pub const PREF_WRITE_BACK_INVALIDATE: i32 = 25;
pub const PREF_PREPARE_FOR_STORE: i32 = 30;

/*
 * The source also defines assembler-only macros (__pref, pref_load,
 * pref_store, pref_load_streamed, pref_store_streamed, pref_load_retained,
 * pref_store_retained, pref_wback_inv, and pref_prepare_for_store).  They
 * emit the MIPS `pref` instruction only when CONFIG_CPU_HAS_PREFETCH is set;
 * assembler-only syntax has no direct Rust item equivalent.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
