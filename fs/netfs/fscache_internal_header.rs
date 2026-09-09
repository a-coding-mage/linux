/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Internal definitions for FS-Cache
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependency supplied by the surrounding translation unit:
// #include "internal.h"

// The C preprocessor conditionally undefines any prior pr_fmt definition.
// Rust macro definitions are scoped, so no separate undefinition is needed.
macro_rules! pr_fmt {
    ($fmt:literal) => {
        concat!("FS-Cache: ", $fmt)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
