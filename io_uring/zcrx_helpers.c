// SPDX-License-Identifier: GPL-2.0
/* Narrow configured macro/inline and field-access boundary, never zcrx.c. */
#include "zcrx_bindings.h"

void *rust_zcrx_poison(void) { return IO_URING_PTR_POISON; }
void rust_zcrx_spin_lock_bh(spinlock_t *p) { spin_lock_bh(p); }
void rust_zcrx_spin_unlock_bh(spinlock_t *p) { spin_unlock_bh(p); }
void rust_zcrx_spin_init_ctx(spinlock_t *p) { spin_lock_init(p); }
void rust_zcrx_spin_init_rq(spinlock_t *p) { spin_lock_init(p); }
void rust_zcrx_spin_init_alloc(spinlock_t *p) { spin_lock_init(p); }
void rust_zcrx_mutex_lock(struct mutex *p) { mutex_lock(p); }
void rust_zcrx_mutex_unlock(struct mutex *p) { mutex_unlock(p); }
void rust_zcrx_mutex_init(struct mutex *p) { mutex_init(p); }
void rust_zcrx_mutex_destroy(struct mutex *p) { mutex_destroy(p); }
void rust_zcrx_assert_mutex(struct mutex *p) { lockdep_assert_held(p); }
void rust_zcrx_assert_spin(spinlock_t *p) { lockdep_assert_held(p); }
void rust_zcrx_lockdep_assert(bool p) { lockdep_assert(p); }
bool rust_zcrx_warn(bool p) { return WARN_ON(p); }

void rust_zcrx_ref_set(refcount_t *p, int n) { refcount_set(p, n); }
void rust_zcrx_ref_inc(refcount_t *p) { refcount_inc(p); }
bool rust_zcrx_ref_dec_and_test(refcount_t *p) { return refcount_dec_and_test(p); }
unsigned rust_zcrx_ref_read(const refcount_t *p) { return refcount_read(p); }
void rust_zcrx_percpu_get(struct percpu_ref *p) { percpu_ref_get(p); }
void rust_zcrx_percpu_put(struct percpu_ref *p) { percpu_ref_put(p); }
int rust_zcrx_atomic_read(const atomic_t *p) { return atomic_read(p); }
void rust_zcrx_atomic_set(atomic_t *p, int v) { atomic_set(p, v); }
void rust_zcrx_atomic_inc(atomic_t *p) { atomic_inc(p); }
int rust_zcrx_atomic_xchg(atomic_t *p, int v) { return atomic_xchg(p, v); }
bool rust_zcrx_atomic_try_cmpxchg(atomic_t *p, int *old, int v) { return atomic_try_cmpxchg(p, old, v); }
u32 rust_zcrx_load_acquire(const u32 *p) { return smp_load_acquire(p); }
void rust_zcrx_store_release(u32 *p, u32 v) { smp_store_release(p, v); }
u64 rust_zcrx_read_once_u64(const u64 *p) { return READ_ONCE(*p); }
void rust_zcrx_write_once_u64(u64 *p, u64 v) { WRITE_ONCE(*p, v); }
unsigned long rust_zcrx_array_index_nospec(unsigned long i, unsigned long n) { return array_index_nospec(i, n); }

/* container_of operates on the configured, complete zcrx.h layout. */
struct io_zcrx_area *rust_zcrx_niov_area(const struct net_iov *n) { return container_of(net_iov_owner(n), struct io_zcrx_area, nia); }
unsigned rust_zcrx_niov_idx(const struct net_iov *n) { return net_iov_idx(n); }
void rust_zcrx_niov_init(struct net_iov *n, struct net_iov_area *a) { net_iov_init(n, a, NET_IOV_IOURING); }
int rust_zcrx_niov_set_dma(struct net_iov *n, dma_addr_t dma) { return net_mp_niov_set_dma_addr(n, dma); }
netmem_ref rust_zcrx_niov_to_netmem(struct net_iov *n) { return net_iov_to_netmem(n); }
struct net_iov *rust_zcrx_netmem_to_niov(netmem_ref n) { return netmem_to_net_iov(n); }
bool rust_zcrx_netmem_is_niov(netmem_ref n) { return netmem_is_net_iov(n); }
struct folio *rust_zcrx_page_folio(struct page *p) { return page_folio(p); }
unsigned long rust_zcrx_folio_nr_pages(struct folio *p) { return folio_nr_pages(p); }
dma_addr_t rust_zcrx_sg_dma_address(struct scatterlist *p) { return sg_dma_address(p); }
unsigned rust_zcrx_sg_dma_len(struct scatterlist *p) { return sg_dma_len(p); }
int rust_zcrx_sg_alloc(struct sg_table *sgt, struct page **pages, unsigned nr, unsigned long len) { return sg_alloc_table_from_pages(sgt, pages, nr, 0, len, GFP_KERNEL_ACCOUNT); }
int rust_zcrx_dma_map(struct device *dev, struct sg_table *sgt) { return dma_map_sgtable(dev, sgt, DMA_FROM_DEVICE, DMA_ATTR_SKIP_CPU_SYNC | DMA_ATTR_WEAK_ORDERING); }
void rust_zcrx_dma_unmap(struct device *dev, struct sg_table *sgt) { dma_unmap_sgtable(dev, sgt, DMA_FROM_DEVICE, DMA_ATTR_SKIP_CPU_SYNC | DMA_ATTR_WEAK_ORDERING); }
#if defined(CONFIG_HAS_DMA) && defined(CONFIG_DMA_NEED_SYNC)
bool rust_zcrx_dma_need_sync(struct device *dev) { return dma_dev_need_sync(dev); }
void rust_zcrx_dma_sync(struct device *dev, dma_addr_t addr, size_t size, enum dma_data_direction dir) { __dma_sync_single_for_device(dev, addr, size, dir); }
#endif
dma_addr_t rust_zcrx_netmem_dma(netmem_ref n) { return page_pool_get_dma_addr_netmem(n); }
void rust_zcrx_pp_put_unrefed(struct page_pool *pp, netmem_ref n) { page_pool_put_unrefed_netmem(pp, n, -1, false); }
long rust_zcrx_pp_unref(netmem_ref n, long nr) { return page_pool_unref_netmem(n, nr); }
bool rust_zcrx_pp_unref_and_test(netmem_ref n) { return page_pool_unref_and_test(n); }
void rust_zcrx_pp_ref(netmem_ref n) { page_pool_ref_netmem(n); }
void rust_zcrx_pp_fragment(netmem_ref n, long nr) { page_pool_fragment_netmem(n, nr); }
bool rust_zcrx_pp_dma_map(struct page_pool *pp) { return pp->dma_map; }

/* Allocation flags retain the C object/array allocation distinction. */
void *rust_zcrx_kzalloc(size_t size) { return kzalloc(size, GFP_KERNEL); }
void *rust_zcrx_kmalloc_array(size_t nr, size_t size) { return kmalloc_array(nr, size, GFP_KERNEL_ACCOUNT | __GFP_ZERO); }
void *rust_zcrx_kvmalloc_array(size_t nr, size_t size) { return kvmalloc_array(nr, size, GFP_KERNEL_ACCOUNT | __GFP_ZERO); }
void *rust_zcrx_alloc_notif_req(void) { return kmem_cache_alloc(req_cachep, GFP_ATOMIC | __GFP_NOWARN | __GFP_ZERO); }
void rust_zcrx_get_uid(struct user_struct *p) { get_uid(p); }
void rust_zcrx_mmgrab(struct mm_struct *p) { mmgrab(p); }
void rust_zcrx_mmdrop(struct mm_struct *p) { mmdrop(p); }
void *rust_zcrx_region_ptr(struct io_mapped_region *p) { return io_region_get_ptr(p); }
size_t rust_zcrx_region_size(struct io_mapped_region *p) { return io_region_size(p); }
bool rust_zcrx_mem_is_zero(const void *p, size_t n) { return mem_is_zero(p, n); }
unsigned long rust_zcrx_copy_from_user(void *to, const void __user *from, size_t n) { return copy_from_user(to, from, n); }
unsigned long rust_zcrx_copy_to_user(void __user *to, const void *from, size_t n) { return copy_to_user(to, from, n); }

struct fd rust_zcrx_fdget(unsigned fd) { return fdget(fd); }
struct file *rust_zcrx_fd_file(struct fd fd) { return fd_file(fd); }
void rust_zcrx_fdput(struct fd fd) { fdput(fd); }
int rust_zcrx_xa_alloc(struct xarray *xa, u32 *id) { return xa_alloc(xa, id, NULL, xa_limit_31b, GFP_KERNEL); }
void *rust_zcrx_xa_store(struct xarray *xa, unsigned long id, void *p) { return xa_store(xa, id, p, GFP_KERNEL); }
void *rust_zcrx_xa_find(struct xarray *xa, unsigned long *id) { return xa_find(xa, id, ULONG_MAX, XA_PRESENT); }
bool rust_zcrx_xa_get_mark(struct xarray *xa, unsigned long id) { return xa_get_mark(xa, id, XA_MARK_1); }
void rust_zcrx_xa_set_mark(struct xarray *xa, unsigned long id) { xa_set_mark(xa, id, XA_MARK_1); }
struct net *rust_zcrx_current_net(void) { return current->nsproxy->net_ns; }
void rust_zcrx_netdev_hold(struct net_device *dev, netdevice_tracker *tracker) { netdev_hold(dev, tracker, GFP_KERNEL); }
void rust_zcrx_netdev_put(struct net_device *dev, netdevice_tracker *tracker) { netdev_put(dev, tracker); }
bool rust_zcrx_fatal_signal_pending(void) { return fatal_signal_pending(current); }
void rust_zcrx_cond_resched(void) { cond_resched(); }

struct zcrx_ctrl_export *rust_zcrx_ctrl_export(struct zcrx_ctrl *p) { return &p->zc_export; }
struct zcrx_ctrl_flush_rq *rust_zcrx_ctrl_flush(struct zcrx_ctrl *p) { return &p->zc_flush; }
struct zcrx_ctrl_arm_event *rust_zcrx_ctrl_arm(struct zcrx_ctrl *p) { return &p->zc_arm_event; }
struct zcrx_ctrl_add_area *rust_zcrx_ctrl_area(struct zcrx_ctrl *p) { return &p->zc_area; }
void rust_zcrx_set_task_work(struct io_kiocb *p, io_req_tw_func_t f) { p->io_task_work.func = f; }
void rust_zcrx_task_work_add(struct io_kiocb *p) { io_req_task_work_add(p); }
bool rust_zcrx_defer_get_cqe(struct io_ring_ctx *ctx, struct io_uring_cqe **cqe) { return io_defer_get_uncommited_cqe(ctx, cqe); }
struct nlattr *rust_zcrx_nla_nest_start(struct sk_buff *skb, int type) { return nla_nest_start(skb, type); }
int rust_zcrx_nla_put_uint(struct sk_buff *skb, int type, u64 value) { return nla_put_uint(skb, type, value); }
void rust_zcrx_nla_nest_cancel(struct sk_buff *skb, struct nlattr *n) { nla_nest_cancel(skb, n); }
void rust_zcrx_nla_nest_end(struct sk_buff *skb, struct nlattr *n) { nla_nest_end(skb, n); }

bool rust_zcrx_partial_kmap(struct folio *p) { return folio_test_partial_kmap(p); }
void *rust_zcrx_kmap(struct page *p) { return kmap_local_page(p); }
void rust_zcrx_kunmap(const void *p) { kunmap_local(p); }
struct page *rust_zcrx_frag_page(const skb_frag_t *p) { return skb_frag_page(p); }
unsigned rust_zcrx_frag_off(const skb_frag_t *p) { return skb_frag_off(p); }
unsigned rust_zcrx_frag_size(const skb_frag_t *p) { return skb_frag_size(p); }
bool rust_zcrx_frag_is_niov(const skb_frag_t *p) { return skb_frag_is_net_iov(p); }
unsigned rust_zcrx_skb_headlen(const struct sk_buff *p) { return skb_headlen(p); }
struct skb_shared_info *rust_zcrx_skb_shinfo(struct sk_buff *p) { return skb_shinfo(p); }
struct sk_buff *rust_zcrx_skb_next(struct sk_buff *p) { return p->next; }
struct page *rust_zcrx_virt_to_page(const void *p) { return virt_to_page(p); }
void *rust_zcrx_desc_data(read_descriptor_t *p) { return p->arg.data; }
void rust_zcrx_desc_set_data(read_descriptor_t *p, void *v) { p->arg.data = v; }
void rust_zcrx_lock_sock(struct sock *sk) { lock_sock(sk); }
bool rust_zcrx_sock_done(const struct sock *sk) { return sock_flag(sk, SOCK_DONE); }
int rust_zcrx_sock_error(struct sock *sk) { return sock_error(sk); }
unsigned rust_zcrx_sock_shutdown(const struct sock *sk) { return sk->sk_shutdown; }
u8 rust_zcrx_sock_state(const struct sock *sk) { return sk->sk_state; }
bool rust_zcrx_is_tcp_recvmsg(struct sock *sk) { return READ_ONCE(sk->sk_prot)->recvmsg == tcp_recvmsg; }
void rust_zcrx_sock_rps_record_flow(const struct sock *sk) { sock_rps_record_flow(sk); }

/* These data tables refer exclusively to Rust definitions of the algorithms. */
const struct memory_provider_ops rust_zcrx_mp_ops = {
 .alloc_netmems = rust_zcrx_pp_alloc_netmems,
 .release_netmem = rust_zcrx_pp_release_netmem,
 .init = rust_zcrx_pp_init,
 .destroy = rust_zcrx_pp_destroy,
 .nl_fill = rust_zcrx_pp_nl_fill,
 .uninstall = rust_zcrx_pp_uninstall,
};
const struct file_operations rust_zcrx_box_fops = {
 .owner = THIS_MODULE,
 .release = rust_zcrx_box_release,
};
bool rust_zcrx_warn_once_01(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_02(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_03(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_04(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_05(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_06(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_07(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_08(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_09(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_10(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_11(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_12(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_13(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_14(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_15(bool p) { return WARN_ON_ONCE(p); }
bool rust_zcrx_warn_once_16(bool p) { return WARN_ON_ONCE(p); }
struct scatterlist *rust_zcrx_sg_next(struct scatterlist *p) { return sg_next(p); }
void rust_zcrx_netdev_lock(struct net_device *p) { netdev_lock(p); }
void rust_zcrx_netdev_unlock(struct net_device *p) { netdev_unlock(p); }
bool rust_zcrx_capable(int cap) { return capable(cap); }
/* Anonymous io_ring_ctx groups vary with configuration; do not guess names. */
unsigned rust_zcrx_ctx_flags(const struct io_ring_ctx *ctx) { return ctx->flags; }
struct percpu_ref *rust_zcrx_ctx_refs(struct io_ring_ctx *ctx) { return &ctx->refs; }
struct mutex *rust_zcrx_ctx_uring_lock(struct io_ring_ctx *ctx) { return &ctx->uring_lock; }
