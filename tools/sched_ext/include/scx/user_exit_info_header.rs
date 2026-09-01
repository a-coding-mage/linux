/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Define struct user_exit_info which is shared between BPF and userspace parts
 * to communicate exit status and other information.
 *
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

// C includes removed:
// #include <stdio.h>
// #include <stdbool.h>
// #include "user_exit_info_common.h"

/*
 * no need to call the following explicitly if SCX_OPS_LOAD() is used
 *
 * C macro:
 * #define UEI_SET_SIZE(__skel, __ops_name, __uei_name) ({                 \
 *      u32 __len = (__skel)->struct_ops.__ops_name->exit_dump_len ?:      \
 *                  UEI_DUMP_DFL_LEN;                                      \
 *      (__skel)->rodata->__uei_name##_dump_len = __len;                   \
 *      RESIZE_ARRAY((__skel), data, __uei_name##_dump, __len);            \
 * })
 *
 * This depends on C token pasting for field names and on RESIZE_ARRAY(),
 * both supplied outside this isolated header.
 */

/*
 * C macro:
 * #define UEI_EXITED(__skel, __uei_name) ({                               \
 *      / * use __sync to force memory barrier * /                          \
 *      __sync_val_compare_and_swap(&(__skel)->data->__uei_name.kind,       \
 *                                  -1, -1);                               \
 * })
 *
 * Rust callers should perform the same compare-and-swap against the selected
 * user_exit_info.kind field. The skeleton type and field selected by
 * __uei_name are external to this isolated header.
 */

/*
 * C macro:
 * #define UEI_REPORT(__skel, __uei_name) ({                               \
 *      struct user_exit_info *__uei = &(__skel)->data->__uei_name;         \
 *      char *__uei_dump =                                                  \
 *          (__skel)->data_##__uei_name##_dump->__uei_name##_dump;         \
 *      if (__uei_dump[0] != '\0') { ... }                                  \
 *      fprintf(stderr, "EXIT: %s", __uei->reason);                        \
 *      if (__uei->msg[0] != '\0') fprintf(stderr, " (%s)", __uei->msg);   \
 *      if (__uei->exit_cpu >= 0) fprintf(stderr, " on CPU %d",            \
 *                                       __uei->exit_cpu);                 \
 *      fputs("\n", stderr);                                               \
 *      __uei->exit_code;                                                   \
 * })
 *
 * This macro also relies on C token pasting and the externally defined
 * struct user_exit_info layout from user_exit_info_common.h.
 */

/*
 * We can't import vmlinux.h while compiling user C code. Let's duplicate
 * scx_exit_code definition.
 */

/* enum scx_exit_code */
pub const SCX_ECODE_RSN_HOTPLUG: u64 = 1u64 << 32;
pub const SCX_ECODE_ACT_RESTART: u64 = 1u64 << 48;

/* enum uei_ecode_mask */
pub const UEI_ECODE_USER_MASK: u64 = (1u64 << 32) - 1;
pub const UEI_ECODE_SYS_RSN_MASK: u64 = ((1u64 << 16) - 1) << 32;
pub const UEI_ECODE_SYS_ACT_MASK: u64 = ((1u64 << 16) - 1) << 48;

/*
 * These macro interpret the ecode returned from UEI_REPORT().
 */
#[inline]
pub const fn UEI_ECODE_USER(__ecode: u64) -> u64 {
    __ecode & UEI_ECODE_USER_MASK
}

#[inline]
pub const fn UEI_ECODE_SYS_RSN(__ecode: u64) -> u64 {
    __ecode & UEI_ECODE_SYS_RSN_MASK
}

#[inline]
pub const fn UEI_ECODE_SYS_ACT(__ecode: u64) -> u64 {
    __ecode & UEI_ECODE_SYS_ACT_MASK
}

#[inline]
pub const fn UEI_ECODE_RESTART(__ecode: u64) -> bool {
    UEI_ECODE_SYS_ACT(__ecode) == SCX_ECODE_ACT_RESTART
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
