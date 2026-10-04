#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# E820's reviewed x86-64 SMP/strong compiler-policy admission checks.
use strict;
use warnings;
my ($mode,$file,@args)=@ARGV;
die "usage: $0 metadata|empty|rust-debug|rust|object|assembly FILE [OBJDUMP]\n" unless defined $file;
sub readall {
    open my $f,'<',$_[0] or die "$_[0]: $!\n";
    local $/; return <$f>;
}
sub metadata {
    my ($s)=@_;
    my ($ids)=$s =~ /^!llvm\.module\.flags = !\{([^\n]*)\}$/m;
    die "missing module flags\n" unless defined $ids;
    my %flags;
    for my $id (split /,\s*/,$ids) {
        die "invalid module flag ID\n" unless $id =~ /^!\d+$/;
        my ($name,$val)=$s =~ /^\Q$id\E = !\{i32 \d+, !"([^"]+)", (.+)\}$/m;
        die "invalid module flag $id\n" unless defined $name;
        die "duplicate module flag $name\n" if exists $flags{$name};
        $flags{$name}=$val;
    }
    for my $pair (['stack-protector-guard-reg','!"gs"'],
                  ['stack-protector-guard-symbol','!"__ref_stack_chk_guard"'],
                  ['Code Model','i32 2']) {
        die "missing/wrong $pair->[0]\n" unless ($flags{$pair->[0]} // '') eq $pair->[1];
    }
    die "unexpected guard offset or kind\n" if exists $flags{'stack-protector-guard-offset'} || exists $flags{'stack-protector-guard'};
    die "wrong target triple\n" unless $s =~ /^target triple = "x86_64-unknown-linux-gnu"$/m;
    return \%flags;
}
if ($mode eq 'rust-debug') {
    my $flags=metadata(readall($file));
    # A private listing adds -g even when the kernel has no debug info. Follow
    # the native compiler's actual DWARF version instead of rustc's default.
    if (exists $flags->{'Dwarf Version'}) {
        die "unsupported native DWARF version\n" unless $flags->{'Dwarf Version'} =~ /^i32 ([45])$/;
        print "-Zdwarf-version=$1\n";
    }
    exit 0;
} elsif ($mode eq 'metadata' || $mode eq 'empty') {
    my $s=readall($file); metadata($s);
    die "native metadata contains code or data\n" if $mode eq 'empty' && $s =~ /^(?:@|define\b|declare\b|module asm\b)/m;
} elsif ($mode eq 'rust') {
    my $s=readall($file); my $count=0;
    while ($s =~ /^define\b[^\n]+ #([0-9]+)[^\n]*\{$/mg) {
        my $id=$1;
        my ($attrs)=$s =~ /^attributes #\Q$id\E = \{([^\n]*)\}$/m;
        die "function without strong strategy\n" unless defined $attrs && $attrs =~ /\bsspstrong\b/;
        $count++;
    }
    die "no Rust function definitions\n" unless $count;
    my $definitions=()=$s =~ /^define\b/mg;
    die "unexamined function definition\n" unless $count==$definitions;
} elsif ($mode eq 'object') {
    die 'OBJDUMP required' unless @args;
    open my $p,'-|',@args,'-dr','--no-show-raw-insn',$file or die $!;
    local $/; my $s=<$p>; close $p or die "objdump failed\n";
    my %body;
    while ($s =~ /^[0-9a-f]+ <([^>]+)>:\n(.*?)(?=^[0-9a-f]+ <|\z)/msg) { $body{$1}=$2; }
    for my $name (qw(e820__setup_pci_gap parse_memopt parse_memmap_one)) {
        my @names=grep { $_ eq $name || /_4e820\d+_?\Q$name\E(?:\.\d+)?$/ } keys %body;
        die "missing/duplicate $name\n" unless @names==1;
        my $b=$body{$names[0]};
        my $guards=()=$b =~ /%gs:(?:0x0)?\(%rip\)[^\n]*\n\s*[0-9a-f]+:\s+R_X86_64_PC32\s+__ref_stack_chk_guard-0x4/g;
        my $fails=()=$b =~ /R_X86_64_PLT32\s+__stack_chk_fail-0x4/g;
        die "bad guard $name: guards=$guards fails=$fails\n" unless $guards==2 && $fails==1;
        print "$name GS_symbol_guards=2 failure_calls=1\n";
    }
} elsif ($mode eq 'assembly') {
    my $s=readall($file); my %body;
    my @functions=$s =~ /^\s*\.type\s+([^,]+),\s*\@function$/mg;
    for my $fn (@functions) {
        my ($body)=$s =~ /^\Q$fn\E:[^\n]*\n(.*?)(?=^\.Lfunc_end\d+:)/ms;
        die "missing assembly body $fn\n" unless defined $body;
        $body{$fn}=$body;
    }
    for my $name (qw(e820__setup_pci_gap parse_memopt parse_memmap_one)) {
        my @names=grep { $_ eq $name || /_4e820\d+_?\Q$name\E(?:\.\d+)?$/ } keys %body;
        die "missing/duplicate $name assembly\n" unless @names==1;
        my $b=$body{$names[0]};
        my $guards=()=$b =~ /%gs:__ref_stack_chk_guard\(%rip\)/g;
        my $fails=()=$b =~ /\bcallq\s+__stack_chk_fail\b/g;
        die "bad assembly guard $name: $guards/$fails\n" unless $guards==2 && $fails==1;
    }
} else { die "unknown mode $mode\n"; }
print "PASS E820 native policy $mode $file\n";
