# ipmitool dcmi power reading

[![dependency status](https://deps.rs/repo/github/wdes/ipmitool-dcmi-power-reading/status.svg)](https://deps.rs/repo/github/wdes/ipmitool-dcmi-power-reading)
[![Build and test rust code](https://github.com/wdes/ipmitool-dcmi-power-reading/actions/workflows/rust.yml/badge.svg)](https://github.com/wdes/ipmitool-dcmi-power-reading/actions/workflows/rust.yml)
![maintenance-status](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

## Why does this tool exist

Because you are asking yourself "how do I get my power consumption numbers" ?

Internet links:

- https://serverfault.com/questions/736068/how-do-i-get-the-power-consumption-of-a-dell-poweredge-server-on-the-cli
- https://serverfault.com/questions/389224/power-usage-via-ipmi-or-bios-or

So you seem to have two options:
- Using racadm: (https://serverfault.com/a/1070451/336084)
- Using ipmitool: (https://serverfault.com/a/1141974/336084) - (https://wiki.evolix.org/HowtoIPMI)

I used the racadm method for years, and I find it slow and sometimes it does not work because an iDRAC session is open.
Then I used ipmitool and it did great, but had text output I needed to parse.

So I first wrote the same tool with the same text output and it worked.
The original C code can be found here: [ipmitool 1.8.19](https://github.com/ipmitool/ipmitool/blob/IPMITOOL_1_8_19/lib/ipmi_dcmi.c#L1398-L1454)

Special thanks to the library [ipmi-rs](https://github.com/datdenkikniet/ipmi-rs) that made this possible.

## Use

```text
A tool to fetch the power reading with ipmi dcmi

Usage: ipmitool-dcmi-power-reading [OPTIONS]

Options:
  -c, --connection-uri <CONNECTION_URI>
          The connection URI to use [default: file:///dev/ipmi0]
      --timeout-ms <TIMEOUT_MS>
          How many milliseconds to wait before timing out while waiting for a response [default: 2000]
      --format <FORMAT>
          The format to output [default: text] [possible values: text, json]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Example (text)

```text
Instantaneous power reading              : 212      Watts
Minimum during sampling period           : 2        Watts
Maximum during sampling period           : 468      Watts
Average power reading over sample period : 184      Watts
IPMI timestamp                           : 2024-05-05 14:17:17 UTC
Sampling period                          : 1000 Milliseconds
Power reading state is                   : activated
```

## Example (json)

```json
{"grp_id":220,"curr_pwr":209,"min_sample":2,"max_sample":468,"avg_pwr":184,"time_stamp":1714918638,"sample":1000,"state":64}
```
