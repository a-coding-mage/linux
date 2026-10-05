# SPDX-License-Identifier: GPL-2.0
# Source registration only. Select no objects and exclude no original owner.
ifeq ($(CONFIG_RUST_TASK_WORK),y)
$(error SOURCE ONLY HOLD: Rust task-work ownership and native behavior are unqualified)
endif

# Future admission must explicitly preserve kernel/task_work.o's complete native
# compiler/instrumentation policy and avoid duplicate public owners or storage.
# The native companion retains primitive/storage/signature work, not the old
# task-work algorithm. Removing this error is not ABI or runtime qualification.
