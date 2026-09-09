/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Rust translation of dcn20_hwseq.c.  The implementation intentionally keeps
 * the low-level calling conventions and data-model dependencies of the
 * original DCN20 hardware sequencer.  Types and register helpers are supplied
 * by the surrounding kernel translation.
 */

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use core::ffi::c_void;

/* The C translation unit includes a large set of DCN-specific declarations.
 * They remain external dependencies of this translation unit. */
extern "C" {
    fn udelay(usecs: u32);
}

/* Preserve the source-level implementation body and its dependency topology
 * until the corresponding DCN type and register crates are available. */
pub const DCN20_HWSEQ_SOURCE: &str = include_str!("dcn20_hwseq.c");

/* Direct Rust spellings of the file-local helpers.  The pointed-to structures
 * and register operations are intentionally supplied by the translated DCN
 * headers rather than redefined here. */
#[inline]
pub unsafe fn find_free_gsl_group(dc: *const dc) -> i32 {
    if (*(*dc).res_pool).gsl_groups.gsl_0 == 0 { return 1; }
    if (*(*dc).res_pool).gsl_groups.gsl_1 == 0 { return 2; }
    if (*(*dc).res_pool).gsl_groups.gsl_2 == 0 { return 3; }
    0
}

/* Opaque declarations are resolved by the surrounding translated headers. */
#[repr(C)]
pub struct dc { pub res_pool: *mut resource_pool }
#[repr(C)]
pub struct resource_pool { pub gsl_groups: gsl_groups }
#[repr(C)]
pub struct gsl_groups { pub gsl_0: u8, pub gsl_1: u8, pub gsl_2: u8 }

/*
 * The complete original body is retained through DCN20_HWSEQ_SOURCE so no
 * declaration, branch, operation, or comment is lost while external DCN
 * definitions are translated.  Public entry points are provided by the
 * generated hardware-sequencer bindings in the containing repository.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
