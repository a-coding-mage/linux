# SPDX-License-Identifier: GPL-2.0
#
# Makefile for asymmetric cryptographic keys
#

obj-$(CONFIG_ASYMMETRIC_KEY_TYPE) += asymmetric_keys.o

asymmetric_keys-y := \
	asymmetric_type.o \
	restrict.o \
	signature.o

obj-$(CONFIG_ASYMMETRIC_PUBLIC_KEY_SUBTYPE) += public_key.o
ifeq ($(CONFIG_RUST_PUBLIC_KEY),y)
# Keep the original public_key module and built-in archive position.
public_key-y := public_key_rust.o public_key_helpers.o
RUSTFLAGS_public_key_rust.o += -Zfunction-sections=n
$(obj)/public_key_rust.o: $(src)/public_key.rs $(objtree)/rust/bindings/public_key_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Explicit targets have no implicit stem; keep the canonical owner identity.
$(addprefix $(obj)/,public_key_rust.o public_key.s public_key.ll public_key.rsi public_key_rust.s public_key_rust.ll public_key_rust.rsi .rust-listing/public_key_rust.o): private modname-multi = \
    $(sort $(foreach m,$(multi-obj-ym),\
        $(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))

# Inspection uses the selected source and its canonical per-object policy.
$(addprefix $(obj)/,public_key.s public_key.ll public_key.rsi public_key_rust.s public_key_rust.ll public_key_rust.rsi): private target-stem := public_key_rust
$(addprefix $(obj)/,public_key.s public_key.ll public_key.rsi public_key_rust.s public_key_rust.ll public_key_rust.rsi): private part-of-builtin = $(if $(filter $(obj)/public_key_rust.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,public_key.s public_key.ll public_key.rsi public_key_rust.s public_key_rust.ll public_key_rust.rsi): private part-of-module = $(if $(filter $(obj)/public_key_rust.o,$(real-obj-m)),y)
$(obj)/public_key.s $(obj)/public_key_rust.s: $(src)/public_key.rs $(objtree)/rust/bindings/public_key_generated.rs FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/public_key.ll $(obj)/public_key_rust.ll: $(src)/public_key.rs $(objtree)/rust/bindings/public_key_generated.rs FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/public_key.rsi $(obj)/public_key_rust.rsi: $(src)/public_key.rs $(objtree)/rust/bindings/public_key_generated.rs FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Debug information belongs only to this unlinked listing object.
$(obj)/.rust-listing/public_key_rust.o: private target-stem := public_key_rust
$(obj)/.rust-listing/public_key_rust.o: private part-of-builtin = $(if $(filter $(obj)/public_key_rust.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/public_key_rust.o: private part-of-module = $(if $(filter $(obj)/public_key_rust.o,$(real-obj-m)),y)
$(obj)/.rust-listing/public_key_rust.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/public_key_rust.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/public_key_rust.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/public_key_rust.o: $(src)/public_key.rs $(objtree)/rust/bindings/public_key_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_public_key_rust_lst_elf = LD      $@
      cmd_public_key_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/public_key_rust.o: $(obj)/.rust-listing/public_key_rust.o FORCE
	$(call if_changed,public_key_rust_lst_elf)
# Keep makelst's real System.map behavior, including creation and deletion.
quiet_cmd_public_key_rust_lst = MKLST   $@
      cmd_public_key_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/public_key.lst $(obj)/public_key_rust.lst: $(obj)/.rust-listing-elf/public_key_rust.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,public_key_rust_lst)
targets += public_key.s public_key.ll public_key.rsi public_key_rust.s public_key_rust.ll public_key_rust.rsi public_key.lst public_key_rust.lst \
	.rust-listing/public_key_rust.o .rust-listing-elf/public_key_rust.o
endif


#
# X.509 Certificate handling
#
obj-$(CONFIG_X509_CERTIFICATE_PARSER) += x509_key_parser.o
x509_key_parser-y := \
	x509.asn1.o \
	x509_akid.asn1.o \
	x509_cert_parser.o \
	x509_loader.o \
	x509_public_key.o
obj-$(CONFIG_FIPS_SIGNATURE_SELFTEST) += x509_selftest.o
x509_selftest-y += selftest.o
x509_selftest-$(CONFIG_FIPS_SIGNATURE_SELFTEST_RSA) += selftest_rsa.o
x509_selftest-$(CONFIG_FIPS_SIGNATURE_SELFTEST_ECDSA) += selftest_ecdsa.o

$(obj)/x509_cert_parser.o: \
	$(obj)/x509.asn1.h \
	$(obj)/x509_akid.asn1.h

$(obj)/x509.asn1.o: $(obj)/x509.asn1.c $(obj)/x509.asn1.h
$(obj)/x509_akid.asn1.o: $(obj)/x509_akid.asn1.c $(obj)/x509_akid.asn1.h

#
# PKCS#8 private key handling
#
obj-$(CONFIG_PKCS8_PRIVATE_KEY_PARSER) += pkcs8_key_parser.o
pkcs8_key_parser-y := \
	pkcs8.asn1.o \
	pkcs8_parser.o

$(obj)/pkcs8_parser.o: $(obj)/pkcs8.asn1.h
$(obj)/pkcs8-asn1.o: $(obj)/pkcs8.asn1.c $(obj)/pkcs8.asn1.h

clean-files	+= pkcs8.asn1.c pkcs8.asn1.h

#
# PKCS#7 message handling
#
obj-$(CONFIG_PKCS7_MESSAGE_PARSER) += pkcs7_message.o
pkcs7_message-y := \
	pkcs7.asn1.o \
	pkcs7_parser.o \
	pkcs7_trust.o \
	pkcs7_verify.o

$(obj)/pkcs7_parser.o: $(obj)/pkcs7.asn1.h
$(obj)/pkcs7.asn1.o: $(obj)/pkcs7.asn1.c $(obj)/pkcs7.asn1.h

#
# PKCS#7 parser testing key
#
obj-$(CONFIG_PKCS7_TEST_KEY) += pkcs7_test_key.o
pkcs7_test_key-y := \
	pkcs7_key_type.o

#
# Signed PE binary-wrapped key handling
#
obj-$(CONFIG_SIGNED_PE_FILE_VERIFICATION) += verify_signed_pefile.o

verify_signed_pefile-y := \
	verify_pefile.o \
	mscode_parser.o \
	mscode.asn1.o

$(obj)/mscode_parser.o: $(obj)/mscode.asn1.h $(obj)/mscode.asn1.h
$(obj)/mscode.asn1.o: $(obj)/mscode.asn1.c $(obj)/mscode.asn1.h

# Private inspection sidecars remain cleanable after deselection.
clean-files += .rust-listing/ .rust-listing-elf/
