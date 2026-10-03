package Elf64;
# SPDX-License-Identifier: GPL-2.0-only
# Newly authored minimal reader of the public ELF64 little-endian format.
# Supports this x86_64 non-LTO pilot only. Unsupported inputs fail closed.
use strict; use warnings;
sub new {
    my ($class, $path) = @_;
    open my $fh, '<:raw', $path or die "$path: $!"; local $/; my $data = <$fh>;
    die "not ELF64 LE x86_64: $path\n" unless substr($data,0,6) eq "\x7fELF\x02\x01" && unpack('v',substr($data,18,2)) == 62;
    my ($base,$size,$count,$names) = (unpack('Q<',substr($data,40,8)),unpack('v',substr($data,58,2)),unpack('v',substr($data,60,2)),unpack('v',substr($data,62,2)));
    die "unsupported section numbering\n" unless $size == 64 && $count && $names < $count;
    my $self = bless {data=>$data,sections=>[],symbols=>[],rels=>[]}, $class;
    for my $i (0..$count-1) {
        my @v = unpack('V V Q< Q< Q< Q< V V Q< Q<',substr($data,$base+$i*64,64));
        push @{$self->{sections}}, {index=>$i,nameoff=>$v[0],type=>$v[1],flags=>$v[2],addr=>$v[3],offset=>$v[4],size=>$v[5],link=>$v[6],info=>$v[7],entry=>$v[9]};
    }
    my $strings=$self->section_bytes($names);
    for (@{$self->{sections}}) { $_->{name}=cstr($strings,$_->{nameoff}); }
    for my $sec (@{$self->{sections}}) {
        next unless $sec->{type} == 2;
        die "unsupported multiple symbol tables\n" if @{$self->{symbols}};
        die "bad symbol size\n" unless $sec->{entry} == 24;
        my $names=$self->section_bytes($sec->{link}); my $raw=$self->section_bytes($sec->{index});
        for (my $i=0;$i<length($raw);$i+=24) {
            my ($n,$info,$other,$section,$value,$len)=unpack('V C C v Q< Q<',substr($raw,$i,24));
            push @{$self->{symbols}}, {name=>cstr($names,$n),type=>$info & 15,bind=>$info>>4,section=>$section,value=>$value,size=>$len};
        }
        $self->{symtab}=$sec->{index};
    }
    die "missing full symbols\n" unless @{$self->{symbols}};
    for my $sec (@{$self->{sections}}) {
        next unless $sec->{type} == 4;
        die "unsupported relocation table\n" unless $sec->{entry} == 24 && $sec->{link} == $self->{symtab};
        my $raw=$self->section_bytes($sec->{index});
        for (my $i=0;$i<length($raw);$i+=24) {
            my ($offset,$info,$addend)=unpack('Q< Q< q<',substr($raw,$i,24));
            push @{$self->{rels}}, {section=>$sec->{info},offset=>$offset,type=>$info & 0xffffffff,symbol=>$self->{symbols}[$info>>32],addend=>$addend};
        }
    }
    return $self;
}
sub cstr { my ($data,$at)=@_; die "string out of range\n" if $at>=length($data); my $end=index($data,"\0",$at); die "unterminated string\n" if $end<0; return substr($data,$at,$end-$at); }
sub section_bytes { my ($s,$i)=@_; my $sec=$s->{sections}[$i] // die "section out of range\n"; die "NOBITS not supported\n" if $sec->{type}==8; return substr($s->{data},$sec->{offset},$sec->{size}); }
sub symbols { @{$_[0]->{symbols}} }
sub one {
    my ($s,$name)=@_; my @m=grep { $_->{name} eq $name && $_->{section}>0 && $_->{section}<0xff00 } $s->symbols;
    die "expected one definition of $name, got ".scalar(@m)."\n" unless @m==1; return $m[0];
}
sub symbol_bytes { my ($s,$sym)=@_; my $sec=$s->{sections}[$sym->{section}]; return substr($s->section_bytes($sym->{section}),$sym->{value}-$sec->{addr},$sym->{size}); }
sub at {
    my ($s,$addr,$len)=@_;
    for my $sec (@{$s->{sections}}) {
        next unless $sec->{type} != 8 && $sec->{flags}&2 && $addr >= $sec->{addr} && $addr+$len <= $sec->{addr}+$sec->{size};
        return substr($s->{data},$sec->{offset}+$addr-$sec->{addr},$len);
    }
    die "address not in file-backed allocated section\n";
}
sub string_at {
    my ($s,$addr)=@_; my $result='';
    for (0..4095) { my $b=$s->at($addr+$_,1); return $result if $b eq "\0"; $result.=$b; }
    die "final string too long\n";
}
sub rels_in { my ($s,$sym)=@_; return grep { $_->{section}==$sym->{section} && $_->{offset}>=$sym->{value} && $_->{offset}<$sym->{value}+$sym->{size} } @{$s->{rels}}; }
1;
