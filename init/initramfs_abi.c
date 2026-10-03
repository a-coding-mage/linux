// SPDX-License-Identifier: GPL-2.0
/*
 * Compiler/architecture boundaries for the Rust initramfs owner.
 * No parser, file operation, decompression, initrd lifetime, or async algorithm
 * lives here. All decisions and ownership remain in initramfs.rs.
 */
#include <linux/init.h>
#include <linux/fs_struct.h>
#include <linux/mm.h>
#include <linux/printk.h>
#include <linux/slab.h>

#include "initramfs_abi.h"

/* kmalloc is an allocation-tag/compiler-builtin macro, not a linkable ABI.
 * Distinct callsites retain separate allocation profiling for all three uses.
 * Typed allocation inference sees the generated Rust object layout. Rust
 * owns every initialization, failure decision and free.
 */
struct rust_initramfs_hash *__init rust_initramfs_alloc_hash(void)
{
	return kmalloc_obj(struct rust_initramfs_hash);
}

struct rust_initramfs_buffers *__init rust_initramfs_alloc_buffers(void)
{
	return kmalloc_obj(struct rust_initramfs_buffers);
}

#ifdef CONFIG_INITRAMFS_PRESERVE_MTIME
struct rust_initramfs_dir_entry *__init rust_initramfs_alloc_dir(size_t nlen)
{
	return kmalloc_flex(struct rust_initramfs_dir_entry, name, nlen);
}
#endif

/* __va is architecture-owned and may depend on the boot address mapping. */
void *__init rust_initramfs_phys_to_virt(phys_addr_t address)
{
	return __va(address);
}

/* These existing scoped-fs primitives require current, WRITE_ONCE and the
 * architecture's warning machinery. Rust holds/restores the scope guard.
 */
struct fs_struct *__init rust_initramfs_override_init_fs(void)
{
	return __override_init_fs();
}

void __init rust_initramfs_revert_init_fs(struct fs_struct *old)
{
	__revert_init_fs(old);
}

/* Preserve dynamic-debug's native descriptor/static-key macro expansion. */
void __init rust_initramfs_detected_compression(const char *name)
{
	pr_debug("Detected %s compressed data\n", name);
}
