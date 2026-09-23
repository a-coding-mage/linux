# SPDX-License-Identifier: GPL-2.0
hostprogs-always-y	:= polgen
ifneq ($(HOST_TOOLS_LANG),c)
polgen-rust		:= y
else
HOST_EXTRACFLAGS += \
	-I$(srctree)/include \
	-I$(srctree)/include/uapi
endif
