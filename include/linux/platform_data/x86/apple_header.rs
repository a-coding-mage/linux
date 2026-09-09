// Conditional declaration from the C header:
// #ifdef CONFIG_X86

/// x86_apple_machine - whether the machine is an x86 Apple Macintosh
#[cfg(CONFIG_X86)]
extern "C" {
    pub static mut x86_apple_machine: bool;
}

// #else
// #define x86_apple_machine false
#[cfg(not(CONFIG_X86))]
pub const x86_apple_machine: bool = false;

// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
