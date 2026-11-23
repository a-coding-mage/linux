// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaves for the Rust string owner: load_unaligned_zeropad() with its
 * exception-table fixup, and the WARN_ON_ONCE() and BUG_ON() sites of
 * sized_strscpy() and strlcat(). The string routines live in lib/string.rs.
 *
 * Built with the original string.o flags (-ffreestanding, no KCOV) and
 * __NO_FORTIFY, like the C unit it accompanies.
 */
#define __NO_FORTIFY
#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/ctype.h>
#include <linux/limits.h>
#include <asm/word-at-a-time.h>

/* lib/string.rs spells the 64-bit little-endian word-at-a-time primitives. */
static_assert(sizeof(unsigned long) == 8);
static_assert(_U == 0x01);
#ifdef __BIG_ENDIAN
#error "lib/string.rs implements the little-endian word-at-a-time primitives"
#endif

unsigned long lupos_string_load_unaligned_zeropad(const void *addr);
void lupos_string_warn_strscpy_count(void);
void __noreturn lupos_string_strlcat_bug(void);

unsigned long lupos_string_load_unaligned_zeropad(const void *addr)
{
	return load_unaligned_zeropad(addr);
}

/* sized_strscpy(): count > INT_MAX. */
void lupos_string_warn_strscpy_count(void)
{
	WARN_ON_ONCE(1);
}

/* strlcat(): dsize >= count would be a bug. */
void __noreturn lupos_string_strlcat_bug(void)
{
	BUG();
}
