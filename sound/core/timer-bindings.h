/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef LUPOS_SND_TIMER_BINDINGS_H
#define LUPOS_SND_TIMER_BINDINGS_H
#include <linux/delay.h>
#include <linux/init.h>
#include <linux/slab.h>
#include <linux/time.h>
#include <linux/mutex.h>
#include <linux/device.h>
#include <linux/module.h>
#include <linux/string.h>
#include <linux/sched/signal.h>
#include <linux/anon_inodes.h>
#include <linux/idr.h>
#include <linux/compat.h>
#include <sound/core.h>
#include <sound/timer.h>
#include <sound/control.h>
#include <sound/info.h>
#include <sound/minors.h>
#include <sound/initval.h>
#include <linux/kmod.h>
/* Private ABI declarations copied from the unchanged timer.c/timer_compat.c. */
enum timer_tread_format {
	TREAD_FORMAT_NONE = 0,
	TREAD_FORMAT_TIME64,
	TREAD_FORMAT_TIME32,
};

struct snd_timer_tread32 {
	int event;
	s32 tstamp_sec;
	s32 tstamp_nsec;
	unsigned int val;
};

struct snd_timer_tread64 {
	int event;
	u8 pad1[4];
	s64 tstamp_sec;
	s64 tstamp_nsec;
	unsigned int val;
	u8 pad2[4];
};

struct snd_timer_user {
	struct snd_timer_instance *timeri;
	int tread;		/* enhanced read with timestamps and events */
	unsigned long ticks;
	unsigned long overrun;
	int qhead;
	int qtail;
	int qused;
	int queue_size;
	bool disconnected;
	struct snd_timer_read *queue;
	struct snd_timer_tread64 *tqueue;
	spinlock_t qlock;
	unsigned long last_resolution;
	unsigned int filter;
	struct timespec64 tstamp;		/* trigger tstamp */
	wait_queue_head_t qchange_sleep;
	struct snd_fasync *fasync;
	struct mutex ioctl_lock;
};

struct snd_timer_status32 {
	s32 tstamp_sec;			/* Timestamp - last update */
	s32 tstamp_nsec;
	unsigned int resolution;	/* current period resolution in ns */
	unsigned int lost;		/* counter of master tick lost */
	unsigned int overrun;		/* count of read queue overruns */
	unsigned int queue;		/* used queue size */
	unsigned char reserved[64];	/* reserved */
};

#define SNDRV_TIMER_IOCTL_STATUS32	_IOR('T', 0x14, struct snd_timer_status32)

struct snd_timer_status64 {
	s64 tstamp_sec;			/* Timestamp - last update */
	s64 tstamp_nsec;
	unsigned int resolution;	/* current period resolution in ns */
	unsigned int lost;		/* counter of master tick lost */
	unsigned int overrun;		/* count of read queue overruns */
	unsigned int queue;		/* used queue size */
	unsigned char reserved[64];	/* reserved */
};

#ifdef CONFIG_SND_UTIMER
#define SNDRV_UTIMERS_MAX_COUNT 128
/* Internal data structure for keeping the state of the userspace-driven timer */
struct snd_utimer {
	char *name;
	struct snd_timer *timer;
	unsigned int id;
};
#endif

#define SNDRV_TIMER_IOCTL_STATUS64	_IOR('T', 0x14, struct snd_timer_status64)

struct snd_timer_system_private {
	struct timer_list tlist;
	struct snd_timer *snd_timer;
	unsigned long last_expires;
	unsigned long last_jiffies;
	unsigned long correction;
};
struct snd_timer_gparams32 {
	struct snd_timer_id tid;
	u32 period_num;
	u32 period_den;
	unsigned char reserved[32];
} __packed;

struct snd_timer_info32 {
	u32 flags;
	s32 card;
	unsigned char id[64];
	unsigned char name[80];
	u32 reserved0;
	u32 resolution;
	unsigned char reserved[64];
};
enum {
 RUST_DEFAULT_TIMER_LIMIT = IS_ENABLED(CONFIG_SND_HRTIMER) ? 4 : 1,
 RUST_TIMER_GFP_KERNEL = GFP_KERNEL,
 RUST_SNDRV_TIMER_VERSION = SNDRV_TIMER_VERSION,
 RUST_EPOLLIN = EPOLLIN,
 RUST_EPOLLRDNORM = EPOLLRDNORM,
 RUST_EPOLLERR = EPOLLERR,
 RUST_TIMER_SYSTEM_RESOLUTION = NSEC_PER_SEC / HZ,
 RUST_SNDRV_TIMER_IOCTL_STATUS32 = SNDRV_TIMER_IOCTL_STATUS32,
 RUST_SNDRV_TIMER_IOCTL_STATUS64 = SNDRV_TIMER_IOCTL_STATUS64,
 RUST_SNDRV_TIMER_IOCTL_START_OLD = _IO('T', 0x20),
 RUST_SNDRV_TIMER_IOCTL_STOP_OLD = _IO('T', 0x21),
 RUST_SNDRV_TIMER_IOCTL_CONTINUE_OLD = _IO('T', 0x22),
 RUST_SNDRV_TIMER_IOCTL_PAUSE_OLD = _IO('T', 0x23),
 RUST_SNDRV_TIMER_IOCTL_GPARAMS32 = _IOW('T', 0x04, struct snd_timer_gparams32),
 RUST_SNDRV_TIMER_IOCTL_INFO32 = _IOR('T', 0x11, struct snd_timer_info32),
 RUST_SNDRV_TIMER_IOCTL_STATUS_COMPAT32 = _IOW('T', 0x14, struct snd_timer_status32),
 RUST_SNDRV_TIMER_IOCTL_STATUS_COMPAT64 = _IOW('T', 0x14, struct snd_timer_status64),
 RUST_SNDRV_TIMER_IOCTL_PVERSION = SNDRV_TIMER_IOCTL_PVERSION,
 RUST_SNDRV_TIMER_IOCTL_NEXT_DEVICE = SNDRV_TIMER_IOCTL_NEXT_DEVICE,
 RUST_SNDRV_TIMER_IOCTL_TREAD_OLD = SNDRV_TIMER_IOCTL_TREAD_OLD,
 RUST_SNDRV_TIMER_IOCTL_GINFO = SNDRV_TIMER_IOCTL_GINFO,
 RUST_SNDRV_TIMER_IOCTL_GPARAMS = SNDRV_TIMER_IOCTL_GPARAMS,
 RUST_SNDRV_TIMER_IOCTL_GSTATUS = SNDRV_TIMER_IOCTL_GSTATUS,
 RUST_SNDRV_TIMER_IOCTL_SELECT = SNDRV_TIMER_IOCTL_SELECT,
 RUST_SNDRV_TIMER_IOCTL_INFO = SNDRV_TIMER_IOCTL_INFO,
 RUST_SNDRV_TIMER_IOCTL_PARAMS = SNDRV_TIMER_IOCTL_PARAMS,
 RUST_SNDRV_TIMER_IOCTL_START = SNDRV_TIMER_IOCTL_START,
 RUST_SNDRV_TIMER_IOCTL_STOP = SNDRV_TIMER_IOCTL_STOP,
 RUST_SNDRV_TIMER_IOCTL_CONTINUE = SNDRV_TIMER_IOCTL_CONTINUE,
 RUST_SNDRV_TIMER_IOCTL_PAUSE = SNDRV_TIMER_IOCTL_PAUSE,
 RUST_SNDRV_TIMER_IOCTL_TREAD64 = SNDRV_TIMER_IOCTL_TREAD64,
 RUST_SNDRV_TIMER_IOCTL_CREATE = SNDRV_TIMER_IOCTL_CREATE,
 RUST_SNDRV_TIMER_IOCTL_TRIGGER = SNDRV_TIMER_IOCTL_TRIGGER,
};
extern int rust_snd_timer_limit, rust_snd_timer_tstamp_monotonic;
struct list_head *rust_timer_list(void);
struct list_head *rust_timer_slave_list(void);
struct list_head *rust_timer_master_list(void);
struct mutex *rust_timer_register_mutex(void);
struct rwlock *rust_timer_instance_lock(void);
void rust_timer_mutex_lock(struct mutex *p);
void rust_timer_mutex_unlock(struct mutex *p);
void rust_timer_spin_lock(spinlock_t *p);
void rust_timer_spin_unlock(spinlock_t *p);
void rust_timer_spin_lock_irq(spinlock_t *p);
void rust_timer_spin_unlock_irq(spinlock_t *p);
unsigned long rust_timer_spin_lock_irqsave(spinlock_t *p);
void rust_timer_spin_unlock_irqrestore(spinlock_t *p, unsigned long flags);
unsigned long rust_timer_read_lock_irqsave(rwlock_t *p);
void rust_timer_read_unlock_irqrestore(rwlock_t *p, unsigned long flags);
void rust_timer_write_lock_irq(rwlock_t *p);
void rust_timer_write_unlock_irq(rwlock_t *p);
void rust_timer_spin_init(spinlock_t *p);
void rust_timer_mutex_init(struct mutex *p);
void rust_timer_list_init(struct list_head *p);
void rust_timer_list_del(struct list_head *p);
void rust_timer_list_del_init(struct list_head *p);
void rust_timer_list_add_tail(struct list_head *p, struct list_head *h);
void rust_timer_list_move_tail(struct list_head *p, struct list_head *h);
void *rust_timer_zalloc(size_t size);
void *rust_timer_zalloc_array(size_t n, size_t size);
char *rust_timer_strdup(const char *p);
void rust_timer_ref_init(struct kref *p);
void rust_timer_ref_get(struct kref *p);
void rust_timer_ref_put(struct kref *p, void (*release)(struct kref *));
int rust_timer_card_number(struct snd_card *p);
bool rust_timer_card_shutdown(struct snd_card *p);
struct device *rust_timer_card_device(struct snd_card *p);
struct module *rust_timer_card_module(struct snd_card *p);
struct module *rust_timer_this_module(void);
bool rust_timer_try_module_get(struct module *p);
void rust_timer_module_put(struct module *p);
void rust_timer_request_global(int n);
void rust_timer_request_card(int n);
int rust_timer_ecards_limit(void);
void rust_timer_work_init(struct work_struct *p, work_func_t fn);
void rust_timer_queue_work(struct work_struct *p);
void rust_timer_setup(struct timer_list *p, void (*fn)(struct timer_list *));
unsigned long rust_timer_jiffies(void);
void rust_timer_udelay(unsigned long n);
bool rust_timer_bug_on(bool condition);
bool rust_timer_warn_on(bool condition);
void rust_timer_warn_corrupt(void);
void rust_timer_invalid_slave(int slave_class);
int rust_timer_device_new(struct snd_card *card, void *data, const struct snd_device_ops *ops);
void *rust_timer_file_private(struct file *p);
void rust_timer_file_set_private(struct file *p, void *data);
unsigned int rust_timer_file_flags(struct file *p);
int rust_timer_pid(void);
unsigned long rust_timer_copy_to_user(void __user *dst, const void *src, unsigned long n);
unsigned long rust_timer_copy_from_user(void *dst, const void __user *src, unsigned long n);
int rust_timer_get_int(const int __user *p, int *v);
int rust_timer_put_int(int __user *p, int v);
int rust_timer_get_u32(const u32 __user *p, unsigned long *v);
void rust_timer_wait_head_init(wait_queue_head_t *p);
void rust_timer_wait_init(wait_queue_entry_t *p);
void rust_timer_current_interruptible(void);
bool rust_timer_signal_pending(void);
void rust_timer_wake_up(wait_queue_head_t *p);
void rust_timer_poll_wait(struct file *f, wait_queue_head_t *p, poll_table *pt);
void *rust_timer_compat_ptr(unsigned long p);
void rust_timer_info_set_read(struct snd_info_entry *p, void (*read)(struct snd_info_entry *, struct snd_info_buffer *));
void rust_timer_oss_register(void);
void rust_timer_oss_unregister(void);
#ifdef CONFIG_SND_UTIMER
int rust_timer_take_id(unsigned int max);
void rust_timer_put_id(unsigned int id);
#endif
int rust_alsa_timer_init(void);
void rust_alsa_timer_exit(void);
#endif
