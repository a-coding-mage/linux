/* SPDX-License-Identifier: GPL-2.0 */
/* Original owner per-CPU storage and configured cpumask_var_t ABI. */
static DEFINE_PER_CPU(struct balance_callback, rust_dl_push_head_storage);
static DEFINE_PER_CPU(struct balance_callback, rust_dl_pull_head_storage);
static DEFINE_PER_CPU(cpumask_var_t, rust_dl_local_cpu_mask_storage);
