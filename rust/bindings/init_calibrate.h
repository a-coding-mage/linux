/* SPDX-License-Identifier: GPL-2.0 */
/* Canonical built-in delay calibration declarations and record layouts. */
#ifndef _RUST_BINDINGS_INIT_CALIBRATE_H
#define _RUST_BINDINGS_INIT_CALIBRATE_H

#ifdef _LINUX_INIT_H
#error "init_calibrate.h must precede linux/init.h in the binding input"
#endif
#pragma push_macro("MODULE")
#undef MODULE
#include <linux/init.h>
#include <linux/delay.h>
#include <linux/jiffies.h>
#include <linux/kstrtox.h>
#include <linux/printk.h>
#include "../../init/calibrate_percpu.h"
#pragma pop_macro("MODULE")

#endif
