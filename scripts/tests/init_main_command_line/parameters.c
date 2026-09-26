/* SPDX-License-Identifier: GPL-2.0-only */
#define console_printk original_console_printk
#define reset_devices original_reset_devices
#include "canonical.h"

int console_printk[4];
unsigned int reset_devices;
#include "parameters.inc"

int original_parameter(unsigned int callback, char *value, int level, unsigned int reset)
{
	console_printk[0] = level;
	console_printk[1] = 11;
	console_printk[2] = 22;
	console_printk[3] = 33;
	reset_devices = reset;
	switch (callback) {
	case 0: return set_reset_devices(value);
	case 1: return debug_kernel(value);
	case 2: return quiet_kernel(value);
	default: return loglevel(value);
	}
}

int original_console(unsigned int index) { return console_printk[index]; }
unsigned int original_reset(void) { return reset_devices; }
