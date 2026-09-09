/*
 * Copyright (C) 2004 Jeff Dike (jdike@addtoit.com)
 * Copyright 2003 PathScale, Inc.
 * Licensed under the GPL
 */

// CONFIG_X86_32 selects the 32-bit definition below; otherwise the
// non-32-bit definition is selected, matching the original preprocessor
// condition.

#[cfg(CONFIG_X86_32)]
macro_rules! VMA_DATA_DEFAULT_FLAGS {
    () => {
        VMA_DATA_FLAGS_TSK_EXEC
    };
}

#[cfg(not(CONFIG_X86_32))]
macro_rules! VMA_STACK_DEFAULT_FLAGS {
    () => {
        append_vma_flags(VMA_DATA_FLAGS_EXEC, VMA_GROWSDOWN_BIT)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
