// Dependency equivalent of <asm/bitsperlong.h>.
//
// The original header selects the architecture-specific syscall table at
// build time based on __BITS_PER_LONG. The corresponding Rust declarations
// are supplied by the architecture-specific dependencies.
//
// #if __BITS_PER_LONG == 64
//     <asm/syscall_table_64.h>
// #else
//     <asm/syscall_table_32.h>
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
