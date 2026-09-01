// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2018 Intel Corporation. All rights reserved.

// C dependencies:
// #include <linux/module.h>
// #include <linux/printk.h>
// #include "watermark.h"
// #include <nfit.h>

use crate::{nfit_mem, set_bit, NFIT_MEM_DIRTY_COUNT};

nfit_test_watermark!(acpi_nfit);

/* strong / override definition of nfit_intel_shutdown_status */
#[no_mangle]
pub unsafe extern "C" fn nfit_intel_shutdown_status(nfit_mem: *mut nfit_mem) {
    unsafe {
        set_bit(NFIT_MEM_DIRTY_COUNT, &mut (*nfit_mem).flags);
        (*nfit_mem).dirty_shutdown = 42;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
