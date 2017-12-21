This project depend on Rust nightly

# How to run

1. setup development environment, you need install [Redis](https://redis.io), [Postgresql](https://www.postgresql.org/) and [Rust(nightly)](https://www.rust-lang.org/en-US/)

2. setup configuration, there is a sample configuration at src/config.toml.sample

      ````
      cp src/config.toml.sample src/config.toml
      ````


3. setup database, you need to install [diesel](http://diesel.rs/)

      ````
      copy .env.example .env

      cargo install diesel_cli

      diesel setup

      diesel run
      ````

4. run 

      ````
      cargo run
      ````
