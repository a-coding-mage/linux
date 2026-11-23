// SPDX-License-Identifier: GPL-2.0-only
/*
 * Native leaves for the Rust string-helpers owner: the kmalloc(),
 * kcalloc() and kzalloc_objs() macro families with their allocation hooks,
 * devres_alloc() with the release callback's name, and the
 * kstrdup_quotable() warning. Formatting, escaping and string logic live in
 * lib/string_helpers.rs.
 */
#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/ctype.h>
#include <linux/device.h>
#include <linux/slab.h>
#include <linux/string_helpers.h>

/* lib/string_helpers.rs spells these values. */
static_assert(STRING_UNITS_10 == 0);
static_assert(STRING_UNITS_2 == 1);
static_assert(STRING_UNITS_MASK == BIT(0));
static_assert(STRING_UNITS_NO_SPACE == BIT(30));
static_assert(STRING_UNITS_NO_BYTES == BIT(31));
static_assert(UNESCAPE_SPACE == BIT(0) && UNESCAPE_OCTAL == BIT(1) &&
	      UNESCAPE_HEX == BIT(2) && UNESCAPE_SPECIAL == BIT(3));
static_assert(ESCAPE_SPACE == BIT(0) && ESCAPE_SPECIAL == BIT(1) &&
	      ESCAPE_NULL == BIT(2) && ESCAPE_OCTAL == BIT(3) &&
	      ESCAPE_NP == BIT(4) && ESCAPE_HEX == BIT(5) &&
	      ESCAPE_NA == BIT(6) && ESCAPE_NAP == BIT(7) &&
	      ESCAPE_APPEND == BIT(8));
static_assert(_U == 0x01 && _L == 0x02 && _D == 0x04 && _P == 0x10 &&
	      _S == 0x20 && _SP == 0x80);

void *lupos_string_helpers_kmalloc(size_t size, gfp_t gfp);
void *lupos_string_helpers_kcalloc(size_t n, size_t size, gfp_t gfp);
int *lupos_string_helpers_kzalloc_ints(int count);
void *lupos_string_helpers_devres_alloc(dr_release_t release, size_t size);
void lupos_string_helpers_warn_quotable(void);

void *lupos_string_helpers_kmalloc(size_t size, gfp_t gfp)
{
	return kmalloc(size, gfp);
}

void *lupos_string_helpers_kcalloc(size_t n, size_t size, gfp_t gfp)
{
	return kcalloc(n, size, gfp);
}

/* parse_int_array(): ints = kzalloc_objs(*ints, nints + 1). */
int *lupos_string_helpers_kzalloc_ints(int count)
{
	int *ints;

	ints = kzalloc_objs(*ints, count);
	return ints;
}

/* devres_alloc(devm_kfree_strarray, sizeof(*ptr), GFP_KERNEL). */
void *lupos_string_helpers_devres_alloc(dr_release_t release, size_t size)
{
	return __devres_alloc_node(release, size, GFP_KERNEL, NUMA_NO_NODE,
				   "devm_kfree_strarray");
}

/* kstrdup_quotable(): the escaped length must match the measured one. */
void lupos_string_helpers_warn_quotable(void)
{
	WARN_ON(1);
}
