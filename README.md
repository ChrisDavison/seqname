# Seqname

Rename all *files* in a directory in sequence (e.g. if you have a bunch of screenshots, and you want them renamed `000.jpg...999.jpg`).

## Usage

    seqname [OPTIONS] DIRS...

    Options:
        --prefix <STR>   String to put before number
        --suffix <STR>   String to put after number

## Examples

    seqname --prefix abc test/
    => test/abc--000.png test/abc--001.png...

    seqname --suffix xyz test/
    => test/000--xyz.png test/001--xyz.png...

Prefix and suffix *can* be used together.
