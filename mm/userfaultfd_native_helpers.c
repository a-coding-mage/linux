// SPDX-License-Identifier: GPL-2.0-only
#define RUST_UFFD_DEFINE_LEAVES
#include "userfaultfd_native_primitives.h"

/* Compiler-defined syscall ABI, metadata, sign extension and initcall entry.
 * The access policy, allocation, descriptor publication and initialization
 * algorithms are the named Rust implementations. */
extern long rust_uffd_sys_userfaultfd(int flags);
SYSCALL_DEFINE1(userfaultfd, int, flags)
{
    return rust_uffd_sys_userfaultfd(flags);
}
extern int __init rust_uffd_init(void);
__initcall(rust_uffd_init);

#ifdef CONFIG_SYSCTL
extern int rust_uffd_sysctl_unprivileged_userfaultfd;
static const struct ctl_table vm_userfaultfd_table[] = {
    {
        .procname = "unprivileged_userfaultfd",
        .data = &rust_uffd_sysctl_unprivileged_userfaultfd,
        .maxlen = sizeof(rust_uffd_sysctl_unprivileged_userfaultfd),
        .mode = 0644,
        .proc_handler = proc_dointvec_minmax,
        .extra1 = SYSCTL_ZERO,
        .extra2 = SYSCTL_ONE,
    },
};
void __init rust_uffd_register_sysctl_table(void)
{
    register_sysctl_init("vm", vm_userfaultfd_table);
}
#endif
