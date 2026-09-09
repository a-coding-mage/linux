// Translated from xfs_trans_resv.c. External XFS symbols are supplied by dependent modules.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables, unused_mut)]

// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright(c) 2000-2003,2005 Silicon Graphics, Inc.
 * Copyright(C) 2010 Red Hat, Inc.
 * All Rights Reserved.
 */
// #include "xfs_platform.h"
// #include "xfs_fs.h"
// #include "xfs_shared.h"
// #include "xfs_format.h"
// #include "xfs_log_format.h"
// #include "xfs_trans_resv.h"
// #include "xfs_mount.h"
// #include "xfs_da_format.h"
// #include "xfs_da_btree.h"
// #include "xfs_inode.h"
// #include "xfs_bmap_btree.h"
// #include "xfs_quota.h"
// #include "xfs_trans.h"
// #include "xfs_qm.h"
// #include "xfs_trans_space.h"
// #include "xfs_rtbitmap.h"
// #include "xfs_attr_item.h"
// #include "xfs_log.h"
// #include "xfs_defer.h"
// #include "xfs_bmap_item.h"
// #include "xfs_extfree_item.h"
// #include "xfs_rmap_item.h"
// #include "xfs_refcount_item.h"
// #include "xfs_trace.h"

const _ALLOC: bool = true;
const _FREE: bool = false;
/*
 * A buffer has a format structure overhead in the log in *to the data, so we need to take this into account when *space in a transaction for a buffer.  Round the space required *to a multiple of 128 bytes so that we don't change the *reservation that has been used for this overhead.
 */
 u32
xfs_buf_log_overhead(void)
{
	return round_up(sizeof(struct xlog_op_header) +
			sizeof(struct xfs_buf_log_format), 128);
}

/*
 * Calculate out transaction log reservation per item in bytes.
 *
 * The nbufs argument is used to indicate the number of items *will be changed in a transaction.  size is used to tell how *bytes should be reserved per item.
 */
 u32
xfs_calc_buf_res(
	u32		nbufs,
	u32		size)
{
	return nbufs * (size + xfs_buf_log_overhead());
}

/*
 * Per-extent log reservation for the btree changes involved in freeing *allocating an extent.  In classic XFS there were two trees that will *modified(bnobt + cntbt).  With rmap enabled, there are three trees
 * (rmapbt).  The number of blocks reserved is based on the formula:
 *
 * num trees * ((2 blocks/*max depth) - 1)
 *
 * Keep in mind that max depth is calculated separately for each type of tree.
 */
u32
xfs_allocfree_block_count(
	*mp,
	u32		num_ops)
{
	let mut blocks: u32;
	blocks = num_ops * 2 * (2 * mp->m_alloc_maxlevels - 1);
	if(xfs_has_rmapbt(mp))
		blocks += num_ops * (2 * mp->m_rmap_maxlevels - 1);
	return blocks;
}

/*
 * Per-extent log reservation for refcount btree changes.  These are never *in the same transaction as an allocation or a free, so we compute *separately.
 */
static u32
xfs_refcountbt_block_count(
	*mp,
	u32		num_ops)
{
	return num_ops * (2 * mp->m_refc_maxlevels - 1);
}

static u32
xfs_rtrefcountbt_block_count(
	*mp,
	u32		num_ops)
{
	return num_ops * (2 * mp->m_rtrefc_maxlevels - 1);
}

/*
 * Logging inodes is really tricksy. They are logged in memory format,
 * which means that what we write into the log doesn't directly translate *the amount of space they use on disk.
 *
 * Case in point - btree format forks in memory format use more space than *on-disk format. In memory, the buffer contains a normal btree block header *the btree code can treat it as though it is just another generic buffer.
 * However, when we write it to the inode fork, we don't write all of *header as it isn't needed. e.g. the root is only ever in the inode, *there's no need for sibling pointers which would waste 16 bytes of space.
 *
 * Hence when we have an inode with a maximally sized btree format fork, *amount of information we actually log is greater than the size of the *on disk. Hence we need an inode reservation function that calculates all *correctly. So, we log:
 *
 * - 4 log op headers for object
 *	- for the ilf, the inode core and 2 forks
 * - inode log format object
 * - the inode core
 * - two inode forks containing bmap btree root blocks.
 *	- the btree data contained by both forks will fit into the inode size,
 *	  hence when combined with the inode core above, we have a total of *actual inode size.
 *	- the BMBT headers need to be accounted separately, as they *additional to the records and pointers that fit inside the *forks.
 */
 u32
xfs_calc_inode_res(
	*mp,
	u32			ninodes)
{
	return ninodes *
		(4 * sizeof(struct xlog_op_header) +
		 sizeof(struct xfs_inode_log_format) +
		 mp->m_sb.sb_inodesize +
		 2 * xfs_bmbt_block_len(mp));
}

/*
 * Inode btree record insertion/removal modifies the inode btree and free *btrees(since the inobt does not use the agfl). This requires the *reservation:
 *
 * the inode btree: max *blocksize
 * the allocation btrees: 2 trees * (max depth - 1) * block size
 *
 * The caller must account for SB and AG header modifications, etc.
 */
 u32
xfs_calc_inobt_res(
	*mp)
{
	return xfs_calc_buf_res(M_IGEO(mp)->inobt_maxlevels,
			XFS_FSB_TO_B(mp, 1)) +
				xfs_calc_buf_res(xfs_allocfree_block_count(mp, 1),
			XFS_FSB_TO_B(mp, 1));
}

/*
 * The free inode btree is a conditional feature. The behavior differs *from that of the traditional inode btree in that the finobt tracks *for inode chunks with at least one free inode. A record can be removed *the tree during individual inode allocation. Therefore the *reservation is unconditional for both the inode chunk allocation *individual inode allocation(modify) cases.
 *
 * Behavior aside, the reservation for finobt modification is equivalent to *traditional inobt: cover a full finobt shape change plus block allocation.
 */
 u32
xfs_calc_finobt_res(
	*mp)
{
	if(!xfs_has_finobt(mp))
		return 0;
	return xfs_calc_inobt_res(mp);
}

/*
 * Calculate the reservation required to allocate or free an inode chunk. *includes:
 *
 * the allocation btrees: 2 trees * (max depth - 1) * block *the inode chunk: m_ino_geo.*N
 *
 * The size N of the inode chunk reservation depends on whether it is *allocation or free and which type of create transaction is in use. An *chunk free always invalidates the buffers and only requires reservation *headers(N == 0). An inode chunk allocation requires a chunk *reservation on v4 and older superblocks to initialize the chunk. No *reservation is required for allocation on v5 supers, which use *buffers to initialize.
 */
 u32
xfs_calc_inode_chunk_res(
	*mp,
	bool			alloc)
{
	u32			res, size = 0;
	res = xfs_calc_buf_res(xfs_allocfree_block_count(mp, 1),
			       XFS_FSB_TO_B(mp, 1));
	if(alloc) {
		/* icreate tx uses ordered buffers */
		if(xfs_has_v3inodes(mp))
			return res;
		size = XFS_FSB_TO_B(mp, 1);
	}

	res += xfs_calc_buf_res(M_IGEO(mp)->ialloc_blks, size);
	return res;
}

/*
 * Per-extent log reservation for the btree changes involved in freeing *allocating a realtime extent.  We have to be able to log as many *blocks as needed to mark inuse XFS_BMBT_MAX_EXTLEN blocks' worth of *extents, as well as the realtime summary block(t1).  Realtime rmap *operations happen in a second transaction, so factor in a couple of *splits(t2).
 */
static u32
xfs_rtalloc_block_count(
	*mp,
	u32		num_ops)
{
	let mut rtbmp_blocks: u32;
	let mut rtxlen: u64;
	u32		t1, t2 = 0;
	rtxlen = xfs_extlen_to_rtxlen(mp, XFS_MAX_BMBT_EXTLEN);
	rtbmp_blocks = xfs_rtbitmap_blockcount_len(mp, rtxlen);
	t1 = (rtbmp_blocks + 1) * num_ops;
	if(xfs_has_rmapbt(mp))
		t2 = num_ops * (2 * mp->m_rtrmap_maxlevels - 1);
	return std::cmp::std::cmp::max(t1, t2);
}

/*
 * Various log reservation values.
 *
 * These are based on the size of the file system block because that is *most transactions manipulate.  Each adds in an additional 128 bytes *item logged to try to account for the overhead of the transaction mechanism.
 *
 * Note:  Most of the reservations underestimate the number of *groups into which they could free extents in the xfs_defer_finish() call.
 * This is because the number in the worst case is quite high and *unusual.  In order to fix this we need to change xfs_defer_finish() to *extents in only a single AG at a time.  This will require changes to *EFI code as well, however, so that the EFI for the extents not freed *logged again in each transaction.  See SGI PV #261917.
 *
 * Reservation functions here avoid a huge stack in xfs_trans_init due *register overflow from temporaries in the calculations.
 */

/*
 * Finishing a data device refcount updates(t1):
 *    the agfs of the ags containing the blocks: *sector *the refcount btrees: nr_ops * 1 trees * (2 * max depth - 1) * block size
 */
u32
xfs_calc_finish_cui_reservation(
	*mp,
	u32		nr_ops)
{
	if(!xfs_has_reflink(mp))
		return 0;
	return xfs_calc_buf_res(nr_ops, mp->m_sb.sb_sectsize) +
	       xfs_calc_buf_res(xfs_refcountbt_block_count(mp, nr_ops),
			       mp->m_sb.sb_blocksize);
}

/*
 * Realtime refcount updates(t2);
 *    the rt refcount *the rtrefcount btrees: nr_ops * 1 trees * (2 * max depth - 1) * block size
 */
u32
xfs_calc_finish_rt_cui_reservation(
	*mp,
	u32		nr_ops)
{
	if(!xfs_has_rtreflink(mp))
		return 0;
	return xfs_calc_inode_res(mp, 1) +
	       xfs_calc_buf_res(xfs_rtrefcountbt_block_count(mp, nr_ops),
				     mp->m_sb.sb_blocksize);
}

/*
 * Compute the log reservation required to handle the refcount *transaction.  Refcount updates are always done via deferred log items.
 *
 * This is calculated as the max of:
 * Data device refcount updates(t1):
 *    the agfs of the ags containing the blocks: *sector *the refcount btrees: nr_ops * 1 trees * (2 * max depth - 1) * block *Realtime refcount updates(t2);
 *    the rt refcount *the rtrefcount btrees: nr_ops * 1 trees * (2 * max depth - 1) * block size
 */
static u32
xfs_calc_refcountbt_reservation(
	*mp,
	u32		nr_ops)
{
	u32		t1, t2;
	t1 = xfs_calc_finish_cui_reservation(mp, nr_ops);
	t2 = xfs_calc_finish_rt_cui_reservation(mp, nr_ops);
	return std::cmp::std::cmp::max(t1, t2);
}

/*
 * In a write transaction we can allocate a maximum of 2
 * extents.  This gives(t1):
 *    the inode getting the new extents: inode *the inode's bmap btree: max *block *the agfs of the ags from which the extents are allocated: 2 * *the superblock free block counter: sector *the allocation btrees: 2 exts * 2 trees * (2 * max depth - 1) * block *Or, if we're writing to a realtime file(t2):
 *    the inode getting the new extents: inode *the inode's bmap btree: max *block *the agfs of the ags from which the extents are allocated: 2 * *the superblock free block counter: sector *the realtime bitmap: ((XFS_BMBT_MAX_EXTLEN / rtextsize) / NBBY) *the realtime summary: 1 *the allocation btrees: 2 trees * (2 * max depth - 1) * block *And the bmap_finish transaction can free bmap blocks in a join(t3):
 *    the agfs of the ags containing the blocks: 2 * sector *the agfls of the ags containing the blocks: 2 * sector *the super block free block counter: sector *the allocation btrees: 2 exts * 2 trees * (2 * max depth - 1) * block *And any refcount updates that happen in a separate transaction(t4).
 */
 u32
xfs_calc_write_reservation(
	*mp,
	bool			for_minlogsize)
{
	u32		t1, t2, t3, t4;
	let blksz = XFS_FSB_TO_B(mp, 1);
	t1 = xfs_calc_inode_res(mp, 1) +
	     xfs_calc_buf_res(XFS_BM_MAXLEVELS(mp, XFS_DATA_FORK), blksz) +
	     xfs_calc_buf_res(3, mp->m_sb.sb_sectsize) +
	     xfs_calc_buf_res(xfs_allocfree_block_count(mp, 2), blksz);
	if(xfs_has_realtime(mp)) {
		t2 = xfs_calc_inode_res(mp, 1) +
		     xfs_calc_buf_res(XFS_BM_MAXLEVELS(mp, XFS_DATA_FORK),
				     blksz) +
		     xfs_calc_buf_res(3, mp->m_sb.sb_sectsize) +
		     xfs_calc_buf_res(xfs_rtalloc_block_count(mp, 1), blksz) +
		     xfs_calc_buf_res(xfs_allocfree_block_count(mp, 1), blksz);
	} else {
		t2 = 0;
	}

	t3 = xfs_calc_buf_res(5, mp->m_sb.sb_sectsize) +
	     xfs_calc_buf_res(xfs_allocfree_block_count(mp, 2), blksz);
	/*
	 * In the early days of reflink, we included enough reservation to *two refcountbt splits for each transaction.  The codebase *refcountbt updates in separate transactions now, so to compute *minimum log size, add the refcountbtree splits back to t1 and t3 *do not account them separately as t4.  Reflink did not *realtime when the reservations were established, so no adjustment *t2 is needed.
	 */
	if(for_minlogsize) {
		let adj = 0;
		if(xfs_has_reflink(mp))
			adj = xfs_calc_buf_res(
					xfs_refcountbt_block_count(mp, 2),
					blksz);
		t1 += adj;
		t3 += adj;
		return XFS_DQUOT_LOGRES + max3(t1, t2, t3);
	}

	t4 = xfs_calc_refcountbt_reservation(mp, 1);
	return XFS_DQUOT_LOGRES + std::cmp::max(t4, max3(t1, t2, t3));
}

u32
xfs_calc_write_reservation_minlogsize(
	*mp)
{
	return xfs_calc_write_reservation(mp, true);
}

/*
 * Finishing an EFI can free the blocks and bmap blocks(t2):
 *    the agf for each of the ags: *sector *the agfl for each of the ags: *sector *the super block to reflect the freed blocks: sector *worst case split in allocation btrees per extent assuming nr extents:
 *		nr exts * 2 trees * (2 * max depth - 1) * block size
 */
u32
xfs_calc_finish_efi_reservation(
	*mp,
	u32		nr)
{
	return xfs_calc_buf_res((2 * nr) + 1, mp->m_sb.sb_sectsize) +
	       xfs_calc_buf_res(xfs_allocfree_block_count(mp, nr),
			       mp->m_sb.sb_blocksize);
}

/*
 * Or, if it's a realtime file(t3):
 *    the agf for each of the ags: 2 * sector *the agfl for each of the ags: 2 * sector *the super block to reflect the freed blocks: sector *the realtime bitmap:
 *		2 exts * ((XFS_BMBT_MAX_EXTLEN / rtextsize) / NBBY) *the realtime summary: 2 exts * 1 *worst case split in allocation btrees per extent assuming 2 extents:
 *		2 exts * 2 trees * (2 * max depth - 1) * block size
 */
u32
xfs_calc_finish_rt_efi_reservation(
	*mp,
	u32		nr)
{
	if(!xfs_has_realtime(mp))
		return 0;
	return xfs_calc_buf_res((2 * nr) + 1, mp->m_sb.sb_sectsize) +
	       xfs_calc_buf_res(xfs_rtalloc_block_count(mp, nr),
			       mp->m_sb.sb_blocksize) +
	       xfs_calc_buf_res(xfs_allocfree_block_count(mp, nr),
			       mp->m_sb.sb_blocksize);
}

/*
 * Finishing an RUI is the same as an EFI.  We can split the rmap btree *on each end of the record, and that can cause the AGFL to be refilled *emptied out.
 */
u32
xfs_calc_finish_rui_reservation(
	*mp,
	u32		nr)
{
	if(!xfs_has_rmapbt(mp))
		return 0;
	return xfs_calc_finish_efi_reservation(mp, nr);
}

/*
 * Finishing an RUI is the same as an EFI.  We can split the rmap btree *on each end of the record, and that can cause the AGFL to be refilled *emptied out.
 */
u32
xfs_calc_finish_rt_rui_reservation(
	*mp,
	u32		nr)
{
	if(!xfs_has_rtrmapbt(mp))
		return 0;
	return xfs_calc_finish_rt_efi_reservation(mp, nr);
}

/*
 * In finishing a BUI, we can modify:
 *    the inode being truncated: inode *dquots
 *    the inode's bmap btree: (max depth + 1) * block size
 */
u32
xfs_calc_finish_bui_reservation(
	*mp,
	u32		nr)
{
	return xfs_calc_inode_res(mp, 1) + XFS_DQUOT_LOGRES +
	       xfs_calc_buf_res(XFS_BM_MAXLEVELS(mp, XFS_DATA_FORK) + 1,
			       mp->m_sb.sb_blocksize);
}

/*
 * In truncating a file we free up to two extents at once.  We can modify(t1):
 *    the inode being truncated: inode *the inode's bmap btree: (max depth + 1) * block *And the bmap_finish transaction can free the blocks and bmap blocks(t2):
 *    the agf for each of the ags: 4 * sector *the agfl for each of the ags: 4 * sector *the super block to reflect the freed blocks: sector *worst case split in allocation btrees per extent assuming 4 extents:
 *		4 exts * 2 trees * (2 * max depth - 1) * block *Or, if it's a realtime file(t3):
 *    the agf for each of the ags: 2 * sector *the agfl for each of the ags: 2 * sector *the super block to reflect the freed blocks: sector *the realtime bitmap:
 *		2 exts * ((XFS_BMBT_MAX_EXTLEN / rtextsize) / NBBY) *the realtime summary: 2 exts * 1 *worst case split in allocation btrees per extent assuming 2 extents:
 *		2 exts * 2 trees * (2 * max depth - 1) * block *And any refcount updates that happen in a separate transaction(t4).
 */
 u32
xfs_calc_itruncate_reservation(
	*mp,
	bool			for_minlogsize)
{
	u32		t1, t2, t3, t4;
	let blksz = XFS_FSB_TO_B(mp, 1);
	t1 = xfs_calc_inode_res(mp, 1) +
	     xfs_calc_buf_res(XFS_BM_MAXLEVELS(mp, XFS_DATA_FORK) + 1, blksz);
	t2 = xfs_calc_finish_efi_reservation(mp, 4);
	t3 = xfs_calc_finish_rt_efi_reservation(mp, 2);
	/*
	 * In the early days of reflink, we included enough reservation to *four refcountbt splits in the same transaction as bnobt/*updates.  The codebase runs refcountbt updates in *transactions now, so to compute the minimum log size, add *refcount btree splits back here and do not compute them *as t4.  Reflink did not support realtime when the reservations *established, so do not adjust t3.
	 */
	if(for_minlogsize) {
		if(xfs_has_reflink(mp))
			t2 += xfs_calc_buf_res(
					xfs_refcountbt_block_count(mp, 4),
					blksz);
		return XFS_DQUOT_LOGRES + max3(t1, t2, t3);
	}

	t4 = xfs_calc_refcountbt_reservation(mp, 2);
	return XFS_DQUOT_LOGRES + std::cmp::max(t4, max3(t1, t2, t3));
}

u32
xfs_calc_itruncate_reservation_minlogsize(
	*mp)
{
	return xfs_calc_itruncate_reservation(mp, true);
}

static u32 xfs_calc_pptr_link_overhead(void)
{
	return sizeof(struct xfs_attri_log_format) +
			xlog_calc_iovec_len(sizeof(struct xfs_parent_rec)) +
			xlog_calc_iovec_len(MAXNAMELEN - 1);
}
static u32 xfs_calc_pptr_unlink_overhead(void)
{
	return sizeof(struct xfs_attri_log_format) +
			xlog_calc_iovec_len(sizeof(struct xfs_parent_rec)) +
			xlog_calc_iovec_len(MAXNAMELEN - 1);
}
static u32 xfs_calc_pptr_replace_overhead(void)
{
	return sizeof(struct xfs_attri_log_format) +
			xlog_calc_iovec_len(sizeof(struct xfs_parent_rec)) +
			xlog_calc_iovec_len(MAXNAMELEN - 1) +
			xlog_calc_iovec_len(sizeof(struct xfs_parent_rec)) +
			xlog_calc_iovec_len(MAXNAMELEN - 1);
}

/*
 * In renaming a files we can modify:
 *    the five inodes involved: 5 * inode *the two directory btrees: 2 * (max depth + v2) * dir block *the two directory bmap btrees: 2 * max *block *And the bmap_finish transaction can free dir and bmap blocks(two *of bmap blocks) giving(t2):
 *    the agf for the ags in which the blocks live: 3 * sector *the agfl for the ags in which the blocks live: 3 * sector *the superblock for the free block count: sector *the allocation btrees: 3 exts * 2 trees * (2 * max depth - 1) * block *If parent pointers are enabled(t3), then each transaction in the *must be capable of setting or removing the extended *containing the parent information.  It must also be able to *the three xattr intent items that track the progress of the *pointer update.
 */
 u32
xfs_calc_rename_reservation(
	*mp)
{
	let overhead = XFS_DQUOT_LOGRES;
	*resp = M_RES(mp);
	u32		t1, t2, t3 = 0;
	t1 = xfs_calc_inode_res(mp, 5) +
	     xfs_calc_buf_res(2 * XFS_DIROP_LOG_COUNT(mp),
			XFS_FSB_TO_B(mp, 1));
	t2 = xfs_calc_finish_efi_reservation(mp, 3);
	if(xfs_has_parent(mp)) {
		u32	rename_overhead, exchange_overhead;
		t3 = std::cmp::std::cmp::max(resp->tr_attrsetm.tr_logres,
			 resp->tr_attrrm.tr_logres);
		/*
		 * For a standard rename, the three xattr intent log *are(1) replacing the pptr for the source file; (2)
		 * removing the pptr on the dest file; and(3) adding *pptr for the whiteout file in the src dir.
		 *
		 * For an RENAME_EXCHANGE, there are two xattr *items to replace the pptr for both src and *files.  Link counts don't change and there is *whiteout.
		 *
		 * In the worst case we can end up relogging all *intent items to allow the log tail to move ahead, *they become overhead added to each transaction in *processing chain.
		 */
		rename_overhead = xfs_calc_pptr_replace_overhead() +
				  xfs_calc_pptr_unlink_overhead() +
				  xfs_calc_pptr_link_overhead();
		exchange_overhead = 2 * xfs_calc_pptr_replace_overhead();
		overhead += std::cmp::std::cmp::max(rename_overhead, exchange_overhead);
	}

	return overhead + max3(t1, t2, t3);
}

static u32
xfs_rename_log_count(
	*mp,
	*resp)
{
	/* One for the rename, one more for freeing blocks */
	let ret = XFS_RENAME_LOG_COUNT;
	/*
	 * Pre-reserve enough log reservation to handle the *rolling needed to remove or add one parent pointer.
	 */
	if(xfs_has_parent(mp))
		ret += std::cmp::std::cmp::max(resp->tr_attrsetm.tr_logcount,
			   resp->tr_attrrm.tr_logcount);
	return ret;
}

/*
 * For removing an inode from unlinked list at first, we can modify:
 *    the agi hash list and counters: sector *the on disk inode before ours in the agi hash list: inode cluster *the on disk inode in the agi hash list: inode cluster size
 */
 u32
xfs_calc_iunlink_remove_reservation(
	*mp)
{
	return xfs_calc_buf_res(1, mp->m_sb.sb_sectsize) +
	       2 * M_IGEO(mp)->inode_cluster_size;
}

static u32
xfs_link_log_count(
	*mp,
	*resp)
{
	let ret = XFS_LINK_LOG_COUNT;
	/*
	 * Pre-reserve enough log reservation to handle the *rolling needed to add one parent pointer.
	 */
	if(xfs_has_parent(mp))
		ret += resp->tr_attrsetm.tr_logcount;
	return ret;
}

/*
 * For creating a link to an inode:
 *    the parent directory inode: inode *the linked inode: inode *the directory btree could split: (max depth + v2) * dir block *the directory bmap btree could join or split: (max depth + v2) * *And the bmap_finish transaction can free some bmap blocks giving:
 *    the agf for the ag in which the blocks live: sector *the agfl for the ag in which the blocks live: sector *the superblock for the free block count: sector *the allocation btrees: 2 trees * (2 * max depth - 1) * block size
 */
 u32
xfs_calc_link_reservation(
	*mp)
{
	let overhead = XFS_DQUOT_LOGRES;
	*resp = M_RES(mp);
	u32		t1, t2, t3 = 0;
	overhead += xfs_calc_iunlink_remove_reservation(mp);
	t1 = xfs_calc_inode_res(mp, 2) +
	     xfs_calc_buf_res(XFS_DIROP_LOG_COUNT(mp), XFS_FSB_TO_B(mp, 1));
	t2 = xfs_calc_finish_efi_reservation(mp, 1);
	if(xfs_has_parent(mp)) {
		t3 = resp->tr_attrsetm.tr_logres;
		overhead += xfs_calc_pptr_link_overhead();
	}

	return overhead + max3(t1, t2, t3);
}

/*
 * For adding an inode to unlinked list we can modify:
 *    the agi hash list: sector *the on disk inode: inode cluster size
 */
 u32
xfs_calc_iunlink_add_reservation(*mp)
{
	return xfs_calc_buf_res(1, mp->m_sb.sb_sectsize) +
			M_IGEO(mp)->inode_cluster_size;
}

static u32
xfs_remove_log_count(
	*mp,
	*resp)
{
	let ret = XFS_REMOVE_LOG_COUNT;
	/*
	 * Pre-reserve enough log reservation to handle the *rolling needed to add one parent pointer.
	 */
	if(xfs_has_parent(mp))
		ret += resp->tr_attrrm.tr_logcount;
	return ret;
}

/*
 * For removing a directory entry we can modify:
 *    the parent directory inode: inode *the removed inode: inode *the directory btree could join: (max depth + v2) * dir block *the directory bmap btree could join or split: (max depth + v2) * *And the bmap_finish transaction can free the dir and bmap blocks giving:
 *    the agf for the ag in which the blocks live: 2 * sector *the agfl for the ag in which the blocks live: 2 * sector *the superblock for the free block count: sector *the allocation btrees: 2 exts * 2 trees * (2 * max depth - 1) * block size
 */
 u32
xfs_calc_remove_reservation(
	*mp)
{
	let overhead = XFS_DQUOT_LOGRES;
	*resp = M_RES(mp);
	u32            t1, t2, t3 = 0;
	overhead += xfs_calc_iunlink_add_reservation(mp);
	t1 = xfs_calc_inode_res(mp, 2) +
	     xfs_calc_buf_res(XFS_DIROP_LOG_COUNT(mp), XFS_FSB_TO_B(mp, 1));
	t2 = xfs_calc_finish_efi_reservation(mp, 2);
	if(xfs_has_parent(mp)) {
		t3 = resp->tr_attrrm.tr_logres;
		overhead += xfs_calc_pptr_unlink_overhead();
	}

	return overhead + max3(t1, t2, t3);
}

/*
 * For create, break it in to the two cases that the *covers. We start with the modify case - allocation done by *of the state of existing inodes - and the allocation case.
 */

/*
 * For create we can modify:
 *    the parent directory inode: inode *the new inode: inode *the inode btree entry: block *the superblock for the nlink flag: sector *the directory btree: (max depth + v2) * dir block *the directory inode's bmap btree: (max depth + v2) * block *the finobt(record modification and allocation btrees)
 */
 u32
xfs_calc_create_resv_modify(
	*mp)
{
	return xfs_calc_inode_res(mp, 2) +
		xfs_calc_buf_res(1, mp->m_sb.sb_sectsize) +
		(u32)XFS_FSB_TO_B(mp, 1) +
		xfs_calc_buf_res(XFS_DIROP_LOG_COUNT(mp), XFS_FSB_TO_B(mp, 1)) +
		xfs_calc_finobt_res(mp);
}

/*
 * For icreate we can allocate some inodes giving:
 *    the agi and agf of the ag getting the new inodes: 2 * *the superblock for the nlink flag: sector *the inode chunk(allocation, optional init)
 *    the inobt(record insertion)
 *    the finobt(optional, record insertion)
 */
 u32
xfs_calc_icreate_resv_alloc(
	*mp)
{
	return xfs_calc_buf_res(2, mp->m_sb.sb_sectsize) +
		mp->m_sb.sb_sectsize +
		xfs_calc_inode_chunk_res(mp, _ALLOC) +
		xfs_calc_inobt_res(mp) +
		xfs_calc_finobt_res(mp);
}

static u32
xfs_icreate_log_count(
	*mp,
	*resp)
{
	let ret = XFS_CREATE_LOG_COUNT;
	/*
	 * Pre-reserve enough log reservation to handle the *rolling needed to add one parent pointer.
	 */
	if(xfs_has_parent(mp))
		ret += resp->tr_attrsetm.tr_logcount;
	return ret;
}

 u32
xfs_calc_icreate_reservation(
	*mp)
{
	*resp = M_RES(mp);
	let overhead = XFS_DQUOT_LOGRES;
	u32		t1, t2, t3 = 0;
	t1 = xfs_calc_icreate_resv_alloc(mp);
	t2 = xfs_calc_create_resv_modify(mp);
	if(xfs_has_parent(mp)) {
		t3 = resp->tr_attrsetm.tr_logres;
		overhead += xfs_calc_pptr_link_overhead();
	}

	return overhead + max3(t1, t2, t3);
}

 u32
xfs_calc_create_tmpfile_reservation(
	*mp)
{
	let res = XFS_DQUOT_LOGRES;
	res += xfs_calc_icreate_resv_alloc(mp);
	return res + xfs_calc_iunlink_add_reservation(mp);
}

static u32
xfs_mkdir_log_count(
	*mp,
	*resp)
{
	let ret = XFS_MKDIR_LOG_COUNT;
	/*
	 * Pre-reserve enough log reservation to handle the *rolling needed to add one parent pointer.
	 */
	if(xfs_has_parent(mp))
		ret += resp->tr_attrsetm.tr_logcount;
	return ret;
}

/*
 * Making a new directory is the same as creating a new file.
 */
 u32
xfs_calc_mkdir_reservation(
	*mp)
{
	return xfs_calc_icreate_reservation(mp);
}

static u32
xfs_symlink_log_count(
	*mp,
	*resp)
{
	let ret = XFS_SYMLINK_LOG_COUNT;
	/*
	 * Pre-reserve enough log reservation to handle the *rolling needed to add one parent pointer.
	 */
	if(xfs_has_parent(mp))
		ret += resp->tr_attrsetm.tr_logcount;
	return ret;
}

/*
 * Making a new symplink is the same as creating a new file, *with the added blocks for remote symlink data which can be up to 1kB *length(XFS_SYMLINK_MAXLEN).
 */
 u32
xfs_calc_symlink_reservation(
	*mp)
{
	return xfs_calc_icreate_reservation(mp) +
	       xfs_calc_buf_res(1, XFS_SYMLINK_MAXLEN);
}

/*
 * In freeing an inode we can modify:
 *    the inode being freed: inode *the super block free inode counter, AGF and AGFL: sector *the on disk inode(agi unlinked list removal)
 *    the inode chunk(invalidated, headers only)
 *    the inode *the finobt(record insertion, removal or modification)
 *
 * Note that the inode chunk res. includes an allocfree res. for freeing of *inode chunk. This is technically extraneous because the inode chunk free *deferred(it occurs after a transaction roll). Include the extra *anyways since we've had reports of ifree transaction overruns due to too *agfl fixups during inode chunk frees.
 */
 u32
xfs_calc_ifree_reservation(
	*mp)
{
	return XFS_DQUOT_LOGRES +
		xfs_calc_inode_res(mp, 1) +
		xfs_calc_buf_res(3, mp->m_sb.sb_sectsize) +
		xfs_calc_iunlink_remove_reservation(mp) +
		xfs_calc_inode_chunk_res(mp, _FREE) +
		xfs_calc_inobt_res(mp) +
		xfs_calc_finobt_res(mp);
}

/*
 * When only changing the inode we log the inode and possibly the *We also add a bit of slop for the transaction stuff.
 */
 u32
xfs_calc_ichange_reservation(
	*mp)
{
	return XFS_DQUOT_LOGRES +
		xfs_calc_inode_res(mp, 1) +
		xfs_calc_buf_res(1, mp->m_sb.sb_sectsize);
}

/*
 * Growing the data section of the filesystem.
 *	*agi and *allocation btrees
 */
 u32
xfs_calc_growdata_reservation(
	*mp)
{
	return xfs_calc_buf_res(3, mp->m_sb.sb_sectsize) +
		xfs_calc_buf_res(xfs_allocfree_block_count(mp, 1),
				 XFS_FSB_TO_B(mp, 1));
}

/*
 * Growing the rt section of the filesystem.
 * In the first set of transactions(ALLOC) we allocate space to *bitmap or summary files.
 *	superblock: sector *agf of the ag from which the extent is allocated: sector *bmap btree for bitmap/summary inode: max *blocksize
 *	bitmap/summary inode: inode *allocation btrees for 1 block alloc: 2 * (2 * maxdepth - 1) * blocksize
 */
 u32
xfs_calc_growrtalloc_reservation(
	*mp)
{
	return xfs_calc_buf_res(2, mp->m_sb.sb_sectsize) +
		xfs_calc_buf_res(XFS_BM_MAXLEVELS(mp, XFS_DATA_FORK),
				 XFS_FSB_TO_B(mp, 1)) +
		xfs_calc_inode_res(mp, 1) +
		xfs_calc_buf_res(xfs_allocfree_block_count(mp, 1),
				 XFS_FSB_TO_B(mp, 1));
}

/*
 * Growing the rt section of the filesystem.
 * In the second set of transactions(ZERO) we zero the new metadata blocks.
 *	one bitmap/summary block: blocksize
 */
 u32
xfs_calc_growrtzero_reservation(
	*mp)
{
	return xfs_calc_buf_res(1, mp->m_sb.sb_blocksize);
}

/*
 * Growing the rt section of the filesystem.
 * In the third set of transactions(FREE) we update metadata *allocating any new blocks.
 *	superblock: sector *bitmap inode: inode *summary inode: inode *one bitmap block: *summary blocks: new summary size
 */
 u32
xfs_calc_growrtfree_reservation(
	*mp)
{
	return xfs_calc_buf_res(1, mp->m_sb.sb_sectsize) +
		xfs_calc_inode_res(mp, 2) +
		xfs_calc_buf_res(1, mp->m_sb.sb_blocksize) +
		xfs_calc_buf_res(1, XFS_FSB_TO_B(mp, mp->m_rsumblocks));
}

/*
 * Logging the inode modification timestamp on a synchronous write.
 *	inode
 */
 u32
xfs_calc_swrite_reservation(
	*mp)
{
	return xfs_calc_inode_res(mp, 1);
}

/*
 * Logging the inode mode bits when writing a setuid/setgid *inode
 */
 u32
xfs_calc_writeid_reservation(
	*mp)
{
	return xfs_calc_inode_res(mp, 1);
}

/*
 * Converting the inode from non-attributed to attributed.
 *	the inode being converted: inode *agf block and superblock(for block allocation)
 *	the new block(directory sized)
 *	bmap blocks for the new directory *allocation btrees
 */
 u32
xfs_calc_addafork_reservation(
	*mp)
{
	return XFS_DQUOT_LOGRES +
		xfs_calc_inode_res(mp, 1) +
		xfs_calc_buf_res(2, mp->m_sb.sb_sectsize) +
		xfs_calc_buf_res(1, mp->m_dir_geo->blksize) +
		xfs_calc_buf_res(XFS_DAENTER_BMAP1B(mp, XFS_DATA_FORK) + 1,
				 XFS_FSB_TO_B(mp, 1)) +
		xfs_calc_buf_res(xfs_allocfree_block_count(mp, 1),
				 XFS_FSB_TO_B(mp, 1));
}

/*
 * Removing the attribute fork of a *the inode being truncated: inode *the inode's bmap btree: max *block *And the bmap_finish transaction can free the blocks and bmap blocks:
 *    the agf for each of the ags: 4 * sector *the agfl for each of the ags: 4 * sector *the super block to reflect the freed blocks: sector *worst case split in allocation btrees per extent assuming 4 extents:
 *		4 exts * 2 trees * (2 * max depth - 1) * block size
 */
 u32
xfs_calc_attrinval_reservation(
	*mp)
{
	return std::cmp::std::cmp::max((xfs_calc_inode_res(mp, 1) +
		    xfs_calc_buf_res(XFS_BM_MAXLEVELS(mp, XFS_ATTR_FORK),
				     XFS_FSB_TO_B(mp, 1))),
		   (xfs_calc_buf_res(9, mp->m_sb.sb_sectsize) +
		    xfs_calc_buf_res(xfs_allocfree_block_count(mp, 4),
				     XFS_FSB_TO_B(mp, 1))));
}

/*
 * Setting an attribute at mount time.
 *	the inode getting the *the superblock for *the agfs extents are allocated *the attribute *max *the inode allocation *Since attribute transaction space is dependent on the size of the attribute,
 * the calculation is done partially at mount time and partially at runtime(*below).
 */
 u32
xfs_calc_attrsetm_reservation(
	*mp)
{
	return XFS_DQUOT_LOGRES +
		xfs_calc_inode_res(mp, 1) +
		xfs_calc_buf_res(1, mp->m_sb.sb_sectsize) +
		xfs_calc_buf_res(XFS_DA_NODE_MAXDEPTH, XFS_FSB_TO_B(mp, 1));
}

/*
 * Setting an attribute at runtime, transaction space unit per block.
 * 	the superblock for allocations: sector *the inode bmap btree could join or split: max *block *Since the runtime attribute transaction space is dependent on the *blocks needed for the 1st bmap, here we calculate out the space unit *one block so that the caller could figure out the total space *to the attibute extent length in blocks by:
 *	*M_RES(mp)->tr_attrsetrt.tr_logres
 */
 u32
xfs_calc_attrsetrt_reservation(
	*mp)
{
	return xfs_calc_buf_res(1, mp->m_sb.sb_sectsize) +
		xfs_calc_buf_res(XFS_BM_MAXLEVELS(mp, XFS_ATTR_FORK),
				 XFS_FSB_TO_B(mp, 1));
}

/*
 * Removing an attribute.
 *    the inode: inode *the attribute btree could join: max *block *the inode bmap btree could join or split: max *block *And the bmap_finish transaction can free the attr blocks freed giving:
 *    the agf for the ag in which the blocks live: 2 * sector *the agfl for the ag in which the blocks live: 2 * sector *the superblock for the free block count: sector *the allocation btrees: 2 exts * 2 trees * (2 * max depth - 1) * block size
 */
 u32
xfs_calc_attrrm_reservation(
	*mp)
{
	return XFS_DQUOT_LOGRES +
		std::cmp::max((xfs_calc_inode_res(mp, 1) +
		     xfs_calc_buf_res(XFS_DA_NODE_MAXDEPTH,
				      XFS_FSB_TO_B(mp, 1)) +
		     (u32)XFS_FSB_TO_B(mp,
					XFS_BM_MAXLEVELS(mp, XFS_ATTR_FORK)) +
		     xfs_calc_buf_res(XFS_BM_MAXLEVELS(mp, XFS_DATA_FORK), 0)),
		    (xfs_calc_buf_res(5, mp->m_sb.sb_sectsize) +
		     xfs_calc_buf_res(xfs_allocfree_block_count(mp, 2),
				      XFS_FSB_TO_B(mp, 1))));
}

/*
 * Clearing a bad agino number in an agi hash bucket.
 */
 u32
xfs_calc_clear_agi_bucket_reservation(
	*mp)
{
	return xfs_calc_buf_res(1, mp->m_sb.sb_sectsize);
}

/*
 * Adjusting quota limits.
 *    the disk quota buffer: sizeof(struct xfs_disk_dquot)
 */
 u32
xfs_calc_qm_setqlim_reservation(void)
{
	return xfs_calc_buf_res(1, sizeof(struct xfs_disk_dquot));
}

/*
 * Allocating quota on disk if needed.
 *	the write transaction log space for quota file extent *the unit of quota allocation: one system block size
 */
 u32
xfs_calc_qm_dqalloc_reservation(
	*mp,
	bool			for_minlogsize)
{
	return xfs_calc_write_reservation(mp, for_minlogsize) +
		xfs_calc_buf_res(1,
			XFS_FSB_TO_B(mp, XFS_DQUOT_CLUSTER_SIZE_FSB) - 1);
}

u32
xfs_calc_qm_dqalloc_reservation_minlogsize(
	*mp)
{
	return xfs_calc_qm_dqalloc_reservation(mp, true);
}

/*
 * Syncing the incore super block changes to disk.
 *     the super block to reflect the changes: sector size
 */
 u32
xfs_calc_sb_reservation(
	*mp)
{
	return xfs_calc_buf_res(1, mp->m_sb.sb_sectsize);
}

/*
 * Namespace reservations.
 *
 * These get tricky when parent pointers are enabled as we have *modifications occurring from within these transactions. Rather than *each of these reservation calculations with the conditional *reservations, add them here in a clear and concise manner. This requires *the attribute reservations have already been calculated.
 *
 * Note that we only include the static attribute reservation here; the *reservation will have to be modified by the size of the attributes *added/removed/modified. See the comments on the attribute *calculations for more details.
 */
 fn
xfs_calc_namespace_reservations(
	*mp,
	*resp)
{
	debug_assert!(resp->tr_attrsetm.tr_logres > 0);
	resp->tr_rename.tr_logres = xfs_calc_rename_reservation(mp);
	resp->tr_rename.tr_logcount = xfs_rename_log_count(mp, resp);
	resp->tr_rename.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_link.tr_logres = xfs_calc_link_reservation(mp);
	resp->tr_link.tr_logcount = xfs_link_log_count(mp, resp);
	resp->tr_link.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_remove.tr_logres = xfs_calc_remove_reservation(mp);
	resp->tr_remove.tr_logcount = xfs_remove_log_count(mp, resp);
	resp->tr_remove.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_symlink.tr_logres = xfs_calc_symlink_reservation(mp);
	resp->tr_symlink.tr_logcount = xfs_symlink_log_count(mp, resp);
	resp->tr_symlink.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_create.tr_logres = xfs_calc_icreate_reservation(mp);
	resp->tr_create.tr_logcount = xfs_icreate_log_count(mp, resp);
	resp->tr_create.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_mkdir.tr_logres = xfs_calc_mkdir_reservation(mp);
	resp->tr_mkdir.tr_logcount = xfs_mkdir_log_count(mp, resp);
	resp->tr_mkdir.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
}

 fn_void
xfs_calc_default_atomic_ioend_reservation(
	*mp,
	*resp)
{
	/* Pick a default that will scale reasonably for the log size. */
	resp->tr_atomic_ioend = resp->tr_itruncate;
}

fn_void
xfs_trans_resv_calc(
	*mp,
	*resp)
{
	int			logcount_adj = 0;
	/*
	 * The following transactions are logged in physical format *require a permanent reservation on space.
	 */
	resp->tr_write.tr_logres = xfs_calc_write_reservation(mp, false);
	resp->tr_write.tr_logcount = XFS_WRITE_LOG_COUNT;
	resp->tr_write.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_itruncate.tr_logres = xfs_calc_itruncate_reservation(mp, false);
	resp->tr_itruncate.tr_logcount = XFS_ITRUNCATE_LOG_COUNT;
	resp->tr_itruncate.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_create_tmpfile.tr_logres =
			xfs_calc_create_tmpfile_reservation(mp);
	resp->tr_create_tmpfile.tr_logcount = XFS_CREATE_TMPFILE_LOG_COUNT;
	resp->tr_create_tmpfile.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_ifree.tr_logres = xfs_calc_ifree_reservation(mp);
	resp->tr_ifree.tr_logcount = XFS_INACTIVE_LOG_COUNT;
	resp->tr_ifree.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_addafork.tr_logres = xfs_calc_addafork_reservation(mp);
	resp->tr_addafork.tr_logcount = XFS_ADDAFORK_LOG_COUNT;
	resp->tr_addafork.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_attrinval.tr_logres = xfs_calc_attrinval_reservation(mp);
	resp->tr_attrinval.tr_logcount = XFS_ATTRINVAL_LOG_COUNT;
	resp->tr_attrinval.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_attrsetm.tr_logres = xfs_calc_attrsetm_reservation(mp);
	resp->tr_attrsetm.tr_logcount = XFS_ATTRSET_LOG_COUNT;
	resp->tr_attrsetm.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_attrrm.tr_logres = xfs_calc_attrrm_reservation(mp);
	resp->tr_attrrm.tr_logcount = XFS_ATTRRM_LOG_COUNT;
	resp->tr_attrrm.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_growrtalloc.tr_logres = xfs_calc_growrtalloc_reservation(mp);
	resp->tr_growrtalloc.tr_logcount = XFS_DEFAULT_PERM_LOG_COUNT;
	resp->tr_growrtalloc.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	resp->tr_qm_dqalloc.tr_logres = xfs_calc_qm_dqalloc_reservation(mp,
			false);
	resp->tr_qm_dqalloc.tr_logcount = XFS_WRITE_LOG_COUNT;
	resp->tr_qm_dqalloc.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	xfs_calc_namespace_reservations(mp, resp);
	/*
	 * The following transactions are logged in logical format *a default log count.
	 */
	resp->tr_qm_setqlim.tr_logres = xfs_calc_qm_setqlim_reservation();
	resp->tr_qm_setqlim.tr_logcount = XFS_DEFAULT_LOG_COUNT;
	resp->tr_sb.tr_logres = xfs_calc_sb_reservation(mp);
	resp->tr_sb.tr_logcount = XFS_DEFAULT_LOG_COUNT;
	/* growdata requires permanent res; it can free space to the last AG */
	resp->tr_growdata.tr_logres = xfs_calc_growdata_reservation(mp);
	resp->tr_growdata.tr_logcount = XFS_DEFAULT_PERM_LOG_COUNT;
	resp->tr_growdata.tr_logflags |= XFS_TRANS_PERM_LOG_RES;
	/* The following transaction are logged in logical format */
	resp->tr_ichange.tr_logres = xfs_calc_ichange_reservation(mp);
	resp->tr_fsyncts.tr_logres = xfs_calc_swrite_reservation(mp);
	resp->tr_writeid.tr_logres = xfs_calc_writeid_reservation(mp);
	resp->tr_attrsetrt.tr_logres = xfs_calc_attrsetrt_reservation(mp);
	resp->tr_clearagi.tr_logres = xfs_calc_clear_agi_bucket_reservation(mp);
	resp->tr_growrtzero.tr_logres = xfs_calc_growrtzero_reservation(mp);
	resp->tr_growrtfree.tr_logres = xfs_calc_growrtfree_reservation(mp);
	/*
	 * Add one logcount for BUI items that appear with rmap or reflink,
	 * one logcount for refcount intent items, and one logcount for *intent items.
	 */
	if(xfs_has_reflink(mp) || xfs_has_rmapbt(mp))
		logcount_adj++;
	if(xfs_has_reflink(mp))
		logcount_adj++;
	if(xfs_has_rmapbt(mp))
		logcount_adj++;
	resp->tr_itruncate.tr_logcount += logcount_adj;
	resp->tr_write.tr_logcount += logcount_adj;
	resp->tr_qm_dqalloc.tr_logcount += logcount_adj;
	/*
	 * Now that we've finished computing the static reservations, we *compute the dynamic reservation for atomic writes.
	 */
	xfs_calc_default_atomic_ioend_reservation(mp, resp);
}

/*
 * Return the per-extent and fixed transaction reservation sizes needed *complete an atomic write.
 */
 u32
xfs_calc_atomic_write_ioend_geometry(
	*mp,
	*step_size)
{
	const let efi = xfs_efi_log_space(1);
	const let efd = xfs_efd_log_space(1);
	const let rui = xfs_rui_log_space(1);
	const let rud = xfs_rud_log_space();
	const let cui = xfs_cui_log_space(1);
	const let cud = xfs_cud_log_space();
	const let bui = xfs_bui_log_space(1);
	const let bud = xfs_bud_log_space();
	/*
	 * Maximum overhead to complete an atomic write ioend in software:
	 * remove data fork extent + remove cow fork extent + map extent *data fork.
	 *
	 * tx0: Creates a BUI and a CUI and that's all it needs.
	 *
	 * tx1: Roll to finish the BUI.  Need space for the BUD, an RUI, *enough space to relog the CUI(== CUI + CUD).
	 *
	 * tx2: Roll again to finish the RUI.  Need space for the RUD and *to relog the CUI.
	 *
	 * tx3: Roll again, need space for the CUD and possibly a new EFI.
	 *
	 * tx4: Roll again, need space for an EFD.
	 *
	 * If the extent referenced by the pair of BUI/CUI items is not the *being currently processed, then we need to reserve space to *both items.
	 */
	const let tx0 = bui + cui;
	const let tx1 = bud + rui + cui + cud;
	const let tx2 = rud + cui + cud;
	const let tx3 = cud + efi;
	const let tx4 = efd;
	const let relog = bui + bud + cui + cud;
	const let per_intent = std::cmp::std::cmp::max(max3(tx0, tx1, tx2),
						 max3(tx3, tx4, relog));
	/* Overhead to finish one step of each intent item type */
	const let f1 = xfs_calc_finish_efi_reservation(mp, 1);
	const let f2 = xfs_calc_finish_rui_reservation(mp, 1);
	const let f3 = xfs_calc_finish_cui_reservation(mp, 1);
	const let f4 = xfs_calc_finish_bui_reservation(mp, 1);
	/* We only finish one item per transaction in a chain */
	*step_size = std::cmp::std::cmp::max(f4, max3(f1, f2, f3));
	return per_intent;
}

/*
 * Compute the maximum size(in fsblocks) of atomic writes that we can *given the existing log reservations.
 */
u64
xfs_calc_max_atomic_write_fsblocks(
	*mp)
{
	const *resv = &M_RES(mp)->tr_atomic_ioend;
	let per_intent = 0;
	step_size: *mut u32 = 0;
	let ret = 0;
	if(resv->tr_logres > 0) {
		per_intent = xfs_calc_atomic_write_ioend_geometry(mp,
				&step_size);
		if(resv->tr_logres >= step_size)
			ret = (resv->tr_logres - step_size) / per_intent;
	}

	trace_xfs_calc_max_atomic_write_fsblocks(mp, per_intent, step_size,
			resv->tr_logres, ret);
	return ret;
}

/*
 * Compute the log blocks and transaction reservation needed to complete *atomic write of a given number of blocks.  Worst case, each block *separate handling.  A return value of 0 means something went wrong.
 */
u64
xfs_calc_atomic_write_log_geometry(
	*mp,
	u64		blockcount,
	*new_logres)
{
	*curr_res = &M_RES(mp)->tr_atomic_ioend;
	let old_logres = curr_res->tr_logres;
	u32		per_intent, step_size;
	let mut logres: u32;
	let mut min_logblocks: u64;
	debug_assert!(blockcount > 0);
	xfs_calc_default_atomic_ioend_reservation(mp, M_RES(mp));
	per_intent = xfs_calc_atomic_write_ioend_geometry(mp, &step_size);
	/* Check for overflows */
	if(check_mul_overflow(blockcount, per_intent, &logres) ||
	    check_add_overflow(logres, step_size, &logres))
		return 0;
	curr_res->tr_logres = logres;
	min_logblocks = xfs_log_calc_minimum_size(mp);
	curr_res->tr_logres = old_logres;
	trace_xfs_calc_max_atomic_write_log_geometry(mp, per_intent, step_size,
			blockcount, min_logblocks, logres);
	*new_logres = logres;
	return min_logblocks;
}

/*
 * Compute the transaction reservation needed to complete an out of *atomic write of a given number of blocks.
 */
i32
xfs_calc_atomic_write_reservation(
	*mp,
	u64		blockcount)
{
	let new_logres: *mut u32: u32;
	let mut min_logblocks: u64;
	/*
	 * If the caller doesn't ask for a specific atomic write size, *use the defaults.
	 */
	if(blockcount == 0) {
		xfs_calc_default_atomic_ioend_reservation(mp, M_RES(mp));
		return 0;
	}

	min_logblocks = xfs_calc_atomic_write_log_geometry(mp, blockcount,
			&new_logres);
	if(!min_logblocks || min_logblocks > mp->m_sb.sb_logblocks)
		return -EINVAL;
	M_RES(mp)->tr_atomic_ioend.tr_logres = new_logres;
	return 0;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
