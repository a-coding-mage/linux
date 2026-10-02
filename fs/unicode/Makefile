# SPDX-License-Identifier: GPL-2.0

ifneq ($(CONFIG_UNICODE),)
obj-y			+= unicode.o
endif
obj-$(CONFIG_UNICODE)	+= utf8data.o
obj-$(CONFIG_UNICODE_NORMALIZATION_KUNIT_TEST) += tests/utf8_kunit.o

unicode-y := utf8-norm.o utf8-core.o

ifeq ($(CONFIG_RUST_UNICODE_CORE),y)
# Keep unicode.o built in even when only the Unicode data is a module.
# The original normalization engine and data retain their C owners.
unicode-y += utf8-core-helpers.o
$(obj)/utf8-core.o: $(src)/utf8-core.rs $(objtree)/rust/bindings/utf8_core_generated.rs FORCE
	+$(call if_changed_rule,rustc_o_rs)
endif

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
