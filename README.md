# Basic Rocket REST example

I couldn't find any Rocket examples that used Postgres, so here we go.

## What's in it
* Docker
* Rocket (Rust)
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
curl -X POST http://localhost:3014/api/items -d '{
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
curl -X PATCH http://localhost:3014/api/items/1 -d '{
  "description": "A new item that is absolutely fantastic!"
}'
```

### Delete Item
```
curl -X DELETE http://localhost:3014/api/items/1
```
