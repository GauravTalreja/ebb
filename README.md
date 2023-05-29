# ebb
UWaterloo Course Search Tool, CS 348 Project

## Requirements

1. [Rust](https://www.rust-lang.org/tools/install)
2. [npm](https://nodejs.org/en/download/package-manager)
3. [Docker](https://docs.docker.com/get-docker/)
4. [perseus-cli](https://crates.io/crates/perseus-cli)
    ```sh
    cargo install perseus-cli
    ```
5. [sqlx-cli](https://crates.io/crates/sqlx-cli)
    ```sh
    cargo install sqlx-cli --no-default-features --features native-tls,postgres
    ```

## Setup

1. Clone the repository.
    ```sh
    git clone git@github.com:GauravTalreja/ebb.git
    cd ebb
    ```

2. Install npm dependencies. This is required each time  ``package-lock.json``  changes. It is unlikely that it will change in the future as we are only using npm for [DaisyUI](https://daisyui.com). 
    ```sh
    npm install
    ```
3. Make a new file named ``.env`` with the following contents.
    ```
    DATABASE_URL=postgres://ebb:ebb@localhost:5432/ebb
    ```
4. Run a local Postgres container.
    ```
    docker compose up
    ```
5. Run ebb migrations. This is necessary because all sqlx macro queries are checked at compile time, and that requires access to a database with the correct schema. It is possible to build without this by removing ``DATABASE_URL`` from the ``.env`` file, in which case sqlx relies on the ``sqlx-data.json`` file. However, you probably want to connect to an actual database during development to test your changes.
    ```
    sqlx migrate run
    ```

## Development

Run a local Postgres container.
```
docker compose up
```

Check your changes as you make them.

```
perseus check -w
```

Build and view your changes in a browser as you make them.

```
perseus serve -w
```

If you want proper syntax highlighting and IDE features for anything annoted with the ``#[cfg(client)]`` macro, change ``"engine"`` to ``"client"`` in ``.cargo/config.toml``. You'll need to change it back if you want it for engine code.

Read about other helpful commands in the [Perseus documentation](https://framesurge.sh/perseus/en-US/docs/0.4.x/first-app/dev-cycle).


## Deployment

Connect to the vm.

(if you're on campus/using the vpn)
```sh
ssh fedora@IP_ADDRESS
```
(otherwise use any computer connected to the campus network as a jump relay, the student cs servers are used here)
```sh
ssh -J QUESTID@linux.student.cs.uwaterloo.ca fedora@IP_ADDRESS
```
Run the deploy script

```sh
cd ebb
./deploy.sh
```
