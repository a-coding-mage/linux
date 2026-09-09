/* SPDX-License-Identifier: (GPL-2.0-or-later OR MIT) */
/*
 * Author: David Heidelberg <david@ixit.cz>
 */

/* Charging compensation method */
pub const SMB3XX_SOFT_TEMP_COMPENSATE_NONE: i32 = 0;
pub const SMB3XX_SOFT_TEMP_COMPENSATE_CURRENT: i32 = 1;
pub const SMB3XX_SOFT_TEMP_COMPENSATE_VOLTAGE: i32 = 2;

/* Charging enable control */
pub const SMB3XX_CHG_ENABLE_SW: i32 = 0;
pub const SMB3XX_CHG_ENABLE_PIN_ACTIVE_LOW: i32 = 1;
pub const SMB3XX_CHG_ENABLE_PIN_ACTIVE_HIGH: i32 = 2;

/* Polarity of INOK signal */
pub const SMB3XX_SYSOK_INOK_ACTIVE_LOW: i32 = 0;
pub const SMB3XX_SYSOK_INOK_ACTIVE_HIGH: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
