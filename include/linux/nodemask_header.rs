/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/nodemask.h. External types and bitmap helpers are supplied by dependencies. */

extern "C" {
    pub static mut _unused_nodemask_arg_: nodemask_t;
    #[cfg(any())]
    pub static mut nr_node_ids: u32;
    #[cfg(any())]
    pub static mut nr_online_nodes: u32;
    pub static mut node_states: [nodemask_t; NR_NODE_STATES as usize];
}

#[cfg(not(any()))]
pub const nr_node_ids: u32 = 1;
#[cfg(not(any()))]
pub const nr_online_nodes: u32 = 1;

#[allow(unused_macros)]
macro_rules! nodemask_pr_args { ($maskp:expr) => { (__nodemask_pr_numnodes($maskp), __nodemask_pr_bits($maskp)) }; }
#[inline(always)] pub unsafe fn __nodemask_pr_numnodes(m: *const nodemask_t) -> u32 { if !m.is_null() { nr_node_ids } else { 0 } }
#[inline(always)] pub unsafe fn __nodemask_pr_bits(m: *const nodemask_t) -> *const usize { if !m.is_null() { (*m).bits.as_ptr() } else { core::ptr::null() } }

#[inline(always)] pub unsafe fn __node_set(node: i32, dstp: *mut nodemask_t) { set_bit(node, (*dstp).bits.as_mut_ptr()); }
#[inline(always)] pub unsafe fn __node_clear(node: i32, dstp: *mut nodemask_t) { clear_bit(node, (*dstp).bits.as_mut_ptr()); }
#[inline(always)] pub unsafe fn __nodes_setall(dstp: *mut nodemask_t, nbits: u32) { bitmap_fill((*dstp).bits.as_mut_ptr(), nbits); }
#[inline(always)] pub unsafe fn __nodes_clear(dstp: *mut nodemask_t, nbits: u32) { bitmap_zero((*dstp).bits.as_mut_ptr(), nbits); }
#[inline(always)] pub unsafe fn __node_test_and_set(node: i32, addr: *mut nodemask_t) -> bool { test_and_set_bit(node, (*addr).bits.as_mut_ptr()) }
#[inline(always)] pub unsafe fn __nodes_and(d: *mut nodemask_t, a: *const nodemask_t, b: *const nodemask_t, n: u32) -> bool { bitmap_and((*d).bits.as_mut_ptr(), (*a).bits.as_ptr(), (*b).bits.as_ptr(), n) }
#[inline(always)] pub unsafe fn __nodes_or(d: *mut nodemask_t, a: *const nodemask_t, b: *const nodemask_t, n: u32) { bitmap_or((*d).bits.as_mut_ptr(), (*a).bits.as_ptr(), (*b).bits.as_ptr(), n); }
#[inline(always)] pub unsafe fn __nodes_xor(d: *mut nodemask_t, a: *const nodemask_t, b: *const nodemask_t, n: u32) { bitmap_xor((*d).bits.as_mut_ptr(), (*a).bits.as_ptr(), (*b).bits.as_ptr(), n); }
#[inline(always)] pub unsafe fn __nodes_andnot(d: *mut nodemask_t, a: *const nodemask_t, b: *const nodemask_t, n: u32) -> bool { bitmap_andnot((*d).bits.as_mut_ptr(), (*a).bits.as_ptr(), (*b).bits.as_ptr(), n) }
#[inline(always)] pub unsafe fn __nodes_copy(d: *mut nodemask_t, s: *const nodemask_t, n: u32) { bitmap_copy((*d).bits.as_mut_ptr(), (*s).bits.as_ptr(), n); }
#[inline(always)] pub unsafe fn __nodes_complement(d: *mut nodemask_t, s: *const nodemask_t, n: u32) { bitmap_complement((*d).bits.as_mut_ptr(), (*s).bits.as_ptr(), n); }
#[inline(always)] pub unsafe fn __nodes_equal(a: *const nodemask_t, b: *const nodemask_t, n: u32) -> bool { bitmap_equal((*a).bits.as_ptr(), (*b).bits.as_ptr(), n) }
#[inline(always)] pub unsafe fn __nodes_intersects(a: *const nodemask_t, b: *const nodemask_t, n: u32) -> bool { bitmap_intersects((*a).bits.as_ptr(), (*b).bits.as_ptr(), n) }
#[inline(always)] pub unsafe fn __nodes_subset(a: *const nodemask_t, b: *const nodemask_t, n: u32) -> bool { bitmap_subset((*a).bits.as_ptr(), (*b).bits.as_ptr(), n) }
#[inline(always)] pub unsafe fn __nodes_empty(a: *const nodemask_t, n: u32) -> bool { bitmap_empty((*a).bits.as_ptr(), n) }
#[inline(always)] pub unsafe fn __nodes_full(a: *const nodemask_t, n: u32) -> bool { bitmap_full((*a).bits.as_ptr(), n) }
#[inline(always)] pub unsafe fn __nodes_weight(a: *const nodemask_t, n: u32) -> i32 { bitmap_weight((*a).bits.as_ptr(), n) }
#[inline(always)] pub unsafe fn __first_node(a: *const nodemask_t) -> u32 { core::cmp::min(MAX_NUMNODES, find_first_bit((*a).bits.as_ptr(), MAX_NUMNODES)) }
#[inline(always)] pub unsafe fn __next_node(n: i32, a: *const nodemask_t) -> u32 { core::cmp::min(MAX_NUMNODES, find_next_bit((*a).bits.as_ptr(), MAX_NUMNODES, n + 1)) }
#[inline(always)] pub unsafe fn __next_node_in(n: i32, a: *const nodemask_t) -> u32 { let r = __next_node(n, a); if r == MAX_NUMNODES { __first_node(a) } else { r } }
#[inline(always)] pub unsafe fn init_nodemask_of_node(mask: *mut nodemask_t, node: i32) { __nodes_clear(mask, MAX_NUMNODES); __node_set(node, mask); }
#[inline(always)] pub unsafe fn __first_unset_node(a: *const nodemask_t) -> u32 { core::cmp::min(MAX_NUMNODES, find_first_zero_bit((*a).bits.as_ptr(), MAX_NUMNODES)) }

macro_rules! node_set { ($n:expr, $d:expr) => { unsafe { __node_set($n, &mut $d) } }; }
macro_rules! node_clear { ($n:expr, $d:expr) => { unsafe { __node_clear($n, &mut $d) } }; }
macro_rules! nodes_setall { ($d:expr) => { unsafe { __nodes_setall(&mut $d, MAX_NUMNODES) } }; }
macro_rules! nodes_clear { ($d:expr) => { unsafe { __nodes_clear(&mut $d, MAX_NUMNODES) } }; }
macro_rules! node_isset { ($n:expr, $m:expr) => { unsafe { test_bit($n, $m.bits.as_ptr()) } }; }
macro_rules! node_test_and_set { ($n:expr, $m:expr) => { unsafe { __node_test_and_set($n, &mut $m) } }; }
macro_rules! nodes_and { ($d:expr,$a:expr,$b:expr) => { unsafe { __nodes_and(&mut $d,&$a,&$b,MAX_NUMNODES) } }; }
macro_rules! nodes_or { ($d:expr,$a:expr,$b:expr) => { unsafe { __nodes_or(&mut $d,&$a,&$b,MAX_NUMNODES) } }; }
macro_rules! nodes_xor { ($d:expr,$a:expr,$b:expr) => { unsafe { __nodes_xor(&mut $d,&$a,&$b,MAX_NUMNODES) } }; }
macro_rules! nodes_andnot { ($d:expr,$a:expr,$b:expr) => { unsafe { __nodes_andnot(&mut $d,&$a,&$b,MAX_NUMNODES) } }; }
macro_rules! nodes_copy { ($d:expr,$s:expr) => { unsafe { __nodes_copy(&mut $d,&$s,MAX_NUMNODES) } }; }
macro_rules! nodes_complement { ($d:expr,$s:expr) => { unsafe { __nodes_complement(&mut $d,&$s,MAX_NUMNODES) } }; }
macro_rules! nodes_equal { ($a:expr,$b:expr) => { unsafe { __nodes_equal(&$a,&$b,MAX_NUMNODES) } }; }
macro_rules! nodes_intersects { ($a:expr,$b:expr) => { unsafe { __nodes_intersects(&$a,&$b,MAX_NUMNODES) } }; }
macro_rules! nodes_subset { ($a:expr,$b:expr) => { unsafe { __nodes_subset(&$a,&$b,MAX_NUMNODES) } }; }
macro_rules! nodes_empty { ($a:expr) => { unsafe { __nodes_empty(&$a,MAX_NUMNODES) } }; }
macro_rules! nodes_full { ($a:expr) => { unsafe { __nodes_full(&$a,MAX_NUMNODES) } }; }
macro_rules! nodes_weight { ($a:expr) => { unsafe { __nodes_weight(&$a,MAX_NUMNODES) } }; }
macro_rules! first_node { ($a:expr) => { unsafe { __first_node(&$a) } }; }
macro_rules! next_node { ($n:expr,$a:expr) => { unsafe { __next_node($n,&$a) } }; }
macro_rules! next_node_in { ($n:expr,$a:expr) => { unsafe { __next_node_in($n,&$a) } }; }
macro_rules! first_unset_node { ($a:expr) => { unsafe { __first_unset_node(&$a) } }; }

#[inline(always)] pub unsafe fn __nodemask_parse_user(buf: *const core::ffi::c_char, len: i32, d: *mut nodemask_t, n: i32) -> i32 { bitmap_parse_user(buf,len,(*d).bits.as_mut_ptr(),n) }
#[inline(always)] pub unsafe fn __nodelist_parse(buf: *const core::ffi::c_char, d: *mut nodemask_t, n: i32) -> i32 { bitmap_parselist(buf,(*d).bits.as_mut_ptr(),n) }
#[inline(always)] pub unsafe fn __node_remap(o: i32, old: *const nodemask_t, new: *const nodemask_t, n: i32) -> i32 { bitmap_bitremap(o,(*old).bits.as_ptr(),(*new).bits.as_ptr(),n) }
#[inline(always)] pub unsafe fn __nodes_remap(d:*mut nodemask_t,s:*const nodemask_t,o:*const nodemask_t,nm:*const nodemask_t,nb:i32){bitmap_remap((*d).bits.as_mut_ptr(),(*s).bits.as_ptr(),(*o).bits.as_ptr(),(*nm).bits.as_ptr(),nb)}
#[inline(always)] pub unsafe fn __nodes_onto(d:*mut nodemask_t,s:*const nodemask_t,r:*const nodemask_t,n:i32){bitmap_onto((*d).bits.as_mut_ptr(),(*s).bits.as_ptr(),(*r).bits.as_ptr(),n)}
#[inline(always)] pub unsafe fn __nodes_fold(d:*mut nodemask_t,s:*const nodemask_t,sz:i32,n:i32){bitmap_fold((*d).bits.as_mut_ptr(),(*s).bits.as_ptr(),sz,n)}

#[repr(C)] #[derive(Copy,Clone)] pub enum node_states { N_POSSIBLE, N_ONLINE, N_NORMAL_MEMORY, N_HIGH_MEMORY, N_MEMORY, N_CPU, N_GENERIC_INITIATOR, NR_NODE_STATES }
#[inline(always)] pub unsafe fn node_state(node:i32,state:node_states)->i32 { if MAX_NUMNODES > 1 { node_isset!(node,node_states[state as usize]) as i32 } else { (node==0) as i32 } }
#[inline(always)] pub unsafe fn node_set_state(node:i32,state:node_states){if MAX_NUMNODES>1{__node_set(node,&mut node_states[state as usize]);}}
#[inline(always)] pub unsafe fn node_clear_state(node:i32,state:node_states){if MAX_NUMNODES>1{__node_clear(node,&mut node_states[state as usize]);}}
#[inline(always)] pub unsafe fn num_node_state(state:node_states)->i32{if MAX_NUMNODES>1{__nodes_weight(&node_states[state as usize],MAX_NUMNODES)}else{1}}
#[inline(always)] pub unsafe fn node_random(maskp:*const nodemask_t)->i32 { #[cfg(all(feature="CONFIG_NUMA",any()))] { let n=find_random_bit((*maskp).bits.as_ptr(),MAX_NUMNODES); return if n<MAX_NUMNODES {n as i32} else {NUMA_NO_NODE}; } 0 }
macro_rules! num_online_nodes { () => { unsafe { num_node_state(node_states::N_ONLINE) } }; }
macro_rules! num_possible_nodes { () => { unsafe { num_node_state(node_states::N_POSSIBLE) } }; }
macro_rules! node_online { ($n:expr) => { unsafe { node_state($n,node_states::N_ONLINE) } }; }
macro_rules! node_possible { ($n:expr) => { unsafe { node_state($n,node_states::N_POSSIBLE) } }; }
macro_rules! nodes_addr { ($s:expr) => { $s.bits.as_mut_ptr() }; }

macro_rules! nodemask_parse_user { ($b:expr,$l:expr,$d:expr) => { unsafe { __nodemask_parse_user($b,$l,&mut $d,MAX_NUMNODES as i32) } }; }
macro_rules! nodelist_parse { ($b:expr,$d:expr) => { unsafe { __nodelist_parse($b,&mut $d,MAX_NUMNODES as i32) } }; }
macro_rules! node_remap { ($b:expr,$o:expr,$n:expr) => { unsafe { __node_remap($b,&$o,&$n,MAX_NUMNODES as i32) } }; }
macro_rules! nodes_remap { ($d:expr,$s:expr,$o:expr,$n:expr) => { unsafe { __nodes_remap(&mut $d,&$s,&$o,&$n,MAX_NUMNODES as i32) } }; }
macro_rules! nodes_onto { ($d:expr,$s:expr,$r:expr) => { unsafe { __nodes_onto(&mut $d,&$s,&$r,MAX_NUMNODES as i32) } }; }
macro_rules! nodes_fold { ($d:expr,$s:expr,$z:expr) => { unsafe { __nodes_fold(&mut $d,&$s,$z,MAX_NUMNODES as i32) } }; }
macro_rules! for_each_node_mask { ($node:ident,$mask:expr,$body:block) => {{ let mut $node=first_node!($mask); while $node<MAX_NUMNODES { $body; $node=next_node!($node,$mask); } }}; }
macro_rules! for_each_node_state { ($node:ident,$state:expr,$body:block) => {{ let mut $node=0; while $node==0 { $body; $node=1; } }}; }
macro_rules! for_each_node { ($node:ident,$body:block) => { for_each_node_state!($node,node_states::N_POSSIBLE,$body) }; }
macro_rules! for_each_online_node { ($node:ident,$body:block) => { for_each_node_state!($node,node_states::N_ONLINE,$body) }; }
macro_rules! for_each_node_with_cpus { ($node:ident,$body:block) => { for_each_node_state!($node,node_states::N_CPU,$body) }; }

pub const NODE_MASK_LAST_WORD: usize = BITMAP_LAST_WORD_MASK(MAX_NUMNODES);
pub const NODE_MASK_NONE: nodemask_t = nodemask_t { bits: [0; BITS_TO_LONGS(MAX_NUMNODES) as usize] };
macro_rules! NODE_MASK_ALL { () => { nodemask_t { bits: [NODE_MASK_LAST_WORD; BITS_TO_LONGS(MAX_NUMNODES) as usize] } }; }
macro_rules! nodemask_of_node { ($node:expr) => {{ let mut m = NODE_MASK_NONE; unsafe { init_nodemask_of_node(&mut m,$node); } m }}; }
macro_rules! node_online_map { () => { unsafe { &mut node_states[node_states::N_ONLINE as usize] } }; }
macro_rules! node_possible_map { () => { unsafe { &mut node_states[node_states::N_POSSIBLE as usize] } }; }
macro_rules! first_online_node { () => { unsafe { __first_node(&node_states[node_states::N_ONLINE as usize]) } }; }
macro_rules! first_memory_node { () => { unsafe { __first_node(&node_states[node_states::N_MEMORY as usize]) } }; }
macro_rules! next_online_node { ($n:expr) => { unsafe { __next_node($n,&node_states[node_states::N_ONLINE as usize]) } }; }
macro_rules! next_memory_node { ($n:expr) => { unsafe { __next_node($n,&node_states[node_states::N_MEMORY as usize]) } }; }
macro_rules! node_set_online { ($n:expr) => { unsafe { node_set_state($n,node_states::N_ONLINE) } }; }
macro_rules! node_set_offline { ($n:expr) => { unsafe { node_clear_state($n,node_states::N_ONLINE) } }; }

/* NODEMASK_ALLOC uses kmalloc/kfree when NODES_SHIFT > 8; otherwise it uses stack storage. */
#[repr(C)] pub struct nodemask_scratch { pub mask1: nodemask_t, pub mask2: nodemask_t }
macro_rules! NODEMASK_ALLOC { ($ty:ty,$name:ident,$flags:expr) => { let mut _ $name: $ty; let $name = &mut _ $name; }; }
macro_rules! NODEMASK_FREE { ($m:expr) => {{ let _ = $m; }}; }
macro_rules! NODEMASK_SCRATCH { ($x:ident) => { NODEMASK_ALLOC!(nodemask_scratch,$x,GFP_KERNEL | __GFP_NORETRY) }; }
macro_rules! NODEMASK_SCRATCH_FREE { ($x:expr) => { NODEMASK_FREE!($x) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
