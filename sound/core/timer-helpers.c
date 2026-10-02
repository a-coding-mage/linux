// SPDX-License-Identifier: GPL-2.0-or-later
/* Canonical macros, static initializers, inline and enum ABI boundaries only.
 * All ALSA timer policy, lifecycle, queue handling and ioctls live in Rust.
 */
#include "timer-bindings.h"
static LIST_HEAD(timer_list_head);
static LIST_HEAD(slave_list_head);
static LIST_HEAD(master_list_head);
static DEFINE_RWLOCK(instance_lock);
static DEFINE_MUTEX(reg_mutex);
struct list_head *rust_timer_list(void) { return &timer_list_head; }
struct list_head *rust_timer_slave_list(void) { return &slave_list_head; }
struct list_head *rust_timer_master_list(void) { return &master_list_head; }
struct mutex *rust_timer_register_mutex(void) { return &reg_mutex; }
rwlock_t *rust_timer_instance_lock(void) { return &instance_lock; }
void rust_timer_mutex_lock(struct mutex *p) { mutex_lock(p); }
void rust_timer_mutex_unlock(struct mutex *p) { mutex_unlock(p); }
void rust_timer_spin_lock(spinlock_t *p) { spin_lock(p); }
void rust_timer_spin_unlock(spinlock_t *p) { spin_unlock(p); }
void rust_timer_spin_lock_irq(spinlock_t *p) { spin_lock_irq(p); }
void rust_timer_spin_unlock_irq(spinlock_t *p) { spin_unlock_irq(p); }
unsigned long rust_timer_spin_lock_irqsave(spinlock_t *p) { unsigned long f; spin_lock_irqsave(p, f); return f; }
void rust_timer_spin_unlock_irqrestore(spinlock_t *p, unsigned long f) { spin_unlock_irqrestore(p, f); }
unsigned long rust_timer_read_lock_irqsave(rwlock_t *p) { unsigned long f; read_lock_irqsave(p, f); return f; }
void rust_timer_read_unlock_irqrestore(rwlock_t *p, unsigned long f) { read_unlock_irqrestore(p, f); }
void rust_timer_write_lock_irq(rwlock_t *p) { write_lock_irq(p); }
void rust_timer_write_unlock_irq(rwlock_t *p) { write_unlock_irq(p); }
void rust_timer_spin_init(spinlock_t *p) { spin_lock_init(p); }
void rust_timer_mutex_init(struct mutex *p) { mutex_init(p); }
void rust_timer_list_init(struct list_head *p) { INIT_LIST_HEAD(p); }
void rust_timer_list_del(struct list_head *p) { list_del(p); }
void rust_timer_list_del_init(struct list_head *p) { list_del_init(p); }
void rust_timer_list_add_tail(struct list_head *p, struct list_head *h) { list_add_tail(p, h); }
void rust_timer_list_move_tail(struct list_head *p, struct list_head *h) { list_move_tail(p, h); }
void *rust_timer_zalloc(size_t n) { return kzalloc(n, GFP_KERNEL); }
void *rust_timer_zalloc_array(size_t n, size_t s) { return kcalloc(n, s, GFP_KERNEL); }
char *rust_timer_strdup(const char *p) { return kstrdup(p, GFP_KERNEL); }
void rust_timer_ref_init(struct kref *p) { kref_init(p); }
void rust_timer_ref_get(struct kref *p) { kref_get(p); }
void rust_timer_ref_put(struct kref *p, void (*release)(struct kref *)) { kref_put(p, release); }
int rust_timer_card_number(struct snd_card *p) { return p->number; }
bool rust_timer_card_shutdown(struct snd_card *p) { return p->shutdown; }
struct device *rust_timer_card_device(struct snd_card *p) { return &p->card_dev; }
struct module *rust_timer_card_module(struct snd_card *p) { return p->module; }
struct module *rust_timer_this_module(void) { return THIS_MODULE; }
bool rust_timer_try_module_get(struct module *p) { return try_module_get(p); }
void rust_timer_module_put(struct module *p) { module_put(p); }
void rust_timer_request_global(int n) { request_module("snd-timer-%i", n); }
void rust_timer_request_card(int n) { request_module("snd-card-%i", n); }
int rust_timer_ecards_limit(void) { return snd_ecards_limit; }
void rust_timer_work_init(struct work_struct *p, work_func_t fn) { INIT_WORK(p, fn); }
void rust_timer_queue_work(struct work_struct *p) { queue_work(system_highpri_wq, p); }
void rust_timer_setup(struct timer_list *p, void (*fn)(struct timer_list *)) { timer_setup(p, fn, 0); }
unsigned long rust_timer_jiffies(void) { return jiffies; }
void rust_timer_udelay(unsigned long n) { udelay(n); }
bool rust_timer_bug_on(bool c) { return snd_BUG_ON(c); }
bool rust_timer_warn_on(bool c) { return WARN_ON(c); }
void rust_timer_warn_corrupt(void) { WARN_ONCE(1, "Corrupt snd_timer_user\n"); }
void rust_timer_invalid_slave(int slave_class) { pr_debug("ALSA: timer: invalid slave class %i\n", slave_class); }
int rust_timer_device_new(struct snd_card *c, void *d, const struct snd_device_ops *o) { return snd_device_new(c, SNDRV_DEV_TIMER, d, o); }
void *rust_timer_file_private(struct file *p) { return p->private_data; }
void rust_timer_file_set_private(struct file *p, void *d) { p->private_data = d; }
unsigned int rust_timer_file_flags(struct file *p) { return p->f_flags; }
int rust_timer_pid(void) { return current->pid; }
unsigned long rust_timer_copy_to_user(void __user *d, const void *s, unsigned long n) { return copy_to_user(d, s, n); }
unsigned long rust_timer_copy_from_user(void *d, const void __user *s, unsigned long n) { return copy_from_user(d, s, n); }
int rust_timer_get_int(const int __user *p, int *v) { return get_user(*v, p); }
int rust_timer_put_int(int __user *p, int v) { return put_user(v, p); }
int rust_timer_get_u32(const u32 __user *p, unsigned long *v) { return get_user(*v, p); }
void rust_timer_wait_head_init(wait_queue_head_t *p) { init_waitqueue_head(p); }
void rust_timer_wait_init(wait_queue_entry_t *p) { init_waitqueue_entry(p, current); }
void rust_timer_current_interruptible(void) { set_current_state(TASK_INTERRUPTIBLE); }
bool rust_timer_signal_pending(void) { return signal_pending(current); }
void rust_timer_wake_up(wait_queue_head_t *p) { wake_up(p); }
void rust_timer_poll_wait(struct file *f, wait_queue_head_t *p, poll_table *pt) { poll_wait(f, p, pt); }
void *rust_timer_compat_ptr(unsigned long p) { return compat_ptr(p); }
void rust_timer_info_set_read(struct snd_info_entry *p, void (*fn)(struct snd_info_entry *, struct snd_info_buffer *)) {
#ifdef CONFIG_SND_PROC_FS
 p->c.text.read = fn;
#endif
}
void rust_timer_oss_register(void) {
#ifdef SNDRV_OSS_INFO_DEV_TIMERS
 snd_oss_info_register(SNDRV_OSS_INFO_DEV_TIMERS, SNDRV_CARDS - 1, "system timer");
#endif
}
void rust_timer_oss_unregister(void) {
#ifdef SNDRV_OSS_INFO_DEV_TIMERS
 snd_oss_info_unregister(SNDRV_OSS_INFO_DEV_TIMERS, SNDRV_CARDS - 1);
#endif
}
#ifdef CONFIG_SND_UTIMER
static DEFINE_IDA(utimer_ids);
int rust_timer_take_id(unsigned int max) { return ida_alloc_max(&utimer_ids, max, GFP_KERNEL); }
void rust_timer_put_id(unsigned int id) { ida_free(&utimer_ids, id); }
#endif
module_param_named(timer_limit, rust_snd_timer_limit, int, 0444);
MODULE_PARM_DESC(timer_limit, "Maximum global timers in system.");
module_param_named(timer_tstamp_monotonic, rust_snd_timer_tstamp_monotonic, int, 0444);
MODULE_PARM_DESC(timer_tstamp_monotonic, "Use posix monotonic clock source for timestamps (default).");
MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>, Takashi Iwai <tiwai@suse.de>");
MODULE_DESCRIPTION("ALSA timer interface");
MODULE_LICENSE("GPL");
MODULE_ALIAS_CHARDEV(CONFIG_SND_MAJOR, SNDRV_MINOR_TIMER);
MODULE_ALIAS("devname:snd/timer");
/* module_init needs a C definition for its alias and init-section ABI. */
static int __init alsa_timer_init(void) { return rust_alsa_timer_init(); }
static void __exit alsa_timer_exit(void) { rust_alsa_timer_exit(); }
module_init(alsa_timer_init);
module_exit(alsa_timer_exit);
