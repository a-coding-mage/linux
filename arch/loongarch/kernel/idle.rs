// SPDX-License-Identifier: GPL-2.0
/*
 * LoongArch idle loop support.
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

unsafe extern "C" {
    fn __arch_cpu_idle();
    fn raw_local_irq_disable();
}

pub unsafe extern "C" fn arch_cpu_idle() {
    __arch_cpu_idle();
    raw_local_irq_disable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
