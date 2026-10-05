// SPDX-License-Identifier: GPL-2.0-only
//! Rust source owner of mm/vmstat.c; all native types and values come from headers.
// Historical initial transcription: d482bb509b7d065808de40ce78b5bca39f40b783
// SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
// RECONCILED-SOURCE-COMMIT: e1d84f501551943a11f4c5271e9f5c85d7e15168
// RECONCILED-C-SHA256: 637aac81a55a905e93b1feae46ebdd31b97bc1727cbe56a621fe5cc3e82379e6
use b::*;
#[cfg(CONFIG_SMP)]
use core::cmp::{max, min};
#[cfg(any(CONFIG_SMP, CONFIG_PROC_FS))]
use core::mem::size_of;
use core::mem::zeroed;
#[cfg(any(
    CONFIG_SMP,
    CONFIG_NUMA,
    CONFIG_PROC_FS,
    all(CONFIG_DEBUG_FS, CONFIG_COMPACTION)
))]
use core::ptr::addr_of;
use core::ptr::{addr_of_mut, null_mut};
use kernel::bindings::vmstat_native as b;
use kernel::ffi::*;

// vm_* arrays retain native aligned-array storage in the catalogued leaf.
static mut NR_MEMMAP_BOOT_PAGES_STORAGE: atomic_long_t = unsafe { zeroed() };
static mut NR_MEMMAP_PAGES_STORAGE: atomic_long_t = unsafe { zeroed() };
#[export_name = "mm_percpu_wq"]
static mut MM_PERCPU_WQ: *mut workqueue_struct = null_mut();

#[cfg(CONFIG_SMP)]
#[inline]
unsafe fn zone_add(delta: c_long, z: *mut zone, item: usize) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        rust_vmstat_atomic_add(delta, addr_of_mut!((*z).vm_stat[item]));
        rust_vmstat_atomic_add(delta, addr_of_mut!(vm_zone_stat[item]));
    }
}
#[cfg(CONFIG_SMP)]
#[inline]
unsafe fn node_add(delta: c_long, p: *mut pglist_data, item: usize) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        rust_vmstat_atomic_add(delta, addr_of_mut!((*p).vm_stat[item]));
        rust_vmstat_atomic_add(delta, addr_of_mut!(vm_node_stat[item]));
    }
}
#[cfg(CONFIG_NUMA)]
#[inline]
unsafe fn numa_add(delta: c_long, z: *mut zone, item: usize) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        rust_vmstat_atomic_add(delta, addr_of_mut!((*z).vm_numa_event[item]));
        rust_vmstat_atomic_add(delta, addr_of_mut!(vm_numa_event[item]));
    }
}
#[cfg(any(CONFIG_NUMA, CONFIG_PROC_FS))]
#[inline]
unsafe fn clamped_read(p: *const atomic_long_t) -> c_ulong {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let x = rust_vmstat_atomic_read(p);
        #[cfg(CONFIG_SMP)]
        let x = max(x, 0);
        x as c_ulong
    }
}
#[cfg(any(CONFIG_NUMA, CONFIG_PROC_FS))]
#[inline]
unsafe fn zone_state(z: *mut zone, item: usize) -> c_ulong {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe { clamped_read(addr_of!((*z).vm_stat[item])) }
}
#[cfg(any(CONFIG_NUMA, CONFIG_PROC_FS))]
#[inline]
unsafe fn node_state_pages(p: *mut pglist_data, item: usize) -> c_ulong {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe { clamped_read(addr_of!((*p).vm_stat[item])) }
}
#[cfg(any(
    CONFIG_SMP,
    CONFIG_NUMA,
    CONFIG_PROC_FS,
    all(CONFIG_DEBUG_FS, CONFIG_COMPACTION)
))]
#[inline]
unsafe fn populated(z: *mut zone) -> bool {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe { (*z).present_pages != 0 }
}
#[cfg(any(CONFIG_SMP, CONFIG_NUMA))]
macro_rules! zones {
    ($z:ident, $body:block) => {{
        let mut $z = rust_vmstat_first_zone();
        while !$z.is_null() {
            if populated($z) {
                $body
            }
            $z = next_zone($z);
        }
    }};
}
#[cfg(CONFIG_SMP)]
macro_rules! nodes {
    ($p:ident, $body:block) => {{
        let mut $p = first_online_pgdat();
        while !$p.is_null() {
            $body;
            $p = next_online_pgdat($p);
        }
    }};
}
#[cfg(any(CONFIG_SMP, CONFIG_NUMA, CONFIG_VM_EVENT_COUNTERS, CONFIG_PROC_FS))]
macro_rules! online_cpus {
    ($cpu:ident, $body:block) => {{
        let mut $cpu = rust_vmstat_next_online_cpu(-1);
        while $cpu < rust_vmstat_nr_cpu_ids() {
            $body;
            $cpu = rust_vmstat_next_online_cpu($cpu);
        }
    }};
}

#[cfg(CONFIG_VM_EVENT_COUNTERS)]
unsafe fn sum_vm_events(ret: *mut c_ulong) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        core::ptr::write_bytes(ret, 0, NR_VM_EVENT_ITEMS as usize);
        online_cpus!(cpu, {
            let state = rust_vmstat_event_cpu(cpu);
            for i in 0..NR_VM_EVENT_ITEMS as usize {
                *ret.add(i) = (*ret.add(i)).wrapping_add((*state).event[i]);
            }
        });
    }
}
#[cfg(CONFIG_VM_EVENT_COUNTERS)]
#[no_mangle]
unsafe extern "C" fn all_vm_events(ret: *mut c_ulong) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        rust_vmstat_cpus_read_lock();
        sum_vm_events(ret);
        rust_vmstat_cpus_read_unlock();
    }
}
#[cfg(CONFIG_VM_EVENT_COUNTERS)]
#[no_mangle]
unsafe extern "C" fn vm_events_fold_cpu(cpu: c_int) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let state = rust_vmstat_event_cpu(cpu);
        for i in 0..NR_VM_EVENT_ITEMS as usize {
            rust_vmstat_count_events(i as _, (*state).event[i] as c_long);
            (*state).event[i] = 0;
        }
    }
}
#[cfg(CONFIG_NUMA)]
unsafe fn fold_vm_zone_numa_events(z: *mut zone) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let mut events = [0 as c_ulong; NR_VM_NUMA_EVENT_ITEMS as usize];
        online_cpus!(cpu, {
            let stats = rust_vmstat_zone_cpu((*z).per_cpu_zonestats, cpu);
            for i in 0..NR_VM_NUMA_EVENT_ITEMS as usize {
                events[i] = events[i].wrapping_add(rust_vmstat_xchg_ulong(
                    addr_of_mut!((*stats).vm_numa_event[i]),
                    0,
                ));
            }
        });
        for i in 0..NR_VM_NUMA_EVENT_ITEMS as usize {
            numa_add(events[i] as c_long, z, i);
        }
    }
}
#[cfg(CONFIG_NUMA)]
#[no_mangle]
unsafe extern "C" fn fold_vm_numa_events() {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        zones!(z, {
            fold_vm_zone_numa_events(z);
        });
    }
}
#[no_mangle]
unsafe extern "C" fn memmap_boot_pages_add(delta: c_long) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        rust_vmstat_atomic_add(delta, addr_of_mut!(NR_MEMMAP_BOOT_PAGES_STORAGE));
    }
}
#[no_mangle]
unsafe extern "C" fn memmap_pages_add(delta: c_long) {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        rust_vmstat_atomic_add(delta, addr_of_mut!(NR_MEMMAP_PAGES_STORAGE));
    }
}
#[cfg(CONFIG_NUMA)]
#[no_mangle]
unsafe extern "C" fn sum_zone_node_page_state(node: c_int, item: zone_stat_item) -> c_ulong {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let p = rust_vmstat_node_data(node);
        let mut count: c_ulong = 0;
        for i in 0..MAX_NR_ZONES as usize {
            count = count.wrapping_add(zone_state(addr_of_mut!((*p).node_zones[i]), item as usize));
        }
        count
    }
}
#[cfg(CONFIG_NUMA)]
#[no_mangle]
unsafe extern "C" fn sum_zone_numa_event_state(node: c_int, item: numa_stat_item) -> c_ulong {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        let p = rust_vmstat_node_data(node);
        let mut count: c_ulong = 0;
        for i in 0..MAX_NR_ZONES as usize {
            count = count.wrapping_add(rust_vmstat_atomic_read(addr_of!(
                (*p).node_zones[i].vm_numa_event[item as usize]
            )) as c_ulong);
        }
        count
    }
}
#[cfg(CONFIG_NUMA)]
#[no_mangle]
unsafe extern "C" fn node_page_state_pages(p: *mut pglist_data, item: node_stat_item) -> c_ulong {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe { node_state_pages(p, item as usize) }
}
#[cfg(CONFIG_NUMA)]
#[no_mangle]
unsafe extern "C" fn node_page_state(p: *mut pglist_data, item: node_stat_item) -> c_ulong {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        rust_vmstat_warn_node_read_bytes(rust_vmstat_item_in_bytes(item as c_int));
        node_state_pages(p, item as usize)
    }
}
#[cfg(CONFIG_NUMA)]
#[no_mangle]
unsafe extern "C" fn node_page_state_monotonic(
    p: *mut pglist_data,
    item: node_stat_item,
) -> c_ulong {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe { rust_vmstat_atomic_read(addr_of!((*p).vm_stat[item as usize])) as c_ulong }
}

#[cfg(CONFIG_SMP)]
include!("vmstat_accounting.rs");
#[cfg(CONFIG_SMP)]
include!("vmstat_work.rs");
#[cfg(CONFIG_COMPACTION)]
include!("vmstat_fragmentation.rs");
#[cfg(any(CONFIG_PROC_FS, CONFIG_SYSFS, CONFIG_NUMA, CONFIG_MEMCG))]
include!("vmstat_text.rs");
#[cfg(any(CONFIG_PROC_FS, all(CONFIG_DEBUG_FS, CONFIG_COMPACTION)))]
include!("vmstat_seq.rs");
#[cfg(CONFIG_PROC_FS)]
include!("vmstat_proc.rs");
#[cfg(all(CONFIG_DEBUG_FS, CONFIG_COMPACTION))]
include!("vmstat_debug.rs");
include!("vmstat_init.rs");
