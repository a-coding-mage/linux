/* SPDX-License-Identifier: GPL-2.0 */
#ifndef _INIT_CALIBRATE_PERCPU_H
#define _INIT_CALIBRATE_PERCPU_H

/* Native macro boundary; storage and all calibration policy live in Rust. */
unsigned long *rust_init_calibrate_cpu_lpj(void);

#endif
