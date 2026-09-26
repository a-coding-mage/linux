/* SPDX-License-Identifier: GPL-2.0-only */
/* Record actual file-service interfaces without opening host console devices. */
#include <linux/fs.h>
#include <linux/file.h>
#include <linux/init_syscalls.h>
#include <linux/printk.h>
#include <linux/err.h>

extern void exit(int) __attribute__((noreturn));
extern int puts(const char *);
extern void original_console(void);
extern void console_on_rootfs(void);

static unsigned long result_pointer;
static unsigned int step, logs, cases;
static int duplicate_result;

#define CHECK(condition) do { if (!(condition)) exit(__LINE__); } while (0)

__attribute__((force_align_arg_pointer))
struct file *filp_open(const char *name, int flags, umode_t mode)
{
	CHECK(step++ == 0);
	CHECK(!strcmp(name, "/dev/console") && flags == O_RDWR && mode == 0);
	return (struct file *)result_pointer;
}

__attribute__((force_align_arg_pointer))
int init_dup(struct file *file)
{
	CHECK((unsigned long)file == result_pointer && step >= 1 && step <= 3);
	step++;
	return duplicate_result;
}

__attribute__((force_align_arg_pointer))
void fput(struct file *file)
{
	CHECK((unsigned long)file == result_pointer && step++ == 4);
}

#ifdef CONFIG_PRINTK
__attribute__((force_align_arg_pointer))
int _printk(const char *format, ...)
{
	CHECK(step == 1 && !strcmp(format, "\0013Warning: unable to open an initial console.\n"));
	logs++;
	return -17; /* The original caller ignores printk's return value. */
}
#endif

static void compare(unsigned long pointer, int error)
{
	void (*callbacks[])(void) = {original_console, console_on_rootfs};
	unsigned int expected_step = IS_ERR((void *)pointer) ? 1 : 5;
	unsigned int expected_logs = IS_ENABLED(CONFIG_PRINTK) && expected_step == 1;
	for (unsigned int owner = 0; owner < ARRAY_SIZE(callbacks); owner++) {
		result_pointer = pointer;
		duplicate_result = error;
		step = logs = 0;
		callbacks[owner]();
		CHECK(step == expected_step && logs == expected_logs);
	}
	cases++;
}

int main(void)
{
	for (long error = -MAX_ERRNO; error < 0; error++)
		compare((unsigned long)error, 0);
	for (unsigned int i = 0; i < 4; i++) {
		unsigned long pointers[] = {0, 1, 0x12345000, (unsigned long)-MAX_ERRNO - 1};
		for (int result = -3; result <= 3; result++)
			compare(pointers[i], result);
	}
	CHECK(cases == MAX_ERRNO + 28);
	puts("INIT_MAIN_CONSOLE_OK cases=4123");
	return 0;
}
