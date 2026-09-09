/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Topological CPU IDs (cids)
 * --------------------------
 *
 * Raw cpu numbers are clumsy for sharding work and communication across
 * topology units, especially from BPF: the space can be sparse, numerical
 * closeness doesn't imply topological closeness (x86 hyperthreading often puts
 * SMT siblings far apart), and a range of cpu ids doesn't mean anything.
 *
 * cids give every cpu a dense, topology-ordered id. CPUs sharing a core, LLC or
 * NUMA node get contiguous cid ranges, so a topology unit becomes a (start,
 * length) slice of cid space.
 *
 * The mapping is built once at root scheduler enable time by walking the
 * topology of online cpus only.
 *
 * Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2026 Tejun Heo <tj@kernel.org>
 */

#[repr(C)]
pub struct scx_cid_tables {
    pub nr_shards: u32,
    pub cid_to_cpu: *mut i16,
    pub cpu_to_cid: *mut i16,
    pub cid_to_shard: *mut i32,
    pub shard_node: *mut i32,
    pub shard_ranges: *mut scx_cid_shard,
    pub topo: *mut scx_cid_topo,
    pub rcu: rcu_head,
}

extern "C" {
    pub static mut scx_nr_cid_shards: u32;
    pub static mut scx_cid_to_cpu_tbl: *mut i16;
    pub static mut scx_cpu_to_cid_tbl: *mut i16;
    pub static mut scx_cid_to_shard: *mut i32;
    pub static mut scx_shard_node: *mut i32;
    pub static mut scx_cid_shard_ranges: *mut scx_cid_shard;
    pub static mut scx_cid_topo: *mut scx_cid_topo;
    pub static mut scx_kfunc_ids_init_cids: btf_id_set8;
    pub static mut scx_kfunc_ids_cid: btf_id_set8;

    pub fn scx_cmask_clear(m: *mut scx_cmask);
    pub fn scx_cmask_fill(m: *mut scx_cmask);
    pub fn scx_cmask_and(dst: *mut scx_cmask, src: *const scx_cmask);
    pub fn scx_cmask_or(dst: *mut scx_cmask, src: *const scx_cmask);
    pub fn scx_cmask_copy(dst: *mut scx_cmask, src: *const scx_cmask);
    pub fn scx_cmask_andnot(dst: *mut scx_cmask, src: *const scx_cmask);
    pub fn scx_cmask_subset(sub: *const scx_cmask, sup: *const scx_cmask) -> bool;
    pub fn scx_cmask_intersects(a: *const scx_cmask, b: *const scx_cmask) -> bool;
    pub fn scx_cmask_empty(m: *const scx_cmask) -> bool;
    pub fn scx_cid_init(sch: *mut scx_sched) -> i32;
    pub fn scx_cid_publish_tables();
    pub fn scx_cid_retire_tables();
    pub fn scx_cid_kfunc_init() -> i32;
}

#[inline]
pub unsafe fn cid_valid(sch: *mut scx_sched, cid: i32) -> bool {
    if cid >= 0 && cid < num_possible_cpus() {
        return true;
    }
    scx_error(sch, "invalid cid %d", cid);
    false
}

#[inline]
pub unsafe fn __scx_cid_to_cpu(cid: i32) -> i32 {
    *scx_cid_to_cpu_tbl.add(cid as usize) as i32
}

#[inline]
pub unsafe fn __scx_cpu_to_cid(cpu: i32) -> i32 {
    *scx_cpu_to_cid_tbl.add(cpu as usize) as i32
}

#[inline]
pub unsafe fn scx_cid_to_cpu(sch: *mut scx_sched, cid: i32) -> i32 {
    let tbl = scx_cid_to_cpu_tbl;
    if !cid_valid(sch, cid) || tbl.is_null() {
        return -EINVAL;
    }
    *tbl.add(cid as usize) as i32
}

#[inline]
pub unsafe fn scx_cpu_to_cid(sch: *mut scx_sched, cpu: i32) -> i32 {
    let tbl = scx_cpu_to_cid_tbl;
    if !scx_cpu_valid(sch, cpu, core::ptr::null_mut()) || tbl.is_null() {
        return -EINVAL;
    }
    *tbl.add(cpu as usize) as i32
}

#[inline]
pub unsafe fn scx_is_cid_type() -> bool {
    static_branch_unlikely(&__scx_is_cid_type)
}

#[inline]
pub unsafe fn __scx_cmask_contains(cid: u32, m: *const scx_cmask) -> bool {
    cid >= (*m).base && cid < (*m).base + (*m).nr_cids
}

#[inline]
pub unsafe fn __scx_cmask_word(cid: u32, m: *const scx_cmask) -> *mut u64 {
    (*m).bits.add((cid / 64 - (*m).base / 64) as usize) as *mut u64
}

#[inline]
pub unsafe fn __scx_cmask_init(m: *mut scx_cmask, base: u32, nr_cids: u32, alloc_cids: u32) {
    let nr_cids = if alloc_cids < nr_cids { alloc_cids } else { nr_cids };
    (*m).base = base;
    (*m).nr_cids = nr_cids;
    (*m).alloc_words = SCX_CMASK_NR_WORDS(alloc_cids);
    core::ptr::write_bytes((*m).bits, 0, (*m).alloc_words as usize);
}

#[inline]
pub unsafe fn scx_cmask_init(m: *mut scx_cmask, base: u32, nr_cids: u32) {
    __scx_cmask_init(m, base, nr_cids, nr_cids);
}

#[inline]
pub unsafe fn scx_cmask_reframe(m: *mut scx_cmask, base: u32, nr_cids: u32) {
    if SCX_CMASK_NR_WORDS(nr_cids) > (*m).alloc_words { return; }
    if nr_cids != 0 {
        let last_word = ((base & 63) + nr_cids - 1) / 64;
        *(*m).bits = 0;
        *(*m).bits.add(last_word as usize) = 0;
    }
    (*m).base = base;
    (*m).nr_cids = nr_cids;
}

#[inline]
pub unsafe fn __scx_cmask_set(cid: u32, m: *mut scx_cmask) {
    if !__scx_cmask_contains(cid, m) { return; }
    *__scx_cmask_word(cid, m) |= 1u64 << (cid & 63);
}

#[inline]
pub unsafe fn scx_cmask_test(cid: u32, m: *const scx_cmask) -> bool {
    if !__scx_cmask_contains(cid, m) { return false; }
    *__scx_cmask_word(cid, m) & (1u64 << (cid & 63)) != 0
}

#[inline]
pub unsafe fn scx_cmask_nr_used_words(m: *const scx_cmask) -> u32 {
    if (*m).nr_cids == 0 { return 0; }
    (((*m).base & 63) + (*m).nr_cids - 1) / 64 + 1
}

#[inline]
pub unsafe fn scx_cpu_arg(cpu: i32) -> i32 {
    if scx_is_cid_type() { __scx_cpu_to_cid(cpu) } else { cpu }
}

#[inline]
pub unsafe fn scx_cpu_ret(sch: *mut scx_sched, cpu_or_cid: i32) -> i32 {
    if cpu_or_cid < 0 || !scx_is_cid_type() { cpu_or_cid } else { scx_cid_to_cpu(sch, cpu_or_cid) }
}

extern "C" {
    pub fn scx_cmask_ref_init(sch: *mut scx_sched, src: *const scx_cmask, r: *mut scx_cmask_ref) -> i32;
    pub fn scx_cmask_ref_init_kern(sch: *mut scx_sched, m: *mut scx_cmask, base: u32, nr_cids: u32, r: *mut scx_cmask_ref);
    pub fn scx_cmask_ref_shard(r: *const scx_cmask_ref, shard_idx: i32, out: *mut scx_cmask);
    pub fn scx_cmask_ref_from_cpumask(r: *const scx_cmask_ref, cpumask: *const cpumask);
    pub fn scx_cmask_ref_or(r: *const scx_cmask_ref, src: *const scx_cmask);
    pub fn scx_cmask_ref_copy(r: *const scx_cmask_ref, src: *const scx_cmask);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
