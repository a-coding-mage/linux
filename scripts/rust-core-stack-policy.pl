#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# Admit only the reviewed native policy and original per-function exemption.
use strict;
use warnings;
use FindBin qw($Bin);
my ($mode, $owner, $file, @args) = @ARGV;
my %owners;
open my $manifest, '<', "$Bin/rust-native-policy-owners" or die "owner manifest: $!\n";
while (<$manifest>) {
    next if /^\s*(?:#|$)/;
    chomp;
    my @fields = split /\s+/;
    die "invalid owner field count\n" unless @fields == 3;
    my ($name, $strategy, $exceptions) = @fields;
    die "invalid owner policy\n" unless defined($exceptions) && $name =~ m{^[A-Za-z_0-9/]+$} && $strategy eq 'strong';
    die "duplicate owner policy $name\n" if exists $owners{$name};
    my @exceptions = $exceptions eq '-' ? () : split /,/, $exceptions;
    die "invalid function exemption\n" if grep { !/^[A-Za-z_][A-Za-z_0-9]*$/ } @exceptions;
    my %seen;
    die "duplicate function exemption\n" if grep { $seen{$_}++ } @exceptions;
    $owners{$name} = \@exceptions;
}
close $manifest;
die "usage: $0 MODE OWNER FILE [OUTPUT|OBJDUMP]\n" unless defined($file) && $owners{$owner};
my %exempt = map { $_ => 1 } @{$owners{$owner}};
sub readall {
    open my $f, '<', $_[0] or die "$_[0]: $!\n";
    local $/; return <$f>;
}
sub metadata {
    my ($s) = @_;
    my ($ids) = $s =~ /^!llvm\.module\.flags = !\{([^\n]*)\}$/m;
    die "missing module flags\n" unless defined $ids;
    my %flags;
    for my $id (split /,\s*/, $ids) {
        die "invalid module flag ID\n" unless $id =~ /^!\d+$/;
        my ($name, $val) = $s =~ /^\Q$id\E = !\{i32 \d+, !"([^"]+)", (.+)\}$/m;
        die "invalid module flag $id\n" unless defined $name;
        die "duplicate module flag $name\n" if exists $flags{$name};
        $flags{$name} = $val;
    }
    for my $pair (['stack-protector-guard-reg', '!"gs"'],
                  ['stack-protector-guard-symbol', '!"__ref_stack_chk_guard"'],
                  ['Code Model', 'i32 2']) {
        die "missing/wrong $pair->[0]\n" unless ($flags{$pair->[0]} // '') eq $pair->[1];
    }
    die "unexpected guard offset or kind\n" if exists $flags{'stack-protector-guard-offset'} || exists $flags{'stack-protector-guard'};
    die "wrong target triple\n" unless $s =~ /^target triple = "x86_64-unknown-linux-gnu"$/m;
    return \%flags;
}
sub functions {
    my ($s, $raw) = @_;
    my @definitions = $s =~ /^(define\b[^\n]*)$/mg;
    die "no Rust function definitions\n" unless @definitions;
    my @functions;
    my %exempt_count;
    for my $definition (@definitions) {
        my ($name, $id) = $definition =~ /\@([A-Za-z_0-9.]+)\([^\n]* #([0-9]+)[^\n]*\{$/;
        die "unexamined function definition: $definition\n" unless defined $id;
        my ($attrs) = $s =~ /^attributes #\Q$id\E = \{([^\n]*)\}$/m;
        die "missing function attribute group $id\n" unless defined $attrs;
        my $is_exempt = $exempt{$name};
        $exempt_count{$name}++ if $is_exempt;
        my @strategies = $attrs =~ /\b(ssp|sspreq|sspstrong)\b/g;
        if (!$raw && $is_exempt) {
            die "$name must retain __no_stack_protector\n" if @strategies;
        } else {
            die "$name does not have exactly native strong strategy\n" unless @strategies == 1 && $strategies[0] eq 'sspstrong';
        }
        push @functions, [$definition, $name, $id, $attrs, $is_exempt];
    }
    for my $name (keys %exempt) {
        die "missing/duplicate $name exemption\n" unless ($exempt_count{$name} // 0) == 1;
    }
    return @functions;
}
if ($mode eq 'prepare') {
    die "one output required\n" unless @args == 1;
    my $s = readall($file);
    my @functions = functions($s, 1);
    # rustc 1.85.1 exposes only a session-wide SSP strategy. Clang represents
    # __attribute__((no_stack_protector)) by omitting all three SSP attributes.
    # Clone the one exempt definition's group; never edit a shared group.
    # No instruction, data, symbol, or other function attribute is changed.
    my @ids = $s =~ /^attributes #(\d+) = /mg;
    my $next = 1 + (sort { $b <=> $a } @ids)[0];
    for my $fn (@functions) {
        next unless $fn->[4];
        my ($definition, $name, $id, $attrs) = @$fn;
        $attrs =~ s/\bsspstrong\b\s*//;
        (my $new = $definition) =~ s/ #\Q$id\E(?=\s)/ #$next/;
        my $changed = $s =~ s/^\Q$definition\E$/$new/m;
        die "failed to isolate $name\n" unless $changed == 1;
        $s =~ s/^(attributes #\d+ = )/attributes #$next = {$attrs}\n\n$1/m
            or die "missing attribute insertion point\n";
        $next++;
    }
    functions($s, 0);
    open my $out, '>', $args[0] or die "$args[0]: $!\n";
    print $out $s or die "$args[0]: $!\n";
    close $out or die "$args[0]: $!\n";
} elsif ($mode eq 'rust') {
    functions(readall($file), 0);
} elsif ($mode eq 'metadata' || $mode eq 'empty' || $mode eq 'rust-debug') {
    my $s = readall($file);
    my $flags = metadata($s);
    die "native metadata contains code or data\n" if $mode eq 'empty' && $s =~ /^(?:@|define\b|declare\b|module asm\b)/m;
    if ($mode eq 'rust-debug') {
        if (exists $flags->{'Dwarf Version'}) {
            die "unsupported native DWARF version\n" unless $flags->{'Dwarf Version'} =~ /^i32 ([45])$/;
            print "-Zdwarf-version=$1\n";
        }
        exit 0;
    }
} elsif ($mode eq 'object') {
    die "OBJDUMP required\n" unless @args;
    open my $p, '-|', @args, '-dr', '--no-show-raw-insn', $file or die $!;
    local $/; my $s = <$p>; close $p or die "objdump failed\n";
    # This is build admission, not the separate all-exits acceptance audit.
    # Require native protection somewhere in each independently audited owner.
    my $guards = () = $s =~ /%gs:(?:0x0)?\(%rip\)[^\n]*\n\s*[0-9a-f]+:\s+R_X86_64_PC32\s+__ref_stack_chk_guard-0x4/g;
    my $failures = () = $s =~ /R_X86_64_PLT32\s+__stack_chk_fail-0x4/g;
    die "native guard/check/failure references absent\n" unless $guards >= 2 && $failures >= 1;
    for my $exempt (keys %exempt) {
        my @bodies = $s =~ /^[0-9a-f]+ <\Q$exempt\E>:\n(.*?)(?=^[0-9a-f]+ <|\z)/msg;
        die "missing/duplicate exempt body\n" unless @bodies == 1;
        die "$exempt unexpectedly guarded\n" if $bodies[0] =~ /__ref_stack_chk_guard|__stack_chk_fail/;
    }
} else {
    die "unknown mode $mode\n";
}
print "PASS native Rust policy $mode $owner $file\n";
