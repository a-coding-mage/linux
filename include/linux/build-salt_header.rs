// Translated from build-salt.h.
//
// The C header includes <linux/elfnote.h>; the corresponding ELF note
// facility is supplied by the surrounding build/dependency context.

pub const LINUX_ELFNOTE_BUILD_SALT: u32 = 0x100;

// C __ASSEMBLER__ selects the assembler ELFNOTE form.  Rust has no direct
// equivalent of that preprocessor mode in this file; the assembler intent is
// preserved here for the build configuration that provides the macro.
//
// #define BUILD_SALT ELFNOTE(Linux, LINUX_ELFNOTE_BUILD_SALT, .asciz CONFIG_BUILD_SALT)

// Non-assembler form:
// #define BUILD_SALT ELFNOTE32("Linux", LINUX_ELFNOTE_BUILD_SALT, CONFIG_BUILD_SALT)
#[macro_export]
macro_rules! BUILD_SALT {
    () => {
        ELFNOTE32!("Linux", $crate::LINUX_ELFNOTE_BUILD_SALT, CONFIG_BUILD_SALT)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
