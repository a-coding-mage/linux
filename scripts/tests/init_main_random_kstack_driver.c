/* SPDX-License-Identifier: GPL-2.0-only */
#include <linux/init.h>
#include <linux/randomize_kstack.h>
#include <linux/string.h>

extern void exit(int) __noreturn;
extern int puts(const char *);
extern struct obs_kernel_param setup_begin[], setup_end[];
extern initcall_entry_t initcall_begin[], initcall_end[];

#define CHECK(condition) do { if (!(condition)) exit(__LINE__); } while (0)
#include "kstrtobool.inc"

bool static_key_initialized = true;
static unsigned int changes, seeded;

#ifdef CONFIG_JUMP_LABEL
void static_key_enable(struct static_key *key)
{
	CHECK(key == &randomize_kstack_offset.key);
	changes++;
	key->enabled.counter = 1;
}
void static_key_disable(struct static_key *key)
{
	CHECK(key == &randomize_kstack_offset.key);
	changes++;
	key->enabled.counter = 0;
}
#endif

/* Valid zero/one key counts never enter the primitive's warning paths. */
int _printk(const char *format, ...) { exit(101); }
void __SCT__WARN_trap(struct bug_entry *bug, ...) { exit(102); }
void prandom_seed_full_state(struct rnd_state __percpu *state)
{
	CHECK(state == &kstack_rnd_state);
	CHECK(!state->s1 && !state->s2 && !state->s3 && !state->s4);
	state->s1 = 1; state->s2 = 2; state->s3 = 3; state->s4 = 4;
	seeded++;
}

int main(void)
{
	CHECK(setup_end - setup_begin == 1);
	CHECK(initcall_end - initcall_begin == 1);
	CHECK(setup_begin->early == 1);
	CHECK(!strcmp(setup_begin->str, "randomize_kstack_offset"));
	CHECK(randomize_kstack_offset.key.enabled.counter == IS_ENABLED(CONFIG_RANDOMIZE_KSTACK_OFFSET_DEFAULT));
	CHECK(setup_begin->setup_func(NULL) == -EINVAL);
	CHECK(!changes);
	for (unsigned int byte = 0; byte < 256; byte++) {
		const char suffix[] = { 0, 'n', 'f', 'N', 'F' };
		for (unsigned int second = 0; second < sizeof(suffix); second++) {
			char value[] = { byte, suffix[second], 0 };
			bool result = false;
			int expected = kstrtobool(value, &result);
			int before = randomize_kstack_offset.key.enabled.counter;
			unsigned int old_changes = changes;
			CHECK(setup_begin->setup_func(value) == expected);
			CHECK(randomize_kstack_offset.key.enabled.counter == (expected ? before : result));
#ifdef CONFIG_JUMP_LABEL
			CHECK(changes == old_changes + !expected);
#endif
		}
	}
	CHECK(!seeded);
	CHECK(initcall_from_entry(initcall_begin)() == 0);
	CHECK(seeded == 1);
	CHECK(kstack_rnd_state.s1 == 1 && kstack_rnd_state.s2 == 2 &&
	      kstack_rnd_state.s3 == 3 && kstack_rnd_state.s4 == 4);
	puts("INIT_MAIN_RANDOM_KSTACK_OK");
	return 0;
}
