// SPDX-License-Identifier: GPL-2.0-only
// Original mm/vmscan.c memcg LRU lifecycle and reparenting.
unsafe fn lru_gen_rotate_memcg(v: *mut lruvec, op: i32) {
    let bin = get_random_u32_below(MEMCG_NR_BINS as u32) as usize;
    let pgdat = lruvec_pgdat(v);
    let lru = addr_of_mut!((*pgdat).memcg_lru);
    let flags = rust_mglru_spin_lock_irqsave(addr_of_mut!((*lru).lock));
    vmwarn!(
        27,
        rust_mglru_hlist_nulls_unhashed(addr_of!((*v).lrugen.list))
    );
    let mut seg = 0;
    let old = (*v).lrugen.gen as usize;
    let mut new = old;
    match op {
        MEMCG_LRU_HEAD => seg = MEMCG_LRU_HEAD,
        MEMCG_LRU_TAIL => seg = MEMCG_LRU_TAIL,
        MEMCG_LRU_OLD => new = get_memcg_gen((*lru).seq),
        MEMCG_LRU_YOUNG => new = get_memcg_gen((*lru).seq.wrapping_add(1)),
        _ => {
            vmwarn!(28, true);
        }
    }
    wr(addr_of_mut!((*v).lrugen.seg), seg as u8);
    wr(addr_of_mut!((*v).lrugen.gen), new as u8);
    rust_mglru_hlist_nulls_del_rcu(addr_of_mut!((*v).lrugen.list));
    if op == MEMCG_LRU_HEAD || op == MEMCG_LRU_OLD {
        rust_mglru_hlist_nulls_add_head_rcu(
            addr_of_mut!((*v).lrugen.list),
            addr_of_mut!((*lru).fifo[new][bin]),
        );
    } else {
        rust_mglru_hlist_nulls_add_tail_rcu(
            addr_of_mut!((*v).lrugen.list),
            addr_of_mut!((*lru).fifo[new][bin]),
        );
    }
    (*lru).nr_memcgs[old] = (*lru).nr_memcgs[old].wrapping_sub(1);
    (*lru).nr_memcgs[new] = (*lru).nr_memcgs[new].wrapping_add(1);
    if (*lru).nr_memcgs[old] == 0 && old == get_memcg_gen((*lru).seq) {
        wr(addr_of_mut!((*lru).seq), (*lru).seq.wrapping_add(1));
    }
    rust_mglru_spin_unlock_irqrestore(addr_of_mut!((*lru).lock), flags);
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
pub unsafe extern "C" fn lru_gen_online_memcg(memcg: *mut mem_cgroup) {
    let bin = get_random_u32_below(MEMCG_NR_BINS as u32) as usize;
    let mut nid = rust_mglru_first_node();
    while nid < MAX_NUMNODES as i32 {
        let pgdat = node_data(nid);
        let v = get_lruvec(memcg, nid);
        let lru = addr_of_mut!((*pgdat).memcg_lru);
        spin_lock_irq(addr_of_mut!((*lru).lock));
        vmwarn!(
            29,
            !rust_mglru_hlist_nulls_unhashed(addr_of!((*v).lrugen.list))
        );
        let gen = get_memcg_gen((*lru).seq);
        (*v).lrugen.gen = gen as u8;
        rust_mglru_hlist_nulls_add_tail_rcu(
            addr_of_mut!((*v).lrugen.list),
            addr_of_mut!((*lru).fifo[gen][bin]),
        );
        (*lru).nr_memcgs[gen] = (*lru).nr_memcgs[gen].wrapping_add(1);
        spin_unlock_irq(addr_of_mut!((*lru).lock));
        nid = rust_mglru_next_node(nid);
    }
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
pub unsafe extern "C" fn lru_gen_offline_memcg(memcg: *mut mem_cgroup) {
    let mut nid = rust_mglru_first_node();
    while nid < MAX_NUMNODES as i32 {
        lru_gen_rotate_memcg(get_lruvec(memcg, nid), MEMCG_LRU_OLD);
        nid = rust_mglru_next_node(nid);
    }
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
pub unsafe extern "C" fn lru_gen_release_memcg(memcg: *mut mem_cgroup) {
    let mut nid = rust_mglru_first_node();
    while nid < MAX_NUMNODES as i32 {
        let pgdat = node_data(nid);
        let v = get_lruvec(memcg, nid);
        let lru = addr_of_mut!((*pgdat).memcg_lru);
        spin_lock_irq(addr_of_mut!((*lru).lock));
        if !rust_mglru_hlist_nulls_unhashed(addr_of!((*v).lrugen.list)) {
            let gen = (*v).lrugen.gen as usize;
            rust_mglru_hlist_nulls_del_init_rcu(addr_of_mut!((*v).lrugen.list));
            (*lru).nr_memcgs[gen] = (*lru).nr_memcgs[gen].wrapping_sub(1);
            if (*lru).nr_memcgs[gen] == 0 && gen == get_memcg_gen((*lru).seq) {
                wr(addr_of_mut!((*lru).seq), (*lru).seq.wrapping_add(1));
            }
        }
        spin_unlock_irq(addr_of_mut!((*lru).lock));
        nid = rust_mglru_next_node(nid);
    }
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
pub unsafe extern "C" fn lru_gen_soft_reclaim(memcg: *mut mem_cgroup, nid: i32) {
    let v = get_lruvec(memcg, nid);
    if rd(addr_of!((*v).lrugen.seg)) as i32 != MEMCG_LRU_HEAD {
        lru_gen_rotate_memcg(v, MEMCG_LRU_HEAD);
    }
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
pub unsafe extern "C" fn recheck_lru_gen_max_memcg(memcg: *mut mem_cgroup, nid: i32) -> bool {
    let v = get_lruvec(memcg, nid);
    for typ in 0..ANON_AND_FILE as usize {
        if get_nr_gens(v, typ) != MAX_NR_GENS as i32 {
            return false;
        }
    }
    true
}
#[cfg(CONFIG_MEMCG)]
unsafe fn try_to_inc_max_seq_nowalk(memcg: *mut mem_cgroup, v: *mut lruvec) {
    let list = get_mm_list(memcg);
    let state = get_mm_state(v);
    let sw = mem_cgroup_swappiness(memcg);
    let max = max_seq(v);
    let mut success = false;
    if !state.is_null() {
        spin_lock(addr_of_mut!((*list).lock));
        vmwarn!(30, (*state).seq.wrapping_add(1) < max);
        if max > (*state).seq {
            wr(addr_of_mut!((*state).seq), (*state).seq.wrapping_add(1));
            success = true;
        }
        spin_unlock(addr_of_mut!((*list).lock));
    } else {
        success = true;
    }
    if success {
        inc_max_seq(v, max, sw);
    }
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
pub unsafe extern "C" fn max_lru_gen_memcg(memcg: *mut mem_cgroup, nid: i32) {
    let v = get_lruvec(memcg, nid);
    for typ in 0..ANON_AND_FILE as usize {
        while get_nr_gens(v, typ) < MAX_NR_GENS as i32 {
            try_to_inc_max_seq_nowalk(memcg, v);
            cond_resched();
        }
    }
}
#[cfg(CONFIG_MEMCG)]
unsafe fn __lru_gen_reparent_memcg(
    child: *mut lruvec,
    parent: *mut lruvec,
    zone: usize,
    typ: usize,
) {
    let c = addr_of_mut!((*child).lrugen);
    let p = addr_of_mut!((*parent).lrugen);
    let lru = typ as u32 * LRU_INACTIVE_FILE as u32;
    for i in 0..get_nr_gens(child, typ) {
        let gen = lru_gen_from_seq((*c).max_seq.wrapping_sub(i as ULong)) as usize;
        let nr = (*c).nr_pages[gen][typ][zone];
        let ca = if lru_gen_is_active(child, gen as i32) {
            LRU_ACTIVE as u32
        } else {
            0
        };
        let pa = if lru_gen_is_active(parent, gen as i32) {
            LRU_ACTIVE as u32
        } else {
            0
        };
        list_splice_tail_init(
            addr_of_mut!((*c).folios[gen][typ][zone]),
            addr_of_mut!((*p).folios[gen][typ][zone]),
        );
        wr(addr_of_mut!((*c).nr_pages[gen][typ][zone]), 0);
        wr(
            addr_of_mut!((*p).nr_pages[gen][typ][zone]),
            (*p).nr_pages[gen][typ][zone].wrapping_add(nr),
        );
        if lru_gen_is_active(child, gen as i32) != lru_gen_is_active(parent, gen as i32) {
            __update_lru_size(child, (lru + ca) as _, zone as _, nr.wrapping_neg());
            __update_lru_size(parent, (lru + pa) as _, zone as _, nr);
        }
    }
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
pub unsafe extern "C" fn lru_gen_reparent_memcg(
    memcg: *mut mem_cgroup,
    parent: *mut mem_cgroup,
    nid: i32,
) {
    let c = get_lruvec(memcg, nid);
    let p = get_lruvec(parent, nid);
    let pgdat = node_data(nid);
    for zone in 0..MAX_NR_ZONES as usize {
        if !managed_zone((*pgdat).node_zones.as_ptr().add(zone)) {
            continue;
        }
        for typ in 0..ANON_AND_FILE as usize {
            __lru_gen_reparent_memcg(c, p, zone, typ);
        }
    }
    for lru in 0..NR_LRU_LISTS as u32 {
        for zone in 0..MAX_NR_ZONES as usize {
            if !managed_zone((*pgdat).node_zones.as_ptr().add(zone)) {
                continue;
            }
            let size = mem_cgroup_get_zone_lru_size(c, lru as _, zone as _);
            if size == 0 {
                continue;
            }
            mem_cgroup_update_lru_size(p, lru as _, zone as _, size as Long);
            mem_cgroup_update_lru_size(c, lru as _, zone as _, (size as Long).wrapping_neg());
        }
    }
}
