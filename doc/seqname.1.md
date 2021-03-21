% SEQNAME(1) Name files in sequence
%
% 2021-03-21


# NAME

seqname - Name files sequentially

# SYNOPSIS

**seqname [--prefix <STR> --suffix <STR>] DIRS...**

# DESCRIPTION

For each file in each directory, name them sequentially. Optionally, add a
prefix for before the number, or a suffix for after the number.


# EXAMPLES

**seqname \-\-prefix abc test/** => *test/abc\-\-000.png test/abc\-\-001.png...*

**seqname \-\-suffix xyz test/** => *test/000\-\-xyz.png test/001\-\-xyz.png...*

**seqname \-\-prefix abc \-\-suffix xyz test/** => *test/abc\-\-000\-\-xyz.png test/abc\-\-001\-\-xyz.png...*

# AUTHORS

Chris Davison <c.jr.davison@gmail.com>
