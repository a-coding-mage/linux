/* SPDX-License-Identifier: GPL-2.0 */
#include <linux/errno.h>
#include <linux/errname.h>
#include <linux/init.h>
#include <linux/limits.h>
#include <linux/module.h>
#include <linux/string.h>
#include <linux/swab.h>
#include <linux/bitrev.h>

const char *errname_original(int error);
const char *errname_rust_call(int error);
unsigned int bitrev_rust_call(unsigned int operation, unsigned long long value);

static int compare_bitrev(u64 value)
{
	if (bitrev8(value) != bitrev_rust_call(0, value) ||
	    bitrev16(value) != bitrev_rust_call(1, value) ||
	    bitrev32(value) != bitrev_rust_call(2, value) ||
	    bitrev8x4(value) != bitrev_rust_call(3, value))
		return -EINVAL;
	return 0;
}

static int check_bitrev(void)
{
	static const u64 wide[] = { 0xffffULL, 0x10000ULL, 0x80000000ULL,
		0xffffffffULL, 0x100000000ULL, 0x1234567887654321ULL, ~0ULL };
	unsigned int i;

#if IS_ENABLED(CONFIG_GENERIC_BITREVERSE)
	for (i = 0; i < 256; ++i)
		if (byte_rev_table[i] != __constant_bitrev8(i))
			return -EINVAL;
#endif
	for (i = 0; i < 65536; ++i)
		if (compare_bitrev(i))
			return -EINVAL;
	for (i = 0; i < ARRAY_SIZE(wide); ++i)
		if (compare_bitrev(wide[i]))
			return -EINVAL;
	return 0;
}

static noinline const char *errname_c_call(int error)
{
	typeof(&errname) volatile actual = errname;
	return actual(error);
}

static int compare(int error)
{
	const char *original = errname_original(error);
	const char *c = errname_c_call(error);
	const char *rust = errname_rust_call(error);

	if ((!original != !c) || (!original != !rust))
		return -EINVAL;
	if (!original)
		return 0;
	if (strcmp(original, c) || strcmp(original, rust) || c != rust)
		return -EINVAL;
	if (errname_c_call(error) != c || errname_rust_call(error) != rust)
		return -EINVAL;
	if (error > 0 && (errname_c_call(-error) + 1 != c ||
			  errname_rust_call(-error) + 1 != rust))
		return -EINVAL;
	return 0;
}

static int __init errname_abi_init(void)
{
	u32 state = 0x93f521;
	int error;
	unsigned int i;

	if (check_bitrev())
		return -EINVAL;
	for (error = -4096; error <= 4096; ++error)
		if (compare(error))
			return -EINVAL;
	if (compare(INT_MIN) || compare(INT_MAX))
		return -EINVAL;
	for (i = 0; i < 100000; ++i) {
		state = state * 1664525u + 1013904223u;
		if (compare((int)state))
			return -EINVAL;
	}
	pr_info("LUPOS_BITREV_ABI_OK values=65543 callers=C,Rust table=%u\n",
		IS_ENABLED(CONFIG_GENERIC_BITREVERSE) ? 256 : 0);
	pr_info("LUPOS_ERRNAME_ABI_OK values=108195 callers=C,Rust sign-pointers=stable\n");
	return 0;
}

static void __exit errname_abi_exit(void) {}
module_init(errname_abi_init);
module_exit(errname_abi_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Original-C checks of selected errno names and bit-reversal helpers");
