# SPDX-License-Identifier: GPL-2.0

ifneq ($(CONFIG_UNICODE),)
obj-y			+= unicode.o
endif
obj-$(CONFIG_UNICODE)	+= utf8data.o
obj-$(CONFIG_UNICODE_NORMALIZATION_KUNIT_TEST) += tests/utf8_kunit.o

unicode-y := utf8-norm.o utf8-core.o

# These directories contain only private listing products and their sidecars.
# Clean them even after either Rust selector has been disabled.
clean-files += .rust-listing/ .rust-listing-elf/

ifneq ($(filter y,$(CONFIG_RUST_UNICODE_CORE) $(CONFIG_RUST_UNICODE_NORM)),)
# Match the original C implicit-rule membership calculation for explicit Rust
# targets, whose $* is empty. Private listing targets restore target-stem below.
unicode-rust-modname-multi = $(sort $(foreach m,$(multi-obj-ym),\
	$(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))

# The canonical private object may be bitcode with inline helpers and LTO.
# Materialize a separate ELF with the original relocatable-link semantics;
# neither the canonical object nor its objtool decision is changed.
quiet_cmd_unicode_rust_lst_elf = LD      $@
      cmd_unicode_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<

# Record map presence in the command so deletion also invalidates the listing.
# Always pass the original path: makelst owns the missing-map warning/behavior.
quiet_cmd_unicode_rust_lst = MKLST   $@
      cmd_unicode_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
endif

ifeq ($(CONFIG_RUST_UNICODE_CORE),y)
# Keep unicode.o built in even when only the Unicode data is a module.
# Normalization is selected independently below; data retains its C owner.
unicode-y += utf8-core-helpers.o
$(addprefix $(obj)/,utf8-core.o utf8-core.s utf8-core.ll utf8-core.rsi \
	.rust-listing/utf8-core.o): private modname-multi = $(unicode-rust-modname-multi)
$(obj)/utf8-core.o: $(src)/utf8-core.rs $(objtree)/rust/bindings/utf8_core_generated.rs \
	$(wildcard $(objtree)/include/config/RUST_UNICODE_CORE) FORCE
	+$(call if_changed_rule,rustc_o_rs)
# Keep all Rust source inspection on the selected source and configured types.
$(obj)/utf8-core.s: $(src)/utf8-core.rs $(objtree)/rust/bindings/utf8_core_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/utf8-core.ll: $(src)/utf8-core.rs $(objtree)/rust/bindings/utf8_core_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/utf8-core.rsi: $(src)/utf8-core.rs $(objtree)/rust/bindings/utf8_core_generated.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Listing-only debug information must never alter the canonical .o or .cmd.
# Retain its source's per-object flags and original built-in/module membership.
$(obj)/.rust-listing/utf8-core.o: private target-stem := utf8-core
$(obj)/.rust-listing/utf8-core.o: private part-of-builtin = $(if $(filter $(obj)/utf8-core.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/utf8-core.o: private part-of-module = $(if $(filter $(obj)/utf8-core.o,$(real-obj-m)),y)
$(obj)/.rust-listing/utf8-core.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/utf8-core.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/utf8-core.o: $(src)/utf8-core.rs $(objtree)/rust/bindings/utf8_core_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/.rust-listing-elf/utf8-core.o: $(obj)/.rust-listing/utf8-core.o FORCE
	$(call if_changed,unicode_rust_lst_elf)
$(obj)/utf8-core.lst: $(obj)/.rust-listing-elf/utf8-core.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,unicode_rust_lst)
targets += utf8-core.s utf8-core.ll utf8-core.rsi utf8-core.lst \
	.rust-listing/utf8-core.o .rust-listing-elf/utf8-core.o
endif

ifeq ($(CONFIG_RUST_UNICODE_NORM),y)
# Rust owns the existing normalization slot; there is no C engine fallback.
$(addprefix $(obj)/,utf8-norm.o utf8-norm.s utf8-norm.ll utf8-norm.rsi \
	.rust-listing/utf8-norm.o): private modname-multi = $(unicode-rust-modname-multi)
$(obj)/utf8-norm.o: $(src)/utf8-norm.rs $(objtree)/rust/bindings/utf8_norm_generated.rs \
	$(wildcard $(objtree)/include/config/RUST_UNICODE_NORM) FORCE
	+$(call if_changed_rule,rustc_o_rs)
# Keep all Rust source inspection on the selected source and configured types.
$(obj)/utf8-norm.s: $(src)/utf8-norm.rs $(objtree)/rust/bindings/utf8_norm_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/utf8-norm.ll: $(src)/utf8-norm.rs $(objtree)/rust/bindings/utf8_norm_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/utf8-norm.rsi: $(src)/utf8-norm.rs $(objtree)/rust/bindings/utf8_norm_generated.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Listing-only debug information must never alter the canonical .o or .cmd.
# Retain its source's per-object flags and original built-in/module membership.
$(obj)/.rust-listing/utf8-norm.o: private target-stem := utf8-norm
$(obj)/.rust-listing/utf8-norm.o: private part-of-builtin = $(if $(filter $(obj)/utf8-norm.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/utf8-norm.o: private part-of-module = $(if $(filter $(obj)/utf8-norm.o,$(real-obj-m)),y)
$(obj)/.rust-listing/utf8-norm.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/utf8-norm.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/utf8-norm.o: $(src)/utf8-norm.rs $(objtree)/rust/bindings/utf8_norm_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
$(obj)/.rust-listing-elf/utf8-norm.o: $(obj)/.rust-listing/utf8-norm.o FORCE
	$(call if_changed,unicode_rust_lst_elf)
$(obj)/utf8-norm.lst: $(obj)/.rust-listing-elf/utf8-norm.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,unicode_rust_lst)
targets += utf8-norm.s utf8-norm.ll utf8-norm.rsi utf8-norm.lst \
	.rust-listing/utf8-norm.o .rust-listing-elf/utf8-norm.o
endif
# With RUST_UNICODE_NORM off, the original C normalization recipe is retained.
# utf8data.o always supplies the unchanged original Unicode tables.


$(obj)/utf8-data.o: $(obj)/utf8data.c

# In the normal build, the checked-in utf8data.c is just shipped.
#
# To generate utf8data.c from UCD, put *.txt files in this directory
# and pass REGENERATE_UTF8DATA=1 from the command line.
ifdef REGENERATE_UTF8DATA

quiet_cmd_utf8data = GEN     $@
      cmd_utf8data = $< \
		-a $(src)/DerivedAge.txt \
		-c $(src)/DerivedCombiningClass.txt \
		-p $(src)/DerivedCoreProperties.txt \
		-d $(src)/UnicodeData.txt \
		-f $(src)/CaseFolding.txt \
		-n $(src)/NormalizationCorrections.txt \
		-t $(src)/NormalizationTest.txt \
		-o $@

$(obj)/utf8data.c: $(obj)/mkutf8data $(filter %.txt, $(cmd_utf8data)) FORCE
	$(call if_changed,utf8data)

else

$(obj)/utf8data.c: $(src)/utf8data.c_shipped FORCE
	$(call if_changed,copy)

endif

targets += utf8data.c
hostprogs += mkutf8data

ifneq ($(HOST_TOOLS_LANG),c)
mkutf8data-rust := y
endif
