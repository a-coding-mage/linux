/* SPDX-License-Identifier: GPL-2.0 */
#include <linux/kernel.h>
#include <linux/initrd.h>
#include <linux/string.h>
#include "../../init/do_mounts.h"

extern void exit(int) __noreturn;
extern int printf(const char *, ...);
extern int atoi(const char *);
extern struct obs_kernel_param setup_begin[], setup_end[];

#define CHECK(condition) do { if (!(condition)) exit(__LINE__); } while (0)
static u64 digest = 14695981039346656037ULL;
static unsigned int creates, unlinks, loads, logs;
static int load_result, unlink_error, mknod_error;
static char events[32];
static unsigned int nr_events;

static void value(u64 item) { digest = (digest ^ item) * 1099511628211ULL; }
static void text(const char *s) { for (; *s; s++) value((unsigned char)*s); value(0); }
static void event(char code) { CHECK(nr_events < sizeof(events)); events[nr_events++] = code; }

int init_unlink(const char *name)
{
	CHECK(!strcmp(name, "/dev/ram") || !strcmp(name, "/initrd.image"));
	text(name); event(!strcmp(name, "/dev/ram") ? 'u' : 'i'); unlinks++;
	return unlink_error;
}
int init_mknod(const char *name, umode_t mode, unsigned int dev)
{
	CHECK(!strcmp(name, "/dev/ram"));
	CHECK(mode == (S_IFBLK | 0600));
	CHECK(dev == new_encode_dev(Root_RAM0));
	text(name); value(mode); value(dev); event('m'); creates++;
	return mknod_error;
}
#ifdef CONFIG_BLK_DEV_RAM
int rd_load_image(void) { event('r'); loads++; return load_result; }
#endif
#ifdef CONFIG_PRINTK
int _printk(const char *format, ...) { text(format); event('p'); logs++; return 0; }
#endif

static int option(const char *name, char *argument, int early)
{
	for (struct obs_kernel_param *p = setup_begin; p < setup_end; p++) {
		if (!strcmp(p->str, name)) {
			CHECK(p->early == early);
			return p->setup_func(argument);
		}
	}
	exit(100);
}

int main(int argc, char **argv)
{
	CHECK(argc == 5);
	CHECK(setup_end - setup_begin == 3);
	CHECK(!initrd_start && !initrd_end && !initrd_below_start_ok);
	CHECK(!phys_initrd_start && !phys_initrd_size);
	initrd_start = 0x10203040; initrd_end = 0xa0b0c0d0; initrd_below_start_ok = -19;
	const char *numbers[] = { "", "0", "1", "0777", "0x12345", "0Xff", "0x", "08", "-1", "+1", " 2",
		"18446744073709551615", "18446744073709551616", "99999999999999999999999999999999999999" };
	const char *suffixes[] = { "", "k", "M", "G", "t", "P", "e", "E", "Kx", "M ", "," };
	const char *tails[] = { "", ",", ",0", ",8K", ",0x12345678", ",18446744073709551615E", ",garbage", ",1,2", "junk,4" };
	char argument[160];
	for (unsigned int alias = 0; alias < 2; alias++) {
		for (unsigned int n = 0; n < ARRAY_SIZE(numbers); n++) {
			for (unsigned int s = 0; s < ARRAY_SIZE(suffixes); s++) {
				for (unsigned int t = 0; t < ARRAY_SIZE(tails); t++) {
					strcpy(argument, numbers[n]); strcat(argument, suffixes[s]); strcat(argument, tails[t]);
					/* Alternating sentinels detect options that must leave both globals alone. */
					phys_initrd_start = 0x9988776655443322ULL + n;
					phys_initrd_size = 0x1122334455667788UL + t;
					value(option(alias ? "initrd" : "initrdmem", argument, 1));
					value(phys_initrd_start); value(phys_initrd_size);
				}
			}
		}
	}
	CHECK(initrd_start == 0x10203040 && initrd_end == 0xa0b0c0d0 && initrd_below_start_ok == -19);
	CHECK(!creates && !unlinks && !loads && !logs);
	int disabled = atoi(argv[1]);
	load_result = atoi(argv[2]); unlink_error = atoi(argv[3]); mknod_error = atoi(argv[4]);
	if (disabled) {
		CHECK(option("noinitrd", NULL, 0) == 1);
		CHECK(logs == IS_ENABLED(CONFIG_PRINTK));
	}
	unsigned int before = nr_events;
	initrd_load();
	CHECK(unlinks == (disabled ? 1 : 2));
	CHECK(creates == !disabled);
	CHECK(loads == (!disabled && IS_ENABLED(CONFIG_BLK_DEV_RAM)));
	CHECK(logs == IS_ENABLED(CONFIG_PRINTK) *
	      (disabled + (!disabled && IS_ENABLED(CONFIG_BLK_DEV_RAM) && load_result != 0)));
	CHECK(events[nr_events - 1] == 'i');
	if (!disabled) { CHECK(events[before] == 'u' && events[before + 1] == 'm'); }
	printf("INIT_MOUNTS_INITRD_OK %llu\n", (unsigned long long)digest);
	return 0;
}
