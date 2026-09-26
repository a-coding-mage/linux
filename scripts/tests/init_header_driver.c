// SPDX-License-Identifier: GPL-2.0
/* Real init.h decoding through entries on both sides of their callbacks. */
#include <linux/init.h>
#include <stddef.h>
#include <stdint.h>

extern initcall_t rust_decode(initcall_entry_t *entry);
extern int rust_call(initcall_entry_t *entry);
extern unsigned long rust_entry_size(void);
extern unsigned long rust_entry_align(void);
extern unsigned long rust_setup_size(void);
extern unsigned long rust_setup_align(void);
extern unsigned long rust_setup_callback_offset(void);
extern unsigned long rust_setup_early_offset(void);
extern void *rust_this_module(void);
#ifndef MODULE
extern int rust_setup_callback(char *value);
extern int rust_setup_call(char *value);
int c_setup_callback(char *value) { return *value + 1; }
#endif

struct module { int private; };
#ifdef MODULE
struct module __this_module;
#endif

__attribute__((section(".init_header.before"), __noinline__))
int callback_before(void) { return 17; }
__attribute__((section(".init_header.after"), __noinline__))
int callback_after(void) { return 23; }

#ifdef CONFIG_HAVE_ARCH_PREL32_RELOCATIONS
asm(".pushsection .init_header.entries,\"a\"\n"
    ".balign 4\n.global entries\nentries:\n"
    ".long callback_before - .\n.long callback_after - .\n.popsection\n");
extern initcall_entry_t entries[2];
static initcall_entry_t null_entry;
#else
static initcall_entry_t entries[] = {callback_before, callback_after};
static initcall_entry_t null_entry;
#endif

int main(void)
{
    if (rust_entry_size() != sizeof(initcall_entry_t) ||
        rust_entry_align() != _Alignof(initcall_entry_t)) return 1;
#ifdef CONFIG_HAVE_ARCH_PREL32_RELOCATIONS
    if (entries[0] >= 0 || entries[1] <= 0) return 2;
    /* Static non-PIE storage keeps zero within signed 32-bit displacement. */
    if ((uintptr_t)&null_entry > INT32_MAX) return 3;
    null_entry = -(int)(uintptr_t)&null_entry;
#endif
    for (unsigned int i = 0; i < 2; ++i) {
        initcall_t volatile original = initcall_from_entry(entries + i);
        initcall_t volatile translated = rust_decode(entries + i);
        if (original != translated || original() != translated()) return 4;
        if (translated() != (i ? 23 : 17) || rust_call(entries + i) != original()) return 5;
    }
    if (initcall_from_entry(&null_entry) || rust_decode(&null_entry) ||
        rust_call(&null_entry) != -1) return 6;
#ifndef MODULE
    if (rust_setup_size() != sizeof(struct obs_kernel_param) ||
        rust_setup_align() != _Alignof(struct obs_kernel_param) ||
        rust_setup_callback_offset() != offsetof(struct obs_kernel_param, setup_func) ||
        rust_setup_early_offset() != offsetof(struct obs_kernel_param, early)) return 7;
    char value = 0x80;
    struct obs_kernel_param volatile setup = {"test", rust_setup_callback, 1};
    if (setup.setup_func(&value) != 130 || rust_setup_call(&value) != 129) return 9;
#endif
    if (rust_this_module() != THIS_MODULE) return 8;
    return 0;
}
