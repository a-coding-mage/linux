// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2017 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>

// Translated from matrix.c. Kernel headers and helper symbols are supplied by
// the surrounding kernel translation unit.

#[repr(C)]
pub struct cpumap {
    pub available: u32,
    pub allocated: u32,
    pub managed: u32,
    pub managed_allocated: u32,
    pub initialized: bool,
    pub online: bool,
    pub managed_map: *mut c_ulong,
    pub alloc_map: *mut c_ulong,
}

#[repr(C)]
pub struct irq_matrix {
    pub matrix_bits: u32,
    pub alloc_start: u32,
    pub alloc_end: u32,
    pub alloc_size: u32,
    pub global_available: u32,
    pub global_reserved: u32,
    pub systembits_inalloc: u32,
    pub total_allocated: u32,
    pub online_maps: u32,
    pub maps: *mut cpumap,
    pub system_map: *mut c_ulong,
    pub scratch_map: *mut c_ulong,
}

pub type c_ulong = usize;
pub type c_int = i32;

// CREATE_TRACE_POINTS; trace/events/irq_matrix.h

extern "C" {
    fn kzalloc_flex(size: usize) -> *mut irq_matrix;
    fn kfree(ptr: *mut irq_matrix);
    fn __alloc_percpu(size: usize, align: usize) -> *mut cpumap;
    fn per_cpu_ptr<T>(ptr: *mut T, cpu: u32) -> *mut T;
    fn this_cpu_ptr<T>(ptr: *mut T) -> *mut T;
    fn smp_processor_id() -> u32;
    fn bitmap_or(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, bits: u32);
    fn bitmap_andnot(dst: *mut c_ulong, a: *const c_ulong, b: *const c_ulong, bits: u32);
    fn bitmap_find_next_zero_area(map: *const c_ulong, size: u32, start: u32, nr: u32, align: u32) -> u32;
    fn bitmap_weight(map: *const c_ulong, bits: u32) -> u32;
    fn find_first_bit(map: *const c_ulong, bits: u32) -> u32;
    fn set_bit(bit: u32, map: *mut c_ulong);
    fn clear_bit(bit: u32, map: *mut c_ulong);
    fn test_and_set_bit(bit: u32, map: *mut c_ulong) -> bool;
    fn test_and_clear_bit(bit: u32, map: *mut c_ulong) -> bool;
    fn cpumask_empty(mask: *const cpumask) -> bool;
    fn cpumask_of(cpu: u32) -> *const cpumask;
    fn for_each_possible_cpu_next(cpu: *mut u32) -> bool;
    fn for_each_cpu_next(cpu: *mut u32, mask: *const cpumask) -> bool;
    fn for_each_online_cpu_next(cpu: *mut i32) -> bool;
    fn trace_irq_matrix_online(m: *mut irq_matrix);
    fn trace_irq_matrix_offline(m: *mut irq_matrix);
    fn trace_irq_matrix_assign_system(bit: u32, m: *mut irq_matrix);
    fn trace_irq_matrix_reserve_managed(bit: u32, cpu: u32, m: *mut irq_matrix, cm: *mut cpumap);
    fn trace_irq_matrix_remove_managed(bit: u32, cpu: u32, m: *mut irq_matrix, cm: *mut cpumap);
    fn trace_irq_matrix_alloc_managed(bit: u32, cpu: u32, m: *mut irq_matrix, cm: *mut cpumap);
    fn trace_irq_matrix_assign(bit: u32, cpu: u32, m: *mut irq_matrix, cm: *mut cpumap);
    fn trace_irq_matrix_reserve(m: *mut irq_matrix);
    fn trace_irq_matrix_remove_reserved(m: *mut irq_matrix);
    fn trace_irq_matrix_alloc(bit: u32, cpu: u32, m: *mut irq_matrix, cm: *mut cpumap);
    fn trace_irq_matrix_free(bit: u32, cpu: u32, m: *mut irq_matrix, cm: *mut cpumap);
    fn pr_warn(msg: *const u8);
    fn seq_printf(sf: *mut seq_file, fmt: *const u8, ...);
    fn cpus_read_lock();
    fn cpus_read_unlock();
}

#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }

#[inline]
unsafe fn matrix_alloc_area(m: *mut irq_matrix, cm: *mut cpumap, num: u32, managed: bool) -> u32 {
    let start = (*m).alloc_start;
    let end = (*m).alloc_end;
    bitmap_or((*m).scratch_map, (*cm).managed_map, (*m).system_map, end);
    bitmap_or((*m).scratch_map, (*m).scratch_map, (*cm).alloc_map, end);
    let area = bitmap_find_next_zero_area((*m).scratch_map, end, start, num, 0);
    if area >= end { return area; }
    if managed { set_bit(area, (*cm).managed_map); } else { set_bit(area, (*cm).alloc_map); }
    area
}

#[inline]
unsafe fn matrix_find_best_cpu(m: *mut irq_matrix, msk: *const cpumask) -> u32 {
    let mut cpu = 0; let mut best_cpu = u32::MAX; let mut maxavl = 0;
    while for_each_cpu_next(&mut cpu, msk) {
        let cm = per_cpu_ptr((*m).maps, cpu);
        if !(*cm).online || (*cm).available <= maxavl { continue; }
        best_cpu = cpu; maxavl = (*cm).available;
    } best_cpu
}

#[inline]
unsafe fn matrix_find_best_cpu_managed(m: *mut irq_matrix, msk: *const cpumask) -> u32 {
    let mut cpu = 0; let mut best_cpu = u32::MAX; let mut allocated = u32::MAX;
    while for_each_cpu_next(&mut cpu, msk) {
        let cm = per_cpu_ptr((*m).maps, cpu);
        if !(*cm).online || (*cm).managed_allocated > allocated { continue; }
        best_cpu = cpu; allocated = (*cm).managed_allocated;
    } best_cpu
}

#[inline]
pub unsafe fn irq_matrix_online(m: *mut irq_matrix) {
    let cm = this_cpu_ptr((*m).maps);
    if (*cm).online { panic!("BUG_ON"); }
    if !(*cm).initialized {
        (*cm).available = (*m).alloc_size;
        (*cm).available -= (*cm).managed + (*m).systembits_inalloc;
        (*cm).initialized = true;
    }
    (*m).global_available += (*cm).available; (*cm).online = true; (*m).online_maps += 1;
    trace_irq_matrix_online(m);
}

pub unsafe fn irq_alloc_matrix(matrix_bits: u32, alloc_start: u32, alloc_end: u32) -> *mut irq_matrix {
    let matrix_size = ((matrix_bits as usize) + (usize::BITS as usize - 1)) / usize::BITS as usize;
    let m = kzalloc_flex(core::mem::size_of::<irq_matrix>() + matrix_size * 2 * core::mem::size_of::<c_ulong>());
    if m.is_null() { return core::ptr::null_mut(); }
    (*m).system_map = (*m).scratch_map.add(matrix_size);
    (*m).matrix_bits = matrix_bits; (*m).alloc_start = alloc_start; (*m).alloc_end = alloc_end; (*m).alloc_size = alloc_end - alloc_start;
    (*m).maps = __alloc_percpu(core::mem::size_of::<cpumap>() + matrix_size * 2 * core::mem::size_of::<c_ulong>(), core::mem::align_of::<cpumap>());
    if (*m).maps.is_null() { kfree(m); return core::ptr::null_mut(); }
    let mut cpu = 0; while for_each_possible_cpu_next(&mut cpu) { let cm = per_cpu_ptr((*m).maps, cpu); (*cm).managed_map = (*cm).alloc_map.add(matrix_size); }
    m
}

#[inline]
pub unsafe fn irq_matrix_offline(m: *mut irq_matrix) {
    let cm = this_cpu_ptr((*m).maps);
    (*m).global_available -= (*cm).available; (*cm).online = false; (*m).online_maps -= 1;
    trace_irq_matrix_offline(m);
}

pub unsafe fn irq_matrix_assign_system(m: *mut irq_matrix, bit: u32, replace: bool) {
    let cm = this_cpu_ptr((*m).maps);
    if bit > (*m).matrix_bits || (*m).online_maps > 1 || ((*m).online_maps != 0 && !replace) { panic!("BUG_ON"); }
    set_bit(bit, (*m).system_map);
    if replace {
        if !test_and_clear_bit(bit, (*cm).alloc_map) { panic!("BUG_ON"); }
        (*cm).allocated -= 1; (*m).total_allocated -= 1;
    }
    if bit >= (*m).alloc_start && bit < (*m).alloc_end { (*m).systembits_inalloc += 1; }
    trace_irq_matrix_assign_system(bit, m);
}

pub unsafe fn irq_matrix_reserve_managed(m: *mut irq_matrix, msk: *const cpumask) -> c_int {
    let mut cpu = 0; let mut failed_cpu;
    while for_each_cpu_next(&mut cpu, msk) {
        let cm = per_cpu_ptr((*m).maps, cpu); let bit = matrix_alloc_area(m, cm, 1, true);
        if bit >= (*m).alloc_end { failed_cpu = cpu; let mut rollback = 0; while for_each_cpu_next(&mut rollback, msk) { if rollback == failed_cpu { break; } irq_matrix_remove_managed(m, cpumask_of(rollback)); } return -28; }
        (*cm).managed += 1;
        if (*cm).online { (*cm).available -= 1; (*m).global_available -= 1; }
        trace_irq_matrix_reserve_managed(bit, cpu, m, cm);
    } 0
}

pub unsafe fn irq_matrix_remove_managed(m: *mut irq_matrix, msk: *const cpumask) {
    let mut cpu = 0; while for_each_cpu_next(&mut cpu, msk) {
        let cm = per_cpu_ptr((*m).maps, cpu); let end = (*m).alloc_end;
        if (*cm).managed == 0 { continue; }
        bitmap_andnot((*m).scratch_map, (*cm).managed_map, (*cm).alloc_map, end);
        let bit = find_first_bit((*m).scratch_map, end); if bit >= end { continue; }
        clear_bit(bit, (*cm).managed_map); (*cm).managed -= 1;
        if (*cm).online { (*cm).available += 1; (*m).global_available += 1; }
        trace_irq_matrix_remove_managed(bit, cpu, m, cm);
    }
}

pub unsafe fn irq_matrix_alloc_managed(m: *mut irq_matrix, msk: *const cpumask, mapped_cpu: *mut u32) -> c_int {
    if cpumask_empty(msk) { return -22; }
    let cpu = matrix_find_best_cpu_managed(m, msk); if cpu == u32::MAX { return -28; }
    let cm = per_cpu_ptr((*m).maps, cpu); bitmap_andnot((*m).scratch_map, (*cm).managed_map, (*cm).alloc_map, (*m).alloc_end);
    let bit = find_first_bit((*m).scratch_map, (*m).alloc_end); if bit >= (*m).alloc_end { return -28; }
    set_bit(bit, (*cm).alloc_map); (*cm).allocated += 1; (*cm).managed_allocated += 1; (*m).total_allocated += 1; *mapped_cpu = cpu;
    trace_irq_matrix_alloc_managed(bit, cpu, m, cm); bit as c_int
}

pub unsafe fn irq_matrix_assign(m: *mut irq_matrix, bit: u32) {
    let cm = this_cpu_ptr((*m).maps); if bit < (*m).alloc_start || bit >= (*m).alloc_end || test_and_set_bit(bit, (*cm).alloc_map) { return; }
    (*cm).allocated += 1; (*m).total_allocated += 1; (*cm).available -= 1; (*m).global_available -= 1;
    trace_irq_matrix_assign(bit, smp_processor_id(), m, cm);
}

pub unsafe fn irq_matrix_reserve(m: *mut irq_matrix) { if (*m).global_reserved == (*m).global_available { pr_warn(b"Interrupt reservation exceeds available resources\0".as_ptr()); } (*m).global_reserved += 1; trace_irq_matrix_reserve(m); }
pub unsafe fn irq_matrix_remove_reserved(m: *mut irq_matrix) { (*m).global_reserved -= 1; trace_irq_matrix_remove_reserved(m); }

pub unsafe fn irq_matrix_alloc(m: *mut irq_matrix, msk: *const cpumask, reserved: bool, mapped_cpu: *mut u32) -> c_int {
    if cpumask_empty(msk) { return -22; } let cpu = matrix_find_best_cpu(m, msk); if cpu == u32::MAX { return -28; }
    let cm = per_cpu_ptr((*m).maps, cpu); let bit = matrix_alloc_area(m, cm, 1, false); if bit >= (*m).alloc_end { return -28; }
    (*cm).allocated += 1; (*cm).available -= 1; (*m).total_allocated += 1; (*m).global_available -= 1; if reserved { (*m).global_reserved -= 1; } *mapped_cpu = cpu; trace_irq_matrix_alloc(bit, cpu, m, cm); bit as c_int
}

pub unsafe fn irq_matrix_free(m: *mut irq_matrix, cpu: u32, bit: u32, managed: bool) {
    let cm = per_cpu_ptr((*m).maps, cpu); if bit < (*m).alloc_start || bit >= (*m).alloc_end || !test_and_clear_bit(bit, (*cm).alloc_map) { return; }
    (*cm).allocated -= 1; if managed { (*cm).managed_allocated -= 1; } if (*cm).online { (*m).total_allocated -= 1; }
    if !managed { (*cm).available += 1; if (*cm).online { (*m).global_available += 1; } } trace_irq_matrix_free(bit, cpu, m, cm);
}

pub unsafe fn irq_matrix_available(m: *mut irq_matrix, cpudown: bool) -> u32 { let cm = this_cpu_ptr((*m).maps); if !cpudown { (*m).global_available } else { (*m).global_available - (*cm).available } }
pub unsafe fn irq_matrix_reserved(m: *mut irq_matrix) -> u32 { (*m).global_reserved }
pub unsafe fn irq_matrix_allocated(m: *mut irq_matrix) -> u32 { let cm = this_cpu_ptr((*m).maps); (*cm).allocated - (*cm).managed_allocated }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
