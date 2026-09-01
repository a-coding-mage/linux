// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

extern "C" {
    pub fn timerlat_aa_init(
        tool: *mut osnoise_tool,
        dump_task: ::std::os::raw::c_int,
        stack_format: stack_format,
    ) -> ::std::os::raw::c_int;
    pub fn timerlat_aa_destroy();

    pub fn timerlat_auto_analysis(
        irq_thresh: ::std::os::raw::c_int,
        thread_thresh: ::std::os::raw::c_int,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
