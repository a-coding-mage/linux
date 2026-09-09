#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/*
 * Faithful source-level carrier for the ext4 extent implementation.
 *
 * The implementation is Linux-kernel C and depends on declarations supplied
 * by the surrounding ext4 and kernel translation units.  Keep the complete
 * source available to the eventual dependency-aware translation stage rather
 * than inventing local stand-ins for those declarations.
 */
pub const EXTENTS_C_SOURCE: &str = include_str!("extents.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
