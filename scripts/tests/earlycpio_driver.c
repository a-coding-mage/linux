/* SPDX-License-Identifier: GPL-2.0-only */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdarg.h>
#include <string.h>
#include <linux/earlycpio.h>
#include <linux/kernel.h>

extern struct cpio_data c_find_cpio_data(const char *, void *, size_t, long *);
static typeof(&find_cpio_data) volatile actual_call = find_cpio_data;
static unsigned char storage[4096] __attribute__((aligned(16)));
static unsigned int active, warnings[2];
static char diagnostics[2][1024];

#define CHECK(condition) do { if (!(condition)) { \
	fprintf(stderr, "earlycpio failure at line %d: %s\n", __LINE__, #condition); \
	exit(1); } } while (0)

int _printk(const char *format, ...)
{
#ifdef CONFIG_PRINTK
	va_list arguments;
	warnings[active]++;
	va_start(arguments, format);
	vsnprintf(diagnostics[active], sizeof(diagnostics[active]), format, arguments);
	va_end(arguments);
#else
	(void)format;
#endif
	return 0;
}

static void field(unsigned char *header, unsigned int index, uint32_t value, int uppercase)
{
	const char *hex = uppercase ? "0123456789ABCDEF" : "0123456789abcdef";
	unsigned int width = index ? 8 : 6;
	unsigned int start = index ? 6 + 8 * (index - 1) : 0;
	for (unsigned int i = 0; i < width; ++i) {
		header[start + width - i - 1] = hex[value & 15];
		value >>= 4;
	}
}

static void header(unsigned char *p, uint32_t mode, uint32_t namesize, uint32_t filesize)
{
	for (unsigned int i = 0; i < 14; ++i)
		field(p, i, 0, 0);
	field(p, 0, 0x070701, 0);
	field(p, 2, mode, 0);
	field(p, 7, filesize, 0);
	field(p, 12, namesize, 0);
}

static unsigned char *append(unsigned char *p, const char *name, uint32_t mode,
			     const char *payload, size_t size)
{
	size_t namesize = strlen(name) + 1;
	header(p, mode, namesize, size);
	memcpy(p + 110, name, namesize);
	p = PTR_ALIGN(p + 110 + namesize, 4);
	memcpy(p, payload, size);
	return PTR_ALIGN(p + size, 4);
}

static struct cpio_data compare(const char *path, unsigned char *data, size_t length, int null_offset)
{
	struct { long before, offset, after; } output[2] = {
		{ 9182, -77113, 5518 }, { 9182, -77113, 5518 } };
	struct cpio_data results[2];
	unsigned char before[sizeof(storage)];

	memcpy(before, storage, sizeof(storage));
	memset(warnings, 0, sizeof(warnings));
	memset(diagnostics, 0, sizeof(diagnostics));
	active = 0;
	results[0] = c_find_cpio_data(path, data, length, null_offset ? NULL : &output[0].offset);
	active = 1;
	results[1] = actual_call(path, data, length, null_offset ? NULL : &output[1].offset);
	CHECK(results[0].data == results[1].data && results[0].size == results[1].size);
	CHECK(!memcmp(results[0].name, results[1].name, MAX_CPIO_FILE_NAME));
	CHECK(!memcmp(output, output + 1, sizeof(output[0])));
	CHECK(output[1].before == 9182 && output[1].after == 5518);
	if (!results[0].data || null_offset)
		CHECK(output[1].offset == -77113);
	CHECK(warnings[0] == warnings[1]);
	CHECK(!memcmp(diagnostics[0], diagnostics[1], sizeof(diagnostics[0])));
	CHECK(!memcmp(before, storage, sizeof(storage)));
	return results[1];
}

static void ordinary_archives(void)
{
	static const char *const names[] = { "dir/a", "dir/", "dir/0123456789abcdef",
		"dir/0123456789abcdefg", "dir/0123456789abcdefgh", "other", "dir/a/b" };
	for (unsigned int alignment = 0; alignment < 8; ++alignment) {
		for (unsigned int i = 0; i < sizeof(names) / sizeof(*names); ++i) {
			unsigned char *start = storage + alignment;
			unsigned char *end;
			memset(storage, 0, sizeof(storage));
			end = append(start, names[i], 0100644, "hello", 5);
			for (size_t length = 0; length <= (size_t)(end - start) + 4; ++length) {
				compare("dir/", start, length, length & 1);
				compare("", start, length, 0);
				compare(names[i], start, length, 0);
			}
			struct cpio_data result = compare("dir/", start, end - start, 0);
			CHECK((result.data != NULL) == !strncmp(names[i], "dir/", 4));
			if (result.data) {
				CHECK(result.size == 5 && !memcmp(result.data, "hello", 5));
				CHECK(!strncmp(result.name, names[i] + 4, MAX_CPIO_FILE_NAME - 1));
				CHECK(result.name[MAX_CPIO_FILE_NAME - 1] == 0);
#ifdef CONFIG_PRINTK
				CHECK(warnings[1] == (strlen(names[i]) + 1 - 4 >= MAX_CPIO_FILE_NAME));
				if (warnings[1]) {
					char expected[1024];
					snprintf(expected, sizeof(expected),
						 KERN_WARNING "File %s exceeding MAX_CPIO_FILE_NAME [%d]\n",
						 names[i], MAX_CPIO_FILE_NAME);
					CHECK(!strcmp(diagnostics[1], expected));
				}
#endif
			}
		}
	}
}

static void iteration(void)
{
	unsigned char *start = storage, *p;
	memset(storage, 0, sizeof(storage));
	/* Zero padding is skipped in four-byte steps, not one byte at a time. */
	p = start + 12;
	p = append(p, "dir/ignored", 0040755, "", 0);
	p = append(p, "other/a", 0100644, "other", 5);
	p = append(p, "dir/first", 0100644, "one", 3);
	p = append(p, "dir/second", 0100644, "two", 3);
	p = append(p, "TRAILER!!!", 0, "", 0);
	for (unsigned int provider = 0; provider < 2; ++provider) {
		long offset = -1, second = -1;
		struct cpio_data one, two;
		active = provider;
		one = provider ? actual_call("dir/", start, p - start, &offset) :
			c_find_cpio_data("dir/", start, p - start, &offset);
		CHECK(one.data && !strcmp(one.name, "first") && offset > 0);
		two = provider ? actual_call("dir/", start + offset, p - start - offset, &second) :
			c_find_cpio_data("dir/", start + offset, p - start - offset, &second);
		CHECK(two.data && !strcmp(two.name, "second") && second > 0);
		compare("dir/", start + offset + second, p - start - offset - second, 0);
	}
}

static void malformed_headers(void)
{
	static const uint32_t sizes[] = { 0, 1, 2, 3, 4, 17, 18, 19, 100, 0x7fffffff, 0xfffffffc, 0xffffffff };
	static const uint32_t modes[] = { 0, 0040000, 0100000, 0120000, 0170000, 0xffffffff };
	static const unsigned char digits[] = { 0, '/', ':', '@', 'G', '`', 'g', 0x80, 0xff, 'A', 'f' };
	for (unsigned int alignment = 0; alignment < 4; ++alignment) {
		unsigned char *p = storage + alignment;
		memset(storage, 0, sizeof(storage));
		append(p, "dir/member", 0100644, "payload", 7);
		for (unsigned int offset = 0; offset < 110; ++offset) {
			unsigned char old = p[offset];
			for (unsigned int i = 0; i < sizeof(digits); ++i) {
				p[offset] = digits[i];
				compare("dir/", p, 256, offset & 1);
				compare("", p, 256, 0);
			}
			p[offset] = old;
		}
		for (unsigned int i = 0; i < sizeof(sizes) / sizeof(*sizes); ++i) {
			for (unsigned int j = 0; j < sizeof(sizes) / sizeof(*sizes); ++j) {
				field(p, 12, sizes[i], 1);
				field(p, 7, sizes[j], 1);
				compare("dir/", p, 256, 0);
				compare("", p, 256, 0);
				compare("path/longer/than/name", p, 256, 1);
			}
		}
		field(p, 12, 11, 0);
		field(p, 7, 7, 0);
		for (unsigned int i = 0; i < sizeof(modes) / sizeof(*modes); ++i) {
			field(p, 2, modes[i], 1);
			compare("dir/", p, 256, 0);
		}
		field(p, 2, 0100644, 0);
		for (uint32_t magic = 0x070700; magic <= 0x070703; ++magic) {
			field(p, 0, magic, 1);
			struct cpio_data result = compare("dir/", p, 256, 0);
			CHECK((result.data != NULL) == (magic == 0x070701 || magic == 0x070702));
		}
	}
}

int main(void)
{
	CHECK(MAX_CPIO_FILE_NAME == 18);
	compare("", NULL, 0, 0);
	compare("", NULL, 110, 0); /* Strict greater-than loop condition. */
	ordinary_archives();
	iteration();
	malformed_headers();
	puts("earlycpio: headers, bounds, iteration, names and diagnostics passed");
	return 0;
}
