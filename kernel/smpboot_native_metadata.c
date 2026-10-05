// SPDX-License-Identifier: GPL-2.0-only
/* Native exports and full __init entry metadata; algorithms remain Rust. */
#error "SOURCE ONLY HOLD: smpboot native metadata is not admitted"

#include "smpboot_native_bindings.h"
#include <linux/export.h>

EXPORT_SYMBOL_GPL(smpboot_register_percpu_thread);
EXPORT_SYMBOL_GPL(smpboot_unregister_percpu_thread);

#ifdef CONFIG_GENERIC_SMP_IDLE_THREAD
void __init idle_thread_set_boot_cpu(void)
{
	lupos_smpboot_idle_thread_set_boot_cpu();
}

void __init idle_threads_init(void)
{
	lupos_smpboot_idle_threads_init();
}
#endif /* CONFIG_GENERIC_SMP_IDLE_THREAD */
