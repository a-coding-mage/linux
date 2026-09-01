/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright 2023 Marian Postevca <posteuca@mutex.one>
 */

/* Header guard __ACP3X_ES83XX_H omitted in Rust. */

unsafe extern "C" {
    pub fn acp3x_es83xx_init_ops(ops: *mut acp_mach_ops);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
