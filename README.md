# ebb
UWaterloo Course Search Tool, CS 348 Project

## local setup

install rust https://www.rust-lang.org/tools/install

install perseus-cli

```
cargo install perseus-cli
```

clone the repo and then

```
cd ebb
```
compile and run locally with

```
perseus serve -w
```

## deploy

ssh into the vm

(if you're on campus/using the vpn)
```
ssh fedora@IP_ADDRESS
```
(otherwise use any computer connected to the campus network as a jump relay, the student cs servers are used here)
```
ssh -J QUESTID@linux.student.cs.uwaterloo.ca fedora@IP_ADDRESS
```
kill the current process (this probably needs to be more graceful)
```
sudo pkill server
```
pull your changes and rebuild the app
```
cd ebb
git pull
perseus deploy
```
restart the server
```
sudo './pkg/server &'
```
