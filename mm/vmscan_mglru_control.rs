// SPDX-License-Identifier: GPL-2.0-only
// Original state transition, sysfs/debugfs, and initialization algorithms.
unsafe fn state_is_valid(v: *mut lruvec) -> bool {
    let g = addr_of!((*v).lrugen);
    if (*g).enabled {
        for lru in 0..LRU_UNEVICTABLE as usize {
            if !list_empty(addr_of!((*v).lists[lru])) {
                return false;
            }
        }
    } else {
        for gen in 0..MAX_NR_GENS as usize {
            for typ in 0..ANON_AND_FILE as usize {
                for zone in 0..MAX_NR_ZONES as usize {
                    if !list_empty(addr_of!((*g).folios[gen][typ][zone])) {
                        return false;
                    }
                }
            }
        }
    }
    true
}
unsafe fn fill_evictable(v: *mut lruvec) -> bool {
    let mut remaining = MAX_LRU_BATCH as i32;
    for lru in 0..LRU_UNEVICTABLE as usize {
        let typ = rust_mglru_is_file_lru(lru as i32);
        let active = rust_mglru_is_active_lru(lru as i32);
        let head = addr_of_mut!((*v).lists[lru]);
        while !list_empty(head) {
            let f = lru_to_folio(head);
            folio_warn!(13, folio_test_unevictable(f), f);
            folio_warn!(14, folio_test_active(f) != active, f);
            folio_warn!(15, folio_is_file_lru(f) as i32 != typ, f);
            folio_warn!(16, folio_lru_gen(f) != -1, f);
            lruvec_del_folio(v, f);
            let success = lru_gen_add_folio(v, f, false);
            vmwarn!(36, !success);
            remaining -= 1;
            if remaining == 0 {
                return false;
            }
        }
    }
    true
}
unsafe fn drain_evictable(v: *mut lruvec) -> bool {
    let mut remaining = MAX_LRU_BATCH as i32;
    for gen in 0..MAX_NR_GENS as usize {
        for typ in 0..ANON_AND_FILE as usize {
            for zone in 0..MAX_NR_ZONES as usize {
                let head = addr_of_mut!((*v).lrugen.folios[gen][typ][zone]);
                while !list_empty(head) {
                    let f = lru_to_folio(head);
                    folio_warn!(17, folio_test_unevictable(f), f);
                    folio_warn!(18, folio_test_active(f), f);
                    folio_warn!(19, folio_is_file_lru(f) as usize != typ, f);
                    folio_warn!(20, folio_zonenum(f) as usize != zone, f);
                    let success = lru_gen_del_folio(v, f, false);
                    vmwarn!(37, !success);
                    lruvec_add_folio(v, f);
                    remaining -= 1;
                    if remaining == 0 {
                        return false;
                    }
                }
            }
        }
    }
    true
}
unsafe fn lru_gen_change_state(enabled: bool) {
    cgroup_lock();
    cpus_read_lock();
    get_online_mems();
    mutex_lock(rust_mglru_state_mutex());
    if enabled != lru_gen_enabled() {
        rust_mglru_switch_enable_cpuslocked();
        rust_mglru_cap_set_cpuslocked(LRU_GEN_CORE as i32, enabled);
        let mut memcg = mem_cgroup_iter(null_mut(), null_mut(), null_mut());
        loop {
            let mut nid = rust_mglru_first_node();
            while nid < MAX_NUMNODES as i32 {
                let v = get_lruvec(memcg, nid);
                lruvec_lock_irq(v);
                vmwarn!(38, !seq_is_valid(v));
                vmwarn!(39, !state_is_valid(v));
                (*v).lrugen.enabled = enabled;
                while !(if enabled {
                    fill_evictable(v)
                } else {
                    drain_evictable(v)
                }) {
                    lruvec_unlock_irq(v);
                    cond_resched();
                    lruvec_lock_irq(v);
                }
                lruvec_unlock_irq(v);
                nid = rust_mglru_next_node(nid);
            }
            cond_resched();
            memcg = mem_cgroup_iter(null_mut(), memcg, null_mut());
            if memcg.is_null() {
                break;
            }
        }
        rust_mglru_switch_disable_cpuslocked();
    }
    mutex_unlock(rust_mglru_state_mutex());
    put_online_mems();
    cpus_read_unlock();
    cgroup_unlock();
}
#[export_name = "rust_mglru_min_ttl_ms_show"]
unsafe extern "C" fn min_ttl_ms_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut CChar,
) -> isize {
    sysfs_emit(
        buf,
        b"%u\n\0".as_ptr().cast(),
        rust_mglru_jiffies_to_msecs(rd(rust_mglru_min_ttl_ptr())),
    ) as isize
}
#[export_name = "rust_mglru_min_ttl_ms_store"]
unsafe extern "C" fn min_ttl_ms_store(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *const CChar,
    len: usize,
) -> isize {
    let mut msecs = 0;
    if kstrtouint(buf, 0, &mut msecs) != 0 {
        return -(EINVAL as isize);
    }
    wr(rust_mglru_min_ttl_ptr(), rust_mglru_msecs_to_jiffies(msecs));
    len as isize
}
#[export_name = "rust_mglru_enabled_show"]
unsafe extern "C" fn enabled_show(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *mut CChar,
) -> isize {
    let mut caps = 0u32;
    if rust_mglru_get_cap(LRU_GEN_CORE as i32) {
        caps |= 1 << LRU_GEN_CORE;
    }
    if should_walk_mmu() {
        caps |= 1 << LRU_GEN_MM_WALK;
    }
    if should_clear_pmd_young() {
        caps |= 1 << LRU_GEN_NONLEAF_YOUNG;
    }
    sysfs_emit(buf, b"0x%04x\n\0".as_ptr().cast(), caps) as isize
}
#[export_name = "rust_mglru_enabled_store"]
unsafe extern "C" fn enabled_store(
    _kobj: *mut kobject,
    _attr: *mut kobj_attribute,
    buf: *const CChar,
    len: usize,
) -> isize {
    let mut caps = 0u32;
    let ch = rust_mglru_tolower(*buf as i32);
    if ch == b'n' as i32 {
        caps = 0;
    } else if ch == b'y' as i32 {
        caps = u32::MAX;
    } else if kstrtouint(buf, 0, &mut caps) != 0 {
        return -(EINVAL as isize);
    }
    for i in 0..NR_LRU_GEN_CAPS as u32 {
        let enabled = caps & (1 << i) != 0;
        if i == LRU_GEN_CORE as u32 {
            lru_gen_change_state(enabled);
        } else {
            rust_mglru_cap_set(i as i32, enabled);
        }
    }
    len as isize
}
#[export_name = "rust_mglru_seq_start"]
unsafe extern "C" fn lru_gen_seq_start(m: *mut seq_file, pos: *mut loff_t) -> *mut Void {
    let mut skip = *pos;
    (*m).private = kvmalloc(PATH_MAX as usize, GFP_KERNEL as _);
    if (*m).private.is_null() {
        return rust_mglru_err_ptr(-(ENOMEM as Long));
    }
    let mut memcg = mem_cgroup_iter(null_mut(), null_mut(), null_mut());
    loop {
        let mut nid = rust_mglru_first_memory_node();
        while nid < MAX_NUMNODES as i32 {
            let old = skip;
            skip = skip.wrapping_sub(1);
            if old == 0 {
                return get_lruvec(memcg, nid).cast();
            }
            nid = rust_mglru_next_memory_node(nid);
        }
        memcg = mem_cgroup_iter(null_mut(), memcg, null_mut());
        if memcg.is_null() {
            break;
        }
    }
    null_mut()
}
#[export_name = "rust_mglru_seq_stop"]
unsafe extern "C" fn lru_gen_seq_stop(m: *mut seq_file, v: *mut Void) {
    if !rust_mglru_is_err_or_null(v) {
        mem_cgroup_iter_break(null_mut(), lruvec_memcg(v.cast()));
    }
    kvfree((*m).private);
    (*m).private = null_mut();
}
#[export_name = "rust_mglru_seq_next"]
unsafe extern "C" fn lru_gen_seq_next(
    _m: *mut seq_file,
    v: *mut Void,
    pos: *mut loff_t,
) -> *mut Void {
    let mut nid = (*lruvec_pgdat(v.cast())).node_id;
    let mut memcg = lruvec_memcg(v.cast());
    *pos = (*pos).wrapping_add(1);
    nid = rust_mglru_next_memory_node(nid);
    if nid == MAX_NUMNODES as i32 {
        memcg = mem_cgroup_iter(null_mut(), memcg, null_mut());
        if memcg.is_null() {
            return null_mut();
        }
        nid = rust_mglru_first_memory_node();
    }
    get_lruvec(memcg, nid).cast()
}
unsafe fn lru_gen_seq_show_full(
    m: *mut seq_file,
    v: *mut lruvec,
    max: ULong,
    min: &[ULong; 2],
    seq: ULong,
) {
    let hist = lru_hist_from_seq(seq) as usize;
    let g = addr_of!((*v).lrugen);
    let state = get_mm_state(v);
    for tier in 0..MAX_NR_TIERS as usize {
        seq_printf(m, b"            %10d\0".as_ptr().cast(), tier as i32);
        for typ in 0..ANON_AND_FILE as usize {
            let mut s = b"xxx";
            let mut n = [0 as ULong; 3];
            if seq == max {
                s = b"RTx";
                n[0] = rd(addr_of!((*g).avg_refaulted[typ][tier]));
                n[1] = rd(addr_of!((*g).avg_total[typ][tier]));
            } else if seq == min[typ] || NR_HIST_GENS > 1 {
                s = b"rep";
                n[0] = atomic_long_read(addr_of!((*g).refaulted[hist][typ][tier])) as ULong;
                n[1] = atomic_long_read(addr_of!((*g).evicted[hist][typ][tier])) as ULong;
                n[2] = rd(addr_of!((*g).protected[hist][typ][tier]));
            }
            for i in 0..3 {
                seq_printf(m, b" %10lu%c\0".as_ptr().cast(), n[i], s[i] as i32);
            }
        }
        seq_putc(m, b'\n' as CChar);
    }
    if state.is_null() {
        return;
    }
    seq_puts(m, b"                      \0".as_ptr().cast());
    for i in 0..NR_MM_STATS as usize {
        let mut s = b"xxxx";
        let mut n: ULong = 0;
        if seq == max && NR_HIST_GENS == 1 {
            s = b"TYFA";
            n = rd(addr_of!((*state).stats[hist][i]));
        } else if seq != max && NR_HIST_GENS > 1 {
            s = b"tyfa";
            n = rd(addr_of!((*state).stats[hist][i]));
        }
        seq_printf(m, b" %10lu%c\0".as_ptr().cast(), n, s[i] as i32);
    }
    seq_putc(m, b'\n' as CChar);
}
#[export_name = "rust_mglru_seq_show"]
unsafe extern "C" fn lru_gen_seq_show(m: *mut seq_file, ptr: *mut Void) -> i32 {
    let full = rust_mglru_debugfs_get_aux_num((*m).file) != 0;
    let v = ptr as *mut lruvec;
    let g = addr_of!((*v).lrugen);
    let nid = (*lruvec_pgdat(v)).node_id;
    let memcg = lruvec_memcg(v);
    let max = max_seq(v);
    let min = min_seq(v);
    if nid == rust_mglru_first_memory_node() {
        let path = if !memcg.is_null() {
            (*m).private as *const CChar
        } else {
            b"\0".as_ptr().cast()
        };
        #[cfg(CONFIG_MEMCG)]
        if !memcg.is_null() {
            cgroup_path((*memcg).css.cgroup, (*m).private.cast(), PATH_MAX as usize);
        }
        seq_printf(
            m,
            b"memcg %llu %s\n\0".as_ptr().cast(),
            mem_cgroup_id(memcg) as u64,
            path,
        );
    }
    seq_printf(m, b" node %5d\n\0".as_ptr().cast(), nid);
    let mut seq = if !full {
        evictable_min_seq(&min, MAX_SWAPPINESS as i32 / 2)
    } else if max >= MAX_NR_GENS as ULong {
        max - MAX_NR_GENS as ULong + 1
    } else {
        0
    };
    while seq <= max {
        let gen = lru_gen_from_seq(seq) as usize;
        let birth = rd(addr_of!((*v).lrugen.timestamps[gen]));
        seq_printf(
            m,
            b" %10lu %10u\0".as_ptr().cast(),
            seq,
            rust_mglru_jiffies_to_msecs(rust_mglru_jiffies().wrapping_sub(birth)),
        );
        for typ in 0..ANON_AND_FILE as usize {
            let mut size: ULong = 0;
            let mark = if full && seq < min[typ] { b'x' } else { b' ' };
            for zone in 0..MAX_NR_ZONES as usize {
                size = size
                    .wrapping_add(
                        core::cmp::max(rd(addr_of!((*g).nr_pages[gen][typ][zone])), 0) as ULong,
                    );
            }
            seq_printf(m, b" %10lu%c\0".as_ptr().cast(), size, mark as i32);
        }
        seq_putc(m, b'\n' as CChar);
        if full {
            lru_gen_seq_show_full(m, v, max, &min, seq);
        }
        seq = seq.wrapping_add(1);
    }
    0
}
unsafe fn run_aging(v: *mut lruvec, seq: ULong, sw: i32, force: bool) -> i32 {
    let max = max_seq(v);
    if seq > max {
        return -(EINVAL as i32);
    }
    if try_to_inc_max_seq(v, max, sw, force) {
        0
    } else {
        -(EEXIST as i32)
    }
}
unsafe fn run_eviction(
    v: *mut lruvec,
    seq: ULong,
    sc: *mut scan_control,
    sw: i32,
    nr: ULong,
) -> i32 {
    if seq.wrapping_add(MIN_NR_GENS as ULong) > max_seq(v) {
        return -(EINVAL as i32);
    }
    (*sc).nr_reclaimed = 0;
    while !signal_pending(current()) {
        let min = min_seq(v);
        if seq < evictable_min_seq(&min, sw) || (*sc).nr_reclaimed >= nr {
            return 0;
        }
        let batch = core::cmp::min(nr - (*sc).nr_reclaimed, MAX_LRU_BATCH as ULong);
        if evict_folios(batch, v, sc, sw) == 0 {
            return 0;
        }
        cond_resched();
    }
    -(EINTR as i32)
}
unsafe fn run_cmd(
    cmd: CChar,
    id: u64,
    nid: i32,
    seq: ULong,
    sc: *mut scan_control,
    mut sw: i32,
    opt: ULong,
) -> i32 {
    if nid < 0 || nid >= MAX_NUMNODES as i32 || !rust_mglru_node_memory(nid) {
        return -(EINVAL as i32);
    }
    let mut err = -(EINVAL as i32);
    let mut memcg = null_mut();
    if !mem_cgroup_disabled() {
        memcg = mem_cgroup_get_from_id(id);
        if memcg.is_null() {
            return -(EINVAL as i32);
        }
    }
    if id == mem_cgroup_id(memcg) as u64 {
        (*sc).target_mem_cgroup = memcg;
        let v = get_lruvec(memcg, nid);
        let valid_swappiness = if sw < MIN_SWAPPINESS as i32 {
            sw = get_swappiness(v, sc);
            true
        } else {
            sw <= SWAPPINESS_ANON_ONLY as i32
        };
        if valid_swappiness {
            if cmd == b'+' as CChar {
                err = run_aging(v, seq, sw, opt != 0);
            } else if cmd == b'-' as CChar {
                err = run_eviction(v, seq, sc, sw, opt);
            }
        }
    }
    mem_cgroup_put(memcg);
    err
}
#[export_name = "rust_mglru_seq_write"]
unsafe extern "C" fn lru_gen_seq_write(
    _file: *mut file,
    src: *const CChar,
    len: usize,
    _pos: *mut loff_t,
) -> isize {
    let mut plug: blk_plug = zeroed();
    let mut err = -(EINVAL as i32);
    let mut sc: scan_control = zeroed();
    sc.set_may_writepage(1);
    sc.set_may_unmap(1);
    sc.set_may_swap(1);
    sc.set_proactive(1);
    sc.reclaim_idx = MAX_NR_ZONES as i8 - 1;
    sc.gfp_mask = GFP_KERNEL as _;
    let buf = kvmalloc(len.wrapping_add(1), GFP_KERNEL as _);
    if buf.is_null() {
        return -(ENOMEM as isize);
    }
    if rust_mglru_copy_from_user(buf, src.cast(), len as ULong) != 0 {
        kvfree(buf);
        return -(EFAULT as isize);
    }
    set_task_reclaim_state(current(), addr_of_mut!(sc.reclaim_state));
    let flags = memalloc_noreclaim_save();
    blk_start_plug(&mut plug);
    if set_mm_walk(null_mut(), true).is_null() {
        err = -(ENOMEM as i32);
    } else {
        let mut next = buf as *mut CChar;
        *next.add(len) = 0;
        loop {
            let mut cur = strsep(&mut next, b",;\n\0".as_ptr().cast());
            if cur.is_null() {
                break;
            }
            cur = skip_spaces(cur) as *mut CChar;
            if *cur == 0 {
                continue;
            }
            let mut cmd: CChar = 0;
            let mut swap_string = [0 as CChar; 5];
            let mut id: u64 = 0;
            let mut nid = 0u32;
            let mut seq: ULong = 0;
            let mut end = 0i32;
            let mut opt = ULong::MAX;
            let mut sw = 0u32;
            let n = rust_mglru_scan_cmd(
                cur,
                &mut cmd,
                &mut id,
                &mut nid,
                &mut seq,
                &mut end,
                swap_string.as_mut_ptr(),
                &mut opt,
            );
            if n < 4 || *cur.add(end as usize) != 0 {
                err = -(EINVAL as i32);
                break;
            }
            if n == 4 {
                sw = u32::MAX;
            } else if strcmp(b"max\0".as_ptr().cast(), swap_string.as_ptr()) == 0 {
                sw = SWAPPINESS_ANON_ONLY as u32;
            } else {
                err = kstrtouint(swap_string.as_ptr(), 0, &mut sw);
                if err != 0 {
                    break;
                }
            }
            err = run_cmd(cmd, id, nid as i32, seq, &mut sc, sw as i32, opt);
            if err != 0 {
                break;
            }
        }
    }
    clear_mm_walk();
    blk_finish_plug(&mut plug);
    memalloc_noreclaim_restore(flags);
    set_task_reclaim_state(current(), null_mut());
    kvfree(buf);
    if err != 0 {
        err as isize
    } else {
        len as isize
    }
}
#[export_name = "rust_mglru_seq_open"]
unsafe extern "C" fn lru_gen_seq_open(_inode: *mut inode, file: *mut file) -> i32 {
    seq_open(file, rust_mglru_seq_ops())
}
#[no_mangle]
pub unsafe extern "C" fn lru_gen_init_pgdat(pgdat: *mut pglist_data) {
    rust_mglru_spin_lock_init(0, addr_of_mut!((*pgdat).memcg_lru.lock));
    for i in 0..MEMCG_NR_GENS as usize {
        for j in 0..MEMCG_NR_BINS as usize {
            rust_mglru_init_hlist_nulls_head(
                addr_of_mut!((*pgdat).memcg_lru.fifo[i][j]),
                i as ULong,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lru_gen_init_lruvec(v: *mut lruvec) {
    let g = addr_of_mut!((*v).lrugen);
    let state = get_mm_state(v);
    (*g).max_seq = MIN_NR_GENS as ULong + 1;
    (*g).enabled = lru_gen_enabled();
    for i in 0..=MIN_NR_GENS as usize + 1 {
        (*g).timestamps[i] = rust_mglru_jiffies();
    }
    for gen in 0..MAX_NR_GENS as usize {
        for typ in 0..ANON_AND_FILE as usize {
            for zone in 0..MAX_NR_ZONES as usize {
                init_list_head(addr_of_mut!((*g).folios[gen][typ][zone]));
            }
        }
    }
    if !state.is_null() {
        (*state).seq = MIN_NR_GENS as ULong;
    }
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
pub unsafe extern "C" fn lru_gen_init_memcg(memcg: *mut mem_cgroup) {
    let list = get_mm_list(memcg);
    if list.is_null() {
        return;
    }
    init_list_head(addr_of_mut!((*list).fifo));
    rust_mglru_spin_lock_init(1, addr_of_mut!((*list).lock));
}
#[cfg(CONFIG_MEMCG)]
#[no_mangle]
pub unsafe extern "C" fn lru_gen_exit_memcg(memcg: *mut mem_cgroup) {
    let list = get_mm_list(memcg);
    vmwarn!(40, !list.is_null() && !list_empty(addr_of!((*list).fifo)));
    let mut nid = rust_mglru_first_node();
    while nid < MAX_NUMNODES as i32 {
        let v = get_lruvec(memcg, nid);
        let state = get_mm_state(v);
        vmwarn!(
            41,
            !memchr_inv(
                addr_of!((*v).lrugen.nr_pages).cast(),
                0,
                size_of::<
                    [[[Long; MAX_NR_ZONES as usize]; ANON_AND_FILE as usize]; MAX_NR_GENS as usize],
                >()
            )
            .is_null()
        );
        rust_mglru_poison_lrugen_list(v);
        if !state.is_null() {
            for i in 0..NR_BLOOM_FILTERS as usize {
                bitmap_free((*state).filters[i]);
                (*state).filters[i] = null_mut();
            }
        }
        nid = rust_mglru_next_node(nid);
    }
}
#[export_name = "rust_mglru_init"]
unsafe extern "C" fn init_lru_gen() -> i32 {
    kernel::build_assert::build_assert!(MIN_NR_GENS + 1 < MAX_NR_GENS);
    kernel::build_assert::build_assert!((1u64 << LRU_GEN_WIDTH) > MAX_NR_GENS as u64);
    if rust_mglru_sysfs_create_group() != 0 {
        rust_mglru_report_sysfs_error();
    }
    rust_mglru_debugfs_create(false);
    rust_mglru_debugfs_create(true);
    0
}
