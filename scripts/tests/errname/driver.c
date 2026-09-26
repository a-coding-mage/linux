/* SPDX-License-Identifier: GPL-2.0 */
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <linux/errname.h>

extern const char *reference_errname(int);
static const char *(*volatile reference)(int) = reference_errname;
static const char *(*volatile translated)(int) = errname;

static int compare(int error)
{
	const char *original = reference(error), *rust = translated(error);
	if ((!original != !rust) || (original && strcmp(original, rust))) {
		fprintf(stderr, "%d: C=%s Rust=%s\n", error,
			original ? original : "NULL", rust ? rust : "NULL");
		return 1;
	}
	if (rust && (translated(error) != rust || error == 0))
		return 2;
	if (rust && error > 0 && translated(-error) + 1 != rust)
		return 3;
	return 0;
}

int main(void)
{
	uint32_t state = 0x93f521;
	for (int error = -4096; error <= 4096; ++error)
		if (compare(error))
			return 1;
	if (compare(INT_MIN) || compare(INT_MAX))
		return 2;
	for (unsigned int i = 0; i < 100000; ++i) {
		state = state * 1664525u + 1013904223u;
		if (compare((int)state))
			return 3;
	}
	puts("errname: 108195 values and stable sign pointers passed");
	return 0;
}
