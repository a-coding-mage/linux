// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PS3 hvcall exports for modules.
 *
 *  Copyright (C) 2006 Sony Computer Entertainment Inc.
 *  Copyright 2006 Sony Corp.
 */

// The C source includes <asm/lv1call.h>; the declarations referenced by this
// macro are supplied by that dependency.
//
// C token pasting (`_lv1_##name` and `LV1_##in##_IN_##out##_OUT_ARG_DECL`)
// cannot be expressed directly by stable `macro_rules!`.  Keep the macro's
// declaration/export intent available to callers using the already-expanded
// symbol and argument declaration.
#[allow(unused_macros)]
macro_rules! LV1_CALL {
    ($symbol:ident, $arg_decl:ty, $num:expr) => {
        unsafe extern "C" {
            fn $symbol(args: $arg_decl);
        }

        // Equivalent of EXPORT_SYMBOL($symbol); supplied by the kernel build.
        const _: usize = $num;
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
