// SPDX-License-Identifier: GPL-2.0-only
// Included in the allocator's single Rust namespace after page_alloc_late.rs.

struct PaZones(*mut zone);
impl Iterator for PaZones {
    type Item = *mut zone;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_null() {
            None
        } else {
            unsafe {
                let z = self.0;
                self.0 = next_zone(z);
                Some(z)
            }
        }
    }
}
unsafe fn pa_zones() -> PaZones {
    PaZones(core::ptr::addr_of_mut!((*first_online_pgdat()).node_zones).cast())
}
struct PaNodes(*mut pglist_data);
impl Iterator for PaNodes {
    type Item = *mut pglist_data;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_null() {
            None
        } else {
            unsafe {
                let n = self.0;
                self.0 = next_online_pgdat(n);
                Some(n)
            }
        }
    }
}
unsafe fn pa_nodes() -> PaNodes {
    PaNodes(first_online_pgdat())
}
struct PaCpus {
    previous: i32,
    online: bool,
}
impl Iterator for PaCpus {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        unsafe {
            let cpu = rust_pa_late_next_cpu(self.previous, self.online);
            if cpu >= rust_pa_late_nr_cpu_ids() {
                None
            } else {
                self.previous = cpu as i32;
                Some(cpu)
            }
        }
    }
}
fn pa_possible_cpus() -> PaCpus {
    PaCpus {
        previous: -1,
        online: false,
    }
}
fn pa_online_cpus() -> PaCpus {
    PaCpus {
        previous: -1,
        online: true,
    }
}

unsafe fn nr_free_zone_pages(offset: i32) -> ULong {
    let list = rust_pa_late_node_zonelist(rust_pa_late_numa_node_id(), RUST_PA_GFP_KERNEL);
    let mut z = rust_pa_late_first_zones_zonelist(list, offset as zone_type, null());
    let mut sum: ULong = 0;
    while !(*z).zone.is_null() {
        let size = rust_pa_late_zone_managed_pages((*z).zone);
        let high = rust_pa_late_high_wmark_pages((*z).zone);
        if size > high {
            sum = sum.wrapping_add(size - high);
        }
        z = rust_pa_late_next_zones_zonelist(z.add(1), offset as zone_type, null());
    }
    sum
}
#[no_mangle]
pub unsafe extern "C" fn nr_free_buffer_pages() -> ULong {
    nr_free_zone_pages(rust_pa_late_gfp_zone(RUST_PA_GFP_USER) as i32)
}
unsafe fn zoneref_set_zone(z: *mut zone, reference: *mut zoneref) {
    (*reference).zone = z;
    (*reference).zone_idx = rust_pa_late_zone_idx(z);
}
unsafe fn build_zonerefs_node(pgdat: *mut pglist_data, references: *mut zoneref) -> i32 {
    let mut count = 0;
    let mut index = MAX_NR_ZONES;
    while index != 0 {
        index -= 1;
        let z = core::ptr::addr_of_mut!((*pgdat).node_zones[index as usize]);
        if rust_pa_late_populated_zone(z) {
            zoneref_set_zone(z, references.add(count as usize));
            count += 1;
            rust_pa_late_check_highest_zone(index);
        }
    }
    count
}
#[cfg(CONFIG_NUMA)]
unsafe fn __parse_numa_zonelist_order(s: *mut CChar) -> i32 {
    if !matches!(*s as u8, b'd' | b'D' | b'n' | b'N') {
        rust_pa_late_print_bad_zonelist_order(s);
        return -(RUST_PA_EINVAL as i32);
    }
    0
}
#[cfg(CONFIG_NUMA)]
static mut numa_zonelist_order: [u8; 5] = *b"Node\0";
const NUMA_ZONELIST_ORDER_LEN: i32 = 16;
#[cfg(CONFIG_NUMA)]
unsafe extern "C" fn numa_zonelist_order_handler(
    table: *const ctl_table,
    write: i32,
    buffer: *mut Void,
    length: *mut usize,
    pos: *mut loff_t,
) -> i32 {
    if write != 0 {
        __parse_numa_zonelist_order(buffer.cast())
    } else {
        proc_dostring(table, write, buffer, length, pos)
    }
}
#[cfg(CONFIG_NUMA)]
static mut node_load: [i32; RUST_PA_MAX_NUMNODES as usize] = [0; RUST_PA_MAX_NUMNODES as usize];
#[cfg(CONFIG_NUMA)]
#[no_mangle]
pub unsafe extern "C" fn find_next_best_node(node: i32, used: *mut nodemask_t) -> i32 {
    if !rust_pa_late_node_isset(node, used) && rust_pa_late_node_state(node, N_MEMORY) {
        rust_pa_late_node_set(node, used);
        return node;
    }
    let mut best = RUST_PA_NUMA_NO_NODE as i32;
    let mut min_val = i32::MAX;
    let mut n = rust_pa_late_first_node_state(N_MEMORY);
    while n < RUST_PA_MAX_NUMNODES as i32 {
        if !rust_pa_late_node_isset(n, used) {
            let mut val = rust_pa_late_node_distance(node, n) + (n < node) as i32;
            if !rust_pa_late_node_cpus_empty(n) {
                val += RUST_PA_PENALTY_FOR_NODE_WITH_CPUS as i32;
            }
            val = val
                .wrapping_mul(RUST_PA_MAX_NUMNODES as i32)
                .wrapping_add(node_load[n as usize]);
            if val < min_val {
                min_val = val;
                best = n;
            }
        }
        n = rust_pa_late_next_node_state(n, N_MEMORY);
    }
    if best >= 0 {
        rust_pa_late_node_set(best, used);
    }
    best
}
#[cfg(CONFIG_NUMA)]
unsafe fn build_zonelists_in_node_order(pgdat: *mut pglist_data, order: *const i32, count: u32) {
    let mut refs =
        core::ptr::addr_of_mut!((*pgdat).node_zonelists[ZONELIST_FALLBACK as usize]._zonerefs)
            .cast::<zoneref>();
    for i in 0..count {
        refs = refs.add(
            build_zonerefs_node(rust_pa_late_node_data(*order.add(i as usize)), refs) as usize,
        );
    }
    (*refs).zone = null_mut();
    (*refs).zone_idx = 0;
}
#[cfg(CONFIG_NUMA)]
unsafe fn build_thisnode_zonelists(pgdat: *mut pglist_data) {
    let mut refs =
        core::ptr::addr_of_mut!((*pgdat).node_zonelists[ZONELIST_NOFALLBACK as usize]._zonerefs)
            .cast::<zoneref>();
    refs = refs.add(build_zonerefs_node(pgdat, refs) as usize);
    (*refs).zone = null_mut();
    (*refs).zone_idx = 0;
}
#[cfg(CONFIG_NUMA)]
unsafe fn build_zonelists(pgdat: *mut pglist_data) {
    static mut NODE_ORDER: [i32; RUST_PA_MAX_NUMNODES as usize] =
        [0; RUST_PA_MAX_NUMNODES as usize];
    let mut used: nodemask_t = zeroed();
    let local = (*pgdat).node_id;
    let mut prev = local;
    NODE_ORDER = [0; RUST_PA_MAX_NUMNODES as usize];
    let mut count = 0u32;
    loop {
        let node = find_next_best_node(local, &mut used);
        if node < 0 {
            break;
        }
        if rust_pa_late_node_distance(local, node) != rust_pa_late_node_distance(local, prev) {
            node_load[node as usize] += 1;
        }
        NODE_ORDER[count as usize] = node;
        count += 1;
        prev = node;
    }
    build_zonelists_in_node_order(pgdat, core::ptr::addr_of!(NODE_ORDER).cast(), count);
    build_thisnode_zonelists(pgdat);
    rust_pa_late_print_fallback_node(local);
    for i in 0..count {
        rust_pa_late_print_node(NODE_ORDER[i as usize]);
    }
    rust_pa_late_print_newline();
}
#[cfg(not(CONFIG_NUMA))]
unsafe fn build_zonelists(pgdat: *mut pglist_data) {
    let mut refs =
        core::ptr::addr_of_mut!((*pgdat).node_zonelists[ZONELIST_FALLBACK as usize]._zonerefs)
            .cast::<zoneref>();
    refs = refs.add(build_zonerefs_node(pgdat, refs) as usize);
    (*refs).zone = null_mut();
    (*refs).zone_idx = 0;
}
#[cfg(all(CONFIG_NUMA, CONFIG_HAVE_MEMORYLESS_NODES))]
#[no_mangle]
pub unsafe extern "C" fn local_memory_node(node: i32) -> i32 {
    let refs = rust_pa_late_first_zones_zonelist(
        rust_pa_late_node_zonelist(node, RUST_PA_GFP_KERNEL),
        rust_pa_late_gfp_zone(RUST_PA_GFP_KERNEL),
        null(),
    );
    rust_pa_late_zonelist_node_idx(refs)
}
const BOOT_PAGESET_HIGH: i32 = 0;
const BOOT_PAGESET_BATCH: i32 = 1;
unsafe fn __build_all_zonelists(data: *mut Void) {
    let own = data.cast::<pglist_data>();
    let flags = rust_pa_late_write_zonelist_lock();
    rust_pa_late_printk_deferred_enter();
    #[cfg(CONFIG_NUMA)]
    {
        node_load = [0; RUST_PA_MAX_NUMNODES as usize];
    }
    if !own.is_null() && !rust_pa_late_node_state((*own).node_id, N_ONLINE) {
        build_zonelists(own);
    } else {
        let mut nid = rust_pa_late_first_node_state(N_POSSIBLE);
        while nid < RUST_PA_MAX_NUMNODES as i32 {
            build_zonelists(rust_pa_late_node_data(nid));
            nid = rust_pa_late_next_node_state(nid, N_POSSIBLE);
        }
        #[cfg(CONFIG_HAVE_MEMORYLESS_NODES)]
        for cpu in pa_online_cpus() {
            rust_pa_late_set_cpu_numa_mem(cpu, local_memory_node(rust_pa_late_cpu_to_node(cpu)));
        }
    }
    rust_pa_late_printk_deferred_exit();
    rust_pa_late_write_zonelist_unlock(flags);
}
#[inline(never)]
#[cold]
#[link_section = ".init.text"]
unsafe fn build_all_zonelists_init() {
    __build_all_zonelists(null_mut());
    for cpu in pa_possible_cpus() {
        per_cpu_pages_init(
            rust_pa_late_boot_pageset_cpu(cpu),
            rust_pa_late_boot_zonestats_cpu(cpu),
        );
    }
    rust_pa_late_mminit_verify_zonelist();
    rust_pa_late_cpuset_init_current_mems_allowed();
}
#[no_mangle]
#[inline(never)]
#[link_section = ".ref.text"]
pub unsafe extern "C" fn build_all_zonelists(pgdat: *mut pglist_data) {
    if system_state == SYSTEM_BOOTING {
        build_all_zonelists_init();
    } else {
        __build_all_zonelists(pgdat.cast());
    }
    let total = nr_free_zone_pages(rust_pa_late_gfp_zone(RUST_PA_GFP_HIGHUSER_MOVABLE) as i32);
    page_group_by_mobility_disabled = (total < pb_pages() * MIGRATE_TYPES as ULong) as i32;
    rust_pa_late_print_built_zonelists(
        rust_pa_late_nr_online_nodes(),
        page_group_by_mobility_disabled,
        total,
    );
    #[cfg(CONFIG_NUMA)]
    {
        rust_pa_late_print_policy_zone();
    }
}
unsafe fn zone_batchsize(z: *mut zone) -> i32 {
    #[cfg(CONFIG_MMU)]
    {
        let mut batch = min(
            rust_pa_late_zone_managed_pages(z) >> 12,
            RUST_PA_SZ_256K as ULong / RUST_PA_PAGE_SIZE as ULong,
        ) as i32;
        if batch <= 1 {
            return 1;
        }
        let value = (batch + batch / 2) as u32;
        batch = (1u32 << (u32::BITS - 1 - value.leading_zeros())) as i32 - 1;
        batch
    }
    #[cfg(not(CONFIG_MMU))]
    {
        1
    }
}
static mut percpu_pagelist_high_fraction: i32 = 0;
unsafe fn zone_highsize(z: *mut zone, batch: i32, online: i32, fraction: i32) -> i32 {
    #[cfg(CONFIG_MMU)]
    {
        let total = if fraction == 0 {
            rust_pa_late_low_wmark_pages(z)
        } else {
            rust_pa_late_zone_managed_pages(z) / fraction as ULong
        };
        let mut cpus = rust_pa_late_node_cpu_count(rust_pa_late_zone_to_nid(z)) as i32 + online;
        if cpus == 0 {
            cpus = rust_pa_late_num_online_cpus() as i32;
        }
        max((total / cpus as ULong) as i32, batch << 2)
    }
    #[cfg(not(CONFIG_MMU))]
    {
        0
    }
}
unsafe fn pageset_update(pcp: *mut per_cpu_pages, low: ULong, high: ULong, batch: ULong) {
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*pcp).batch), batch as i32);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*pcp).high_min), low as i32);
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*pcp).high_max), high as i32);
}
unsafe fn per_cpu_pages_init(pcp: *mut per_cpu_pages, stats: *mut per_cpu_zonestat) {
    core::ptr::write_bytes(pcp, 0, 1);
    // Empty per_cpu_zonestat is allowed by the native !SMP && !NUMA ABI.
    if core::mem::size_of::<per_cpu_zonestat>() != 0 {
        core::ptr::write_bytes(stats, 0, 1);
    }
    rust_pa_late_pcp_lock_init(core::ptr::addr_of_mut!((*pcp).lock));
    for i in 0..RUST_PA_NR_PCP_LISTS as usize {
        let h = core::ptr::addr_of_mut!((*pcp).lists[i]);
        (*h).next = h;
        (*h).prev = h;
    }
    (*pcp).high_min = BOOT_PAGESET_HIGH;
    (*pcp).high_max = BOOT_PAGESET_HIGH;
    (*pcp).batch = BOOT_PAGESET_BATCH;
}
unsafe fn __zone_set_pageset_high_and_batch(z: *mut zone, low: ULong, high: ULong, batch: ULong) {
    for cpu in pa_possible_cpus() {
        pageset_update(
            rust_pa_late_per_cpu_pages((*z).per_cpu_pageset, cpu),
            low,
            high,
            batch,
        );
    }
}
unsafe fn zone_set_pageset_high_and_batch(z: *mut zone, online: i32) {
    let batch = zone_batchsize(z);
    let low;
    let high;
    if percpu_pagelist_high_fraction != 0 {
        low = zone_highsize(z, batch, online, percpu_pagelist_high_fraction);
        high = low;
    } else {
        low = zone_highsize(z, batch, online, 0);
        high = zone_highsize(z, batch, online, MIN_PERCPU_PAGELIST_HIGH_FRACTION as i32);
    }
    if (*z).pageset_high_min == low && (*z).pageset_high_max == high && (*z).pageset_batch == batch
    {
        return;
    }
    (*z).pageset_high_min = low;
    (*z).pageset_high_max = high;
    (*z).pageset_batch = batch;
    __zone_set_pageset_high_and_batch(z, low as ULong, high as ULong, batch as ULong);
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
pub unsafe extern "C" fn setup_zone_pageset(z: *mut zone) {
    if core::mem::size_of::<per_cpu_zonestat>() > 0 {
        (*z).per_cpu_zonestats = rust_pa_late_alloc_percpu_zonestats();
    }
    (*z).per_cpu_pageset = rust_pa_late_alloc_percpu_pages();
    for cpu in pa_possible_cpus() {
        per_cpu_pages_init(
            rust_pa_late_per_cpu_pages((*z).per_cpu_pageset, cpu),
            rust_pa_late_per_cpu_zonestats((*z).per_cpu_zonestats, cpu),
        );
    }
    zone_set_pageset_high_and_batch(z, 0);
}
unsafe fn zone_pcp_update(z: *mut zone, online: i32) {
    rust_pa_late_pcp_batch_lock();
    zone_set_pageset_high_and_batch(z, online);
    rust_pa_late_pcp_batch_unlock();
}
unsafe fn zone_pcp_update_cacheinfo(z: *mut zone, cpu: u32) {
    let pcp = rust_pa_late_per_cpu_pages((*z).per_cpu_pageset, cpu);
    let cci = get_cpu_cacheinfo(cpu);
    pcp_spin_lock_nopin(pcp);
    if ((*cci).per_cpu_data_slice_size >> RUST_PA_PAGE_SHIFT) > (3 * (*pcp).batch) as u32 {
        (*pcp).flags |= RUST_PA_PCPF_FREE_HIGH_BATCH as u8;
    } else {
        (*pcp).flags &= !(RUST_PA_PCPF_FREE_HIGH_BATCH as u8);
    }
    pcp_spin_unlock_nopin(pcp);
}
#[no_mangle]
pub unsafe extern "C" fn setup_pcp_cacheinfo(cpu: u32) {
    for z in pa_zones() {
        if rust_pa_late_populated_zone(z) {
            zone_pcp_update_cacheinfo(z, cpu);
        }
    }
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn setup_per_cpu_pageset() {
    for z in pa_zones() {
        if rust_pa_late_populated_zone(z) {
            setup_zone_pageset(z);
        }
    }
    #[cfg(CONFIG_NUMA)]
    for cpu in pa_possible_cpus() {
        let stats = rust_pa_late_boot_zonestats_cpu(cpu);
        core::ptr::write_bytes(core::ptr::addr_of_mut!((*stats).vm_numa_event), 0, 1);
    }
    for node in pa_nodes() {
        (*node).per_cpu_nodestats = rust_pa_late_alloc_percpu_nodestats();
    }
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
pub unsafe extern "C" fn zone_pcp_init(z: *mut zone) {
    (*z).per_cpu_pageset = rust_pa_late_boot_pageset();
    (*z).per_cpu_zonestats = rust_pa_late_boot_zonestats();
    (*z).pageset_high_min = BOOT_PAGESET_HIGH;
    (*z).pageset_high_max = BOOT_PAGESET_HIGH;
    (*z).pageset_batch = BOOT_PAGESET_BATCH;
    if rust_pa_late_populated_zone(z) {
        rust_pa_late_print_zone_init(z, zone_batchsize(z));
    }
}
#[no_mangle]
pub unsafe extern "C" fn adjust_managed_page_count(p: *mut page, count: Long) {
    rust_pa_late_atomic_long_add(
        count,
        core::ptr::addr_of_mut!((*rust_pa_late_page_zone(p)).managed_pages),
    );
    rust_pa_late_totalram_pages_add(count);
    setup_per_zone_lowmem_reserve();
}
#[no_mangle]
pub unsafe extern "C" fn free_reserved_pages(p: *mut page, order: u32) {
    let count = (1 as ULong) << order;
    rust_pa_late_warn_reserved_alignment(rust_pa_late_page_to_pfn(p) & (count - 1) != 0);
    rust_pa_late_warn_reserved_order(order > RUST_PA_MAX_PAGE_ORDER);
    let mut i = 0i32;
    while (i as ULong) < count {
        let next = p.add(i as usize);
        rust_pa_late_clear_page_tag_ref(next);
        rust_pa_late_set_page_count(next, 0);
        rust_pa_late_clear_page_reserved(next);
        i += 1;
    }
    adjust_managed_page_count(p, count as Long);
    __free_frozen_pages(p, order, FPI_NONE);
}
unsafe extern "C" fn page_alloc_cpu_dead(cpu: u32) -> i32 {
    lru_add_drain_cpu(cpu as i32);
    rust_pa_late_mlock_drain_remote(cpu as i32);
    drain_pages(cpu);
    rust_pa_late_vm_events_fold_cpu(cpu as i32);
    rust_pa_late_cpu_vm_stats_fold(cpu as i32);
    for z in pa_zones() {
        if rust_pa_late_populated_zone(z) {
            zone_pcp_update(z, 0);
        }
    }
    0
}
unsafe extern "C" fn page_alloc_cpu_online(_cpu: u32) -> i32 {
    for z in pa_zones() {
        if rust_pa_late_populated_zone(z) {
            zone_pcp_update(z, 1);
        }
    }
    0
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn page_alloc_init_cpuhp() {
    let ret = rust_pa_late_cpuhp_setup(Some(page_alloc_cpu_online), Some(page_alloc_cpu_dead));
    rust_pa_late_warn_cpuhp(ret < 0);
}
unsafe fn calculate_totalreserve_pages() {
    let mut reserve: ULong = 0;
    for pgdat in pa_nodes() {
        (*pgdat).totalreserve_pages = 0;
        for i in 0..MAX_NR_ZONES as usize {
            let z = core::ptr::addr_of_mut!((*pgdat).node_zones[i]);
            let mut value: Long = 0;
            let mut j = MAX_NR_ZONES as usize - 1;
            while j > i {
                if (*z).lowmem_reserve[j] != 0 {
                    value = (*z).lowmem_reserve[j];
                    break;
                }
                j -= 1;
            }
            value = (value as ULong).wrapping_add(rust_pa_late_high_wmark_pages(z)) as Long;
            value = min(value as ULong, rust_pa_late_zone_managed_pages(z)) as Long;
            (*pgdat).totalreserve_pages = (*pgdat).totalreserve_pages.wrapping_add(value as ULong);
            reserve = reserve.wrapping_add(value as ULong);
        }
    }
    totalreserve_pages = reserve;
    rust_pa_late_trace_totalreserve(reserve);
}
unsafe fn setup_per_zone_lowmem_reserve() {
    for pgdat in pa_nodes() {
        for i in 0..MAX_NR_ZONES as usize - 1 {
            let z = core::ptr::addr_of_mut!((*pgdat).node_zones[i]);
            let ratio = sysctl_lowmem_reserve_ratio[i];
            let clear = ratio == 0 || rust_pa_late_zone_managed_pages(z) == 0;
            let mut managed: ULong = 0;
            for j in i + 1..MAX_NR_ZONES as usize {
                let upper = core::ptr::addr_of_mut!((*pgdat).node_zones[j]);
                managed = managed.wrapping_add(rust_pa_late_zone_managed_pages(upper));
                (*z).lowmem_reserve[j] = if clear {
                    0
                } else {
                    (managed / ratio as ULong) as Long
                };
                rust_pa_late_trace_lowmem_reserve(z, upper, (*z).lowmem_reserve[j]);
            }
        }
    }
    calculate_totalreserve_pages();
}
unsafe fn __setup_per_zone_wmarks() {
    let pages_min = (min_free_kbytes >> (RUST_PA_PAGE_SHIFT - 10)) as ULong;
    let mut lowmem: ULong = 0;
    for z in pa_zones() {
        if !rust_pa_late_is_highmem(z) && rust_pa_late_zone_idx(z) != ZONE_MOVABLE as i32 {
            lowmem = lowmem.wrapping_add(rust_pa_late_zone_managed_pages(z));
        }
    }
    for z in pa_zones() {
        let flags = rust_pa_late_zone_lock_irqsave(z);
        let mut tmp = (pages_min as u64).wrapping_mul(rust_pa_late_zone_managed_pages(z) as u64)
            / lowmem as u64;
        (*z)._watermark[WMARK_MIN as usize] =
            if rust_pa_late_is_highmem(z) || rust_pa_late_zone_idx(z) == ZONE_MOVABLE as i32 {
                (rust_pa_late_zone_managed_pages(z) / 1024)
                    .clamp(RUST_PA_SWAP_CLUSTER_MAX as ULong, 128)
            } else {
                tmp as ULong
            };
        let managed = rust_pa_late_zone_managed_pages(z);
        // mult_frac avoids overflow by dividing before multiplying the quotient.
        let scaled = (managed / 10000)
            .wrapping_mul(watermark_scale_factor as ULong)
            .wrapping_add((managed % 10000).wrapping_mul(watermark_scale_factor as ULong) / 10000);
        tmp = max(tmp >> 2, scaled as u64);
        (*z).watermark_boost = 0;
        (*z)._watermark[WMARK_LOW as usize] =
            rust_pa_late_min_wmark_pages(z).wrapping_add(tmp as ULong);
        (*z)._watermark[WMARK_HIGH as usize] =
            rust_pa_late_low_wmark_pages(z).wrapping_add(tmp as ULong);
        (*z)._watermark[WMARK_PROMO as usize] =
            rust_pa_late_high_wmark_pages(z).wrapping_add(tmp as ULong);
        rust_pa_late_trace_wmarks(z);
        rust_pa_late_zone_unlock_irqrestore(z, flags);
    }
    calculate_totalreserve_pages();
}
#[no_mangle]
pub unsafe extern "C" fn setup_per_zone_wmarks() {
    rust_pa_late_wmark_lock();
    __setup_per_zone_wmarks();
    rust_pa_late_wmark_unlock();
    for z in pa_zones() {
        zone_pcp_update(z, 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn calculate_min_free_kbytes() {
    let kb = nr_free_buffer_pages().wrapping_mul((RUST_PA_PAGE_SIZE >> 10) as ULong);
    let new_value = int_sqrt(kb.wrapping_mul(16)) as i32;
    if new_value > user_min_free_kbytes {
        min_free_kbytes = new_value.clamp(128, 262144);
    } else {
        rust_pa_late_print_min_free_unchanged(new_value, user_min_free_kbytes);
    }
}
#[no_mangle]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
pub unsafe extern "C" fn init_per_zone_wmark_min() -> i32 {
    calculate_min_free_kbytes();
    setup_per_zone_wmarks();
    rust_pa_late_refresh_zone_stat_thresholds();
    setup_per_zone_lowmem_reserve();
    #[cfg(CONFIG_NUMA)]
    {
        setup_min_unmapped_ratio();
        setup_min_slab_ratio();
    }
    rust_pa_late_khugepaged_min_free_kbytes_update();
    0
}

unsafe extern "C" fn min_free_kbytes_sysctl_handler(
    t: *const ctl_table,
    write: i32,
    buffer: *mut Void,
    len: *mut usize,
    pos: *mut loff_t,
) -> i32 {
    let rc = proc_dointvec_minmax(t, write, buffer, len, pos);
    if rc != 0 {
        return rc;
    }
    if write != 0 {
        user_min_free_kbytes = min_free_kbytes;
        setup_per_zone_wmarks();
    }
    0
}
unsafe extern "C" fn watermark_scale_factor_sysctl_handler(
    t: *const ctl_table,
    write: i32,
    buffer: *mut Void,
    len: *mut usize,
    pos: *mut loff_t,
) -> i32 {
    let rc = proc_dointvec_minmax(t, write, buffer, len, pos);
    if rc != 0 {
        return rc;
    }
    if write != 0 {
        setup_per_zone_wmarks();
    }
    0
}
#[cfg(CONFIG_NUMA)]
unsafe fn setup_min_unmapped_ratio() {
    for p in pa_nodes() {
        (*p).min_unmapped_pages = 0;
    }
    for z in pa_zones() {
        let p = (*z).zone_pgdat;
        (*p).min_unmapped_pages = (*p).min_unmapped_pages.wrapping_add(
            rust_pa_late_zone_managed_pages(z).wrapping_mul(sysctl_min_unmapped_ratio as ULong)
                / 100,
        );
    }
}
#[cfg(CONFIG_NUMA)]
unsafe extern "C" fn sysctl_min_unmapped_ratio_sysctl_handler(
    t: *const ctl_table,
    write: i32,
    buffer: *mut Void,
    len: *mut usize,
    pos: *mut loff_t,
) -> i32 {
    let rc = proc_dointvec_minmax(t, write, buffer, len, pos);
    if rc != 0 {
        return rc;
    }
    if write != 0 {
        setup_min_unmapped_ratio();
    }
    0
}
#[cfg(CONFIG_NUMA)]
unsafe fn setup_min_slab_ratio() {
    for p in pa_nodes() {
        (*p).min_slab_pages = 0;
    }
    for z in pa_zones() {
        let p = (*z).zone_pgdat;
        (*p).min_slab_pages = (*p).min_slab_pages.wrapping_add(
            rust_pa_late_zone_managed_pages(z).wrapping_mul(sysctl_min_slab_ratio as ULong) / 100,
        );
    }
}
#[cfg(CONFIG_NUMA)]
unsafe extern "C" fn sysctl_min_slab_ratio_sysctl_handler(
    t: *const ctl_table,
    write: i32,
    buffer: *mut Void,
    len: *mut usize,
    pos: *mut loff_t,
) -> i32 {
    let rc = proc_dointvec_minmax(t, write, buffer, len, pos);
    if rc != 0 {
        return rc;
    }
    if write != 0 {
        setup_min_slab_ratio();
    }
    0
}
unsafe extern "C" fn lowmem_reserve_ratio_sysctl_handler(
    t: *const ctl_table,
    write: i32,
    buffer: *mut Void,
    len: *mut usize,
    pos: *mut loff_t,
) -> i32 {
    if write == 0 {
        return proc_dointvec_minmax(t, write, buffer, len, pos);
    }
    let mut tmp = *t;
    let mut ratios = sysctl_lowmem_reserve_ratio;
    tmp.data = ratios.as_mut_ptr().cast();
    let rc = proc_dointvec_minmax(&tmp, write, buffer, len, pos);
    if rc != 0 {
        return rc;
    }
    sysctl_lowmem_reserve_ratio = ratios;
    setup_per_zone_lowmem_reserve();
    0
}
unsafe extern "C" fn percpu_pagelist_high_fraction_sysctl_handler(
    t: *const ctl_table,
    write: i32,
    buffer: *mut Void,
    len: *mut usize,
    pos: *mut loff_t,
) -> i32 {
    if write == 0 {
        return proc_dointvec_minmax(t, write, buffer, len, pos);
    }
    rust_pa_late_pcp_batch_lock();
    let old = percpu_pagelist_high_fraction;
    let mut ret = proc_dointvec_minmax(t, write, buffer, len, pos);
    if ret >= 0 {
        if percpu_pagelist_high_fraction != 0
            && percpu_pagelist_high_fraction < MIN_PERCPU_PAGELIST_HIGH_FRACTION as i32
        {
            percpu_pagelist_high_fraction = old;
            ret = -(RUST_PA_EINVAL as i32);
        } else if percpu_pagelist_high_fraction != old {
            for z in pa_zones() {
                if rust_pa_late_populated_zone(z) {
                    zone_set_pageset_high_and_batch(z, 0);
                }
            }
        }
    }
    rust_pa_late_pcp_batch_unlock();
    ret
}

// The native table is const: only its pointed-to sysctl values are mutable.
const PA_SYSCTL_COUNT: usize = 6 + if cfg!(CONFIG_NUMA) { 3 } else { 0 };
#[repr(transparent)]
struct PaSysctlTable([ctl_table; PA_SYSCTL_COUNT]);
// The immutable descriptor holds native static data pointers and function
// pointers. The proc/sysctl subsystem synchronizes access to pointed-to data.
unsafe impl Sync for PaSysctlTable {}
type PaProcHandler =
    unsafe extern "C" fn(*const ctl_table, i32, *mut Void, *mut usize, *mut loff_t) -> i32;
const unsafe fn pa_sysctl_value(index: u32) -> *const Void {
    core::ptr::addr_of!(sysctl_vals)
        .cast::<i32>()
        .add(index as usize)
        .cast()
}
const unsafe fn pa_sysctl_entry(
    name: *const CChar,
    data: *mut Void,
    size: i32,
    handler: PaProcHandler,
    low: *const Void,
    high: *const Void,
) -> ctl_table {
    let mut t: ctl_table = zeroed();
    t.procname = name;
    t.data = data;
    t.maxlen = size;
    t.mode = 0o644;
    t.proc_handler = Some(handler);
    t.extra1 = low.cast_mut();
    t.extra2 = high.cast_mut();
    t
}
static page_alloc_sysctl_table: PaSysctlTable = unsafe {
    PaSysctlTable([
        pa_sysctl_entry(
            c"min_free_kbytes".as_ptr(),
            core::ptr::addr_of_mut!(min_free_kbytes).cast(),
            core::mem::size_of::<i32>() as i32,
            min_free_kbytes_sysctl_handler,
            pa_sysctl_value(RUST_PA_LATE_SYSCTL_ZERO_INDEX),
            null(),
        ),
        pa_sysctl_entry(
            c"watermark_boost_factor".as_ptr(),
            core::ptr::addr_of_mut!(watermark_boost_factor).cast(),
            core::mem::size_of::<i32>() as i32,
            proc_dointvec_minmax,
            pa_sysctl_value(RUST_PA_LATE_SYSCTL_ZERO_INDEX),
            null(),
        ),
        pa_sysctl_entry(
            c"watermark_scale_factor".as_ptr(),
            core::ptr::addr_of_mut!(watermark_scale_factor).cast(),
            core::mem::size_of::<i32>() as i32,
            watermark_scale_factor_sysctl_handler,
            pa_sysctl_value(RUST_PA_LATE_SYSCTL_ONE_INDEX),
            pa_sysctl_value(RUST_PA_LATE_SYSCTL_THREE_THOUSAND_INDEX),
        ),
        pa_sysctl_entry(
            c"defrag_mode".as_ptr(),
            core::ptr::addr_of_mut!(defrag_mode).cast(),
            core::mem::size_of::<i32>() as i32,
            proc_dointvec_minmax,
            pa_sysctl_value(RUST_PA_LATE_SYSCTL_ZERO_INDEX),
            pa_sysctl_value(RUST_PA_LATE_SYSCTL_ONE_INDEX),
        ),
        pa_sysctl_entry(
            c"percpu_pagelist_high_fraction".as_ptr(),
            core::ptr::addr_of_mut!(percpu_pagelist_high_fraction).cast(),
            core::mem::size_of::<i32>() as i32,
            percpu_pagelist_high_fraction_sysctl_handler,
            pa_sysctl_value(RUST_PA_LATE_SYSCTL_ZERO_INDEX),
            null(),
        ),
        pa_sysctl_entry(
            c"lowmem_reserve_ratio".as_ptr(),
            core::ptr::addr_of_mut!(sysctl_lowmem_reserve_ratio).cast(),
            core::mem::size_of::<[i32; RUST_PA_MAX_NR_ZONES as usize]>() as i32,
            lowmem_reserve_ratio_sysctl_handler,
            pa_sysctl_value(RUST_PA_LATE_SYSCTL_ZERO_INDEX),
            null(),
        ),
        #[cfg(CONFIG_NUMA)]
        pa_sysctl_entry(
            c"numa_zonelist_order".as_ptr(),
            core::ptr::addr_of_mut!(numa_zonelist_order).cast(),
            NUMA_ZONELIST_ORDER_LEN,
            numa_zonelist_order_handler,
            null(),
            null(),
        ),
        #[cfg(CONFIG_NUMA)]
        pa_sysctl_entry(
            c"min_unmapped_ratio".as_ptr(),
            core::ptr::addr_of_mut!(sysctl_min_unmapped_ratio).cast(),
            core::mem::size_of::<i32>() as i32,
            sysctl_min_unmapped_ratio_sysctl_handler,
            pa_sysctl_value(RUST_PA_LATE_SYSCTL_ZERO_INDEX),
            pa_sysctl_value(RUST_PA_LATE_SYSCTL_ONE_HUNDRED_INDEX),
        ),
        #[cfg(CONFIG_NUMA)]
        pa_sysctl_entry(
            c"min_slab_ratio".as_ptr(),
            core::ptr::addr_of_mut!(sysctl_min_slab_ratio).cast(),
            core::mem::size_of::<i32>() as i32,
            sysctl_min_slab_ratio_sysctl_handler,
            pa_sysctl_value(RUST_PA_LATE_SYSCTL_ZERO_INDEX),
            pa_sysctl_value(RUST_PA_LATE_SYSCTL_ONE_HUNDRED_INDEX),
        ),
    ])
};
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn page_alloc_sysctl_init() {
    rust_pa_late_register_sysctl(
        core::ptr::addr_of!(page_alloc_sysctl_table.0).cast(),
        PA_SYSCTL_COUNT,
    );
}

unsafe fn free_prepared_contig_range(mut p: *mut page, mut count: ULong) {
    let mut pfn = rust_pa_late_page_to_pfn(p);
    while count != 0 {
        let mut order = if pfn != 0 {
            pfn.trailing_zeros()
        } else {
            RUST_PA_MAX_PAGE_ORDER
        };
        order = min(order, ULong::BITS - 1 - count.leading_zeros());
        order = min(order, RUST_PA_MAX_PAGE_ORDER);
        __free_frozen_pages(p, order, FPI_PREPARED);
        let step = (1 as ULong) << order;
        pfn = pfn.wrapping_add(step);
        p = p.add(step as usize);
        count -= step;
    }
}
unsafe fn __free_contig_range_common(pfn: ULong, count: ULong, frozen: bool) {
    let mut start: *mut page = null_mut();
    let mut start_index: ULong = 0;
    let mut start_section: ULong = 0;
    for i in 0..count {
        let p = rust_pa_late_pfn_to_page(pfn.wrapping_add(i));
        rust_pa_late_warn_contig_head(rust_pa_late_page_head(p));
        rust_pa_late_warn_contig_tail(rust_pa_late_page_tail(p));
        let mut can_free = true;
        if !frozen {
            can_free = rust_pa_late_put_page_testzero(p);
        }
        if can_free {
            can_free = free_pages_prepare(p, 0);
        }
        if !can_free {
            if !start.is_null() {
                free_prepared_contig_range(start, i - start_index);
                start = null_mut();
            }
            continue;
        }
        let sec = rust_pa_late_memdesc_section(p);
        if !start.is_null() && sec != start_section {
            free_prepared_contig_range(start, i - start_index);
            start = p;
            start_index = i;
            start_section = sec;
        } else if start.is_null() {
            start = p;
            start_index = i;
            start_section = sec;
        }
    }
    if !start.is_null() {
        free_prepared_contig_range(start, count - start_index);
    }
}
#[no_mangle]
pub unsafe extern "C" fn __free_contig_range(pfn: ULong, count: ULong) {
    __free_contig_range_common(pfn, count, false);
}

#[cfg(CONFIG_CONTIG_ALLOC)]
unsafe fn alloc_contig_dump_pages(list: *mut list_head) {
    if rust_pa_late_migrate_debug_enabled() {
        rust_pa_late_dump_stack();
        let mut entry = (*list).next;
        while entry != list {
            dump_page(rust_pa_late_lru_page(entry), c"migration failure".as_ptr());
            entry = (*entry).next;
        }
    }
}
#[cfg(CONFIG_CONTIG_ALLOC)]
unsafe fn __alloc_contig_migrate_range(cc: *mut compact_control, start: ULong, end: ULong) -> i32 {
    let mut pfn = start;
    let mut tries = 0u32;
    let mut ret = 0;
    let mut mtc: migration_target_control = zeroed();
    mtc.nid = rust_pa_late_zone_to_nid((*cc).zone);
    mtc.gfp_mask = (*cc).gfp_mask;
    mtc.reason = MR_CONTIG_RANGE;
    lru_cache_disable();
    let list = core::ptr::addr_of_mut!((*cc).migratepages);
    while pfn < end || (*list).next != list {
        if rust_pa_late_fatal_signal_pending(rust_pa_late_current()) {
            ret = -(RUST_PA_EINTR as i32);
            break;
        }
        if (*list).next == list {
            (*cc).nr_migratepages = 0;
            ret = isolate_migratepages_range(cc, pfn, end);
            if ret != 0 && ret != -(RUST_PA_EAGAIN as i32) {
                break;
            }
            pfn = (*cc).migrate_pfn;
            tries = 0;
        } else {
            tries += 1;
            if tries == 5 {
                ret = -(RUST_PA_EBUSY as i32);
                break;
            }
        }
        let reclaimed = reclaim_clean_pages_from_list((*cc).zone, list);
        (*cc).nr_migratepages = (*cc).nr_migratepages.wrapping_sub(reclaimed);
        ret = rust_pa_late_migrate_pages(list, &mut mtc, (*cc).mode);
        if ret == -(RUST_PA_ENOMEM as i32) {
            break;
        }
    }
    lru_cache_enable();
    if ret < 0 {
        if (*cc).gfp_mask & RUST_PA___GFP_NOWARN == 0 && ret == -(RUST_PA_EBUSY as i32) {
            alloc_contig_dump_pages(list);
        }
        putback_movable_pages(list);
    }
    if ret < 0 {
        ret
    } else {
        0
    }
}
#[cfg(CONFIG_CONTIG_ALLOC)]
unsafe fn split_free_frozen_pages(lists: *mut list_head, gfp: gfp_t) {
    for order in 0..RUST_PA_NR_PAGE_ORDERS {
        let count = 1i32 << order;
        let list = lists.add(order as usize);
        let mut entry = (*list).next;
        while entry != list {
            let p = rust_pa_late_lru_page(entry);
            entry = (*entry).next;
            post_alloc_hook(p, order, gfp, RUST_PA_ALLOC_DEFAULT);
            if order == 0 {
                continue;
            }
            __split_page(p, order);
            rust_pa_late_list_del(rust_pa_late_page_lru(p));
            for i in 0..count {
                rust_pa_late_list_add_tail(rust_pa_late_page_lru(p.add(i as usize)), lists);
            }
        }
    }
}
#[cfg(CONFIG_CONTIG_ALLOC)]
unsafe fn __alloc_contig_verify_gfp_mask(mut gfp: gfp_t, cc_gfp: *mut gfp_t) -> i32 {
    let reclaim = RUST_PA___GFP_IO | RUST_PA___GFP_FS | RUST_PA___GFP_RECLAIM;
    let action = RUST_PA___GFP_COMP
        | RUST_PA___GFP_RETRY_MAYFAIL
        | RUST_PA___GFP_NOWARN
        | RUST_PA___GFP_ZERO
        | RUST_PA___GFP_ZEROTAGS
        | RUST_PA___GFP_SKIP_ZERO
        | RUST_PA___GFP_SKIP_KASAN;
    let cc_action = RUST_PA___GFP_RETRY_MAYFAIL | RUST_PA___GFP_NOWARN;
    gfp &= !(RUST_PA_GFP_ZONEMASK
        | RUST_PA___GFP_RECLAIMABLE
        | RUST_PA___GFP_WRITE
        | RUST_PA___GFP_HARDWALL
        | RUST_PA___GFP_THISNODE
        | RUST_PA___GFP_MOVABLE);
    if gfp & !(reclaim | action) != 0 {
        return -(RUST_PA_EINVAL as i32);
    }
    *cc_gfp = (gfp & (reclaim | cc_action)) | RUST_PA___GFP_MOVABLE | RUST_PA___GFP_RETRY_MAYFAIL;
    0
}
#[cfg(CONFIG_CONTIG_ALLOC)]
unsafe fn __free_contig_frozen_range(pfn: ULong, count: ULong) {
    __free_contig_range_common(pfn, count, true);
}
#[cfg(CONFIG_CONTIG_ALLOC)]
#[no_mangle]
pub unsafe extern "C" fn alloc_contig_frozen_range_noprof(
    start: ULong,
    end: ULong,
    alloc_flags: acr_flags_t,
    mut gfp: gfp_t,
) -> i32 {
    let size = end.wrapping_sub(start);
    let order = rust_pa_late_ilog2(size);
    let mut cc: compact_control = zeroed();
    cc.order = -1;
    cc.zone = rust_pa_late_page_zone(rust_pa_late_pfn_to_page(start));
    cc.mode = MIGRATE_SYNC;
    cc.ignore_skip_hint = true;
    cc.no_set_skip_hint = true;
    cc.alloc_contig = true;
    let migrate = core::ptr::addr_of_mut!(cc.migratepages);
    (*migrate).next = migrate;
    (*migrate).prev = migrate;
    let mode = if alloc_flags & RUST_PA_ACR_FLAGS_CMA != 0 {
        PB_ISOLATE_MODE_CMA_ALLOC
    } else {
        PB_ISOLATE_MODE_OTHER
    };
    if rust_pa_late_warn_contig_order(
        gfp & RUST_PA___GFP_COMP != 0 && order > RUST_PA_MAX_FOLIO_ORDER,
    ) {
        return -(RUST_PA_EINVAL as i32);
    }
    gfp = rust_pa_late_current_gfp_context(gfp);
    if __alloc_contig_verify_gfp_mask(gfp, &mut cc.gfp_mask) != 0 {
        return -(RUST_PA_EINVAL as i32);
    }
    let ret = 'allocate: {
        let mut ret = start_isolate_page_range(start, end, mode);
        if ret != 0 {
            break 'allocate ret;
        }
        drain_all_pages(cc.zone);
        ret = __alloc_contig_migrate_range(&mut cc, start, end);
        if ret != 0 && ret != -(RUST_PA_EBUSY as i32) {
            break 'allocate ret;
        }
        ret = rust_pa_late_replace_free_hugepage_folios(start, end);
        if ret != 0 {
            break 'allocate ret;
        }
        let outer_start = find_large_buddy(start);
        if test_pages_isolated(outer_start, end, mode) != 0 {
            break 'allocate -(RUST_PA_EBUSY as i32);
        }
        let outer_end = isolate_freepages_range(&mut cc, outer_start, end);
        if outer_end == 0 {
            break 'allocate -(RUST_PA_EBUSY as i32);
        }
        if gfp & RUST_PA___GFP_COMP == 0 {
            split_free_frozen_pages(core::ptr::addr_of_mut!(cc.freepages).cast(), gfp);
            if start != outer_start {
                __free_contig_frozen_range(outer_start, start - outer_start);
            }
            if end != outer_end {
                __free_contig_frozen_range(end, outer_end - end);
            }
        } else if start == outer_start && end == outer_end && size.is_power_of_two() {
            let head = rust_pa_late_pfn_to_page(start);
            check_new_pages(head, order);
            prep_new_page(head, order, gfp, RUST_PA_ALLOC_DEFAULT);
        } else {
            ret = -(RUST_PA_EINVAL as i32);
            rust_pa_late_warn_contig_range(start, end, outer_start, outer_end);
        }
        ret
    };
    // Native code calls undo even when start_isolate_page_range reports failure.
    undo_isolate_page_range(start, end);
    ret
}
#[cfg(CONFIG_CONTIG_ALLOC)]
#[no_mangle]
pub unsafe extern "C" fn alloc_contig_range_noprof(
    start: ULong,
    end: ULong,
    flags: acr_flags_t,
    gfp: gfp_t,
) -> i32 {
    if rust_pa_late_warn_contig_comp(gfp & RUST_PA___GFP_COMP != 0) {
        return -(RUST_PA_EINVAL as i32);
    }
    let ret = alloc_contig_frozen_range_noprof(start, end, flags, gfp);
    if ret == 0 {
        rust_pa_late_set_pages_refcounted(rust_pa_late_pfn_to_page(start), end - start);
    }
    ret
}
#[cfg(CONFIG_CONTIG_ALLOC)]
unsafe fn pfn_range_valid_contig(
    z: *mut zone,
    mut start: ULong,
    count: ULong,
    skip_huge: bool,
    skipped: *mut bool,
) -> bool {
    let end = start.wrapping_add(count);
    while start < end {
        let mut step: ULong = 1;
        let mut p = rust_pa_late_pfn_to_online_page(start);
        if p.is_null() || rust_pa_late_page_zone(p) != z {
            return false;
        }
        if page_is_unmovable(z, p, PB_ISOLATE_MODE_OTHER, &mut step) {
            return false;
        }
        if rust_pa_late_page_huge(p) {
            if skip_huge {
                *skipped = true;
                return false;
            }
            p = rust_pa_late_compound_head(p);
            let order = rust_pa_late_compound_order(p);
            if order >= RUST_PA_MAX_FOLIO_ORDER || count <= (1i32 << order) as ULong {
                return false;
            }
        }
        start = start.wrapping_add(step);
    }
    true
}
#[cfg(CONFIG_CONTIG_ALLOC)]
unsafe fn zone_spans_last_pfn(z: *const zone, start: ULong, count: ULong) -> bool {
    rust_pa_late_zone_spans_pfn(z, start.wrapping_add(count).wrapping_sub(1))
}
#[cfg(CONFIG_CONTIG_ALLOC)]
#[no_mangle]
pub unsafe extern "C" fn alloc_contig_frozen_pages_noprof(
    count: ULong,
    gfp: gfp_t,
    nid: i32,
    mask: *mut nodemask_t,
) -> *mut page {
    let mut skip_huge = true;
    let mut skipped = false;
    loop {
        let list = rust_pa_late_node_zonelist(nid, gfp);
        let highest = rust_pa_late_gfp_zone(gfp);
        let mut refs = rust_pa_late_first_zones_zonelist(list, highest, mask);
        while !(*refs).zone.is_null() {
            let z = (*refs).zone;
            let mut flags = rust_pa_late_zone_lock_irqsave(z);
            // Preserve ALIGN's power-of-two mask arithmetic for all caller values.
            let mut pfn =
                (*z).zone_start_pfn.wrapping_add(count.wrapping_sub(1)) & !count.wrapping_sub(1);
            while zone_spans_last_pfn(z, pfn, count) {
                if pfn_range_valid_contig(z, pfn, count, skip_huge, &mut skipped) {
                    rust_pa_late_zone_unlock_irqrestore(z, flags);
                    let ret = alloc_contig_frozen_range_noprof(
                        pfn,
                        pfn.wrapping_add(count),
                        RUST_PA_ACR_FLAGS_NONE,
                        gfp,
                    );
                    if ret == 0 {
                        return rust_pa_late_pfn_to_page(pfn);
                    }
                    flags = rust_pa_late_zone_lock_irqsave(z);
                }
                pfn = pfn.wrapping_add(count);
            }
            rust_pa_late_zone_unlock_irqrestore(z, flags);
            refs = rust_pa_late_next_zones_zonelist(refs.add(1), highest, mask);
        }
        if skip_huge && skipped {
            skip_huge = false;
            continue;
        }
        return null_mut();
    }
}
#[cfg(CONFIG_CONTIG_ALLOC)]
#[no_mangle]
pub unsafe extern "C" fn alloc_contig_pages_noprof(
    count: ULong,
    gfp: gfp_t,
    nid: i32,
    mask: *mut nodemask_t,
) -> *mut page {
    if rust_pa_late_warn_contig_pages_comp(gfp & RUST_PA___GFP_COMP != 0) {
        return null_mut();
    }
    let p = alloc_contig_frozen_pages_noprof(count, gfp, nid, mask);
    if !p.is_null() {
        rust_pa_late_set_pages_refcounted(p, count);
    }
    p
}
#[cfg(CONFIG_CONTIG_ALLOC)]
#[no_mangle]
pub unsafe extern "C" fn free_contig_frozen_range(pfn: ULong, count: ULong) {
    let p = rust_pa_late_pfn_to_page(pfn);
    let order = rust_pa_late_ilog2(count);
    if rust_pa_late_warn_free_contig_not_head(p != rust_pa_late_compound_head(p)) {
        return;
    }
    if rust_pa_late_page_head(p) {
        rust_pa_late_warn_free_contig_order(order != rust_pa_late_compound_order(p));
        free_frozen_pages(p, order);
        return;
    }
    __free_contig_frozen_range(pfn, count);
}
#[cfg(CONFIG_CONTIG_ALLOC)]
#[no_mangle]
pub unsafe extern "C" fn free_contig_range(pfn: ULong, count: ULong) {
    if rust_pa_late_warn_free_contig_head(rust_pa_late_page_head(rust_pa_late_pfn_to_page(pfn))) {
        return;
    }
    __free_contig_range(pfn, count);
}

#[no_mangle]
pub unsafe extern "C" fn zone_pcp_disable(z: *mut zone) {
    rust_pa_late_pcp_batch_lock();
    __zone_set_pageset_high_and_batch(z, 0, 0, 1);
    __drain_all_pages(z, true);
}
#[no_mangle]
pub unsafe extern "C" fn zone_pcp_enable(z: *mut zone) {
    __zone_set_pageset_high_and_batch(
        z,
        (*z).pageset_high_min as ULong,
        (*z).pageset_high_max as ULong,
        (*z).pageset_batch as ULong,
    );
    rust_pa_late_pcp_batch_unlock();
}
#[no_mangle]
pub unsafe extern "C" fn zone_pcp_reset(z: *mut zone) {
    if (*z).per_cpu_pageset != rust_pa_late_boot_pageset() {
        for cpu in pa_online_cpus() {
            drain_zonestat(
                z,
                rust_pa_late_per_cpu_zonestats((*z).per_cpu_zonestats, cpu),
            );
        }
        free_percpu((*z).per_cpu_pageset.cast());
        (*z).per_cpu_pageset = rust_pa_late_boot_pageset();
        if (*z).per_cpu_zonestats != rust_pa_late_boot_zonestats() {
            free_percpu((*z).per_cpu_zonestats.cast());
            (*z).per_cpu_zonestats = rust_pa_late_boot_zonestats();
        }
    }
}
#[cfg(CONFIG_MEMORY_HOTREMOVE)]
#[no_mangle]
pub unsafe extern "C" fn __offline_isolated_pages(start: ULong, end: ULong) -> ULong {
    let mut offline: ULong = 0;
    let mut pfn = start;
    offline_mem_sections(pfn, end);
    let z = rust_pa_late_page_zone(rust_pa_late_pfn_to_page(pfn));
    let flags = rust_pa_late_zone_lock_irqsave(z);
    while pfn < end {
        let p = rust_pa_late_pfn_to_page(pfn);
        if !rust_pa_late_page_buddy(p) && rust_pa_late_page_hwpoison(p) {
            pfn += 1;
            continue;
        }
        if rust_pa_late_page_offline(p) {
            rust_pa_late_bug_offline_count(rust_pa_late_page_count(p) != 0);
            rust_pa_late_bug_offline_buddy(rust_pa_late_page_buddy(p));
            offline += 1;
            pfn += 1;
            continue;
        }
        rust_pa_late_bug_isolated_count(rust_pa_late_page_count(p) != 0);
        rust_pa_late_bug_isolated_buddy(!rust_pa_late_page_buddy(p));
        rust_pa_late_warn_isolated_migrate(
            rust_pa_late_get_pageblock_migratetype(p) != MIGRATE_ISOLATE as i32,
        );
        let order = rust_pa_late_buddy_order(p);
        del_page_from_free_list(p, z, order, MIGRATE_ISOLATE as i32);
        pfn = pfn.wrapping_add((1i32 << order) as ULong);
    }
    rust_pa_late_zone_unlock_irqrestore(z, flags);
    end.wrapping_sub(start).wrapping_sub(offline)
}
#[no_mangle]
pub unsafe extern "C" fn is_free_buddy_page(p: *const page) -> bool {
    let pfn = rust_pa_late_page_to_pfn(p);
    let mut order = 0;
    while order < RUST_PA_NR_PAGE_ORDERS {
        let head = p.sub((pfn & ((1i32 << order).wrapping_sub(1) as ULong)) as usize);
        if rust_pa_late_page_buddy(head) && rust_pa_late_buddy_order_unsafe(head) >= order as ULong
        {
            break;
        }
        order += 1;
    }
    order <= RUST_PA_MAX_PAGE_ORDER
}
#[cfg(CONFIG_MEMORY_FAILURE)]
unsafe fn add_to_free_list(p: *mut page, z: *mut zone, order: u32, migrate: i32, tail: bool) {
    __add_to_free_list(p, z, order, migrate, tail);
    account_freepages(z, 1i32 << order, migrate);
}
#[cfg(CONFIG_MEMORY_FAILURE)]
unsafe fn break_down_buddy_pages(
    z: *mut zone,
    mut p: *mut page,
    target: *mut page,
    low: i32,
    mut high: i32,
    migrate: i32,
) {
    let mut size = (1i32 << high) as ULong;
    while high > low {
        high -= 1;
        size >>= 1;
        let buddy;
        if target >= p.add(size as usize) {
            buddy = p;
            p = p.add(size as usize);
        } else {
            buddy = p.add(size as usize);
        }
        if rust_pa_late_set_page_guard(z, buddy, high as u32) {
            continue;
        }
        add_to_free_list(buddy, z, high as u32, migrate, false);
        set_buddy_order(buddy, high as u32);
    }
}
#[cfg(CONFIG_MEMORY_FAILURE)]
#[no_mangle]
pub unsafe extern "C" fn take_page_off_buddy(p: *mut page) -> bool {
    let z = rust_pa_late_page_zone(p);
    let pfn = rust_pa_late_page_to_pfn(p);
    let flags = rust_pa_late_zone_lock_irqsave(z);
    let mut taken = false;
    for order in 0..RUST_PA_NR_PAGE_ORDERS {
        let head = p.sub((pfn & ((1i32 << order).wrapping_sub(1) as ULong)) as usize);
        let page_order = rust_pa_late_buddy_order(head) as i32;
        if rust_pa_late_page_buddy(head) && page_order as u32 >= order {
            let head_pfn = rust_pa_late_page_to_pfn(head);
            let migrate = rust_pa_late_get_pfnblock_migratetype(head, head_pfn);
            del_page_from_free_list(head, z, page_order as u32, migrate);
            break_down_buddy_pages(z, head, p, 0, page_order, migrate);
            rust_pa_late_set_hwpoison_takenoff(p);
            taken = true;
            break;
        }
        if rust_pa_late_page_count(head) > 0 {
            break;
        }
    }
    rust_pa_late_zone_unlock_irqrestore(z, flags);
    taken
}
#[cfg(CONFIG_MEMORY_FAILURE)]
#[no_mangle]
pub unsafe extern "C" fn put_page_back_buddy(p: *mut page) -> bool {
    let z = rust_pa_late_page_zone(p);
    let flags = rust_pa_late_zone_lock_irqsave(z);
    let mut cleared = false;
    if rust_pa_late_put_page_testzero(p) {
        let pfn = rust_pa_late_page_to_pfn(p);
        let migrate = rust_pa_late_get_pfnblock_migratetype(p, pfn);
        rust_pa_late_clear_hwpoison_takenoff(p);
        __free_one_page(p, pfn, z, 0, migrate, FPI_NONE);
        cleared = rust_pa_late_test_clear_hwpoison(p);
    }
    rust_pa_late_zone_unlock_irqrestore(z, flags);
    cleared
}
// Enum-argument public entry retains its C enum/KCFI identity in the ABI thunk.
#[no_mangle]
pub unsafe extern "C" fn rust_pa_has_managed_zone(index: u32) -> bool {
    for p in pa_nodes() {
        if rust_pa_late_managed_zone(core::ptr::addr_of_mut!((*p).node_zones[index as usize])) {
            return true;
        }
    }
    false
}

#[cfg(CONFIG_UNACCEPTED_MEMORY)]
static mut lazy_accept: bool = true;
#[cfg(CONFIG_UNACCEPTED_MEMORY)]
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
unsafe extern "C" fn accept_memory_parse(p: *mut CChar) -> i32 {
    if rust_pa_late_strcmp(p, c"lazy".as_ptr()) == 0 {
        lazy_accept = true;
        0
    } else if rust_pa_late_strcmp(p, c"eager".as_ptr()) == 0 {
        lazy_accept = false;
        0
    } else {
        -(RUST_PA_EINVAL as i32)
    }
}
#[cfg(CONFIG_UNACCEPTED_MEMORY)]
unsafe fn page_contains_unaccepted(p: *mut page, order: u32) -> bool {
    range_contains_unaccepted_memory(
        rust_pa_late_page_to_phys(p),
        (RUST_PA_PAGE_SIZE as ULong) << order,
    )
}
#[cfg(CONFIG_UNACCEPTED_MEMORY)]
unsafe fn __accept_page(z: *mut zone, flags: *mut ULong, p: *mut page) {
    rust_pa_late_list_del(rust_pa_late_page_lru(p));
    account_freepages(
        z,
        -(RUST_PA_MAX_ORDER_NR_PAGES as i32),
        MIGRATE_MOVABLE as i32,
    );
    rust_pa_late_mod_zone_page_state(z, NR_UNACCEPTED, -(RUST_PA_MAX_ORDER_NR_PAGES as Long));
    rust_pa_late_clear_page_unaccepted(p);
    rust_pa_late_zone_unlock_irqrestore(z, *flags);
    accept_memory(
        rust_pa_late_page_to_phys(p),
        (RUST_PA_PAGE_SIZE as ULong) << RUST_PA_MAX_PAGE_ORDER,
    );
    __free_pages_ok(p, RUST_PA_MAX_PAGE_ORDER, FPI_TO_TAIL);
}
#[cfg(CONFIG_UNACCEPTED_MEMORY)]
#[no_mangle]
pub unsafe extern "C" fn accept_page(p: *mut page) {
    let z = rust_pa_late_page_zone(p);
    let mut flags = rust_pa_late_zone_lock_irqsave(z);
    if !rust_pa_late_page_unaccepted(p) {
        rust_pa_late_zone_unlock_irqrestore(z, flags);
        return;
    }
    __accept_page(z, &mut flags, p);
}
#[cfg(CONFIG_UNACCEPTED_MEMORY)]
unsafe fn try_to_accept_memory_one(z: *mut zone) -> bool {
    let mut flags = rust_pa_late_zone_lock_irqsave(z);
    let list = core::ptr::addr_of_mut!((*z).unaccepted_pages);
    if (*list).next == list {
        rust_pa_late_zone_unlock_irqrestore(z, flags);
        return false;
    }
    __accept_page(z, &mut flags, rust_pa_late_lru_page((*list).next));
    true
}
#[cfg(CONFIG_UNACCEPTED_MEMORY)]
unsafe fn cond_accept_memory(z: *mut zone, order: u32, flags: i32) -> bool {
    let list = core::ptr::addr_of_mut!((*z).unaccepted_pages);
    if (*list).next == list || flags & RUST_PA_ALLOC_NOLOCK as i32 != 0 {
        return false;
    }
    let mark = rust_pa_late_promo_wmark_pages(z) as Long;
    if mark == 0 {
        return try_to_accept_memory_one(z);
    }
    let mut to_accept = (mark as ULong).wrapping_sub(
        rust_pa_late_zone_page_state(z, NR_FREE_PAGES)
            .wrapping_sub(__zone_watermark_unusable_free(z, order, 0) as ULong)
            .wrapping_sub(rust_pa_late_zone_page_state(z, NR_UNACCEPTED)),
    ) as Long;
    let mut accepted = false;
    while to_accept > 0 {
        if !try_to_accept_memory_one(z) {
            break;
        }
        accepted = true;
        to_accept -= RUST_PA_MAX_ORDER_NR_PAGES as Long;
    }
    accepted
}
#[cfg(CONFIG_UNACCEPTED_MEMORY)]
unsafe fn __free_unaccepted(p: *mut page) -> bool {
    if !lazy_accept {
        return false;
    }
    let z = rust_pa_late_page_zone(p);
    let flags = rust_pa_late_zone_lock_irqsave(z);
    rust_pa_late_list_add_tail(
        rust_pa_late_page_lru(p),
        core::ptr::addr_of_mut!((*z).unaccepted_pages),
    );
    account_freepages(z, RUST_PA_MAX_ORDER_NR_PAGES as i32, MIGRATE_MOVABLE as i32);
    rust_pa_late_mod_zone_page_state(z, NR_UNACCEPTED, RUST_PA_MAX_ORDER_NR_PAGES as Long);
    rust_pa_late_set_page_unaccepted(p);
    rust_pa_late_zone_unlock_irqrestore(z, flags);
    true
}
#[cfg(not(CONFIG_UNACCEPTED_MEMORY))]
unsafe fn page_contains_unaccepted(_p: *mut page, _order: u32) -> bool {
    false
}
#[cfg(not(CONFIG_UNACCEPTED_MEMORY))]
unsafe fn cond_accept_memory(_z: *mut zone, _order: u32, _flags: i32) -> bool {
    false
}
#[cfg(not(CONFIG_UNACCEPTED_MEMORY))]
#[inline(always)]
unsafe fn __free_unaccepted(_p: *mut page) -> bool {
    kernel::build_error!("__free_unaccepted requires CONFIG_UNACCEPTED_MEMORY");
}
#[no_mangle]
pub unsafe extern "C" fn alloc_frozen_pages_nolock_noprof(
    gfp: gfp_t,
    mut nid: i32,
    order: u32,
) -> *mut page {
    if nid == RUST_PA_NUMA_NO_NODE as i32 {
        nid = rust_pa_late_numa_node_id();
    }
    __alloc_frozen_pages_noprof(gfp, order, nid, null_mut(), RUST_PA_ALLOC_NOLOCK)
}
#[no_mangle]
pub unsafe extern "C" fn alloc_pages_nolock_noprof(gfp: gfp_t, nid: i32, order: u32) -> *mut page {
    let p = alloc_frozen_pages_nolock_noprof(gfp, nid, order);
    if !p.is_null() {
        rust_pa_late_set_page_refcounted(p);
    }
    p
}
