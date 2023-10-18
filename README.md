# hexgrep

find the position of bytes sequences within binary files matching hexadecimal patterns

```bash
$ hexgrep --help
find the position of bytes sequences within binary files matching hexadecimal patterns

Usage: hexgrep [OPTIONS] <pattern>...

Arguments:
  <pattern>...  hex

Options:
  -p, --progress
  -h, --help      Print help
  -V, --version   Print version
```


## Examples

Finding all zip files within `/tmp`

```bash
hexgrep 50 4b 03 04 14 00 00 00 08 00 f6 0b 52 57 78 75 73 28 f5 01 00 00 08 06 00 00 08 00 1c 00 4d 61 6b 65 66 69 6c 65 55 54 09 00 03 80 35 2f 65 81 35 2f /tmp
```
