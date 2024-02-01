use axum::{extract, extract:: Path, response::Html, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::error::Error;
use axum::extract::State;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    //include static file
    async fn handler() -> Html<&'static str> {
        Html(include_str!("index.html"))
    }

    // connection to PGSQL database
    let conn_url = "postgresql://localhost:5432";
    let pool: sqlx::Pool<sqlx::Postgres> = sqlx::PgPool::connect(&conn_url).await?;

    // route creation
    let app = Router::new()
    .route("/", get(handler))
    .route("/users", post(create_user))
    .route("/users/login", post(user_login))
    .route("/lair", get(look_for_lair).post(new_form))
    .route("/lair/:id", get(one_lair).delete(delete_form))
    .with_state(pool.clone());

    // server creation
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    //create table of users if not exist
    sqlx::query(
		r#"
CREATE TABLE IF NOT EXISTS users (
    account_name text UNIQUE NOT NULL PRIMARY KEY,
    account_password text NOT NULL
);"#,
	)
	.execute(&pool)
	.await?;

    //create table of annoucement if not exist
    sqlx::query(
		r#"
CREATE TABLE IF NOT EXISTS announcements (
    account_name text references users(account_name),
    id SERIAL UNIQUE PRIMARY KEY, 
    title text NOT NULL, 
    image text NOT NULL, 
    description text NOT NULL, 
    lon DOUBLE PRECISION NOT NULL, 
    lat DOUBLE PRECISION NOT NULL
);"#,
	)
	.execute(&pool)
	.await?;

    Ok(())
}

    //creation struct person
    #[derive(Serialize, Deserialize)]
    struct User{
        account_name: String,
        account_password: String,
    }

    //creation struct new lairs
    #[derive(Serialize, Deserialize)]
    struct Lairs{
        account_name: String,
        id: i64,
        title: String,
        image: String,
        description: String,
        lon: i64,
        lat: i64,
    }

    //creation struct for when they search for a property by looking at map
    #[derive(Serialize, Deserialize)]
    struct SearchingLairs{
        br_lat: i64,
        br_lng: i64,
        tl_lat: i64,
        tl_lng: i64,
        search: String,
        limit: i64,
    }

    //add new user to user table

    async fn create_user(State(pool): State<sqlx::Pool<sqlx::Postgres>>, extract::Json(user): extract::Json<User>) -> Json<(String, String)> {
        let result: (String, String) = sqlx::query_as(
            r#"
        INSERT INTO users (account_name, account_password)
        VALUES ( $1, $2 )
            "#,).bind(user.account_name).bind(user.account_password).fetch_one(&pool).await.unwrap();

            Json(result)
    }

    //user login
    async fn user_login(State(pool): State<sqlx::Pool<sqlx::Postgres>>, extract::Json(user): extract::Json<User>) -> Json<(String, String)> {
        let result: (String, String) = sqlx::query_as(
            r#"
        SELECT account_name, account_password FROM users 
        WHERE account_name = ($1) AND account_password = ($2)
            "#,).bind(user.account_name).bind(user.account_password).fetch_one(&pool).await.unwrap();

            Json(result)
    }

    //looping for a property on the map    
    async fn look_for_lair(State(pool): State<sqlx::Pool<sqlx::Postgres>>,  extract::Json(search_lair): extract::Json<SearchingLairs>) -> Json<Vec<(String, String, String, String, String)>>{
        let result: Vec<(String, String, String, String, String)> = sqlx::query_as(
            r#"
        SELECT id, title, image, lon, lat FROM announcements WHERE (lat > ($1) AND lat < ($2) AND lon > ($3) AND lon < ($4)) AND title LIKE '($5)' LIMIT ($6) OFFSET ($6)
            "#,).bind(search_lair.br_lat).bind(search_lair.tl_lat).bind(search_lair.tl_lng).bind(search_lair.br_lng).bind(search_lair.search).bind(search_lair.limit)
            .fetch_all(&pool).await.unwrap();

        Json(result)
    }

    //JE PENSE QUE CETTE FONCTION EST FOIREUSE    
    async fn new_form(State(pool): State<sqlx::Pool<sqlx::Postgres>>, extract::Json(lair): extract::Json<Lairs>) -> Json<(String, String, String, String, String, String, String)> {
        let result: (String, String, String, String, String, String, String) = sqlx::query_as(
            r#"
        INSERT INTO announcements (account_name, id, title, image, description, lon, lat)
        VALUES ( $1, $2, $3, $4, $5, $6, $7 )
            "#,).bind(lair.account_name).bind(lair.id).bind(lair.title).bind(lair.image).bind(lair.description).bind(lair.lon).bind(lair.lat).fetch_one(&pool).await.unwrap();

        Json(result)
    }

    //looking for a particular property
    async fn one_lair(State(pool): State<sqlx::Pool<sqlx::Postgres>>, extract::Json(lair): extract::Json<Lairs>) -> Json<(String, String, String, String, String, String, String)> {
        let result: (String, String, String, String, String, String, String) = sqlx::query_as(
            r#"
        SELECT * FROM announcements WHERE id = ( $1 )
            "#,).bind(lair.id).fetch_one(&pool).await.unwrap();
        
        Json(result)
    }

    //deleting a property from database
    async fn delete_form(Path(id): Path<i64>, State(pool): State<sqlx::Pool<sqlx::Postgres>>) -> () {
        let result: () = sqlx::query_as(
            r#"
        DELETE FROM announcements WHERE id = ( $1 )
            "#,).bind(id).fetch_one(&pool).await.unwrap();
    }