// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    fn arm_smccc_get_soc_id_version() -> i32;
}

const SMCCC_SOC_ID_T241: i32 = 0x036b0241;

#[no_mangle]
pub unsafe extern "C" fn acpi_arch_thermal_cpufreq_pctg() -> i32 {
    let soc_id: i32 = arm_smccc_get_soc_id_version();

    /*
     * Check JEP106 code for NVIDIA Tegra241 chip (036b:0241) and
     * reduce the CPUFREQ Thermal reduction percentage to 5%.
     */
    if soc_id == SMCCC_SOC_ID_T241 {
        return 5;
    }

    0
}

// EXPORT_SYMBOL_GPL(acpi_arch_thermal_cpufreq_pctg);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
