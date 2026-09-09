// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012-2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Dependency supplied by the surrounding KVM hypervisor code.

pub unsafe fn __kvm_timer_set_cntvoff(cntvoff: u64) {
    write_sysreg!(cntvoff, cntvoff_el2);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
