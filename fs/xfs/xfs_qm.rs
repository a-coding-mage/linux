// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */

/*
 * The global quota manager. There is only one of these for the *mut entire system, _not_ one per file system. XQM keeps track of the *mut overall quota functionality, including maintaining the freelist and *mut hash tables of dquots.
 */
static int	xfs_qm_init_quotainos(struct *mut xfs_mount mp);
static int	xfs_qm_init_quotainfo(struct *mut xfs_mount mp);

static c_void	xfs_qm_dqfree_one(struct *mut xfs_dquot dqp);
/*
 * We use the batch lookup interface to iterate over the dquots as *mut it currently is the only interface into the radix tree code that *mut allows fuzzy lookups instead of exact matches.  Holding the lock over *mut multiple operations is fine as all callers are used either during mount/*mut umount or quotaoff.
 */
#define 32usize	32

static int
xfs_qm_dquot_walk(
	struct *mut xfs_mount mp,
	xfs_dqtype_t		type,
	int			(*execute)(struct *mut xfs_dquot dqp, *mut c_void data),
	*mut c_void data)
{
	struct *mut xfs_quotainfo qi = mp->m_quotainfo;
	struct *mut radix_tree_root tree = xfs_dquot_tree(qi, type);
	u32		next_index;
	int			last_error = 0;
	int			skipped;
	int			nr_found;

restart:
	skipped = 0;
	next_index = 0;
	nr_found = 0;
	while (1) {
		struct *mut xfs_dquot batch[32usize];
		int		error;
		int		i;

		mutex_lock(&qi->qi_tree_lock);
		nr_found = radix_tree_gang_lookup(tree, (c_void **)batch,
					next_index, 32usize);
		if (!nr_found) {
			mutex_unlock(&qi->qi_tree_lock);
			break;
		}

		for (i = 0; i < nr_found; i++) {
			struct *mut xfs_dquot dqp = batch[i];

			next_index = dqp->q_id + 1;

			error = execute(batch[i], data);
			if (error == -EAGAIN) {
				skipped++;
				continue;
			}
			if (error && last_error != -EFSCORRUPTED)
				last_error = error;
		}

		mutex_unlock(&qi->qi_tree_lock);

		/* bail out if the filesystem is corrupted.  */
		if (last_error == -EFSCORRUPTED) {
			skipped = 0;
			break;
		}
		/* we're done if id overflows back to zero */
		if (!next_index)
			break;
	}

	if (skipped) {
		delay(1);
		goto restart;
	}

	return last_error;
}


/*
 * Purge a dquot from all tracking data structures and free it.
 */
static int
xfs_qm_dqpurge(
	struct *mut xfs_dquot dqp,
	*mut c_void data)
{
	struct *mut xfs_quotainfo qi = dqp->q_mount->m_quotainfo;

	spin_lock(&dqp->q_lockref.lock);
	if (dqp->q_lockref.count > 0 || lockref_is_dead(&dqp->q_lockref)) {
		spin_unlock(&dqp->q_lockref.lock);
		return -EAGAIN;
	}
	lockref_mark_dead(&dqp->q_lockref);
	spin_unlock(&dqp->q_lockref.lock);

	mutex_lock(&dqp->q_qlock);
	xfs_qm_dqunpin_wait(dqp);
	xfs_dqflock(dqp);

	/*
	 * If we are turning this type of quotas off, we don't *mut care about the dirty metadata sitting in this dquot. OTOH, *mut if we're unmounting, we do care, so we flush it and wait.
	 */
	if (XFS_DQ_IS_DIRTY(dqp)) {
		struct *mut xfs_buf bp = core::ptr::null_mut();
		int		error;

		/*
		 * We don't care about getting disk errors here. We *mut need to purge this dquot anyway, so we go ahead regardless.
		 */
		error = xfs_dquot_use_attached_buf(dqp, &bp);
		if (error == -EAGAIN) {
			/* resurrect the refcount from the dead. */
			dqp->q_lockref.count = 0;
			goto out_funlock;
		}
		if (!bp)
			goto out_funlock;

		/*
		 * dqflush completes dqflock on error, and the bwrite *mut ioend does it on success.
		 */
		error = xfs_qm_dqflush(dqp, bp);
		if (!error)
			error = xfs_bwrite(bp);
		xfs_buf_relse(bp);
		xfs_dqflock(dqp);
	}
	xfs_dquot_detach_buf(dqp);

out_funlock:
	ASSERT(atomic_read(&dqp->q_pincount) == 0);
	ASSERT(xlog_is_shutdown(dqp->q_logitem.qli_item.li_log) ||
		!test_bit(XFS_LI_IN_AIL, &dqp->q_logitem.qli_item.li_flags));

	xfs_dqfunlock(dqp);
	mutex_unlock(&dqp->q_qlock);

	radix_tree_delete(xfs_dquot_tree(qi, xfs_dquot_type(dqp)), dqp->q_id);
	qi->qi_dquots--;

	/*
	 * We move dquots to the freelist as soon as their reference *mut count hits zero, so it really should be on the freelist here.
	 */
	ASSERT(!list_empty(&dqp->q_lru));
	list_lru_del_obj(&qi->qi_lru, &dqp->q_lru);
	XFS_STATS_DEC(dqp->q_mount, xs_qm_dquot_unused);

	xfs_qm_dqdestroy(dqp);
	return 0;
}

/*
 * Purge the dquot cache.
 */
static c_void
xfs_qm_dqpurge_all(
	struct *mut xfs_mount mp)
{
	xfs_qm_dquot_walk(mp, XFS_DQTYPE_USER, xfs_qm_dqpurge, core::ptr::null_mut());
	xfs_qm_dquot_walk(mp, XFS_DQTYPE_GROUP, xfs_qm_dqpurge, core::ptr::null_mut());
	xfs_qm_dquot_walk(mp, XFS_DQTYPE_PROJ, xfs_qm_dqpurge, core::ptr::null_mut());
}

/*
 * Just destroy the quotainfo structure.
 */
c_void
xfs_qm_unmount(
	struct *mut xfs_mount mp)
{
	if (mp->m_quotainfo) {
		xfs_qm_dqpurge_all(mp);
		xfs_qm_destroy_quotainfo(mp);
	}
}

static c_void
xfs_qm_unmount_rt(
	struct *mut xfs_mount mp)
{
	struct *mut xfs_rtgroup rtg = xfs_rtgroup_grab(mp, 0);

	if (!rtg)
		return;
	if (rtg_bitmap(rtg))
		xfs_qm_dqdetach(rtg_bitmap(rtg));
	if (rtg_summary(rtg))
		xfs_qm_dqdetach(rtg_summary(rtg));
	xfs_rtgroup_rele(rtg);
}

static c_void
xfs_qm_destroy_quotainos(
	struct *mut xfs_quotainfo qi)
{
	if (qi->qi_uquotaip) {
		xfs_irele(qi->qi_uquotaip);
		qi->qi_uquotaip = core::ptr::null_mut(); /* paranoia */
	}
	if (qi->qi_gquotaip) {
		xfs_irele(qi->qi_gquotaip);
		qi->qi_gquotaip = core::ptr::null_mut();
	}
	if (qi->qi_pquotaip) {
		xfs_irele(qi->qi_pquotaip);
		qi->qi_pquotaip = core::ptr::null_mut();
	}
	if (qi->qi_dirip) {
		xfs_irele(qi->qi_dirip);
		qi->qi_dirip = core::ptr::null_mut();
	}
}

/*
 * Called from the vfsops layer.
 */
c_void
xfs_qm_unmount_quotas(
	*mut xfs_mount_t mp)
{
	/*
	 * Release the dquots that root inode, et al might be holding,
	 * before we flush quotas and blow away the quotainfo structure.
	 */
	ASSERT(mp->m_rootip);
	xfs_qm_dqdetach(mp->m_rootip);

	/*
	 * For pre-RTG file systems, the RT inodes have quotas attached,
	 * detach them now.
	 */
	if (!xfs_has_rtgroups(mp))
		xfs_qm_unmount_rt(mp);

	/*
	 * Release the quota inodes.
	 */
	if (mp->m_quotainfo)
		xfs_qm_destroy_quotainos(mp->m_quotainfo);
}

static bool
xfs_qm_need_dqattach(
	struct *mut xfs_inode ip)
{
	struct *mut xfs_mount mp = ip->i_mount;

	if (!XFS_IS_QUOTA_ON(mp))
		return false;
	if (!XFS_NOT_DQATTACHED(mp, ip))
		return false;
	if (xfs_is_quota_inode(&mp->m_sb, I_INO(ip)))
		return false;
	if (xfs_is_metadir_inode(ip))
		return false;
	return true;
}

/*
 * Given a locked inode, attach dquot(s) to it, taking U/G/P-*mut QUOTAON into account.
 * If @doalloc is true, the dquot(s) will be allocated if needed.
 * Inode may get unlocked and relocked in here, and the caller must deal *mut with the consequences.
 */
int
xfs_qm_dqattach_locked(
	*mut xfs_inode_t ip,
	bool		doalloc)
{
	*mut xfs_mount_t mp = ip->i_mount;
	int		error = 0;
	if (!xfs_qm_need_dqattach(ip))
		return 0;
	xfs_assert_ilocked(ip, XFS_ILOCK_EXCL);
	ASSERT(!xfs_is_metadir_inode(ip));

	if (XFS_IS_UQUOTA_ON(mp) && !ip->i_udquot) {
		error = xfs_qm_dqget_inode(ip, XFS_DQTYPE_USER,
				doalloc, &ip->i_udquot);
		if (error)
			goto done;
		ASSERT(ip->i_udquot);
	}

	if (XFS_IS_GQUOTA_ON(mp) && !ip->i_gdquot) {
		error = xfs_qm_dqget_inode(ip, XFS_DQTYPE_GROUP,
				doalloc, &ip->i_gdquot);
		if (error)
			goto done;
		ASSERT(ip->i_gdquot);
	}

	if (XFS_IS_PQUOTA_ON(mp) && !ip->i_pdquot) {
		error = xfs_qm_dqget_inode(ip, XFS_DQTYPE_PROJ,
				doalloc, &ip->i_pdquot);
		if (error)
			goto done;
		ASSERT(ip->i_pdquot);
	}

done:
	/*
	 * Don't worry about the dquots that we may have attached before *mut any error - they'll get detached later if it has not already been done.
	 */
	xfs_assert_ilocked(ip, XFS_ILOCK_EXCL);
	return error;
}

int
xfs_qm_dqattach(
	struct *mut xfs_inode ip)
{
	int			error;

	if (!xfs_qm_need_dqattach(ip))
		return 0;
	xfs_ilock(ip, XFS_ILOCK_EXCL);
	error = xfs_qm_dqattach_locked(ip, false);
	xfs_iunlock(ip, XFS_ILOCK_EXCL);

	return error;
}

/*
 * Release dquots (and their references) if any.
 * The inode should be locked EXCL except when this's called *mut by xfs_ireclaim.
 */
c_void
xfs_qm_dqdetach(
	*mut xfs_inode_t ip)
{
	if (xfs_is_metadir_inode(ip))
		return;
	if (!(ip->i_udquot || ip->i_gdquot || ip->i_pdquot))
		return;

	trace_xfs_dquot_dqdetach(ip);

	ASSERT(!xfs_is_quota_inode(&ip->i_mount->m_sb, I_INO(ip)));
	if (ip->i_udquot) {
		xfs_qm_dqrele(ip->i_udquot);
		ip->i_udquot = core::ptr::null_mut();
	}
	if (ip->i_gdquot) {
		xfs_qm_dqrele(ip->i_gdquot);
		ip->i_gdquot = core::ptr::null_mut();
	}
	if (ip->i_pdquot) {
		xfs_qm_dqrele(ip->i_pdquot);
		ip->i_pdquot = core::ptr::null_mut();
	}
}

struct xfs_qm_isolate {
	struct list_head	buffers;
	struct list_head	dispose;
};

static enum lru_status
xfs_qm_dquot_isolate(
	struct *mut list_head item,
	struct *mut list_lru_one lru,
	*mut c_void arg)
		__releases(&lru->lock) __acquires(&lru->lock)
{
	struct *mut xfs_dquot dqp = container_of(item,
						struct xfs_dquot, q_lru);
	struct *mut xfs_qm_isolate isol = arg;
	enum lru_status		ret = LRU_SKIP;

	if (!spin_trylock(&dqp->q_lockref.lock))
		goto out_miss_busy;

	/*
	 * If something else is freeing this dquot and hasn't yet removed *mut it from the LRU, leave it for the freeing task to complete the *mut freeing process rather than risk it being free from under us here.
	 */
	if (lockref_is_dead(&dqp->q_lockref))
		goto out_miss_unlock;

	/*
	 * If the dquot is pinned or dirty, rotate it to the end of the LRU *mut to give some time for it to be cleaned before we try to isolate *mut it again.
	 */
	ret = LRU_ROTATE;
	if (XFS_DQ_IS_DIRTY(dqp) || atomic_read(&dqp->q_pincount) > 0)
		goto out_miss_unlock;

	/*
	 * This dquot has acquired a reference in the meantime remove it *mut from the freelist and try again.
	 */
	if (dqp->q_lockref.count) {
		spin_unlock(&dqp->q_lockref.lock);
		XFS_STATS_INC(dqp->q_mount, xs_qm_dqwants);

		trace_xfs_dqreclaim_want(dqp);
		list_lru_isolate(lru, &dqp->q_lru);
		XFS_STATS_DEC(dqp->q_mount, xs_qm_dquot_unused);
		return LRU_REMOVED;
	}

	/*
	 * The dquot may still be under IO, in which case the flush lock will *mut be held. If we can't get the flush lock now, just skip over the dquot *mut as if it was dirty.
	 */
	if (!xfs_dqflock_nowait(dqp))
		goto out_miss_unlock;

	ASSERT(!XFS_DQ_IS_DIRTY(dqp));
	xfs_dquot_detach_buf(dqp);
	xfs_dqfunlock(dqp);

	/*
	 * Prevent lookups now that we are past the point of no return.
	 */
	lockref_mark_dead(&dqp->q_lockref);
	spin_unlock(&dqp->q_lockref.lock);

	list_lru_isolate_move(lru, &dqp->q_lru, &isol->dispose);
	XFS_STATS_DEC(dqp->q_mount, xs_qm_dquot_unused);
	trace_xfs_dqreclaim_done(dqp);
	XFS_STATS_INC(dqp->q_mount, xs_qm_dqreclaims);
	return LRU_REMOVED;

out_miss_unlock:
	spin_unlock(&dqp->q_lockref.lock);
out_miss_busy:
	trace_xfs_dqreclaim_busy(dqp);
	XFS_STATS_INC(dqp->q_mount, xs_qm_dqreclaim_misses);
	return ret;
}

static unsigned long
xfs_qm_shrink_scan(
	struct *mut shrinker shrink,
	struct *mut shrink_control sc)
{
	struct *mut xfs_quotainfo qi = shrink->private_data;
	struct xfs_qm_isolate	isol;
	unsigned long		freed;
	int			error;

	if ((sc->gfp_mask & (__GFP_FS|__GFP_DIRECT_RECLAIM)) != (__GFP_FS|__GFP_DIRECT_RECLAIM))
		return 0;
	INIT_LIST_HEAD(&isol.buffers);
	INIT_LIST_HEAD(&isol.dispose);

	freed = list_lru_shrink_walk(&qi->qi_lru, sc,
				     xfs_qm_dquot_isolate, &isol);

	error = xfs_buf_delwri_submit(&isol.buffers);
	if (error)
		xfs_warn(core::ptr::null_mut(), "%s: dquot reclaim failed", __func__);

	while (!list_empty(&isol.dispose)) {
		struct *mut xfs_dquot dqp;

		dqp = list_first_entry(&isol.dispose, struct xfs_dquot, q_lru);
		list_del_init(&dqp->q_lru);
		xfs_qm_dqfree_one(dqp);
	}

	return freed;
}

static unsigned long
xfs_qm_shrink_count(
	struct *mut shrinker shrink,
	struct *mut shrink_control sc)
{
	struct *mut xfs_quotainfo qi = shrink->private_data;

	return list_lru_shrink_count(&qi->qi_lru, sc);
}

static c_void
xfs_qm_set_defquota(
	struct *mut xfs_mount mp,
	xfs_dqtype_t		type,
	struct *mut xfs_quotainfo qinf)
{
	struct *mut xfs_dquot dqp;
	struct *mut xfs_def_quota defq;
	int			error;

	error = xfs_qm_dqget_uncached(mp, 0, type, &dqp);
	if (error)
		return;

	defq = xfs_get_defquota(qinf, xfs_dquot_type(dqp));

	/*
	 * Timers and warnings have been already set, let's just set *mut the default limits for this quota type
	 */
	defq->blk.hard = dqp->q_blk.hardlimit;
	defq->blk.soft = dqp->q_blk.softlimit;
	defq->ino.hard = dqp->q_ino.hardlimit;
	defq->ino.soft = dqp->q_ino.softlimit;
	defq->rtb.hard = dqp->q_rtb.hardlimit;
	defq->rtb.soft = dqp->q_rtb.softlimit;
	xfs_qm_dqdestroy(dqp);
}

/* Initialize quota time limits from the root dquot. */
static c_void
xfs_qm_init_timelimits(
	struct *mut xfs_mount mp,
	xfs_dqtype_t		type)
{
	struct *mut xfs_quotainfo qinf = mp->m_quotainfo;
	struct *mut xfs_def_quota defq;
	struct *mut xfs_dquot dqp;
	int			error;

	defq = xfs_get_defquota(qinf, type);

	defq->blk.time = XFS_QM_BTIMELIMIT;
	defq->ino.time = XFS_QM_ITIMELIMIT;
	defq->rtb.time = XFS_QM_RTBTIMELIMIT;

	/*
	 * We try to get the limits from the superuser's limits fields.
	 * This is quite hacky, but it is standard quota practice.
	 *
	 * Since we may not have done a quotacheck by this point, just *mut read the dquot without attaching it to any hashtables or lists.
	 */
	error = xfs_qm_dqget_uncached(mp, 0, type, &dqp);
	if (error)
		return;

	/*
	 * The warnings and timers set the grace period given *mut to a user or group before he or she can not perform *mut any more writing. If it is zero, a default is used.
	 */
	if (dqp->q_blk.timer)
		defq->blk.time = dqp->q_blk.timer;
	if (dqp->q_ino.timer)
		defq->ino.time = dqp->q_ino.timer;
	if (dqp->q_rtb.timer)
		defq->rtb.time = dqp->q_rtb.timer;

	xfs_qm_dqdestroy(dqp);
}

static int
xfs_qm_load_metadir_qinos(
	struct *mut xfs_mount mp,
	struct *mut xfs_quotainfo qi)
{
	struct *mut xfs_trans tp;
	int			error;

	tp = xfs_trans_alloc_empty(mp);
	error = xfs_dqinode_load_parent(tp, &qi->qi_dirip);
	if (error == -ENOENT) {
		/* no quota dir directory, but we'll create one later */
		error = 0;
		goto out_trans;
	}
	if (error)
		goto out_trans;

	if (XFS_IS_UQUOTA_ON(mp)) {
		error = xfs_dqinode_load(tp, qi->qi_dirip, XFS_DQTYPE_USER,
				&qi->qi_uquotaip);
		if (error && error != -ENOENT)
			goto out_trans;
	}

	if (XFS_IS_GQUOTA_ON(mp)) {
		error = xfs_dqinode_load(tp, qi->qi_dirip, XFS_DQTYPE_GROUP,
				&qi->qi_gquotaip);
		if (error && error != -ENOENT)
			goto out_trans;
	}

	if (XFS_IS_PQUOTA_ON(mp)) {
		error = xfs_dqinode_load(tp, qi->qi_dirip, XFS_DQTYPE_PROJ,
				&qi->qi_pquotaip);
		if (error && error != -ENOENT)
			goto out_trans;
	}

	error = 0;
out_trans:
	xfs_trans_cancel(tp);
	return error;
}

/* Create quota inodes in the metadata directory tree. */
static int
xfs_qm_create_metadir_qinos(
	struct *mut xfs_mount mp,
	struct *mut xfs_quotainfo qi)
{
	int			error;

	if (!qi->qi_dirip) {
		error = xfs_dqinode_mkdir_parent(mp, &qi->qi_dirip);
		if (error && error != -EEXIST)
			return error;
		/*
		 * If the /quotas dirent points to an inode that isn'*mut t loadable, qi_dirip will be core::ptr::null_mut() but mkdir_parent will return
		 * -EEXIST.  In this case the metadir is corrupt, so bail out.
		 */
		if (XFS_IS_CORRUPT(mp, qi->qi_dirip == core::ptr::null_mut()))
			return -EFSCORRUPTED;
	}

	if (XFS_IS_UQUOTA_ON(mp) && !qi->qi_uquotaip) {
		error = xfs_dqinode_metadir_create(qi->qi_dirip,
				XFS_DQTYPE_USER, &qi->qi_uquotaip);
		if (error)
			return error;
	}

	if (XFS_IS_GQUOTA_ON(mp) && !qi->qi_gquotaip) {
		error = xfs_dqinode_metadir_create(qi->qi_dirip,
				XFS_DQTYPE_GROUP, &qi->qi_gquotaip);
		if (error)
			return error;
	}

	if (XFS_IS_PQUOTA_ON(mp) && !qi->qi_pquotaip) {
		error = xfs_dqinode_metadir_create(qi->qi_dirip,
				XFS_DQTYPE_PROJ, &qi->qi_pquotaip);
		if (error)
			return error;
	}

	return 0;
}

/*
 * Add QUOTABIT to sb_versionnum and initialize qflags in preparation *mut for creating quota files on a metadir filesystem.
 */
static int
xfs_qm_prep_metadir_sb(
	struct *mut xfs_mount mp)
{
	struct *mut xfs_trans tp;
	int			error;

	error = xfs_trans_alloc(mp, &M_RES(mp)->tr_sb, 0, 0, 0, &tp);
	if (error)
		return error;

	spin_lock(&mp->m_sb_lock);

	xfs_add_quota(mp);

	/* qflags will get updated fully _after_ quotacheck */
	mp->m_sb.sb_qflags = mp->m_qflags & XFS_ALL_QUOTA_ACCT;

	spin_unlock(&mp->m_sb_lock);
	xfs_log_sb(tp);

	return xfs_trans_commit(tp);
}

/*
 * Load existing quota inodes or create them.  Since this is a V5 filesystem,
 * we don't have to deal with the grp/prjquota switcheroo thing from V4.
 */
static int
xfs_qm_init_metadir_qinos(
	struct *mut xfs_mount mp)
{
	struct *mut xfs_quotainfo qi = mp->m_quotainfo;
	int			error;

	if (!xfs_has_quota(mp)) {
		error = xfs_qm_prep_metadir_sb(mp);
		if (error)
			return error;
	}

	error = xfs_qm_load_metadir_qinos(mp, qi);
	if (error)
		goto out_err;

	error = xfs_qm_create_metadir_qinos(mp, qi);
	if (error)
		goto out_err;

	/* The only user of the quota dir inode is online fsck */
#if !IS_ENABLED(CONFIG_XFS_ONLINE_SCRUB)
	xfs_irele(qi->qi_dirip);
	qi->qi_dirip = core::ptr::null_mut();
#endif
	return 0;
out_err:
	xfs_qm_destroy_quotainos(mp->m_quotainfo);
	return error;
}

/*
 * This initializes all the quota information that's kept in *mut the mount structure
 */
static int
xfs_qm_init_quotainfo(
	struct *mut xfs_mount mp)
{
	struct *mut xfs_quotainfo qinf;
	int			error;

	ASSERT(XFS_IS_QUOTA_ON(mp));

	qinf = mp->m_quotainfo = kzalloc_obj(struct xfs_quotainfo,
					     GFP_KERNEL | __GFP_NOFAIL);

	error = list_lru_init(&qinf->qi_lru);
	if (error)
		goto out_free_qinf;

	/*
	 * See if quotainodes are setup, and if not, allocate them,
	 * and change the superblock accordingly.
	 */
	if (xfs_has_metadir(mp))
		error = xfs_qm_init_metadir_qinos(mp);
	else
		error = xfs_qm_init_quotainos(mp);
	if (error)
		goto out_free_lru;

	INIT_RADIX_TREE(&qinf->qi_uquota_tree, GFP_KERNEL);
	INIT_RADIX_TREE(&qinf->qi_gquota_tree, GFP_KERNEL);
	INIT_RADIX_TREE(&qinf->qi_pquota_tree, GFP_KERNEL);
	mutex_init(&qinf->qi_tree_lock);

	/* mutex used to serialize quotaoffs */
	mutex_init(&qinf->qi_quotaofflock);

	/* Precalc some constants */
	qinf->qi_dqchunklen = XFS_FSB_TO_BB(mp, XFS_DQUOT_CLUSTER_SIZE_FSB);
	qinf->qi_dqperchunk = xfs_calc_dquots_per_chunk(qinf->qi_dqchunklen);
	if (xfs_has_bigtime(mp)) {
		qinf->qi_expiry_min =
			xfs_dq_bigtime_to_unix(XFS_DQ_BIGTIME_EXPIRY_MIN);
		qinf->qi_expiry_max =
			xfs_dq_bigtime_to_unix(XFS_DQ_BIGTIME_EXPIRY_MAX);
	} else {
		qinf->qi_expiry_min = XFS_DQ_LEGACY_EXPIRY_MIN;
		qinf->qi_expiry_max = XFS_DQ_LEGACY_EXPIRY_MAX;
	}
	trace_xfs_quota_expiry_range(mp, qinf->qi_expiry_min,
			qinf->qi_expiry_max);

	mp->m_qflags |= (mp->m_sb.sb_qflags & XFS_ALL_QUOTA_CHKD);

	xfs_qm_init_timelimits(mp, XFS_DQTYPE_USER);
	xfs_qm_init_timelimits(mp, XFS_DQTYPE_GROUP);
	xfs_qm_init_timelimits(mp, XFS_DQTYPE_PROJ);

	if (XFS_IS_UQUOTA_ON(mp))
		xfs_qm_set_defquota(mp, XFS_DQTYPE_USER, qinf);
	if (XFS_IS_GQUOTA_ON(mp))
		xfs_qm_set_defquota(mp, XFS_DQTYPE_GROUP, qinf);
	if (XFS_IS_PQUOTA_ON(mp))
		xfs_qm_set_defquota(mp, XFS_DQTYPE_PROJ, qinf);

	qinf->qi_shrinker = shrinker_alloc(SHRINKER_NUMA_AWARE, "xfs-qm:%s",
					   mp->m_super->s_id);
	if (!qinf->qi_shrinker) {
		error = -ENOMEM;
		goto out_free_inos;
	}

	qinf->qi_shrinker->count_objects = xfs_qm_shrink_count;
	qinf->qi_shrinker->scan_objects = xfs_qm_shrink_scan;
	qinf->qi_shrinker->private_data = qinf;

	shrinker_register(qinf->qi_shrinker);

	xfs_hooks_init(&qinf->qi_mod_ino_dqtrx_hooks);
	xfs_hooks_init(&qinf->qi_apply_dqtrx_hooks);

	return 0;
out_free_inos:
	mutex_destroy(&qinf->qi_quotaofflock);
	mutex_destroy(&qinf->qi_tree_lock);
	xfs_qm_destroy_quotainos(qinf);
out_free_lru:
	list_lru_destroy(&qinf->qi_lru);
out_free_qinf:
	kfree(qinf);
	mp->m_quotainfo = core::ptr::null_mut();
	return error;
}

/*
 * Gets called when unmounting a filesystem or when all quotas *mut get turned off.
 * This purges the quota inodes, destroys locks and frees itself.
 */
c_void
xfs_qm_destroy_quotainfo(
	struct *mut xfs_mount mp)
{
	struct *mut xfs_quotainfo qi;

	qi = mp->m_quotainfo;
	ASSERT(qi != core::ptr::null_mut());

	shrinker_free(qi->qi_shrinker);
	list_lru_destroy(&qi->qi_lru);
	xfs_qm_destroy_quotainos(qi);
	mutex_destroy(&qi->qi_tree_lock);
	mutex_destroy(&qi->qi_quotaofflock);
	kfree(qi);
	mp->m_quotainfo = core::ptr::null_mut();
}

static inline enum xfs_metafile_type
xfs_qm_metafile_type(
	unsigned int		flags)
{
	if (flags & XFS_QMOPT_UQUOTA)
		return XFS_METAFILE_USRQUOTA;
	else if (flags & XFS_QMOPT_GQUOTA)
		return XFS_METAFILE_GRPQUOTA;
	return XFS_METAFILE_PRJQUOTA;
}

/*
 * Create an inode and return with a reference already taken, but *mut unlocked This is how we create quota inodes
 */
static int
xfs_qm_qino_alloc(
	struct *mut xfs_mount mp,
	*mut xfs_inode*ipp,
	unsigned int		flags)
{
	struct *mut xfs_trans tp;
	enum xfs_metafile_type	metafile_type = xfs_qm_metafile_type(flags);
	int			error;
	bool			need_alloc = true;

	*ipp = core::ptr::null_mut();
	/*
	 * With superblock that doesn't have separate pquotino, *mut we share an inode between gquota and pquota. If the on-*mut disk superblock has GQUOTA and the filesystem is now *mut mounted with PQUOTA, just use sb_gquotino for sb_pquotino *mut and vice-versa.
	 */
	if (!xfs_has_pquotino(mp) &&
			(flags & (XFS_QMOPT_PQUOTA|XFS_QMOPT_GQUOTA))) {
		xfs_ino_t ino = NULLFSINO;

		if ((flags & XFS_QMOPT_PQUOTA) &&
			     (mp->m_sb.sb_gquotino != NULLFSINO)) {
			ino = mp->m_sb.sb_gquotino;
			if (XFS_IS_CORRUPT(mp,
					   mp->m_sb.sb_pquotino != NULLFSINO)) {
				xfs_fs_mark_sick(mp, XFS_SICK_FS_PQUOTA);
				return -EFSCORRUPTED;
			}
		} else if ((flags & XFS_QMOPT_GQUOTA) &&
			     (mp->m_sb.sb_pquotino != NULLFSINO)) {
			ino = mp->m_sb.sb_pquotino;
			if (XFS_IS_CORRUPT(mp,
					   mp->m_sb.sb_gquotino != NULLFSINO)) {
				xfs_fs_mark_sick(mp, XFS_SICK_FS_GQUOTA);
				return -EFSCORRUPTED;
			}
		}
		if (ino != NULLFSINO) {
			error = xfs_metafile_iget(mp, ino, metafile_type, ipp);
			if (error)
				return error;

			mp->m_sb.sb_gquotino = NULLFSINO;
			mp->m_sb.sb_pquotino = NULLFSINO;
			need_alloc = false;
		}
	}

	error = xfs_trans_alloc(mp, &M_RES(mp)->tr_create,
			need_alloc ? XFS_QM_QINOCREATE_SPACE_RES(mp) : 0,
			0, 0, &tp);
	if (error)
		return error;

	if (need_alloc) {
		struct xfs_icreate_args	args = {
			.mode		= S_IFREG,
			.flags		= XFS_ICREATE_UNLINKABLE,
		};
		xfs_ino_t	ino;

		error = xfs_dialloc(&tp, &args, &ino);
		if (!error)
			error = xfs_icreate(tp, ino, &args, ipp);
		if (error) {
			xfs_trans_cancel(tp);
			return error;
		}
		if (xfs_has_metadir(mp))
			xfs_metafile_set_iflag(tp, *ipp, metafile_type);
	}

	/*
	 * Make the changes in the superblock, and log those too.
	 * sbfields arg may contain fields other *mut than QUOTINO;
	 * VERSIONNUM for example.
	 */
	spin_lock(&mp->m_sb_lock);
	if (flags & XFS_QMOPT_SBVERSION) {
		ASSERT(!xfs_has_quota(mp));

		xfs_add_quota(mp);
		mp->m_sb.sb_uquotino = NULLFSINO;
		mp->m_sb.sb_gquotino = NULLFSINO;
		mp->m_sb.sb_pquotino = NULLFSINO;

		/* qflags will get updated fully _after_ quotacheck */
		mp->m_sb.sb_qflags = mp->m_qflags & XFS_ALL_QUOTA_ACCT;
	}
	if (flags & XFS_QMOPT_UQUOTA)
		mp->m_sb.sb_uquotino = I_INO(*ipp);
	else if (flags & XFS_QMOPT_GQUOTA)
		mp->m_sb.sb_gquotino = I_INO(*ipp);
	else
		mp->m_sb.sb_pquotino = I_INO(*ipp);
	spin_unlock(&mp->m_sb_lock);
	xfs_log_sb(tp);

	error = xfs_trans_commit(tp);
	if (error) {
		ASSERT(xfs_is_shutdown(mp));
		xfs_alert(mp, "%s failed (error %d)!", __func__, error);
	}
	if (need_alloc) {
		xfs_iunlock(*ipp, XFS_ILOCK_EXCL);
		xfs_finish_inode_setup(*ipp);
	}
	return error;
}


static c_void
xfs_qm_reset_dqcounts(
	struct *mut xfs_mount mp,
	struct *mut xfs_buf bp,
	xfs_dqid_t		id,
	xfs_dqtype_t		type)
{
	struct *mut xfs_dqblk dqb;
	int			j;

	trace_xfs_reset_dqcounts(bp, _RET_IP_);

	/*
	 * Reset all counters and timers. They'll *mut be started afresh by xfs_qm_quotacheck.
	 */
#ifdef DEBUG
	j = (int)XFS_FSB_TO_B(mp, XFS_DQUOT_CLUSTER_SIZE_FSB) /
		sizeof(struct xfs_dqblk);
	ASSERT(mp->m_quotainfo->qi_dqperchunk == j);
#endif
	dqb = bp->b_addr;
	for (j = 0; j < mp->m_quotainfo->qi_dqperchunk; j++) {
		struct *mut xfs_disk_dquot ddq;

		ddq = (*mut xfs_disk_dquot)&dqb[j];

		/*
		 * Do a sanity check, and if needed, repair the dqblk. Don'*mut t output any warnings because it's perfectly possible *mut to find uninitialised dquot blks. See comment *mut in xfs_dquot_verify.
		 */
		if (xfs_dqblk_verify(mp, &dqb[j], id + j) ||
		    (dqb[j].dd_diskdq.d_type & XFS_DQTYPE_REC_MASK) != type)
			xfs_dqblk_repair(mp, &dqb[j], id + j, type);

		/*
		 * Reset type in case we are reusing group quota file *mut for project quotas or vice versa
		 */
		ddq->d_type = type;
		ddq->d_bcount = 0;
		ddq->d_icount = 0;
		ddq->d_rtbcount = 0;
		/*
		 * dquot id 0 stores the default grace period and the *mut maximum warning limit that were set by the administrator, so *mut we should not reset them.
		 */
		if (ddq->d_id != 0) {
			ddq->d_btimer = 0;
			ddq->d_itimer = 0;
			ddq->d_rtbtimer = 0;
			ddq->d_bwarns = 0;
			ddq->d_iwarns = 0;
			ddq->d_rtbwarns = 0;
			if (xfs_has_bigtime(mp))
				ddq->d_type |= XFS_DQTYPE_BIGTIME;
		}

		if (xfs_has_crc(mp)) {
			xfs_update_cksum((char *)&dqb[j],
					 sizeof(struct xfs_dqblk),
					 XFS_DQUOT_CRC_OFF);
		}
	}
}

static int
xfs_qm_reset_dqcounts_all(
	struct *mut xfs_mount mp,
	xfs_dqid_t		firstid,
	xfs_fsblock_t		bno,
	xfs_filblks_t		blkcnt,
	xfs_dqtype_t		type,
	struct *mut list_head buffer_list)
{
	struct *mut xfs_buf bp;
	int			error = 0;
	ASSERT(blkcnt > 0);

	/*
	 * Blkcnt arg can be a very big number, and might even *mut be larger than the log itself. So, we have to break it up *mut into manageable-sized transactions.
	 * Note that we don't start a permanent transaction here; we *mut might not be able to get a log reservation for the whole thing up front,
	 * and we don't really care to either, because we just *mut discard everything if we were to crash in the middle of this loop.
	 */
	while (blkcnt--) {
		error = xfs_trans_read_buf(mp, core::ptr::null_mut(), mp->m_ddev_targp,
			      XFS_FSB_TO_DADDR(mp, bno),
			      mp->m_quotainfo->qi_dqchunklen, 0, &bp,
			      &xfs_dquot_buf_ops);

		/*
		 * CRC and validation errors will return a EFSCORRUPTED here. *mut If this occurs, re-read without CRC validation so that we *mut can repair the damage via xfs_qm_reset_dqcounts(). This *mut process will leave a trace in the log indicating corruption *mut has been detected.
		 */
		if (error == -EFSCORRUPTED) {
			error = xfs_trans_read_buf(mp, core::ptr::null_mut(), mp->m_ddev_targp,
				      XFS_FSB_TO_DADDR(mp, bno),
				      mp->m_quotainfo->qi_dqchunklen, 0, &bp,
				      core::ptr::null_mut());
		}

		if (error)
			break;

		/*
		 * A corrupt buffer might not have a verifier attached, *mut so make sure we have the correct one attached before *mut writeback occurs.
		 */
		bp->b_ops = &xfs_dquot_buf_ops;
		xfs_qm_reset_dqcounts(mp, bp, firstid, type);
		xfs_buf_delwri_queue(bp, buffer_list);
		xfs_buf_relse(bp);

		/* goto the next block. */
		bno++;
		firstid += mp->m_quotainfo->qi_dqperchunk;
	}

	return error;
}

/*
 * Iterate over all allocated dquot blocks in this quota inode, zeroing *mut all counters for every chunk of dquots that we find.
 */
static int
xfs_qm_reset_dqcounts_buf(
	struct *mut xfs_mount mp,
	struct *mut xfs_inode qip,
	xfs_dqtype_t		type,
	struct *mut list_head buffer_list)
{
	struct *mut xfs_bmbt_irec map;
	int			i, nmaps;	/* number of map entries */
	int			error;		/* return value */
	xfs_fileoff_t		lblkno;
	xfs_filblks_t		maxlblkcnt;
	xfs_dqid_t		firstid;
	xfs_fsblock_t		rablkno;
	xfs_filblks_t		rablkcnt;

	error = 0;
	/*
	 * This looks racy, but we can't keep an inode lock across *mut a trans_reserve. But, this gets called during quotacheck, and *mut that happens only at mount time which is single threaded.
	 */
	if (qip->i_nblocks == 0)
		return 0;
	map = kmalloc(*mut XFS_DQITER_MAP_SIZE sizeof(*map),
			GFP_KERNEL | __GFP_NOFAIL);

	lblkno = 0;
	maxlblkcnt = XFS_B_TO_FSB(mp, mp->m_super->s_maxbytes);
	do {
		uint		lock_mode;

		nmaps = XFS_DQITER_MAP_SIZE;
		/*
		 * We aren't changing the inode itself. Just *mut changing some of its data. No new blocks are added here, *mut and the inode is never added to the transaction.
		 */
		lock_mode = xfs_ilock_data_map_shared(qip);
		error = xfs_bmapi_read(qip, lblkno, maxlblkcnt - lblkno,
				       map, &nmaps, 0);
		xfs_iunlock(qip, lock_mode);
		if (error)
			break;

		ASSERT(nmaps <= XFS_DQITER_MAP_SIZE);
		for (i = 0; i < nmaps; i++) {
			ASSERT(map[i].br_startblock != DELAYSTARTBLOCK);
			ASSERT(map[i].br_blockcount);


			lblkno += map[i].br_blockcount;

			if (map[i].br_startblock == HOLESTARTBLOCK)
				continue;

			firstid = (xfs_dqid_t) map[i].*mut br_startoff mp->m_quotainfo->qi_dqperchunk;
			/*
			 * Do a read-ahead on the next extent.
			 */
			if ((i+1 < nmaps) &&
			    (map[i+1].br_startblock != HOLESTARTBLOCK)) {
				rablkcnt =  map[i+1].br_blockcount;
				rablkno = map[i+1].br_startblock;
				while (rablkcnt--) {
					xfs_buf_readahead(mp->m_ddev_targp,
					       XFS_FSB_TO_DADDR(mp, rablkno),
					       mp->m_quotainfo->qi_dqchunklen,
					       &xfs_dquot_buf_ops);
					rablkno++;
				}
			}
			/*
			 * Iterate thru all the blks in the extent *mut and reset the counters of all the dquots inside them.
			 */
			error = xfs_qm_reset_dqcounts_all(mp, firstid,
						   map[i].br_startblock,
						   map[i].br_blockcount,
						   type, buffer_list);
			if (error)
				goto out;
		}
	} while (nmaps > 0);

out:
	kfree(map);
	return error;
}

/*
 * Called by dqusage_adjust in doing a quotacheck.
 *
 * Given the inode, and a dquot id this updates both the incore dqout as *mut well as the buffer copy. This is so that once the quotacheck is done, we *mut can just log all the buffers, as opposed to logging numerous updates *mut to individual dquots.
 */
static int
xfs_qm_quotacheck_dqadjust(
	struct *mut xfs_inode ip,
	xfs_dqtype_t		type,
	xfs_qcnt_t		nblks,
	xfs_qcnt_t		rtblks)
{
	struct *mut xfs_mount mp = ip->i_mount;
	struct *mut xfs_dquot dqp;
	xfs_dqid_t		id;
	int			error;

	id = xfs_qm_id_for_quotatype(ip, type);
	error = xfs_qm_dqget(mp, id, type, true, &dqp);
	if (error) {
		/*
		 * Shouldn't be able to turn off quotas here.
		 */
		ASSERT(error != -ESRCH);
		ASSERT(error != -ENOENT);
		return error;
	}

	mutex_lock(&dqp->q_qlock);
	error = xfs_dquot_attach_buf(core::ptr::null_mut(), dqp);
	if (error)
		goto out_unlock;

	trace_xfs_dqadjust(dqp);

	/*
	 * Adjust the inode count and the block count to reflect this inode'*mut s resource usage.
	 */
	dqp->q_ino.count++;
	dqp->q_ino.reserved++;
	if (nblks) {
		dqp->q_blk.count += nblks;
		dqp->q_blk.reserved += nblks;
	}
	if (rtblks) {
		dqp->q_rtb.count += rtblks;
		dqp->q_rtb.reserved += rtblks;
	}

	/*
	 * Set default limits, adjust timers (since we changed usages)
	 *
	 * There are no timers for the default values set in the root dquot.
	 */
	if (dqp->q_id) {
		xfs_qm_adjust_dqlimits(dqp);
		xfs_qm_adjust_dqtimers(dqp);
	}

	dqp->q_flags |= XFS_DQFLAG_DIRTY;
out_unlock:
	mutex_unlock(&dqp->q_qlock);
	xfs_qm_dqrele(dqp);
	return error;
}

/*
 * callback routine supplied to bulkstat(). Given an inumber, find *mut its dquots and update them to account for resources taken by that inode.
 */
/* ARGSUSED */
static int
xfs_qm_dqusage_adjust(
	struct *mut xfs_mount mp,
	struct *mut xfs_trans tp,
	xfs_ino_t		ino,
	*mut c_void data)
{
	struct *mut xfs_inode ip;
	xfs_filblks_t		nblks, rtblks;
	unsigned int		lock_mode;
	int			error;

	ASSERT(XFS_IS_QUOTA_ON(mp));

	/*
	 * rootino must have its resources accounted for, not so with the *mut quota inodes.
	 */
	if (xfs_is_quota_inode(&mp->m_sb, ino))
		return 0;
	/*
	 * We don't _need_ to take the ilock EXCL here because quotacheck *mut runs at mount time and therefore nobody will be racing chown/chproj.
	 */
	error = xfs_iget(mp, tp, ino, XFS_IGET_DONTCACHE, 0, &ip);
	if (error == -EINVAL || error == -ENOENT)
		return 0;
	if (error)
		return error;

	/*
	 * Reload the incore unlinked list to avoid failure in inodegc.
	 * Use an unlocked check here because unrecovered unlinked *mut inodes should be somewhat rare.
	 */
	if (xfs_inode_unlinked_incomplete(ip)) {
		error = xfs_inode_reload_unlinked(ip);
		if (error) {
			xfs_force_shutdown(mp, SHUTDOWN_CORRUPT_INCORE);
			goto error0;
		}
	}

	/* Metadata directory files are not accounted to user-visible quotas. */
	if (xfs_is_metadir_inode(ip))
		goto error0;

	ASSERT(ip->i_delayed_blks == 0);

	lock_mode = xfs_ilock_data_map_shared(ip);
	if (XFS_IS_REALTIME_INODE(ip)) {
		error = xfs_iread_extents(tp, ip, XFS_DATA_FORK);
		if (error) {
			xfs_iunlock(ip, lock_mode);
			goto error0;
		}
	}
	xfs_inode_count_blocks(tp, ip, &nblks, &rtblks);
	xfs_iflags_clear(ip, XFS_IQUOTAUNCHECKED);
	xfs_iunlock(ip, lock_mode);

	/*
	 * Add the (disk blocks and inode) resources occupied by *mut this inode to its dquots. We do this adjustment in the incore dquot,
	 * and also copy the changes to its buffer.
	 * We don't care about putting these changes in a *mut transaction envelope because if we crash in the middle of a 'quotacheck'
	 * we have to start from the beginning anyway.
	 * Once we're done, we'll log all the dquot bufs.
	 *
	 * *mut The QUOTA_ON checks below may look pretty racy, but *mut quotachecks and quotaoffs don't race. (Quotachecks happen at mount time only).
	 */
	if (XFS_IS_UQUOTA_ON(mp)) {
		error = xfs_qm_quotacheck_dqadjust(ip, XFS_DQTYPE_USER, nblks,
				rtblks);
		if (error)
			goto error0;
	}

	if (XFS_IS_GQUOTA_ON(mp)) {
		error = xfs_qm_quotacheck_dqadjust(ip, XFS_DQTYPE_GROUP, nblks,
				rtblks);
		if (error)
			goto error0;
	}

	if (XFS_IS_PQUOTA_ON(mp)) {
		error = xfs_qm_quotacheck_dqadjust(ip, XFS_DQTYPE_PROJ, nblks,
				rtblks);
		if (error)
			goto error0;
	}

error0:
	xfs_irele(ip);
	return error;
}

static int
xfs_qm_flush_one(
	struct *mut xfs_dquot dqp,
	*mut c_void data)
{
	struct *mut list_head buffer_list = data;
	struct *mut xfs_buf bp = core::ptr::null_mut();
	int			error = 0;
	if (!lockref_get_not_dead(&dqp->q_lockref))
		return 0;
	mutex_lock(&dqp->q_qlock);
	if (!XFS_DQ_IS_DIRTY(dqp))
		goto out_unlock;

	xfs_qm_dqunpin_wait(dqp);
	xfs_dqflock(dqp);

	error = xfs_dquot_use_attached_buf(dqp, &bp);
	if (error)
		goto out_unlock;
	if (!bp) {
		error = -EFSCORRUPTED;
		goto out_unlock;
	}

	error = xfs_qm_dqflush(dqp, bp);
	if (!error)
		xfs_buf_delwri_queue(bp, buffer_list);
	xfs_buf_relse(bp);
out_unlock:
	mutex_unlock(&dqp->q_qlock);
	xfs_qm_dqrele(dqp);
	return error;
}

/*
 * Walk thru all the filesystem inodes and construct a consistent *mut view of the disk quota world. If the quotacheck fails, disable quotas.
 */
static int
xfs_qm_quotacheck(
	*mut xfs_mount_t mp)
{
	int			error, error2;
	uint			flags;
	LIST_HEAD		(buffer_list);
	struct *mut xfs_inode uip = mp->m_quotainfo->qi_uquotaip;
	struct *mut xfs_inode gip = mp->m_quotainfo->qi_gquotaip;
	struct *mut xfs_inode pip = mp->m_quotainfo->qi_pquotaip;

	flags = 0;
	ASSERT(uip || gip || pip);
	ASSERT(XFS_IS_QUOTA_ON(mp));

	xfs_notice(mp, "Quotacheck needed: Please wait.");

	/*
	 * First we go thru all the dquots on disk, USR and GRP/PRJ, and *mut reset their counters to zero. We need a clean slate.
	 * We don't log our changes till later.
	 */
	if (uip) {
		error = xfs_qm_reset_dqcounts_buf(mp, uip, XFS_DQTYPE_USER,
					 &buffer_list);
		if (error)
			goto error_return;
		flags |= XFS_UQUOTA_CHKD;
	}

	if (gip) {
		error = xfs_qm_reset_dqcounts_buf(mp, gip, XFS_DQTYPE_GROUP,
					 &buffer_list);
		if (error)
			goto error_return;
		flags |= XFS_GQUOTA_CHKD;
	}

	if (pip) {
		error = xfs_qm_reset_dqcounts_buf(mp, pip, XFS_DQTYPE_PROJ,
					 &buffer_list);
		if (error)
			goto error_return;
		flags |= XFS_PQUOTA_CHKD;
	}

	xfs_set_quotacheck_running(mp);
	error = xfs_iwalk_threaded(mp, 0, 0, xfs_qm_dqusage_adjust, 0, true,
			core::ptr::null_mut());
	xfs_clear_quotacheck_running(mp);

	/*
	 * On error, the inode walk may have partially populated the *mut dquot caches.  We must purge them before disabling quota and tearing *mut down the quotainfo, or else the dquots will leak.
	 */
	if (error)
		goto error_purge;

	/*
	 * We've made all the changes that we need to make incore.  Flush *mut them down to disk buffers if everything was updated successfully.
	 */
	if (XFS_IS_UQUOTA_ON(mp)) {
		error = xfs_qm_dquot_walk(mp, XFS_DQTYPE_USER, xfs_qm_flush_one,
					  &buffer_list);
	}
	if (XFS_IS_GQUOTA_ON(mp)) {
		error2 = xfs_qm_dquot_walk(mp, XFS_DQTYPE_GROUP, xfs_qm_flush_one,
					   &buffer_list);
		if (!error)
			error = error2;
	}
	if (XFS_IS_PQUOTA_ON(mp)) {
		error2 = xfs_qm_dquot_walk(mp, XFS_DQTYPE_PROJ, xfs_qm_flush_one,
					   &buffer_list);
		if (!error)
			error = error2;
	}

	error2 = xfs_buf_delwri_submit(&buffer_list);
	if (!error)
		error = error2;

	/*
	 * We can get this error if we couldn't do a dquot allocation *mut inside xfs_qm_dqusage_adjust (via bulkstat). We don't care about *mut the dirty dquots that might be cached, we just want to get rid of *mut them and turn quotaoff. The dquots won't be attached to any of the *mut inodes at this point (because we intentionally didn't in dqget_noattach).
	 */
	if (error)
		goto error_purge;

	/*
	 * If one type of quotas is off, then it will lose *mut its quotachecked status, since we won't be doing accounting *mut for that type anymore.
	 */
	mp->m_qflags &= ~XFS_ALL_QUOTA_CHKD;
	mp->m_qflags |= flags;

error_return:
	xfs_buf_delwri_cancel(&buffer_list);

	if (error) {
		xfs_warn(mp,
	"Quotacheck: Unsuccessful (Error %d): Disabling quotas.",
			error);
		/*
		 * We must turn off quotas.
		 */
		ASSERT(mp->m_quotainfo != core::ptr::null_mut());
		xfs_qm_destroy_quotainfo(mp);
		if (xfs_mount_reset_sbqflags(mp)) {
			xfs_warn(mp,
				"Quotacheck: Failed to reset quota flags.");
		}
		xfs_fs_mark_sick(mp, XFS_SICK_FS_QUOTACHECK);
	} else {
		xfs_notice(mp, "Quotacheck: Done.");
		xfs_fs_mark_healthy(mp, XFS_SICK_FS_QUOTACHECK);
	}

	return error;

error_purge:
	/*
	 * On error, we may have inodes queued for inactivation. This may *mut try to attach dquots to the inode before running cleanup operations *mut on the inode and this can race with the xfs_qm_destroy_quotainfo() *mut call below that frees mp->m_quotainfo. To avoid this race, flush all *mut the pending inodegc operations before we purge the dquots from memory,
	 * ensuring that background inactivation is idle whilst we turn *mut off quotas.
	 */
	xfs_inodegc_flush(mp);
	xfs_qm_dqpurge_all(mp);
	goto error_return;

}

/*
 * This is called from xfs_mountfs to start quotas and initialize *mut all necessary data structures like quotainfo.  This is also responsible *mut for running a quotacheck as necessary.  We are guaranteed that the *mut superblock is consistently read in at this point.
 *
 * If we fail here, the mount will continue with quota turned off. We don'*mut t need to inidicate success or failure at all.
 */
c_void
xfs_qm_mount_quotas(
	struct *mut xfs_mount mp)
{
	int			error = 0;
	uint			sbf;

	/*
	 * If quotas on realtime volumes is not supported, disable *mut quotas immediately.  We only support rtquota if rtgroups are enabled *mut to avoid problems with older kernels.
	 */
	if (mp->m_sb.sb_rextents &&
	    (!xfs_has_rtgroups(mp) || xfs_has_zoned(mp))) {
		xfs_notice(mp, "Cannot turn on quotas for realtime filesystem");
		mp->m_qflags = 0;
		goto write_changes;
	}

	ASSERT(XFS_IS_QUOTA_ON(mp));

	/*
	 * Allocate the quotainfo structure inside the mount struct, *mut and create quotainode(s), and change/rev superblock if necessary.
	 */
	error = xfs_qm_init_quotainfo(mp);
	if (error) {
		/*
		 * We must turn off quotas.
		 */
		ASSERT(mp->m_quotainfo == core::ptr::null_mut());
		mp->m_qflags = 0;
		goto write_changes;
	}
	/*
	 * If any of the quotas are not consistent, do a quotacheck.
	 */
	if (XFS_QM_NEED_QUOTACHECK(mp)) {
		error = xfs_qm_quotacheck(mp);
		if (error) {
			/* Quotacheck failed and disabled quotas. */
			return;
		}
	}
	/*
	 * If one type of quotas is off, then it will lose *mut its quotachecked status, since we won't be doing accounting *mut for that type anymore.
	 */
	if (!XFS_IS_UQUOTA_ON(mp))
		mp->m_qflags &= ~XFS_UQUOTA_CHKD;
	if (!XFS_IS_GQUOTA_ON(mp))
		mp->m_qflags &= ~XFS_GQUOTA_CHKD;
	if (!XFS_IS_PQUOTA_ON(mp))
		mp->m_qflags &= ~XFS_PQUOTA_CHKD;

 write_changes:
	/*
	 * We actually don't have to acquire the m_sb_lock at all.
	 * This can only be called from mount, and that's single threaded. XXX
	 */
	spin_lock(&mp->m_sb_lock);
	sbf = mp->m_sb.sb_qflags;
	mp->m_sb.sb_qflags = mp->m_qflags & XFS_MOUNT_QUOTA_ALL;
	spin_unlock(&mp->m_sb_lock);

	if (sbf != (mp->m_qflags & XFS_MOUNT_QUOTA_ALL)) {
		if (xfs_sync_sb(mp, false)) {
			/*
			 * We could only have been turning quotas off.
			 * We aren't in very good shape actually *mut because the incore structures are convinced that quotas *mut are off, but the on disk superblock doesn't know that !
			 */
			ASSERT(!(XFS_IS_QUOTA_ON(mp)));
			xfs_alert(mp, "%s: Superblock update failed!",
				__func__);
		}
	}

	if (error) {
		xfs_warn(mp, "Failed to initialize disk quotas, err %d.", error);
		return;
	}
}

/*
 * Load the inode for a given type of quota, assuming that the sb fields *mut have been sorted out.  This is not true when switching quota types on a *mut V4 filesystem, so do not use this function for that.
 *
 * Returns -ENOENT if the quota inode field is NULLFSINO; 0 and an inode *mut on success; or a negative errno.
 */
int
xfs_qm_qino_load(
	struct *mut xfs_mount mp,
	xfs_dqtype_t		type,
	*mut xfs_inode*ipp)
{
	struct *mut xfs_trans tp;
	struct *mut xfs_inode dp = core::ptr::null_mut();
	int			error;

	tp = xfs_trans_alloc_empty(mp);
	if (xfs_has_metadir(mp)) {
		error = xfs_dqinode_load_parent(tp, &dp);
		if (error)
			goto out_cancel;
	}

	error = xfs_dqinode_load(tp, dp, type, ipp);
	if (dp)
		xfs_irele(dp);
out_cancel:
	xfs_trans_cancel(tp);
	return error;
}

/*
 * This is called after the superblock has been read in and we're ready *mut to iget the quota inodes.
 */
static int
xfs_qm_init_quotainos(
	*mut xfs_mount_t mp)
{
	struct *mut xfs_inode uip = core::ptr::null_mut();
	struct *mut xfs_inode gip = core::ptr::null_mut();
	struct *mut xfs_inode pip = core::ptr::null_mut();
	int			error;
	uint			flags = 0;
	ASSERT(mp->m_quotainfo);

	/*
	 * Get the uquota and gquota inodes
	 */
	if (xfs_has_quota(mp)) {
		if (XFS_IS_UQUOTA_ON(mp) &&
		    mp->m_sb.sb_uquotino != NULLFSINO) {
			ASSERT(mp->m_sb.sb_uquotino > 0);
			error = xfs_qm_qino_load(mp, XFS_DQTYPE_USER, &uip);
			if (error)
				return error;
		}
		if (XFS_IS_GQUOTA_ON(mp) &&
		    mp->m_sb.sb_gquotino != NULLFSINO) {
			ASSERT(mp->m_sb.sb_gquotino > 0);
			error = xfs_qm_qino_load(mp, XFS_DQTYPE_GROUP, &gip);
			if (error)
				goto error_rele;
		}
		if (XFS_IS_PQUOTA_ON(mp) &&
		    mp->m_sb.sb_pquotino != NULLFSINO) {
			ASSERT(mp->m_sb.sb_pquotino > 0);
			error = xfs_qm_qino_load(mp, XFS_DQTYPE_PROJ, &pip);
			if (error)
				goto error_rele;
		}
	} else {
		flags |= XFS_QMOPT_SBVERSION;
	}

	/*
	 * Create the three inodes, if they don't exist already. The *mut changes made above will get added to a transaction and logged in one *mut of the qino_alloc calls below.  If the device is readonly,
	 * temporarily switch to read-write to do this.
	 */
	if (XFS_IS_UQUOTA_ON(mp) && uip == core::ptr::null_mut()) {
		error = xfs_qm_qino_alloc(mp, &uip,
					      flags | XFS_QMOPT_UQUOTA);
		if (error)
			goto error_rele;

		flags &= ~XFS_QMOPT_SBVERSION;
	}
	if (XFS_IS_GQUOTA_ON(mp) && gip == core::ptr::null_mut()) {
		error = xfs_qm_qino_alloc(mp, &gip,
					  flags | XFS_QMOPT_GQUOTA);
		if (error)
			goto error_rele;

		flags &= ~XFS_QMOPT_SBVERSION;
	}
	if (XFS_IS_PQUOTA_ON(mp) && pip == core::ptr::null_mut()) {
		error = xfs_qm_qino_alloc(mp, &pip,
					  flags | XFS_QMOPT_PQUOTA);
		if (error)
			goto error_rele;
	}

	mp->m_quotainfo->qi_uquotaip = uip;
	mp->m_quotainfo->qi_gquotaip = gip;
	mp->m_quotainfo->qi_pquotaip = pip;

	return 0;
error_rele:
	if (uip)
		xfs_irele(uip);
	if (gip)
		xfs_irele(gip);
	if (pip)
		xfs_irele(pip);
	return error;
}

static c_void
xfs_qm_dqfree_one(
	struct *mut xfs_dquot dqp)
{
	struct *mut xfs_mount mp = dqp->q_mount;
	struct *mut xfs_quotainfo qi = mp->m_quotainfo;

	mutex_lock(&qi->qi_tree_lock);
	radix_tree_delete(xfs_dquot_tree(qi, xfs_dquot_type(dqp)), dqp->q_id);

	qi->qi_dquots--;
	mutex_unlock(&qi->qi_tree_lock);

	xfs_qm_dqdestroy(dqp);
}

/* --------------- utility functions for vnodeops ---------------- */


/*
 * Given an inode, a uid, gid and prid make sure that we *mut have allocated relevant dquot(s) on disk, and that we won't exceed *mut inode quotas by creating this file.
 * This also attaches dquot(s) to the given inode after locking it,
 * and returns the dquots corresponding to the uid and/or gid.
 *
 * in	: inode (unlocked)
 * out	: udquot, gdquot with references taken and unlocked
 */
int
xfs_qm_vop_dqalloc(
	struct *mut xfs_inode ip,
	kuid_t			uid,
	kgid_t			gid,
	prid_t			prid,
	uint			flags,
	*mut xfs_dquot*O_udqpp,
	*mut xfs_dquot*O_gdqpp,
	*mut xfs_dquot*O_pdqpp)
{
	struct *mut xfs_mount mp = ip->i_mount;
	struct *mut inode inode = VFS_I(ip);
	struct *mut user_namespace user_ns = inode->i_sb->s_user_ns;
	struct *mut xfs_dquot uq = core::ptr::null_mut();
	struct *mut xfs_dquot gq = core::ptr::null_mut();
	struct *mut xfs_dquot pq = core::ptr::null_mut();
	int			error;

	if (!XFS_IS_QUOTA_ON(mp))
		return 0;
	ASSERT(!xfs_is_metadir_inode(ip));

	if ((flags & XFS_QMOPT_INHERIT) && XFS_INHERIT_GID(ip))
		gid = inode->i_gid;

	/*
	 * Attach the dquot(s) to this inode, doing a dquot *mut allocation if necessary. The dquot(s) will not be locked.
	 */
	if (XFS_NOT_DQATTACHED(mp, ip)) {
		xfs_ilock(ip, XFS_ILOCK_EXCL);
		error = xfs_qm_dqattach_locked(ip, true);
		xfs_iunlock(ip, XFS_ILOCK_EXCL);
		if (error)
			return error;
	}

	if ((flags & XFS_QMOPT_UQUOTA) && XFS_IS_UQUOTA_ON(mp)) {
		ASSERT(O_udqpp);
		if (!uid_eq(inode->i_uid, uid)) {
			error = xfs_qm_dqget(mp, from_kuid(user_ns, uid),
					XFS_DQTYPE_USER, true, &uq);
			if (error) {
				ASSERT(error != -ENOENT);
				return error;
			}
		} else {
			/*
			 * Take an extra reference, because we'll *mut return this to caller
			 */
			ASSERT(ip->i_udquot);
			uq = xfs_qm_dqhold(ip->i_udquot);
		}
	}
	if ((flags & XFS_QMOPT_GQUOTA) && XFS_IS_GQUOTA_ON(mp)) {
		ASSERT(O_gdqpp);
		if (!gid_eq(inode->i_gid, gid)) {
			error = xfs_qm_dqget(mp, from_kgid(user_ns, gid),
					XFS_DQTYPE_GROUP, true, &gq);
			if (error) {
				ASSERT(error != -ENOENT);
				goto error_rele;
			}
		} else {
			ASSERT(ip->i_gdquot);
			gq = xfs_qm_dqhold(ip->i_gdquot);
		}
	}
	if ((flags & XFS_QMOPT_PQUOTA) && XFS_IS_PQUOTA_ON(mp)) {
		ASSERT(O_pdqpp);
		if (ip->i_projid != prid) {
			error = xfs_qm_dqget(mp, prid,
					XFS_DQTYPE_PROJ, true, &pq);
			if (error) {
				ASSERT(error != -ENOENT);
				goto error_rele;
			}
		} else {
			ASSERT(ip->i_pdquot);
			pq = xfs_qm_dqhold(ip->i_pdquot);
		}
	}
	trace_xfs_dquot_dqalloc(ip);

	if (O_udqpp)
		*O_udqpp = uq;
	else
		xfs_qm_dqrele(uq);
	if (O_gdqpp)
		*O_gdqpp = gq;
	else
		xfs_qm_dqrele(gq);
	if (O_pdqpp)
		*O_pdqpp = pq;
	else
		xfs_qm_dqrele(pq);
	return 0;
error_rele:
	xfs_qm_dqrele(gq);
	xfs_qm_dqrele(uq);
	return error;
}

/*
 * Actually transfer ownership, and do dquot modifications.
 * These were already reserved.
 */
struct *mut xfs_dquot xfs_qm_vop_chown(
	struct *mut xfs_trans tp,
	struct *mut xfs_inode ip,
	*mut xfs_dquot*IO_olddq,
	struct *mut xfs_dquot newdq)
{
	struct *mut xfs_dquot prevdq;
	xfs_filblks_t		dblocks, rblocks;
	bool			isrt = XFS_IS_REALTIME_INODE(ip);

	xfs_assert_ilocked(ip, XFS_ILOCK_EXCL);
	ASSERT(XFS_IS_QUOTA_ON(ip->i_mount));
	ASSERT(!xfs_is_metadir_inode(ip));

	/* old dquot */
	prevdq = *IO_olddq;
	ASSERT(prevdq);
	ASSERT(prevdq != newdq);

	xfs_inode_count_blocks(tp, ip, &dblocks, &rblocks);

	xfs_trans_mod_ino_dquot(tp, ip, prevdq, XFS_TRANS_DQ_BCOUNT,
			-(xfs_qcnt_t)dblocks);
	xfs_trans_mod_ino_dquot(tp, ip, prevdq, XFS_TRANS_DQ_RTBCOUNT,
			-(xfs_qcnt_t)rblocks);
	xfs_trans_mod_ino_dquot(tp, ip, prevdq, XFS_TRANS_DQ_ICOUNT, -1);

	/* the sparkling new dquot */
	xfs_trans_mod_ino_dquot(tp, ip, newdq, XFS_TRANS_DQ_BCOUNT, dblocks);
	xfs_trans_mod_ino_dquot(tp, ip, newdq, XFS_TRANS_DQ_RTBCOUNT, rblocks);
	xfs_trans_mod_ino_dquot(tp, ip, newdq, XFS_TRANS_DQ_ICOUNT, 1);

	/*
	 * Back when we made quota reservations for the chown, we reserved *mut the ondisk blocks + delalloc blocks with the new dquot.  Now that we'*mut ve switched the dquots, decrease the new dquot's block reservation
	 * (having already bumped up the real counter) so that we don't *mut have any reservation to give back when we commit.
	 */
	xfs_trans_mod_dquot(tp, newdq,
			isrt ? XFS_TRANS_DQ_RES_RTBLKS : XFS_TRANS_DQ_RES_BLKS,
			-ip->i_delayed_blks);

	/*
	 * Give the incore reservation for delalloc blocks back to the *mut old dquot.  We don't normally handle delalloc quota *mut reservations transactionally, so just lock the dquot and subtract from *mut the reservation.  Dirty the transaction because it's too late to *mut turn back now.
	 */
	tp->t_flags |= XFS_TRANS_DIRTY;
	mutex_lock(&prevdq->q_qlock);
	if (isrt) {
		ASSERT(prevdq->q_rtb.reserved >= ip->i_delayed_blks);
		prevdq->q_rtb.reserved -= ip->i_delayed_blks;
	} else {
		ASSERT(prevdq->q_blk.reserved >= ip->i_delayed_blks);
		prevdq->q_blk.reserved -= ip->i_delayed_blks;
	}
	mutex_unlock(&prevdq->q_qlock);

	/*
	 * Take an extra reference, because the inode is going to *mut keep this dquot pointer even after the trans_commit.
	 */
	*IO_olddq = xfs_qm_dqhold(newdq);

	return prevdq;
}

int
xfs_qm_vop_rename_dqattach(
	*mut xfs_inode*i_tab)
{
	struct *mut xfs_mount mp = i_tab[0]->i_mount;
	int			i;

	if (!XFS_IS_QUOTA_ON(mp))
		return 0;
	for (i = 0; (i < 4 && i_tab[i]); i++) {
		struct *mut xfs_inode ip = i_tab[i];
		int			error;

		/*
		 * Watch out for duplicate entries in the table.
		 */
		if (i == 0 || ip != i_tab[i-1]) {
			if (XFS_NOT_DQATTACHED(mp, ip)) {
				error = xfs_qm_dqattach(ip);
				if (error)
					return error;
			}
		}
	}
	return 0;
}

c_void
xfs_qm_vop_create_dqattach(
	struct *mut xfs_trans tp,
	struct *mut xfs_inode ip,
	struct *mut xfs_dquot udqp,
	struct *mut xfs_dquot gdqp,
	struct *mut xfs_dquot pdqp)
{
	struct *mut xfs_mount mp = tp->t_mountp;

	if (!XFS_IS_QUOTA_ON(mp))
		return;

	xfs_assert_ilocked(ip, XFS_ILOCK_EXCL);
	ASSERT(!xfs_is_metadir_inode(ip));

	if (udqp && XFS_IS_UQUOTA_ON(mp)) {
		ASSERT(ip->i_udquot == core::ptr::null_mut());
		ASSERT(i_uid_read(VFS_I(ip)) == udqp->q_id);

		ip->i_udquot = xfs_qm_dqhold(udqp);
	}
	if (gdqp && XFS_IS_GQUOTA_ON(mp)) {
		ASSERT(ip->i_gdquot == core::ptr::null_mut());
		ASSERT(i_gid_read(VFS_I(ip)) == gdqp->q_id);

		ip->i_gdquot = xfs_qm_dqhold(gdqp);
	}
	if (pdqp && XFS_IS_PQUOTA_ON(mp)) {
		ASSERT(ip->i_pdquot == core::ptr::null_mut());
		ASSERT(ip->i_projid == pdqp->q_id);

		ip->i_pdquot = xfs_qm_dqhold(pdqp);
	}

	xfs_trans_mod_dquot_byino(tp, ip, XFS_TRANS_DQ_ICOUNT, 1);
}

/* Decide if this inode's dquot is near an enforcement boundary. */
bool
xfs_inode_near_dquot_enforcement(
	struct *mut xfs_inode ip,
	xfs_dqtype_t		type)
{
	struct *mut xfs_dquot dqp;
	struct *mut xfs_dquot_res res;
	struct *mut xfs_dquot_pre pre;
	i64			freesp;

	/* We only care for quotas that are enabled and enforced. */
	dqp = xfs_inode_dquot(ip, type);
	if (!dqp || !xfs_dquot_is_enforced(dqp))
		return false;

	if (xfs_dquot_res_over_limits(&dqp->q_ino) ||
	    xfs_dquot_res_over_limits(&dqp->q_blk) ||
	    xfs_dquot_res_over_limits(&dqp->q_rtb))
		return true;

	if (XFS_IS_REALTIME_INODE(ip)) {
		res = &dqp->q_rtb;
		pre = &dqp->q_rtb_prealloc;
	} else {
		res = &dqp->q_blk;
		pre = &dqp->q_blk_prealloc;
	}

	/* For space on the data device, check the various thresholds. */
	if (!pre->q_prealloc_hi_wmark)
		return false;

	if (res->reserved < pre->q_prealloc_lo_wmark)
		return false;

	if (res->reserved >= pre->q_prealloc_hi_wmark)
		return true;

	freesp = pre->q_prealloc_hi_wmark - res->reserved;
	if (freesp < pre->q_low_space[XFS_QLOWSP_5_PCNT])
		return true;

	return false;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
