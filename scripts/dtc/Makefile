# SPDX-License-Identifier: GPL-2.0
# scripts/dtc makefile

# *** Also keep .gitignore in sync when changing ***
hostprogs-always-$(CONFIG_DTC)		+= dtc fdtoverlay
hostprogs-always-$(CHECK_DTBS)		+= dtc

dtc-rust	:= y
fdtoverlay-rust	:= y
fdtget-rust	:= y
fdtput-rust	:= y

# The query/edit applets are explicit host targets, not prerequisites of every
# device-tree build. All three import the same checked byte-slice libfdt modules.
hostprogs += fdtget fdtput
