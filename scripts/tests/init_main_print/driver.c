/* SPDX-License-Identifier: GPL-2.0-only */
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern void original_print(const char *);
extern void rust_print(const char *);
extern int original_argument_evaluation(int *);
extern int rust_argument_evaluation(int *);
static void (*volatile rust_call)(const char *) = rust_print;
static unsigned int active, calls[2], cases;
static size_t lengths[2];
static char logs[2][1024 * 1024];

#define CHECK(condition) do { if (!(condition)) { \
	fprintf(stderr, "init/main print failure line %d case %u\n", __LINE__, cases); \
	exit(1); } } while (0)

/* The unchanged kernel caller uses its real eight-byte/no-SSE stack policy;
 * align this instrumented boundary before entering the host's libc varargs. */
__attribute__((force_align_arg_pointer))
int _printk(const char *format, ...)
{
	va_list args;
	int size;
	size_t available = sizeof(logs[active]) - lengths[active];
	CHECK(!strcmp(format, "\0015%s%s\n") || !strcmp(format, "\0015%s%.*s%s\n"));
	va_start(args, format);
	size = vsnprintf(logs[active] + lengths[active], available, format, args);
	va_end(args);
	CHECK(size >= 0 && (size_t)size < available);
	lengths[active] += (size_t)size + 1; /* Retain each individual printk record. */
	calls[active]++;
	return size;
}

static void compare(const char *line)
{
	char *before = strdup(line);
	CHECK(before);
	memset(lengths, 0, sizeof(lengths));
	memset(calls, 0, sizeof(calls));
	active = 0;
	original_print(line);
	active = 1;
	rust_call(line);
	CHECK(calls[0] == calls[1] && lengths[0] == lengths[1]);
	CHECK(!memcmp(logs[0], logs[1], lengths[0]));
	CHECK(!strcmp(before, line));
	free(before);
	cases++;
}

int main(void)
{
	static const char *const ordinary[] = {
		"", " ", "  ", "a", " a", "a ", " a ", " a  b ", "\t\n\r",
		"quotes=\"The quick brown fox jumps over the lazy dog.\"",
		"\xff \x80\tword\nnext", "a=1 b=2 c=3 d=4 e=5 f=6",
		"a                    b          c                        ",
	};
	char buffer[8193];
	unsigned int random = 0x194721;
	for (size_t i = 0; i < sizeof(ordinary) / sizeof(*ordinary); i++)
		compare(ordinary[i]);
	for (unsigned int length = 0; length <= 10; length++) {
		for (unsigned int pattern = 0; pattern < (1U << length); pattern++) {
			for (unsigned int i = 0; i < length; i++)
				buffer[i] = pattern & (1U << i) ? ' ' : 'x';
			buffer[length] = 0;
			compare(buffer);
		}
	}
	for (unsigned int length = 0; length <= 4096; length++) {
		if (length > 84 && length < 1000 && length % 127)
			continue;
		memset(buffer, 'x', length);
		buffer[length] = 0;
		compare(buffer);
		if (length) {
			buffer[length / 2] = ' ';
			compare(buffer);
		}
	}
	for (unsigned int trial = 0; trial < 96; trial++) {
		unsigned int length = trial * 83;
		for (unsigned int i = 0; i < length; i++) {
			random = random * 1664525U + 1013904223U;
			buffer[i] = random % 5 ? (char)(1 + random % 255) : ' ';
		}
		buffer[length] = 0;
		compare(buffer);
	}
	int before[2] = { 0, 0 }, returns[2];
	for (active = 0; active < 2; active++) {
		lengths[active] = calls[active] = 0;
		returns[active] = active ? rust_argument_evaluation(&before[active]) :
			original_argument_evaluation(&before[active]);
	}
	CHECK(before[0] == 1 && before[1] == 1);
	CHECK(returns[0] == returns[1]);
	CHECK(calls[0] == calls[1] && lengths[0] == lengths[1]);
	CHECK(!memcmp(logs[0], logs[1], lengths[0]));
	printf("INIT_MAIN_PRINT_OK cases=%u argument-evaluations=1\n", cases);
	return 0;
}
