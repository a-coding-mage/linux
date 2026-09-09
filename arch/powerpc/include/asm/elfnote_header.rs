/* SPDX-License-Identifier: GPL-2.0 */
/*
 * PowerPC ELF notes.
 *
 * Copyright 2019, IBM Corporation
 */

/*
 * These note types should live in a SHT_NOTE segment and have
 * "PowerPC" in the name field.
 */

/*
 * The capabilities supported/required by this kernel (bitmap).
 *
 * This type uses a bitmap as "desc" field. Each bit is described
 * in arch/powerpc/kernel/note.S
 */
pub const PPC_ELFNOTE_CAPABILITIES: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
