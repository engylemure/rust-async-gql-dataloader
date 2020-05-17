# rust-graphql-with-dataloader

A Rust GraphQL API with [actix-web](https://github.com/actix/actix-web), [juniper](https://github.com/graphql-rust/juniper), [sqlx](https://crates.io/crates/sqlx) and [dataloader](https://github.com/cksac/dataloader-rs)

## Starting
Just run, and wait for a while since the initial compilation is slow...
```
docker-compose up -d 
```

but after the start the graphql api will be accessible at http://localhost:8080/ with POST http method
you can access a playground at your browser in http://localhost:8080/

## Development

Consider that for running the project in development mode you should use these commands 
for starting the database and the Container for building and starting the application.
### Initial build for the Images.
```
docker-compose build
```
### Initialize the Database
```
docker-compose up -d db
```
### Initialize the API container attaching the bash command
```
docker-compose run api bash
```
### At the bash inside the container
```
cargo build
```
### For running in the attached mode
```
cargo run
```

## Production 

For running in the production mode just execute the command:
```
docker-compose up -d 
```