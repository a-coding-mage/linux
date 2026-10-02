# SPDX-License-Identifier: GPL-2.0
#
# Makefile for the linux ipc.
#

obj-$(CONFIG_SYSVIPC_COMPAT) += compat.o
obj-$(CONFIG_SYSVIPC) += util.o msgutil.o msg.o sem.o shm.o syscall.o
obj-$(CONFIG_SYSVIPC_SYSCTL) += ipc_sysctl.o
obj-$(CONFIG_POSIX_MQUEUE) += mqueue.o msgutil.o
obj-$(CONFIG_IPC_NS) += namespace.o
obj-$(CONFIG_POSIX_MQUEUE_SYSCTL) += mq_sysctl.o


ifeq ($(CONFIG_RUST_SYSVIPC_MSG),y)
# Keep msg.o's original archive position, selecting the Rust implementation.
obj-y += msg_helpers.o
RUSTFLAGS_msg.o += -Zfunction-sections=n
$(obj)/msg.o: $(src)/msg.rs $(objtree)/rust/bindings/ipc_msg_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif
