#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# Keep every effective C code-generation option. C preprocessing inputs have
# already been consumed by the metadata companion and cannot apply to LLVM IR.
use strict;
use warnings;
my $check_only=@ARGV && $ARGV[0] eq '--check-policy' ? shift @ARGV : '';
my @native;
my $strategy='none';
while (@ARGV) {
    my $arg=shift @ARGV;
    if ($arg eq '-I' || $arg eq '-include' || $arg eq '-D' || $arg eq '-U') {
        die "missing preprocessing operand\n" unless @ARGV;
        shift @ARGV; next;
    }
    next if $arg =~ /^(?:-Wp,-MMD,|-I|-D|-U|-nostdinc$|-fmacro-prefix-map=)/;
    $strategy='strong' if $arg eq '-fstack-protector-strong';
    $strategy='basic' if $arg eq '-fstack-protector';
    $strategy='all' if $arg eq '-fstack-protector-all';
    $strategy='none' if $arg eq '-fno-stack-protector';
    push @native,$arg;
}
die "missing compiler\n" unless @native;
die "effective native stack policy is $strategy, expected strong\n" unless $strategy eq 'strong';
exit 0 if $check_only;
exec {$native[0]} @native or die "$native[0]: $!\n";
