// SPDX-License-Identifier: GPL-2.0-only
// mm/vmscan.c:2692-5967, frozen e1d84f501551943a11f4c5271e9f5c85d7e15168.
// Shared native types and header constants come from vmscan.rs / b.
#[cfg(CONFIG_LRU_GEN)]
mod mglru {
    use super::*;
    const BLOOM_FILTER_SHIFT: u32 = 15;
    const MEMCG_LRU_NOP: i32 = 0;
    const MEMCG_LRU_HEAD: i32 = 1;
    const MEMCG_LRU_TAIL: i32 = 2;
    const MEMCG_LRU_OLD: i32 = 3;
    const MEMCG_LRU_YOUNG: i32 = 4;

    // Preserve READ_ONCE/WRITE_ONCE's native instrumentation and compiler
    // semantics, including KCSAN, at the exact original scalar width.
    trait Once: Copy {
        unsafe fn read(p: *const Self) -> Self;
        unsafe fn write(p: *mut Self, value: Self);
    }
    impl Once for ULong {
        unsafe fn read(p: *const Self) -> Self {
            read_ulong(p)
        }
        unsafe fn write(p: *mut Self, v: Self) {
            write_ulong(p, v)
        }
    }
    impl Once for Long {
        unsafe fn read(p: *const Self) -> Self {
            rust_mglru_read_long(p)
        }
        unsafe fn write(p: *mut Self, v: Self) {
            rust_mglru_write_long(p, v)
        }
    }
    impl Once for u8 {
        unsafe fn read(p: *const Self) -> Self {
            rust_mglru_read_byte(p)
        }
        unsafe fn write(p: *mut Self, v: Self) {
            rust_mglru_write_byte(p, v)
        }
    }
    impl Once for *mut ULong {
        unsafe fn read(p: *const Self) -> Self {
            rust_mglru_read_filter(p)
        }
        unsafe fn write(p: *mut Self, v: Self) {
            rust_mglru_write_filter(p, v)
        }
    }
    unsafe fn rd<T: Once>(p: *const T) -> T {
        T::read(p)
    }
    unsafe fn wr<T: Once>(p: *mut T, v: T) {
        T::write(p, v)
    }
    macro_rules! vmwarn {
        ($id:expr, $v:expr) => {
            if cfg!(CONFIG_DEBUG_VM) {
                rust_mglru_vmwarn($id, $v)
            }
        };
    }
    macro_rules! warn {
        ($id:expr, $v:expr) => {
            rust_mglru_warn($id, $v)
        };
    }
    macro_rules! folio_warn {
        ($id:expr, $v:expr, $f:expr) => {
            if cfg!(CONFIG_DEBUG_VM) {
                rust_mglru_folio_warn($id, $v, $f)
            }
        };
    }
    unsafe fn max_seq(v: *mut lruvec) -> ULong {
        rd(addr_of!((*v).lrugen.max_seq))
    }
    unsafe fn min_seq(v: *mut lruvec) -> [ULong; 2] {
        [
            rd(addr_of!((*v).lrugen.min_seq[LRU_GEN_ANON as usize])),
            rd(addr_of!((*v).lrugen.min_seq[LRU_GEN_FILE as usize])),
        ]
    }
    fn min_type(sw: i32) -> usize {
        (sw == 0) as usize
    }
    fn max_type(sw: i32) -> usize {
        (sw < SWAPPINESS_ANON_ONLY as i32) as usize
    }
    fn evictable_min_seq(s: &[ULong; 2], sw: i32) -> ULong {
        min(s[min_type(sw)], s[max_type(sw)])
    }
    fn get_memcg_gen(s: ULong) -> usize {
        (s % MEMCG_NR_GENS as ULong) as usize
    }
    fn get_memcg_bin(b: usize) -> usize {
        b % MEMCG_NR_BINS as usize
    }
    unsafe fn should_walk_mmu() -> bool {
        rust_mglru_arch_has_hw_pte_young() && rust_mglru_get_cap(LRU_GEN_MM_WALK as i32)
    }
    unsafe fn should_clear_pmd_young() -> bool {
        rust_mglru_arch_has_hw_nonleaf_pmd_young()
            && rust_mglru_get_cap(LRU_GEN_NONLEAF_YOUNG as i32)
    }

    unsafe fn get_lruvec(memcg: *mut mem_cgroup, nid: i32) -> *mut lruvec {
        let pgdat = node_data(nid);
        #[cfg(CONFIG_MEMCG)]
        if !memcg.is_null() {
            let v = addr_of_mut!((*(*(*memcg).nodeinfo.as_mut_ptr().add(nid as usize))).lruvec);
            if (*v).pgdat.is_null() {
                (*v).pgdat = pgdat;
            }
            return v;
        }
        vmwarn!(1, !mem_cgroup_disabled());
        addr_of_mut!((*pgdat).__lruvec)
    }
    unsafe fn get_swappiness(v: *mut lruvec, sc: *mut scan_control) -> i32 {
        let memcg = lruvec_memcg(v);
        let pgdat = lruvec_pgdat(v);
        let sw = sc_swappiness(sc, memcg);
        if sw == SWAPPINESS_ANON_ONLY as i32 {
            return sw;
        }
        if (*sc).may_swap() == 0 {
            return 0;
        }
        if !can_demote((*pgdat).node_id, sc, memcg)
            && mem_cgroup_get_nr_swap_pages(memcg) < MIN_LRU_BATCH as ULong
        {
            return 0;
        }
        sw
    }
    unsafe fn get_nr_gens(v: *mut lruvec, typ: usize) -> i32 {
        (*v).lrugen
            .max_seq
            .wrapping_sub((*v).lrugen.min_seq[typ])
            .wrapping_add(1) as i32
    }
    unsafe fn seq_is_valid(v: *mut lruvec) -> bool {
        for typ in 0..ANON_AND_FILE as usize {
            let n = get_nr_gens(v, typ);
            if n < MIN_NR_GENS as i32 || n > MAX_NR_GENS as i32 {
                return false;
            }
        }
        true
    }
    fn filter_gen_from_seq(seq: ULong) -> usize {
        (seq % NR_BLOOM_FILTERS as ULong) as usize
    }
    unsafe fn get_item_key(item: *mut Void) -> [i32; 2] {
        let hash = rust_mglru_hash_ptr(item, BLOOM_FILTER_SHIFT * 2);
        kernel::build_assert::build_assert!(BLOOM_FILTER_SHIFT * 2 <= u32::BITS);
        [
            (hash & ((1 << BLOOM_FILTER_SHIFT) - 1)) as i32,
            (hash >> BLOOM_FILTER_SHIFT) as i32,
        ]
    }
    unsafe fn test_bloom_filter(state: *mut lru_gen_mm_state, seq: ULong, item: *mut Void) -> bool {
        let filter = rd(addr_of!((*state).filters[filter_gen_from_seq(seq)]));
        if filter.is_null() {
            return true;
        }
        let key = get_item_key(item);
        rust_mglru_test_bit(key[0], filter) && rust_mglru_test_bit(key[1], filter)
    }
    unsafe fn update_bloom_filter(state: *mut lru_gen_mm_state, seq: ULong, item: *mut Void) {
        let filter = rd(addr_of!((*state).filters[filter_gen_from_seq(seq)]));
        if filter.is_null() {
            return;
        }
        let key = get_item_key(item);
        for k in key {
            if !rust_mglru_test_bit(k, filter) {
                rust_mglru_set_bit(k, filter);
            }
        }
    }
    unsafe fn reset_bloom_filter(state: *mut lru_gen_mm_state, seq: ULong) {
        let gen = filter_gen_from_seq(seq);
        let filter = (*state).filters[gen];
        if !filter.is_null() {
            bitmap_clear(filter, 0, 1 << BLOOM_FILTER_SHIFT);
            return;
        }
        let filter = bitmap_zalloc(
            1 << BLOOM_FILTER_SHIFT,
            (__GFP_HIGH | __GFP_NOMEMALLOC | __GFP_NOWARN) as _,
        );
        wr(addr_of_mut!((*state).filters[gen]), filter);
    }
    #[cfg(CONFIG_LRU_GEN_WALKS_MMU)]
    unsafe fn get_mm_list(memcg: *mut mem_cgroup) -> *mut lru_gen_mm_list {
        #[cfg(CONFIG_MEMCG)]
        if !memcg.is_null() {
            return addr_of_mut!((*memcg).mm_list);
        }
        vmwarn!(2, !mem_cgroup_disabled());
        rust_mglru_fallback_mm_list()
    }
    #[cfg(not(CONFIG_LRU_GEN_WALKS_MMU))]
    unsafe fn get_mm_list(_: *mut mem_cgroup) -> *mut lru_gen_mm_list {
        null_mut()
    }
    #[cfg(CONFIG_LRU_GEN_WALKS_MMU)]
    unsafe fn get_mm_state(v: *mut lruvec) -> *mut lru_gen_mm_state {
        addr_of_mut!((*v).mm_state)
    }
    #[cfg(not(CONFIG_LRU_GEN_WALKS_MMU))]
    unsafe fn get_mm_state(_: *mut lruvec) -> *mut lru_gen_mm_state {
        null_mut()
    }
    #[cfg(CONFIG_LRU_GEN_WALKS_MMU)]
    unsafe fn get_next_mm(walk: *mut lru_gen_mm_walk) -> *mut mm_struct {
        let pgdat = lruvec_pgdat((*walk).lruvec);
        let state = get_mm_state((*walk).lruvec);
        let mm = rust_mglru_mm_from_list((*state).head);
        let key = (*pgdat).node_id % (size_of::<ULong>() * 8) as i32;
        if !(*walk).force_scan && !rust_mglru_test_bit(key, rust_mglru_mm_bitmap_ptr(mm)) {
            return null_mut();
        }
        rust_mglru_clear_bit(key, rust_mglru_mm_bitmap_ptr(mm));
        mmgrab(mm);
        mm
    }
    #[cfg(not(CONFIG_LRU_GEN_WALKS_MMU))]
    unsafe fn get_next_mm(_: *mut lru_gen_mm_walk) -> *mut mm_struct {
        null_mut()
    }
    #[cfg(CONFIG_LRU_GEN_WALKS_MMU)]
    #[no_mangle]
    pub unsafe extern "C" fn lru_gen_add_mm(mm: *mut mm_struct) {
        let memcg = get_mem_cgroup_from_mm(mm);
        let list = get_mm_list(memcg);
        vmwarn!(3, !list_empty(rust_mglru_mm_list_ptr(mm)));
        #[cfg(CONFIG_MEMCG)]
        {
            vmwarn!(4, !rust_mglru_mm_memcg(mm).is_null());
            rust_mglru_mm_set_memcg(mm, memcg);
        }
        spin_lock(addr_of_mut!((*list).lock));
        let mut nid = rust_mglru_first_memory_node();
        while nid < MAX_NUMNODES as i32 {
            let state = get_mm_state(get_lruvec(memcg, nid));
            if (*state).tail == addr_of_mut!((*list).fifo) {
                (*state).tail = rust_mglru_mm_list_ptr(mm);
            }
            nid = rust_mglru_next_memory_node(nid);
        }
        list_add_tail(rust_mglru_mm_list_ptr(mm), addr_of_mut!((*list).fifo));
        spin_unlock(addr_of_mut!((*list).lock));
    }
    #[cfg(CONFIG_LRU_GEN_WALKS_MMU)]
    #[no_mangle]
    pub unsafe extern "C" fn lru_gen_del_mm(mm: *mut mm_struct) {
        if list_empty(rust_mglru_mm_list_ptr(mm)) {
            return;
        }
        #[cfg(CONFIG_MEMCG)]
        let memcg = rust_mglru_mm_memcg(mm);
        #[cfg(not(CONFIG_MEMCG))]
        let memcg = null_mut();
        let list = get_mm_list(memcg);
        spin_lock(addr_of_mut!((*list).lock));
        let mut nid = rust_mglru_first_node();
        while nid < MAX_NUMNODES as i32 {
            let state = get_mm_state(get_lruvec(memcg, nid));
            if (*state).head == rust_mglru_mm_list_ptr(mm) {
                (*state).head = (*(*state).head).prev;
            }
            if (*state).tail == rust_mglru_mm_list_ptr(mm) {
                (*state).tail = (*(*state).tail).next;
            }
            nid = rust_mglru_next_node(nid);
        }
        list_del_init(rust_mglru_mm_list_ptr(mm));
        spin_unlock(addr_of_mut!((*list).lock));
        #[cfg(CONFIG_MEMCG)]
        {
            mem_cgroup_put(rust_mglru_mm_memcg(mm));
            rust_mglru_mm_set_memcg(mm, null_mut());
        }
    }
    #[cfg(all(CONFIG_LRU_GEN_WALKS_MMU, CONFIG_MEMCG))]
    #[no_mangle]
    pub unsafe extern "C" fn lru_gen_migrate_mm(mm: *mut mm_struct) {
        let task = rust_mglru_mm_owner_protected(mm);
        vmwarn!(5, (*task).mm != mm);
        rust_mglru_assert_task_alloc_lock(task);
        if mem_cgroup_disabled() || rust_mglru_mm_memcg(mm).is_null() {
            return;
        }
        rcu_read_lock();
        let memcg = mem_cgroup_from_task(task);
        rcu_read_unlock();
        if memcg == rust_mglru_mm_memcg(mm) {
            return;
        }
        vmwarn!(6, list_empty(rust_mglru_mm_list_ptr(mm)));
        lru_gen_del_mm(mm);
        lru_gen_add_mm(mm);
    }
    unsafe fn reset_mm_stats(walk: *mut lru_gen_mm_walk, last: bool) {
        let v = (*walk).lruvec;
        let state = get_mm_state(v);
        rust_mglru_assert_mm_list_lock(get_mm_list(lruvec_memcg(v)));
        let hist = lru_hist_from_seq((*walk).seq) as usize;
        for i in 0..NR_MM_STATS as usize {
            wr(
                addr_of_mut!((*state).stats[hist][i]),
                (*state).stats[hist][i].wrapping_add((*walk).mm_stats[i] as ULong),
            );
            (*walk).mm_stats[i] = 0;
        }
        if NR_HIST_GENS > 1 && last {
            let hist = lru_hist_from_seq((*walk).seq.wrapping_add(1)) as usize;
            for i in 0..NR_MM_STATS as usize {
                wr(addr_of_mut!((*state).stats[hist][i]), 0);
            }
        }
    }
    unsafe fn iterate_mm_list(walk: *mut lru_gen_mm_walk, iter: *mut *mut mm_struct) -> bool {
        let mut first = false;
        let mut last = false;
        let mut mm = null_mut();
        let v = (*walk).lruvec;
        let list = get_mm_list(lruvec_memcg(v));
        let state = get_mm_state(v);
        spin_lock(addr_of_mut!((*list).lock));
        vmwarn!(7, (*state).seq.wrapping_add(1) < (*walk).seq);
        if (*walk).seq > (*state).seq {
            if (*state).head.is_null() {
                (*state).head = addr_of_mut!((*list).fifo);
            }
            if (*state).head == addr_of_mut!((*list).fifo) {
                first = true;
            }
            loop {
                (*state).head = (*(*state).head).next;
                if (*state).head == addr_of_mut!((*list).fifo) {
                    wr(addr_of_mut!((*state).seq), (*state).seq.wrapping_add(1));
                    last = true;
                    break;
                }
                if (*state).tail.is_null() || (*state).tail == (*state).head {
                    (*state).tail = (*(*state).head).next;
                    (*walk).force_scan = true;
                }
                mm = get_next_mm(walk);
                if !mm.is_null() {
                    break;
                }
            }
        }
        if !(*iter).is_null() || last {
            reset_mm_stats(walk, last);
        }
        spin_unlock(addr_of_mut!((*list).lock));
        if !mm.is_null() && first {
            reset_bloom_filter(state, (*walk).seq.wrapping_add(1));
        }
        if !(*iter).is_null() {
            mmdrop(*iter);
        }
        *iter = mm;
        last
    }
    unsafe fn iterate_mm_list_nowalk(v: *mut lruvec, seq: ULong) -> bool {
        let mut success = false;
        let list = get_mm_list(lruvec_memcg(v));
        let state = get_mm_state(v);
        spin_lock(addr_of_mut!((*list).lock));
        vmwarn!(8, (*state).seq.wrapping_add(1) < seq);
        if seq > (*state).seq {
            (*state).head = null_mut();
            (*state).tail = null_mut();
            wr(addr_of_mut!((*state).seq), (*state).seq.wrapping_add(1));
            success = true;
        }
        spin_unlock(addr_of_mut!((*list).lock));
        success
    }
    unsafe fn read_ctrl_pos(
        v: *mut lruvec,
        typ: usize,
        tier: usize,
        gain: i32,
        pos: *mut ctrl_pos,
    ) {
        let g = addr_of_mut!((*v).lrugen);
        let hist = lru_hist_from_seq((*g).min_seq[typ]) as usize;
        (*pos).gain = gain;
        (*pos).refaulted = 0;
        (*pos).total = 0;
        for i in tier % MAX_NR_TIERS as usize..=min(tier, MAX_NR_TIERS as usize - 1) {
            (*pos).refaulted = (*pos)
                .refaulted
                .wrapping_add((*g).avg_refaulted[typ][i])
                .wrapping_add(atomic_long_read(addr_of!((*g).refaulted[hist][typ][i])) as ULong);
            (*pos).total = (*pos)
                .total
                .wrapping_add((*g).avg_total[typ][i])
                .wrapping_add((*g).protected[hist][typ][i])
                .wrapping_add(atomic_long_read(addr_of!((*g).evicted[hist][typ][i])) as ULong);
        }
    }
    unsafe fn reset_ctrl_pos(v: *mut lruvec, typ: usize, carryover: bool) {
        let g = addr_of_mut!((*v).lrugen);
        let clear = if carryover {
            NR_HIST_GENS == 1
        } else {
            NR_HIST_GENS > 1
        };
        let seq = if carryover {
            (*g).min_seq[typ]
        } else {
            (*g).max_seq.wrapping_add(1)
        };
        rust_mglru_assert_lru_lock(v);
        if !carryover && !clear {
            return;
        }
        let hist = lru_hist_from_seq(seq) as usize;
        for tier in 0..MAX_NR_TIERS as usize {
            if carryover {
                let sum = (*g).avg_refaulted[typ][tier].wrapping_add(atomic_long_read(addr_of!(
                    (*g).refaulted[hist][typ][tier]
                )) as ULong);
                wr(addr_of_mut!((*g).avg_refaulted[typ][tier]), sum / 2);
                let sum = (*g).avg_total[typ][tier]
                    .wrapping_add((*g).protected[hist][typ][tier])
                    .wrapping_add(
                        atomic_long_read(addr_of!((*g).evicted[hist][typ][tier])) as ULong
                    );
                wr(addr_of_mut!((*g).avg_total[typ][tier]), sum / 2);
            }
            if clear {
                atomic_long_set(addr_of_mut!((*g).refaulted[hist][typ][tier]), 0);
                atomic_long_set(addr_of_mut!((*g).evicted[hist][typ][tier]), 0);
                wr(addr_of_mut!((*g).protected[hist][typ][tier]), 0);
            }
        }
    }
    unsafe fn positive_ctrl_err(sp: *const ctrl_pos, pv: *const ctrl_pos) -> bool {
        (*pv).refaulted < MIN_LRU_BATCH as ULong
            || (*pv)
                .refaulted
                .wrapping_mul((*sp).total.wrapping_add(MIN_LRU_BATCH as ULong))
                .wrapping_mul((*sp).gain as ULong)
                <= (*sp)
                    .refaulted
                    .wrapping_add(1)
                    .wrapping_mul((*pv).total)
                    .wrapping_mul((*pv).gain as ULong)
    }

    include!("vmscan_mglru_aging.rs");
    include!("vmscan_mglru_memcg.rs");
    include!("vmscan_mglru_eviction.rs");
    include!("vmscan_mglru_control.rs");
}
#[cfg(CONFIG_LRU_GEN)]
use mglru::{lru_gen_age_node, lru_gen_shrink_lruvec, lru_gen_shrink_node};
#[cfg(not(CONFIG_LRU_GEN))]
#[inline(always)]
unsafe fn lru_gen_age_node(_: *mut pglist_data, _: *mut scan_control) {
    kernel::build_assert::build_assert!(false, "original !CONFIG_LRU_GEN BUILD_BUG");
}
#[cfg(not(CONFIG_LRU_GEN))]
#[inline(always)]
unsafe fn lru_gen_shrink_lruvec(_: *mut lruvec, _: *mut scan_control) {
    kernel::build_assert::build_assert!(false, "original !CONFIG_LRU_GEN BUILD_BUG");
}
#[cfg(not(CONFIG_LRU_GEN))]
#[inline(always)]
unsafe fn lru_gen_shrink_node(_: *mut pglist_data, _: *mut scan_control) {
    kernel::build_assert::build_assert!(false, "original !CONFIG_LRU_GEN BUILD_BUG");
}
