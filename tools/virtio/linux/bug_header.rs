// SPDX-License-Identifier: GPL-2.0
// C header guard removed: _LINUX_BUG_H
// C dependency preserved for translation context: #include <asm/bug.h>

macro_rules! BUG_ON {
    ($__BUG_ON_cond:expr) => {
        assert!(!$__BUG_ON_cond)
    };
}

macro_rules! BUG {
    () => {
        std::process::abort()
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
