/* SPDX-License-Identifier: GPL-2.0 */
/*
 * NOTE: This header *must not* be included.
 *
 * If you're implementing a GPIO driver, only include <linux/gpio/driver.h>
 * If you're implementing a GPIO consumer, only include <linux/gpio/consumer.h>
 * If you're using the legacy interfaces, include <linux/gpio/legacy.h>
 */

// C header guard: __LINUX_GPIO_H

// C dependency: #include <linux/types.h>

// When CONFIG_GPIOLIB is enabled, the C header includes:
// #include <linux/gpio/consumer.h>

// When CONFIG_GPIOLIB_LEGACY is enabled, the C header includes:
// #include <linux/gpio/legacy.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
