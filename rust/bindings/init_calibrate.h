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

/*
 * bindgen keeps the first HZ macro it sees, the uapi USER_HZ-style 100 from
 * asm-generic/param.h, not the kernel's HZ (CONFIG_HZ). Export the
 * configured tick rate as a Clang-evaluated constant instead.
 */
enum { LUPOS_CALIBRATE_HZ = HZ };

#endif
