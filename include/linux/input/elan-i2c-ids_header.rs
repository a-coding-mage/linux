/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Elan I2C/SMBus Touchpad device whitelist
 *
 * Copyright (c) 2013 ELAN Microelectronics Corp.
 *
 * Author: æ維 (Duson Lin) <dusonlin@emc.com.tw>
 * Author: KT Liao <kt.liao@emc.com.tw>
 * Version: 1.6.3
 *
 * Based on cyapa driver:
 * copyright (c) 2011-2012 Cypress Semiconductor, Inc.
 * copyright (c) 2011-2012 Google, Inc.
 *
 * Trademarks are the property of their respective owners.
 */

// Dependency corresponding to <linux/device-id/acpi.h>.

pub static elan_acpi_id: [acpi_device_id; 52] = [
    acpi_device_id { id: *b"ELAN0000\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0100\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0600\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0601\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0602\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0603\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0604\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0605\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0606\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0607\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0608\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0609\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN060B\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN060C\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN060F\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0610\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0611\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0612\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0615\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0616\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0617\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0618\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0619\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN061A\0", driver_data: 0 },
    /* { "ELAN061B", 0 }, not working on the Lenovo Legion Y7000 */
    acpi_device_id { id: *b"ELAN061C\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN061D\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN061E\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN061F\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0620\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0621\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0622\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0623\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0624\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0625\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0626\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0627\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0628\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0629\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN062A\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN062B\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN062C\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN062D\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN062E\0", driver_data: 0 }, /* Lenovo V340 Whiskey Lake U */
    acpi_device_id { id: *b"ELAN062F\0", driver_data: 0 }, /* Lenovo V340 Comet Lake U */
    acpi_device_id { id: *b"ELAN0631\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0632\0", driver_data: 0 },
    acpi_device_id { id: *b"ELAN0633\0", driver_data: 0 }, /* Lenovo S145 */
    acpi_device_id { id: *b"ELAN0634\0", driver_data: 0 }, /* Lenovo V340 Ice lake */
    acpi_device_id { id: *b"ELAN0635\0", driver_data: 0 }, /* Lenovo V1415-IIL */
    acpi_device_id { id: *b"ELAN0636\0", driver_data: 0 }, /* Lenovo V1415-Dali */
    acpi_device_id { id: *b"ELAN0637\0", driver_data: 0 }, /* Lenovo V1415-IGLR */
    acpi_device_id { id: [0; 9], driver_data: 0 },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
