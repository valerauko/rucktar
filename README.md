# Basic actix-web REST example

Based on example from https://github.com/valerauko/rucktar/tree/rocket-rest-example

## What's in it
* Docker
* actix-web (Rust)
* Postgres

## How does it work?
1. `docker-compose up`
2. `docker-compose exec api sh`
3. Set up `diesel_cli`
  * `cargo install diesel_cli`
  * `diesel setup`
  * `diesel migration run`
4. `cargo run`
5. At this point you can use `curl` to hit the API as you please (see below)

## Example `curl` commands
### Create new Item
```
curl -X POST http://localhost:3014/api/items --header 'Content-Type: application/json' -d '{
  "name": "New Item 1",
  "description": "A new item"
}'
```
This will return the JSON representation of the Item inserted (with ID and a `created_at` timestamp).

### Get item
```
curl http://localhost:3014/api/items/1
```

### List Items
```
curl http://localhost:3014/api/items
```

### Update Item
```
curl -X PATCH http://localhost:3014/api/items/1 --header 'Content-Type: application/json' -d '{
  "description": "A new item that is absolutely fantastic!"
}'
```

### Delete Item
```
curl -X DELETE http://localhost:3014/api/items/1
```

## Catches
Some of the issues I ran into on the way:
* `diesel` doesn't work with the `rust:slim` Docker image. It dies with `error: linking with 'cc' failed: exit code: 1`.
* I had to specify `features` in `Cargo.toml` to get stuff working. I'm pretty sure this shouldn't be necessary, but at least it works.
* Postgres `BigInt` maps to `i32`, not `i64`
* In database-related functions, `use crate::db::schema::items::dsl::*` re-defines all the field-names in the local scope. Took a while to figure out why the compiler said the function parameter `id` wasn't used.
