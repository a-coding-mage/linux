#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# Adaptation of the existing full-symbol comparator: exactly six new selectors.
use strict; use warnings; use Digest::SHA qw(sha256_hex); use JSON::PP;
my ($preset,$actual,$provider,$lock)=@ARGV;
die "usage: compare-config.pl preset actual c|rust config-lock.json\n" unless defined($lock) && $provider =~ /\A(?:c|rust)\z/;
sub raw {my($path)=@_;my$f;if($path=~/\.gz\z/){open $f,"-|","gzip","-dc","--",$path or die$!}else{open $f,"<:raw",$path or die$!}local$/;my$s=<$f>;close$f or die "input decode failed\n";return$s}
sub read_config {
 my($path)=@_;my%map;
 for(split /\n/,raw($path)) {
  my($key,$value);
  if(/\A(CONFIG_[A-Za-z0-9_]+)=(.*)\z/){($key,$value)=($1,$2)}
  elsif(/\A# (CONFIG_[A-Za-z0-9_]+) is not set\z/){($key,$value)=($1,'n')}
  else{next}
  die "duplicate $key\n" if exists $map{$key};$map{$key}=$value;
 }
 die "empty config\n" unless keys%map;return\%map;
}
my$identity=decode_json(raw($lock));
die "bad config lock\n" unless join(',',sort keys%$identity) eq 'c_sha256,rust_sha256,schema' && $identity->{schema}==1;
for(qw(c_sha256 rust_sha256)){die "unfrozen config lock\n" unless $identity->{$_}=~/\A[0-9a-f]{64}\z/}
die "wrong frozen Rust input\n" unless sha256_hex(raw($preset)) eq $identity->{rust_sha256};
die "wrong frozen actual config\n" unless sha256_hex(raw($actual)) eq $identity->{"${provider}_sha256"};
my$want=read_config($preset);my$got=read_config($actual);
for my$key(qw(CONFIG_RUST_BPF_LPM_TRIE CONFIG_RUST_BPF_TOKEN CONFIG_RUST_POWER_PROCESS CONFIG_RUST_BLK_CRYPTO_FALLBACK CONFIG_RUST_SELINUX_SERVICES CONFIG_RUST_SELINUX_POLICYDB)){
 die "Rust profile does not select $key\n" unless ($want->{$key}//'') eq 'y';
 $want->{$key}=$provider eq 'rust'?'y':'n';
}
die "config symbol set changed\n" unless join(',',sort keys%$want) eq join(',',sort keys%$got);
for(keys%$want){die "unexpected config value $_\n" unless $got->{$_} eq $want->{$_}}
print "CONFIG_EXACT_FROZEN_MATCH provider=$provider symbols=",scalar(keys%$got),"\n";
