// SPDX-License-Identifier: GPL-2.0

#include <linux/smp.h>

__rust_helper unsigned int rust_helper_raw_smp_processor_id(void)
{
	return raw_smp_processor_id();
}

#ifdef CONFIG_ARM64_PTR_AUTH
#include <asm/cpufeature.h>

/* Preserve the finalized-capability check and compiler alternatives used by
 * the architecture; boot key initialization and register state stay in Rust. */
__rust_helper bool rust_helper_system_supports_address_auth(void)
{
	return system_supports_address_auth();
}
#endif
