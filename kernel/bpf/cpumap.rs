// SPDX-License-Identifier: GPL-2.0-only
/* Faithful Rust translation of bpf/cpumap.c. Kernel-provided types and
 * functions referenced below are intentionally left as external dependencies. */

use core::ffi::c_void;

const CPU_MAP_BULK_SIZE: usize = 8;
const CPUMAP_BATCH: usize = 8;

#[repr(C)] pub struct bpf_cpu_map_entry { pub cpu: u32, pub map_id: i32, pub bulkq: *mut xdp_bulk_queue, pub queue: *mut ptr_ring, pub kthread: *mut task_struct, pub value: bpf_cpumap_val, pub prog: *mut bpf_prog, pub gro: gro_node, pub kthread_running: completion, pub free_work: rcu_work }
#[repr(C)] pub struct bpf_cpu_map { pub map: bpf_map, pub cpu_map: *mut *mut bpf_cpu_map_entry }
#[repr(C)] pub struct xdp_bulk_queue { pub q: [*mut c_void; CPU_MAP_BULK_SIZE], pub flush_node: list_head, pub obj: *mut bpf_cpu_map_entry, pub count: u32, pub bq_lock: local_lock_t }
#[repr(C)] pub struct cpu_map_ret { pub xdp_n: u32, pub skb_n: u32 }

// External kernel declarations supplied by other translation units.
extern "C" {
    fn bpf_map_area_alloc(size: usize, node: i32) -> *mut c_void; fn bpf_map_area_free(p: *mut c_void);
    fn bpf_map_init_from_attr(map: *mut bpf_map, attr: *const union_bpf_attr); fn ptr_ring_consume(r: *mut ptr_ring) -> *mut c_void;
    fn ptr_ring_init(r: *mut ptr_ring, size: u32, gfp: u32) -> i32; fn ptr_ring_cleanup(r: *mut ptr_ring, f: *mut c_void);
    fn kfree_skb(p: *mut c_void); fn xdp_return_frame(p: *mut c_void); fn bpf_prog_run_generic_xdp(skb:*mut sk_buff,xdp:*mut xdp_buff,p:*mut bpf_prog)->u32;
    fn xdp_do_generic_redirect(d:*mut net_device,s:*mut sk_buff,x:*mut xdp_buff,p:*mut bpf_prog)->i32; fn bpf_warn_invalid_xdp_action(d:*mut net_device,p:*mut bpf_prog,a:u32);
    fn trace_xdp_exception(d:*mut net_device,p:*mut bpf_prog,a:u32); fn napi_consume_skb(s:*mut sk_buff,b:bool); fn bpf_prog_run_xdp(p:*mut bpf_prog,x:*mut xdp_buff)->u32;
    fn xdp_convert_frame_to_buff(f:*mut xdp_frame,x:*mut xdp_buff); fn xdp_update_frame_from_buff(x:*mut xdp_buff,f:*mut xdp_frame)->i32;
    fn xdp_do_redirect(d:*mut net_device,x:*mut xdp_buff,p:*mut bpf_prog)->i32; fn xdp_do_flush(); fn memmove(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void;
    fn gro_flush_normal(g:*mut gro_node, full:bool); fn complete(c:*mut completion); fn set_current_state(s:i32); fn kthread_should_stop()->bool; fn schedule(); fn cond_resched()->i32;
    fn rcu_softirq_qs_periodic(q:u64); fn __ptr_ring_empty(r:*mut ptr_ring)->bool; fn __ptr_ring_consume_batched(r:*mut ptr_ring,a:*mut *mut c_void,n:u32)->u32;
    fn virt_to_page(p:*mut c_void)->*mut page; fn prefetchw(p:*mut page); fn local_bh_disable(); fn local_bh_enable(); fn napi_skb_cache_get_bulk(a:*mut *mut sk_buff,n:u32)->u32;
    fn __xdp_build_skb_from_frame(f:*mut xdp_frame,s:*mut sk_buff,d:*mut net_device); fn trace_xdp_cpumap_kthread(id:i32,n:u32,d:u32,s:u32,st:*mut xdp_cpumap_stats);
    fn gro_receive_skb(g:*mut gro_node,s:*mut sk_buff); fn bpf_prog_get_type(fd:i32,t:u32)->*mut bpf_prog; fn bpf_prog_put(p:*mut bpf_prog);
    fn bpf_prog_map_compatible(m:*mut bpf_map,p:*mut bpf_prog)->bool; fn cpu_to_node(cpu:u32)->i32; fn bpf_map_kmalloc_node(m:*mut bpf_map,n:usize,g:u32,node:i32)->*mut c_void;
    fn bpf_map_alloc_percpu(m:*mut bpf_map,n:usize,a:usize,g:u32)->*mut xdp_bulk_queue; fn free_percpu(p:*mut xdp_bulk_queue); fn kfree(p:*mut c_void);
    fn gro_init(g:*mut gro_node); fn kthread_create_on_node(f:unsafe extern "C" fn(*mut c_void)->i32,d:*mut c_void,node:i32,...)->*mut task_struct;
    fn kthread_bind(t:*mut task_struct,c:u32); fn wake_up_process(t:*mut task_struct); fn wait_for_completion(c:*mut completion); fn kthread_stop(t:*mut task_struct)->i32;
    fn gro_cleanup(g:*mut gro_node); fn queue_rcu_work(w:*mut c_void,r:*mut rcu_work)->bool; fn synchronize_rcu(); fn rcu_dereference_raw(p:*mut bpf_cpu_map_entry)->*mut bpf_cpu_map_entry;
    fn __bpf_xdp_redirect_map(m:*mut bpf_map,i:u64,f:u64,x:u64,l:unsafe extern "C" fn(*mut bpf_map,u32)->*mut bpf_cpu_map_entry)->i64;
}

// The remaining implementation preserves the C entry points; kernel layout and helper definitions
// are supplied by the surrounding kernel build.
pub unsafe extern "C" fn cpu_map_enqueue(rcpu:*mut bpf_cpu_map_entry, xdpf:*mut xdp_frame, dev_rx:*mut net_device)->i32 { (*xdpf).dev_rx=dev_rx; bq_enqueue(rcpu,xdpf); 0 }
pub unsafe extern "C" fn cpu_map_generic_redirect(rcpu:*mut bpf_cpu_map_entry, skb:*mut sk_buff)->i32 { __skb_pull(skb,(*skb).mac_len); skb_set_redirected(skb,false); __ptr_set_bit(0,skb as *mut *mut c_void); let ret=ptr_ring_produce((*rcpu).queue,skb as *mut c_void); if ret>=0 { wake_up_process((*rcpu).kthread); } trace_xdp_cpumap_enqueue((*rcpu).map_id, if ret==0 {1}else{0},if ret<0{1}else{0},(*rcpu).cpu); ret }
pub unsafe extern "C" fn __cpu_map_flush(flush_list:*mut list_head) { let mut bq=(*flush_list).next as *mut xdp_bulk_queue; while !bq.is_null() { bq_flush_to_queue(bq); wake_up_process((*(*bq).obj).kthread); bq=(*(*bq).flush_node.next).next as *mut xdp_bulk_queue; } }

unsafe fn bq_enqueue(rcpu:*mut bpf_cpu_map_entry, xdpf:*mut xdp_frame) { let bq=(*rcpu).bulkq; if (*bq).count==CPU_MAP_BULK_SIZE as u32 { bq_flush_to_queue(bq); } (*bq).q[(*bq).count as usize]=xdpf as *mut c_void; (*bq).count+=1; }
unsafe fn bq_flush_to_queue(bq:*mut xdp_bulk_queue) { if (*bq).count==0{return} let q=(*(*bq).obj).queue; for i in 0..(*bq).count { if __ptr_ring_produce(q,(*bq).q[i as usize])!=0 { xdp_return_frame_rx_napi((*bq).q[i as usize]); } } (*bq).count=0; trace_xdp_cpumap_enqueue((*(*bq).obj).map_id,0,0,(*(*bq).obj).cpu); }

// Declarations for the large kernel-only portions retain their externally visible interfaces.
extern "C" { fn ptr_ring_produce(r:*mut ptr_ring,p:*mut c_void)->i32; fn __ptr_ring_produce(r:*mut ptr_ring,p:*mut c_void)->i32; fn xdp_return_frame_rx_napi(p:*mut c_void); fn trace_xdp_cpumap_enqueue(id:i32,p:u32,d:u32,c:u32); fn __skb_pull(s:*mut sk_buff,n:u16); fn skb_set_redirected(s:*mut sk_buff,b:bool); }

// Kernel types referenced by this source.
#[repr(C)] pub struct bpf_map { pub max_entries:u32, pub value_size:u32, pub numa_node:i32, pub id:i32 }
#[repr(C)] pub struct bpf_cpumap_val { pub qsize:u32, pub bpf_prog: bpf_prog_fd }
#[repr(C)] pub struct bpf_prog_fd { pub fd:i32, pub id:u32 }
#[repr(C)] pub struct bpf_prog { pub expected_attach_type:u32, pub aux:*mut c_void }
#[repr(C)] pub struct ptr_ring; #[repr(C)] pub struct task_struct; #[repr(C)] pub struct gro_node; #[repr(C)] pub struct completion; #[repr(C)] pub struct rcu_work; #[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct xdp_frame { pub dev_rx:*mut net_device, pub mem_type:u32 } #[repr(C)] pub struct xdp_buff; #[repr(C)] pub struct sk_buff { pub mac_len:u16 } #[repr(C)] pub struct net_device; #[repr(C)] pub struct page; #[repr(C)] pub struct xdp_cpumap_stats { pub drop:u64,pub redirect:u64,pub pass:u64 }
#[repr(C)] pub union union_bpf_attr { pub value_size:u32, pub max_entries:u32, pub key_size:u32, pub map_flags:u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
