// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/se/7619/setup.c
 *
 * Copyright (C) 2006 Yoshinori Sato
 *
 * Hitachi SH7619 SolutionEngine Support.
 */

// C dependencies supplied by other translation units/headers:
// linux/init.h, linux/platform_device.h, asm/io.h, and asm/machvec.h

unsafe fn se7619_mode_pins() -> i32 {
    MODE_PIN2 | MODE_PIN0
}

/*
 * The Machine Vector
 */

// The C __initmv annotation is preserved as a comment; its storage/linkage
// semantics are supplied by the surrounding platform definitions.
static mut mv_se: sh_machine_vector = sh_machine_vector {
    mv_name: "SolutionEngine",
    mv_mode_pins: se7619_mode_pins,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
