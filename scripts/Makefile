# SPDX-License-Identifier: GPL-2.0
###
# scripts contains sources for various helper programs used throughout
# the kernel for the build process.

hostprogs-always-$(CONFIG_KALLSYMS)			+= kallsyms
hostprogs-always-$(BUILD_C_RECORDMCOUNT)		+= recordmcount
hostprogs-always-$(CONFIG_BUILDTIME_TABLE_SORT)		+= sorttable
hostprogs-always-$(CONFIG_ASN1)				+= asn1_compiler
hostprogs-always-$(CONFIG_MODULE_SIG_FORMAT)		+= sign-file
hostprogs-always-$(CONFIG_SYSTEM_EXTRA_CERTIFICATE)	+= insert-sys-cert
hostprogs-always-$(CONFIG_RUST_KERNEL_DOCTESTS)		+= rustdoc_test_builder
hostprogs-always-$(CONFIG_RUST_KERNEL_DOCTESTS)		+= rustdoc_test_gen
hostprogs-always-$(CONFIG_TRACEPOINTS)			+= tracepoint-update

sorttable-rust := y
tracepoint-update-rust := y

ifneq ($(or $(CONFIG_X86_64),$(CONFIG_X86_32)),)
always-$(CONFIG_RUST)					+= target.json
filechk_rust_target = $< < include/config/auto.conf

$(obj)/target.json: scripts/generate_rust_target include/config/auto.conf FORCE
	$(call filechk,rust_target)
endif

hostprogs += generate_rust_target
generate_rust_target-rust := y
kallsyms-rust := y
gen_packed_field_checks-rust := y
asn1_compiler-rust := y
unifdef-rust := y
insert-sys-cert-rust := y
sign-file-rust := y
recordmcount-rust := y
rustdoc_test_builder-rust := y
rustdoc_test_gen-rust := y

HOSTRUSTFLAGS_sorttable += --check-cfg='cfg(UNWINDER_ORC_ENABLED)' \
			  --check-cfg='cfg(MCOUNT_SORT_ENABLED)'
include $(srctree)/scripts/Makefile.openssl
HOSTRUSTFLAGS_sign-file += $(openssl-host-rustflags)
$(obj)/sign-file: $(srctree)/scripts/openssl_config.py

ifdef CONFIG_UNWINDER_ORC
HOSTRUSTFLAGS_sorttable += --cfg UNWINDER_ORC_ENABLED
endif

ifdef CONFIG_BUILDTIME_MCOUNT_SORT
HOSTRUSTFLAGS_sorttable += --cfg MCOUNT_SORT_ENABLED
endif

# The following programs are only built on demand
hostprogs += unifdef gen_packed_field_checks

# The module linker script is preprocessed on demand
targets += module.lds

subdir-$(CONFIG_GCC_PLUGINS) += gcc-plugins
subdir-$(CONFIG_GENKSYMS) += genksyms
subdir-$(CONFIG_GENDWARFKSYMS) += gendwarfksyms
subdir-$(CONFIG_SECURITY_SELINUX) += selinux
subdir-$(CONFIG_SECURITY_IPE) += ipe

# Let clean descend into subdirs
subdir-	+= basic dtc gdb kconfig mod
