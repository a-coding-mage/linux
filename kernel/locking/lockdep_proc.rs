// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of locking/lockdep_proc.c. */

// Kernel dependencies supplied by the surrounding translation unit.

#[allow(dead_code)]
unsafe fn l_next(m: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let class = v as *mut lock_class;
    let class = unsafe { class.add(1) };
    unsafe { *pos = class.offset_from(lock_classes) as loff_t; }
    if unsafe { *pos } > max_lock_class_idx as loff_t { core::ptr::null_mut() } else { class as *mut _ }
}

unsafe fn l_start(_: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let idx = unsafe { *pos } as usize;
    if idx > max_lock_class_idx as usize { core::ptr::null_mut() } else { unsafe { lock_classes.add(idx) as *mut _ } }
}
unsafe fn l_stop(_: *mut seq_file, _: *mut core::ffi::c_void) {}

unsafe fn print_name(m: *mut seq_file, class: *mut lock_class) {
    let mut str_buf = [0i8; KSYM_NAME_LEN];
    let name = unsafe { (*class).name };
    if name.is_null() { let name = __get_key_name(unsafe { (*class).key }, str_buf.as_mut_ptr()); seq_printf(m, "%s", name); }
    else {
        seq_printf(m, "%s", name);
        if unsafe { (*class).name_version > 1 } { seq_printf(m, "#%d", unsafe { (*class).name_version }); }
        if unsafe { (*class).subclass != 0 } { seq_printf(m, "/%d", unsafe { (*class).subclass }); }
    }
}

unsafe fn l_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    let class = v as *mut lock_class;
    if v == lock_classes as *mut _ { seq_printf(m, "all lock classes:\n"); }
    let idx = unsafe { class.offset_from(lock_classes) as usize };
    if !test_bit(idx, lock_classes_in_use) { return 0; }
    seq_printf(m, "%p", unsafe { (*class).key });
    #[cfg(CONFIG_DEBUG_LOCKDEP)] seq_printf(m, " OPS:%8ld", debug_class_ops_read(class));
    #[cfg(CONFIG_PROVE_LOCKING)] {
        seq_printf(m, " FD:%5ld", lockdep_count_forward_deps(class));
        seq_printf(m, " BD:%5ld", lockdep_count_backward_deps(class));
        let mut usage = [0i8; LOCK_USAGE_CHARS]; get_usage_chars(class, usage.as_mut_ptr()); seq_printf(m, " %s", usage.as_ptr());
    }
    seq_printf(m, ": "); print_name(m, class); seq_puts(m, "\n");
    #[cfg(CONFIG_PROVE_LOCKING)] for entry in list_for_each_entry!(class, locks_after, entry) {
        if entry.distance == 1 { seq_printf(m, " -> [%p] ", entry.class.key); print_name(m, entry.class); seq_puts(m, "\n"); }
    }
    #[cfg(CONFIG_PROVE_LOCKING)] seq_puts(m, "\n"); 0
}

static lockdep_ops: seq_operations = seq_operations { start: l_start, next: l_next, stop: l_stop, show: l_show };

#[cfg(CONFIG_PROVE_LOCKING)]
unsafe fn lc_start(_: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    if unsafe { *pos } < 0 { return core::ptr::null_mut(); }
    if unsafe { *pos } == 0 { return SEQ_START_TOKEN; }
    unsafe { lock_chains.add((*pos - 1) as usize) as *mut _ }
}
#[cfg(CONFIG_PROVE_LOCKING)] unsafe fn lc_next(_: *mut seq_file, _: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void { unsafe { *pos = lockdep_next_lockchain(*pos - 1) + 1; } lc_start(core::ptr::null_mut(), pos) }
#[cfg(CONFIG_PROVE_LOCKING)] unsafe fn lc_stop(_: *mut seq_file, _: *mut core::ffi::c_void) {}
#[cfg(CONFIG_PROVE_LOCKING)] unsafe fn lc_show(m: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    if v == SEQ_START_TOKEN { if nr_free_chain_hlocks == 0 { seq_printf(m, "(buggered) "); } seq_printf(m, "all lock chains:\n"); return 0; }
    let chain = v as *mut lock_chain; seq_printf(m, "irq_context: %s\n", irq_strs[unsafe { (*chain).irq_context }]);
    for i in 0..unsafe { (*chain).depth } { let class = lock_chain_get_class(chain, i); if unsafe { (*class).key.is_null() } { continue; } seq_printf(m, "[%p] ", unsafe { (*class).key }); print_name(m, class); seq_puts(m, "\n"); } seq_puts(m, "\n"); 0
}

unsafe fn lockdep_stats_debug_show(m: *mut seq_file) {
    #[cfg(CONFIG_DEBUG_LOCKDEP)] {
        let hi1=debug_atomic_read(hardirqs_on_events); let hi2=debug_atomic_read(hardirqs_off_events); let hr1=debug_atomic_read(redundant_hardirqs_on); let hr2=debug_atomic_read(redundant_hardirqs_off); let si1=debug_atomic_read(softirqs_on_events); let si2=debug_atomic_read(softirqs_off_events); let sr1=debug_atomic_read(redundant_softirqs_on); let sr2=debug_atomic_read(redundant_softirqs_off);
        seq_printf(m," chain lookup misses:           %11llu\n",debug_atomic_read(chain_lookup_misses)); seq_printf(m," chain lookup hits:             %11llu\n",debug_atomic_read(chain_lookup_hits)); seq_printf(m," cyclic checks:                 %11llu\n",debug_atomic_read(nr_cyclic_checks)); seq_printf(m," redundant checks:              %11llu\n",debug_atomic_read(nr_redundant_checks)); seq_printf(m," redundant links:               %11llu\n",debug_atomic_read(nr_redundant)); seq_printf(m," find-mask forwards checks:     %11llu\n",debug_atomic_read(nr_find_usage_forwards_checks)); seq_printf(m," find-mask backwards checks:    %11llu\n",debug_atomic_read(nr_find_usage_backwards_checks));
        seq_printf(m," hardirq on events:             %11llu\n",hi1); seq_printf(m," hardirq off events:            %11llu\n",hi2); seq_printf(m," redundant hardirq ons:         %11llu\n",hr1); seq_printf(m," redundant hardirq offs:        %11llu\n",hr2); seq_printf(m," softirq on events:             %11llu\n",si1); seq_printf(m," softirq off events:            %11llu\n",si2); seq_printf(m," redundant softirq ons:         %11llu\n",sr1); seq_printf(m," redundant softirq offs:        %11llu\n",sr2);
    }
}

// The remaining statistics and lock-stat routines retain the C control flow and
// call the corresponding kernel interfaces supplied by lockdep_internals.
unsafe fn lockdep_stats_show(m: *mut seq_file, _: *mut core::ffi::c_void) -> i32 {
    let (mut nr_unused,mut nr_uncategorized,mut nr_irq_safe,mut nr_irq_unsafe,mut nr_softirq_safe,mut nr_softirq_unsafe,mut nr_hardirq_safe,mut nr_hardirq_unsafe,mut nr_irq_read_safe,mut nr_irq_read_unsafe,mut nr_softirq_read_safe,mut nr_softirq_read_unsafe,mut nr_hardirq_read_safe,mut nr_hardirq_read_unsafe,mut sum_forward_deps)=(0usize,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    #[cfg(CONFIG_PROVE_LOCKING)] for idx in 0..=max_lock_class_idx as usize { if !test_bit(idx,lock_classes_in_use){continue} let c=lock_classes.add(idx); let u=(*c).usage_mask; if u==0{nr_unused+=1} if u==LOCKF_USED{nr_uncategorized+=1} if u&LOCKF_USED_IN_IRQ!=0{nr_irq_safe+=1} if u&LOCKF_ENABLED_IRQ!=0{nr_irq_unsafe+=1} if u&LOCKF_USED_IN_SOFTIRQ!=0{nr_softirq_safe+=1} if u&LOCKF_ENABLED_SOFTIRQ!=0{nr_softirq_unsafe+=1} if u&LOCKF_USED_IN_HARDIRQ!=0{nr_hardirq_safe+=1} if u&LOCKF_ENABLED_HARDIRQ!=0{nr_hardirq_unsafe+=1} if u&LOCKF_USED_IN_IRQ_READ!=0{nr_irq_read_safe+=1} if u&LOCKF_ENABLED_IRQ_READ!=0{nr_irq_read_unsafe+=1} if u&LOCKF_USED_IN_SOFTIRQ_READ!=0{nr_softirq_read_safe+=1} if u&LOCKF_ENABLED_SOFTIRQ_READ!=0{nr_softirq_read_unsafe+=1} if u&LOCKF_USED_IN_HARDIRQ_READ!=0{nr_hardirq_read_safe+=1} if u&LOCKF_ENABLED_HARDIRQ_READ!=0{nr_hardirq_read_unsafe+=1} sum_forward_deps+=lockdep_count_forward_deps(c) as usize; }
    seq_printf(m," lock-classes:                  %11lu [max: %lu]\n",nr_lock_classes,MAX_LOCKDEP_KEYS); seq_printf(m," dynamic-keys:                  %11lu\n",nr_dynamic_keys); seq_printf(m," direct dependencies:           %11lu [max: %lu]\n",nr_list_entries,MAX_LOCKDEP_ENTRIES); seq_printf(m," indirect dependencies:         %11lu\n",sum_forward_deps); seq_printf(m," all direct dependencies:       %11lu\n",nr_irq_unsafe*nr_irq_safe+nr_hardirq_unsafe*nr_hardirq_safe+nr_list_entries); seq_printf(m," hardirq-safe locks:            %11lu\n",nr_hardirq_safe); seq_printf(m," hardirq-unsafe locks:          %11lu\n",nr_hardirq_unsafe); seq_printf(m," softirq-safe locks:            %11lu\n",nr_softirq_safe); seq_printf(m," softirq-unsafe locks:          %11lu\n",nr_softirq_unsafe); seq_printf(m," irq-safe locks:                %11lu\n",nr_irq_safe); seq_printf(m," irq-unsafe locks:              %11lu\n",nr_irq_unsafe); seq_printf(m," uncategorized locks:           %11lu\n",nr_uncategorized); seq_printf(m," unused locks:                  %11lu\n",nr_unused); seq_printf(m," max locking depth:             %11u\n",max_lockdep_depth); seq_printf(m," max lock class index:          %11lu\n",max_lock_class_idx); lockdep_stats_debug_show(m); seq_printf(m," debug_locks:                   %11u\n",debug_locks); seq_printf(m,"\n zapped classes:                %11lu\n",nr_zapped_classes); 0
}

unsafe fn lockdep_proc_init() -> i32 { proc_create_seq("lockdep",S_IRUSR,core::ptr::null_mut(),&lockdep_ops); proc_create_single("lockdep_stats",S_IRUSR,core::ptr::null_mut(),lockdep_stats_show); 0 }

#[cfg(CONFIG_LOCK_STAT)]
#[repr(C)] struct lock_stat_data { class: *mut lock_class, stats: lock_class_stats }
#[cfg(CONFIG_LOCK_STAT)]
#[repr(C)] struct lock_stat_seq { iter_end: *mut lock_stat_data, stats: [lock_stat_data; MAX_LOCKDEP_KEYS] }
#[cfg(CONFIG_LOCK_STAT)]
unsafe fn lock_stat_cmp(l: *const core::ffi::c_void, r: *const core::ffi::c_void) -> i32 {
    let dl=l as *const lock_stat_data; let dr=r as *const lock_stat_data;
    ((*dr).stats.read_waittime.nr + (*dr).stats.write_waittime.nr - (*dl).stats.read_waittime.nr - (*dl).stats.write_waittime.nr) as i32
}
#[cfg(CONFIG_LOCK_STAT)] unsafe fn seq_line(m:*mut seq_file,c:i8,offset:i32,length:i32){for _ in 0..offset{seq_puts(m," ")}for _ in 0..length{seq_putc(m,c)}seq_puts(m,"\n")}
#[cfg(CONFIG_LOCK_STAT)] unsafe fn snprint_time(buf:*mut i8,_:usize,mut nr:i64){nr=nr.wrapping_add(5);let div=nr/1000;let rem=nr%1000;snprintf(buf,22,"%lld.%02d",div,rem/10)}
#[cfg(CONFIG_LOCK_STAT)] unsafe fn seq_time(m:*mut seq_file,time:i64){let mut n=[0i8;22];snprint_time(n.as_mut_ptr(),22,time);seq_printf(m," %14s",n.as_ptr())}
#[cfg(CONFIG_LOCK_STAT)] unsafe fn seq_lock_time(m:*mut seq_file,lt:*mut lock_time){seq_printf(m,"%14lu",(*lt).nr);seq_time(m,(*lt).min);seq_time(m,(*lt).max);seq_time(m,(*lt).total);seq_time(m,if (*lt).nr!=0{div64_u64((*lt).total,(*lt).nr)}else{0})}
#[cfg(CONFIG_LOCK_STAT)] unsafe fn seq_stats(m:*mut seq_file,data:*mut lock_stat_data){let c=(*data).class;let s=&mut (*data).stats;let mut name=[0i8;39];let cname=(*c).name;if !cname.is_null(){snprintf(name.as_mut_ptr(),38,"%s",cname)}else{let mut b=[0i8;KSYM_NAME_LEN];snprintf(name.as_mut_ptr(),38,"%s",__get_key_name((*c).key,b.as_mut_ptr()))}if s.write_holdtime.nr!=0{seq_printf(m,"%40s:",name.as_ptr());seq_printf(m,"%14lu ",s.bounces[bounce_acquired_write]);seq_lock_time(m,&mut s.write_waittime);seq_puts(m,"\n")}if s.read_holdtime.nr!=0{seq_printf(m,"%38s-R:",name.as_ptr());seq_lock_time(m,&mut s.read_waittime);seq_puts(m,"\n")}for i in 0..LOCKSTAT_POINTS{if (*c).contention_point[i]==0{break}seq_printf(m,"%40s %14lu\n",name.as_ptr(),s.contention_point[i])}}
#[cfg(CONFIG_LOCK_STAT)] unsafe fn seq_header(m:*mut seq_file){seq_puts(m,"lock_stat version 0.4\n");if debug_locks==0{seq_puts(m,"*WARNING* lock debugging disabled!! - possibly due to a lockdep warning\n")}seq_line(m,b'-' as i8,0,40+1+12*(14+1));seq_puts(m,"class name                         con-bounces    contentions    waittime-min    waittime-max    waittime-total   waittime-avg\n");seq_line(m,b'-' as i8,0,40+1+12*(14+1))}
#[cfg(CONFIG_LOCK_STAT)] unsafe fn lock_stat_open(_: *mut inode,file:*mut file)->i32{let d=vmalloc(core::mem::size_of::<lock_stat_seq>()) as *mut lock_stat_seq;if d.is_null(){return -12}seq_open(file,&lockstat_ops)}
#[cfg(CONFIG_LOCK_STAT)] unsafe fn lock_stat_write(_: *mut file,_:*const i8,count:usize,_:*mut loff_t)->isize{count as isize}
#[cfg(CONFIG_LOCK_STAT)] unsafe fn lock_stat_release(i:*mut inode,f:*mut file)->i32{seq_release(i,f)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
