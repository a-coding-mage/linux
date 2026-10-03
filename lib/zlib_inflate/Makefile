# SPDX-License-Identifier: GPL-2.0-only
#
# This is a modified version of zlib, which does all memory
# allocation ahead of time.
#
# This is only the decompression, see zlib_deflate for the
# the compression
#
# Decompression needs to be serialized for each memory
# allocation.
#
# (The upsides of the simplification is that you can't get in
# any nasty situations wrt memory management, and that the
# uncompression can be done without blocking on allocation).
#

obj-$(CONFIG_ZLIB_INFLATE) += zlib_inflate.o

ifeq ($(CONFIG_RUST_ZLIB_INFLATE),y)
# The Rust crate owns every codec body, export record and module metadata.
# The C member contains allocation primitives only, with no zlib algorithm.
zlib_inflate-objs := zlib_inflate_rust.o kernel_alloc.o
RUST_ALLOWED_FEATURES_zlib_inflate_rust.o := $(if $(CONFIG_PPC),asm_experimental_arch)
$(obj)/zlib_inflate_rust.o: $(objtree)/rust/bindings/zlib_inflate_generated.rs
else
# Retained unchanged as the original C control provider.
zlib_inflate-objs := inffast.o inflate.o infutil.o \
		     inftrees.o inflate_syms.o
endif
