# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [image decoding](#image-decoding)

## Benchmark Results

### image decoding

|        | `zune jpg`               | `png`                            | `zune PNG`                       | `mozjpg`                        | `jpeg-decoder`                  | `jxl-oxide slow`                 | `libjxl slow`                    | `jxl-oxide fast`                 | `libjxl fast`                    | `avif`                           | `webp`                           |
|:-------|:-------------------------|:---------------------------------|:---------------------------------|:--------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:-------------------------------- |
|        | `45.55 ms` (✅ **1.00x**) | `118.67 ms` (❌ *2.61x slower*)   | `153.34 ms` (❌ *3.37x slower*)   | `35.83 ms` (✅ **1.27x faster**) | `51.34 ms` (❌ *1.13x slower*)   | `227.59 ms` (❌ *5.00x slower*)   | `239.14 ms` (❌ *5.25x slower*)   | `210.08 ms` (❌ *4.61x slower*)   | `187.45 ms` (❌ *4.12x slower*)   | `184.81 ms` (❌ *4.06x slower*)   | `59.19 ms` (❌ *1.30x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

