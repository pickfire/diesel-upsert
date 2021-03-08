Diesel upsert experiment
========================

Previous: https://github.com/pickfire/diesel-join-not-exist

This time it requires diesel-git which isn't released.

Rust not able to recommend `users::table` when `hello::schema::users` is used
even though it is a relative re-export.

`on_conflict` gets runtime error instead of compile-time error when non
primary key or unique key is used. TODO

I was wondering how upsert could be done but is still surprised that all these
could be made type-safe although there are a little bit of edge cases not being
covered.

The example I tested out here have 3 struct (based on the previous ones).
sqlite was used for easy testing.

    +---------+     +---------+     +---------+
    | User    |<-+  | Post    |     | Comment |
    +---------+  |  +---------+     +---------+
    | id      |  |  | id      |     | id      |
    | name    |  |  | title   |     | body    |
    |         |  |  | body    |<----+ post_id |
    |         |  |  | user_id +--+--+ user_id |
    +---------+  |  +---------+  |  +---------+
                 +---------------+

I want to find all posts that does not have any comments. The query,

```rust
let query = diesel::insert_into(users)
    .values(&user)
    .on_conflict(id)
    .do_update()
    .set(name.eq(excluded(name)));
```

Which results in the SQL query,

```sql
INSERT INTO `users` (`id`, `name`)
VALUES (?, ?)
   ON CONFLICT (`id`)
   DO UPDATE SET `name` = excluded.`name` -- binds: [2, "John"]
```

## Get started

Rust, diesel_cli (with `sqlite` feature) is required.

```
$ diesel migration run
$ cargo run --bin init  # populate database
$ cargo run --bin hello  # multi join query
```
