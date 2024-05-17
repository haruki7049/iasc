# iasc
"iasc", an Ip-address And Subnet-mask Calculator.

**This tool is still beta!!**
### Completed part
- Prefix to Subnet-mask
- Subnet-mask to Prefix

### Incomplete part
- IPv6 support
- Calculate the number of hosts
- Calculate Network address and Broadcast address
- Calculate the range of IP addresses
- and etc...

## Usage
```bash
iasc --conversion-type prefix-to-subnet --prefix-length 27
# Output
255.255.255.224
```
```bash
iasc --conversion-type subnet-to-prefix --subnet-mask 255.255.240.0
# Output
20
```
