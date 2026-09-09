// Dependency: <asm-generic/asm-prototypes.h>
// Dependency: <asm/checksum.h>

// Preserved build-time condition: CONFIG_UML_X86
#[cfg(CONFIG_UML_X86)]
unsafe extern "C" {
    pub fn cmpxchg8b_emu();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
