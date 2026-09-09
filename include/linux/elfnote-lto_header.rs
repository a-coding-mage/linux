// Translated from elfnote-lto.h.
// Dependency intent: ELFNOTE32 is supplied by linux/elfnote.h.

pub const LINUX_ELFNOTE_LTO_INFO: u32 = 0x101;

// The CONFIG_LTO build-time condition is preserved here.  Define CONFIG_LTO
// in the surrounding build when the LTO configuration is enabled.
#[cfg(CONFIG_LTO)]
macro_rules! BUILD_LTO_INFO {
    () => {
        ELFNOTE32!("Linux", LINUX_ELFNOTE_LTO_INFO, 1)
    };
}

#[cfg(not(CONFIG_LTO))]
macro_rules! BUILD_LTO_INFO {
    () => {
        ELFNOTE32!("Linux", LINUX_ELFNOTE_LTO_INFO, 0)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
