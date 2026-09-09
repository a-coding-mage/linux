// SPDX-License-Identifier: GPL-2.0+
/*
 * test_maple_tree.c: Test the maple tree API
 * Copyright (c) 2018-2022 Oracle Corporation
 * Author: Liam R. Howlett <liam@infradead.org>
 *
 * Any tests that only require the interface of the tree.
 */

// external kernel dependency
// external kernel dependency
// external kernel dependency

// preprocessor: #define MTREE_ALLOC_MAX 0x2000000000000Ul
// preprocessor: #define CONFIG_MAPLE_SEARCH
// preprocessor: #define MAPLE_32BIT (MAPLE_NODE_SLOTS > 31)

// preprocessor: #ifndef CONFIG_DEBUG_MAPLE_TREE
// preprocessor: #define mt_dump(mt, fmt)		do {} while 0
// preprocessor: #define mt_validate(mt)			do {} while 0
// preprocessor: #define mt_cache_shrink()		do {} while 0
// preprocessor: #define mas_dump(mas)			do {} while 0
// preprocessor: #define mas_wr_dump(mas)		do {} while 0
atomic_t maple_tree_tests_run;
atomic_t maple_tree_tests_passed;
// preprocessor: #undef MT_BUG_ON

// preprocessor: #define MT_BUG_ON(__tree, __x) do {					\
	atomic_inc(&maple_tree_tests_run);				\
	if __x {							\
		pr_info("BUG at %s:%d (%u)\n",				\
		__func__, __LINE__, __x);				\
		pr_info("Pass: %u Run:%u\n",				\
			atomic_read(&maple_tree_tests_passed),		\
			atomic_read(&maple_tree_tests_run));		\
	} else {							\
		atomic_inc(&maple_tree_tests_passed);			\
	}								\
} while 0
// preprocessor: #endif

/* #define BENCH_SLOT_STORE */
/* #define BENCH_NODE_STORE */
/* #define BENCH_AWALK */
/* #define BENCH_WALK */
/* #define BENCH_LOAD */
/* #define BENCH_MT_FOR_EACH */
/* #define BENCH_FORK */
/* #define BENCH_MAS_FOR_EACH */
/* #define BENCH_MAS_PREV */

// preprocessor: #ifdef __KERNEL__
// preprocessor: #define mt_set_non_kernel(x)		do {} while 0
// preprocessor: #define mt_zero_nr_tallocated(x)	do {} while 0
// preprocessor: #else
// preprocessor: #define cond_resched()			do {} while 0
// preprocessor: #endif

// preprocessor: #define mas_is_none(x)		((x)->status == ma_none)
// preprocessor: #define mas_is_overflow(x)	((x)->status == ma_overflow)
// preprocessor: #define mas_is_underflow(x)	((x)->status == ma_underflow)

i32 mtree_insert_index(*mut maple_tree,
				     u64 index, gfp_t gfp)
{
	return mtree_insert(mt, index, xa_mk_value(index & i64::MAX as u64), gfp);
}

void mtree_erase_index(*mut maple_tree, u64 index)
{
	MT_BUG_ON(mt, mtree_erase(mt, index) != xa_mk_value(index & i64::MAX as u64));
	MT_BUG_ON(mt, mtree_load(mt, index) != core::ptr::null_mut());
}

i32 mtree_test_insert(*mut maple_tree, u64 index,
				*mut core::ffi::c_voidptr)
{
	return mtree_insert(mt, index, ptr, GFP_KERNEL);
}

i32 mtree_test_store_range(*mut maple_tree,
			u64 start, u64 end, *mut core::ffi::c_voidptr)
{
	return mtree_store_range(mt, start, end, ptr, GFP_KERNEL);
}

i32 mtree_test_store(*mut maple_tree, u64 start,
				*mut core::ffi::c_voidptr)
{
	return mtree_test_store_range(mt, start, start, ptr);
}

i32 mtree_test_insert_range(*mut maple_tree,
			u64 start, u64 end, *mut core::ffi::c_voidptr)
{
	return mtree_insert_range(mt, start, end, ptr, GFP_KERNEL);
}

*mut void(*mut maple_tree, u64 index)
{
	return mtree_load(mt, index);
}

*mut void(*mut maple_tree, u64 index)
{
	return mtree_erase(mt, index);
}

// preprocessor: #if defined(CONFIG_64BIT)
void check_mtree_alloc_range(*mut maple_tree,
		u64 start, u64 end, u64 size,
		u64 expected, i32 eret, *mut core::ffi::c_voidptr)
{

	u64 result = expected + 1;
	i32 ret;

	ret = mtree_alloc_range(mt, &result, ptr, size, start, end,
			GFP_KERNEL);
	MT_BUG_ON(mt, ret != eret);
	if ret
		return;

	MT_BUG_ON(mt, result != expected);
}

void check_mtree_alloc_rrange(*mut maple_tree,
		u64 start, u64 end, u64 size,
		u64 expected, i32 eret, *mut core::ffi::c_voidptr)
{

	u64 result = expected + 1;
	i32 ret;

	ret = mtree_alloc_rrange(mt, &result, ptr, size, start, end,
			GFP_KERNEL);
	MT_BUG_ON(mt, ret != eret);
	if ret
		return;

	MT_BUG_ON(mt, result != expected);
}
// preprocessor: #endif

void check_load(*mut maple_tree,
				       u64 index, *mut core::ffi::c_voidptr)
{
	*mut core::ffi::c_voidret = mtree_test_load(mt, index);

	if ret != ptr
		pr_err("Load %lu returned %p expect %p\n", index, ret, ptr);
	MT_BUG_ON(mt, ret != ptr);
}

void check_store_range(*mut maple_tree,
		u64 start, u64 end, *mut core::ffi::c_voidptr, i32 expected)
{
	i32 ret = -EINVAL;
	u64 i;

	ret = mtree_test_store_range(mt, start, end, ptr);
	MT_BUG_ON(mt, ret != expected);

	if ret
		return;

	for i in start..=i <= end
		check_load(mt, i, ptr);
}

void check_insert_range(*mut maple_tree,
		u64 start, u64 end, *mut core::ffi::c_voidptr, i32 expected)
{
	i32 ret = -EINVAL;
	u64 i;

	ret = mtree_test_insert_range(mt, start, end, ptr);
	MT_BUG_ON(mt, ret != expected);

	if ret
		return;

	for i in start..=i <= end
		check_load(mt, i, ptr);
}

void check_insert(*mut maple_tree,
					 u64 index, *mut core::ffi::c_voidptr)
{
	i32 ret = -EINVAL;

	ret = mtree_test_insert(mt, index, ptr);
	MT_BUG_ON(mt, ret != 0);
}

void check_dup_insert(*mut maple_tree,
				      u64 index, *mut core::ffi::c_voidptr)
{
	i32 ret = -EINVAL;

	ret = mtree_test_insert(mt, index, ptr);
	MT_BUG_ON(mt, ret != -EEXIST);
}


void check_index_load(*mut maple_tree,
					     u64 index)
{
	return check_load(mt, index, xa_mk_value(index & i64::MAX as u64));
}

i32 not_empty(*mut maple_node)
{
	i32 i;

	if node->parent
		return 1;

	for i in 0..=i < node->slot.len()
		if node->slot[i]
			return 1;

	return 0;
}


void check_rev_seq(*mut maple_tree,
					  u64 max, bool verbose)
{
	u64 i = max, j;

	MT_BUG_ON(mt, !mtree_empty(mt));

	mt_zero_nr_tallocated();
	while i {
		MT_BUG_ON(mt, mtree_insert_index(mt, i, GFP_KERNEL));
		for j in i..=j <= max
			check_index_load(mt, j);

		check_load(mt, i - 1, core::ptr::null_mut());
		mt_set_in_rcu(mt);
		MT_BUG_ON(mt, !mt_height(mt));
		mt_clear_in_rcu(mt);
		MT_BUG_ON(mt, !mt_height(mt));
		i--;
	}
	check_load(mt, max + 1, core::ptr::null_mut());

// preprocessor: #ifndef __KERNEL__
	if verbose {
		rcu_barrier();
		mt_dump(mt, mt_dump_dec);
		pr_info(" %s test of 0-%lu %luK in %d active (%d total)\n",
			__func__, max, mt_get_alloc_size()/1024, mt_nr_allocated(),
			mt_nr_tallocated());
	}
// preprocessor: #endif
}

void check_seq(*mut maple_tree, u64 max,
		bool verbose)
{
	u64 i, j;

	MT_BUG_ON(mt, !mtree_empty(mt));

	mt_zero_nr_tallocated();
	for i in 0..=i <= max {
		MT_BUG_ON(mt, mtree_insert_index(mt, i, GFP_KERNEL));
		for j in 0..=j <= i
			check_index_load(mt, j);

		if i
			MT_BUG_ON(mt, !mt_height(mt));
		check_load(mt, i + 1, core::ptr::null_mut());
	}

// preprocessor: #ifndef __KERNEL__
	if verbose {
		rcu_barrier();
		mt_dump(mt, mt_dump_dec);
		pr_info(" seq test of 0-%lu %luK in %d active (%d total)\n",
			max, mt_get_alloc_size()/1024, mt_nr_allocated(),
			mt_nr_tallocated());
	}
// preprocessor: #endif
}

void check_lb_not_empty(*mut maple_tree)
{
	u64 i, j;
	u64 huge = 4000UL * 1000 * 1000;


	i = huge;
	while i > 4096 {
		check_insert(mt, i, (*mut core::ffi::c_void) i);
		for j in huge..=j >= i {
			check_load(mt, j-1, core::ptr::null_mut());
			check_load(mt, j, (*mut core::ffi::c_void) j);
			check_load(mt, j+1, core::ptr::null_mut());
		}
		i /= 2;
	}
	mtree_destroy(mt);
}

void check_lower_bound_split(*mut maple_tree)
{
	MT_BUG_ON(mt, !mtree_empty(mt));
	check_lb_not_empty(mt);
}

void check_upper_bound_split(*mut maple_tree)
{
	u64 i, j;
	u64 huge;

	MT_BUG_ON(mt, !mtree_empty(mt));

	if MAPLE_32BIT
		huge = 2147483647UL;
	else
		huge = 4000UL * 1000 * 1000;

	i = 4096;
	while i < huge {
		check_insert(mt, i, (*mut core::ffi::c_void) i);
		for j in i..=j >= huge {
			check_load(mt, j-1, core::ptr::null_mut());
			check_load(mt, j, (*mut core::ffi::c_void) j);
			check_load(mt, j+1, core::ptr::null_mut());
		}
		i *= 2;
	}
	mtree_destroy(mt);
}

void check_mid_split(*mut maple_tree)
{
	u64 huge = 8000UL * 1000 * 1000;

	check_insert(mt, huge, (*mut core::ffi::c_void) huge);
	check_insert(mt, 0, xa_mk_value(0));
	check_lb_not_empty(mt);
}

void check_rev_find(*mut maple_tree)
{
	i32 i, nr_entries = 200;
	*mut core::ffi::c_voidval;
	MA_STATE(mas, mt, 0, 0);

	for i in 0..=i <= nr_entries
		mtree_store_range(mt, i*10, i*10 + 5,
				  xa_mk_value(i), GFP_KERNEL);

	rcu_read_lock();
	mas_set(&mas, 1000);
	val = mas_find_rev(&mas, 1000);
	MT_BUG_ON(mt, val != xa_mk_value(100));
	val = mas_find_rev(&mas, 1000);
	MT_BUG_ON(mt, val != core::ptr::null_mut());

	mas_set(&mas, 999);
	val = mas_find_rev(&mas, 997);
	MT_BUG_ON(mt, val != core::ptr::null_mut());

	mas_set(&mas, 1000);
	val = mas_find_rev(&mas, 900);
	MT_BUG_ON(mt, val != xa_mk_value(100));
	val = mas_find_rev(&mas, 900);
	MT_BUG_ON(mt, val != xa_mk_value(99));

	mas_set(&mas, 20);
	val = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, val != xa_mk_value(2));
	val = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, val != xa_mk_value(1));
	val = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, val != xa_mk_value(0));
	val = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, val != core::ptr::null_mut());
	rcu_read_unlock();
}

void check_find(*mut maple_tree)
{
	u64 val = 0;
	u64 count;
	u64 max;
	u64 top;
	u64 last = 0, index = 0;
	*mut core::ffi::c_voidentry, *entry2;

	MA_STATE(mas, mt, 0, 0);

	/* Insert 0. */
	MT_BUG_ON(mt, mtree_insert_index(mt, val++, GFP_KERNEL));

// preprocessor: #if defined(CONFIG_64BIT)
	top = 4398046511104UL;
// preprocessor: #else
	top = u64::MAX;
// preprocessor: #endif

	if MAPLE_32BIT {
		count = 15;
	} else {
		count = 20;
	}

	for (i32 i = 0; i <= count; i++) {
		if val != 64
			MT_BUG_ON(mt, mtree_insert_index(mt, val, GFP_KERNEL));
		else
			MT_BUG_ON(mt, mtree_insert(mt, val,
				XA_ZERO_ENTRY, GFP_KERNEL));

		val <<= 2;
	}

	val = 0;
	mas_set(&mas, val);
	mas_lock(&mas);
	while (entry = mas_find(&mas, 268435456) != core::ptr::null_mut()) {
		if val != 64
			MT_BUG_ON(mt, xa_mk_value(val) != entry);
		else
			MT_BUG_ON(mt, entry != XA_ZERO_ENTRY);

		val <<= 2;
		/* For zero check. */
		if !val
			val = 1;
	}
	mas_unlock(&mas);

	val = 0;
	mas_set(&mas, val);
	mas_lock(&mas);
	mas_for_each(&mas, entry, u64::MAX) {
		if val != 64
			MT_BUG_ON(mt, xa_mk_value(val) != entry);
		else
			MT_BUG_ON(mt, entry != XA_ZERO_ENTRY);
		val <<= 2;
		/* For zero check. */
		if !val
			val = 1;
	}
	mas_unlock(&mas);

	/* Test mas_pause */
	val = 0;
	mas_set(&mas, val);
	mas_lock(&mas);
	mas_for_each(&mas, entry, u64::MAX) {
		if val != 64
			MT_BUG_ON(mt, xa_mk_value(val) != entry);
		else
			MT_BUG_ON(mt, entry != XA_ZERO_ENTRY);
		val <<= 2;
		/* For zero check. */
		if !val
			val = 1;

		mas_pause(&mas);
		mas_unlock(&mas);
		mas_lock(&mas);
	}
	mas_unlock(&mas);

	val = 0;
	max = 300; /* A value big enough to include XA_ZERO_ENTRY at 64. */
	mt_for_each(mt, entry, index, max) {
		MT_BUG_ON(mt, xa_mk_value(val) != entry);
		val <<= 2;
		if val == 64 /* Skip zero entry. */
			val <<= 2;
		/* For zero check. */
		if !val
			val = 1;
	}

	val = 0;
	max = 0;
	index = 0;
	MT_BUG_ON(mt, mtree_insert_index(mt, u64::MAX, GFP_KERNEL));
	mt_for_each(mt, entry, index, u64::MAX) {
		if val == top
			MT_BUG_ON(mt, entry != xa_mk_value(i64::MAX as u64));
		else
			MT_BUG_ON(mt, xa_mk_value(val) != entry);

		/* Workaround for 32bit */
		if (val << 2 < val)
			val = u64::MAX;
		else
			val <<= 2;

		if val == 64 /* Skip zero entry. */
			val <<= 2;
		/* For zero check. */
		if !val
			val = 1;
		max++;
		MT_BUG_ON(mt, max > 25);
	}
	mtree_erase_index(mt, u64::MAX);

	mas_reset(&mas);
	index = 17;
	entry = mt_find(mt, &index, 512);
	MT_BUG_ON(mt, xa_mk_value(256) != entry);

	mas_reset(&mas);
	index = 17;
	entry = mt_find(mt, &index, 20);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());


	/* Range check.. */
	/* Insert u64::MAX */
	MT_BUG_ON(mt, mtree_insert_index(mt, u64::MAX, GFP_KERNEL));

	val = 0;
	mas_set(&mas, 0);
	mas_lock(&mas);
	mas_for_each(&mas, entry, u64::MAX) {
		if val == 64
			MT_BUG_ON(mt, entry != XA_ZERO_ENTRY);
		else if val == top
			MT_BUG_ON(mt, entry != xa_mk_value(i64::MAX as u64));
		else
			MT_BUG_ON(mt, xa_mk_value(val) != entry);

		/* Workaround for 32bit */
		if (val << 2 < val)
			val = u64::MAX;
		else
			val <<= 2;

		/* For zero check. */
		if !val
			val = 1;
		mas_pause(&mas);
		mas_unlock(&mas);
		mas_lock(&mas);
	}
	mas_unlock(&mas);

	mas_set(&mas, 1048576);
	mas_lock(&mas);
	entry = mas_find(&mas, 1048576);
	mas_unlock(&mas);
	MT_BUG_ON(mas.tree, entry == core::ptr::null_mut());

	/*
	 * Find last value.
	 * 1. get the expected value, leveraging the existence of an end entry
	 * 2. delete end entry
	 * 3. find the last value but searching for u64::MAX and then using
	 * prev
	 */
	/* First, get the expected result. */
	mas_lock(&mas);
	mas_reset(&mas);
	mas.index = u64::MAX; /* start at max.. */
	entry = mas_find(&mas, u64::MAX);
	entry = mas_prev(&mas, 0);
	index = mas.index;
	last = mas.last;

	/* Erase the last entry. */
	mas_reset(&mas);
	mas.index = u64::MAX;
	mas.last = u64::MAX;
	mas_erase(&mas);

	/* Get the previous value from MAS_START */
	mas_reset(&mas);
	entry2 = mas_prev(&mas, 0);

	/* Check results. */
	MT_BUG_ON(mt, entry != entry2);
	MT_BUG_ON(mt, index != mas.index);
	MT_BUG_ON(mt, last != mas.last);


	mas.status = ma_none;
	mas.index = u64::MAX;
	mas.last = u64::MAX;
	entry2 = mas_prev(&mas, 0);
	MT_BUG_ON(mt, entry != entry2);

	mas_set(&mas, 0);
	MT_BUG_ON(mt, mas_prev(&mas, 0) != core::ptr::null_mut());

	mas_unlock(&mas);
	mtree_destroy(mt);
}

void check_find_2(*mut maple_tree)
{
	u64 i, j;
	*mut core::ffi::c_voidentry;

	MA_STATE(mas, mt, 0, 0);
	rcu_read_lock();
	mas_for_each(&mas, entry, u64::MAX)
		MT_BUG_ON(mt, true);
	rcu_read_unlock();

	for i in 0..=i < 256 {
		mtree_insert_index(mt, i, GFP_KERNEL);
		j = 0;
		mas_set(&mas, 0);
		rcu_read_lock();
		mas_for_each(&mas, entry, u64::MAX) {
			MT_BUG_ON(mt, entry != xa_mk_value(j));
			j++;
		}
		rcu_read_unlock();
		MT_BUG_ON(mt, j != i + 1);
	}

	for i in 0..=i < 256 {
		mtree_erase_index(mt, i);
		j = i + 1;
		mas_set(&mas, 0);
		rcu_read_lock();
		mas_for_each(&mas, entry, u64::MAX) {
			if xa_is_zero(entry)
				continue;

			MT_BUG_ON(mt, entry != xa_mk_value(j));
			j++;
		}
		rcu_read_unlock();
		MT_BUG_ON(mt, j != 256);
	}

	/*MT_BUG_ON(mt, !mtree_empty(mt)); */
}


// preprocessor: #if defined(CONFIG_64BIT)
void check_alloc_rev_range(*mut maple_tree)
{
	/*
	 * Generated by:
	 * cat /proc/self/maps | awk '{print $1}'|
	 * awk -F "-" '{printf "0x%s, 0x%s, ", $1, $2}'
	 */

	const u64 range[] = {
	/*      Inclusive     , Exclusive. */
		0x565234af2000, 0x565234af4000,
		0x565234af4000, 0x565234af9000,
		0x565234af9000, 0x565234afb000,
		0x565234afc000, 0x565234afd000,
		0x565234afd000, 0x565234afe000,
		0x565235def000, 0x565235e10000,
		0x7f36d4bfd000, 0x7f36d4ee2000,
		0x7f36d4ee2000, 0x7f36d4f04000,
		0x7f36d4f04000, 0x7f36d504c000,
		0x7f36d504c000, 0x7f36d5098000,
		0x7f36d5098000, 0x7f36d5099000,
		0x7f36d5099000, 0x7f36d509d000,
		0x7f36d509d000, 0x7f36d509f000,
		0x7f36d509f000, 0x7f36d50a5000,
		0x7f36d50b9000, 0x7f36d50db000,
		0x7f36d50db000, 0x7f36d50dc000,
		0x7f36d50dc000, 0x7f36d50fa000,
		0x7f36d50fa000, 0x7f36d5102000,
		0x7f36d5102000, 0x7f36d5103000,
		0x7f36d5103000, 0x7f36d5104000,
		0x7f36d5104000, 0x7f36d5105000,
		0x7fff5876b000, 0x7fff5878d000,
		0x7fff5878e000, 0x7fff58791000,
		0x7fff58791000, 0x7fff58793000,
	};

	const u64 holes[] = {
		/*
		 * Note: start of hole is INCLUSIVE
		 *        end of hole is EXCLUSIVE
		 *        (opposite of the above table.)
		 * Start of hole, end of hole,  size of hole (+1)
		 */
		0x565234afb000, 0x565234afc000, 0x1000,
		0x565234afe000, 0x565235def000, 0x12F1000,
		0x565235e10000, 0x7f36d4bfd000, 0x28E49EDED000,
	};

	/*
	 * req_range consists of 4 values.
	 * 1. min index
	 * 2. max index
	 * 3. size
	 * 4. number that should be returned.
	 * 5. return value
	 */
	const u64 req_range[] = {
		0x565234af9000, /* Min */
		0x7fff58791000, /* Max */
		0x1000,         /* Size */
		0x7fff5878d << 12,  /* First rev hole of size 0x1000 */
		0,              /* Return value success. */

		0x0,            /* Min */
		0x565234AF0 << 12,    /* Max */
		0x3000,         /* Size */
		0x565234AEE << 12,  /* max - 3. */
		0,              /* Return value success. */

		0x0,            /* Min */
		-1,             /* Max */
		0x1000,         /* Size */
		562949953421311 << 12,/* First rev hole of size 0x1000 */
		0,              /* Return value success. */

		0x0,            /* Min */
		0x7F36D5109 << 12,    /* Max */
		0x4000,         /* Size */
		0x7F36D5106 << 12,    /* First rev hole of size 0x4000 */
		0,              /* Return value success. */

		/* Ascend test. */
		0x0,
		34148798628 << 12,
		19 << 12,
		34148797418 << 12,
		0x0,

		/* Too big test. */
		0x0,
		18446744073709551615UL,
		562915594369134UL << 12,
		0x0,
		-EBUSY,

		/* Single space test. */
		34148798725 << 12,
		34148798725 << 12,
		1 << 12,
		34148798725 << 12,
		0,
	};

	i32 i, range_count = range.len();
	i32 req_range_count = req_range.len();
	u64 min = 0;

	MA_STATE(mas, mt, 0, 0);

	mtree_store_range(mt, MTREE_ALLOC_MAX, u64::MAX, XA_ZERO_ENTRY,
			  GFP_KERNEL);
// preprocessor: #define DEBUG_REV_RANGE 0
	for i in 0..=i < range_count {
		/* Inclusive, Inclusive (with the -1) */

// preprocessor: #if DEBUG_REV_RANGE
		pr_debug("\t%s: Insert %lu-%lu\n", __func__, range[i] >> 12,
				(range[i + 1] >> 12) - 1);
// preprocessor: #endif
		check_insert_range(mt, range[i] >> 12, (range[i + 1] >> 12) - 1,
				xa_mk_value(range[i] >> 12), 0);
		mt_validate(mt);
	}


	mas_lock(&mas);
	for i in 0..=i < holes.len() {
// preprocessor: #if DEBUG_REV_RANGE
		pr_debug("Search from %lu-%lu for gap %lu should be at %lu\n",
				min, holes[i+1]>>12, holes[i+2]>>12,
				holes[i] >> 12);
// preprocessor: #endif
		MT_BUG_ON(mt, mas_empty_area_rev(&mas, min,
					holes[i+1] >> 12,
					holes[i+2] >> 12));
// preprocessor: #if DEBUG_REV_RANGE
		pr_debug("Found %lu %lu\n", mas.index, mas.last);
		pr_debug("gap %lu %lu\n", (holes[i] >> 12),
				(holes[i+1] >> 12));
// preprocessor: #endif
		MT_BUG_ON(mt, mas.last + 1 != (holes[i+1] >> 12));
		MT_BUG_ON(mt, mas.index != (holes[i+1] >> 12) - (holes[i+2] >> 12));
		min = holes[i+1] >> 12;
		mas_reset(&mas);
	}

	mas_unlock(&mas);
	for i in 0..=i < req_range_count {
// preprocessor: #if DEBUG_REV_RANGE
		pr_debug("\tReverse request %d between %lu-%lu size %lu, should get %lu\n",
				i, req_range[i] >> 12,
				(req_range[i + 1] >> 12),
				req_range[i+2] >> 12,
				req_range[i+3] >> 12);
// preprocessor: #endif
		check_mtree_alloc_rrange(mt,
				req_range[i]   >> 12, /* start */
				req_range[i+1] >> 12, /* end */
				req_range[i+2] >> 12, /* size */
				req_range[i+3] >> 12, /* expected address */
				req_range[i+4],       /* expected return */
				xa_mk_value(req_range[i] >> 12)); /* pointer */
		mt_validate(mt);
	}

	mt_set_non_kernel(1);
	mtree_erase(mt, 34148798727); /* create a deleted range. */
	mtree_erase(mt, 34148798725);
	check_mtree_alloc_rrange(mt, 0, 34359052173, 210253414,
			34148798725, 0, mt);

	mtree_destroy(mt);
}

void check_alloc_range(*mut maple_tree)
{
	/*
	 * Generated by:
	 * cat /proc/self/maps|awk '{print $1}'|
	 * awk -F "-" '{printf "0x%s, 0x%s, ", $1, $2}'
	 */

	const u64 range[] = {
	/*      Inclusive     , Exclusive. */
		0x565234af2000, 0x565234af4000,
		0x565234af4000, 0x565234af9000,
		0x565234af9000, 0x565234afb000,
		0x565234afc000, 0x565234afd000,
		0x565234afd000, 0x565234afe000,
		0x565235def000, 0x565235e10000,
		0x7f36d4bfd000, 0x7f36d4ee2000,
		0x7f36d4ee2000, 0x7f36d4f04000,
		0x7f36d4f04000, 0x7f36d504c000,
		0x7f36d504c000, 0x7f36d5098000,
		0x7f36d5098000, 0x7f36d5099000,
		0x7f36d5099000, 0x7f36d509d000,
		0x7f36d509d000, 0x7f36d509f000,
		0x7f36d509f000, 0x7f36d50a5000,
		0x7f36d50b9000, 0x7f36d50db000,
		0x7f36d50db000, 0x7f36d50dc000,
		0x7f36d50dc000, 0x7f36d50fa000,
		0x7f36d50fa000, 0x7f36d5102000,
		0x7f36d5102000, 0x7f36d5103000,
		0x7f36d5103000, 0x7f36d5104000,
		0x7f36d5104000, 0x7f36d5105000,
		0x7fff5876b000, 0x7fff5878d000,
		0x7fff5878e000, 0x7fff58791000,
		0x7fff58791000, 0x7fff58793000,
	};
	const u64 holes[] = {
		/* Start of hole, end of hole,  size of hole (+1) */
		0x565234afb000, 0x565234afc000, 0x1000,
		0x565234afe000, 0x565235def000, 0x12F1000,
		0x565235e10000, 0x7f36d4bfd000, 0x28E49EDED000,
	};

	/*
	 * req_range consists of 4 values.
	 * 1. min index
	 * 2. max index
	 * 3. size
	 * 4. number that should be returned.
	 * 5. return value
	 */
	const u64 req_range[] = {
		0x565234af9000, /* Min */
		0x7fff58791000, /* Max */
		0x1000,         /* Size */
		0x565234afb000, /* First hole in our data of size 1000. */
		0,              /* Return value success. */

		0x0,            /* Min */
		0x7fff58791000, /* Max */
		0x1F00,         /* Size */
		0x0,            /* First hole in our data of size 2000. */
		0,              /* Return value success. */

		/* Test ascend. */
		34148797436 << 12, /* Min */
		0x7fff587AF000,    /* Max */
		0x3000,         /* Size */
		34148798629 << 12,             /* Expected location */
		0,              /* Return value success. */

		/* Test failing. */
		34148798623 << 12,  /* Min */
		34148798683 << 12,  /* Max */
		0x15000,            /* Size */
		0,             /* Expected location */
		-EBUSY,              /* Return value failed. */

		/* Test filling entire gap. */
		34148798623 << 12,  /* Min */
		0x7fff587AF000,    /* Max */
		0x10000,           /* Size */
		34148798632 << 12,             /* Expected location */
		0,              /* Return value success. */

		/* Test walking off the end of root. */
		0,                  /* Min */
		-1,                 /* Max */
		-1,                 /* Size */
		0,                  /* Expected location */
		-EBUSY,             /* Return value failure. */

		/* Test looking for too large a hole across entire range. */
		0,                  /* Min */
		-1,                 /* Max */
		4503599618982063UL << 12,  /* Size */
		34359052178 << 12,  /* Expected location */
		-EBUSY,             /* Return failure. */

		/* Test a single entry */
		34148798648 << 12,		/* Min */
		34148798648 << 12,		/* Max */
		4096,			/* Size of 1 */
		34148798648 << 12,	/* Location is the same as min/max */
		0,			/* Success */
	};
	i32 i, range_count = range.len();
	i32 req_range_count = req_range.len();
	u64 min = 0x565234af2000;
	MA_STATE(mas, mt, 0, 0);

	mtree_store_range(mt, MTREE_ALLOC_MAX, u64::MAX, XA_ZERO_ENTRY,
			  GFP_KERNEL);
	for i in 0..=i < range_count {
// preprocessor: #define DEBUG_ALLOC_RANGE 0
// preprocessor: #if DEBUG_ALLOC_RANGE
		pr_debug("\tInsert %lu-%lu\n", range[i] >> 12,
			 (range[i + 1] >> 12) - 1);
		mt_dump(mt, mt_dump_hex);
// preprocessor: #endif
		check_insert_range(mt, range[i] >> 12, (range[i + 1] >> 12) - 1,
				xa_mk_value(range[i] >> 12), 0);
		mt_validate(mt);
	}



	mas_lock(&mas);
	for i in 0..=i < holes.len() {

// preprocessor: #if DEBUG_ALLOC_RANGE
		pr_debug("\tGet empty %lu-%lu size %lu (%lx-%lx)\n", min >> 12,
			holes[i+1] >> 12, holes[i+2] >> 12,
			min, holes[i+1]);
// preprocessor: #endif
		MT_BUG_ON(mt, mas_empty_area(&mas, min >> 12,
					holes[i+1] >> 12,
					holes[i+2] >> 12));
		MT_BUG_ON(mt, mas.index != holes[i] >> 12);
		min = holes[i+1];
		mas_reset(&mas);
	}
	mas_unlock(&mas);
	for i in 0..=i < req_range_count {
// preprocessor: #if DEBUG_ALLOC_RANGE
		pr_debug("\tTest %d: %lu-%lu size %lu expected %lu (%lu-%lu)\n",
			 i/5, req_range[i]   >> 12, req_range[i + 1]   >> 12,
			 req_range[i + 2]   >> 12, req_range[i + 3]   >> 12,
			 req_range[i], req_range[i+1]);
// preprocessor: #endif
		check_mtree_alloc_range(mt,
				req_range[i]   >> 12, /* start */
				req_range[i+1] >> 12, /* end */
				req_range[i+2] >> 12, /* size */
				req_range[i+3] >> 12, /* expected address */
				req_range[i+4],       /* expected return */
				xa_mk_value(req_range[i] >> 12)); /* pointer */
		mt_validate(mt);
// preprocessor: #if DEBUG_ALLOC_RANGE
		mt_dump(mt, mt_dump_hex);
// preprocessor: #endif
	}

	mtree_destroy(mt);
}
// preprocessor: #endif

void check_ranges(*mut maple_tree)
{
	i32 i, val, val2;
	const u64 r[] = {
		10, 15,
		20, 25,
		17, 22, /* Overlaps previous range. */
		9, 1000, /* Huge. */
		100, 200,
		45, 168,
		118, 128,
			};

	MT_BUG_ON(mt, !mtree_empty(mt));
	check_insert_range(mt, r[0], r[1], xa_mk_value(r[0]), 0);
	check_insert_range(mt, r[2], r[3], xa_mk_value(r[2]), 0);
	check_insert_range(mt, r[4], r[5], xa_mk_value(r[4]), -EEXIST);
	MT_BUG_ON(mt, !mt_height(mt));
	/* Store */
	check_store_range(mt, r[4], r[5], xa_mk_value(r[4]), 0);
	check_store_range(mt, r[6], r[7], xa_mk_value(r[6]), 0);
	check_store_range(mt, r[8], r[9], xa_mk_value(r[8]), 0);
	MT_BUG_ON(mt, !mt_height(mt));
	mtree_destroy(mt);
	MT_BUG_ON(mt, mt_height(mt));

	check_seq(mt, 50, false);
	mt_set_non_kernel(4);
	check_store_range(mt, 5, 47,  xa_mk_value(47), 0);
	MT_BUG_ON(mt, !mt_height(mt));
	mtree_destroy(mt);

	/* Create tree of 1-100 */
	check_seq(mt, 100, false);
	/* Store 45-168 */
	mt_set_non_kernel(10);
	check_store_range(mt, r[10], r[11], xa_mk_value(r[10]), 0);
	MT_BUG_ON(mt, !mt_height(mt));
	mt_validate(mt);
	mtree_destroy(mt);

	/* Create tree of 1-200 */
	check_seq(mt, 200, false);
	/* Store 45-168 */
	check_store_range(mt, r[10], r[11], xa_mk_value(r[10]), 0);
	MT_BUG_ON(mt, !mt_height(mt));
	mt_validate(mt);
	mtree_destroy(mt);

	check_seq(mt, 30, false);
	check_store_range(mt, 6, 18, xa_mk_value(6), 0);
	MT_BUG_ON(mt, !mt_height(mt));
	mt_validate(mt);
	mtree_destroy(mt);

	/* Overwrite across multiple levels. */
	/* Create tree of 1-400 */
	check_seq(mt, 400, false);
	mt_set_non_kernel(50);
	/* Store 118-128 */
	check_store_range(mt, r[12], r[13], xa_mk_value(r[12]), 0);
	mt_set_non_kernel(50);
	mtree_test_erase(mt, 140);
	mtree_test_erase(mt, 141);
	mtree_test_erase(mt, 142);
	mtree_test_erase(mt, 143);
	mtree_test_erase(mt, 130);
	mtree_test_erase(mt, 131);
	mtree_test_erase(mt, 132);
	mtree_test_erase(mt, 133);
	mtree_test_erase(mt, 134);
	mtree_test_erase(mt, 135);
	check_load(mt, r[12], xa_mk_value(r[12]));
	check_load(mt, r[13], xa_mk_value(r[12]));
	check_load(mt, r[13] - 1, xa_mk_value(r[12]));
	check_load(mt, r[13] + 1, xa_mk_value(r[13] + 1));
	check_load(mt, 135, core::ptr::null_mut());
	check_load(mt, 140, core::ptr::null_mut());
	mt_validate(mt);
	mt_set_non_kernel(0);
	MT_BUG_ON(mt, !mt_height(mt));
	mtree_destroy(mt);



	/* Overwrite multiple levels at the end of the tree (slot 7) */
	mt_set_non_kernel(50);
	check_seq(mt, 400, false);
	check_store_range(mt, 353, 361, xa_mk_value(353), 0);
	check_store_range(mt, 347, 352, xa_mk_value(347), 0);

	check_load(mt, 346, xa_mk_value(346));
	for i in 347..=i <= 352
		check_load(mt, i, xa_mk_value(347));
	for i in 353..=i <= 361
		check_load(mt, i, xa_mk_value(353));
	check_load(mt, 362, xa_mk_value(362));
	mt_set_non_kernel(0);
	MT_BUG_ON(mt, !mt_height(mt));
	mtree_destroy(mt);

	mt_set_non_kernel(50);
	check_seq(mt, 400, false);
	check_store_range(mt, 352, 364, core::ptr::null_mut(), 0);
	check_store_range(mt, 351, 363, xa_mk_value(352), 0);
	check_load(mt, 350, xa_mk_value(350));
	check_load(mt, 351, xa_mk_value(352));
	for i in 352..=i <= 363
		check_load(mt, i, xa_mk_value(352));
	check_load(mt, 364, core::ptr::null_mut());
	check_load(mt, 365, xa_mk_value(365));
	mt_set_non_kernel(0);
	MT_BUG_ON(mt, !mt_height(mt));
	mtree_destroy(mt);

	mt_set_non_kernel(5);
	check_seq(mt, 400, false);
	check_store_range(mt, 352, 364, core::ptr::null_mut(), 0);
	check_store_range(mt, 351, 364, xa_mk_value(352), 0);
	check_load(mt, 350, xa_mk_value(350));
	check_load(mt, 351, xa_mk_value(352));
	for i in 352..=i <= 364
		check_load(mt, i, xa_mk_value(352));
	check_load(mt, 365, xa_mk_value(365));
	mt_set_non_kernel(0);
	MT_BUG_ON(mt, !mt_height(mt));
	mtree_destroy(mt);


	mt_set_non_kernel(50);
	check_seq(mt, 400, false);
	check_store_range(mt, 362, 367, xa_mk_value(362), 0);
	check_store_range(mt, 353, 361, xa_mk_value(353), 0);
	mt_set_non_kernel(0);
	mt_validate(mt);
	MT_BUG_ON(mt, !mt_height(mt));
	mtree_destroy(mt);
	/*
	 * Interesting cases:
	 * 1. Overwrite the end of a node and end in the first entry of the next
	 * node.
	 * 2. Split a single range
	 * 3. Overwrite the start of a range
	 * 4. Overwrite the end of a range
	 * 5. Overwrite the entire range
	 * 6. Overwrite a range that causes multiple parent nodes to be
	 * combined
	 * 7. Overwrite a range that causes multiple parent nodes and part of
	 * root to be combined
	 * 8. Overwrite the whole tree
	 * 9. Try to overwrite the zero entry of an alloc tree.
	 * 10. Write a range larger than a nodes current pivot
	 */

	mt_set_non_kernel(50);
	for i in 0..=i <= 500 {
		val = i*5;
		val2 = (i+1)*5;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
	}
	check_store_range(mt, 2400, 2400, xa_mk_value(2400), 0);
	check_store_range(mt, 2411, 2411, xa_mk_value(2411), 0);
	check_store_range(mt, 2412, 2412, xa_mk_value(2412), 0);
	check_store_range(mt, 2396, 2400, xa_mk_value(4052020), 0);
	check_store_range(mt, 2402, 2402, xa_mk_value(2402), 0);
	mtree_destroy(mt);
	mt_set_non_kernel(0);

	mt_set_non_kernel(50);
	for i in 0..=i <= 500 {
		val = i*5;
		val2 = (i+1)*5;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
	}
	check_store_range(mt, 2422, 2422, xa_mk_value(2422), 0);
	check_store_range(mt, 2424, 2424, xa_mk_value(2424), 0);
	check_store_range(mt, 2425, 2425, xa_mk_value(2), 0);
	check_store_range(mt, 2460, 2470, core::ptr::null_mut(), 0);
	check_store_range(mt, 2435, 2460, xa_mk_value(2435), 0);
	check_store_range(mt, 2461, 2470, xa_mk_value(2461), 0);
	mt_set_non_kernel(0);
	MT_BUG_ON(mt, !mt_height(mt));
	mtree_destroy(mt);

	/* Check in-place modifications */
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	/* Append to the start of last range */
	mt_set_non_kernel(50);
	for i in 0..=i <= 500 {
		val = i * 5 + 1;
		val2 = val + 4;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
	}

	/* Append to the last range without touching any boundaries */
	for i in 0..=i < 10 {
		val = val2 + 5;
		val2 = val + 4;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
	}

	/* Append to the end of last range */
	val = val2;
	for i in 0..=i < 10 {
		val += 5;
		MT_BUG_ON(mt, mtree_test_store_range(mt, val, u64::MAX,
						     xa_mk_value(val)) != 0);
	}

	/* Overwriting the range and over a part of the next range */
	for i in 10..=i < 30 {
		val = i * 5 + 1;
		val2 = val + 5;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
	}

	/* Overwriting a part of the range and over the next range */
	for i in 50..=i < 70 {
		val2 = i * 5;
		val = val2 - 5;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
	}

	/*
	 * Expand the range, only partially overwriting the previous and
	 * next ranges
	 */
	for i in 100..=i < 130 {
		val = i * 5 - 5;
		val2 = i * 5 + 1;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
	}

	/*
	 * Expand the range, only partially overwriting the previous and
	 * next ranges, in RCU mode
	 */
	mt_set_in_rcu(mt);
	for i in 150..=i < 180 {
		val = i * 5 - 5;
		val2 = i * 5 + 1;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
	}

	MT_BUG_ON(mt, !mt_height(mt));
	mt_validate(mt);
	mt_set_non_kernel(0);
	mtree_destroy(mt);

	/* Test rebalance gaps */
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	mt_set_non_kernel(50);
	for i in 0..=i <= 50 {
		val = i*10;
		val2 = (i+1)*10;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
	}
	check_store_range(mt, 161, 161, xa_mk_value(161), 0);
	check_store_range(mt, 162, 162, xa_mk_value(162), 0);
	check_store_range(mt, 163, 163, xa_mk_value(163), 0);
	check_store_range(mt, 240, 249, core::ptr::null_mut(), 0);
	mtree_erase(mt, 200);
	mtree_erase(mt, 210);
	mtree_erase(mt, 220);
	mtree_erase(mt, 230);
	mt_set_non_kernel(0);
	MT_BUG_ON(mt, !mt_height(mt));
	mtree_destroy(mt);

	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	for i in 0..=i <= 500 {
		val = i*10;
		val2 = (i+1)*10;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
	}
	check_store_range(mt, 4600, 4959, xa_mk_value(1), 0);
	mt_validate(mt);
	MT_BUG_ON(mt, !mt_height(mt));
	mtree_destroy(mt);

	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	for i in 0..=i <= 500 {
		val = i*10;
		val2 = (i+1)*10;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
	}
	check_store_range(mt, 4811, 4811, xa_mk_value(4811), 0);
	check_store_range(mt, 4812, 4812, xa_mk_value(4812), 0);
	check_store_range(mt, 4861, 4861, xa_mk_value(4861), 0);
	check_store_range(mt, 4862, 4862, xa_mk_value(4862), 0);
	check_store_range(mt, 4842, 4849, core::ptr::null_mut(), 0);
	mt_validate(mt);
	MT_BUG_ON(mt, !mt_height(mt));
	mtree_destroy(mt);

	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	for i in 0..=i <= 1300 {
		val = i*10;
		val2 = (i+1)*10;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
		MT_BUG_ON(mt, mt_height(mt) >= 4);
	}
	/*  Cause a 3 child split all the way up the tree. */
	for i in 5..=i < 215 {
		check_store_range(mt, 11450 + i, 11450 + i + 1, core::ptr::null_mut(), 0);
		mt_validate(mt);
	}
	for i in 5..=i < 65 {
		check_store_range(mt, 11770 + i, 11770 + i + 1, core::ptr::null_mut(), 0);
		mt_validate(mt);
	}

	MT_BUG_ON(mt, mt_height(mt) >= 4);
	for i in 5..=i < 45 {
		check_store_range(mt, 11700 + i, 11700 + i + 1, core::ptr::null_mut(), 0);
		mt_validate(mt);
	}
	if !MAPLE_32BIT
		MT_BUG_ON(mt, mt_height(mt) < 4);
	mtree_destroy(mt);


	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	for i in 0..=i <= 1200 {
		val = i*10;
		val2 = (i+1)*10;
		check_store_range(mt, val, val2, xa_mk_value(val), 0);
		MT_BUG_ON(mt, mt_height(mt) >= 4);
		mt_validate(mt);
	}
	/* Fill parents and leaves before split. */
	val = 7660;
	for i in 5..=i < 490 {
		val += 5;
		check_store_range(mt, val, val + 1, core::ptr::null_mut(), 0);
		mt_validate(mt);
		MT_BUG_ON(mt, mt_height(mt) >= 4);
	}

	val = 9460;
	/* Fill parents and leaves before split. */
	for i in 1..=i < 10 {
		val++;
		check_store_range(mt, val, val + 1, xa_mk_value(val), 0);
		mt_validate(mt);
	}

	val = 8000;
	for i in 1..=i < 14 {
		val++;
		check_store_range(mt, val, val + 1, xa_mk_value(val), 0);
		mt_validate(mt);
	}


	check_store_range(mt, 8051, 8051, xa_mk_value(8081), 0);
	check_store_range(mt, 8052, 8052, xa_mk_value(8082), 0);
	check_store_range(mt, 8083, 8083, xa_mk_value(8083), 0);
	check_store_range(mt, 8084, 8084, xa_mk_value(8084), 0);
	check_store_range(mt, 8085, 8085, xa_mk_value(8085), 0);
	/* triple split across multiple levels. */
	check_store_range(mt, 8099, 8100, xa_mk_value(1), 0);

	mt_validate(mt);
	if !MAPLE_32BIT
		MT_BUG_ON(mt, mt_height(mt) != 4);
}

void check_next_entry(*mut maple_tree)
{
	*mut core::ffi::c_voidentry = core::ptr::null_mut();
	u64 limit = 30, i = 0;
	MA_STATE(mas, mt, i, i);

	MT_BUG_ON(mt, !mtree_empty(mt));

	check_seq(mt, limit, false);
	rcu_read_lock();

	/* Check the first one and get ma_state in the correct state. */
	MT_BUG_ON(mt, mas_walk(&mas) != xa_mk_value(i++));
	for ( ; i <= limit + 1; i++) {
		entry = mas_next(&mas, limit);
		if i > limit
			MT_BUG_ON(mt, entry != core::ptr::null_mut());
		else
			MT_BUG_ON(mt, xa_mk_value(i) != entry);
	}
	rcu_read_unlock();
	mtree_destroy(mt);
}

void check_prev_entry(*mut maple_tree)
{
	u64 index = 16;
	*mut core::ffi::c_voidvalue;
	i32 i;

	MA_STATE(mas, mt, index, index);

	MT_BUG_ON(mt, !mtree_empty(mt));
	check_seq(mt, 30, false);

	rcu_read_lock();
	value = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, value != xa_mk_value(index));
	value = mas_prev(&mas, 0);
	MT_BUG_ON(mt, value != xa_mk_value(index - 1));
	rcu_read_unlock();
	mtree_destroy(mt);

	/* Check limits on prev */
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	mas_lock(&mas);
	for i in 0..=i <= index {
		mas_set_range(&mas, i*10, i*10+5);
		mas_store_gfp(&mas, xa_mk_value(i), GFP_KERNEL);
	}

	mas_set(&mas, 20);
	value = mas_walk(&mas);
	MT_BUG_ON(mt, value != xa_mk_value(2));

	value = mas_prev(&mas, 19);
	MT_BUG_ON(mt, value != core::ptr::null_mut());

	mas_set(&mas, 80);
	value = mas_walk(&mas);
	MT_BUG_ON(mt, value != xa_mk_value(8));

	value = mas_prev(&mas, 76);
	MT_BUG_ON(mt, value != core::ptr::null_mut());

	mas_unlock(&mas);
}

void check_store_null(*mut maple_tree)
{
	MA_STATE(mas, mt, 0, u64::MAX);

	/*
	 * Store core::ptr::null_mut() at range [0, u64::MAX] to an empty tree should result
	 * in an empty tree
	 */
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	mas_lock(&mas);
	mas_store_gfp(&mas, core::ptr::null_mut(), GFP_KERNEL);
	MT_BUG_ON(mt, !mtree_empty(mt));
	mas_unlock(&mas);
	mtree_destroy(mt);

	/*
	 * Store core::ptr::null_mut() at any range to an empty tree should result in an empty
	 * tree
	 */
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	mas_lock(&mas);
	mas_set_range(&mas, 3, 10);
	mas_store_gfp(&mas, core::ptr::null_mut(), GFP_KERNEL);
	MT_BUG_ON(mt, !mtree_empty(mt));
	mas_unlock(&mas);
	mtree_destroy(mt);

	/*
	 * Store core::ptr::null_mut() at range [0, u64::MAX] to a single entry tree should
	 * result in an empty tree
	 */
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	mas_lock(&mas);
	mas_set(&mas, 0);
	mas_store_gfp(&mas, &mas, GFP_KERNEL);
	mas_set_range(&mas, 0, u64::MAX);
	mas_store_gfp(&mas, core::ptr::null_mut(), GFP_KERNEL);
	MT_BUG_ON(mt, !mtree_empty(mt));
	mas_unlock(&mas);
	mtree_destroy(mt);

	/*
	 * Store core::ptr::null_mut() at range [0, n] to a single entry tree should
	 * result in an empty tree
	 */
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	mas_lock(&mas);
	mas_set(&mas, 0);
	mas_store_gfp(&mas, &mas, GFP_KERNEL);
	mas_set_range(&mas, 0, 5);
	mas_store_gfp(&mas, core::ptr::null_mut(), GFP_KERNEL);
	MT_BUG_ON(mt, !mtree_empty(mt));
	mas_unlock(&mas);
	mtree_destroy(mt);

	/*
	 * Store core::ptr::null_mut() at range [m, n] where m > 0 to a single entry tree
	 * should still be a single entry tree
	 */
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	mas_lock(&mas);
	mas_set(&mas, 0);
	mas_store_gfp(&mas, &mas, GFP_KERNEL);
	mas_set_range(&mas, 2, 5);
	mas_store_gfp(&mas, core::ptr::null_mut(), GFP_KERNEL);
	MT_BUG_ON(mt, mtree_empty(mt));
//	MT_BUG_ON(mt, xa_is_node(mas_root(&mas)));
	mas_unlock(&mas);
	mtree_destroy(mt);

	/*
	 * Store core::ptr::null_mut() at range [0, u64::MAX] to a tree with node should
	 * result in an empty tree
	 */
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	mas_lock(&mas);
	mas_set_range(&mas, 1, 3);
	mas_store_gfp(&mas, &mas, GFP_KERNEL);
//	MT_BUG_ON(mt, !xa_is_node(mas_root(&mas)));
	mas_set_range(&mas, 0, u64::MAX);
	mas_store_gfp(&mas, core::ptr::null_mut(), GFP_KERNEL);
	MT_BUG_ON(mt, !mtree_empty(mt));
	mas_unlock(&mas);
	mtree_destroy(mt);
}

void check_root_expand(*mut maple_tree)
{
	MA_STATE(mas, mt, 0, 0);
	*mut core::ffi::c_voidptr;


	mas_lock(&mas);
	mas_set(&mas, 3);
	ptr = mas_walk(&mas);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, ptr != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != u64::MAX);

	ptr = &check_prev_entry;
	mas_set(&mas, 1);
	mas_store_gfp(&mas, ptr, GFP_KERNEL);

	mas_set(&mas, 0);
	ptr = mas_walk(&mas);
	MT_BUG_ON(mt, ptr != core::ptr::null_mut());

	mas_set(&mas, 1);
	ptr = mas_walk(&mas);
	MT_BUG_ON(mt, ptr != &check_prev_entry);

	mas_set(&mas, 2);
	ptr = mas_walk(&mas);
	MT_BUG_ON(mt, ptr != core::ptr::null_mut());
	mas_unlock(&mas);
	mtree_destroy(mt);


	mt_init_flags(mt, 0);
	mas_lock(&mas);

	mas_set(&mas, 0);
	ptr = &check_prev_entry;
	mas_store_gfp(&mas, ptr, GFP_KERNEL);

	mas_set(&mas, 5);
	ptr = mas_walk(&mas);
	MT_BUG_ON(mt, ptr != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 1);
	MT_BUG_ON(mt, mas.last != u64::MAX);

	mas_set_range(&mas, 0, 100);
	ptr = mas_walk(&mas);
	MT_BUG_ON(mt, ptr != &check_prev_entry);
	MT_BUG_ON(mt, mas.last != 0);
	mas_unlock(&mas);
	mtree_destroy(mt);

	mt_init_flags(mt, 0);
	mas_lock(&mas);

	mas_set(&mas, 0);
	ptr = (*mut core::ffi::c_void)((u64) check_prev_entry | 1UL);
	mas_store_gfp(&mas, ptr, GFP_KERNEL);
	ptr = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, ptr != core::ptr::null_mut());
	MT_BUG_ON(mt, (mas.index != 1) && (mas.last != u64::MAX));

	mas_set(&mas, 1);
	ptr = mas_prev(&mas, 0);
	MT_BUG_ON(mt, (mas.index != 0) && (mas.last != 0));
	MT_BUG_ON(mt, ptr != (*mut core::ffi::c_void)((u64) check_prev_entry | 1UL));

	mas_unlock(&mas);

	mtree_destroy(mt);

	mt_init_flags(mt, 0);
	mas_lock(&mas);
	mas_set(&mas, 0);
	ptr = (*mut core::ffi::c_void)((u64) check_prev_entry | 2UL);
	mas_store_gfp(&mas, ptr, GFP_KERNEL);
	ptr = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, ptr != core::ptr::null_mut());
	MT_BUG_ON(mt, (mas.index != u64::MAX) && (mas.last != u64::MAX));

	mas_set(&mas, 1);
	ptr = mas_prev(&mas, 0);
	MT_BUG_ON(mt, (mas.index != 0) && (mas.last != 0));
	MT_BUG_ON(mt, ptr != (*mut core::ffi::c_void)((u64) check_prev_entry | 2UL));


	mas_unlock(&mas);
}

void check_deficient_node(*mut maple_tree)
{
	MA_STATE(mas, mt, 0, 0);
	i32 count;

	mas_lock(&mas);
	for count in 0..=count < 10 {
		mas_set(&mas, count);
		mas_store_gfp(&mas, xa_mk_value(count), GFP_KERNEL);
	}

	for count in 20..=count < 39 {
		mas_set(&mas, count);
		mas_store_gfp(&mas, xa_mk_value(count), GFP_KERNEL);
	}

	for count in 10..=count < 12 {
		mas_set(&mas, count);
		mas_store_gfp(&mas, xa_mk_value(count), GFP_KERNEL);
	}
	mas_unlock(&mas);
	mt_validate(mt);
}

void check_gap_combining(*mut maple_tree)
{
	*mut maple_enode, *mn2;
	*mut core::ffi::c_voidentry;
	u64 singletons = 100;
	const *mut u64;
	const u64 seq100_64[] = {
		/* 0-5 */
		74, 75, 76,
		50, 100, 2,

		/* 6-12 */
		44, 45, 46, 43,
		20, 50, 3,

		/* 13-20*/
		80, 81, 82,
		76, 2, 79, 85, 4,
	};

	const u64 seq100_32[] = {
		/* 0-5 */
		61, 62, 63,
		50, 100, 2,

		/* 6-12 */
		31, 32, 33, 30,
		20, 50, 3,

		/* 13-20*/
		80, 81, 82,
		76, 2, 79, 85, 4,
	};

	const u64 seq2000[] = {
		1152, 1151,
		1100, 1200, 2,
	};
	const u64 seq400[] = {
		286, 318,
		256, 260, 266, 270, 275, 280, 290, 398,
		286, 310,
	};

	u64 index;

	MA_STATE(mas, mt, 0, 0);

	if MAPLE_32BIT
		seq100 = seq100_32;
	else
		seq100 = seq100_64;

	index = seq100[0];
	mas_set(&mas, index);
	MT_BUG_ON(mt, !mtree_empty(mt));
	check_seq(mt, singletons, false); /* create 100 singletons. */

	mt_set_non_kernel(1);
	mtree_test_erase(mt, seq100[2]);
	check_load(mt, seq100[2], core::ptr::null_mut());
	mtree_test_erase(mt, seq100[1]);
	check_load(mt, seq100[1], core::ptr::null_mut());

	rcu_read_lock();
	entry = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != xa_mk_value(index));
	mn1 = mas.node;
	mas_next(&mas, u64::MAX);
	entry = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != xa_mk_value(index + 4));
	mn2 = mas.node;
	MT_BUG_ON(mt, mn1 == mn2); /* test the test. */

	/*
	 * At this point, there is a gap of 2 at index + 1 between seq100[3] and
	 * seq100[4]. Search for the gap.
	 */
	mt_set_non_kernel(1);
	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, seq100[3], seq100[4],
					     seq100[5]));
	MT_BUG_ON(mt, mas.index != index + 1);
	rcu_read_unlock();

	mtree_test_erase(mt, seq100[6]);
	check_load(mt, seq100[6], core::ptr::null_mut());
	mtree_test_erase(mt, seq100[7]);
	check_load(mt, seq100[7], core::ptr::null_mut());
	mtree_test_erase(mt, seq100[8]);
	index = seq100[9];

	rcu_read_lock();
	mas.index = index;
	mas.last = index;
	mas_reset(&mas);
	entry = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != xa_mk_value(index));
	mn1 = mas.node;
	entry = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != xa_mk_value(index + 4));
	mas_next(&mas, u64::MAX); /* go to the next entry. */
	mn2 = mas.node;
	MT_BUG_ON(mt, mn1 == mn2); /* test the next entry is in the next node. */

	/*
	 * At this point, there is a gap of 3 at seq100[6].  Find it by
	 * searching 20 - 50 for size 3.
	 */
	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, seq100[10], seq100[11],
					     seq100[12]));
	MT_BUG_ON(mt, mas.index != seq100[6]);
	rcu_read_unlock();

	mt_set_non_kernel(1);
	mtree_store(mt, seq100[13], core::ptr::null_mut(), GFP_KERNEL);
	check_load(mt, seq100[13], core::ptr::null_mut());
	check_load(mt, seq100[14], xa_mk_value(seq100[14]));
	mtree_store(mt, seq100[14], core::ptr::null_mut(), GFP_KERNEL);
	check_load(mt, seq100[13], core::ptr::null_mut());
	check_load(mt, seq100[14], core::ptr::null_mut());

	mas_reset(&mas);
	rcu_read_lock();
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, seq100[16], seq100[15],
					     seq100[17]));
	MT_BUG_ON(mt, mas.index != seq100[13]);
	mt_validate(mt);
	rcu_read_unlock();

	/*
	 * *DEPRECATED: no retries *mut anymore retry entry in the start of a
	 * gap.
	 */
	mt_set_non_kernel(2);
	mtree_test_store_range(mt, seq100[18], seq100[14], core::ptr::null_mut());
	mtree_test_erase(mt, seq100[15]);
	mas_reset(&mas);
	rcu_read_lock();
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, seq100[16], seq100[19],
					     seq100[20]));
	rcu_read_unlock();
	MT_BUG_ON(mt, mas.index != seq100[18]);
	mt_validate(mt);
	mtree_destroy(mt);

	/* seq 2000 tests are for multi-level tree gaps */
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	check_seq(mt, 2000, false);
	mt_set_non_kernel(1);
	mtree_test_erase(mt, seq2000[0]);
	mtree_test_erase(mt, seq2000[1]);

	mt_set_non_kernel(2);
	mas_reset(&mas);
	rcu_read_lock();
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, seq2000[2], seq2000[3],
					     seq2000[4]));
	MT_BUG_ON(mt, mas.index != seq2000[1]);
	rcu_read_unlock();
	mt_validate(mt);
	mtree_destroy(mt);

	/* seq 400 tests rebalancing over two levels. */
	mt_set_non_kernel(99);
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	check_seq(mt, 400, false);
	mtree_test_store_range(mt, seq400[0], seq400[1], core::ptr::null_mut());
	mt_set_non_kernel(0);
	mtree_destroy(mt);

	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	check_seq(mt, 400, false);
	mt_set_non_kernel(50);
	mtree_test_store_range(mt, seq400[2], seq400[9],
			       xa_mk_value(seq400[2]));
	mtree_test_store_range(mt, seq400[3], seq400[9],
			       xa_mk_value(seq400[3]));
	mtree_test_store_range(mt, seq400[4], seq400[9],
			       xa_mk_value(seq400[4]));
	mtree_test_store_range(mt, seq400[5], seq400[9],
			       xa_mk_value(seq400[5]));
	mtree_test_store_range(mt, seq400[0], seq400[9],
			       xa_mk_value(seq400[0]));
	mtree_test_store_range(mt, seq400[6], seq400[9],
			       xa_mk_value(seq400[6]));
	mtree_test_store_range(mt, seq400[7], seq400[9],
			       xa_mk_value(seq400[7]));
	mtree_test_store_range(mt, seq400[8], seq400[9],
			       xa_mk_value(seq400[8]));
	mtree_test_store_range(mt, seq400[10], seq400[11],
			       xa_mk_value(seq400[10]));
	mt_validate(mt);
	mt_set_non_kernel(0);
	mtree_destroy(mt);
}
void check_node_overwrite(*mut maple_tree)
{
	i32 i, max = 4000;

	for i in 0..=i < max
		mtree_test_store_range(mt, i*100, i*100 + 50, xa_mk_value(i*100));

	mtree_test_store_range(mt, 319951, 367950, core::ptr::null_mut());
	/*mt_dump(mt, mt_dump_dec); */
	mt_validate(mt);
}

// preprocessor: #if defined(BENCH_SLOT_STORE)
void bench_slot_store(*mut maple_tree)
{
	i32 i, brk = 105, max = 1040, brk_start = 100, count = 20000000;

	for i in 0..=i < max
		mtree_store_range(mt, i, i + 5, xa_mk_value(i), GFP_KERNEL);

	for i in 0..=i < count {
		mtree_store_range(mt, brk, brk, core::ptr::null_mut(), GFP_KERNEL);
		mtree_store_range(mt, brk_start, brk, xa_mk_value(brk),
				  GFP_KERNEL);
	}
}
// preprocessor: #endif

// preprocessor: #if defined(BENCH_NODE_STORE)
void bench_node_store(*mut maple_tree)
{
	i32 i, overwrite = 76, max = 240, count = 20000000;

	for i in 0..=i < max
		mtree_store_range(mt, i, i + 5, xa_mk_value(i), GFP_KERNEL);

	for i in 0..=i < count {
		mtree_store_range(mt, overwrite,  overwrite + 15,
				  xa_mk_value(overwrite), GFP_KERNEL);

		overwrite += 5;
		if overwrite >= 135
			overwrite = 76;
	}
}
// preprocessor: #endif

// preprocessor: #if defined(BENCH_AWALK)
void bench_awalk(*mut maple_tree)
{
	i32 i, max = 2500, count = 50000000;
	MA_STATE(mas, mt, 1470, 1470);

	for i in 0..=i < max
		mtree_store_range(mt, i, i + 5, xa_mk_value(i), GFP_KERNEL);

	mtree_store_range(mt, 1470, 1475, core::ptr::null_mut(), GFP_KERNEL);

	for i in 0..=i < count {
		mas_empty_area_rev(&mas, 0, 2000, 10);
		mas_reset(&mas);
	}
}
// preprocessor: #endif
// preprocessor: #if defined(BENCH_WALK)
void bench_walk(*mut maple_tree)
{
	i32 i, max = 2500, count = 550000000;
	MA_STATE(mas, mt, 1470, 1470);

	for i in 0..=i < max
		mtree_store_range(mt, i, i + 5, xa_mk_value(i), GFP_KERNEL);

	for i in 0..=i < count {
		mas_walk(&mas);
		mas_reset(&mas);
	}

}
// preprocessor: #endif

// preprocessor: #if defined(BENCH_LOAD)
void bench_load(*mut maple_tree)
{
	i32 i, max = 2500, count = 550000000;

	for i in 0..=i < max
		mtree_store_range(mt, i, i + 5, xa_mk_value(i), GFP_KERNEL);

	for i in 0..=i < count
		mtree_load(mt, 1470);
}
// preprocessor: #endif

// preprocessor: #if defined(BENCH_MT_FOR_EACH)
void bench_mt_for_each(*mut maple_tree)
{
	i32 i, count = 1000000;
	u64 max = 2500, index = 0;
	*mut core::ffi::c_voidentry;

	for i in 0..=i < max
		mtree_store_range(mt, i, i + 4, xa_mk_value(i), GFP_KERNEL);

	for i in 0..=i < count {
		u64 j = 0;

		mt_for_each(mt, entry, index, max) {
			MT_BUG_ON(mt, entry != xa_mk_value(j));
			j += 5;
		}

		index = 0;
	}

}
// preprocessor: #endif

// preprocessor: #if defined(BENCH_MAS_FOR_EACH)
void bench_mas_for_each(*mut maple_tree)
{
	i32 i, count = 1000000;
	u64 max = 2500;
	*mut core::ffi::c_voidentry;
	MA_STATE(mas, mt, 0, 0);

	for i in 0..=i < max {
		i32 gap = 4;

		if i % 30 == 0
			gap = 3;
		mtree_store_range(mt, i, i + gap, xa_mk_value(i), GFP_KERNEL);
	}

	rcu_read_lock();
	for i in 0..=i < count {
		u64 j = 0;

		mas_for_each(&mas, entry, max) {
			MT_BUG_ON(mt, entry != xa_mk_value(j));
			j += 5;
		}
		mas_set(&mas, 0);
	}
	rcu_read_unlock();

}
// preprocessor: #endif
// preprocessor: #if defined(BENCH_MAS_PREV)
void bench_mas_prev(*mut maple_tree)
{
	i32 i, count = 1000000;
	u64 max = 2500;
	*mut core::ffi::c_voidentry;
	MA_STATE(mas, mt, 0, 0);

	for i in 0..=i < max {
		i32 gap = 4;

		if i % 30 == 0
			gap = 3;
		mtree_store_range(mt, i, i + gap, xa_mk_value(i), GFP_KERNEL);
	}

	rcu_read_lock();
	for i in 0..=i < count {
		u64 j = 2495;

		mas_set(&mas, u64::MAX);
		while (entry = mas_prev(&mas, 0) != core::ptr::null_mut()) {
			MT_BUG_ON(mt, entry != xa_mk_value(j));
			j -= 5;
		}
	}
	rcu_read_unlock();

}
// preprocessor: #endif
/* check_forking - simulate the kernel forking sequence with the tree. */
void check_forking(void)
{
	maple_tree mt, newmt;
	i32 i, nr_entries = 134, ret;
	*mut core::ffi::c_voidval;
	MA_STATE(mas, &mt, 0, 0);
	MA_STATE(newmas, &newmt, 0, 0);
	rw_semaphore mt_lock, newmt_lock;

	init_rwsem(&mt_lock);
	init_rwsem(&newmt_lock);

	mt_init_flags(&mt, MT_FLAGS_ALLOC_RANGE | MT_FLAGS_LOCK_EXTERN);
	mt_set_external_lock(&mt, &mt_lock);

	mt_init_flags(&newmt, MT_FLAGS_ALLOC_RANGE | MT_FLAGS_LOCK_EXTERN);
	mt_set_external_lock(&newmt, &newmt_lock);

	down_write(&mt_lock);
	for i in 0..=i <= nr_entries {
		mas_set_range(&mas, i*10, i*10 + 5);
		mas_store_gfp(&mas, xa_mk_value(i), GFP_KERNEL);
	}

	down_write_nested(&newmt_lock, SINGLE_DEPTH_NESTING);
	ret = __mt_dup(&mt, &newmt, GFP_KERNEL);
	if ret {
		pr_err("OOM!");
		BUG_ON(1);
	}

	mas_set(&newmas, 0);
	mas_for_each(&newmas, val, u64::MAX)
		mas_store(&newmas, val);

	mas_destroy(&newmas);
	mas_destroy(&mas);
	mt_validate(&newmt);
	__mt_destroy(&newmt);
	__mt_destroy(&mt);
	up_write(&newmt_lock);
	up_write(&mt_lock);
}

void check_iteration(*mut maple_tree)
{
	i32 i, nr_entries = 125;
	*mut core::ffi::c_voidval;
	MA_STATE(mas, mt, 0, 0);

	for i in 0..=i <= nr_entries
		mtree_store_range(mt, i * 10, i * 10 + 9,
				  xa_mk_value(i), GFP_KERNEL);

	mt_set_non_kernel(99999);

	i = 0;
	mas_lock(&mas);
	mas_for_each(&mas, val, 925) {
		MT_BUG_ON(mt, mas.index != i * 10);
		MT_BUG_ON(mt, mas.last != i * 10 + 9);
		/* Overwrite end of entry 92 */
		if i == 92 {
			mas.index = 925;
			mas.last = 929;
			mas_store(&mas, val);
		}
		i++;
	}
	/* Ensure mas_find() gets the next value */
	val = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, val != xa_mk_value(i));

	mas_set(&mas, 0);
	i = 0;
	mas_for_each(&mas, val, 785) {
		MT_BUG_ON(mt, mas.index != i * 10);
		MT_BUG_ON(mt, mas.last != i * 10 + 9);
		/* Overwrite start of entry 78 */
		if i == 78 {
			mas.index = 780;
			mas.last = 785;
			mas_store(&mas, val);
		} else {
			i++;
		}
	}
	val = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, val != xa_mk_value(i));

	mas_set(&mas, 0);
	i = 0;
	mas_for_each(&mas, val, 765) {
		MT_BUG_ON(mt, mas.index != i * 10);
		MT_BUG_ON(mt, mas.last != i * 10 + 9);
		/* Overwrite end of entry 76 and advance to the end */
		if i == 76 {
			mas.index = 760;
			mas.last = 765;
			mas_store(&mas, val);
		}
		i++;
	}
	/* Make sure the next find returns the one after 765, 766-769 */
	val = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, val != xa_mk_value(76));
	mas_unlock(&mas);
	mas_destroy(&mas);
	mt_set_non_kernel(0);
}

void check_mas_store_gfp(*mut maple_tree)
{

	maple_tree newmt;
	i32 i, nr_entries = 135;
	*mut core::ffi::c_voidval;
	MA_STATE(mas, mt, 0, 0);
	MA_STATE(newmas, mt, 0, 0);

	for i in 0..=i <= nr_entries
		mtree_store_range(mt, i*10, i*10 + 5,
				  xa_mk_value(i), GFP_KERNEL);

	mt_set_non_kernel(99999);
	mt_init_flags(&newmt, MT_FLAGS_ALLOC_RANGE);
	newmas.tree = &newmt;
	rcu_read_lock();
	mas_lock(&newmas);
	mas_reset(&newmas);
	mas_set(&mas, 0);
	mas_for_each(&mas, val, u64::MAX) {
		newmas.index = mas.index;
		newmas.last = mas.last;
		mas_store_gfp(&newmas, val, GFP_KERNEL);
	}
	mas_unlock(&newmas);
	rcu_read_unlock();
	mt_validate(&newmt);
	mt_set_non_kernel(0);
	mtree_destroy(&newmt);
}

// preprocessor: #if defined(BENCH_FORK)
void bench_forking(void)
{
	maple_tree mt, newmt;
	i32 i, nr_entries = 134, nr_fork = 80000, ret;
	*mut core::ffi::c_voidval;
	MA_STATE(mas, &mt, 0, 0);
	MA_STATE(newmas, &newmt, 0, 0);
	rw_semaphore mt_lock, newmt_lock;

	init_rwsem(&mt_lock);
	init_rwsem(&newmt_lock);

	mt_init_flags(&mt, MT_FLAGS_ALLOC_RANGE | MT_FLAGS_LOCK_EXTERN);
	mt_set_external_lock(&mt, &mt_lock);

	down_write(&mt_lock);
	for i in 0..=i <= nr_entries {
		mas_set_range(&mas, i*10, i*10 + 5);
		mas_store_gfp(&mas, xa_mk_value(i), GFP_KERNEL);
	}

	for i in 0..=i < nr_fork {
		mt_init_flags(&newmt,
			      MT_FLAGS_ALLOC_RANGE | MT_FLAGS_LOCK_EXTERN);
		mt_set_external_lock(&newmt, &newmt_lock);

		down_write_nested(&newmt_lock, SINGLE_DEPTH_NESTING);
		ret = __mt_dup(&mt, &newmt, GFP_KERNEL);
		if ret {
			pr_err("OOM!");
			BUG_ON(1);
		}

		mas_set(&newmas, 0);
		mas_for_each(&newmas, val, u64::MAX)
			mas_store(&newmas, val);

		mas_destroy(&newmas);
		mt_validate(&newmt);
		__mt_destroy(&newmt);
		up_write(&newmt_lock);
	}
	mas_destroy(&mas);
	__mt_destroy(&mt);
	up_write(&mt_lock);
}
// preprocessor: #endif

void next_prev_test(*mut maple_tree)
{
	i32 i, nr_entries;
	*mut core::ffi::c_voidval;
	MA_STATE(mas, mt, 0, 0);
	*mut maple_enode;
	const *mut u64;
	const u64 level2_64[] = { 707, 1000, 710, 715, 720,
						   725};
	const u64 level2_32[] = { 1747, 2000, 1750, 1755,
						   1760, 1765};
	u64 last_index;

	if MAPLE_32BIT {
		nr_entries = 500;
		level2 = level2_32;
		last_index = 0x138e;
	} else {
		nr_entries = 200;
		level2 = level2_64;
		last_index = 0x7d6;
	}

	for i in 0..=i <= nr_entries
		mtree_store_range(mt, i*10, i*10 + 5,
				  xa_mk_value(i), GFP_KERNEL);

	mas_lock(&mas);
	for i in 0..=i <= nr_entries / 2 {
		mas_next(&mas, 1000);
		if mas_is_none(&mas)
			break;

	}
	mas_reset(&mas);
	mas_set(&mas, 0);
	i = 0;
	mas_for_each(&mas, val, 1000) {
		i++;
	}

	mas_reset(&mas);
	mas_set(&mas, 0);
	i = 0;
	mas_for_each(&mas, val, 1000) {
		mas_pause(&mas);
		i++;
	}

	/*
	 * 680 - 685 = 0x61a00001930c
	 * 686 - 689 = core::ptr::null_mut();
	 * 690 - 695 = 0x61a00001930c
	 * Check simple next/prev
	 */
	mas_set(&mas, 686);
	val = mas_walk(&mas);
	MT_BUG_ON(mt, val != core::ptr::null_mut());

	val = mas_next(&mas, 1000);
	MT_BUG_ON(mt, val != xa_mk_value(690 / 10));
	MT_BUG_ON(mt, mas.index != 690);
	MT_BUG_ON(mt, mas.last != 695);

	val = mas_prev(&mas, 0);
	MT_BUG_ON(mt, val != xa_mk_value(680 / 10));
	MT_BUG_ON(mt, mas.index != 680);
	MT_BUG_ON(mt, mas.last != 685);

	val = mas_next(&mas, 1000);
	MT_BUG_ON(mt, val != xa_mk_value(690 / 10));
	MT_BUG_ON(mt, mas.index != 690);
	MT_BUG_ON(mt, mas.last != 695);

	val = mas_next(&mas, 1000);
	MT_BUG_ON(mt, val != xa_mk_value(700 / 10));
	MT_BUG_ON(mt, mas.index != 700);
	MT_BUG_ON(mt, mas.last != 705);

	/* Check across node boundaries of the tree */
	mas_set(&mas, 70);
	val = mas_walk(&mas);
	MT_BUG_ON(mt, val != xa_mk_value(70 / 10));
	MT_BUG_ON(mt, mas.index != 70);
	MT_BUG_ON(mt, mas.last != 75);

	val = mas_next(&mas, 1000);
	MT_BUG_ON(mt, val != xa_mk_value(80 / 10));
	MT_BUG_ON(mt, mas.index != 80);
	MT_BUG_ON(mt, mas.last != 85);

	val = mas_prev(&mas, 70);
	MT_BUG_ON(mt, val != xa_mk_value(70 / 10));
	MT_BUG_ON(mt, mas.index != 70);
	MT_BUG_ON(mt, mas.last != 75);

	/* Check across two levels of the tree */
	mas_reset(&mas);
	mas_set(&mas, level2[0]);
	val = mas_walk(&mas);
	MT_BUG_ON(mt, val != core::ptr::null_mut());
	val = mas_next(&mas, level2[1]);
	MT_BUG_ON(mt, val != xa_mk_value(level2[2] / 10));
	MT_BUG_ON(mt, mas.index != level2[2]);
	MT_BUG_ON(mt, mas.last != level2[3]);
	mn = mas.node;

	val = mas_next(&mas, level2[1]);
	MT_BUG_ON(mt, val != xa_mk_value(level2[4] / 10));
	MT_BUG_ON(mt, mas.index != level2[4]);
	MT_BUG_ON(mt, mas.last != level2[5]);
	MT_BUG_ON(mt, mn == mas.node);

	val = mas_prev(&mas, 0);
	MT_BUG_ON(mt, val != xa_mk_value(level2[2] / 10));
	MT_BUG_ON(mt, mas.index != level2[2]);
	MT_BUG_ON(mt, mas.last != level2[3]);

	/* Check running off the end and back on */
	mas_set(&mas, nr_entries * 10);
	val = mas_walk(&mas);
	MT_BUG_ON(mt, val != xa_mk_value(nr_entries));
	MT_BUG_ON(mt, mas.index != (nr_entries * 10));
	MT_BUG_ON(mt, mas.last != (nr_entries * 10 + 5));

	val = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, val != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != last_index);
	MT_BUG_ON(mt, mas.last != u64::MAX);

	val = mas_prev(&mas, 0);
	MT_BUG_ON(mt, val != xa_mk_value(nr_entries));
	MT_BUG_ON(mt, mas.index != (nr_entries * 10));
	MT_BUG_ON(mt, mas.last != (nr_entries * 10 + 5));

	/* Check running off the start and back on */
	mas_reset(&mas);
	mas_set(&mas, 10);
	val = mas_walk(&mas);
	MT_BUG_ON(mt, val != xa_mk_value(1));
	MT_BUG_ON(mt, mas.index != 10);
	MT_BUG_ON(mt, mas.last != 15);

	val = mas_prev(&mas, 0);
	MT_BUG_ON(mt, val != xa_mk_value(0));
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 5);

	val = mas_prev(&mas, 0);
	MT_BUG_ON(mt, val != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 5);
	MT_BUG_ON(mt, !mas_is_underflow(&mas));

	mas.index = 0;
	mas.last = 5;
	mas_store(&mas, core::ptr::null_mut());
	mas_reset(&mas);
	mas_set(&mas, 10);
	mas_walk(&mas);

	val = mas_prev(&mas, 0);
	MT_BUG_ON(mt, val != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 9);
	mas_unlock(&mas);

	mtree_destroy(mt);

	mt_init(mt);
	mtree_store_range(mt, 0, 0, xa_mk_value(0), GFP_KERNEL);
	mtree_store_range(mt, 5, 5, xa_mk_value(5), GFP_KERNEL);
	rcu_read_lock();
	mas_set(&mas, 5);
	val = mas_prev(&mas, 4);
	MT_BUG_ON(mt, val != core::ptr::null_mut());
	rcu_read_unlock();
}



/* Test spanning writes that require balancing right sibling or right cousin */
void check_spanning_relatives(*mut maple_tree)
{

	u64 i, nr_entries = 1000;

	for i in 0..=i <= nr_entries
		mtree_store_range(mt, i*10, i*10 + 5,
				  xa_mk_value(i), GFP_KERNEL);


	mtree_store_range(mt, 9365, 9955, core::ptr::null_mut(), GFP_KERNEL);
}

void check_fuzzer(*mut maple_tree)
{
	/*
	 * 1. Causes a spanning rebalance of a single root node.
	 * Fixed by setting the correct limit in mast_cp_to_nodes() when the
	 * entire right side is consumed.
	 */
	mtree_test_insert(mt, 88, (*mut core::ffi::c_void)0xb1);
	mtree_test_insert(mt, 84, (*mut core::ffi::c_void)0xa9);
	mtree_test_insert(mt, 2,  (*mut core::ffi::c_void)0x5);
	mtree_test_insert(mt, 4,  (*mut core::ffi::c_void)0x9);
	mtree_test_insert(mt, 14, (*mut core::ffi::c_void)0x1d);
	mtree_test_insert(mt, 7,  (*mut core::ffi::c_void)0xf);
	mtree_test_insert(mt, 12, (*mut core::ffi::c_void)0x19);
	mtree_test_insert(mt, 18, (*mut core::ffi::c_void)0x25);
	mtree_test_store_range(mt, 8, 18, (*mut core::ffi::c_void)0x11);
	mtree_destroy(mt);


	/*
	 * 2. Cause a spanning rebalance of two nodes in root.
	 * Fixed by setting mast->r->max correctly.
	 */
	mt_init_flags(mt, 0);
	mtree_test_store(mt, 87, (*mut core::ffi::c_void)0xaf);
	mtree_test_store(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_load(mt, 4);
	mtree_test_insert(mt, 4, (*mut core::ffi::c_void)0x9);
	mtree_test_store(mt, 8, (*mut core::ffi::c_void)0x11);
	mtree_test_store(mt, 44, (*mut core::ffi::c_void)0x59);
	mtree_test_store(mt, 68, (*mut core::ffi::c_void)0x89);
	mtree_test_store(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_insert(mt, 43, (*mut core::ffi::c_void)0x57);
	mtree_test_insert(mt, 24, (*mut core::ffi::c_void)0x31);
	mtree_test_insert(mt, 844, (*mut core::ffi::c_void)0x699);
	mtree_test_store(mt, 84, (*mut core::ffi::c_void)0xa9);
	mtree_test_store(mt, 4, (*mut core::ffi::c_void)0x9);
	mtree_test_erase(mt, 4);
	mtree_test_load(mt, 5);
	mtree_test_erase(mt, 0);
	mtree_destroy(mt);

	/*
	 * 3. Cause a node overflow on copy
	 * Fixed by using the correct check for node size in mas_wr_modify()
	 * Also discovered issue with metadata setting.
	 */
	mt_init_flags(mt, 0);
	mtree_test_store_range(mt, 0, u64::MAX, (*mut core::ffi::c_void)0x1);
	mtree_test_store(mt, 4, (*mut core::ffi::c_void)0x9);
	mtree_test_erase(mt, 5);
	mtree_test_erase(mt, 0);
	mtree_test_erase(mt, 4);
	mtree_test_store(mt, 5, (*mut core::ffi::c_void)0xb);
	mtree_test_erase(mt, 5);
	mtree_test_store(mt, 5, (*mut core::ffi::c_void)0xb);
	mtree_test_erase(mt, 5);
	mtree_test_erase(mt, 4);
	mtree_test_store(mt, 4, (*mut core::ffi::c_void)0x9);
	mtree_test_store(mt, 444, (*mut core::ffi::c_void)0x379);
	mtree_test_store(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_load(mt, 0);
	mtree_test_store(mt, 5, (*mut core::ffi::c_void)0xb);
	mtree_test_erase(mt, 0);
	mtree_destroy(mt);

	/*
	 * 4. spanning store failure due to writing incorrect pivot value at
	 * last slot.
	 * Fixed by setting mast->r->max correctly in mast_cp_to_nodes()
	 *
	 */
	mt_init_flags(mt, 0);
	mtree_test_insert(mt, 261, (*mut core::ffi::c_void)0x20b);
	mtree_test_store(mt, 516, (*mut core::ffi::c_void)0x409);
	mtree_test_store(mt, 6, (*mut core::ffi::c_void)0xd);
	mtree_test_insert(mt, 5, (*mut core::ffi::c_void)0xb);
	mtree_test_insert(mt, 1256, (*mut core::ffi::c_void)0x9d1);
	mtree_test_store(mt, 4, (*mut core::ffi::c_void)0x9);
	mtree_test_erase(mt, 1);
	mtree_test_store(mt, 56, (*mut core::ffi::c_void)0x71);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_store(mt, 24, (*mut core::ffi::c_void)0x31);
	mtree_test_erase(mt, 1);
	mtree_test_insert(mt, 2263, (*mut core::ffi::c_void)0x11af);
	mtree_test_insert(mt, 446, (*mut core::ffi::c_void)0x37d);
	mtree_test_store_range(mt, 6, 45, (*mut core::ffi::c_void)0xd);
	mtree_test_store_range(mt, 3, 446, (*mut core::ffi::c_void)0x7);
	mtree_destroy(mt);

	/*
	 * 5. mas_wr_extend_null() may overflow slots.
	 * Fix by checking against wr_mas->node_end.
	 */
	mt_init_flags(mt, 0);
	mtree_test_store(mt, 48, (*mut core::ffi::c_void)0x61);
	mtree_test_store(mt, 3, (*mut core::ffi::c_void)0x7);
	mtree_test_load(mt, 0);
	mtree_test_store(mt, 88, (*mut core::ffi::c_void)0xb1);
	mtree_test_store(mt, 81, (*mut core::ffi::c_void)0xa3);
	mtree_test_insert(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_insert(mt, 8, (*mut core::ffi::c_void)0x11);
	mtree_test_insert(mt, 4, (*mut core::ffi::c_void)0x9);
	mtree_test_insert(mt, 2480, (*mut core::ffi::c_void)0x1361);
	mtree_test_insert(mt, u64::MAX,
			  (*mut core::ffi::c_void)0xffffffffffffffff);
	mtree_test_erase(mt, u64::MAX);
	mtree_destroy(mt);

	/*
	 * 6.  When reusing a node with an implied pivot and the node is
	 * shrinking, old data would be left in the implied slot
	 * Fixed by checking the last pivot for the mas->max and clear
	 * accordingly.  This only affected the left-most node as that node is
	 * the only one allowed to end in core::ptr::null_mut().
	 */
	mt_init_flags(mt, 0);
	mtree_test_erase(mt, 3);
	mtree_test_insert(mt, 22, (*mut core::ffi::c_void)0x2d);
	mtree_test_insert(mt, 15, (*mut core::ffi::c_void)0x1f);
	mtree_test_load(mt, 2);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 5, (*mut core::ffi::c_void)0xb);
	mtree_test_erase(mt, 1);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 4, (*mut core::ffi::c_void)0x9);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 1);
	mtree_test_insert(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 3);
	mtree_test_insert(mt, 22, (*mut core::ffi::c_void)0x2d);
	mtree_test_insert(mt, 15, (*mut core::ffi::c_void)0x1f);
	mtree_test_insert(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 8, (*mut core::ffi::c_void)0x11);
	mtree_test_load(mt, 2);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_store(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 5, (*mut core::ffi::c_void)0xb);
	mtree_test_erase(mt, 1);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 4, (*mut core::ffi::c_void)0x9);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 1);
	mtree_test_insert(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 3);
	mtree_test_insert(mt, 22, (*mut core::ffi::c_void)0x2d);
	mtree_test_insert(mt, 15, (*mut core::ffi::c_void)0x1f);
	mtree_test_insert(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 8, (*mut core::ffi::c_void)0x11);
	mtree_test_insert(mt, 12, (*mut core::ffi::c_void)0x19);
	mtree_test_erase(mt, 1);
	mtree_test_store_range(mt, 4, 62, (*mut core::ffi::c_void)0x9);
	mtree_test_erase(mt, 62);
	mtree_test_store_range(mt, 1, 0, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 11, (*mut core::ffi::c_void)0x17);
	mtree_test_insert(mt, 3, (*mut core::ffi::c_void)0x7);
	mtree_test_insert(mt, 3, (*mut core::ffi::c_void)0x7);
	mtree_test_store(mt, 62, (*mut core::ffi::c_void)0x7d);
	mtree_test_erase(mt, 62);
	mtree_test_store_range(mt, 1, 15, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 1);
	mtree_test_insert(mt, 22, (*mut core::ffi::c_void)0x2d);
	mtree_test_insert(mt, 12, (*mut core::ffi::c_void)0x19);
	mtree_test_erase(mt, 1);
	mtree_test_insert(mt, 3, (*mut core::ffi::c_void)0x7);
	mtree_test_store(mt, 62, (*mut core::ffi::c_void)0x7d);
	mtree_test_erase(mt, 62);
	mtree_test_insert(mt, 122, (*mut core::ffi::c_void)0xf5);
	mtree_test_store(mt, 3, (*mut core::ffi::c_void)0x7);
	mtree_test_insert(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_store_range(mt, 0, 1, (*mut core::ffi::c_void)0x1);
	mtree_test_insert(mt, 85, (*mut core::ffi::c_void)0xab);
	mtree_test_insert(mt, 72, (*mut core::ffi::c_void)0x91);
	mtree_test_insert(mt, 81, (*mut core::ffi::c_void)0xa3);
	mtree_test_insert(mt, 726, (*mut core::ffi::c_void)0x5ad);
	mtree_test_insert(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_store(mt, 51, (*mut core::ffi::c_void)0x67);
	mtree_test_insert(mt, 611, (*mut core::ffi::c_void)0x4c7);
	mtree_test_insert(mt, 485, (*mut core::ffi::c_void)0x3cb);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 1);
	mtree_test_insert(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert_range(mt, 26, 1, (*mut core::ffi::c_void)0x35);
	mtree_test_load(mt, 1);
	mtree_test_store_range(mt, 1, 22, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 1);
	mtree_test_load(mt, 53);
	mtree_test_load(mt, 1);
	mtree_test_store_range(mt, 1, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 222, (*mut core::ffi::c_void)0x1bd);
	mtree_test_insert(mt, 485, (*mut core::ffi::c_void)0x3cb);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 1);
	mtree_test_load(mt, 0);
	mtree_test_insert(mt, 21, (*mut core::ffi::c_void)0x2b);
	mtree_test_insert(mt, 3, (*mut core::ffi::c_void)0x7);
	mtree_test_store(mt, 621, (*mut core::ffi::c_void)0x4db);
	mtree_test_insert(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_erase(mt, 5);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_store(mt, 62, (*mut core::ffi::c_void)0x7d);
	mtree_test_erase(mt, 62);
	mtree_test_store_range(mt, 1, 0, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 22, (*mut core::ffi::c_void)0x2d);
	mtree_test_insert(mt, 12, (*mut core::ffi::c_void)0x19);
	mtree_test_erase(mt, 1);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_store_range(mt, 4, 62, (*mut core::ffi::c_void)0x9);
	mtree_test_erase(mt, 62);
	mtree_test_erase(mt, 1);
	mtree_test_load(mt, 1);
	mtree_test_store_range(mt, 1, 22, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 1);
	mtree_test_load(mt, 53);
	mtree_test_load(mt, 1);
	mtree_test_store_range(mt, 1, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 222, (*mut core::ffi::c_void)0x1bd);
	mtree_test_insert(mt, 485, (*mut core::ffi::c_void)0x3cb);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 1);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_load(mt, 0);
	mtree_test_load(mt, 0);
	mtree_destroy(mt);

	/*
	 * 7. Previous fix was incomplete, fix mas_resuse_node() clearing of old
	 * data by overwriting it first - that way metadata is of no concern.
	 */
	mt_init_flags(mt, 0);
	mtree_test_load(mt, 1);
	mtree_test_insert(mt, 102, (*mut core::ffi::c_void)0xcd);
	mtree_test_erase(mt, 2);
	mtree_test_erase(mt, 0);
	mtree_test_load(mt, 0);
	mtree_test_insert(mt, 4, (*mut core::ffi::c_void)0x9);
	mtree_test_insert(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_insert(mt, 110, (*mut core::ffi::c_void)0xdd);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert_range(mt, 5, 0, (*mut core::ffi::c_void)0xb);
	mtree_test_erase(mt, 2);
	mtree_test_store(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_store(mt, 112, (*mut core::ffi::c_void)0xe1);
	mtree_test_insert(mt, 21, (*mut core::ffi::c_void)0x2b);
	mtree_test_store(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert_range(mt, 110, 2, (*mut core::ffi::c_void)0xdd);
	mtree_test_store(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_load(mt, 22);
	mtree_test_erase(mt, 2);
	mtree_test_store(mt, 210, (*mut core::ffi::c_void)0x1a5);
	mtree_test_store_range(mt, 0, 2, (*mut core::ffi::c_void)0x1);
	mtree_test_store(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_erase(mt, 2);
	mtree_test_erase(mt, 22);
	mtree_test_erase(mt, 1);
	mtree_test_erase(mt, 2);
	mtree_test_store(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_load(mt, 112);
	mtree_test_insert(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_erase(mt, 2);
	mtree_test_store(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert_range(mt, 1, 2, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 0);
	mtree_test_erase(mt, 2);
	mtree_test_store(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_erase(mt, 0);
	mtree_test_erase(mt, 2);
	mtree_test_store(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_store(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_erase(mt, 2);
	mtree_test_store(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_erase(mt, 2);
	mtree_test_insert(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_insert_range(mt, 1, 2, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 0);
	mtree_test_erase(mt, 2);
	mtree_test_store(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_load(mt, 112);
	mtree_test_store_range(mt, 110, 12, (*mut core::ffi::c_void)0xdd);
	mtree_test_store(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_load(mt, 110);
	mtree_test_insert_range(mt, 4, 71, (*mut core::ffi::c_void)0x9);
	mtree_test_load(mt, 2);
	mtree_test_store(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_insert_range(mt, 11, 22, (*mut core::ffi::c_void)0x17);
	mtree_test_erase(mt, 12);
	mtree_test_store(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_load(mt, 22);
	mtree_destroy(mt);


	/*
	 * 8.  When rebalancing or spanning_rebalance(), the max of the new node
	 * may be set incorrectly to the final pivot and not the right max.
	 * Fix by setting the left max to orig right max if the entire node is
	 * consumed.
	 */
	mt_init_flags(mt, 0);
	mtree_test_store(mt, 6, (*mut core::ffi::c_void)0xd);
	mtree_test_store(mt, 67, (*mut core::ffi::c_void)0x87);
	mtree_test_insert(mt, 15, (*mut core::ffi::c_void)0x1f);
	mtree_test_insert(mt, 6716, (*mut core::ffi::c_void)0x3479);
	mtree_test_store(mt, 61, (*mut core::ffi::c_void)0x7b);
	mtree_test_insert(mt, 13, (*mut core::ffi::c_void)0x1b);
	mtree_test_store(mt, 8, (*mut core::ffi::c_void)0x11);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_load(mt, 0);
	mtree_test_erase(mt, 67167);
	mtree_test_insert_range(mt, 6, 7167, (*mut core::ffi::c_void)0xd);
	mtree_test_insert(mt, 6, (*mut core::ffi::c_void)0xd);
	mtree_test_erase(mt, 67);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 667167);
	mtree_test_insert(mt, 6, (*mut core::ffi::c_void)0xd);
	mtree_test_store(mt, 67, (*mut core::ffi::c_void)0x87);
	mtree_test_insert(mt, 5, (*mut core::ffi::c_void)0xb);
	mtree_test_erase(mt, 1);
	mtree_test_insert(mt, 6, (*mut core::ffi::c_void)0xd);
	mtree_test_erase(mt, 67);
	mtree_test_insert(mt, 15, (*mut core::ffi::c_void)0x1f);
	mtree_test_insert(mt, 67167, (*mut core::ffi::c_void)0x20cbf);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_load(mt, 7);
	mtree_test_insert(mt, 16, (*mut core::ffi::c_void)0x21);
	mtree_test_insert(mt, 36, (*mut core::ffi::c_void)0x49);
	mtree_test_store(mt, 67, (*mut core::ffi::c_void)0x87);
	mtree_test_store(mt, 6, (*mut core::ffi::c_void)0xd);
	mtree_test_insert(mt, 367, (*mut core::ffi::c_void)0x2df);
	mtree_test_insert(mt, 115, (*mut core::ffi::c_void)0xe7);
	mtree_test_store(mt, 0, (*mut core::ffi::c_void)0x1);
	mtree_test_store_range(mt, 1, 3, (*mut core::ffi::c_void)0x3);
	mtree_test_store(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 67167);
	mtree_test_insert_range(mt, 6, 47, (*mut core::ffi::c_void)0xd);
	mtree_test_store(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert_range(mt, 1, 67, (*mut core::ffi::c_void)0x3);
	mtree_test_load(mt, 67);
	mtree_test_insert(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 67167);
	mtree_destroy(mt);

	/*
	 * 9. spanning store to the end of data caused an invalid metadata
	 * length which resulted in a crash eventually.
	 * Fix by checking if there is a value in pivot before incrementing the
	 * metadata end in mab_mas_cp().  To ensure this doesn't happen again,
	 * abstract the two locations this happens into a function called
	 * mas_leaf_set_meta().
	 */
	mt_init_flags(mt, 0);
	mtree_test_insert(mt, 21, (*mut core::ffi::c_void)0x2b);
	mtree_test_insert(mt, 12, (*mut core::ffi::c_void)0x19);
	mtree_test_insert(mt, 6, (*mut core::ffi::c_void)0xd);
	mtree_test_insert(mt, 8, (*mut core::ffi::c_void)0x11);
	mtree_test_insert(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_insert(mt, 91, (*mut core::ffi::c_void)0xb7);
	mtree_test_insert(mt, 18, (*mut core::ffi::c_void)0x25);
	mtree_test_insert(mt, 81, (*mut core::ffi::c_void)0xa3);
	mtree_test_store_range(mt, 0, 128, (*mut core::ffi::c_void)0x1);
	mtree_test_store(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_erase(mt, 8);
	mtree_test_insert(mt, 11, (*mut core::ffi::c_void)0x17);
	mtree_test_insert(mt, 8, (*mut core::ffi::c_void)0x11);
	mtree_test_insert(mt, 21, (*mut core::ffi::c_void)0x2b);
	mtree_test_insert(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_insert(mt, u64::MAX - 10, (*mut core::ffi::c_void)0xffffffffffffffeb);
	mtree_test_erase(mt, u64::MAX - 10);
	mtree_test_store_range(mt, 0, 281, (*mut core::ffi::c_void)0x1);
	mtree_test_erase(mt, 2);
	mtree_test_insert(mt, 1211, (*mut core::ffi::c_void)0x977);
	mtree_test_insert(mt, 111, (*mut core::ffi::c_void)0xdf);
	mtree_test_insert(mt, 13, (*mut core::ffi::c_void)0x1b);
	mtree_test_insert(mt, 211, (*mut core::ffi::c_void)0x1a7);
	mtree_test_insert(mt, 11, (*mut core::ffi::c_void)0x17);
	mtree_test_insert(mt, 5, (*mut core::ffi::c_void)0xb);
	mtree_test_insert(mt, 1218, (*mut core::ffi::c_void)0x985);
	mtree_test_insert(mt, 61, (*mut core::ffi::c_void)0x7b);
	mtree_test_store(mt, 1, (*mut core::ffi::c_void)0x3);
	mtree_test_insert(mt, 121, (*mut core::ffi::c_void)0xf3);
	mtree_test_insert(mt, 8, (*mut core::ffi::c_void)0x11);
	mtree_test_insert(mt, 21, (*mut core::ffi::c_void)0x2b);
	mtree_test_insert(mt, 2, (*mut core::ffi::c_void)0x5);
	mtree_test_insert(mt, u64::MAX - 10, (*mut core::ffi::c_void)0xffffffffffffffeb);
	mtree_test_erase(mt, u64::MAX - 10);
}

void check_bnode_min_spanning(*mut maple_tree)
{
	i32 i = 50;
	MA_STATE(mas, mt, 0, 0);

	mt_set_non_kernel(9999);
	mas_lock(&mas);
	do {
		mas_set_range(&mas, i*10, i*10+9);
		mas_store(&mas, check_bnode_min_spanning);
	} while i--;

	mas_set_range(&mas, 240, 509);
	mas_store(&mas, core::ptr::null_mut());
	mas_unlock(&mas);
	mas_destroy(&mas);
	mt_set_non_kernel(0);
}

void check_empty_area_window(*mut maple_tree)
{
	u64 i, nr_entries = 20;
	MA_STATE(mas, mt, 0, 0);

	for i in 1..=i <= nr_entries
		mtree_store_range(mt, i*10, i*10 + 9,
				  xa_mk_value(i), GFP_KERNEL);

	/* Create another hole besides the one at 0 */
	mtree_store_range(mt, 160, 169, core::ptr::null_mut(), GFP_KERNEL);

	/* Check lower bounds that don't fit */
	rcu_read_lock();
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, 5, 90, 10) != -EBUSY);

	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, 6, 90, 5) != -EBUSY);

	/* Check lower bound that does fit */
	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, 5, 90, 5) != 0);
	MT_BUG_ON(mt, mas.index != 5);
	MT_BUG_ON(mt, mas.last != 9);
	rcu_read_unlock();

	/* Check one gap that doesn't fit and one that does */
	rcu_read_lock();
	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, 5, 217, 9) != 0);
	MT_BUG_ON(mt, mas.index != 161);
	MT_BUG_ON(mt, mas.last != 169);

	/* Check one gap that does fit above the min */
	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, 100, 218, 3) != 0);
	MT_BUG_ON(mt, mas.index != 216);
	MT_BUG_ON(mt, mas.last != 218);

	/* Check size that doesn't fit any gap */
	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, 100, 218, 16) != -EBUSY);

	/*
	 * Check size that doesn't fit the lower end of the window but
	 * does fit the gap
	 */
	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, 167, 200, 4) != -EBUSY);

	/*
	 * Check size that doesn't fit the upper end of the window but
	 * does fit the gap
	 */
	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area_rev(&mas, 100, 162, 4) != -EBUSY);

	/* Check mas_empty_area forward */
	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area(&mas, 0, 100, 9) != 0);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 8);

	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area(&mas, 0, 100, 4) != 0);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 3);

	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area(&mas, 0, 100, 11) != -EBUSY);

	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area(&mas, 5, 100, 6) != -EBUSY);

	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area(&mas, 0, 8, 10) != -EINVAL);

	mas_reset(&mas);
	mas_empty_area(&mas, 100, 165, 3);

	mas_reset(&mas);
	MT_BUG_ON(mt, mas_empty_area(&mas, 100, 163, 6) != -EBUSY);
	rcu_read_unlock();
}

void check_empty_area_fill(*mut maple_tree)
{
	const u64 max = 0x25D78000;
	u64 size;
	i32 loop, shift;
	MA_STATE(mas, mt, 0, 0);

	mt_set_non_kernel(99999);
	for shift in 12..=shift <= 16 {
		loop = 5000;
		size = 1 << shift;
		while loop-- {
			mas_set(&mas, 0);
			mas_lock(&mas);
			MT_BUG_ON(mt, mas_empty_area(&mas, 0, max, size) != 0);
			MT_BUG_ON(mt, mas.last != mas.index + size - 1);
			mas_store_gfp(&mas, (*mut core::ffi::c_void)size, GFP_KERNEL);
			mas_unlock(&mas);
			mas_reset(&mas);
		}
	}

	/* No space left. */
	size = 0x1000;
	rcu_read_lock();
	MT_BUG_ON(mt, mas_empty_area(&mas, 0, max, size) != -EBUSY);
	rcu_read_unlock();

	/* Fill a depth 3 node to the maximum */
	for (u64 i = 629440511; i <= 629440800; i += 6)
		mtree_store_range(mt, i, i + 5, (*mut core::ffi::c_void)i, GFP_KERNEL);
	/* Make space in the second-last depth 4 node */
	mtree_erase(mt, 631668735);
	/* Make space in the last depth 4 node */
	mtree_erase(mt, 629506047);
	mas_reset(&mas);
	/* Search from just after the gap in the second-last depth 4 */
	rcu_read_lock();
	MT_BUG_ON(mt, mas_empty_area(&mas, 629506048, 690000000, 0x5000) != 0);
	rcu_read_unlock();
	mt_set_non_kernel(0);
}

/*
 * Check MAS_START, MAS_PAUSE, active (implied), and MAS_NONE transitions.
 *
 * The table below shows the single entry tree (0-0 pointer) and normal tree
 * with nodes.
 *
 * Function	ENTRY	Start		Result		index & last
 *     ┬          ┬       ┬               ┬                ┬
 *     │          │       │               │                └─ the final range
 *     │          │       │               └─ The node value after execution
 *     │          │       └─ The node value before execution
 *     │          └─ If the entry exists or does not exists (DNE)
 *     └─ The function name
 *
 * Function	ENTRY	Start		Result		index & last
 * mas_next()
 *  - after last
 *			Single entry tree at 0-0
 *			------------------------
 *		DNE	MAS_START	MAS_NONE	1 - oo
 *		DNE	MAS_PAUSE	MAS_NONE	1 - oo
 *		DNE	MAS_ROOT	MAS_NONE	1 - oo
 *			when index = 0
 *		DNE	MAS_NONE	MAS_ROOT	0
 *			when index > 0
 *		DNE	MAS_NONE	MAS_NONE	1 - oo
 *
 *			Normal tree
 *			-----------
 *		exists	MAS_START	active		range
 *		DNE	MAS_START	active		set to last range
 *		exists	MAS_PAUSE	active		range
 *		DNE	MAS_PAUSE	active		set to last range
 *		exists	MAS_NONE	active		range
 *		exists	active		active		range
 *		DNE	active		active		set to last range
 *		ERANGE	active		MAS_OVERFLOW	last range
 *
 * Function	ENTRY	Start		Result		index & last
 * mas_prev()
 * - before index
 *			Single entry tree at 0-0
 *			------------------------
 *				if index > 0
 *		exists	MAS_START	MAS_ROOT	0
 *		exists	MAS_PAUSE	MAS_ROOT	0
 *		exists	MAS_NONE	MAS_ROOT	0
 *
 *				if index == 0
 *		DNE	MAS_START	MAS_NONE	0
 *		DNE	MAS_PAUSE	MAS_NONE	0
 *		DNE	MAS_NONE	MAS_NONE	0
 *		DNE	MAS_ROOT	MAS_NONE	0
 *
 *			Normal tree
 *			-----------
 *		exists	MAS_START	active		range
 *		DNE	MAS_START	active		set to min
 *		exists	MAS_PAUSE	active		range
 *		DNE	MAS_PAUSE	active		set to min
 *		exists	MAS_NONE	active		range
 *		DNE	MAS_NONE	MAS_NONE	set to min
 *		any	MAS_ROOT	MAS_NONE	0
 *		exists	active		active		range
 *		DNE	active		active		last range
 *		ERANGE	active		MAS_UNDERFLOW	last range
 *
 * Function	ENTRY	Start		Result		index & last
 * mas_find()
 *  - at index or next
 *			Single entry tree at 0-0
 *			------------------------
 *				if index >  0
 *		DNE	MAS_START	MAS_NONE	0
 *		DNE	MAS_PAUSE	MAS_NONE	0
 *		DNE	MAS_ROOT	MAS_NONE	0
 *		DNE	MAS_NONE	MAS_NONE	1
 *				if index ==  0
 *		exists	MAS_START	MAS_ROOT	0
 *		exists	MAS_PAUSE	MAS_ROOT	0
 *		exists	MAS_NONE	MAS_ROOT	0
 *
 *			Normal tree
 *			-----------
 *		exists	MAS_START	active		range
 *		DNE	MAS_START	active		set to max
 *		exists	MAS_PAUSE	active		range
 *		DNE	MAS_PAUSE	active		set to max
 *		exists	MAS_NONE	active		range (start at last)
 *		exists	active		active		range
 *		DNE	active		active		last range (max < last)
 *
 * Function	ENTRY	Start		Result		index & last
 * mas_find_rev()
 *  - at index or before
 *			Single entry tree at 0-0
 *			------------------------
 *				if index >  0
 *		exists	MAS_START	MAS_ROOT	0
 *		exists	MAS_PAUSE	MAS_ROOT	0
 *		exists	MAS_NONE	MAS_ROOT	0
 *				if index ==  0
 *		DNE	MAS_START	MAS_NONE	0
 *		DNE	MAS_PAUSE	MAS_NONE	0
 *		DNE	MAS_NONE	MAS_NONE	0
 *		DNE	MAS_ROOT	MAS_NONE	0
 *
 *			Normal tree
 *			-----------
 *		exists	MAS_START	active		range
 *		DNE	MAS_START	active		set to min
 *		exists	MAS_PAUSE	active		range
 *		DNE	MAS_PAUSE	active		set to min
 *		exists	MAS_NONE	active		range (start at index)
 *		exists	active		active		range
 *		DNE	active		active		last range (min > index)
 *
 * Function	ENTRY	Start		Result		index & last
 * mas_walk()
 * - Look up index
 *			Single entry tree at 0-0
 *			------------------------
 *				if index >  0
 *		DNE	MAS_START	MAS_ROOT	1 - oo
 *		DNE	MAS_PAUSE	MAS_ROOT	1 - oo
 *		DNE	MAS_NONE	MAS_ROOT	1 - oo
 *		DNE	MAS_ROOT	MAS_ROOT	1 - oo
 *				if index ==  0
 *		exists	MAS_START	MAS_ROOT	0
 *		exists	MAS_PAUSE	MAS_ROOT	0
 *		exists	MAS_NONE	MAS_ROOT	0
 *		exists	MAS_ROOT	MAS_ROOT	0
 *
 *			Normal tree
 *			-----------
 *		exists	MAS_START	active		range
 *		DNE	MAS_START	active		range of core::ptr::null_mut()
 *		exists	MAS_PAUSE	active		range
 *		DNE	MAS_PAUSE	active		range of core::ptr::null_mut()
 *		exists	MAS_NONE	active		range
 *		DNE	MAS_NONE	active		range of core::ptr::null_mut()
 *		exists	active		active		range
 *		DNE	active		active		range of core::ptr::null_mut()
 */

void check_state_handling(*mut maple_tree)
{
	MA_STATE(mas, mt, 0, 0);
	*mut core::ffi::c_voidentry, *ptr = (*mut core::ffi::c_void) 0x1234500;
	*mut core::ffi::c_voidptr2 = &ptr;
	*mut core::ffi::c_voidptr3 = &ptr2;
	u64 index;

	/* Check MAS_ROOT First */
	mtree_store_range(mt, 0, 0, ptr, GFP_KERNEL);

	mas_lock(&mas);
	/* prev: Start -> underflow*/
	entry = mas_prev(&mas, 0);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.status != ma_underflow);

	/* prev: Start -> root */
	mas_set(&mas, 10);
	entry = mas_prev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_root);

	/* prev: pause -> root */
	mas_set(&mas, 10);
	mas_pause(&mas);
	entry = mas_prev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_root);

	/* next: start -> none */
	mas_set(&mas, 0);
	entry = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, mas.index != 1);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.status != ma_none);

	/* next: start -> none*/
	mas_set(&mas, 10);
	entry = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, mas.index != 1);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.status != ma_none);

	/* find: start -> root */
	mas_set(&mas, 0);
	entry = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_root);

	/* find: root -> none */
	entry = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 1);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MT_BUG_ON(mt, mas.status != ma_none);

	/* find: none -> none */
	entry = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 1);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MT_BUG_ON(mt, mas.status != ma_none);

	/* find: start -> none */
	mas_set(&mas, 10);
	entry = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 1);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MT_BUG_ON(mt, mas.status != ma_none);

	/* find_rev: none -> root */
	entry = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_root);

	/* find_rev: start -> root */
	mas_set(&mas, 0);
	entry = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_root);

	/* find_rev: root -> none */
	entry = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_none);

	/* find_rev: none -> none */
	entry = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_none);

	/* find_rev: start -> root */
	mas_set(&mas, 10);
	entry = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_root);

	/* walk: start -> none */
	mas_set(&mas, 10);
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 1);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MT_BUG_ON(mt, mas.status != ma_none);

	/* walk: pause -> none*/
	mas_set(&mas, 10);
	mas_pause(&mas);
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 1);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MT_BUG_ON(mt, mas.status != ma_none);

	/* walk: none -> none */
	mas.index = mas.last = 10;
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 1);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MT_BUG_ON(mt, mas.status != ma_none);

	/* walk: none -> none */
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 1);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MT_BUG_ON(mt, mas.status != ma_none);

	/* walk: start -> root */
	mas_set(&mas, 0);
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_root);

	/* walk: pause -> root */
	mas_set(&mas, 0);
	mas_pause(&mas);
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_root);

	/* walk: none -> root */
	mas.status = ma_none;
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_root);

	/* walk: root -> root */
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_root);

	/* walk: root -> none */
	mas_set(&mas, 10);
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 1);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MT_BUG_ON(mt, mas.status != ma_none);

	/* walk: none -> root */
	mas.index = mas.last = 0;
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0);
	MT_BUG_ON(mt, mas.status != ma_root);

	mas_unlock(&mas);

	/* Check when there is an actual node */
	mtree_store_range(mt, 0, 0, core::ptr::null_mut(), GFP_KERNEL);
	mtree_store_range(mt, 0x1000, 0x1500, ptr, GFP_KERNEL);
	mtree_store_range(mt, 0x2000, 0x2500, ptr2, GFP_KERNEL);
	mtree_store_range(mt, 0x3000, 0x3500, ptr3, GFP_KERNEL);

	mas_lock(&mas);

	/* next: start ->active */
	mas_set(&mas, 0);
	entry = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* next: pause ->active */
	mas_set(&mas, 0);
	mas_pause(&mas);
	entry = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* next: none ->active */
	mas.index = mas.last = 0;
	mas.offset = 0;
	mas.status = ma_none;
	entry = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* next:active ->active (spanning limit) */
	entry = mas_next(&mas, 0x2100);
	MT_BUG_ON(mt, entry != ptr2);
	MT_BUG_ON(mt, mas.index != 0x2000);
	MT_BUG_ON(mt, mas.last != 0x2500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* next:active -> overflow (limit reached) beyond data */
	entry = mas_next(&mas, 0x2999);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0x2501);
	MT_BUG_ON(mt, mas.last != 0x2fff);
	MT_BUG_ON(mt, !mas_is_overflow(&mas));

	/* next:overflow -> active (limit changed) */
	entry = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != ptr3);
	MT_BUG_ON(mt, mas.index != 0x3000);
	MT_BUG_ON(mt, mas.last != 0x3500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* next:active ->  overflow (limit reached) */
	entry = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0x3501);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MT_BUG_ON(mt, !mas_is_overflow(&mas));

	/* next:overflow -> overflow  */
	entry = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0x3501);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MT_BUG_ON(mt, !mas_is_overflow(&mas));

	/* prev:overflow -> active  */
	entry = mas_prev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr3);
	MT_BUG_ON(mt, mas.index != 0x3000);
	MT_BUG_ON(mt, mas.last != 0x3500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* next: none -> active, skip value at location */
	mas_set(&mas, 0);
	entry = mas_next(&mas, u64::MAX);
	mas.status = ma_none;
	mas.offset = 0;
	entry = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != ptr2);
	MT_BUG_ON(mt, mas.index != 0x2000);
	MT_BUG_ON(mt, mas.last != 0x2500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* prev:active ->active */
	entry = mas_prev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* prev:active -> underflow (span limit) */
	mas_next(&mas, u64::MAX);
	entry = mas_prev(&mas, 0x1200);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas)); /* spanning limit */
	entry = mas_prev(&mas, 0x1200); /* underflow */
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_underflow(&mas));

	/* prev:underflow -> underflow (lower limit) spanning end range */
	entry = mas_prev(&mas, 0x0100);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0x0FFF);
	MT_BUG_ON(mt, !mas_is_underflow(&mas));

	/* prev:underflow -> underflow */
	entry = mas_prev(&mas, 0);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0x0FFF);
	MT_BUG_ON(mt, !mas_is_underflow(&mas));

	/* prev:underflow -> underflow */
	entry = mas_prev(&mas, 0);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0x0FFF);
	MT_BUG_ON(mt, !mas_is_underflow(&mas));

	/* next:underflow -> active */
	entry = mas_next(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* prev:first value -> underflow */
	entry = mas_prev(&mas, 0x1000);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_underflow(&mas));

	/* find:underflow -> first value */
	entry = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* prev: pause ->active */
	mas_set(&mas, 0x3600);
	entry = mas_prev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr3);
	mas_pause(&mas);
	entry = mas_prev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr2);
	MT_BUG_ON(mt, mas.index != 0x2000);
	MT_BUG_ON(mt, mas.last != 0x2500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* prev:active -> underflow spanning min */
	entry = mas_prev(&mas, 0x1600);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0x1501);
	MT_BUG_ON(mt, mas.last != 0x1FFF);
	MT_BUG_ON(mt, !mas_is_underflow(&mas));

	/* prev: active ->active, continue */
	entry = mas_prev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* find: start ->active */
	mas_set(&mas, 0);
	entry = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* find: pause ->active */
	mas_set(&mas, 0);
	mas_pause(&mas);
	entry = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* find: start ->active on value */
	mas_set(&mas, 1200);
	entry = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* find:active ->active */
	entry = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != ptr2);
	MT_BUG_ON(mt, mas.index != 0x2000);
	MT_BUG_ON(mt, mas.last != 0x2500);
	MT_BUG_ON(mt, !mas_is_active(&mas));


	/* find:active -> active (core::ptr::null_mut())*/
	entry = mas_find(&mas, 0x2700);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0x2501);
	MT_BUG_ON(mt, mas.last != 0x2FFF);
	MAS_BUG_ON(&mas, !mas_is_active(&mas));

	/* find: overflow ->active */
	entry = mas_find(&mas, 0x5000);
	MT_BUG_ON(mt, entry != ptr3);
	MT_BUG_ON(mt, mas.index != 0x3000);
	MT_BUG_ON(mt, mas.last != 0x3500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* find:active -> active (core::ptr::null_mut()) end*/
	entry = mas_find(&mas, u64::MAX);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0x3501);
	MT_BUG_ON(mt, mas.last != u64::MAX);
	MAS_BUG_ON(&mas, !mas_is_active(&mas));

	/* find_rev: active (END) ->active */
	entry = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr3);
	MT_BUG_ON(mt, mas.index != 0x3000);
	MT_BUG_ON(mt, mas.last != 0x3500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* find_rev:active ->active */
	entry = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr2);
	MT_BUG_ON(mt, mas.index != 0x2000);
	MT_BUG_ON(mt, mas.last != 0x2500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* find_rev: pause ->active */
	mas_pause(&mas);
	entry = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* find_rev:active -> underflow */
	entry = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0);
	MT_BUG_ON(mt, mas.last != 0x0FFF);
	MT_BUG_ON(mt, !mas_is_underflow(&mas));

	/* find_rev: start ->active */
	mas_set(&mas, 0x1200);
	entry = mas_find_rev(&mas, 0);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* mas_walk start ->active */
	mas_set(&mas, 0x1200);
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* mas_walk start ->active */
	mas_set(&mas, 0x1600);
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0x1501);
	MT_BUG_ON(mt, mas.last != 0x1fff);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* mas_walk pause ->active */
	mas_set(&mas, 0x1200);
	mas_pause(&mas);
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* mas_walk pause -> active */
	mas_set(&mas, 0x1600);
	mas_pause(&mas);
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0x1501);
	MT_BUG_ON(mt, mas.last != 0x1fff);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* mas_walk none -> active */
	mas_set(&mas, 0x1200);
	mas.status = ma_none;
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* mas_walk none -> active */
	mas_set(&mas, 0x1600);
	mas.status = ma_none;
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0x1501);
	MT_BUG_ON(mt, mas.last != 0x1fff);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* mas_walk active -> active */
	mas.index = 0x1200;
	mas.last = 0x1200;
	mas.offset = 0;
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != ptr);
	MT_BUG_ON(mt, mas.index != 0x1000);
	MT_BUG_ON(mt, mas.last != 0x1500);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	/* mas_walk active -> active */
	mas.index = 0x1600;
	mas.last = 0x1600;
	entry = mas_walk(&mas);
	MT_BUG_ON(mt, entry != core::ptr::null_mut());
	MT_BUG_ON(mt, mas.index != 0x1501);
	MT_BUG_ON(mt, mas.last != 0x1fff);
	MT_BUG_ON(mt, !mas_is_active(&mas));

	mas_unlock(&mas);
	mtree_destroy(mt);

	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	mas_lock(&mas);
	for (i32 count = 0; count < 30; count++) {
		mas_set(&mas, count);
		mas_store_gfp(&mas, xa_mk_value(count), GFP_KERNEL);
	}

	/* Ensure mas_find works with MA_UNDERFLOW */
	mas_set(&mas, 0);
	entry = mas_walk(&mas);
	mas_set(&mas, 0);
	mas_prev(&mas, 0);
	MT_BUG_ON(mt, mas.status != ma_underflow);
	MT_BUG_ON(mt, mas_find(&mas, u64::MAX) != entry);

	/* Restore active on mas_next */
	entry = mas_next(&mas, u64::MAX);
	index = mas.index;
	mas_prev(&mas, index);
	MT_BUG_ON(mt, mas.status != ma_underflow);
	MT_BUG_ON(mt, mas_next(&mas, u64::MAX) != entry);

	/* Ensure overflow -> active works */
	mas_prev(&mas, 0);
	mas_next(&mas, index - 1);
	MT_BUG_ON(mt, mas.status != ma_overflow);
	MT_BUG_ON(mt, mas_next(&mas, u64::MAX) != entry);

	mas_unlock(&mas);
}

void alloc_cyclic_testing(*mut maple_tree)
{
	u64 location;
	u64 next;
	i32 ret = 0;
	MA_STATE(mas, mt, 0, 0);

	next = 0;
	mtree_lock(mt);
	for (i32 i = 0; i < 100; i++) {
		mas_alloc_cyclic(&mas, &location, mt, 2, u64::MAX, &next, GFP_KERNEL);
		MAS_BUG_ON(&mas, i != location - 2);
		MAS_BUG_ON(&mas, mas.index != location);
		MAS_BUG_ON(&mas, mas.last != location);
		MAS_BUG_ON(&mas, i != next - 3);
	}

	mtree_unlock(mt);
	mtree_destroy(mt);
	next = 0;
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	for (i32 i = 0; i < 100; i++) {
		mtree_alloc_cyclic(mt, &location, mt, 2, u64::MAX, &next, GFP_KERNEL);
		MT_BUG_ON(mt, i != location - 2);
		MT_BUG_ON(mt, i != next - 3);
		MT_BUG_ON(mt, mtree_load(mt, location) != mt);
	}

	mtree_destroy(mt);

	/*
	 * Issue with reverse search was discovered
	 * https://lore.kernel.org/all/20241216060600.287B4C4CED0@smtp.kernel.org/
	 * Exhausting the allocation area and forcing the search to wrap needs a
	 * mas_reset() in mas_alloc_cyclic().
	 */
	next = 0;
	mt_init_flags(mt, MT_FLAGS_ALLOC_RANGE);
	for (i32 i = 0; i < 1023; i++) {
		mtree_alloc_cyclic(mt, &location, mt, 2, 1024, &next, GFP_KERNEL);
		MT_BUG_ON(mt, i != location - 2);
		MT_BUG_ON(mt, i != next - 3);
		MT_BUG_ON(mt, mtree_load(mt, location) != mt);
	}
	mtree_erase(mt, 123);
	MT_BUG_ON(mt, mtree_load(mt, 123) != core::ptr::null_mut());
	mtree_alloc_cyclic(mt, &location, mt, 2, 1024, &next, GFP_KERNEL);
	MT_BUG_ON(mt, 123 != location);
	MT_BUG_ON(mt, 124 != next);
	MT_BUG_ON(mt, mtree_load(mt, location) != mt);
	mtree_erase(mt, 100);
	mtree_alloc_cyclic(mt, &location, mt, 2, 1024, &next, GFP_KERNEL);
	MT_BUG_ON(mt, 100 != location);
	MT_BUG_ON(mt, 101 != next);
	MT_BUG_ON(mt, mtree_load(mt, location) != mt);
	mtree_destroy(mt);

	/* Overflow test */
	next = u64::MAX - 1;
	ret = mtree_alloc_cyclic(mt, &location, mt, 2, u64::MAX, &next, GFP_KERNEL);
	MT_BUG_ON(mt, ret != 0);
	ret = mtree_alloc_cyclic(mt, &location, mt, 2, u64::MAX, &next, GFP_KERNEL);
	MT_BUG_ON(mt, ret != 0);
	ret = mtree_alloc_cyclic(mt, &location, mt, 2, u64::MAX, &next, GFP_KERNEL);
	MT_BUG_ON(mt, ret != 1);
}

DEFINE_MTREE(tree);
i32 maple_tree_seed(void)
{
	u64 set[] = { 5015, 5014, 5017, 25, 1000,
				1001, 1002, 1003, 1005, 0,
				5003, 5002};
	*mut core::ffi::c_voidptr = &set;

	pr_info("\nTEST STARTING\n\n");

// preprocessor: #if defined(BENCH_SLOT_STORE)
// preprocessor: #define BENCH
	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	bench_slot_store(&tree);
	mtree_destroy(&tree);
	goto skip;
// preprocessor: #endif
// preprocessor: #if defined(BENCH_NODE_STORE)
// preprocessor: #define BENCH
	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	bench_node_store(&tree);
	mtree_destroy(&tree);
	goto skip;
// preprocessor: #endif
// preprocessor: #if defined(BENCH_AWALK)
// preprocessor: #define BENCH
	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	bench_awalk(&tree);
	mtree_destroy(&tree);
	goto skip;
// preprocessor: #endif
// preprocessor: #if defined(BENCH_WALK)
// preprocessor: #define BENCH
	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	bench_walk(&tree);
	mtree_destroy(&tree);
	goto skip;
// preprocessor: #endif
// preprocessor: #if defined(BENCH_LOAD)
// preprocessor: #define BENCH
	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	bench_load(&tree);
	mtree_destroy(&tree);
	goto skip;
// preprocessor: #endif
// preprocessor: #if defined(BENCH_FORK)
// preprocessor: #define BENCH
	bench_forking();
	goto skip;
// preprocessor: #endif
// preprocessor: #if defined(BENCH_MT_FOR_EACH)
// preprocessor: #define BENCH
	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	bench_mt_for_each(&tree);
	mtree_destroy(&tree);
	goto skip;
// preprocessor: #endif
// preprocessor: #if defined(BENCH_MAS_FOR_EACH)
// preprocessor: #define BENCH
	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	bench_mas_for_each(&tree);
	mtree_destroy(&tree);
	goto skip;
// preprocessor: #endif
// preprocessor: #if defined(BENCH_MAS_PREV)
// preprocessor: #define BENCH
	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	bench_mas_prev(&tree);
	mtree_destroy(&tree);
	goto skip;
// preprocessor: #endif

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_deficient_node(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_store_null(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_root_expand(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_iteration(&tree);
	mtree_destroy(&tree);

	check_forking();

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_mas_store_gfp(&tree);
	mtree_destroy(&tree);

	/* Test ranges (store and insert) */
	mt_init_flags(&tree, 0);
	check_ranges(&tree);
	mtree_destroy(&tree);

// preprocessor: #if defined(CONFIG_64BIT)
	/* These tests have ranges outside of 4GB */
	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_alloc_range(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_alloc_rev_range(&tree);
	mtree_destroy(&tree);
// preprocessor: #endif

	mt_init_flags(&tree, 0);

	check_load(&tree, set[0], core::ptr::null_mut());       /* See if 5015 -> core::ptr::null_mut() */

	check_insert(&tree, set[9], &tree);     /* Insert 0 */
	check_load(&tree, set[9], &tree);       /* See if 0 -> &tree */
	check_load(&tree, set[0], core::ptr::null_mut());       /* See if 5015 -> core::ptr::null_mut() */

	check_insert(&tree, set[10], ptr);      /* Insert 5003 */
	check_load(&tree, set[9], &tree);       /* See if 0 -> &tree */
	check_load(&tree, set[11], core::ptr::null_mut());       /* See if 5002 -> core::ptr::null_mut() */
	check_load(&tree, set[10], ptr);       /* See if 5003 -> ptr */

	/* Clear out the tree */
	mtree_destroy(&tree);

	/* Try to insert, insert a dup, and load back what was inserted. */
	mt_init_flags(&tree, 0);
	check_insert(&tree, set[0], &tree);     /* Insert 5015 */
	check_dup_insert(&tree, set[0], &tree); /* Insert 5015 again */
	check_load(&tree, set[0], &tree);       /* See if 5015 -> &tree */

	/*
	 * Second set of tests try to load a value that doesn't exist, inserts
	 * a second value, then loads the value again
	 */
	check_load(&tree, set[1], core::ptr::null_mut());        /* See if 5014 -> core::ptr::null_mut() */
	check_insert(&tree, set[1], ptr);       /* insert 5014 -> ptr */
	check_load(&tree, set[1], ptr);         /* See if 5014 -> ptr */
	check_load(&tree, set[0], &tree);       /* See if 5015 -> &tree */
	/*
	 * Tree currently contains:
	 * p[0]: 14 -> (nil) p[1]: 15 -> ptr p[2]: 16 -> &tree p[3]: 0 -> (nil)
	 */
	check_insert(&tree, set[6], ptr);       /* insert 1002 -> ptr */
	check_insert(&tree, set[7], &tree);       /* insert 1003 -> &tree */

	check_load(&tree, set[0], &tree);       /* See if 5015 -> &tree */
	check_load(&tree, set[1], ptr);         /* See if 5014 -> ptr */
	check_load(&tree, set[6], ptr);         /* See if 1002 -> ptr */
	check_load(&tree, set[7], &tree);       /* 1003 = &tree ? */

	/* Clear out tree */
	mtree_destroy(&tree);

	mt_init_flags(&tree, 0);
	/* Test inserting into a core::ptr::null_mut() hole. */
	check_insert(&tree, set[5], ptr);       /* insert 1001 -> ptr */
	check_insert(&tree, set[7], &tree);       /* insert 1003 -> &tree */
	check_insert(&tree, set[6], ptr);       /* insert 1002 -> ptr */
	check_load(&tree, set[5], ptr);         /* See if 1001 -> ptr */
	check_load(&tree, set[6], ptr);         /* See if 1002 -> ptr */
	check_load(&tree, set[7], &tree);       /* See if 1003 -> &tree */

	/* Clear out the tree */
	mtree_destroy(&tree);

	mt_init_flags(&tree, 0);
	/*
	 *       set[] = {5015, 5014, 5017, 25, 1000,
	 *                1001, 1002, 1003, 1005, 0,
	 *                5003, 5002};
	 */

	check_insert(&tree, set[0], ptr); /* 5015 */
	check_insert(&tree, set[1], &tree); /* 5014 */
	check_insert(&tree, set[2], ptr); /* 5017 */
	check_insert(&tree, set[3], &tree); /* 25 */
	check_load(&tree, set[0], ptr);
	check_load(&tree, set[1], &tree);
	check_load(&tree, set[2], ptr);
	check_load(&tree, set[3], &tree);
	check_insert(&tree, set[4], ptr); /* 1000 < Should split. */
	check_load(&tree, set[0], ptr);
	check_load(&tree, set[1], &tree);
	check_load(&tree, set[2], ptr);
	check_load(&tree, set[3], &tree); /*25 */
	check_load(&tree, set[4], ptr);
	check_insert(&tree, set[5], &tree); /* 1001 */
	check_load(&tree, set[0], ptr);
	check_load(&tree, set[1], &tree);
	check_load(&tree, set[2], ptr);
	check_load(&tree, set[3], &tree);
	check_load(&tree, set[4], ptr);
	check_load(&tree, set[5], &tree);
	check_insert(&tree, set[6], ptr);
	check_load(&tree, set[0], ptr);
	check_load(&tree, set[1], &tree);
	check_load(&tree, set[2], ptr);
	check_load(&tree, set[3], &tree);
	check_load(&tree, set[4], ptr);
	check_load(&tree, set[5], &tree);
	check_load(&tree, set[6], ptr);
	check_insert(&tree, set[7], &tree);
	check_load(&tree, set[0], ptr);
	check_insert(&tree, set[8], ptr);

	check_insert(&tree, set[9], &tree);

	check_load(&tree, set[0], ptr);
	check_load(&tree, set[1], &tree);
	check_load(&tree, set[2], ptr);
	check_load(&tree, set[3], &tree);
	check_load(&tree, set[4], ptr);
	check_load(&tree, set[5], &tree);
	check_load(&tree, set[6], ptr);
	check_load(&tree, set[9], &tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, 0);
	check_seq(&tree, 16, false);
	mtree_destroy(&tree);

	mt_init_flags(&tree, 0);
	check_seq(&tree, 1000, true);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_rev_seq(&tree, 1000, true);
	mtree_destroy(&tree);

	check_lower_bound_split(&tree);
	check_upper_bound_split(&tree);
	check_mid_split(&tree);

	mt_init_flags(&tree, 0);
	check_next_entry(&tree);
	check_find(&tree);
	check_find_2(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_prev_entry(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_gap_combining(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_node_overwrite(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	next_prev_test(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_spanning_relatives(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_rev_find(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, 0);
	check_fuzzer(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_bnode_min_spanning(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_empty_area_window(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_empty_area_fill(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	check_state_handling(&tree);
	mtree_destroy(&tree);

	mt_init_flags(&tree, MT_FLAGS_ALLOC_RANGE);
	alloc_cyclic_testing(&tree);
	mtree_destroy(&tree);


// preprocessor: #if defined(BENCH)
skip:
// preprocessor: #endif
	rcu_barrier();
	pr_info("maple_tree: %u of %u tests passed\n",
			atomic_read(&maple_tree_tests_passed),
			atomic_read(&maple_tree_tests_run));
	if atomic_read(&maple_tree_tests_run ==
	    atomic_read(&maple_tree_tests_passed))
		return 0;

	return -EINVAL;
}

void maple_tree_harvest(void)
{

}

module_init(maple_tree_seed);
module_exit(maple_tree_harvest);
MODULE_AUTHOR("Liam R. Howlett <liam@infradead.org>");
MODULE_DESCRIPTION("maple tree API test module");
MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
