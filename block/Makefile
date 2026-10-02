# SPDX-License-Identifier: GPL-2.0
#
# Makefile for the kernel block layer
#

CONTEXT_ANALYSIS := y

obj-y		:= bdev.o fops.o bio.o elevator.o blk-core.o blk-sysfs.o \
			blk-flush.o blk-settings.o blk-ioc.o blk-map.o \
			blk-merge.o blk-timeout.o blk-lib.o blk-mq.o \
			blk-mq-tag.o blk-mq-dma.o blk-stat.o \
			blk-mq-sysfs.o blk-mq-cpumap.o blk-mq-sched.o ioctl.o \
			genhd.o ioprio.o badblocks.o partitions/ blk-rq-qos.o \
			disk-events.o blk-ia-ranges.o early-lookup.o

ifeq ($(CONFIG_RUST_BLK_MQ_SCHED),y)
# Preserve the original archive slot and keep blk-mq-sched.c as the default.
obj-y += blk-mq-sched-rust.o
$(objtree)/rust/bindings/blk_mq_sched_generated.rs: FORCE
	+$(Q)$(MAKE) $(build)=rust rust/bindings/blk_mq_sched_generated.rs
$(obj)/blk-mq-sched.o: $(src)/blk-mq-sched_rust.rs $(objtree)/rust/bindings/blk_mq_sched_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/blk-mq-sched.s: $(src)/blk-mq-sched_rust.rs $(objtree)/rust/bindings/blk_mq_sched_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/blk-mq-sched.ll: $(src)/blk-mq-sched_rust.rs $(objtree)/rust/bindings/blk_mq_sched_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif

obj-$(CONFIG_BLK_ERROR_INJECTION) += error-injection.o
obj-$(CONFIG_BLK_DEV_BSG_COMMON) += bsg.o
obj-$(CONFIG_BLK_DEV_BSGLIB)	+= bsg-lib.o
obj-$(CONFIG_BLK_CGROUP)	+= blk-cgroup.o
obj-$(CONFIG_BLK_CGROUP_RWSTAT)	+= blk-cgroup-rwstat.o
obj-$(CONFIG_BLK_CGROUP_FC_APPID) += blk-cgroup-fc-appid.o
obj-$(CONFIG_BLK_DEV_THROTTLING)	+= blk-throttle.o
obj-$(CONFIG_BLK_CGROUP_IOPRIO)	+= blk-ioprio.o
obj-$(CONFIG_BLK_CGROUP_IOLATENCY)	+= blk-iolatency.o
obj-$(CONFIG_BLK_CGROUP_IOCOST)	+= blk-iocost.o
obj-$(CONFIG_MQ_IOSCHED_DEADLINE)	+= mq-deadline.o
obj-$(CONFIG_MQ_IOSCHED_KYBER)	+= kyber-iosched.o
bfq-y				:= bfq-iosched.o bfq-wf2q.o bfq-cgroup.o
obj-$(CONFIG_IOSCHED_BFQ)	+= bfq.o

obj-$(CONFIG_BLK_DEV_INTEGRITY) += bio-integrity.o blk-integrity.o t10-pi.o \
				   bio-integrity-auto.o bio-integrity-fs.o
obj-$(CONFIG_BLK_DEV_ZONED)	+= blk-zoned.o
obj-$(CONFIG_BLK_WBT)		+= blk-wbt.o
obj-$(CONFIG_BLK_DEBUG_FS)	+= blk-mq-debugfs.o
obj-$(CONFIG_BLK_SED_OPAL)	+= sed-opal.o
obj-$(CONFIG_BLK_PM)		+= blk-pm.o
obj-$(CONFIG_BLK_INLINE_ENCRYPTION)	+= blk-crypto.o blk-crypto-profile.o \
					   blk-crypto-sysfs.o
obj-$(CONFIG_BLK_INLINE_ENCRYPTION_FALLBACK)	+= blk-crypto-fallback.o

ifeq ($(CONFIG_RUST_BLK_CRYPTO_FALLBACK),y)
# Keep the original archive slot; retained C remains the default provider.
obj-y += blk-crypto-fallback-helpers.o
$(objtree)/rust/bindings/blk_crypto_fallback_generated.rs: FORCE
	+$(Q)$(MAKE) $(build)=rust rust/bindings/blk_crypto_fallback_generated.rs
$(obj)/blk-crypto-fallback.o: $(src)/blk-crypto-fallback.rs $(objtree)/rust/bindings/blk_crypto_fallback_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/blk-crypto-fallback.s: $(src)/blk-crypto-fallback.rs $(objtree)/rust/bindings/blk_crypto_fallback_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/blk-crypto-fallback.ll: $(src)/blk-crypto-fallback.rs $(objtree)/rust/bindings/blk_crypto_fallback_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif
obj-$(CONFIG_BLOCK_HOLDER_DEPRECATED)	+= holder.o
