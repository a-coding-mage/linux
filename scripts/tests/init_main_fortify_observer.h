/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef INIT_MAIN_FORTIFY_OBSERVER_H
#define INIT_MAIN_FORTIFY_OBSERVER_H

/* Host-only failure boundary, matching init_version_driver.c. The real
 * fortified string wrappers must never reject these fixtures' valid inputs.
 * A call fails the executable rather than replacing a successful operation.
 */
#ifdef CONFIG_FORTIFY_SOURCE
/* Other linked fixture objects may be optimized even when this observer is
 * built at -O0. Keep the canonical failure declaration available in both.
 */
void __fortify_panic(const u8 reason, const size_t avail, const size_t size) __cold __noreturn;
extern int printf(const char *, ...);
extern void exit(int) __attribute__((noreturn));

void __fortify_panic(const u8 reason, const size_t avail, const size_t size)
{
	printf("unexpected fortify panic %u %zu %zu\n", reason, avail, size);
	exit(91);
}
#endif

#endif
