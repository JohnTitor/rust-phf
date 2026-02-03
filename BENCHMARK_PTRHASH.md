# PtrHash vs CHD microbenchmark summary

Source: user-provided `cargo bench -p phf_generator` output (Criterion).
Numbers below use the median time from each Criterion bracket (middle value).
Times are normalized to human-readable units; speedup = CHD / PtrHash (higher is better for PtrHash).

## Executive summary

- Overall geometric-mean speedup: **1.74x**
- large geometric-mean speedup: **4.41x**
- medium geometric-mean speedup: **11.39x**
- small geometric-mean speedup: **0.14x**
- xlarge geometric-mean speedup: **3.24x**

## Small

| Size | CHD (median) | PtrHash (median) | Speedup (CHD/PtrHash) |
| ---: | ---: | ---: | ---: |
| 0 | 20.472 ns | 5.336 ns | 3.84x |
| 1 | 172.220 ns | 181.390 ns | 0.95x |
| 2 | 232.560 ns | 23.323 us | 0.01x |
| 5 | 283.150 ns | 60.526 us | 0.00x |
| 10 | 911.670 ns | 22.022 us | 0.04x |
| 25 | 11.789 us | 35.649 us | 0.33x |
| 50 | 5.556 us | 183.650 us | 0.03x |
| 75 | 76.159 us | 28.482 us | 2.67x |

## Medium

| Size | CHD (median) | PtrHash (median) | Speedup (CHD/PtrHash) |
| ---: | ---: | ---: | ---: |
| 100 | 13.668 us | 22.722 us | 0.60x |
| 250 | 80.141 us | 5.721 us | 14.01x |
| 500 | 235.170 us | 11.158 us | 21.08x |
| 1000 | 540.660 us | 22.087 us | 24.48x |
| 2500 | 1.234 ms | 59.858 us | 20.61x |
| 5000 | 2.474 ms | 128.860 us | 19.20x |
| 7500 | 3.803 ms | 262.270 us | 14.50x |

## Large

| Size | CHD (median) | PtrHash (median) | Speedup (CHD/PtrHash) |
| ---: | ---: | ---: | ---: |
| 10000 | 4.657 ms | 1.804 ms | 2.58x |
| 25000 | 13.247 ms | 2.166 ms | 6.12x |
| 50000 | 29.885 ms | 6.987 ms | 4.28x |
| 75000 | 46.597 ms | 8.314 ms | 5.60x |

## Xlarge

| Size | CHD (median) | PtrHash (median) | Speedup (CHD/PtrHash) |
| ---: | ---: | ---: | ---: |
| 100000 | 64.042 ms | 34.542 ms | 1.85x |
| 250000 | 201.440 ms | 62.224 ms | 3.24x |
| 500000 | 469.330 ms | 124.770 ms | 3.76x |
| 750000 | 752.120 ms | 246.020 ms | 3.06x |
| 1000000 | 1.411 s | 273.430 ms | 5.16x |

## Notes

- Very small sizes show overhead-dominated variance; larger sizes reflect steady-state generation costs.
- Criterion emitted sampling warnings for larger sizes; comparisons still use the reported medians.