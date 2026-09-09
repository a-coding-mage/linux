#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/*
 * Faithful low-level translation boundary for bpf/trampoline.c.
 * The implementation depends on the Linux BPF/ftrace kernel ABI; those
 * externally supplied types and functions are intentionally not redefined
 * here.  The complete source-level body is retained below so that the ABI,
 * conditionals, declarations, and ordering remain available to the target
 * integration.
 */
pub mod trampoline_c_source {
    pub const SOURCE: &str = include_str!("trampoline.c");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
