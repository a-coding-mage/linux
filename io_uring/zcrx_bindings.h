/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_IO_URING_ZCRX_BINDINGS_H
#define LUPOS_IO_URING_ZCRX_BINDINGS_H
#include <linux/kernel.h>
#include <linux/errno.h>
#include <linux/dma-map-ops.h>
#include <linux/mm.h>
#include <linux/nospec.h>
#include <linux/io_uring.h>
#include <linux/netdevice.h>
#include <linux/rtnetlink.h>
#include <linux/skbuff_ref.h>
#include <linux/anon_inodes.h>
#include <linux/file.h>
#include <linux/nsproxy.h>
#include <net/page_pool/helpers.h>
#include <net/page_pool/memory_provider.h>
#include <net/netlink.h>
#include <net/netdev_queues.h>
#include <net/netdev_rx_queue.h>
#include <net/tcp.h>
#include <net/rps.h>
#include <trace/events/page_pool.h>
#include <uapi/linux/io_uring.h>
#include "io_uring.h"
#include "kbuf.h"
#include "memmap.h"
#include "zcrx.h"
#include "rsrc.h"

enum {
 RUST_ZCRX_PAGE_SHIFT = PAGE_SHIFT,
 RUST_ZCRX_PAGE_SIZE = PAGE_SIZE,
 RUST_ZCRX_REFILL_CAP = MIN(64 * MAX(PAGE_SIZE / 1024, 1), 1024),
 RUST_ZCRX_HEAD_OFFSET = offsetof(struct zcrx_rq_hdr, head),
 RUST_ZCRX_TAIL_OFFSET = offsetof(struct zcrx_rq_hdr, tail),
 RUST_ZCRX_RQES_OFFSET = ALIGN(sizeof(struct zcrx_rq_hdr), L1_CACHE_BYTES),
};
/* Typed aliases let configured bindgen evaluate compound macros and enum values. */
static const unsigned int RUST_ZCRX_SUPPORTED_REG_FLAGS = ZCRX_SUPPORTED_REG_FLAGS;
static const unsigned int RUST_ZCRX_EVENT_TYPE_MASK = ZCRX_EVENT_TYPE_MASK;
static const u64 RUST_ZCRX_AREA_MASK = IORING_ZCRX_AREA_MASK;
static const unsigned int RUST_ZCRX_OP_NOP = IORING_OP_NOP;
static const unsigned int RUST_ZCRX_F_MULTISHOT = IO_URING_F_MULTISHOT;

/* Original same-unit symbols are implemented in Rust, never C fallbacks. */
netmem_ref rust_zcrx_pp_alloc_netmems(struct page_pool *, gfp_t);
bool rust_zcrx_pp_release_netmem(struct page_pool *, netmem_ref);
int rust_zcrx_pp_init(struct page_pool *);
void rust_zcrx_pp_destroy(struct page_pool *);
int rust_zcrx_pp_nl_fill(void *, struct sk_buff *, struct netdev_rx_queue *);
void rust_zcrx_pp_uninstall(void *, struct netdev_rx_queue *);
int rust_zcrx_box_release(struct inode *, struct file *);
extern const struct memory_provider_ops rust_zcrx_mp_ops;
extern const struct file_operations rust_zcrx_box_fops;

/* Prototypes below correspond one-for-one to zcrx_helpers.c definitions. */
void *rust_zcrx_poison(void);
void rust_zcrx_spin_lock_bh(spinlock_t *p);
void rust_zcrx_spin_unlock_bh(spinlock_t *p);
void rust_zcrx_spin_init_ctx(spinlock_t *p);
void rust_zcrx_spin_init_rq(spinlock_t *p);
void rust_zcrx_spin_init_alloc(spinlock_t *p);
void rust_zcrx_mutex_lock(struct mutex *p);
void rust_zcrx_mutex_unlock(struct mutex *p);
void rust_zcrx_mutex_init(struct mutex *p);
void rust_zcrx_mutex_destroy(struct mutex *p);
void rust_zcrx_assert_mutex(struct mutex *p);
void rust_zcrx_assert_spin(spinlock_t *p);
void rust_zcrx_lockdep_assert(bool p);
bool rust_zcrx_warn(bool p);
void rust_zcrx_ref_set(refcount_t *p, int n);
void rust_zcrx_ref_inc(refcount_t *p);
bool rust_zcrx_ref_dec_and_test(refcount_t *p);
unsigned rust_zcrx_ref_read(const refcount_t *p);
void rust_zcrx_percpu_get(struct percpu_ref *p);
void rust_zcrx_percpu_put(struct percpu_ref *p);
int rust_zcrx_atomic_read(const atomic_t *p);
void rust_zcrx_atomic_set(atomic_t *p, int v);
void rust_zcrx_atomic_inc(atomic_t *p);
int rust_zcrx_atomic_xchg(atomic_t *p, int v);
bool rust_zcrx_atomic_try_cmpxchg(atomic_t *p, int *old, int v);
u32 rust_zcrx_load_acquire(const u32 *p);
void rust_zcrx_store_release(u32 *p, u32 v);
u64 rust_zcrx_read_once_u64(const u64 *p);
void rust_zcrx_write_once_u64(u64 *p, u64 v);
unsigned long rust_zcrx_array_index_nospec(unsigned long i, unsigned long n);
struct io_zcrx_area *rust_zcrx_niov_area(const struct net_iov *n);
unsigned rust_zcrx_niov_idx(const struct net_iov *n);
void rust_zcrx_niov_init(struct net_iov *n, struct net_iov_area *a);
int rust_zcrx_niov_set_dma(struct net_iov *n, dma_addr_t dma);
netmem_ref rust_zcrx_niov_to_netmem(struct net_iov *n);
struct net_iov *rust_zcrx_netmem_to_niov(netmem_ref n);
bool rust_zcrx_netmem_is_niov(netmem_ref n);
struct folio *rust_zcrx_page_folio(struct page *p);
unsigned long rust_zcrx_folio_nr_pages(struct folio *p);
dma_addr_t rust_zcrx_sg_dma_address(struct scatterlist *p);
unsigned rust_zcrx_sg_dma_len(struct scatterlist *p);
int rust_zcrx_sg_alloc(struct sg_table *sgt, struct page **pages, unsigned nr, unsigned long len);
int rust_zcrx_dma_map(struct device *dev, struct sg_table *sgt);
void rust_zcrx_dma_unmap(struct device *dev, struct sg_table *sgt);
bool rust_zcrx_dma_need_sync(struct device *dev);
void rust_zcrx_dma_sync(struct device *dev, dma_addr_t addr, size_t size, enum dma_data_direction dir);
dma_addr_t rust_zcrx_netmem_dma(netmem_ref n);
void rust_zcrx_pp_put_unrefed(struct page_pool *pp, netmem_ref n);
long rust_zcrx_pp_unref(netmem_ref n, long nr);
bool rust_zcrx_pp_unref_and_test(netmem_ref n);
void rust_zcrx_pp_ref(netmem_ref n);
void rust_zcrx_pp_fragment(netmem_ref n, long nr);
bool rust_zcrx_pp_dma_map(struct page_pool *pp);
void *rust_zcrx_kzalloc(size_t size);
void *rust_zcrx_kmalloc_array(size_t nr, size_t size);
void *rust_zcrx_kvmalloc_array(size_t nr, size_t size);
void *rust_zcrx_alloc_notif_req(void);
void rust_zcrx_get_uid(struct user_struct *p);
void rust_zcrx_mmgrab(struct mm_struct *p);
void rust_zcrx_mmdrop(struct mm_struct *p);
void *rust_zcrx_region_ptr(struct io_mapped_region *p);
size_t rust_zcrx_region_size(struct io_mapped_region *p);
bool rust_zcrx_mem_is_zero(const void *p, size_t n);
unsigned long rust_zcrx_copy_from_user(void *to, const void __user *from, size_t n);
unsigned long rust_zcrx_copy_to_user(void __user *to, const void *from, size_t n);
struct fd rust_zcrx_fdget(unsigned fd);
struct file *rust_zcrx_fd_file(struct fd fd);
void rust_zcrx_fdput(struct fd fd);
int rust_zcrx_xa_alloc(struct xarray *xa, u32 *id);
void *rust_zcrx_xa_store(struct xarray *xa, unsigned long id, void *p);
void *rust_zcrx_xa_find(struct xarray *xa, unsigned long *id);
bool rust_zcrx_xa_get_mark(struct xarray *xa, unsigned long id);
void rust_zcrx_xa_set_mark(struct xarray *xa, unsigned long id);
struct net *rust_zcrx_current_net(void);
void rust_zcrx_netdev_hold(struct net_device *dev, netdevice_tracker *tracker);
void rust_zcrx_netdev_put(struct net_device *dev, netdevice_tracker *tracker);
bool rust_zcrx_fatal_signal_pending(void);
void rust_zcrx_cond_resched(void);
struct zcrx_ctrl_export *rust_zcrx_ctrl_export(struct zcrx_ctrl *p);
struct zcrx_ctrl_flush_rq *rust_zcrx_ctrl_flush(struct zcrx_ctrl *p);
struct zcrx_ctrl_arm_event *rust_zcrx_ctrl_arm(struct zcrx_ctrl *p);
struct zcrx_ctrl_add_area *rust_zcrx_ctrl_area(struct zcrx_ctrl *p);
void rust_zcrx_set_task_work(struct io_kiocb *p, io_req_tw_func_t f);
void rust_zcrx_task_work_add(struct io_kiocb *p);
bool rust_zcrx_defer_get_cqe(struct io_ring_ctx *ctx, struct io_uring_cqe **cqe);
struct nlattr *rust_zcrx_nla_nest_start(struct sk_buff *skb, int type);
int rust_zcrx_nla_put_uint(struct sk_buff *skb, int type, u64 value);
void rust_zcrx_nla_nest_cancel(struct sk_buff *skb, struct nlattr *n);
void rust_zcrx_nla_nest_end(struct sk_buff *skb, struct nlattr *n);
bool rust_zcrx_partial_kmap(struct folio *p);
void *rust_zcrx_kmap(struct page *p);
void rust_zcrx_kunmap(const void *p);
struct page *rust_zcrx_frag_page(const skb_frag_t *p);
unsigned rust_zcrx_frag_off(const skb_frag_t *p);
unsigned rust_zcrx_frag_size(const skb_frag_t *p);
bool rust_zcrx_frag_is_niov(const skb_frag_t *p);
unsigned rust_zcrx_skb_headlen(const struct sk_buff *p);
struct skb_shared_info *rust_zcrx_skb_shinfo(struct sk_buff *p);
struct sk_buff *rust_zcrx_skb_next(struct sk_buff *p);
struct page *rust_zcrx_virt_to_page(const void *p);
void *rust_zcrx_desc_data(read_descriptor_t *p);
void rust_zcrx_desc_set_data(read_descriptor_t *p, void *v);
void rust_zcrx_lock_sock(struct sock *sk);
bool rust_zcrx_sock_done(const struct sock *sk);
int rust_zcrx_sock_error(struct sock *sk);
unsigned rust_zcrx_sock_shutdown(const struct sock *sk);
u8 rust_zcrx_sock_state(const struct sock *sk);
bool rust_zcrx_is_tcp_recvmsg(struct sock *sk);
void rust_zcrx_sock_rps_record_flow(const struct sock *sk);
bool rust_zcrx_warn_once_01(bool p);
bool rust_zcrx_warn_once_02(bool p);
bool rust_zcrx_warn_once_03(bool p);
bool rust_zcrx_warn_once_04(bool p);
bool rust_zcrx_warn_once_05(bool p);
bool rust_zcrx_warn_once_06(bool p);
bool rust_zcrx_warn_once_07(bool p);
bool rust_zcrx_warn_once_08(bool p);
bool rust_zcrx_warn_once_09(bool p);
bool rust_zcrx_warn_once_10(bool p);
bool rust_zcrx_warn_once_11(bool p);
bool rust_zcrx_warn_once_12(bool p);
bool rust_zcrx_warn_once_13(bool p);
bool rust_zcrx_warn_once_14(bool p);
bool rust_zcrx_warn_once_15(bool p);
bool rust_zcrx_warn_once_16(bool p);
struct scatterlist *rust_zcrx_sg_next(struct scatterlist *p);
void rust_zcrx_netdev_lock(struct net_device *p);
void rust_zcrx_netdev_unlock(struct net_device *p);
bool rust_zcrx_capable(int cap);
unsigned rust_zcrx_ctx_flags(const struct io_ring_ctx *ctx);
struct percpu_ref *rust_zcrx_ctx_refs(struct io_ring_ctx *ctx);
struct mutex *rust_zcrx_ctx_uring_lock(struct io_ring_ctx *ctx);
#endif
