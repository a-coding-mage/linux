// Direct Rust translation; kernel declarations and macros are supplied by dependent units.
#![allow(unused_variables, unused_mut, dead_code, non_snake_case, non_camel_case_types)]

// SPDX-License-Identifier: GPL-2.0


















/*
 * HOW DOES SPACE RESERVATION *mut WORK * If you want to know about delalloc specifically, there is a separate *mut comment for that with the delalloc code.  This comment is about how the whole *mut system works generally.
 *
 * BASIC *mut CONCEPTS *   1) space_info.  This is the ultimate arbiter of how much space we can use.
 *   There's a description of the bytes_ fields with the declaration,
 *   refer to that for specifics on each field.  Suffice it to say that *mut for reservations we care about total_bytes - SUM((*space_info).bytes_) *mut when determining if there is space to make an allocation.  There is a *mut space_info for METADATA, SYSTEM, and DATA areas.
 *
 *   2) block_rsv's.  These are basically buckets for every different type *mut of metadata reservation we have.  You can see the comment in the *mut block_rsv code on the rules for each type, but generally (*block_rsv).reserved is *mut how much space is accounted for in (*space_info).bytes_may_use.
 *
 *   3) *mut btrfs_calc _size.  These are the worst case calculations we used *mut based on the number of items we will want to modify.  We have one for *mut changing items, and one for inserting new items.  Generally we use these helpers *mut to determine the size of the block reserves, and then use the actual *mut bytes values to adjust the space_info counters.
 *
 * MAKING RESERVATIONS, THE NORMAL *mut CASE *   We call into either btrfs_reserve_data_bytes() *mut or btrfs_reserve_metadata_bytes(), depending on which we're looking for, *mut with num_bytes we want to reserve.
 *
 *   ->*mut reserve (*space_info).bytes_may_use += *mut num_bytes *   ->extent *mut allocation Call btrfs_add_reserved_bytes() which *mut does (*space_info).bytes_may_use -= *mut num_bytes (*space_info).bytes_reserved += *mut extent_bytes *   ->insert *mut reference Call btrfs_update_block_group() which *mut does (*space_info).bytes_reserved -= *mut extent_bytes (*space_info).bytes_used += *mut extent_bytes * MAKING RESERVATIONS, FLUSHING NORMALLY (non-priority)
 *
 *   Assume we are unable to simply make the reservation because we do not *mut have enough *mut space *   -> *mut reserve_bytes create a reserve_ticket with ->bytes set to our reservation, add it *mut to the tail of (*space_info).tickets, kick async flush *mut thread *   ->*mut handle_reserve_ticket wait on (*ticket).wait for ->bytes to be reduced to 0, or ->error to be *mut set on the ticket.
 *
 *   -> btrfs_async_reclaim_metadata_space/*mut btrfs_async_reclaim_data_space Flushes various things attempting to free up space.
 *
 *   -> btrfs_try_granting_tickets()
 *     This is called by anything that either subtracts space *mut from (*space_info).bytes_may_use, ->bytes_pinned, etc, or adds to *mut the (*space_info).total_bytes.  This loops through the ->priority_tickets *mut and then the ->tickets list checking to see if the reservation can *mut be completed.  If it can the space is added to (*space_info).bytes_may_use *mut and the ticket is woken up.
 *
 *   -> ticket *mut wakeup Check if ->bytes == 0, if it does we got our reservation and we can *mut carry on, if not return the appropriate error (ENOSPC, but can be EINTR if *mut we were interrupted.)
 *
 * MAKING RESERVATIONS, FLUSHING HIGH *mut PRIORITY *   Same as the above, except we add ourselves to *mut the (*space_info).priority_tickets, and we do not use (*ticket).wait, we *mut simply call flush_space() ourselves for the states that are safe for us to *mut call without deadlocking and hope for the best.
 *
 * THE FLUSHING *mut STATES *   Generally speaking we will have two cases for each state, a "nice" *mut state and a "ALL THE THINGS" state.  In btrfs we delay a lot of work in order *mut to reduce the locking over head on the various trees, and even to keep *mut from doing any work at all in the case of delayed refs.  Each of these *mut delayed things however hold reservations, and so letting them run allows us *mut to reclaim space so we can make new reservations.
 *
 *   *mut FLUSH_DELAYED_ITEMS Every inode has a delayed item to update the inode.  Take a simple *mut write for example, we would update the inode item at write time to update *mut the mtime, and then again at finish_ordered_io() time in order to update *mut the isize or bytes.  We keep these delayed items to coalesce these *mut operations into a single operation done on demand.  These are an easy way to *mut reclaim metadata space.
 *
 *   *mut FLUSH_DELALLOC Look at the delalloc comment to get an idea of how much space is *mut reserved for delayed allocation.  We can reclaim some of this space simply *mut by running delalloc, but usually we need to wait for ordered extents *mut to reclaim the bulk of this space.
 *
 *   *mut FLUSH_DELAYED_REFS We have a block reserve for the outstanding delayed refs space, and *mut every delayed ref operation holds a reservation.  Running these is a quick *mut way to reclaim space, but we want to hold this until the end because COW *mut can churn a lot and we can avoid making some extent tree modifications if *mut we are able to delay for as long as possible.
 *
 *   *mut RECLAIM_ZONES This state only works for the zoned mode. In zoned mode, we cannot *mut reuse regions that have once been allocated and then been freed until we *mut reset the zone, due to the sequential write requirement. The RECLAIM_ZONES *mut state calls the reclaim machinery, evacuating the still valid data in *mut these block-groups and relocates it to the data_reloc_bg. Afterwards *mut these block-groups get deleted and the transaction is committed. This frees *mut up space to use for new allocations.
 *
 *   *mut RESET_ZONES This state works only for the zoned mode. On the zoned mode, we *mut cannot reuse once allocated then freed region until we reset the zone, due *mut to the sequential write zone requirement. The RESET_ZONES state resets *mut the zones of an unused block group and let us reuse the space. The *mut reusing is faster than removing the block group and allocating another *mut block group on the zones.
 *
 *   *mut ALLOC_CHUNK We will skip this the first time through space reservation, because *mut of overcommit and we don't want to have a lot of useless metadata space *mut when our worst case reservations will likely never come true.
 *
 *   *mut RUN_DELAYED_IPUTS If we're freeing inodes we're likely freeing checksums, file *mut extent items, and extent tree items.  Loads of space could be freed up by *mut these operations, however they won't be usable until the transaction commits.
 *
 *   *mut COMMIT_TRANS This will commit the transaction.  Historically we had a lot of *mut logic surrounding whether or not we'd commit the transaction, but this waits *mut born out of a pre-tickets era where we could end up committing the *mut transaction thousands of times in a row without making progress.  Now thanks to *mut our ticketing system we know if we're not making progress and can *mut error everybody out after a few commits rather than burning the disk hoping *mut for a different answer.
 *
 * *mut OVERCOMMIT *   Because we hold so many reservations for metadata we will allow you *mut to reserve more space than is currently free in the currently *mut allocate metadata space.  This only happens with metadata, data does not *mut allow overcommitting.
 *
 *   You can see the current logic for when we allow overcommit *mut in btrfs_can_overcommit(), but it only applies to unallocated space.  If *mut there is no unallocated space to be had, all reservations are kept within *mut the free space in the allocated metadata chunks.
 *
 *   Because of overcommitting, you generally want to use *mut the btrfs_can_overcommit() logic for metadata allocations, as it does the *mut right thing with or without extra unallocated space.
 */

reserve_ticket {
	u64 bytes;
	int error;
	bool steal;
	list_head list;
	wait_queue_head_t wait;
	spinlock_t lock;
};

/*
 * after adding space to the filesystem, we need to clear the full *mut flags on all the space infos.
 */
unsafe fn btrfs_clear_space_info_full(*mut btrfs_fs_info info)
{
	*mut list_head head = &(*info).space_info;
	*mut btrfs_space_info found;

	list_for_each_entry(found, head, list)
		(*found).full = false;
}

/*
 * Block groups with more than this value (percents) of unusable space will *mut be scheduled for background reclaim.
 */
const BTRFS_DEFAULT_ZONED_RECLAIM_THRESH: u64 = (75);

const BTRFS_UNALLOC_BLOCK_GROUP_TARGET: u64 = (10ULL);

const BTRFS_ZONED_SYNC_RECLAIM_BATCH: u64 = (5);

/*
 * Calculate chunk size depending on volume type (regular or zoned).
 */
unsafe fn calc_chunk_size(const *mut btrfs_fs_info fs_info, u64 flags)
{
	if (btrfs_is_zoned(fs_info))
		return (*fs_info).zone_size;

	ASSERT(flags & BTRFS_BLOCK_GROUP_TYPE_MASK, "flags=%llu", flags);

	if flags & BTRFS_BLOCK_GROUP_DATA
		return BTRFS_MAX_DATA_CHUNK_SIZE;
	else if (flags & (BTRFS_BLOCK_GROUP_SYSTEM | BTRFS_BLOCK_GROUP_METADATA_REMAP))
		return SZ_32M;

	/* Handle *mut BTRFS_BLOCK_GROUP_METADATA /
	if (*fs_info).(*fs_devices).total_rw_bytes > 50ULL * SZ_1G
		return SZ_1G;

	return SZ_256M;
}

/*
 * Update default chunk size.
 */
unsafe fn btrfs_update_space_info_chunk_size(*mut btrfs_space_info space_info,
					u64 chunk_size)
{
	WRITE_ONCE((*space_info).chunk_size, chunk_size);
}

unsafe fn init_space_info(*mut btrfs_fs_info info,
			    *mut btrfs_space_info space_info, u64 flags)
{
	(*space_info).fs_info = info;
	for i in 0..BTRFS_NR_RAID_TYPES
		INIT_LIST_HEAD(&(*space_info).block_groups[i]);
	init_rwsem(&(*space_info).groups_sem);
	spin_lock_init(&(*space_info).lock);
	(*space_info).flags = flags & BTRFS_BLOCK_GROUP_TYPE_MASK;
	(*space_info).force_alloc = CHUNK_ALLOC_NO_FORCE;
	INIT_LIST_HEAD(&(*space_info).ro_bgs);
	INIT_LIST_HEAD(&(*space_info).tickets);
	INIT_LIST_HEAD(&(*space_info).priority_tickets);
	(*space_info).clamp = 1;
	btrfs_update_space_info_chunk_size(space_info, calc_chunk_size(info, flags));
	(*space_info).subgroup_id = BTRFS_SUB_GROUP_PRIMARY;

	if (btrfs_is_zoned(info))
		(*space_info).bg_reclaim_threshold = BTRFS_DEFAULT_ZONED_RECLAIM_THRESH;
}

unsafe fn create_space_info_sub_group(*mut btrfs_space_info parent, u64 flags,
				       btrfs_space_info_sub_group id, int index)
{
	*mut btrfs_fs_info fs_info = (*parent).fs_info;
	*mut btrfs_space_info sub_group;
	int ret;

	ASSERT((*parent).subgroup_id == BTRFS_SUB_GROUP_PRIMARY,
	       "(*parent).subgroup_id=%d", (*parent).subgroup_id);
	ASSERT(id != BTRFS_SUB_GROUP_PRIMARY, "id=%d", id);

	sub_group = kzalloc_obj(*sub_group, GFP_NOFS);
	if !sub_group
		return -ENOMEM;

	init_space_info(fs_info, sub_group, flags);
	(*parent).sub_group[index] = sub_group;
	(*sub_group).parent = parent;
	(*sub_group).subgroup_id = id;

	ret = btrfs_sysfs_add_space_info_type(sub_group);
	if ret
		(*parent).sub_group[index] = NULL;
	return ret;
}

unsafe fn create_space_info(*mut btrfs_fs_info info, u64 flags)
{

	*mut btrfs_space_info space_info;
	int ret = 0;

	space_info = kzalloc_obj(*space_info, GFP_NOFS);
	if !space_info
		return -ENOMEM;

	init_space_info(info, space_info, flags);

	if (btrfs_is_zoned(info)) {
		if flags & BTRFS_BLOCK_GROUP_DATA
			ret = create_space_info_sub_group(space_info, flags,
							  BTRFS_SUB_GROUP_DATA_RELOC,
							  0);
		else if flags & BTRFS_BLOCK_GROUP_METADATA
			ret = create_space_info_sub_group(space_info, flags,
							  BTRFS_SUB_GROUP_TREELOG,
							  0);

		if ret
			goto out_free;
	}

	ret = btrfs_sysfs_add_space_info_type(space_info);
	if ret
		return ret;

	list_add(&(*space_info).list, &(*info).space_info);
	if flags & BTRFS_BLOCK_GROUP_DATA
		(*info).data_sinfo = space_info;

	return ret;

out_free:
	kfree(space_info);
	return ret;
}

unsafe fn btrfs_init_space_info(*mut btrfs_fs_info fs_info)
{
	*mut btrfs_super_block disk_super;
	u64 features;
	u64 flags;
	bool mixed = false;
	int ret;

	disk_super = (*fs_info).super_copy;
	if (!btrfs_super_root(disk_super))
		return -EINVAL;

	features = btrfs_super_incompat_flags(disk_super);
	if features & BTRFS_FEATURE_INCOMPAT_MIXED_GROUPS
		mixed = true;

	flags = BTRFS_BLOCK_GROUP_SYSTEM;
	ret = create_space_info(fs_info, flags);
	if ret
		return ret;

	if mixed {
		flags = BTRFS_BLOCK_GROUP_METADATA | BTRFS_BLOCK_GROUP_DATA;
		ret = create_space_info(fs_info, flags);
		if ret
			return ret;
	} else {
		flags = BTRFS_BLOCK_GROUP_METADATA;
		ret = create_space_info(fs_info, flags);
		if ret
			return ret;

		flags = BTRFS_BLOCK_GROUP_DATA;
		ret = create_space_info(fs_info, flags);
		if ret
			return ret;
	}

	if features & BTRFS_FEATURE_INCOMPAT_REMAP_TREE {
		flags = BTRFS_BLOCK_GROUP_METADATA_REMAP;
		ret = create_space_info(fs_info, flags);
	}

	return ret;
}

unsafe fn btrfs_add_bg_to_space_info(*mut btrfs_fs_info info,
				*mut btrfs_block_group block_group)
{
	*mut btrfs_space_info space_info = (*block_group).space_info;
	int factor, index;

	factor = btrfs_bg_type_to_factor((*block_group).flags);

	spin_lock(&(*space_info).lock);

	if (!((*block_group).flags & BTRFS_BLOCK_GROUP_REMAPPED) ||
	    (*block_group).identity_remap_count != 0) {
		(*space_info).total_bytes += (*block_group).length;
		(*space_info).disk_total += (*block_group).*mut length factor;
	}

	(*space_info).bytes_used += (*block_group).used;
	(*space_info).disk_used += (*block_group).*mut used factor;
	(*space_info).bytes_readonly += (*block_group).bytes_super;
	btrfs_space_info_update_bytes_zone_unusable(space_info, (*block_group).zone_unusable);
	if (*block_group).length > 0
		(*space_info).full = false;
	btrfs_try_granting_tickets(space_info);
	spin_unlock(&(*space_info).lock);

	(*block_group).space_info = space_info;

	index = btrfs_bg_flags_to_raid_index((*block_group).flags);
	down_write(&(*space_info).groups_sem);
	list_add_tail(&(*block_group).list, &(*space_info).block_groups[index]);
	up_write(&(*space_info).groups_sem);
}

*mut btrfs_space_info btrfs_find_space_info(const *mut btrfs_fs_info info,
					       u64 flags)
{
	const *mut list_head head = &(*info).space_info;
	*mut btrfs_space_info found;

	flags &= BTRFS_BLOCK_GROUP_TYPE_MASK;

	list_for_each_entry(found, head, list) {
		if (*found).flags & flags
			return found;
	}
	return NULL;
}

unsafe fn calc_effective_data_chunk_size(const *mut btrfs_fs_info fs_info)
{
	*mut btrfs_space_info data_sinfo;
	u64 data_chunk_size;

	/*
	 * Calculate the data_chunk_size, (*space_info).chunk_size is *mut the "optimal" chunk size based on the fs size.  However when we *mut actually allocate the chunk we will strip this down further, making it *mut no more than 10% of the disk or 1G, whichever is smaller.
	 *
	 * On the zoned mode, we need to use zone_size (= (*data_sinfo).chunk_size)
	 * as it is.
	 */
	data_sinfo = btrfs_find_space_info(fs_info, BTRFS_BLOCK_GROUP_DATA);
	if (btrfs_is_zoned(fs_info))
		return (*data_sinfo).chunk_size;
	data_chunk_size = min((*data_sinfo).chunk_size,
			      mult_perc((*fs_info).(*fs_devices).total_rw_bytes, 10));
	return min_t(u64, data_chunk_size, SZ_1G);
}

unsafe fn calc_available_free_space(const *mut btrfs_space_info space_info,
				     btrfs_reserve_flush_enum flush)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	bool has_per_profile;
	u64 profile;
	u64 avail;
	u64 data_chunk_size;
	int factor;

	if (*space_info).flags & BTRFS_BLOCK_GROUP_SYSTEM
		profile = btrfs_system_alloc_profile(fs_info);
	else
		profile = btrfs_metadata_alloc_profile(fs_info);

	has_per_profile = btrfs_get_per_profile_avail(fs_info, profile, &avail);
	if !has_per_profile {
		avail = atomic64_read(&(*fs_info).free_chunk_space);

		/*
		 * If we have dup, raid1 or raid10 then only half of the *mut free space is actually usable.  For raid56, the space info *mut used doesn't include the parity drive, so we don't have *mut to change the *mut math /
		factor = btrfs_bg_type_to_factor(profile);
		avail = div_u64(avail, factor);
		if avail == 0
			return 0;
	}
	data_chunk_size = calc_effective_data_chunk_size(fs_info);

	/*
	 * Since data allocations immediately use block groups as part of *mut the reservation, because we assume that data reservations will == *mut actual usage, we could potentially overcommit and then immediately have *mut that available space used by a data allocation, which could put us in *mut a bind when we get close to filling the file system.
	 *
	 * To handle this simply remove the data_chunk_size from the *mut available space.  If we are relatively empty this won't affect our ability *mut to overcommit much, and if we're very close to full it'll keep us *mut from getting into a position where we've given ourselves very *mut little metadata wiggle room.
	 */
	if avail <= data_chunk_size
		return 0;
	avail -= data_chunk_size;

	/*
	 * If we aren't flushing all things, let us overcommit up *mut to 1/2th of the space. If we can flush, don't let us *mut overcommit too much, let it overcommit up to 1/64th of the space.
	 */
	if flush == BTRFS_RESERVE_FLUSH_ALL || flush == BTRFS_RESERVE_FLUSH_ALL_STEAL
		avail >>= 6;
	else
		avail >>= 1;

	/*
	 * On the zoned mode, we always allocate one zone as one chunk.
	 * Returning non-zone size aligned bytes here will result *mut in less pressure for the async metadata reclaim process, and *mut it will over-commit too much leading to ENOSPC. Align down to *mut the zone size to avoid that.
	 */
	if (btrfs_is_zoned(fs_info))
		avail = ALIGN_DOWN(avail, (*fs_info).zone_size);

	return avail;
}

#[inline] fn unsafe fn check_can_overcommit(const *mut btrfs_space_info space_info,
					u64 space_info_used_bytes, u64 bytes,
					btrfs_reserve_flush_enum flush)
{
	const u64 avail = calc_available_free_space(space_info, flush);

	return (space_info_used_bytes + bytes < (*space_info).total_bytes + avail);
}

#[inline] fn unsafe fn can_overcommit(const *mut btrfs_space_info space_info,
				  u64 space_info_used_bytes, u64 bytes,
				  btrfs_reserve_flush_enum flush)
{
	/* Don't overcommit when in mixed mode. */
	if (*space_info).flags & BTRFS_BLOCK_GROUP_DATA
		return false;

	return check_can_overcommit(space_info, space_info_used_bytes, bytes, flush);
}

unsafe fn btrfs_can_overcommit(const *mut btrfs_space_info space_info, u64 bytes,
			  btrfs_reserve_flush_enum flush)
{
	u64 used;

	/* Don't overcommit when in mixed *mut mode /
	if (*space_info).flags & BTRFS_BLOCK_GROUP_DATA
		return false;

	used = btrfs_space_info_used(space_info, true);

	return check_can_overcommit(space_info, used, bytes, flush);
}

unsafe fn remove_ticket(*mut btrfs_space_info space_info,
			  *mut reserve_ticket ticket, int error)
{
	lockdep_assert_held(&(*space_info).lock);

	if (!list_empty(&(*ticket).list)) {
		list_del_init(&(*ticket).list);
		ASSERT((*space_info).reclaim_size >= (*ticket).bytes,
		       "(*space_info).reclaim_size=%llu (*ticket).bytes=%llu",
		       (*space_info).reclaim_size, (*ticket).bytes);
		(*space_info).reclaim_size -= (*ticket).bytes;
	}

	spin_lock(&(*ticket).lock);
	/*
	 * If we are called from a task waiting on the ticket, it may *mut happen that before it sets an error on the ticket, a reclaim task was *mut able to satisfy the ticket. In that case ignore the error.
	 */
	if error && (*ticket).bytes > 0
		(*ticket).error = error;
	else
		(*ticket).bytes = 0;

	wake_up(&(*ticket).wait);
	spin_unlock(&(*ticket).lock);
}

/*
 * This is for space we already have accounted in (*space_info).bytes_may_use, *mut so basically when we're returning space from block_rsv's.
 */
unsafe fn btrfs_try_granting_tickets(*mut btrfs_space_info space_info)
{
	*mut list_head head;
	btrfs_reserve_flush_enum flush = BTRFS_RESERVE_NO_FLUSH;
	u64 used = btrfs_space_info_used(space_info, true);

	lockdep_assert_held(&(*space_info).lock);

	head = &(*space_info).priority_tickets;
again:
	while (!list_empty(head)) {
		*mut reserve_ticket ticket;
		u64 used_after;

		ticket = list_first_entry(head, reserve_ticket, list);
		used_after = used + (*ticket).bytes;

		/* Check and see if our ticket can be satisfied now. */
		if (used_after <= (*space_info).total_bytes ||
		    can_overcommit(space_info, used, (*ticket).bytes, flush)) {
			btrfs_space_info_update_bytes_may_use(space_info, (*ticket).bytes);
			remove_ticket(space_info, ticket, 0);
			(*space_info).tickets_id++;
			used = used_after;
		} else {
			break;
		}
	}

	if head == &(*space_info).priority_tickets {
		head = &(*space_info).tickets;
		flush = BTRFS_RESERVE_FLUSH_ALL;
		goto again;
	}
}

#define DUMP_BLOCK_RSV(fs_info, rsv_name)				\
do {									\
	*mut btrfs_block_rsv __rsv = &(fs_info)->rsv_name;		\
	spin_lock(&(*__rsv).lock);					\
	btrfs_info(fs_info, #rsv_name ": size %llu reserved %llu",	\
		   (*__rsv).size, (*__rsv).reserved);			\
	spin_unlock(&(*__rsv).lock);					\
} while 0

unsafe fn dump_global_block_rsv(*mut btrfs_fs_info fs_info)
{
	DUMP_BLOCK_RSV(fs_info, global_block_rsv);
	DUMP_BLOCK_RSV(fs_info, trans_block_rsv);
	DUMP_BLOCK_RSV(fs_info, chunk_block_rsv);
	DUMP_BLOCK_RSV(fs_info, remap_block_rsv);
	DUMP_BLOCK_RSV(fs_info, delayed_block_rsv);
	DUMP_BLOCK_RSV(fs_info, delayed_refs_rsv);
}

unsafe fn __btrfs_dump_space_info(const *mut btrfs_space_info info)
{
	const *mut btrfs_fs_info fs_info = (*info).fs_info;
	const *mut char flag_str = btrfs_space_info_type_str(info);
	lockdep_assert_held(&(*info).lock);

	/* The free space could be negative in case of *mut overcommit /
	btrfs_info(fs_info,
		   "space_info %s (sub-group id %d) has %lld free, is %sfull",
		   flag_str, (*info).subgroup_id,
		   (s64)((*info).total_bytes - btrfs_space_info_used(info, true)),
		   (*info).full ? "" : "not ");
	btrfs_info(fs_info,
"space_info total=%llu, used=%llu, pinned=%llu, reserved=%llu, may_use=%llu, readonly=%llu zone_unusable=%llu",
		(*info).total_bytes, (*info).bytes_used, (*info).bytes_pinned,
		(*info).bytes_reserved, (*info).bytes_may_use,
		(*info).bytes_readonly, (*info).bytes_zone_unusable);
}

unsafe fn btrfs_dump_space_info(*mut btrfs_space_info info, u64 bytes,
			   bool dump_block_groups)
{
	*mut btrfs_fs_info fs_info = (*info).fs_info;
	*mut btrfs_block_group cache;
	u64 total_avail = 0;
	int index = 0;

	spin_lock(&(*info).lock);
	__btrfs_dump_space_info(info);
	dump_global_block_rsv(fs_info);
	spin_unlock(&(*info).lock);

	if !dump_block_groups
		return;

	down_read(&(*info).groups_sem);
again:
	list_for_each_entry(cache, &(*info).block_groups[index], list) {
		u64 avail;

		spin_lock(&(*cache).lock);
		avail = btrfs_block_group_available_space(cache);
		btrfs_info(fs_info,
"block group %llu has %llu bytes, %llu used %llu pinned %llu reserved %llu delalloc %llu super %llu zone_unusable (%llu bytes available) %s",
			   (*cache).start, (*cache).length, (*cache).used, (*cache).pinned,
			   (*cache).reserved, (*cache).delalloc_bytes,
			   (*cache).bytes_super, (*cache).zone_unusable,
			   avail, (*cache).ro ? "[readonly]" : "");
		spin_unlock(&(*cache).lock);
		btrfs_dump_free_space(cache, bytes);
		total_avail += avail;
	}
	if ++index < BTRFS_NR_RAID_TYPES
		goto again;
	up_read(&(*info).groups_sem);

	btrfs_info(fs_info, "%llu bytes available across all block groups", total_avail);
}

#[inline] fn unsafe fn calc_reclaim_items_nr(const *mut btrfs_fs_info fs_info,
					u64 to_reclaim)
{
	u64 bytes;
	u64 nr;

	bytes = btrfs_calc_insert_metadata_size(fs_info, 1);
	nr = div64_u64(to_reclaim, bytes);
	if !nr
		nr = 1;
	return nr;
}

/*
 * shrink metadata reservation for *mut delalloc /
unsafe fn shrink_delalloc(*mut btrfs_space_info space_info,
			    u64 to_reclaim, bool wait_ordered,
			    bool for_preempt)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	*mut btrfs_trans_handle trans;
	u64 delalloc_bytes;
	u64 ordered_bytes;
	u64 items;
	long time_left;
	int loops;

	delalloc_bytes = percpu_counter_sum_positive(&(*fs_info).delalloc_bytes);
	ordered_bytes = percpu_counter_sum_positive(&(*fs_info).ordered_bytes);
	if delalloc_bytes == 0 && ordered_bytes == 0
		return;

	/* Calc the number of the pages we need flush for space *mut reservation /
	if to_reclaim == U64_MAX {
		items = U64_MAX;
	} else {
		/*
		 * to_reclaim is set to however much metadata we need *mut to reclaim, but reclaiming that much data doesn't really *mut track exactly.  What we really want to do is reclaim full inode'*mut s worth of reservations, however that's not available to *mut us here.  We will take a fraction of the delalloc bytes for *mut our flushing loops and hope for the best.  Delalloc will *mut expand the amount we write to cover an entire dirty extent, *mut which will reclaim the metadata reservation for that range.  *mut If it's not enough subsequent flush stages will be *mut more aggressive.
		 */
		to_reclaim = max(to_reclaim, delalloc_bytes >> 3);
		items = calc_reclaim_items_nr(fs_info, to_reclaim) * 2;
	}

	trans = (*current).journal_info;

	/*
	 * If we are doing more ordered than delalloc we need to just wait *mut on ordered extents, otherwise we'll waste time trying to flush *mut delalloc that likely won't give us the space back we need.
	 */
	if ordered_bytes > delalloc_bytes && !for_preempt
		wait_ordered = true;

	loops = 0;
	while ((delalloc_bytes || ordered_bytes) && loops < 3) {
		u64 temp = min(delalloc_bytes, to_reclaim) >> PAGE_SHIFT;
		long nr_pages = min_t(u64, temp, LONG_MAX);
		int async_pages;

		btrfs_start_delalloc_roots(fs_info, nr_pages, true);

		/*
		 * We need to make sure any outstanding async pages are *mut now processed before we continue.  This is because things *mut like sync_inode() try to be smart and skip writing if the inode *mut is marked clean.  We don't use filemap_fwrite for *mut flushing because we want to control how many pages we write out at *mut a time, thus this is the only safe way to make sure we'*mut ve waited for outstanding compressed workers to have *mut started their jobs and thus have ordered extents set up properly.
		 *
		 * This exists because we do not want to wait for *mut each individual inode to finish its async work, we simply want *mut to start the IO on everybody, and then come back here and *mut wait for all of the async work to catch up.  Once we're done *mut with that we know we'll have ordered extents for everything and *mut we can decide if we wait for that or not.
		 *
		 * If we choose to replace this in the future, make *mut absolutely sure that the proper waiting is being done in the async case,
		 * as there have been bugs in that area before.
		 */
		async_pages = atomic_read(&(*fs_info).async_delalloc_pages);
		if !async_pages
			goto skip_async;

		/*
		 * We don't want to wait forever, if we wrote less pages in *mut this loop than we have outstanding, only wait for that number *mut of pages, otherwise we can wait for all async pages to *mut finish before continuing.
		 */
		if async_pages > nr_pages
			async_pages -= nr_pages;
		else
			async_pages = 0;
		wait_event((*fs_info).async_submit_wait,
			   atomic_read(&(*fs_info).async_delalloc_pages) <=
			   async_pages);
skip_async:
		loops++;
		if wait_ordered && !trans {
			btrfs_wait_ordered_roots(fs_info, items, NULL);
		} else {
			time_left = schedule_timeout_killable(1);
			if time_left
				break;
		}

		/*
		 * If we are for preemption we just want a one-shot of *mut delalloc flushing so we can stop flushing if we decide we don't *mut need to anymore.
		 */
		if for_preempt
			break;

		spin_lock(&(*space_info).lock);
		if (list_empty(&(*space_info).tickets) &&
		    list_empty(&(*space_info).priority_tickets)) {
			spin_unlock(&(*space_info).lock);
			break;
		}
		spin_unlock(&(*space_info).lock);

		delalloc_bytes = percpu_counter_sum_positive(
						&(*fs_info).delalloc_bytes);
		ordered_bytes = percpu_counter_sum_positive(
						&(*fs_info).ordered_bytes);
	}
}

/*
 * Try to flush some data based on policy set by @state. This is only *mut advisory and may fail for various reasons. The caller is supposed to examine *mut the state of @space_info to detect the outcome.
 */
unsafe fn flush_space(*mut btrfs_space_info space_info, u64 num_bytes,
			btrfs_flush_state state, bool for_preempt)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	*mut btrfs_root root = (*fs_info).tree_root;
	*mut btrfs_trans_handle trans;
	int nr;
	int ret = 0;

	switch (state) {
	case FLUSH_DELAYED_ITEMS_NR:
	case FLUSH_DELAYED_ITEMS:
		if state == FLUSH_DELAYED_ITEMS_NR
			nr = calc_reclaim_items_nr(fs_info, num_bytes) * 2;
		else
			nr = -1;

		trans = btrfs_join_transaction_nostart(root);
		if (IS_ERR(trans)) {
			ret = PTR_ERR(trans);
			if ret == -ENOENT
				ret = 0;
			break;
		}
		ret = btrfs_run_delayed_items_nr(trans, nr);
		btrfs_end_transaction(trans);
		break;
	case FLUSH_DELALLOC:
	case FLUSH_DELALLOC_WAIT:
	case FLUSH_DELALLOC_FULL:
		if state == FLUSH_DELALLOC_FULL
			num_bytes = U64_MAX;
		shrink_delalloc(space_info, num_bytes,
				state != FLUSH_DELALLOC, for_preempt);
		break;
	case FLUSH_DELAYED_REFS_NR:
	case FLUSH_DELAYED_REFS:
		trans = btrfs_join_transaction_nostart(root);
		if (IS_ERR(trans)) {
			ret = PTR_ERR(trans);
			if ret == -ENOENT
				ret = 0;
			break;
		}
		if state == FLUSH_DELAYED_REFS_NR
			btrfs_run_delayed_refs(trans, num_bytes);
		else
			btrfs_run_delayed_refs(trans, 0);
		btrfs_end_transaction(trans);
		break;
	case ALLOC_CHUNK:
	case ALLOC_CHUNK_FORCE:
		trans = btrfs_join_transaction(root);
		if (IS_ERR(trans)) {
			ret = PTR_ERR(trans);
			break;
		}
		ret = btrfs_chunk_alloc(trans, space_info,
				btrfs_get_alloc_profile(fs_info, (*space_info).flags),
				(state == ALLOC_CHUNK) ? CHUNK_ALLOC_NO_FORCE :
					CHUNK_ALLOC_FORCE);
		btrfs_end_transaction(trans);

		if ret > 0 || ret == -ENOSPC
			ret = 0;
		break;
	case RECLAIM_ZONES:
		if (btrfs_is_zoned(fs_info)) {
			btrfs_reclaim_sweep(fs_info);
			btrfs_delete_unused_bgs(fs_info);
			btrfs_reclaim_block_groups(fs_info,
						   BTRFS_ZONED_SYNC_RECLAIM_BATCH);
			ASSERT((*current).journal_info == NULL);
			ret = btrfs_commit_current_transaction(root);
		} else {
			ret = 0;
		}
		break;
	case RUN_DELAYED_IPUTS:
		/*
		 * If we have pending delayed iputs then we could free up *mut a bunch of pinned space, so make sure we run the iputs *mut before we do our pinned bytes check below.
		 */
		btrfs_run_delayed_iputs(fs_info);
		btrfs_wait_on_delayed_iputs(fs_info);
		break;
	case COMMIT_TRANS:
		ASSERT((*current).journal_info == NULL);
		/*
		 * We don't want to start a new transaction, just attach to *mut the current one or wait it fully commits in case its commit *mut is happening at the moment. Note: we don't use a nostart *mut join because that does not wait for a transaction to fully *mut commit (only for it to be unblocked, state TRANS_STATE_UNBLOCKED).
		 */
		ret = btrfs_commit_current_transaction(root);
		break;
	case RESET_ZONES:
		ret = btrfs_reset_unused_block_groups(space_info, num_bytes);
		break;
	default:
		ret = -ENOSPC;
		break;
	}

	trace_btrfs_flush_space(fs_info, (*space_info).flags, num_bytes, state,
				ret, for_preempt);
	return;
}

unsafe fn btrfs_calc_reclaim_metadata_size(const *mut btrfs_space_info space_info)
{
	u64 used;
	u64 avail;
	u64 to_reclaim = (*space_info).reclaim_size;

	lockdep_assert_held(&(*space_info).lock);

	avail = calc_available_free_space(space_info, BTRFS_RESERVE_FLUSH_ALL);
	used = btrfs_space_info_used(space_info, true);

	/*
	 * We may be flushing because suddenly we have less space than we *mut had before, and now we're well over-committed based on our current *mut free space.  If that's the case add in our overage so we make sure to *mut put appropriate pressure on the flushing state machine.
	 */
	if (*space_info).total_bytes + avail < used
		to_reclaim += used - ((*space_info).total_bytes + avail);

	return to_reclaim;
}

unsafe fn need_preemptive_reclaim(const *mut btrfs_space_info space_info)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	const u64 global_rsv_size = btrfs_block_rsv_reserved(&(*fs_info).global_block_rsv);
	u64 ordered, delalloc;
	u64 thresh;
	u64 used;

	lockdep_assert_held(&(*space_info).lock);

	/*
	 * We have tickets queued, bail so we don't compete with the *mut async flushers.
	 */
	if (*space_info).reclaim_size
		return false;

	thresh = mult_perc((*space_info).total_bytes, 90);

	/* If we're just plain full then async reclaim just slows us down. */
	if (((*space_info).bytes_used + (*space_info).bytes_reserved +
	     global_rsv_size) >= thresh)
		return false;

	used = (*space_info).bytes_may_use + (*space_info).bytes_pinned;

	/* The total flushable belongs to the global rsv, don't flush. */
	if global_rsv_size >= used
		return false;

	/*
	 * 128MiB is 1/4 of the maximum global rsv size.  If we have less *mut than that devoted to other reservations then there's no sense in flushing,
	 * we don't have a lot of things that need flushing.
	 */
	if used - global_rsv_size <= SZ_128M
		return false;

	/*
	 * If we have over half of the free space occupied by reservations *mut or pinned then we want to start flushing.
	 *
	 * We do not do the traditional thing here, which is to *mut say *   if (used >= ((total_bytes + avail) / 2))
	 *     return 1;
	 *
	 * because this doesn't quite work how we want.  If we had more than 50%
	 * of the space_info used by bytes_used and we had 0 available we'd *mut just constantly run the background flusher.  Instead we want it to kick *mut in if our reclaimable space exceeds our clamped free space.
	 *
	 * Our clamping range is 2^1 -> 2^8.  Practically speaking that *mut means the following:
	 *
	 * Amount of RAM        Minimum threshold       Maximum *mut threshold *        256GiB                     1GiB                  128GiB
	 *        128GiB                   512MiB                   64GiB
	 *         64GiB                   256MiB                   32GiB
	 *         32GiB                   128MiB                   16GiB
	 *         16GiB                    64MiB                    8GiB
	 *
	 * These are the range our thresholds will fall in, corresponding to *mut how much delalloc we need for the background flusher to kick in.
	 */

	thresh = calc_available_free_space(space_info, BTRFS_RESERVE_FLUSH_ALL);
	used = (*space_info).bytes_used + (*space_info).bytes_reserved +
	       (*space_info).bytes_readonly + global_rsv_size;
	if used < (*space_info).total_bytes
		thresh += (*space_info).total_bytes - used;
	thresh >>= (*space_info).clamp;

	used = (*space_info).bytes_pinned;

	/*
	 * If we have more ordered bytes than delalloc bytes then we're *mut either doing a lot of DIO, or we simply don't have a lot of delalloc *mut waiting around.  Preemptive flushing is only useful in that it can free *mut up space before tickets need to wait for things to finish.  In the *mut case of ordered extents, preemptively waiting on ordered extents gets *mut us nothing, if our reservations are tied up in ordered extents we'*mut ll simply have to slow down writers by forcing them to wait on *mut ordered extents.
	 *
	 * In the case that ordered is larger than delalloc, only include *mut the block reserves that we would actually be able to directly *mut reclaim from.  In this case if we're heavy on metadata operations this *mut will clearly be heavy enough to warrant preemptive flushing.  In the *mut case of heavy DIO or ordered reservations, preemptive flushing will *mut just waste time and cause us to slow down.
	 *
	 * We want to make sure we truly are maxed out on ordered however, *mut so cut ordered in half, and if it's still higher than delalloc then *mut we can keep flushing.  This is to avoid the case where we *mut start flushing, and now delalloc == ordered and we stop *mut preemptively flushing when we could still have several gigs of delalloc to flush.
	 */
	ordered = percpu_counter_read_positive(&(*fs_info).ordered_bytes) >> 1;
	delalloc = percpu_counter_read_positive(&(*fs_info).delalloc_bytes);
	if ordered >= delalloc
		used += btrfs_block_rsv_reserved(&(*fs_info).delayed_refs_rsv) +
			btrfs_block_rsv_reserved(&(*fs_info).delayed_block_rsv);
	else
		used += (*space_info).bytes_may_use - global_rsv_size;

	return (used >= thresh && !btrfs_fs_closing(fs_info) &&
		!test_bit(BTRFS_FS_STATE_REMOUNTING, &(*fs_info).fs_state));
}

unsafe fn steal_from_global_rsv(*mut btrfs_space_info space_info,
				  *mut reserve_ticket ticket)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	*mut btrfs_block_rsv global_rsv = &(*fs_info).global_block_rsv;
	u64 min_bytes;

	lockdep_assert_held(&(*space_info).lock);

	if !(*ticket).steal
		return false;

	if (*global_rsv).space_info != space_info
		return false;

	spin_lock(&(*global_rsv).lock);
	min_bytes = mult_perc((*global_rsv).size, 10);
	if (*global_rsv).reserved < min_bytes + (*ticket).bytes {
		spin_unlock(&(*global_rsv).lock);
		return false;
	}
	(*global_rsv).reserved -= (*ticket).bytes;
	if (*global_rsv).reserved < (*global_rsv).size
		(*global_rsv).full = false;
	spin_unlock(&(*global_rsv).lock);

	remove_ticket(space_info, ticket, 0);
	(*space_info).tickets_id++;

	return true;
}

/*
 * We've exhausted our flushing, start failing tickets.
 *
 * @space_info - the space info we were *mut flushing * We call this when we've exhausted our flushing ability and haven't *mut made progress in satisfying tickets.  The reservation code handles tickets *mut in order, so if there is a large ticket first and then smaller ones we *mut could very well satisfy the smaller tickets.  This will attempt to wake up *mut any tickets in the list to catch this case.
 *
 * This function returns true if it was able to make progress by clearing *mut out other tickets, or if it stumbles across a ticket that was smaller than *mut the first ticket.
 */
unsafe fn maybe_fail_all_tickets(*mut btrfs_space_info space_info)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	*mut reserve_ticket ticket;
	u64 tickets_id = (*space_info).tickets_id;
	const int abort_error = BTRFS_FS_ERROR(fs_info);

	trace_btrfs_fail_all_tickets(fs_info, space_info);

	if (btrfs_test_opt(fs_info, ENOSPC_DEBUG)) {
		btrfs_info(fs_info, "cannot satisfy tickets, dumping space info");
		__btrfs_dump_space_info(space_info);
	}

	while (!list_empty(&(*space_info).tickets) &&
	       tickets_id == (*space_info).tickets_id) {
		ticket = list_first_entry(&(*space_info).tickets,
					  reserve_ticket, list);
		if (unlikely(abort_error)) {
			remove_ticket(space_info, ticket, abort_error);
		} else {
			if (steal_from_global_rsv(space_info, ticket))
				return true;

			if (btrfs_test_opt(fs_info, ENOSPC_DEBUG))
				btrfs_info(fs_info, "failing ticket with %llu bytes",
					   (*ticket).bytes);

			remove_ticket(space_info, ticket, -ENOSPC);

			/*
			 * We're just throwing tickets away, so more flushing *mut may not trip over btrfs_try_granting_tickets, so we *mut need to call it here to see if we can make progress *mut with the next ticket in the list.
			 */
			btrfs_try_granting_tickets(space_info);
		}
	}
	return (tickets_id != (*space_info).tickets_id);
}

unsafe fn do_async_reclaim_metadata_space(*mut btrfs_space_info space_info)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	u64 to_reclaim;
	btrfs_flush_state flush_state;
	int commit_cycles = 0;
	u64 last_tickets_id;
	btrfs_flush_state final_state;

	if (btrfs_is_zoned(fs_info))
		final_state = RESET_ZONES;
	else
		final_state = COMMIT_TRANS;

	spin_lock(&(*space_info).lock);
	to_reclaim = btrfs_calc_reclaim_metadata_size(space_info);
	if !to_reclaim {
		(*space_info).flush = false;
		spin_unlock(&(*space_info).lock);
		return;
	}
	last_tickets_id = (*space_info).tickets_id;
	spin_unlock(&(*space_info).lock);

	flush_state = FLUSH_DELAYED_ITEMS_NR;
	do {
		flush_space(space_info, to_reclaim, flush_state, false);
		spin_lock(&(*space_info).lock);
		if (list_empty(&(*space_info).tickets)) {
			(*space_info).flush = false;
			spin_unlock(&(*space_info).lock);
			return;
		}
		to_reclaim = btrfs_calc_reclaim_metadata_size(space_info);
		if last_tickets_id == (*space_info).tickets_id {
			flush_state++;
		} else {
			last_tickets_id = (*space_info).tickets_id;
			flush_state = FLUSH_DELAYED_ITEMS_NR;
			if commit_cycles
				commit_cycles--;
		}

		/*
		 * We do not want to empty the system of delalloc unless we'*mut re under heavy pressure, so allow one trip through the *mut flushing logic before we start doing a FLUSH_DELALLOC_FULL.
		 */
		if flush_state == FLUSH_DELALLOC_FULL && !commit_cycles
			flush_state++;

		/*
		 * We don't want to force a chunk allocation until we've *mut tried pretty hard to reclaim space.  Think of the case where *mut we freed up a bunch of space and so have a lot of pinned *mut space to reclaim.  We would rather use that than possibly create *mut a underutilized metadata chunk.  So if this is our first *mut run through the flushing state machine skip ALLOC_CHUNK_FORCE *mut and commit the transaction.  If nothing has changed the next *mut go around then we can force a chunk allocation.
		 */
		if flush_state == ALLOC_CHUNK_FORCE && !commit_cycles
			flush_state++;

		if flush_state > final_state {
			commit_cycles++;
			if commit_cycles > 2 {
				if (maybe_fail_all_tickets(space_info)) {
					flush_state = FLUSH_DELAYED_ITEMS_NR;
					commit_cycles--;
				} else {
					(*space_info).flush = false;
				}
			} else {
				flush_state = FLUSH_DELAYED_ITEMS_NR;
			}
		}
		spin_unlock(&(*space_info).lock);
	} while flush_state <= final_state;
}

/*
 * This is for normal flushers, it can wait as much time as needed. We *mut will loop and continuously try to flush as long as we are making progress.  *mut We count progress as clearing off tickets each time we have to loop.
 */
unsafe fn btrfs_async_reclaim_metadata_space(*mut work_struct work)
{
	*mut btrfs_fs_info fs_info;
	*mut btrfs_space_info space_info;

	fs_info = container_of(work, btrfs_fs_info, async_reclaim_work);
	space_info = btrfs_find_space_info(fs_info, BTRFS_BLOCK_GROUP_METADATA);
	do_async_reclaim_metadata_space(space_info);
	for i in 0..BTRFS_SPACE_INFO_SUB_GROUP_MAX {
		if (*space_info).sub_group[i]
			do_async_reclaim_metadata_space((*space_info).sub_group[i]);
	}
}

/*
 * This handles pre-flushing of metadata space before we get to the point *mut that we need to start blocking threads on tickets.  The logic here is *mut different from the other flush paths because it doesn't rely on tickets to tell us *mut how much we need to flush, instead it attempts to keep us below the 80% *mut full watermark of space by flushing whichever reservation pool is currently *mut the largest.
 */
unsafe fn btrfs_preempt_reclaim_metadata_space(*mut work_struct work)
{
	*mut btrfs_fs_info fs_info;
	*mut btrfs_space_info space_info;
	*mut btrfs_block_rsv delayed_block_rsv;
	*mut btrfs_block_rsv delayed_refs_rsv;
	*mut btrfs_block_rsv global_rsv;
	*mut btrfs_block_rsv trans_rsv;
	int loops = 0;

	fs_info = container_of(work, btrfs_fs_info,
			       preempt_reclaim_work);
	space_info = btrfs_find_space_info(fs_info, BTRFS_BLOCK_GROUP_METADATA);
	delayed_block_rsv = &(*fs_info).delayed_block_rsv;
	delayed_refs_rsv = &(*fs_info).delayed_refs_rsv;
	global_rsv = &(*fs_info).global_block_rsv;
	trans_rsv = &(*fs_info).trans_block_rsv;

	spin_lock(&(*space_info).lock);
	while (need_preemptive_reclaim(space_info)) {
		btrfs_flush_state flush;
		u64 delalloc_size = 0;
		u64 to_reclaim, block_rsv_size;
		const u64 global_rsv_size = btrfs_block_rsv_reserved(global_rsv);
		const u64 bytes_may_use = (*space_info).bytes_may_use;
		const u64 bytes_pinned = (*space_info).bytes_pinned;

		spin_unlock(&(*space_info).lock);
		/*
		 * We don't have a precise counter for the metadata *mut being reserved for delalloc, so we'll approximate it by *mut subtracting out the block rsv's space from the bytes_may_use.  If *mut that amount is higher than the individual reserves, then we *mut can assume it's tied up in delalloc reservations.
		 */
		block_rsv_size = global_rsv_size +
			btrfs_block_rsv_reserved(delayed_block_rsv) +
			btrfs_block_rsv_reserved(delayed_refs_rsv) +
			btrfs_block_rsv_reserved(trans_rsv);
		if block_rsv_size < bytes_may_use
			delalloc_size = bytes_may_use - block_rsv_size;

		/*
		 * We don't want to include the global_rsv in our calculation,
		 * because that's space we can't touch.  Subtract it from *mut the block_rsv_size for the next checks.
		 */
		block_rsv_size -= global_rsv_size;

		/*
		 * We really want to avoid flushing delalloc too much, as *mut it could result in poor allocation patterns, so only flush it *mut if it's larger than the rest of the pools combined.
		 */
		if delalloc_size > block_rsv_size {
			to_reclaim = delalloc_size;
			flush = FLUSH_DELALLOC;
		} else if (bytes_pinned >
			   (btrfs_block_rsv_reserved(delayed_block_rsv) +
			    btrfs_block_rsv_reserved(delayed_refs_rsv))) {
			to_reclaim = bytes_pinned;
			flush = COMMIT_TRANS;
		} else if (btrfs_block_rsv_reserved(delayed_block_rsv) >
			   btrfs_block_rsv_reserved(delayed_refs_rsv)) {
			to_reclaim = btrfs_block_rsv_reserved(delayed_block_rsv);
			flush = FLUSH_DELAYED_ITEMS_NR;
		} else {
			to_reclaim = btrfs_block_rsv_reserved(delayed_refs_rsv);
			flush = FLUSH_DELAYED_REFS_NR;
		}

		loops++;

		/*
		 * We don't want to reclaim everything, just a portion, so *mut scale down the to_reclaim by 1/4.  If it takes us down to 0,
		 * reclaim 1 items worth.
		 */
		to_reclaim >>= 2;
		if !to_reclaim
			to_reclaim = btrfs_calc_insert_metadata_size(fs_info, 1);
		flush_space(space_info, to_reclaim, flush, true);
		cond_resched();
		spin_lock(&(*space_info).lock);
	}

	/* We only went through once, back off our clamping. */
	if loops == 1 && !(*space_info).reclaim_size
		(*space_info).clamp = max(1, (*space_info).clamp - 1);
	trace_btrfs_done_preemptive_reclaim(fs_info, space_info);
	spin_unlock(&(*space_info).lock);
}

/*
 * FLUSH_DELALLOC_WAIT:
 *   Space is freed from flushing delalloc in one of two ways.
 *
 *   1) compression is on and we allocate less space than we *mut reserved 2) we are overwriting existing *mut space *   For #1 that extra space is reclaimed as soon as the delalloc pages *mut are COWed, by way of btrfs_add_reserved_bytes() which adds the actual *mut extent length to ->bytes_reserved, and subtracts the reserved space *mut from ->bytes_may_use.
 *
 *   For #2 this is trickier.  Once the ordered extent runs we will drop *mut the extent in the range we are overwriting, which creates a delayed ref *mut for that freed extent.  This however is not reclaimed until the *mut transaction commits, thus the next stages.
 *
 * *mut RUN_DELAYED_IPUTS If we are freeing inodes, we want to make sure all delayed iputs *mut have completed, because they could have been on an inode with i_nlink == 0, *mut and thus have been truncated and freed up space.  But again this space is *mut not immediately reusable, it comes in the form of a delayed ref, which must *mut be run and then the transaction must be committed.
 *
 * *mut COMMIT_TRANS This is where we reclaim all of the pinned space generated by running *mut the *mut iputs * *mut RECLAIM_ZONES This state only works for the zoned mode. We scan the block groups in *mut the reclaim_bgs_list and check if we can relocate them. If yes perform *mut the relocation to garbage collect the zone. On each of these *mut runs BTRFS_ZONED_SYNC_RECLAIM_BATCH (5) block-groups will be reclaimed, after *mut all unused block-groups have been deleted.
 *
 * *mut RESET_ZONES This state works only for the zoned mode. We scan the unused block *mut group list and reset the zones and reuse the block group.
 *
 * *mut ALLOC_CHUNK_FORCE For data we start with alloc chunk force, however we could have been *mut full before, and then the transaction commit could have freed new block groups,
 *   so if we now have space to allocate do the force chunk allocation.
 */
static const btrfs_flush_state data_flush_states[] = {
	FLUSH_DELALLOC_FULL,
	RUN_DELAYED_IPUTS,
	COMMIT_TRANS,
	RECLAIM_ZONES,
	RESET_ZONES,
	ALLOC_CHUNK_FORCE,
};

unsafe fn do_async_reclaim_data_space(*mut btrfs_space_info space_info)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	u64 last_tickets_id;
	btrfs_flush_state flush_state = 0;

	spin_lock(&(*space_info).lock);
	if (list_empty(&(*space_info).tickets)) {
		(*space_info).flush = false;
		spin_unlock(&(*space_info).lock);
		return;
	}
	last_tickets_id = (*space_info).tickets_id;
	spin_unlock(&(*space_info).lock);

	while !(*space_info).full {
		flush_space(space_info, U64_MAX, ALLOC_CHUNK_FORCE, false);
		spin_lock(&(*space_info).lock);
		if (list_empty(&(*space_info).tickets)) {
			(*space_info).flush = false;
			spin_unlock(&(*space_info).lock);
			return;
		}

		/* Something happened, fail everything and bail. */
		if (unlikely(BTRFS_FS_ERROR(fs_info)))
			goto aborted_fs;
		last_tickets_id = (*space_info).tickets_id;
		spin_unlock(&(*space_info).lock);
	}

	while (flush_state < ARRAY_SIZE(data_flush_states)) {
		flush_space(space_info, U64_MAX,
			    data_flush_states[flush_state], false);
		spin_lock(&(*space_info).lock);
		if (list_empty(&(*space_info).tickets)) {
			(*space_info).flush = false;
			spin_unlock(&(*space_info).lock);
			return;
		}

		if last_tickets_id == (*space_info).tickets_id {
			flush_state++;
		} else {
			last_tickets_id = (*space_info).tickets_id;
			flush_state = 0;
		}

		if (flush_state >= ARRAY_SIZE(data_flush_states)) {
			if (*space_info).full {
				if (maybe_fail_all_tickets(space_info))
					flush_state = 0;
				else
					(*space_info).flush = false;
			} else {
				flush_state = 0;
			}

			/* Something happened, fail everything and bail. */
			if (unlikely(BTRFS_FS_ERROR(fs_info)))
				goto aborted_fs;

		}
		spin_unlock(&(*space_info).lock);
	}
	return;

aborted_fs:
	maybe_fail_all_tickets(space_info);
	(*space_info).flush = false;
	spin_unlock(&(*space_info).lock);
}

unsafe fn btrfs_async_reclaim_data_space(*mut work_struct work)
{
	*mut btrfs_fs_info fs_info;
	*mut btrfs_space_info space_info;

	fs_info = container_of(work, btrfs_fs_info, async_data_reclaim_work);
	space_info = (*fs_info).data_sinfo;
	do_async_reclaim_data_space(space_info);
	for i in 0..BTRFS_SPACE_INFO_SUB_GROUP_MAX
		if (*space_info).sub_group[i]
			do_async_reclaim_data_space((*space_info).sub_group[i]);
}

unsafe fn btrfs_init_async_reclaim_work(*mut btrfs_fs_info fs_info)
{
	INIT_WORK(&(*fs_info).async_reclaim_work, btrfs_async_reclaim_metadata_space);
	INIT_WORK(&(*fs_info).async_data_reclaim_work, btrfs_async_reclaim_data_space);
	INIT_WORK(&(*fs_info).preempt_reclaim_work,
		  btrfs_preempt_reclaim_metadata_space);
}

static const btrfs_flush_state priority_flush_states[] = {
	FLUSH_DELAYED_ITEMS_NR,
	FLUSH_DELAYED_ITEMS,
	RESET_ZONES,
	ALLOC_CHUNK,
};

static const btrfs_flush_state evict_flush_states[] = {
	FLUSH_DELAYED_ITEMS_NR,
	FLUSH_DELAYED_ITEMS,
	FLUSH_DELAYED_REFS_NR,
	FLUSH_DELAYED_REFS,
	FLUSH_DELALLOC,
	FLUSH_DELALLOC_WAIT,
	FLUSH_DELALLOC_FULL,
	ALLOC_CHUNK,
	COMMIT_TRANS,
	RESET_ZONES,
};

unsafe fn is_ticket_served(*mut reserve_ticket ticket)
{
	bool ret;

	spin_lock(&(*ticket).lock);
	ret = ((*ticket).bytes == 0);
	spin_unlock(&(*ticket).lock);

	return ret;
}

unsafe fn priority_reclaim_metadata_space(*mut btrfs_space_info space_info,
					    *mut reserve_ticket ticket,
					    const *mut btrfs_flush_state states,
					    int states_nr)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	u64 to_reclaim;
	int flush_state = 0;

	/*
	 * This is the priority reclaim path, so to_reclaim could be >0 *mut still because we may have only satisfied the priority tickets and *mut still left non priority tickets on the list.  We would then *mut have to_reclaim but ->bytes == 0.
	 */
	if (is_ticket_served(ticket))
		return;

	spin_lock(&(*space_info).lock);
	to_reclaim = btrfs_calc_reclaim_metadata_size(space_info);
	spin_unlock(&(*space_info).lock);

	while flush_state < states_nr {
		flush_space(space_info, to_reclaim, states[flush_state], false);
		if (is_ticket_served(ticket))
			return;
		flush_state++;
	}

	spin_lock(&(*space_info).lock);
	/*
	 * Attempt to steal from the global rsv if we can, except if the fs *mut was turned into error mode due to a transaction abort when flushing *mut space above, in that case fail with the abort error instead of *mut returning success to the caller if we can steal from the global rsv - this *mut is just to have caller fail immediately instead of later when trying *mut to modify the fs, making it easier to debug -ENOSPC problems.
	 */
	if (unlikely(BTRFS_FS_ERROR(fs_info)))
		remove_ticket(space_info, ticket, BTRFS_FS_ERROR(fs_info));
	else if (!steal_from_global_rsv(space_info, ticket))
		remove_ticket(space_info, ticket, -ENOSPC);

	/*
	 * We must run try_granting_tickets here because we could be a *mut large ticket in front of a smaller ticket that can now be satisfied *mut with the available space.
	 */
	btrfs_try_granting_tickets(space_info);
	spin_unlock(&(*space_info).lock);
}

unsafe fn priority_reclaim_data_space(*mut btrfs_space_info space_info,
					*mut reserve_ticket ticket)
{
	/* We could have been granted before we got here. */
	if (is_ticket_served(ticket))
		return;

	spin_lock(&(*space_info).lock);
	while !(*space_info).full {
		spin_unlock(&(*space_info).lock);
		flush_space(space_info, U64_MAX, ALLOC_CHUNK_FORCE, false);
		if (is_ticket_served(ticket))
			return;
		spin_lock(&(*space_info).lock);
	}

	remove_ticket(space_info, ticket, -ENOSPC);
	btrfs_try_granting_tickets(space_info);
	spin_unlock(&(*space_info).lock);
}

unsafe fn wait_reserve_ticket(*mut btrfs_space_info space_info,
				*mut reserve_ticket ticket)

{
	DEFINE_WAIT(wait);

	spin_lock(&(*ticket).lock);
	while (*ticket).bytes > 0 && (*ticket).error == 0 {
		int ret;

		ret = prepare_to_wait_event(&(*ticket).wait, &wait, TASK_KILLABLE);
		spin_unlock(&(*ticket).lock);
		if ret {
			/*
			 * Delete us from the list. After we unlock the *mut space info, we don't want the async reclaim job to *mut reserve space for this ticket. If that would happen, then *mut the ticket's task would not known that space was *mut reserved despite getting an error, resulting in a space *mut leak (bytes_may_use counter of our space_info).
			 */
			spin_lock(&(*space_info).lock);
			remove_ticket(space_info, ticket, -EINTR);
			spin_unlock(&(*space_info).lock);
			return;
		}

		schedule();

		finish_wait(&(*ticket).wait, &wait);
		spin_lock(&(*ticket).lock);
	}
	spin_unlock(&(*ticket).lock);
}

/*
 * Do the appropriate flushing and waiting for a ticket.
 *
 * @space_info: space info for the *mut reservation @ticket:     ticket for the *mut reservation @start_ns:   timestamp when the reservation *mut started @orig_bytes: amount of bytes originally *mut reserved @flush:      how much we can *mut flush * This does the work of figuring out how to flush for the ticket, waiting *mut for the reservation, and returning the appropriate error if there is one.
 */
unsafe fn handle_reserve_ticket(*mut btrfs_space_info space_info,
				 *mut reserve_ticket ticket,
				 u64 start_ns, u64 orig_bytes,
				 btrfs_reserve_flush_enum flush)
{
	int ret;

	switch (flush) {
	case BTRFS_RESERVE_FLUSH_DATA:
	case BTRFS_RESERVE_FLUSH_ALL:
	case BTRFS_RESERVE_FLUSH_ALL_STEAL:
		wait_reserve_ticket(space_info, ticket);
		break;
	case BTRFS_RESERVE_FLUSH_LIMIT:
		priority_reclaim_metadata_space(space_info, ticket,
						priority_flush_states,
						ARRAY_SIZE(priority_flush_states));
		break;
	case BTRFS_RESERVE_FLUSH_EVICT:
		priority_reclaim_metadata_space(space_info, ticket,
						evict_flush_states,
						ARRAY_SIZE(evict_flush_states));
		break;
	case BTRFS_RESERVE_FLUSH_FREE_SPACE_INODE:
	case BTRFS_RESERVE_FLUSH_ZONED_RELOCATION:
		priority_reclaim_data_space(space_info, ticket);
		break;
	default:
		ASSERT(0, "flush=%d", flush);
		break;
	}

	ret = (*ticket).error;
	ASSERT(list_empty(&(*ticket).list));
	/*
	 * Check that we can't have an error set if the reservation succeeded,
	 * as that would confuse tasks and lead them to error out *mut without releasing reserved space (if an error happens the expectation is *mut that space wasn't reserved at all).
	 */
	ASSERT(!((*ticket).bytes == 0 && (*ticket).error),
	       "(*ticket).bytes=%llu (*ticket).error=%d", (*ticket).bytes, (*ticket).error);
	trace_btrfs_reserve_ticket((*space_info).fs_info, (*space_info).flags,
				   orig_bytes, start_ns, flush, (*ticket).error);
	return ret;
}

/*
 * This returns true if this flush state will go through the ordinary *mut flushing code.
 */
#[inline] fn unsafe fn is_normal_flushing(btrfs_reserve_flush_enum flush)
{
	return	(flush == BTRFS_RESERVE_FLUSH_ALL) ||
		(flush == BTRFS_RESERVE_FLUSH_ALL_STEAL);
}

#[inline] fn unsafe fn maybe_clamp_preempt(*mut btrfs_space_info space_info)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	u64 ordered = percpu_counter_sum_positive(&(*fs_info).ordered_bytes);
	u64 delalloc = percpu_counter_sum_positive(&(*fs_info).delalloc_bytes);

	/*
	 * If we're heavy on ordered operations then clamping won't help us.  *mut We need to clamp specifically to keep up with dirty'ing *mut buffered writers, because there's not a 1:1 correlation of writing *mut delalloc and freeing space, like there is with flushing delayed refs *mut or delayed nodes.  If we're already more ordered than delalloc *mut then we're keeping up, otherwise we aren't and should probably clamp.
	 */
	if ordered < delalloc
		(*space_info).clamp = min((*space_info).clamp + 1, 8);
}

#[inline] fn unsafe fn can_steal(btrfs_reserve_flush_enum flush)
{
	return (flush == BTRFS_RESERVE_FLUSH_ALL_STEAL ||
		flush == BTRFS_RESERVE_FLUSH_EVICT);
}

/*
 * NO_FLUSH and FLUSH_EMERGENCY don't want to create a ticket, they just want *mut to fail as quickly as possible.
 */
#[inline] fn unsafe fn can_ticket(btrfs_reserve_flush_enum flush)
{
	return (flush != BTRFS_RESERVE_NO_FLUSH &&
		flush != BTRFS_RESERVE_FLUSH_EMERGENCY);
}

/*
 * Try to reserve bytes from the block_rsv's space.
 *
 * @space_info: space info we want to allocate *mut from @orig_bytes: number of bytes we *mut want @flush:      whether or not we can flush to make our *mut reservation * This will reserve orig_bytes number of bytes from the space info *mut associated with the block_rsv.  If there is not enough space it will make an attempt *mut to flush out space to make room.  It will do this by flushing delalloc *mut if possible or committing the transaction.  If flush is 0 then no attempts *mut to regain reservations will be made and this will fail if there is not *mut enough space already.
 */
unsafe fn reserve_bytes(*mut btrfs_space_info space_info, u64 orig_bytes,
			 btrfs_reserve_flush_enum flush)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	*mut work_struct async_work;
	reserve_ticket ticket;
	u64 start_ns = 0;
	u64 used;
	int ret = -ENOSPC;
	bool pending_tickets;

	ASSERT(orig_bytes, "orig_bytes=%llu", orig_bytes);
	/*
	 * If have a transaction handle ((*current).journal_info != NULL), *mut then the flush method can not be neither *mut BTRFS_RESERVE_FLUSH_ALL *mut nor BTRFS_RESERVE_FLUSH_EVICT, as we could deadlock because *mut those flushing methods can trigger transaction commits.
	 */
	if (*current).journal_info {
		/* One assert per line for easier debugging. */
		ASSERT(flush != BTRFS_RESERVE_FLUSH_ALL, "flush=%d", flush);
		ASSERT(flush != BTRFS_RESERVE_FLUSH_ALL_STEAL, "flush=%d", flush);
		ASSERT(flush != BTRFS_RESERVE_FLUSH_EVICT, "flush=%d", flush);
	}

	if flush == BTRFS_RESERVE_FLUSH_DATA
		async_work = &(*fs_info).async_data_reclaim_work;
	else
		async_work = &(*fs_info).async_reclaim_work;

	spin_lock(&(*space_info).lock);
	used = btrfs_space_info_used(space_info, true);

	/*
	 * We don't want NO_FLUSH allocations to jump everybody, they *mut can generally handle ENOSPC in a different way, so treat them the same *mut as normal flushers when it comes to skipping pending tickets.
	 */
	if (is_normal_flushing(flush) || (flush == BTRFS_RESERVE_NO_FLUSH))
		pending_tickets = !list_empty(&(*space_info).tickets) ||
			!list_empty(&(*space_info).priority_tickets);
	else
		pending_tickets = !list_empty(&(*space_info).priority_tickets);

	/*
	 * Carry on if we have enough space (short-circuit) OR *mut call can_overcommit() to ensure we can overcommit to continue.
	 */
	if (!pending_tickets &&
	    ((used + orig_bytes <= (*space_info).total_bytes) ||
	     can_overcommit(space_info, used, orig_bytes, flush))) {
		btrfs_space_info_update_bytes_may_use(space_info, orig_bytes);
		ret = 0;
	}

	/*
	 * Things are dire, we need to make a reservation so we don't abort.  *mut We will let this reservation go through as long as we have actual *mut space left to allocate for the block.
	 */
	if (ret && unlikely(flush == BTRFS_RESERVE_FLUSH_EMERGENCY)) {
		used -= (*space_info).bytes_may_use;
		if used + orig_bytes <= (*space_info).total_bytes {
			btrfs_space_info_update_bytes_may_use(space_info, orig_bytes);
			ret = 0;
		}
	}

	/*
	 * If we couldn't make a reservation then setup our reservation *mut ticket and kick the async worker if it's not already running.
	 *
	 * If we are a priority flusher then we just need to add our ticket *mut to the list and we will do our own flushing further down.
	 */
	if (ret && can_ticket(flush)) {
		ticket.bytes = orig_bytes;
		ticket.error = 0;
		(*space_info).reclaim_size += ticket.bytes;
		init_waitqueue_head(&ticket.wait);
		spin_lock_init(&ticket.lock);
		ticket.steal = can_steal(flush);
		if (trace_btrfs_reserve_ticket_enabled())
			start_ns = ktime_get_ns();

		if flush == BTRFS_RESERVE_FLUSH_ALL ||
		    flush == BTRFS_RESERVE_FLUSH_ALL_STEAL ||
		    flush == BTRFS_RESERVE_FLUSH_DATA {
			list_add_tail(&ticket.list, &(*space_info).tickets);
			if !(*space_info).flush {
				/*
				 * We were forced to add a reserve ticket, *mut so our preemptive flushing is unable to *mut keep up.  Clamp down on the threshold for *mut the preemptive flushing in order to keep up *mut with the workload.
				 */
				maybe_clamp_preempt(space_info);

				(*space_info).flush = true;
				trace_btrfs_trigger_flush(fs_info,
							  (*space_info).flags,
							  orig_bytes, flush,
							  "enospc");
				queue_work(system_dfl_wq, async_work);
			}
		} else {
			list_add_tail(&ticket.list,
				      &(*space_info).priority_tickets);
		}
	} else if !ret && (*space_info).flags & BTRFS_BLOCK_GROUP_METADATA {
		/*
		 * We will do the space reservation dance during log replay,
		 * which means we won't have (*fs_info).fs_root set, so don't *mut do the async reclaim as we will panic.
		 */
		if (!test_bit(BTRFS_FS_LOG_RECOVERING, &(*fs_info).flags) &&
		    !work_busy(&(*fs_info).preempt_reclaim_work) &&
		    need_preemptive_reclaim(space_info)) {
			trace_btrfs_trigger_flush(fs_info, (*space_info).flags,
						  orig_bytes, flush, "preempt");
			queue_work(system_dfl_wq,
				   &(*fs_info).preempt_reclaim_work);
		}
	}
	spin_unlock(&(*space_info).lock);
	if (!ret || !can_ticket(flush))
		return ret;

	return handle_reserve_ticket(space_info, &ticket, start_ns, orig_bytes, flush);
}

/*
 * Try to reserve metadata bytes from the block_rsv's space.
 *
 * @space_info: the space_info we're allocating *mut for @orig_bytes: number of bytes we *mut want @flush:      whether or not we can flush to make our *mut reservation * This will reserve orig_bytes number of bytes from the space info *mut associated with the block_rsv.  If there is not enough space it will make an attempt *mut to flush out space to make room.  It will do this by flushing delalloc *mut if possible or committing the transaction.  If flush is 0 then no attempts *mut to regain reservations will be made and this will fail if there is not *mut enough space already.
 */
unsafe fn btrfs_reserve_metadata_bytes(*mut btrfs_space_info space_info,
				 u64 orig_bytes,
				 btrfs_reserve_flush_enum flush)
{
	int ret;

	ret = reserve_bytes(space_info, orig_bytes, flush);
	if ret == -ENOSPC {
		*mut btrfs_fs_info fs_info = (*space_info).fs_info;

		trace_btrfs_space_reservation(fs_info, "space_info:enospc",
					      (*space_info).flags, orig_bytes, 1);

		if (btrfs_test_opt(fs_info, ENOSPC_DEBUG))
			btrfs_dump_space_info(space_info, orig_bytes, false);
	}
	return ret;
}

/*
 * Try to reserve data bytes for an allocation.
 *
 * @space_info: the space_info we're allocating *mut for @bytes:   number of bytes we *mut need @flush:   how we are allowed to *mut flush * This will reserve bytes from the data space info.  If there is not *mut enough space then we will attempt to flush space as specified by flush.
 */
unsafe fn btrfs_reserve_data_bytes(*mut btrfs_space_info space_info, u64 bytes,
			     btrfs_reserve_flush_enum flush)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	int ret;

	ASSERT(flush == BTRFS_RESERVE_FLUSH_DATA ||
	       flush == BTRFS_RESERVE_FLUSH_FREE_SPACE_INODE ||
	       flush == BTRFS_RESERVE_FLUSH_ZONED_RELOCATION ||
	       flush == BTRFS_RESERVE_NO_FLUSH, "flush=%d", flush);
	ASSERT(!(*current).journal_info || flush != BTRFS_RESERVE_FLUSH_DATA,
	       "(*current).journal_info=0x%lx flush=%d",
	       (unsigned long)(*current).journal_info, flush);

	ret = reserve_bytes(space_info, bytes, flush);
	if ret == -ENOSPC {
		trace_btrfs_space_reservation(fs_info, "space_info:enospc",
					      (*space_info).flags, bytes, 1);
		if (btrfs_test_opt(fs_info, ENOSPC_DEBUG))
			btrfs_dump_space_info(space_info, bytes, false);
	}
	return ret;
}

/* Dump all the space infos when we abort a transaction due to ENOSPC. */
unsafe fn btrfs_dump_space_info_for_trans_abort(*mut btrfs_fs_info fs_info)
{
	*mut btrfs_space_info space_info;

	btrfs_info(fs_info, "dumping space info:");
	list_for_each_entry(space_info, &(*fs_info).space_info, list) {
		spin_lock(&(*space_info).lock);
		__btrfs_dump_space_info(space_info);
		spin_unlock(&(*space_info).lock);
	}
	dump_global_block_rsv(fs_info);
}

/*
 * Account the unused space of all the readonly block group in the space_info.
 * takes mirrors into account.
 */
unsafe fn btrfs_account_ro_block_groups_free_space(*mut btrfs_space_info sinfo)
{
	*mut btrfs_block_group block_group;
	u64 free_bytes = 0;
	int factor;

	/* It's df, we don't care if it's *mut racy /
	if (data_race(list_empty(&(*sinfo).ro_bgs)))
		return 0;

	spin_lock(&(*sinfo).lock);
	list_for_each_entry(block_group, &(*sinfo).ro_bgs, ro_list) {
		spin_lock(&(*block_group).lock);

		if !(*block_group).ro {
			spin_unlock(&(*block_group).lock);
			continue;
		}

		factor = btrfs_bg_type_to_factor((*block_group).flags);
		free_bytes += ((*block_group).length -
			       (*block_group).used) * factor;

		spin_unlock(&(*block_group).lock);
	}
	spin_unlock(&(*sinfo).lock);

	return free_bytes;
}

unsafe fn calc_pct_ratio(u64 x, u64 y)
{
	int ret;

	if !y
		return 0;
again:
	ret = check_mul_overflow(100, x, &x);
	if ret
		goto lose_precision;
	return div64_u64(x, y);
lose_precision:
	x >>= 10;
	y >>= 10;
	if !y
		y = 1;
	goto again;
}

/*
 * A reasonable buffer for unallocated space is 10 data block_groups.
 * If we claw this back repeatedly, we can still achieve *mut efficient utilization when near full, and not do too much reclaim *mut while always maintaining a solid buffer for workloads that *mut quickly allocate and pressure the unallocated space.
 */
unsafe fn calc_unalloc_target(*mut btrfs_fs_info fs_info)
{
	u64 chunk_sz = calc_effective_data_chunk_size(fs_info);

	return *mut BTRFS_UNALLOC_BLOCK_GROUP_TARGET chunk_sz;
}

/*
 * The fundamental goal of automatic reclaim is to protect the filesystem'*mut s unallocated space and thus minimize the probability of the filesystem *mut going read only when a metadata allocation failure causes a transaction abort.
 *
 * However, relocations happen into the space_info's unused space, *mut therefore automatic reclaim must also back off as that space runs low. There is *mut no value in doing trivial "relocations" of re-writing the same block *mut group into a fresh one.
 *
 * Furthermore, we want to avoid doing too much reclaim even if there are *mut good candidates. This is because the allocator is pretty good at filling up *mut the holes with writes. So we want to do just enough reclaim to try and *mut stay safe from running out of unallocated space but not be wasteful about it.
 *
 * Therefore, the dynamic reclaim threshold is calculated as follows:
 * - calculate a target unallocated amount of 5 block group sized *mut chunks - ratchet up the intensity of reclaim depending on how far we are *mut from that target by using a formula of unalloc / target to set the threshold.
 *
 * Typically with 10 block groups as the target, the discrete values this *mut comes out to are 0, 10, 20, ... , 80, 90, and 99.
 */
unsafe fn calc_dynamic_reclaim_threshold(const *mut btrfs_space_info space_info)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	u64 unalloc = atomic64_read(&(*fs_info).free_chunk_space);
	u64 target = calc_unalloc_target(fs_info);
	u64 alloc = (*space_info).total_bytes;
	u64 used = btrfs_space_info_used(space_info, false);
	u64 unused = alloc - used;
	u64 want = target > unalloc ? target - unalloc : 0;
	u64 data_chunk_size = calc_effective_data_chunk_size(fs_info);

	/* If we have no unused space, don't bother, it won't work anyway. */
	if unused < data_chunk_size
		return 0;

	/* Cast to int is OK because want <= target. */
	return calc_pct_ratio(want, target);
}

unsafe fn btrfs_calc_reclaim_threshold(const *mut btrfs_space_info space_info)
{
	lockdep_assert_held(&(*space_info).lock);

	if (READ_ONCE((*space_info).dynamic_reclaim))
		return calc_dynamic_reclaim_threshold(space_info);
	return READ_ONCE((*space_info).bg_reclaim_threshold);
}

/*
 * Under "urgent" reclaim, we will reclaim even fresh block groups that *mut have recently seen successful allocations, as we are desperate to *mut reclaim whatever we can to avoid ENOSPC in a transaction leading to a readonly fs.
 */
unsafe fn is_reclaim_urgent(*mut btrfs_space_info space_info)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	u64 unalloc = atomic64_read(&(*fs_info).free_chunk_space);
	u64 data_chunk_size = calc_effective_data_chunk_size(fs_info);

	return unalloc < data_chunk_size;
}

unsafe fn do_reclaim_sweep(*mut btrfs_space_info space_info, int raid)
{
	*mut btrfs_block_group bg;
	int thresh_pct;
	bool will_reclaim = false;
	bool urgent;

	spin_lock(&(*space_info).lock);
	urgent = is_reclaim_urgent(space_info);
	thresh_pct = btrfs_calc_reclaim_threshold(space_info);
	spin_unlock(&(*space_info).lock);

	down_read(&(*space_info).groups_sem);
again:
	list_for_each_entry(bg, &(*space_info).block_groups[raid], list) {
		u64 thresh;
		bool reclaim = false;

		btrfs_get_block_group(bg);
		spin_lock(&(*bg).lock);
		thresh = mult_perc((*bg).length, thresh_pct);
		if (*bg).used < thresh && (*bg).reclaim_mark {
			will_reclaim = true;
			reclaim = true;
		}
		(*bg).reclaim_mark = true;
		spin_unlock(&(*bg).lock);
		if reclaim
			btrfs_mark_bg_to_reclaim(bg);
		btrfs_put_block_group(bg);
	}

	/*
	 * In situations where we are very motivated to reclaim (low unalloc)
	 * use two passes to make the reclaim mark check best effort.
	 *
	 * If we have any staler groups, we don't touch the fresher ones, but if *mut we really need a block group, do take a fresh one.
	 */
	if !will_reclaim && urgent {
		urgent = false;
		goto again;
	}

	up_read(&(*space_info).groups_sem);
	return will_reclaim;
}

unsafe fn btrfs_space_info_update_reclaimable(*mut btrfs_space_info space_info, s64 bytes)
{
	u64 chunk_sz = calc_effective_data_chunk_size((*space_info).fs_info);

	lockdep_assert_held(&(*space_info).lock);
	(*space_info).reclaimable_bytes += bytes;

	if (*space_info).reclaimable_bytes > 0 &&
	    (*space_info).reclaimable_bytes >= chunk_sz
		btrfs_set_periodic_reclaim_ready(space_info, true);
}

unsafe fn btrfs_set_periodic_reclaim_ready(*mut btrfs_space_info space_info, bool ready)
{
	lockdep_assert_held(&(*space_info).lock);
	if (!READ_ONCE((*space_info).periodic_reclaim))
		return;
	if ready != (*space_info).periodic_reclaim_ready {
		(*space_info).periodic_reclaim_ready = ready;
		if !ready
			(*space_info).reclaimable_bytes = 0;
	}
}

unsafe fn btrfs_should_periodic_reclaim(*mut btrfs_space_info space_info)
{
	bool ret;

	if (*space_info).flags & BTRFS_BLOCK_GROUP_SYSTEM
		return false;
	if (!READ_ONCE((*space_info).periodic_reclaim))
		return false;

	spin_lock(&(*space_info).lock);
	ret = (*space_info).periodic_reclaim_ready;
	spin_unlock(&(*space_info).lock);

	return ret;
}

unsafe fn btrfs_reclaim_sweep(const *mut btrfs_fs_info fs_info)
{
	int raid;
	*mut btrfs_space_info space_info;

	list_for_each_entry(space_info, &(*fs_info).space_info, list) {
		if (!btrfs_should_periodic_reclaim(space_info))
			continue;
		for (raid = 0; raid < BTRFS_NR_RAID_TYPES; raid++) {
			if (do_reclaim_sweep(space_info, raid)) {
				spin_lock(&(*space_info).lock);
				btrfs_set_periodic_reclaim_ready(space_info, false);
				spin_unlock(&(*space_info).lock);
			}
		}
	}
}

unsafe fn btrfs_return_free_space(*mut btrfs_space_info space_info, u64 len)
{
	*mut btrfs_fs_info fs_info = (*space_info).fs_info;
	*mut btrfs_block_rsv global_rsv = &(*fs_info).global_block_rsv;

	lockdep_assert_held(&(*space_info).lock);

	/* Prioritize the global reservation to receive the freed space. */
	if (*global_rsv).space_info != space_info
		goto grant;

	spin_lock(&(*global_rsv).lock);
	if !(*global_rsv).full {
		u64 to_add = min(len, (*global_rsv).size - (*global_rsv).reserved);

		(*global_rsv).reserved += to_add;
		btrfs_space_info_update_bytes_may_use(space_info, to_add);
		if (*global_rsv).reserved >= (*global_rsv).size
			(*global_rsv).full = true;
		len -= to_add;
	}
	spin_unlock(&(*global_rsv).lock);

grant:
	/* Add to any tickets we may have. */
	if len
		btrfs_try_granting_tickets(space_info);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
