use diesel::prelude::*;
use diesel::upsert::excluded;

fn main() {
    use hello::models::User;
    use hello::schema::users::dsl::*;

    let conn = hello::establish_connection();

    let user = User {
        id: 2,
        name: String::from("John"),
    };
    let query = diesel::insert_into(users)
        .values(&user)
        .on_conflict(id)
        .do_update()
        .set(name.eq(excluded(name)));
    println!(
        "{}",
        diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query).to_string()
    );
    let data = query.execute(&conn).unwrap();

    dbg!(data);
}
