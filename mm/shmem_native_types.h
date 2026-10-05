/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_SHMEM_TYPES_H
#define RUST_SHMEM_TYPES_H
#include "shmem_native_includes.h"
typedef pgoff_t rust_shmem_pgoff_t;
#ifdef CONFIG_SHMEM
struct shmem_falloc {
	wait_queue_head_t *waitq; /* faults into hole wait for punch to end */
	pgoff_t start;		/* start of range currently being fallocated */
	pgoff_t next;		/* the next page offset to be fallocated */
	pgoff_t nr_falloced;	/* how many new pages have been fallocated */
	pgoff_t nr_unswapped;	/* how often writeout refused to swap out */
};
struct shmem_options {
	unsigned long long blocks;
	unsigned long long inodes;
	struct mempolicy *mpol;
	kuid_t uid;
	kgid_t gid;
	umode_t mode;
	bool full_inums;
	int huge;
	int seen;
	bool noswap;
	unsigned short quota_types;
	struct shmem_quota_limits qlimits;
#if IS_ENABLED(CONFIG_UNICODE)
	struct unicode_map *encoding;
	bool strict_encoding;
#endif
#define SHMEM_SEEN_BLOCKS 1
#define SHMEM_SEEN_INODES 2
#define SHMEM_SEEN_HUGE 4
#define SHMEM_SEEN_INUMS 8
#define SHMEM_SEEN_QUOTA 16
};
enum shmem_param {
	Opt_gid,
	Opt_huge,
	Opt_mode,
	Opt_mpol,
	Opt_nr_blocks,
	Opt_nr_inodes,
	Opt_size,
	Opt_uid,
	Opt_inode32,
	Opt_inode64,
	Opt_noswap,
	Opt_quota,
	Opt_usrquota,
	Opt_grpquota,
	Opt_usrquota_block_hardlimit,
	Opt_usrquota_inode_hardlimit,
	Opt_grpquota_block_hardlimit,
	Opt_grpquota_inode_hardlimit,
	Opt_casefold_version,
	Opt_casefold,
	Opt_strict_encoding,
};
#ifdef CONFIG_TRANSPARENT_HUGEPAGE
enum huge_mode {
	HUGE_SHMEM_ENABLED_ALWAYS = 0,
	HUGE_SHMEM_ENABLED_INHERIT,
	HUGE_SHMEM_ENABLED_WITHIN_SIZE,
	HUGE_SHMEM_ENABLED_ADVISE,
	HUGE_SHMEM_ENABLED_NEVER,
};
#endif
#define BOGO_DIRENT_SIZE 20
#define BOGO_INODE_SIZE 1024
#define SHORT_SYMLINK_LEN 128
#define SHMEM_INO_BATCH 1024
#define SHMEM_HUGE_NEVER	0
#define SHMEM_HUGE_ALWAYS	1
#define SHMEM_HUGE_WITHIN_SIZE	2
#define SHMEM_HUGE_ADVISE	3
#define SHMEM_HUGE_DENY		(-1)
#define SHMEM_HUGE_FORCE	(-2)
#endif
#include "shmem_native_declarations.h"
#endif
