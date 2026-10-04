// SPDX-License-Identifier: GPL-2.0
// Original mm/slab_common.c:699-1155. Native headers define cache geometry.
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RSC_INIT_COLD, cold)]
pub unsafe extern "C" fn create_boot_cache(
    s: *mut bindings::kmem_cache,
    name: *const CChar,
    size: UInt,
    flags: slab_flags_t,
    useroffset: UInt,
    usersize: UInt,
) {
    let mut align = RSC_ARCH_KMALLOC_MINALIGN as UInt;
    // C ffs(size) identifies the largest power-of-two divisor, not next_power_of_two.
    if flags & RSC_SLAB_KMALLOC != 0 {
        align = max(align, 1u32 << size.trailing_zeros());
    }
    let mut args: kmem_cache_args = zeroed();
    args.align = calculate_alignment(flags, align, size);
    #[cfg(CONFIG_HARDENED_USERCOPY)]
    {
        args.useroffset = useroffset;
        args.usersize = usersize;
    }
    let err = do_kmem_cache_create(s, name, size, addr_of_mut!(args), flags);
    if err != 0 {
        panic(
            c"Creation of kmalloc slab %s size=%u failed. Reason %d\n".as_ptr(),
            name,
            size,
            err,
        );
    }
    (*s).refcount = -1;
}
#[link_section = ".init.text"]
#[cfg_attr(RSC_INIT_COLD, cold)]
unsafe fn create_kmalloc_cache(
    name: *const CChar,
    size: UInt,
    flags: slab_flags_t,
) -> *mut bindings::kmem_cache {
    let s = rust_slab_common_boot_cache_zalloc(kmem_cache, RSC_GFP_NOWAIT)
        .cast::<bindings::kmem_cache>();
    if s.is_null() {
        panic(c"Out of memory when creating slab %s\n".as_ptr(), name);
    }
    create_boot_cache(s, name, size, flags | RSC_SLAB_KMALLOC, 0, size);
    rust_slab_common_list_add(addr_of_mut!((*s).list), addr_of_mut!(slab_caches));
    (*s).refcount = 1;
    s
}
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut kmalloc_size_index: [u8; 24] = [
    3, 4, 5, 5, 6, 6, 6, 6, 1, 1, 1, 1, 7, 7, 7, 7, 2, 2, 2, 2, 2, 2, 2, 2,
];
#[no_mangle]
pub unsafe extern "C" fn kmalloc_size_roundup(size: usize) -> usize {
    if size != 0 && size <= RSC_KMALLOC_MAX_CACHE_SIZE as usize {
        // The token is a native configuration-dependent type; this leaf only
        // materializes __kmalloc_token(0) and invokes external kmalloc_slab.
        return (*rust_slab_common_kmalloc_slab_roundup(size)).object_size as usize;
    }
    if size != 0 && size <= RSC_KMALLOC_MAX_SIZE as usize {
        return (RSC_PAGE_SIZE as usize) << rust_slab_common_get_order(size);
    }
    size
}
// The shared native declaration fixes both the size and order of .name[] fields.
// Pointers refer exclusively to static NUL-terminated strings.
unsafe impl Sync for kmalloc_info_struct {}
macro_rules! kmalloc_info_entry {
    ($size:expr, $short:literal) => {{
        let mut names = [null(); NR_KMALLOC_TYPES as usize];
        names[KMALLOC_NORMAL as usize] = concat!("kmalloc-", $short, "\0").as_ptr().cast();
        #[cfg(not(CONFIG_SLUB_TINY))]
        {
            names[KMALLOC_RECLAIM as usize] = concat!("kmalloc-rcl-", $short, "\0").as_ptr().cast();
        }
        #[cfg(CONFIG_MEMCG)]
        {
            names[KMALLOC_CGROUP as usize] = concat!("kmalloc-cg-", $short, "\0").as_ptr().cast();
        }
        #[cfg(CONFIG_ZONE_DMA)]
        {
            names[KMALLOC_DMA as usize] = concat!("dma-kmalloc-", $short, "\0").as_ptr().cast();
        }
        #[cfg(CONFIG_SLAB_OBJ_EXT)]
        {
            names[KMALLOC_NO_OBJ_EXT as usize] =
                concat!("kmalloc-no-objext-", $short, "\0").as_ptr().cast();
        }
        #[cfg(CONFIG_KMALLOC_PARTITION_CACHES)]
        {
            macro_rules! partition {
                ($n:expr, $digits:literal) => {
                    if RSC_KMALLOC_PARTITION_CACHES_NR >= $n {
                        names[KMALLOC_PARTITION_START as usize + $n as usize] =
                            concat!("kmalloc-part-", $digits, "-", $short, "\0")
                                .as_ptr()
                                .cast();
                    }
                };
            }
            partition!(1, "01");
            partition!(2, "02");
            partition!(3, "03");
            partition!(4, "04");
            partition!(5, "05");
            partition!(6, "06");
            partition!(7, "07");
            partition!(8, "08");
            partition!(9, "09");
            partition!(10, "10");
            partition!(11, "11");
            partition!(12, "12");
            partition!(13, "13");
            partition!(14, "14");
            partition!(15, "15");
        }
        kmalloc_info_struct {
            name: names,
            size: $size,
        }
    }};
}
#[no_mangle]
#[link_section = ".init.rodata"]
pub static kmalloc_info: [kmalloc_info_struct; 22] = [
    kmalloc_info_entry!(0, "0"),
    kmalloc_info_entry!(96, "96"),
    kmalloc_info_entry!(192, "192"),
    kmalloc_info_entry!(8, "8"),
    kmalloc_info_entry!(16, "16"),
    kmalloc_info_entry!(32, "32"),
    kmalloc_info_entry!(64, "64"),
    kmalloc_info_entry!(128, "128"),
    kmalloc_info_entry!(256, "256"),
    kmalloc_info_entry!(512, "512"),
    kmalloc_info_entry!(1024, "1k"),
    kmalloc_info_entry!(2048, "2k"),
    kmalloc_info_entry!(4096, "4k"),
    kmalloc_info_entry!(8192, "8k"),
    kmalloc_info_entry!(16384, "16k"),
    kmalloc_info_entry!(32768, "32k"),
    kmalloc_info_entry!(65536, "64k"),
    kmalloc_info_entry!(131072, "128k"),
    kmalloc_info_entry!(262144, "256k"),
    kmalloc_info_entry!(524288, "512k"),
    kmalloc_info_entry!(1048576, "1M"),
    kmalloc_info_entry!(2097152, "2M"),
];
#[inline]
fn size_index_elem(bytes: UInt) -> usize {
    bytes.wrapping_sub(1) as usize / 8
}
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RSC_INIT_COLD, cold)]
pub unsafe extern "C" fn setup_kmalloc_cache_index_table() {
    const {
        assert!(RSC_KMALLOC_MIN_SIZE <= 256 && RSC_KMALLOC_MIN_SIZE.is_power_of_two());
    }
    let mut i = 8;
    while i < RSC_KMALLOC_MIN_SIZE as UInt {
        let elem = size_index_elem(i);
        if elem >= 24 {
            break;
        }
        kmalloc_size_index[elem] = RSC_KMALLOC_SHIFT_LOW as u8;
        i += 8;
    }
    if RSC_KMALLOC_MIN_SIZE >= 64 {
        let mut i = 72;
        while i <= 96 {
            kmalloc_size_index[size_index_elem(i)] = 7;
            i += 8;
        }
    }
    if RSC_KMALLOC_MIN_SIZE >= 128 {
        let mut i = 136;
        while i <= 192 {
            kmalloc_size_index[size_index_elem(i)] = 8;
            i += 8;
        }
    }
}
unsafe fn __kmalloc_minalign() -> UInt {
    let mut minalign = rust_slab_common_dma_get_cache_alignment();
    if cfg!(CONFIG_DMA_BOUNCE_UNALIGNED_KMALLOC) && rust_slab_common_is_swiotlb_allocated() {
        minalign = RSC_ARCH_KMALLOC_MINALIGN as UInt;
    }
    max(minalign, rust_slab_common_arch_slab_minalign())
}
#[link_section = ".init.text"]
#[cfg_attr(RSC_INIT_COLD, cold)]
unsafe fn new_kmalloc_cache(idx: Int, cache_type: kmalloc_cache_type) {
    let mut flags: slab_flags_t = 0;
    let minalign = __kmalloc_minalign();
    let mut aligned_size = kmalloc_info[idx as usize].size;
    let mut aligned_idx = idx;
    let t = cache_type as usize;
    if KMALLOC_RECLAIM != KMALLOC_NORMAL && cache_type == KMALLOC_RECLAIM {
        flags |= RSC_SLAB_RECLAIM_ACCOUNT;
    } else if cfg!(CONFIG_MEMCG) && cache_type == KMALLOC_CGROUP {
        if rust_slab_common_mem_cgroup_kmem_disabled() {
            kmalloc_caches[t][idx as usize] = kmalloc_caches[KMALLOC_NORMAL as usize][idx as usize];
            return;
        }
        flags |= RSC_SLAB_ACCOUNT;
    } else if cfg!(CONFIG_SLAB_OBJ_EXT) && cache_type == KMALLOC_NO_OBJ_EXT {
        if !rust_slab_common_need_kmalloc_no_objext() {
            kmalloc_caches[t][idx as usize] = kmalloc_caches[KMALLOC_NORMAL as usize][idx as usize];
            return;
        }
        flags |= RSC_SLAB_NO_OBJ_EXT | RSC_SLAB_NO_MERGE;
    } else if cfg!(CONFIG_ZONE_DMA) && cache_type == KMALLOC_DMA {
        flags |= RSC_SLAB_CACHE_DMA;
    }
    #[cfg(CONFIG_KMALLOC_PARTITION_CACHES)]
    if cache_type >= KMALLOC_PARTITION_START && cache_type <= KMALLOC_PARTITION_END {
        flags |= RSC_SLAB_NO_MERGE;
    }
    if !rust_slab_common_mem_cgroup_kmem_disabled() {
        if cache_type == KMALLOC_NORMAL && KMALLOC_RECLAIM != KMALLOC_NORMAL {
            flags |= RSC_SLAB_NO_MERGE;
        } else if flags & RSC_SLAB_NO_OBJ_EXT == 0 {
            flags |= RSC_SLAB_MAY_ACCOUNT;
        }
    }
    if minalign > RSC_ARCH_KMALLOC_MINALIGN as UInt {
        aligned_size = align_uint(aligned_size, minalign);
        aligned_idx = rust_slab_common_kmalloc_index(aligned_size as usize) as Int;
    }
    if kmalloc_caches[t][aligned_idx as usize].is_null() {
        kmalloc_caches[t][aligned_idx as usize] = create_kmalloc_cache(
            kmalloc_info[aligned_idx as usize].name[t],
            aligned_size,
            flags,
        );
    }
    if idx != aligned_idx {
        kmalloc_caches[t][idx as usize] = kmalloc_caches[t][aligned_idx as usize];
    }
}
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RSC_INIT_COLD, cold)]
pub unsafe extern "C" fn create_kmalloc_caches() {
    let mut cache_type = KMALLOC_NORMAL;
    while cache_type < NR_KMALLOC_TYPES {
        if RSC_KMALLOC_MIN_SIZE <= 32 {
            new_kmalloc_cache(1, cache_type);
        }
        if RSC_KMALLOC_MIN_SIZE <= 64 {
            new_kmalloc_cache(2, cache_type);
        }
        let mut i = RSC_KMALLOC_SHIFT_LOW as Int;
        while i <= RSC_KMALLOC_SHIFT_HIGH as Int {
            new_kmalloc_cache(i, cache_type);
            i += 1;
        }
        cache_type += 1;
    }
    #[cfg(CONFIG_KMALLOC_PARTITION_RANDOM)]
    {
        random_kmalloc_seed = get_random_u64() as ULong;
    }
    slab_state = UP;
    if cfg!(CONFIG_SLAB_BUCKETS) {
        let mut args: kmem_cache_args = zeroed();
        kmem_buckets_cache = __kmem_cache_create_args(
            c"kmalloc_buckets".as_ptr(),
            size_of::<kmem_buckets>() as UInt,
            addr_of_mut!(args),
            RSC_SLAB_NO_MERGE,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn kmalloc_fix_flags(mut flags: gfp_t) -> gfp_t {
    let invalid_mask = flags & RSC_GFP_SLAB_BUG_MASK;
    flags &= !RSC_GFP_SLAB_BUG_MASK;
    sc_log!(
        "\x014Unexpected gfp: %#x (%pGg). Fixing up to gfp: %#x (%pGg). Fix your code!\n",
        invalid_mask,
        addr_of!(invalid_mask),
        flags,
        addr_of!(flags)
    );
    rust_slab_common_dump_stack();
    flags
}
#[cfg(CONFIG_SLAB_FREELIST_RANDOM)]
unsafe fn freelist_randomize(list: *mut UInt, count: UInt) {
    for i in 0..count {
        *list.add(i as usize) = i;
    }
    // Callers guarantee count >= 2, matching the C count - 1 precondition.
    let mut i = count.wrapping_sub(1);
    while i > 0 {
        let rand = rust_slab_common_get_random_u32_below(i.wrapping_add(1));
        core::ptr::swap(list.add(i as usize), list.add(rand as usize));
        i -= 1;
    }
}
#[cfg(CONFIG_SLAB_FREELIST_RANDOM)]
#[no_mangle]
pub unsafe extern "C" fn cache_random_seq_create(
    cachep: *mut bindings::kmem_cache,
    count: UInt,
    gfp: gfp_t,
) -> Int {
    if count < 2 || !(*cachep).random_seq.is_null() {
        return 0;
    }
    (*cachep).random_seq = rust_slab_common_kcalloc(count as usize, size_of::<UInt>(), gfp).cast();
    if (*cachep).random_seq.is_null() {
        return -(RSC_ENOMEM as Int);
    }
    freelist_randomize((*cachep).random_seq, count);
    0
}
#[cfg(CONFIG_SLAB_FREELIST_RANDOM)]
#[no_mangle]
pub unsafe extern "C" fn cache_random_seq_destroy(cachep: *mut bindings::kmem_cache) {
    kfree((*cachep).random_seq.cast());
    (*cachep).random_seq = null_mut();
}
