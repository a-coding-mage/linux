#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# Newly authored against public source and ELF64. No imported private auditor.
use strict; use warnings; use FindBin; use lib $FindBin::Bin; use Elf64;
my ($out)=@ARGV; die "usage: audit-elf.pl output-tree\n" unless defined $out;
my $final=Elf64->new("$out/vmlinux");
my $norm=Elf64->new("$out/fs/unicode/utf8-norm.o");
my $core=Elf64->new("$out/fs/unicode/utf8-core.o");
my $test=Elf64->new("$out/fs/unicode/tests/utf8_kunit.o");
my $data=Elf64->new("$out/fs/unicode/utf8data.o");
my @api=qw(utf8version_is_supported utf8nlen utf8ncursor utf8byte);
my @cases=qw(check_supported_versions check_utf8_comparisons check_utf8_nfdicf check_utf8_nfdi);
my %api=map {$_=>1} @api;
for my $name (@api) {
    my $n=$norm->one($name); my $f=$final->one($name);
    die "not a function $name\n" unless $n->{type}==2 && $f->{type}==2;
    printf "OWNER_FUNCTION %s input_bytes=%d final_bytes=%d\n",$name,$n->{size},$f->{size};
}
# Prove the selected provider's linked code, not merely same-named symbols.
# Track section placement from final API symbols and actual relocations, then
# compare every non-relocation byte of the complete referenced source sections.
# Named relocation targets must resolve to their exact final symbols. Anonymous
# source sections must have one consistent placement and match in full as well.
sub prove_linked_object {
my ($obj,$label)=@_;
my (%placement,%checked);
for my $n (grep {$_->{type}==2 && $_->{section}>0 && $_->{section}<0xff00} $obj->symbols) {
    my $name=$n->{name}; my $f=$final->one($name);
    die "provider function size changed $name\n" unless $n->{size}==$f->{size};
    my $base=$f->{value}-$n->{value};
    die "inconsistent provider section placement\n" if exists($placement{$n->{section}}) && $placement{$n->{section}}!=$base;
    $placement{$n->{section}}=$base;
}
while (my @pending=grep {!$checked{$_}} keys %placement) {
    my $section=$pending[0]; my $sec=$obj->{sections}[$section];
    die "unsupported referenced source section\n" unless $sec->{type}==1 && $sec->{flags}&2;
    my $expected=$obj->section_bytes($section);
    my $actual=$final->at($placement{$section},$sec->{size});
    for my $r (grep {$_->{section}==$section} @{$obj->{rels}}) {
        my $width=$r->{type}==1 ? 8 : 4;
        my $bytes=substr($actual,$r->{offset},$width); my $target;
        if ($r->{type}==2 || $r->{type}==4) {
            $target=$placement{$section}+$r->{offset}+unpack('l<',$bytes)-$r->{addend};
        } elsif ($r->{type}==11) {
            $target=unpack('Q<',pack('q<',unpack('l<',$bytes)))-$r->{addend};
        } elsif ($r->{type}==10) { $target=unpack('V',$bytes)-$r->{addend}; }
        elsif ($r->{type}==1) { $target=unpack('Q<',$bytes)-$r->{addend}; }
        else { die "unsupported provider relocation $r->{type}\n"; }
        my $symbol=$r->{symbol};
        my $defined=$symbol->{section}>0 && $symbol->{section}<0xff00;
        my $target_section=$defined ? $obj->{sections}[$symbol->{section}] : undef;
        if ($defined && ($target_section->{flags}&0x30)==0x30) {
            # SHF_MERGE|SHF_STRINGS permits string pooling; prove content at the
            # actual referenced offset instead of assuming an affine placement.
            my $bias=($r->{type}==2 || $r->{type}==4) ? 4 : 0;
            my $literal=Elf64::cstr($obj->section_bytes($symbol->{section}),$symbol->{value}+$r->{addend}+$bias);
            die "merged source string changed\n" unless $final->string_at($target+$r->{addend}+$bias) eq $literal;
        } elsif ($symbol->{name} && $symbol->{type}!=3) {
            my $destination=$final->one($symbol->{name});
            die "provider import/call/data redirected $symbol->{name}\n" unless $target==$destination->{value};
            if ($defined && $target_section->{flags}&2) {
                my $base=$target-$symbol->{value};
                die "inconsistent named section placement\n" if exists($placement{$symbol->{section}}) && $placement{$symbol->{section}}!=$base;
                $placement{$symbol->{section}}=$base;
            }
        } else {
            die "anonymous undefined provider relocation\n" unless $symbol->{section}>0 && $symbol->{section}<0xff00;
            my $base=$target-$symbol->{value};
            die "inconsistent anonymous section placement\n" if exists($placement{$symbol->{section}}) && $placement{$symbol->{section}}!=$base;
            $placement{$symbol->{section}}=$base;
        }
        substr($expected,$r->{offset},$width)=$bytes;
    }
    die "selected provider bytes differ in final $sec->{name}\n" unless $actual eq $expected;
    $checked{$section}=1;
    print "LINKED_OBJECT_SECTION_BYTES $label $sec->{name} $sec->{size}\n";
}
}
prove_linked_object($norm,'selected_normalizer');
prove_linked_object($test,'original_C_tests');
prove_linked_object($core,'original_C_core');
prove_linked_object(Elf64->new("$out/lib/crc/crc16.o"),'original_C_crc16');
# Every undefined import in the selected provider resolves in the actual image.
# Panic imports are recorded; no panic-name heuristic replaces original tests.
for my $sym ($norm->symbols) {
    next if $sym->{section} || !$sym->{name};
    my $f=$final->one($sym->{name});
    printf "PROVIDER_IMPORT_RESOLVED %s binding=%d\n",$sym->{name},$f->{bind};
}
my %calls;
for my $entry (['original_test',$test],['original_core',$core]) {
    my ($label,$obj)=@$entry;
    for my $rel (@{$obj->{rels}}) {
        my $name=$rel->{symbol}{name}; next unless $api{$name};
        next unless $obj->{sections}[$rel->{section}]{flags}&4;
        die "unsupported API relocation $rel->{type}\n" unless $rel->{type}==2 || $rel->{type}==4;
        my @owner=grep {$_->{type}==2 && $_->{section}==$rel->{section} && $_->{value}<=$rel->{offset} && $_->{value}+$_->{size}>$rel->{offset}} $obj->symbols;
        die "ambiguous source caller\n" unless @owner==1;
        my $fn=$final->one($owner[0]{name}); my $target=$final->one($name);
        my $field=$fn->{value}+$rel->{offset}-$owner[0]{value};
        my $actual=unpack('V',$final->at($field,4));
        # Arithmetic stays in small differences to preserve exact 64-bit VA math.
        my $expected=($target->{value}-$field+$rel->{addend}) & 0xffffffff;
        die "final caller resolution mismatch $name\n" unless $actual==$expected;
        my $opcode=$final->at($field-1,1);
        die "API reference is not a direct call/jump\n" unless $opcode eq "\xe8" || $opcode eq "\xe9";
        $calls{$name}++; printf "FINAL_CALL %s %s -> %s\n",$label,$owner[0]{name},$name;
    }
}
for (@api) { die "no original C caller for $_\n" unless $calls{$_}; }
# Reconstruct each source object's relocated registration/table data using final
# pointers. Non-pointer bytes, function pointer targets, names and sentinels must
# all match. No hardcoded C struct size or field offset is used.
sub relocated_data {
    my ($obj,$name)=@_; my $src=$obj->one($name); my $dst=$final->one($name);
    die "size mismatch $name\n" unless $src->{size}==$dst->{size};
    my $expected=$obj->symbol_bytes($src); my $actual=$final->symbol_bytes($dst); my @targets;
    for my $r ($obj->rels_in($src)) {
        die "unsupported data relocation in $name\n" unless $r->{type}==1;
        my $at=$r->{offset}-$src->{value}; my $target=$r->{symbol};
        die "external data pointer in $name\n" unless $target->{section}>0 && $target->{section}<0xff00;
        my $value=$target->{value}+$r->{addend};
        my @named=grep {($_->{type}==1 || $_->{type}==2) && $_->{section}==$target->{section} && $_->{value}==$value && $_->{size}>0} $obj->symbols;
        my $ptr=unpack('Q<',substr($actual,$at,8));
        if (@named==1) {
            my $want=$final->one($named[0]{name}); die "wrong pointer $name -> $named[0]{name}\n" unless $ptr==$want->{value};
            push @targets,$named[0]{name};
        } elsif (@named==0) {
            my $str=Elf64::cstr($obj->section_bytes($target->{section}),$value);
            die "wrong string pointer $name\n" unless $final->string_at($ptr) eq $str;
            push @targets,"string:$str";
        } else { die "ambiguous data target $name\n"; }
        substr($expected,$at,8)=substr($actual,$at,8);
    }
    die "non-pointer data mismatch $name\n" unless $expected eq $actual;
    print "FINAL_DATA $name ",join(',',@targets),"\n";
    return @targets;
}
my @registered=relocated_data($test,'unicode_normalization_test_cases');
my @functions=grep {/^check_/} @registered;
my @names=map {s/^string://r} grep {/^string:check_/} @registered;
die "original four case registrations/order changed\n" unless join(',',@functions) eq join(',',@cases) && join(',',@names) eq join(',',@cases);
my @suite=relocated_data($test,'unicode_normalization_test_suite');
for my $required (qw(unicode_normalization_test_cases init_test_ucd exit_test_ucd)) {
    die "missing suite pointer $required\n" unless grep {$_ eq $required} @suite;
}
my $table_start=$final->one('__kunit_suites_start')->{value};
my $table_end=$final->one('__kunit_suites_end')->{value};
die "invalid KUnit linker table bounds\n" unless $table_end>$table_start && ($table_end-$table_start)%8==0;
my $table=$final->at($table_start,$table_end-$table_start);
my $suiteva=$final->one('unicode_normalization_test_suite')->{value};
my $count=grep {$_==$suiteva} unpack('Q<*',$table);
die "suite must be linked once\n" unless $count==1;
for my $name (qw(utf8agetab utf8nfdicfdata utf8nfdidata utf8data)) {
    my $a=$data->one($name); my $b=$final->one($name);
    die "original Unicode table changed $name\n" unless $data->symbol_bytes($a) eq $final->symbol_bytes($b);
    print "ORIGINAL_TABLE_BYTES $name $a->{size}\n";
}
relocated_data($data,'utf8_data_table');
print "FINAL_ELF_OWNERSHIP_PASS original_C_calls=resolved cases=4 suite=1 original_tables=equal\n";
