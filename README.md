# iasc
"iasc", an Ip-address And Subnet-mask Calculator.

## Usage
```bash
# Use IP address and Subnet mask
iasc --ip-address=192.168.0.1 --subnet-mask=255.255.255.0
```
This command's result is:
```text
IP Address: 192.168.0.1
Subnet Mask: 255.255.255.0
Prefix Length: /24
Network Address: 192.168.0.0
Broadcast Address: 192.168.0.255
```

```bash
# Use IP address and Prefix length
iasc --ip-address=192.168.0.1 --prefix-length=16
```
This command's result is:
```text
IP Address: 192.168.0.1
Subnet Mask: 255.255.255.0
Prefix Length: /16
Network Address: 192.168.0.0
Broadcast Address: 192.168.255.255
```
