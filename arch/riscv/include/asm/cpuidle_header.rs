/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2021 Allwinner Ltd
 * Copyright (C) 2021 Western Digital Corporation or its affiliates.
 */

// Dependencies supplied by <asm/barrier.h> and <asm/processor.h>.
extern "C" {
    fn mb();
    fn wait_for_interrupt();
}

#[inline]
unsafe fn cpu_do_idle() {
    /*
     * Add mb() here to ensure that all
     * IO/MEM accesses are completed prior
     * to entering WFI.
     */
    mb();
    wait_for_interrupt();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
