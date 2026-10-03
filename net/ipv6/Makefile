# SPDX-License-Identifier: GPL-2.0
#
# Makefile for the Linux TCP/IP (INET6) layer.
#

obj-$(CONFIG_IPV6) += ipv6.o

ipv6-y :=	af_inet6.o anycast.o ip6_output.o ip6_input.o addrconf.o \
		addrlabel.o \
		route.o ip6_fib.o ipv6_sockglue.o ndisc.o udp.o \
		raw.o icmp.o mcast.o reassembly.o tcp_ipv6.o ping.o \
		exthdrs.o datagram.o ip6_flowlabel.o inet6_connection_sock.o \
		udp_offload.o seg6.o fib6_notifier.o rpl.o ioam6.o

ipv6-$(CONFIG_SYSCTL) += sysctl_net_ipv6.o
ipv6-$(CONFIG_IPV6_MROUTE) += ip6mr.o

ipv6-$(CONFIG_XFRM) += xfrm6_policy.o xfrm6_state.o xfrm6_input.o \
	xfrm6_output.o xfrm6_protocol.o
ipv6-$(CONFIG_NETFILTER) += netfilter.o
ipv6-$(CONFIG_IPV6_MULTIPLE_TABLES) += fib6_rules.o
ipv6-$(CONFIG_PROC_FS) += proc.o
ipv6-$(CONFIG_SYN_COOKIES) += syncookies.o
ipv6-$(CONFIG_NETLABEL) += calipso.o
ipv6-$(CONFIG_IPV6_SEG6_LWTUNNEL) += seg6_iptunnel.o seg6_local.o
ipv6-$(CONFIG_IPV6_SEG6_HMAC) += seg6_hmac.o
ipv6-$(CONFIG_IPV6_RPL_LWTUNNEL) += rpl_iptunnel.o
ipv6-$(CONFIG_IPV6_IOAM6_LWTUNNEL) += ioam6_iptunnel.o

obj-$(CONFIG_INET6_AH) += ah6.o
obj-$(CONFIG_INET6_ESP) += esp6.o
obj-$(CONFIG_INET6_ESP_OFFLOAD) += esp6_offload.o
obj-$(CONFIG_INET6_IPCOMP) += ipcomp6.o
obj-$(CONFIG_INET6_XFRM_TUNNEL) += xfrm6_tunnel.o
obj-$(CONFIG_INET6_TUNNEL) += tunnel6.o
obj-$(CONFIG_IPV6_MIP6) += mip6.o
obj-$(CONFIG_IPV6_ILA) += ila/
obj-$(CONFIG_NETFILTER)	+= netfilter/

obj-$(CONFIG_IPV6_VTI) += ip6_vti.o
obj-$(CONFIG_IPV6_SIT) += sit.o
obj-$(CONFIG_IPV6_TUNNEL) += ip6_tunnel.o
obj-$(CONFIG_IPV6_GRE) += ip6_gre.o
obj-$(CONFIG_IPV6_FOU) += fou6.o

obj-y += addrconf_core.o exthdrs_core.o ip6_checksum.o ip6_icmp.o
obj-$(CONFIG_INET) += output_core.o protocol.o \
			ip6_offload.o exthdrs_offload.o

obj-$(subst m,y,$(CONFIG_IPV6)) += inet6_hashtables.o

ifneq ($(CONFIG_IPV6),)
obj-$(CONFIG_NET_UDP_TUNNEL) += ip6_udp_tunnel.o
obj-y += mcast_snoop.o ndisc_snoop.o
obj-$(CONFIG_TCP_AO) += tcp_ao.o
endif

ifdef CONFIG_GCOV_PROFILE_NETFILTER
GCOV_PROFILE_netfilter.o := y
endif

clean-files += .rust-listing/ .rust-listing-elf/

ifeq ($(CONFIG_RUST_IPV6_SOCKGLUE),y)
ipv6-y += ipv6_sockglue_helpers.o
RUSTFLAGS_ipv6_sockglue.o += -Zfunction-sections=n
ipv6_sockglue-rust-deps := $(objtree)/rust/bindings/ipv6_sockglue_generated.rs
# Explicit Rust rules have no implicit-rule stem; retain composite membership.
ipv6_sockglue-rust-modname-multi = $(sort $(foreach m,$(multi-obj-ym),\
	$(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,ipv6_sockglue.o ipv6_sockglue.s ipv6_sockglue.ll ipv6_sockglue.rsi \
	.rust-listing/ipv6_sockglue.o): private modname-multi = $(ipv6_sockglue-rust-modname-multi)
$(obj)/ipv6_sockglue.o: $(src)/ipv6_sockglue.rs $(ipv6_sockglue-rust-deps) \
	$(wildcard $(objtree)/include/config/RUST_IPV6_SOCKGLUE) FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Keep inspection on the selected source, ahead of the generic C recipes.
$(obj)/ipv6_sockglue.s: $(src)/ipv6_sockglue.rs $(ipv6_sockglue-rust-deps) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/ipv6_sockglue.ll: $(src)/ipv6_sockglue.rs $(ipv6_sockglue-rust-deps) FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/ipv6_sockglue.rsi: $(src)/ipv6_sockglue.rs $(ipv6_sockglue-rust-deps) FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Listing debug flags and sidecars stay private; never replace the real owner.
$(obj)/.rust-listing/ipv6_sockglue.o: private target-stem := ipv6_sockglue
$(obj)/.rust-listing/ipv6_sockglue.o: private part-of-builtin = $(if $(filter $(obj)/ipv6_sockglue.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/ipv6_sockglue.o: private part-of-module = $(if $(filter $(obj)/ipv6_sockglue.o,$(real-obj-m)),y)
$(obj)/.rust-listing/ipv6_sockglue.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/ipv6_sockglue.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/ipv6_sockglue.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/ipv6_sockglue.o: $(src)/ipv6_sockglue.rs $(ipv6_sockglue-rust-deps) FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_ipv6_sockglue_rust_lst_elf = LD      $@
      cmd_ipv6_sockglue_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/ipv6_sockglue.o: $(obj)/.rust-listing/ipv6_sockglue.o FORCE
	$(call if_changed,ipv6_sockglue_rust_lst_elf)
quiet_cmd_ipv6_sockglue_rust_lst = MKLST   $@
      cmd_ipv6_sockglue_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/ipv6_sockglue.lst: $(obj)/.rust-listing-elf/ipv6_sockglue.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,ipv6_sockglue_rust_lst)
targets += ipv6_sockglue.s ipv6_sockglue.ll ipv6_sockglue.rsi ipv6_sockglue.lst \
	.rust-listing/ipv6_sockglue.o .rust-listing-elf/ipv6_sockglue.o
endif

clean-files += .rust-listing/ .rust-listing-elf/

ifeq ($(CONFIG_RUST_INET6_AH),y)
ah6-y := ah6_rust.o ah6_helpers.o
ah6-rust-deps := $(objtree)/rust/bindings/ah6_generated.rs
# Explicit Rust rules have no implicit-rule stem; retain composite membership.
ah6-rust-modname-multi = $(sort $(foreach m,$(multi-obj-ym),\
	$(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,ah6_rust.o ah6.s ah6.ll ah6.rsi ah6_rust.s ah6_rust.ll ah6_rust.rsi \
	.rust-listing/ah6_rust.o): private modname-multi = $(ah6-rust-modname-multi)
$(obj)/ah6_rust.o: $(src)/ah6.rs $(ah6-rust-deps) \
	$(wildcard $(objtree)/include/config/RUST_INET6_AH) FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Keep inspection on the selected source, ahead of the generic C recipes.
$(addprefix $(obj)/,ah6.s ah6.ll ah6.rsi ah6_rust.s ah6_rust.ll ah6_rust.rsi): private target-stem := ah6_rust
$(addprefix $(obj)/,ah6.s ah6.ll ah6.rsi ah6_rust.s ah6_rust.ll ah6_rust.rsi): private part-of-builtin = $(if $(filter $(obj)/ah6_rust.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,ah6.s ah6.ll ah6.rsi ah6_rust.s ah6_rust.ll ah6_rust.rsi): private part-of-module = $(if $(filter $(obj)/ah6_rust.o,$(real-obj-m)),y)
$(obj)/ah6.s $(obj)/ah6_rust.s: $(src)/ah6.rs $(ah6-rust-deps) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/ah6.ll $(obj)/ah6_rust.ll: $(src)/ah6.rs $(ah6-rust-deps) FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/ah6.rsi $(obj)/ah6_rust.rsi: $(src)/ah6.rs $(ah6-rust-deps) FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Listing debug flags and sidecars stay private; never replace the real owner.
$(obj)/.rust-listing/ah6_rust.o: private target-stem := ah6_rust
$(obj)/.rust-listing/ah6_rust.o: private part-of-builtin = $(if $(filter $(obj)/ah6_rust.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/ah6_rust.o: private part-of-module = $(if $(filter $(obj)/ah6_rust.o,$(real-obj-m)),y)
$(obj)/.rust-listing/ah6_rust.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/ah6_rust.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/ah6_rust.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/ah6_rust.o: $(src)/ah6.rs $(ah6-rust-deps) FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_ah6_rust_lst_elf = LD      $@
      cmd_ah6_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/ah6_rust.o: $(obj)/.rust-listing/ah6_rust.o FORCE
	$(call if_changed,ah6_rust_lst_elf)
quiet_cmd_ah6_rust_lst = MKLST   $@
      cmd_ah6_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/ah6.lst $(obj)/ah6_rust.lst: $(obj)/.rust-listing-elf/ah6_rust.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,ah6_rust_lst)
targets += ah6.s ah6.ll ah6.rsi ah6_rust.s ah6_rust.ll ah6_rust.rsi ah6.lst ah6_rust.lst \
	.rust-listing/ah6_rust.o .rust-listing-elf/ah6_rust.o
endif

clean-files += .rust-listing/ .rust-listing-elf/

ifeq ($(CONFIG_RUST_IPV6_GRE),y)
ip6_gre-y := ip6_gre_rust.o ip6_gre_helpers.o
ip6_gre-rust-deps := $(objtree)/rust/bindings/ip6_gre_generated.rs
# Explicit Rust rules have no implicit-rule stem; retain composite membership.
ip6_gre-rust-modname-multi = $(sort $(foreach m,$(multi-obj-ym),\
	$(if $(filter $(target-stem).o,$(call suffix-search,$m,.o,-objs -y -m)),$(m:.o=))))
$(addprefix $(obj)/,ip6_gre_rust.o ip6_gre.s ip6_gre.ll ip6_gre.rsi ip6_gre_rust.s ip6_gre_rust.ll ip6_gre_rust.rsi \
	.rust-listing/ip6_gre_rust.o): private modname-multi = $(ip6_gre-rust-modname-multi)
$(obj)/ip6_gre_rust.o: $(src)/ip6_gre.rs $(ip6_gre-rust-deps) \
	$(wildcard $(objtree)/include/config/RUST_IPV6_GRE) FORCE
	+$(call if_changed_rule,rustc_o_rs)

# Keep inspection on the selected source, ahead of the generic C recipes.
$(addprefix $(obj)/,ip6_gre.s ip6_gre.ll ip6_gre.rsi ip6_gre_rust.s ip6_gre_rust.ll ip6_gre_rust.rsi): private target-stem := ip6_gre_rust
$(addprefix $(obj)/,ip6_gre.s ip6_gre.ll ip6_gre.rsi ip6_gre_rust.s ip6_gre_rust.ll ip6_gre_rust.rsi): private part-of-builtin = $(if $(filter $(obj)/ip6_gre_rust.o,$(real-obj-y) $(lib-y)),y)
$(addprefix $(obj)/,ip6_gre.s ip6_gre.ll ip6_gre.rsi ip6_gre_rust.s ip6_gre_rust.ll ip6_gre_rust.rsi): private part-of-module = $(if $(filter $(obj)/ip6_gre_rust.o,$(real-obj-m)),y)
$(obj)/ip6_gre.s $(obj)/ip6_gre_rust.s: $(src)/ip6_gre.rs $(ip6_gre-rust-deps) FORCE
	+$(call if_changed_dep,rustc_s_rs)
$(obj)/ip6_gre.ll $(obj)/ip6_gre_rust.ll: $(src)/ip6_gre.rs $(ip6_gre-rust-deps) FORCE
	+$(call if_changed_dep,rustc_ll_rs)
$(obj)/ip6_gre.rsi $(obj)/ip6_gre_rust.rsi: $(src)/ip6_gre.rs $(ip6_gre-rust-deps) FORCE
	+$(call if_changed_dep,rustc_rsi_rs)

# Listing debug flags and sidecars stay private; never replace the real owner.
$(obj)/.rust-listing/ip6_gre_rust.o: private target-stem := ip6_gre_rust
$(obj)/.rust-listing/ip6_gre_rust.o: private part-of-builtin = $(if $(filter $(obj)/ip6_gre_rust.o,$(real-obj-y) $(lib-y)),y)
$(obj)/.rust-listing/ip6_gre_rust.o: private part-of-module = $(if $(filter $(obj)/ip6_gre_rust.o,$(real-obj-m)),y)
$(obj)/.rust-listing/ip6_gre_rust.o: private override RUSTFLAGS_KERNEL += -Cdebuginfo=2
$(obj)/.rust-listing/ip6_gre_rust.o: private override RUSTFLAGS_MODULE += -Cdebuginfo=2
$(obj)/.rust-listing/ip6_gre_rust.o: private override KBUILD_CFLAGS += -g
$(obj)/.rust-listing/ip6_gre_rust.o: $(src)/ip6_gre.rs $(ip6_gre-rust-deps) FORCE
	+$(call if_changed_rule,rustc_o_rs)
quiet_cmd_ip6_gre_rust_lst_elf = LD      $@
      cmd_ip6_gre_rust_lst_elf = $(LD) $(ld_flags) -r -o $@ $<
$(obj)/.rust-listing-elf/ip6_gre_rust.o: $(obj)/.rust-listing/ip6_gre_rust.o FORCE
	$(call if_changed,ip6_gre_rust_lst_elf)
quiet_cmd_ip6_gre_rust_lst = MKLST   $@
      cmd_ip6_gre_rust_lst = : System.map=$(if $(wildcard System.map),present,absent); \
	$(CONFIG_SHELL) $(srctree)/scripts/makelst $< System.map $(OBJDUMP) > $@
$(obj)/ip6_gre.lst $(obj)/ip6_gre_rust.lst: $(obj)/.rust-listing-elf/ip6_gre_rust.o $(srctree)/scripts/makelst \
	$(wildcard System.map) FORCE
	$(call if_changed,ip6_gre_rust_lst)
targets += ip6_gre.s ip6_gre.ll ip6_gre.rsi ip6_gre_rust.s ip6_gre_rust.ll ip6_gre_rust.rsi ip6_gre.lst ip6_gre_rust.lst \
	.rust-listing/ip6_gre_rust.o .rust-listing-elf/ip6_gre_rust.o
endif
