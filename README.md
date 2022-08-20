# simple-api-rust
A simple api writing in rust. This api was made to learn the basic of rust and web development.

### Libraries
I choose the [Rocket](https://rocket.rs/) as a web framework. It's easy to use, lightweight and with a lot of docs.

For persistence layer, ORM and query builder, I used [diesel](https://diesel.rs/) with
[r2d2](https://github.com/sfackler/r2d2) as a pool connection. At the moment this app only supports SQLite.
In order to use the migrations you need to install diesel-cli
`cargo install diesel_cli --no-default-features --features sqlite`.

For Openapi 3.0 and Swagger generation I found [utopia](https://github.com/juhaku/utoipa) a good enough solution.

### Usage
Make sure you setup **rust nightly** compiler to build the app.
I strongly recommend to use [rustup](https://rustup.rs/) to configure it.
To start using run the followings commands in order
```bash
make generate-database
make generate-envs
make run
```

Then you can open any browser go to http://localhost:8081/swagger/index.html and start play around.

### Makefile
A Makefile is provided with the following goals.
* Create environments files
    ```bash
    make generate-envs
    ```
* Create the databases files and directory
    ```bash
    make generate-database
    ```
* Build with debug symbols
    ```bash
    make debug
    ```
* Build for release
    ```bash
    make release
    ```
* Run
    ```bash
    make run
    ```
* Tests
    ```bash
    make test
    ```
* Format the source code
    ```bash
    make fmt
    ```
* Docker related
    ```bash
    todo!
    ```

Hope this help you and happy hacking.
