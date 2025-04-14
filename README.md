# Why is libz so fast on unbuffered data!?

Data can be found here: https://github.com/lh3/biofast/releases/tag/biofast-data-v1.

## Run

`cargo bench`

```
Gnuplot not found, using plotters backend
Running bench
Benchmarking stdlib read: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 14.0s, or reduce sample count to 30.
stdlib read             time:   [128.70 ms 130.17 ms 131.95 ms]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe

Benchmarking gzread (raw): Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 15.5s, or reduce sample count to 30.
gzread (raw)            time:   [152.98 ms 153.63 ms 154.34 ms]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

Benchmarking gzread (BufReader): Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.9s, or reduce sample count to 50.
gzread (BufReader)      time:   [97.291 ms 98.843 ms 100.86 ms]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high severe
```



