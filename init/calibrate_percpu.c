// SPDX-License-Identifier: GPL-2.0
/* Architecture macro boundary for init/calibrate.rs; no calibration logic. */
#include <linux/build_bug.h>
#include <linux/percpu.h>
#include <linux/smp.h>
#include "calibrate_percpu.h"

#if !defined(CONFIG_X86_64) && !defined(CONFIG_ARM64)
#error "Rust delay calibration per-CPU storage needs an audited architecture"
#endif

/* Rust emits the scalar storage in the same section, with unsigned-long
 * alignment. These architectures have no additional PER_CPU_ATTRIBUTES.
 * Verify the section contract against the native header, including !SMP.
 */
#ifdef CONFIG_SMP
static_assert(__builtin_strcmp(PER_CPU_BASE_SECTION, ".data..percpu") == 0);
#else
static_assert(__builtin_strcmp(PER_CPU_BASE_SECTION, ".data") == 0);
#endif
DECLARE_PER_CPU(unsigned long, __rust_calibrate_cpu_loops_per_jiffy);

unsigned long *rust_init_calibrate_cpu_lpj(void)
{
	int this_cpu = smp_processor_id();

	return &per_cpu(__rust_calibrate_cpu_loops_per_jiffy, this_cpu);
}
