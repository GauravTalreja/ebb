# ebb
UWaterloo Course Search Tool, CS 348 Project

## local setup

install rust https://www.rust-lang.org/tools/install

install npm https://nodejs.org/en/download/package-manager

install perseus-cli
```
cargo install perseus-cli
```

clone the repo and then
```
cd ebb
```

install npm packages (this only needs to be done one time after cloning the repo)
```
npm install
```

compile and run locally with
```
perseus serve -w
```

## deploy

ssh into the vm

(if you're on campus/using the vpn)
```sh
ssh fedora@IP_ADDRESS
```
(otherwise use any computer connected to the campus network as a jump relay, the student cs servers are used here)
```sh
ssh -J QUESTID@linux.student.cs.uwaterloo.ca fedora@IP_ADDRESS
```
run the deploy script

```sh
cd ebb
./deploy.sh
```
