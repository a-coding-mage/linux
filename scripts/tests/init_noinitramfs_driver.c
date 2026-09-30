/* SPDX-License-Identifier: GPL-2.0-only */
#include <linux/init.h>
#include <linux/stat.h>
#include <linux/kdev_t.h>
#include <linux/init_syscalls.h>
#include <linux/umh.h>
#include <linux/string.h>
#include <linux/printk.h>

extern void exit(int) __noreturn;
extern int printf(const char *, ...);
extern initcall_entry_t rootfs_begin[], rootfs_end[];

#define CHECK(condition) do { if (!(condition)) exit(__LINE__); } while (0)
static int results[3];
static unsigned int calls, logs, enabled;
static u64 digest = 14695981039346656037ULL;
static void value(u64 input) { digest = (digest ^ input) * 1099511628211ULL; }

void __usermodehelper_set_disable_depth(enum umh_disable_depth depth)
{
	CHECK(!calls && !enabled && !logs && depth == UMH_ENABLED);
	enabled++; value(depth);
}

int init_mkdir(const char *name, umode_t mode)
{
	CHECK(enabled == 1 && (calls == 0 || calls == 2));
	CHECK(!strcmp(name, calls ? "/root" : "/dev"));
	CHECK(mode == (calls ? 0700 : 0755));
	value(mode);
	return results[calls++];
}

int init_mknod(const char *name, umode_t mode, unsigned int device)
{
	CHECK(enabled == 1 && calls == 1 && !logs);
	CHECK(!strcmp(name, "/dev/console"));
	CHECK(mode == (S_IFCHR | S_IRUSR | S_IWUSR));
	CHECK(device == new_encode_dev(MKDEV(5, 1)));
	value(mode); value(device);
	return results[calls++];
}

#ifdef CONFIG_PRINTK
int _printk(const char *format, ...)
{
	CHECK(enabled == 1 && calls >= 1 && calls <= 3 && !logs);
	CHECK(results[calls - 1] < 0);
	CHECK(!strcmp(format, KERN_WARNING "Failed to create a rootfs\n"));
	logs++;
	return 37;
}
#endif

int main(void)
{
	const int errors[] = { (-2147483647 - 1), -517, -17, -1, 0, 1, 2147483647 };
	unsigned int cases = 0;
	CHECK(rootfs_end - rootfs_begin == 1);
	for (unsigned int first = 0; first < ARRAY_SIZE(errors); first++) {
		for (unsigned int second = 0; second < ARRAY_SIZE(errors); second++) {
			for (unsigned int third = 0; third < ARRAY_SIZE(errors); third++) {
				results[0] = errors[first]; results[1] = errors[second]; results[2] = errors[third];
				calls = logs = enabled = 0;
				int expected = 0;
				unsigned int count = 3;
				for (unsigned int index = 0; index < 3; index++) {
					if (results[index] < 0) { expected = results[index]; count = index + 1; break; }
				}
				int actual = initcall_from_entry(rootfs_begin)();
				CHECK(actual == expected && calls == count && enabled == 1);
				CHECK(logs == (IS_ENABLED(CONFIG_PRINTK) && expected < 0));
				value((unsigned int)actual); value(calls); value(logs);
				cases++;
			}
		}
	}
	printf("INIT_NOINITRAMFS_OK cases=%u digest=%llu\n", cases, (unsigned long long)digest);
	return 0;
}
