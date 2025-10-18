# How to Run
```shell
cargo run -- -i [ipaddrss] -s [start port] -e [end port]
```
## Example 1
```shell
cargo run -- -i 192.168.0.7 -s 20 -e 80
```
uses async/await to scan the ports

## Example 2
```shell
cargo run -- -i 192.168.0.7
```

if no ipaddress is specified or ipaddress not found, the default is 127.0.0.1
## Example 3
```shell
cargo run
```

### Note: 
 *-h* : help shows e.g cargo run -- -h or cargo -- -help

 *-i* : used to specify ipv4 address

 *-s* : used to specify start port

 *-e* : used to specify end port