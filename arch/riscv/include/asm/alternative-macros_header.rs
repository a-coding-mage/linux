/* SPDX-License-Identifier: GPL-2.0 */

// Rust translation of alternative-macros.h.
// CONFIG_RISCV_ALTERNATIVE and __ASSEMBLER__ are build-time C/assembler
// conditions; the corresponding macro forms are retained below as Rust
// declarative macros and assembly-text fragments.

#[cfg(feature = "CONFIG_RISCV_ALTERNATIVE")]
#[cfg(feature = "__ASSEMBLER__")]
macro_rules! ALT_ENTRY {
    ($oldptr:tt, $newptr:tt, $vendor_id:tt, $patch_id:tt, $new_len:tt) => {
        concat!(
            ".4byte ", stringify!($oldptr), " - .\n",
            ".4byte ", stringify!($newptr), " - .\n",
            ".2byte ", stringify!($vendor_id), "\n",
            ".2byte ", stringify!($new_len), "\n",
            ".4byte ", stringify!($patch_id), "\n"
        )
    };
}

#[cfg(feature = "CONFIG_RISCV_ALTERNATIVE")]
#[cfg(feature = "__ASSEMBLER__")]
macro_rules! ALT_NEW_CONTENT {
    ($vendor_id:tt, $patch_id:tt, $enable:tt, $new_c:expr) => {
        concat!(
            ".if ", stringify!($enable), "\n",
            ".pushsection .alternative, \"a\"\n",
            ALT_ENTRY!(886b, 888f, $vendor_id, $patch_id, 889f - 888f),
            ".popsection\n.subsection 1\n888 :\n",
            ".option push\n.option norvc\n.option norelax\n",
            $new_c, "\n.option pop\n889 :\n",
            ".org . - (889b - 888b) + (887b - 886b)\n",
            ".org . - (887b - 886b) + (889b - 888b)\n",
            ".previous\n.endif\n"
        )
    };
}

#[cfg(feature = "CONFIG_RISCV_ALTERNATIVE")]
#[cfg(feature = "__ASSEMBLER__")]
macro_rules! ALTERNATIVE_CFG {
    ($old_c:expr, $new_c:expr, $vendor_id:tt, $patch_id:tt, $enable:tt) => {
        concat!(
            ".option push\n.option norvc\n.option norelax\n",
            $old_c, "\n.option pop\n887 :\n",
            ALT_NEW_CONTENT!($vendor_id, $patch_id, $enable, $new_c)
        )
    };
}

#[cfg(feature = "CONFIG_RISCV_ALTERNATIVE")]
#[cfg(not(feature = "__ASSEMBLER__"))]
macro_rules! ALT_ENTRY {
    ($oldptr:expr, $newptr:expr, $vendor_id:expr, $patch_id:expr, $newlen:expr) => {
        concat!(".4byte ((", $oldptr, ") - .)\n.4byte ((", $newptr,
                ") - .)\n.2byte ", $vendor_id, "\n.2byte ", $newlen,
                "\n.4byte ", $patch_id, "\n")
    };
}

#[cfg(feature = "CONFIG_RISCV_ALTERNATIVE")]
#[cfg(not(feature = "__ASSEMBLER__"))]
macro_rules! ALT_NEW_CONTENT {
    ($vendor_id:expr, $patch_id:expr, $enable:expr, $new_c:expr) => {
        concat!(".if ", stringify!($enable), " == 1\n.pushsection .alternative, \"a\"\n",
                ALT_ENTRY!("886b", "888f", stringify!($vendor_id), stringify!($patch_id), "889f - 888f"),
                ".popsection\n.subsection 1\n888 :\n.option push\n.option norvc\n.option norelax\n",
                $new_c, "\n.option pop\n889 :\n",
                ".org . - (887b - 886b) + (889b - 888b)\n",
                ".org . - (889b - 888b) + (887b - 886b)\n.previous\n.endif\n")
    };
}

#[cfg(not(feature = "CONFIG_RISCV_ALTERNATIVE"))]
macro_rules! __ALTERNATIVE_CFG {
    ($old_c:expr, $($rest:tt)*) => { concat!($old_c, "\n") };
}
#[cfg(not(feature = "CONFIG_RISCV_ALTERNATIVE"))]
macro_rules! __ALTERNATIVE_CFG_2 {
    ($old_c:expr, $($rest:tt)*) => { concat!($old_c, "\n") };
}

// Usage:
//   ALTERNATIVE(old_content, new_content, vendor_id, patch_id, CONFIG_k)
// in assembly, or asm!(ALTERNATIVE!(old_content, new_content, vendor_id,
// patch_id, CONFIG_k));
// old_content is replaced by new_content when the configured patch is enabled.
macro_rules! _ALTERNATIVE_CFG {
    ($old_c:expr, $new_c:expr, $vendor_id:expr, $patch_id:expr, $config_k:expr) => {
        __ALTERNATIVE_CFG!($old_c, $new_c, $vendor_id, $patch_id, $config_k)
    };
}
macro_rules! _ALTERNATIVE_CFG_2 {
    ($old_c:expr, $new_c1:expr, $vendor_id1:expr, $patch_id1:expr, $config1:expr,
     $new_c2:expr, $vendor_id2:expr, $patch_id2:expr, $config2:expr) => {
        __ALTERNATIVE_CFG_2!($old_c, $new_c1, $vendor_id1, $patch_id1, $config1,
                             $new_c2, $vendor_id2, $patch_id2, $config2)
    };
}

macro_rules! ALTERNATIVE {
    ($old:expr, $new:expr, $vendor:expr, $patch:expr, $config:expr) => {
        _ALTERNATIVE_CFG!($old, $new, $vendor, $patch, $config)
    };
}
macro_rules! ALTERNATIVE_2 {
    ($old:expr, $new1:expr, $vendor1:expr, $patch1:expr, $config1:expr,
     $new2:expr, $vendor2:expr, $patch2:expr, $config2:expr) => {
        _ALTERNATIVE_CFG_2!($old, $new1, $vendor1, $patch1, $config1,
                            $new2, $vendor2, $patch2, $config2)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
