# SPDX-License-Identifier: GPL-2.0-only
ifeq ($(CONFIG_RUST_DIV64),y)
obj-y += div64_rust.o
# Only this native owner needs weak linkage for the original __div64_32 ABI.
# PowerPC's export-record assembly is still experimental in Rust 1.85.
RUST_ALLOWED_FEATURES_div64_rust.o := linkage$(if $(CONFIG_PPC),$(comma)asm_experimental_arch)
else
obj-y += div64.o
endif
ifeq ($(CONFIG_RUST_GCD_LCM),y)
obj-y += gcd_lcm_rust.o
else
obj-y += gcd.o lcm.o
endif
ifeq ($(CONFIG_RUST_INT_LOG),y)
obj-y += int_log_rust.o
else
obj-y += int_log.o
endif
ifeq ($(CONFIG_RUST_INT_MATH),y)
obj-y += int_math_rust.o
else
obj-y += int_pow.o int_sqrt.o
endif
ifeq ($(CONFIG_RUST_RECIPROCAL_DIV),y)
obj-y += reciprocal_div_rust.o
else
obj-y += reciprocal_div.o
endif

obj-$(CONFIG_CORDIC)		+= cordic.o
ifeq ($(CONFIG_RUST_CORDIC),y)
# Keep the original tristate selection and cordic.ko module identity.
cordic-y := cordic_rust.o
RUST_ALLOWED_FEATURES_cordic_rust.o := $(if $(CONFIG_PPC),asm_experimental_arch)
endif
obj-$(CONFIG_POLYNOMIAL)	+= polynomial.o
ifeq ($(CONFIG_RUST_POLYNOMIAL),y)
# Preserve the original tristate selection and polynomial.ko module identity.
polynomial-y := polynomial_rust.o
RUST_ALLOWED_FEATURES_polynomial_rust.o := $(if $(CONFIG_PPC),asm_experimental_arch)
endif
obj-$(CONFIG_PRIME_NUMBERS)	+= prime_numbers.o
ifeq ($(CONFIG_RUST_PRIME_NUMBERS),y)
# Select the existing translation, retaining prime_numbers.ko and the C choice.
RUST_ALLOWED_FEATURES_prime_numbers.o := $(if $(CONFIG_PPC),asm_experimental_arch)
$(obj)/prime_numbers.o: $(src)/prime_numbers.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)



# Explicit targets have no implicit stem; keep the canonical owner identity.
$(addprefix $(obj)/,prime_numbers.o prime_numbers.s prime_numbers.ll prime_numbers.rsi .rust-listing/prime_numbers.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))

# Inspection uses the selected source and its canonical per-object policy.
$(addprefix $(obj)/,prime_numbers.s prime_numbers.ll prime_numbers.rsi): private target-stem := prime_numbers
$(addprefix $(obj)/,prime_numbers.s prime_numbers.ll prime_numbers.rsi): private part-of-builtin = $(if $(filter $(obj)/prime_numbers.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,prime_numbers.s prime_numbers.ll prime_numbers.rsi): private part-of-module = $(if $(filter $(obj)/prime_numbers.o,$(real-obj-m)),y)
$(obj)/prime_numbers.s: $(src)/prime_numbers.rs $(src)/prime_numbers_mutex.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/prime_numbers.ll: $(src)/prime_numbers.rs $(src)/prime_numbers_mutex.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/prime_numbers.rsi: $(src)/prime_numbers.rs $(src)/prime_numbers_mutex.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Debug information belongs only to this unlinked listing object.
$(obj)/.rust-listing/prime_numbers.o: private target-stem := prime_numbers
$(obj)/.rust-listing/prime_numbers.o: private part-of-builtin = $(if $(filter $(obj)/prime_numbers.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/prime_numbers.o: private part-of-module = $(if $(filter $(obj)/prime_numbers.o,$(real-obj-m)),y)
$(obj)/.rust-listing/prime_numbers.o: private is-single-obj-m = $(and $(part-of-module),$(filter $(obj)/prime_numbers.o,$(obj-m)),y)
$(obj)/.rust-listing/prime_numbers.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/prime_numbers.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/prime_numbers.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/prime_numbers.o: $(src)/prime_numbers.rs $(src)/prime_numbers_mutex.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_prime_numbers_rust_lst_elf = LD      $@
      cmd_prime_numbers_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/prime_numbers.o: $(obj)/.rust-listing/prime_numbers.o FORCE
	$(call if_changed,prime_numbers_rust_lst_elf)
# Keep makelst's real System.map behavior, including creation and deletion.
quiet_cmd_prime_numbers_rust_lst = MKLST   $@
      cmd_prime_numbers_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/prime_numbers.lst: $(obj)/.rust-listing-elf/prime_numbers.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,prime_numbers_rust_lst)
targets += prime_numbers.s prime_numbers.ll prime_numbers.rsi prime_numbers.lst \
	.rust-listing/prime_numbers.o .rust-listing-elf/prime_numbers.o
endif
obj-$(CONFIG_RATIONAL)		+= rational.o
ifeq ($(CONFIG_RUST_RATIONAL),y)
# Preserve the original built-in/module selection and rational.ko name.
rational-y := rational_rust.o
endif

obj-$(CONFIG_TEST_DIV64)	+= test_div64.o
obj-$(CONFIG_TEST_MULDIV64)	+= test_mul_u64_u64_div_u64.o

# Explicit rules select the existing translations despite their C counterparts
# having the same basename, including assembly/LLVM inspection targets.
# Preserve the original module names and C choice.
ifeq ($(CONFIG_RUST_DIV64_TESTS),y)
$(obj)/test_div64.o: $(src)/test_div64.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

$(obj)/test_div64.s: $(src)/test_div64.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)

$(obj)/test_div64.ll: $(src)/test_div64.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)

$(obj)/test_mul_u64_u64_div_u64.o: $(src)/test_mul_u64_u64_div_u64.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

$(obj)/test_mul_u64_u64_div_u64.s: $(src)/test_mul_u64_u64_div_u64.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)

$(obj)/test_mul_u64_u64_div_u64.ll: $(src)/test_mul_u64_u64_div_u64.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
endif

obj-y += tests/

# Private inspection sidecars remain cleanable after deselection.
clean-files += .rust-listing/ .rust-listing-elf/
