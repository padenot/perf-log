# perf-log

Try different techniques to serialize the filename, the line number, and a message, into a buffer, to send that via FFI, to log.

Sample output on a my linux machine (i9-7940X CPU @ 3.10GHz, frequency scaling disabled)


```
log1                    time:   [139.59 ns 139.66 ns 139.73 ns]                 
                        change: [-0.2777% +0.1440% +0.5795%] (p = 0.53 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe

log2                    time:   [84.530 ns 84.652 ns 84.813 ns]                 
                        change: [+0.0199% +0.3685% +0.6967%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
  2 (2.00%) high severe

log3                    time:   [89.948 ns 90.085 ns 90.252 ns]                 
                        change: [-3.6666% -3.3054% -2.9063%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe

log4                    time:   [90.698 ns 90.803 ns 90.939 ns]                 
                        change: [-0.9815% -0.6622% -0.3311%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  7 (7.00%) high severe
```

tl;dr, [log_2](https://github.com/padenot/perf-log/blob/master/src/lib.rs#L110-L122) is faster. This could potentially be made even faster by using `itoa`, that is already in use in Gecko.
