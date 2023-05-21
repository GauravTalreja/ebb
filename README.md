# ebb
UWaterloo Course Search Tool, CS 348 Project

## local setup

install rust https://www.rust-lang.org/tools/install

install npm

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
```
ssh fedora@IP_ADDRESS
```
(otherwise use any computer connected to the campus network as a jump relay, the student cs servers are used here)
```sh
ssh -J QUESTID@linux.student.cs.uwaterloo.ca fedora@IP_ADDRESS
```
kill the current process (this probably needs to be more graceful)
```sh
sudo pkill server
```
pull your changes and rebuild the app
```sh
cd ebb
git pull
perseus deploy
```
IMPORTANT: Copy the generated styles into the ``pkg/dist`` folder.
```sh
cp dist/tailwind.css pkg/dist/
```
restart the server
```
sudo './pkg/server &'
```
