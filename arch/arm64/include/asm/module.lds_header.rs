// Linker-script section declarations translated from module.lds.h.
//
// SECTIONS {
//     .plt 0 : { BYTE(0) }
//     .init.plt 0 : { BYTE(0) }
//     .text.ftrace_trampoline 0 : { BYTE(0) }
//     .init.text.ftrace_trampoline 0 : { BYTE(0) }
//
//     // CONFIG_UNWIND_TABLES:
//     // Currently, unwind info is only used at module load time, so it can be
//     // placed into the .init allocation.
//     .init.eh_frame 0 : { *(.eh_frame) }
// }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
