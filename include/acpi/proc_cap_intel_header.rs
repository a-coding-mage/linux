/* SPDX-License-Identifier: GPL-2.0 */

/* Vendor specific processor capabilities bit definition
 * for Intel processors. Those bits are used to convey OSPM
 * power management capabilities to the platform.
 */

pub const ACPI_PROC_CAP_P_FFH: u32 = 0x0001;
pub const ACPI_PROC_CAP_C_C1_HALT: u32 = 0x0002;
pub const ACPI_PROC_CAP_T_FFH: u32 = 0x0004;
pub const ACPI_PROC_CAP_SMP_C1PT: u32 = 0x0008;
pub const ACPI_PROC_CAP_SMP_C2C3: u32 = 0x0010;
pub const ACPI_PROC_CAP_SMP_P_SWCOORD: u32 = 0x0020;
pub const ACPI_PROC_CAP_SMP_C_SWCOORD: u32 = 0x0040;
pub const ACPI_PROC_CAP_SMP_T_SWCOORD: u32 = 0x0080;
pub const ACPI_PROC_CAP_C_C1_FFH: u32 = 0x0100;
pub const ACPI_PROC_CAP_C_C2C3_FFH: u32 = 0x0200;
pub const ACPI_PROC_CAP_SMP_P_HWCOORD: u32 = 0x0800;
pub const ACPI_PROC_CAP_COLLAB_PROC_PERF: u32 = 0x1000;

pub const ACPI_PROC_CAP_EST_CAPABILITY_SMP: u32 =
    ACPI_PROC_CAP_SMP_C1PT | ACPI_PROC_CAP_C_C1_HALT | ACPI_PROC_CAP_P_FFH;

pub const ACPI_PROC_CAP_EST_CAPABILITY_SWSMP: u32 =
    ACPI_PROC_CAP_SMP_C1PT
        | ACPI_PROC_CAP_C_C1_HALT
        | ACPI_PROC_CAP_SMP_P_SWCOORD
        | ACPI_PROC_CAP_SMP_P_HWCOORD
        | ACPI_PROC_CAP_P_FFH;

pub const ACPI_PROC_CAP_C_CAPABILITY_SMP: u32 =
    ACPI_PROC_CAP_SMP_C2C3
        | ACPI_PROC_CAP_SMP_C1PT
        | ACPI_PROC_CAP_C_C1_HALT
        | ACPI_PROC_CAP_C_C1_FFH
        | ACPI_PROC_CAP_C_C2C3_FFH;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
