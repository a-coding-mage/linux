/* SPDX-License-Identifier: GPL-2.0 */
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/slab.h>
#include <linux/string.h>

char **argv_split_original(gfp_t flags, const char *source, int *count);
void argv_free_original(char **vector);
char **argv_split_rust_call(gfp_t flags, const char *source, int *count);
void argv_free_rust_call(char **vector);

static unsigned int completed;

static noinline char **argv_split_c_call(gfp_t flags, const char *source, int *count)
{
	typeof(&argv_split) volatile actual = argv_split;
	return actual(flags, source, count);
}

static noinline void argv_free_c_call(char **vector)
{
	typeof(&argv_free) volatile actual = argv_free;
	actual(vector);
}

static int compare(const char *text, gfp_t flags, bool null_count)
{
	char **vectors[3];
	char *source = NULL;
	int counts[3] = { 7391, 7391, 7391 };
	size_t length = text ? strlen(text) : 0;
	unsigned int argc = 0, i, j;
	int error = -EINVAL;

	if (text) {
		source = kstrdup(text, GFP_KERNEL);
		if (!source)
			return -ENOMEM;
	}
	vectors[0] = argv_split_original(flags, source, null_count ? NULL : &counts[0]);
	vectors[1] = argv_split_c_call(flags, source, null_count ? NULL : &counts[1]);
	vectors[2] = argv_split_rust_call(flags, source, null_count ? NULL : &counts[2]);
	for (i = 0; i < ARRAY_SIZE(vectors); ++i) {
		if (!text && (vectors[i] || counts[i] != 7391))
			goto out;
		if (text && !vectors[i]) {
			error = -ENOMEM;
			goto out;
		}
		if (null_count && counts[i] != 7391)
			goto out;
	}
	if (!text)
		goto success;
	if (strcmp(source, text))
		goto out;
	/* Each provider has finished its private snapshot before this change. */
	memset(source, 'X', length);
	while (vectors[0][argc]) {
		if (++argc > length)
			goto out;
	}
	for (i = 0; i < ARRAY_SIZE(vectors); ++i) {
		char *snapshot = vectors[i][-1];

		if (!snapshot || snapshot == source ||
		    (unsigned long)vectors[i] % __alignof__(char *) ||
		    ksize(vectors[i] - 1) < (argc + 2) * sizeof(char *) ||
		    ksize(snapshot) < length + 1 ||
		    memcmp(snapshot, vectors[0][-1], length + 1) ||
		    (!null_count && counts[i] != argc) || vectors[i][argc])
			goto out;
		for (j = 0; j < argc; ++j) {
			if ((unsigned long)vectors[i][j] < (unsigned long)snapshot ||
			    (unsigned long)vectors[i][j] >= (unsigned long)snapshot + length ||
			    vectors[i][j] - snapshot != vectors[0][j] - vectors[0][-1] ||
			    strcmp(vectors[i][j], vectors[0][j]))
				goto out;
		}
	}
	/* Even the public NULL slot is not an allocation owner. */
	for (i = 0; i < ARRAY_SIZE(vectors); ++i)
		for (j = 0; j <= argc; ++j)
			vectors[i][j] = ERR_PTR(-EINVAL);
success:
	completed++;
	error = 0;
out:
	if (vectors[0])
		argv_free_original(vectors[0]);
	if (vectors[1])
		argv_free_c_call(vectors[1]);
	if (vectors[2])
		argv_free_rust_call(vectors[2]);
	kfree(source);
	return error;
}

static int exercise(const char *text)
{
	static const gfp_t flags[] = { GFP_KERNEL, GFP_KERNEL | __GFP_ZERO,
		GFP_KERNEL_ACCOUNT | __GFP_ZERO };
	unsigned int i, null_count;
	int error;

	for (i = 0; i < ARRAY_SIZE(flags); ++i)
		for (null_count = 0; null_count < 2; ++null_count) {
			error = compare(text, flags[i], null_count);
			if (error)
				return error;
		}
	return 0;
}

static int __init argv_split_abi_init(void)
{
	static const char *const cases[] = { "", " \t\r\n\v\f", "word", " a b  c ",
		"'two words' \\\"still split\\\"", "a\240b", "a\0ignored tail" };
	char bytes[129];
	u32 random = 0x748291;
	unsigned int i, j;
	int error = exercise(NULL);

	if (error)
		return error;
	for (i = 0; i < ARRAY_SIZE(cases); ++i) {
		error = exercise(cases[i]);
		if (error)
			return error;
	}
	for (i = 0; i < 256; ++i) {
		bytes[0] = i; bytes[1] = 'a'; bytes[2] = i; bytes[3] = 'b'; bytes[4] = 0;
		error = exercise(bytes);
		if (error)
			return error;
	}
	for (i = 0; i < 128; ++i) {
		for (j = 0; j < sizeof(bytes) - 1; ++j) {
			random = random * 1664525U + 1013904223U;
			bytes[j] = random >> 24;
		}
		bytes[j] = 0;
		error = exercise(bytes);
		if (error)
			return error;
	}
	if (completed != 2352)
		return -EINVAL;
	pr_info("LUPOS_ARGV_SPLIT_ABI_OK cases=2352 callers=C,Rust flags=3 hidden-owner=stable\n");
	return 0;
}

static void __exit argv_split_abi_exit(void) {}
module_init(argv_split_abi_init);
module_exit(argv_split_abi_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Original-C checks of selected argument-vector ownership and callers");
