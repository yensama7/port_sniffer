# How to Run
```shell
cargo run -- -j [thread] [ipaddrss]
```
## Example 1
```shell
cargo run -- -j 1000 192.168.0.7
```
Here the 1000 is the number of threads, and the 192.168.0.7 is the ip address

## Example 2
```shell
cargo run -- 192.168.0.7
```
if no thread is specified, the default is 4


### Note: 
 *-h* : help shows e.g cargo run -- -h or cargo -- -help

 *-j* : used to specify number of threads to use