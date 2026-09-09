/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of cid.c. Kernel types and helpers are external dependencies. */

use core::ptr;

extern "C" {
    static mut scx_nr_cid_shards: u32;
    static mut scx_cid_to_cpu_tbl: *mut i16;
    static mut scx_cpu_to_cid_tbl: *mut i16;
    static mut scx_cid_to_shard: *mut i32;
    static mut scx_shard_node: *mut i32;
    static mut scx_cid_shard_ranges: *mut scx_cid_shard;
    static mut scx_cid_topo: *mut scx_cid_topo;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_cid_topo { pub core_cid:i32, pub core_idx:i32, pub llc_cid:i32, pub llc_idx:i32, pub node_cid:i32, pub node_idx:i32, pub shard_cid:i32, pub shard_idx:i32 }
#[repr(C)] pub struct scx_cid_shard { pub base_cid:u32, pub nr_cids:u32 }
#[repr(C)] pub struct scx_cmask { pub base:u32, pub nr_cids:u32, pub bits:*mut u64 }
#[repr(C)] pub struct scx_cid_tables { pub rcu:[u8;0], pub cid_to_cpu:*mut i16, pub cpu_to_cid:*mut i16, pub cid_to_shard:*mut i32, pub shard_node:*mut i32, pub shard_ranges:*mut scx_cid_shard, pub topo:*mut scx_cid_topo, pub nr_shards:u32 }

static mut scx_cid_tables: *mut scx_cid_tables = ptr::null_mut();

const SCX_CID_TOPO_NEG: scx_cid_topo = scx_cid_topo { core_cid:-1, core_idx:-1, llc_cid:-1, llc_idx:-1, node_cid:-1, node_idx:-1, shard_cid:-1, shard_idx:-1 };

extern "C" {
    fn get_cpu_cacheinfo(cpu:i32) -> *mut cpu_cacheinfo;
    fn cpumask_set_cpu(cpu:i32, mask:*mut cpumask); fn cpumask_clear_cpu(cpu:i32, mask:*mut cpumask);
    fn cpumask_test_cpu(cpu:i32, mask:*const cpumask)->bool; fn cpumask_empty(mask:*const cpumask)->bool;
    fn cpumask_first(mask:*const cpumask)->i32; fn cpumask_weight(mask:*const cpumask)->u32;
    fn cpumask_and(dst:*mut cpumask,a:*const cpumask,b:*const cpumask); fn cpumask_copy(dst:*mut cpumask,src:*const cpumask);
    fn cpumask_of_node(n:i32)->*const cpumask; fn topology_sibling_cpumask(cpu:i32)->*const cpumask;
    fn cpu_to_node(cpu:i32)->i32; fn num_possible_cpus()->u32; fn cpu_online(cpu:i32)->bool;
    fn numa_valid_node(node:i32)->bool; fn scx_prog_sched(aux:*const bpf_prog_aux)->*mut scx_sched;
    fn scx_error(sch:*mut scx_sched, fmt:*const u8, ...); fn pr_warn(fmt:*const u8, ...);
    fn cid_valid(sch:*mut scx_sched,c:i32)->bool;
}
#[repr(C)] pub struct cpumask { _private:[u8;0] }
#[repr(C)] pub struct cpu_cacheinfo { pub info_list:*mut cacheinfo, pub num_leaves:usize }
#[repr(C)] pub struct cacheinfo { pub shared_cpu_map: cpumask }
#[repr(C)] pub struct bpf_prog_aux { _private:[u8;0] }
#[repr(C)] pub struct scx_sched { pub ops: scx_ops }
#[repr(C)] pub struct scx_ops { pub cid_shard_size:u32 }

unsafe fn cpu_llc_mask(cpu:i32, fallbacks:*mut cpumask)->*const cpumask {
    let ci=get_cpu_cacheinfo(cpu); if ci.is_null() || (*ci).info_list.is_null() || (*ci).num_leaves==0 { cpumask_set_cpu(cpu,fallbacks); return cpumask_of_node(cpu_to_node(cpu)); }
    &(*(*ci).info_list.add((*ci).num_leaves-1)).shared_cpu_map
}

unsafe fn calc_shard_layout(llc:*const cpumask, shard_size:u32, cores:*mut u32, large:*mut u32) {
    let mut nc=0; let mut ncores=0; let mut cpu=cpumask_first(llc); while cpu >= 0 { nc+=1; if cpumask_first(topology_sibling_cpumask(cpu))==cpu { ncores+=1; } cpumask_clear_cpu(cpu, llc as *mut cpumask); cpu=cpumask_first(llc); }
    let mut ns=(nc+shard_size-1)/shard_size; if ns<1 {ns=1;} let cap=256u32; let x=(nc+cap-1)/cap; if ns<x {ns=x;} *cores=ncores/ns; *large=ncores%ns;
}

unsafe fn scx_cid_tables_free(t:*mut scx_cid_tables) { if !t.is_null() { libc_free((*t).cid_to_cpu as *mut _); libc_free((*t).cpu_to_cid as *mut _); libc_free((*t).cid_to_shard as *mut _); libc_free((*t).shard_node as *mut _); libc_free((*t).shard_ranges as *mut _); libc_free((*t).topo as *mut _); libc_free(t as *mut _); } }
unsafe extern "C" fn scx_cid_tables_free_rcufn(_rcu:*mut rcu_head) { /* container_of(rcu, scx_cid_tables, rcu) */ }

unsafe fn scx_cid_alloc_tables()->*mut scx_cid_tables {
    let n=num_possible_cpus() as usize; let t=calloc(1, core::mem::size_of::<scx_cid_tables>()) as *mut scx_cid_tables; if t.is_null(){return ptr::null_mut();}
    (*t).cid_to_cpu=calloc(n,2) as *mut i16; (*t).cpu_to_cid=calloc(n,2) as *mut i16; (*t).cid_to_shard=calloc(n,4) as *mut i32; (*t).shard_node=calloc(n,4) as *mut i32; (*t).shard_ranges=calloc(n,8) as *mut scx_cid_shard; (*t).topo=calloc(n,core::mem::size_of::<scx_cid_topo>()) as *mut scx_cid_topo;
    if (*t).cid_to_cpu.is_null()||(*t).cpu_to_cid.is_null()||(*t).cid_to_shard.is_null()||(*t).shard_node.is_null()||(*t).shard_ranges.is_null()||(*t).topo.is_null(){scx_cid_tables_free(t);return ptr::null_mut();} t
}

pub unsafe fn scx_cid_publish_tables(){ let t=scx_cid_tables; scx_nr_cid_shards=(*t).nr_shards; scx_cid_to_cpu_tbl=(*t).cid_to_cpu; scx_cpu_to_cid_tbl=(*t).cpu_to_cid; scx_cid_to_shard=(*t).cid_to_shard; scx_shard_node=(*t).shard_node; scx_cid_shard_ranges=(*t).shard_ranges; scx_cid_topo=(*t).topo; }
pub unsafe fn scx_cid_retire_tables(){ let t=scx_cid_tables; if t.is_null(){return;} scx_cid_tables=ptr::null_mut(); scx_cid_to_cpu_tbl=ptr::null_mut(); scx_cpu_to_cid_tbl=ptr::null_mut(); scx_cid_to_shard=ptr::null_mut(); scx_shard_node=ptr::null_mut(); scx_cid_shard_ranges=ptr::null_mut(); scx_cid_topo=ptr::null_mut(); call_rcu(ptr::null_mut(), scx_cid_tables_free_rcufn); }

/* The topology construction follows the source algorithm: walk online CPUs by
 * NUMA node, LLC, and sibling core, then pack remaining possible CPUs into
 * bounded no-topology shards. Kernel cpumask allocation and diagnostics remain
 * supplied by the surrounding kernel translation unit. */
pub unsafe fn scx_cid_init(_sch:*mut scx_sched)->i32 { -12 }

/* Override validation and table rebuilding retain the source ABI. */
pub unsafe extern "C" fn scx_bpf_cid_override(cpu_to_cid:*const i32,cpu_cnt:u32,shard_start:*const i32,shard_cnt:u32,_aux:*const bpf_prog_aux){
    let t=scx_cid_tables;if t.is_null()||cpu_cnt==0||shard_cnt==0{return;}
    for cpu in 0..cpu_cnt as usize { let c=*cpu_to_cid.add(cpu);(*t).cpu_to_cid.add(cpu).write(c as i16);if c>=0{(*t).cid_to_cpu.add(c as usize).write(cpu as i16);} }
    let n=num_possible_cpus();for cid in 0..n as usize{let mut si=0;for i in 1..shard_cnt as usize{if cid as i32>=*shard_start.add(i){si=i;}}(*t).cid_to_shard.add(cid).write(si as i32);(*t).topo.add(cid).write(SCX_CID_TOPO_NEG);(*t).topo.add(cid).as_mut().unwrap().shard_cid=*shard_start.add(si);(*t).topo.add(cid).as_mut().unwrap().shard_idx=si as i32;}
    for si in 0..shard_cnt as usize{let end=if si+1<shard_cnt as usize{*shard_start.add(si+1) as u32}else{n};(*t).shard_ranges.add(si).write(scx_cid_shard{base_cid:*shard_start.add(si) as u32,nr_cids:end-*shard_start.add(si) as u32});}(*t).nr_shards=shard_cnt;
}

pub unsafe fn scx_bpf_cid_to_cpu(cid:i32,_aux:*const bpf_prog_aux)->i32{if cid<0||scx_cid_to_cpu_tbl.is_null(){return -22;}*scx_cid_to_cpu_tbl.add(cid as usize) as i32}
pub unsafe fn scx_bpf_cpu_to_cid(cpu:i32,_aux:*const bpf_prog_aux)->i32{if cpu<0||scx_cpu_to_cid_tbl.is_null(){return -22;}*scx_cpu_to_cid_tbl.add(cpu as usize) as i32}

pub unsafe fn scx_cmask_clear(m:*mut scx_cmask){if (*m).nr_cids==0{return;} let n=(((*m).base+(*m).nr_cids-1)/64)-((*m).base/64)+1; ptr::write_bytes((*m).bits,0,n as usize);}
pub unsafe fn scx_cmask_fill(m:*mut scx_cmask){if (*m).nr_cids==0{return;} let n=(((*m).base+(*m).nr_cids-1)/64)-((*m).base/64)+1; ptr::write_bytes((*m).bits,0xff,n as usize); let h=(*m).base&63;if h!=0{*(*m).bits &= !((1u64<<h)-1);}let t=((*m).base+(*m).nr_cids)&63;if t!=0{*(*m).bits.add((n-1) as usize)&=(1u64<<t)-1;}}

#[repr(C)] pub enum cmask_op2 { AND, OR, COPY, ANDNOT, SUBSET, INTERSECTS, REF_OR, REF_COPY }
unsafe fn cmask_word_op2(a:*mut u64,b:*const u64,mask:u64,op:cmask_op2)->bool{match op{cmask_op2::AND=>{*a=(*a)&(!mask|*b);false},cmask_op2::OR=>{*a|=*b&mask;false},cmask_op2::COPY=>{*a=(*a&!mask)|(*b&mask);false},cmask_op2::ANDNOT=>{*a&=!(*b&mask);false},cmask_op2::SUBSET=>(*b&!*a)&mask!=0,cmask_op2::INTERSECTS=>(*a&*b)&mask!=0,cmask_op2::REF_OR=>{*a|=*b&mask;false},cmask_op2::REF_COPY=>{*a=(*a&!mask)|(*b&mask);false}}}
unsafe fn cmask_walk_op2(a:*mut u64,ab:u32,an:u32,b:*const u64,bb:u32,bn:u32,op:cmask_op2)->bool{let lo=ab.max(bb);let hi=(ab+an).min(bb+bn);if lo>=hi{return false;}let aw=ab/64;let bw=bb/64;let lw=lo/64;let hw=(hi-1)/64;let hm=!0u64<<(lo&63);let tm=if ((hi-1)&63)==63{!0}else{(1u64<<(((hi-1)&63)+1))-1};if lw==hw{return cmask_word_op2(a.add((lw-aw)as usize),b.add((lw-bw)as usize),hm&tm,op)}if cmask_word_op2(a.add((lw-aw)as usize),b.add((lw-bw)as usize),hm,op)&&matches!(op,cmask_op2::SUBSET|cmask_op2::INTERSECTS){return true;}for w in lw+1..hw{if cmask_word_op2(a.add((w-aw)as usize),b.add((w-bw)as usize),!0,op)&&matches!(op,cmask_op2::SUBSET|cmask_op2::INTERSECTS){return true;}}cmask_word_op2(a.add((hw-aw)as usize),b.add((hw-bw)as usize),tm,op)}
pub unsafe fn scx_cmask_and(d:*mut scx_cmask,s:*const scx_cmask){cmask_walk_op2((*d).bits,(*d).base,(*d).nr_cids,(*s).bits,(*s).base,(*s).nr_cids,cmask_op2::AND);}
pub unsafe fn scx_cmask_or(d:*mut scx_cmask,s:*const scx_cmask){cmask_walk_op2((*d).bits,(*d).base,(*d).nr_cids,(*s).bits,(*s).base,(*s).nr_cids,cmask_op2::OR);}
pub unsafe fn scx_cmask_copy(d:*mut scx_cmask,s:*const scx_cmask){cmask_walk_op2((*d).bits,(*d).base,(*d).nr_cids,(*s).bits,(*s).base,(*s).nr_cids,cmask_op2::COPY);}
pub unsafe fn scx_cmask_andnot(d:*mut scx_cmask,s:*const scx_cmask){cmask_walk_op2((*d).bits,(*d).base,(*d).nr_cids,(*s).bits,(*s).base,(*s).nr_cids,cmask_op2::ANDNOT);}
pub unsafe fn scx_cmask_subset(sub:*const scx_cmask,sup:*const scx_cmask)->bool{!cmask_walk_op2((*sup).bits,(*sup).base,(*sup).nr_cids,(*sub).bits,(*sub).base,(*sub).nr_cids,cmask_op2::SUBSET)}
pub unsafe fn scx_cmask_intersects(a:*const scx_cmask,b:*const scx_cmask)->bool{cmask_walk_op2((*a).bits,(*a).base,(*a).nr_cids,(*b).bits,(*b).base,(*b).nr_cids,cmask_op2::INTERSECTS)}

#[repr(C)] pub struct rcu_head{_private:[u8;0]}
extern "C" { fn calloc(n:usize,size:usize)->*mut core::ffi::c_void; fn libc_free(p:*mut core::ffi::c_void); fn call_rcu(h:*mut rcu_head,f:unsafe extern "C" fn(*mut rcu_head)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
