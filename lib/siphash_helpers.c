// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
/*
 * Native word-at-a-time tail load for the Rust siphash owner. The load may
 * cross into an unmapped page; its exception-table fixup zero-pads, so it
 * must remain the configured architecture primitive.
 */
#include <linux/compiler.h>
#include <linux/dcache.h>
#include <asm/word-at-a-time.h>

unsigned long lupos_siphash_load_unaligned_zeropad(const void *addr);

#ifdef CONFIG_DCACHE_WORD_ACCESS
unsigned long lupos_siphash_load_unaligned_zeropad(const void *addr)
{
	return load_unaligned_zeropad(addr);
}
#endif
