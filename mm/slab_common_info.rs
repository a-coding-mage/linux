// SPDX-License-Identifier: GPL-2.0
// Original mm/slab_common.c:1157-1306: diagnostics, sensitive free, BPF query.
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn print_slabinfo_header(m: *mut seq_file) {
    rust_slab_common_seq_puts(m, c"slabinfo - version: 2.1\n".as_ptr());
    rust_slab_common_seq_puts(
        m,
        c"# name            <active_objs> <num_objs> <objsize> <objperslab> <pagesperslab>"
            .as_ptr(),
    );
    rust_slab_common_seq_puts(
        m,
        c" : tunables <limit> <batchcount> <sharedfactor>".as_ptr(),
    );
    rust_slab_common_seq_puts(
        m,
        c" : slabdata <active_slabs> <num_slabs> <sharedavail>".as_ptr(),
    );
    seq_putc(m, b'\n' as CChar);
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[no_mangle]
pub unsafe extern "C" fn slab_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut Void {
    rust_slab_common_mutex_lock(addr_of_mut!(slab_mutex));
    seq_list_start(addr_of_mut!(slab_caches), *pos)
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[no_mangle]
pub unsafe extern "C" fn slab_next(_m: *mut seq_file, p: *mut Void, pos: *mut loff_t) -> *mut Void {
    seq_list_next(p, addr_of_mut!(slab_caches), pos)
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[no_mangle]
pub unsafe extern "C" fn slab_stop(_m: *mut seq_file, _p: *mut Void) {
    rust_slab_common_mutex_unlock(addr_of_mut!(slab_mutex));
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn cache_show(s: *mut bindings::kmem_cache, m: *mut seq_file) {
    let mut sinfo: slabinfo = zeroed();
    get_slabinfo(s, addr_of_mut!(sinfo));
    seq_printf(
        m,
        c"%-17s %6lu %6lu %6u %4u %4d".as_ptr(),
        (*s).name,
        sinfo.active_objs,
        sinfo.num_objs,
        (*s).size,
        sinfo.objects_per_slab,
        1i32.wrapping_shl(sinfo.cache_order),
    );
    seq_printf(
        m,
        c" : tunables %4u %4u %4u".as_ptr(),
        sinfo.limit,
        sinfo.batchcount,
        sinfo.shared,
    );
    seq_printf(
        m,
        c" : slabdata %6lu %6lu %6lu".as_ptr(),
        sinfo.active_slabs,
        sinfo.num_slabs,
        sinfo.shared_avail,
    );
    seq_putc(m, b'\n' as CChar);
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[no_mangle]
pub unsafe extern "C" fn slab_show(m: *mut seq_file, p: *mut Void) -> Int {
    let s = cache_from_list(p.cast());
    if p == slab_caches.next.cast() {
        print_slabinfo_header(m);
    }
    cache_show(s, m);
    0
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[no_mangle]
pub unsafe extern "C" fn dump_unreclaimable_slab() {
    if !rust_slab_common_mutex_trylock(addr_of_mut!(slab_mutex)) {
        sc_log!("\x014excessive unreclaimable slab but cannot dump stats\n");
        return;
    }
    sc_log!("\x016Unreclaimable slab info:\n");
    sc_log!("\x016Name                      Used          Total\n");
    let head = addr_of_mut!(slab_caches);
    let mut p = (*head).next;
    // Every get_slabinfo output field used below is written by the SLUB owner.
    let mut sinfo = core::mem::MaybeUninit::<slabinfo>::uninit();
    while p != head {
        let s = cache_from_list(p);
        p = (*p).next;
        if (*s).flags & RSC_SLAB_RECLAIM_ACCOUNT != 0 {
            continue;
        }
        get_slabinfo(s, sinfo.as_mut_ptr());
        let info = sinfo.as_ptr();
        if (*info).num_objs > 0 {
            sc_log!(
                "\x016%-17s %10luKB %10luKB\n",
                (*s).name,
                (*info).active_objs.wrapping_mul((*s).size as ULong) / 1024,
                (*info).num_objs.wrapping_mul((*s).size as ULong) / 1024
            );
        }
    }
    rust_slab_common_mutex_unlock(addr_of_mut!(slab_mutex));
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[no_mangle]
pub unsafe extern "C" fn slabinfo_open(_inode: *mut inode, file: *mut bindings::file) -> Int {
    seq_open(file, addr_of!(slabinfo_op))
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RSC_INIT_COLD, cold)]
pub unsafe extern "C" fn slab_proc_init() -> Int {
    rust_slab_common_proc_create(
        c"slabinfo".as_ptr(),
        0o400,
        null_mut(),
        addr_of!(slabinfo_proc_ops),
    );
    0
}
#[no_mangle]
pub unsafe extern "C" fn kfree_sensitive(p: *const Void) {
    let mem = p.cast_mut();
    let ks = ksize(mem);
    if ks != 0 {
        rust_slab_common_kasan_unpoison_range(mem, ks);
        rust_slab_common_memzero_explicit(mem, ks);
    }
    kfree(mem);
}
// Native BPF declaration wrapper preserves __bpf_kfunc's BTF/CFI attributes.
#[cfg(CONFIG_BPF_SYSCALL)]
#[no_mangle]
pub unsafe extern "C" fn rust_slab_common_bpf_get_kmem_cache(
    addr: u64,
) -> *mut bindings::kmem_cache {
    let p = addr as Long as *mut Void;
    if !rust_slab_common_virt_addr_valid(p) {
        return null_mut();
    }
    let slab = rust_slab_common_virt_to_slab(p);
    if slab.is_null() {
        null_mut()
    } else {
        (*slab).slab_cache
    }
}
