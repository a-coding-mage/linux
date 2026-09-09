/* Faithful low-level Rust translation of cvmx-lmcx-defs.h. */
#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* CVMX_ADD_IO_SEG and OCTEON family symbols are supplied by the surrounding target. */


/* Family-dependent inline register functions. */

extern "C" { fn cvmx_get_octeon_family() -> u32; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
