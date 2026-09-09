/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// Dependency supplied by the corresponding uapi header:
// #include <uapi/asm/elf.h>

// The following declarations are active only when __uClinux__ is not defined
// in the original build configuration.  The conditional blocks in this
// header contain no declarations.
#[cfg(not(__uClinux__))]
// The original ELF_GREG_T, ELF_NGREG, ELF_GREGSET_T, and ELF_FPREGSET_T
// conditional blocks are empty in this header.

// The __MICROBLAZEEL__ conditional block is empty in this header.

// C macro equivalent. The `ex` argument is intentionally unused, as in C.
#[macro_export]
macro_rules! SET_PERSONALITY {
    ($ex:expr) => {
        set_personality(
            PER_LINUX_32BIT
                | ((*current).personality & (!PER_MASK)),
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
