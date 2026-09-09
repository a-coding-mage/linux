/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: __LINUX_MFD_SC27XX_PMIC_H */

extern "C" {
    pub fn sprd_pmic_detect_charger_type(dev: *mut device) -> usb_charger_type;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
