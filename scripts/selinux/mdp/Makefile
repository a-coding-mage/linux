# SPDX-License-Identifier: GPL-2.0
hostprogs-always-y += mdp
mdp-rust := y

# Match C #ifdef CONFIG_*: only built-in (y) symbols are defined, not modules.
# These flags also work before a kernel configuration has been generated.
mdp-configs := EXT2_FS_SECURITY EXT4_FS_SECURITY EXT4_USE_FOR_EXT2 \
	JFS_SECURITY JFFS2_FS_SECURITY XFS_FS GFS2_FS BTRFS_FS \
	F2FS_FS_SECURITY OCFS2_FS OVERLAY_FS SQUASHFS_XATTR UNIX98_PTYS \
	HUGETLBFS TMPFS DEVTMPFS POSIX_MQUEUE PROC_FS SECURITY_SELINUX \
	SYSFS DEBUG_FS TRACING PSTORE
HOSTRUSTFLAGS_mdp += $(foreach config,$(mdp-configs),\
	$(if $(filter y,$(CONFIG_$(config))),--cfg CONFIG_$(config)))

clean-files	:= policy.* file_contexts
