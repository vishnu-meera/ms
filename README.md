# ms

A small Rust CLI for converting between time units and milliseconds — inspired by the [vercel/ms](https://github.com/vercel/ms) npm package.

## Build

```sh
cargo build --release
```

The binary will be at `target/release/ms`.

## Usage

```sh
ms <number> [unit]
```

- With **one argument** (a number of milliseconds), `ms` prints a short human-readable form.
- With **two arguments** (a number and a unit), `ms` converts that duration into milliseconds.

## Examples

### Convert a unit to milliseconds

```sh
$ ms 2 days
172800000

$ ms 1 d
86400000

$ ms 10 h
36000000

$ ms 2.5 hrs
9000000

$ ms 2 h
7200000

$ ms 1 m
60000

$ ms 5 s
5000

$ ms 1 y
31557600000
```

### Format milliseconds into a short string

```sh
$ ms 60000
1 min

$ ms 3600000
1 h

$ ms 86400000
1 d

$ ms 100
100 ms
```

## Supported units

| Unit         | Accepted spellings                                  |
| ------------ | --------------------------------------------------- |
| Years        | `y`, `yr`, `yrs`, `year`, `years`                   |
| Months       | `mo`, `month`, `months`                             |
| Weeks        | `w`, `wk`, `week`, `weeks`                          |
| Days         | `d`, `day`, `days`                                  |
| Hours        | `h`, `hr`, `hrs`, `hour`, `hours`                   |
| Minutes      | `m`, `min`, `mins`, `minute`, `minutes`             |
| Seconds      | `s`, `sec`, `secs`, `second`, `seconds`             |
| Milliseconds | `ms`, `msec`, `msecs`, `millisecond`, `milliseconds` |

Units are case-insensitive. A year is `365.25` days; a month is `1/12` of a year.

## Notes

- Negative numbers are treated as their absolute value (e.g. `ms -1 h` → `3600000`).
- Passing an unknown unit or a non-numeric value exits with a non-zero status and an error message.
- Unlike the JS original, the number and unit must be passed as **separate arguments** (`ms 2 days`, not `ms "2 days"`).
