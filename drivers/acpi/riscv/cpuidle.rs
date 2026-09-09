// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024, Ventana Micro Systems Inc
 *\tAuthor: Sunil V L <sunilvl@ventanamicro.com>
 *
 */

// Dependencies supplied by the surrounding kernel translation.

pub const RISCV_FFH_LPI_TYPE_MASK: u64 = 0xF000_0000_0000_0000;
pub const RISCV_FFH_LPI_RSVD_MASK: u64 = 0x0FFF_FFFF_0000_0000;

pub const RISCV_FFH_LPI_TYPE_SBI: u64 = 1u64 << 60;

unsafe fn acpi_cpu_init_idle(cpu: u32) -> i32 {
    let pr = per_cpu(processors, cpu);

    if unlikely(pr.is_null() || !(*pr).flags.has_lpi) {
        return -EINVAL;
    }

    if !riscv_sbi_hsm_is_supported() {
        return -ENODEV;
    }

    if (*pr).power.count <= 1 {
        return -ENODEV;
    }

    let mut i = 1;
    while i < (*pr).power.count {
        let lpi = &mut (*pr).power.lpi_states[i as usize];

        /*
         * Validate Entry Method as per FFH spec.
         * bits[63:60] should be 0x1
         * bits[59:32] should be 0x0
         * bits[31:0] represent a SBI power_state
         */
        if (((lpi.address & RISCV_FFH_LPI_TYPE_MASK) != RISCV_FFH_LPI_TYPE_SBI)
            || (lpi.address & RISCV_FFH_LPI_RSVD_MASK) != 0)
        {
            pr_warn!("Invalid LPI entry method %#llx\n", lpi.address);
            return -EINVAL;
        }

        let state: u32 = lpi.address as u32;
        if !riscv_sbi_suspend_state_is_valid(state) {
            pr_warn!("Invalid SBI power state %#x\n", state);
            return -EINVAL;
        }

        i += 1;
    }

    0
}

pub unsafe fn acpi_processor_ffh_lpi_probe(cpu: u32) -> i32 {
    acpi_cpu_init_idle(cpu)
}

pub unsafe fn acpi_processor_ffh_lpi_enter(lpi: *mut acpi_lpi_state) -> i32 {
    let state: u32 = (*lpi).address as u32;

    if state & SBI_HSM_SUSP_NON_RET_BIT != 0 {
        CPU_PM_CPU_IDLE_ENTER_PARAM!(riscv_sbi_hart_suspend, (*lpi).index, state)
    } else {
        CPU_PM_CPU_IDLE_ENTER_RETENTION_PARAM!(riscv_sbi_hart_suspend, (*lpi).index, state)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
