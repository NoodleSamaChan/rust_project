use axum::{Router, routing::{get, post, delete}, extract::Path, response::Html};
use sqlx::{postgres::{PgPoolOptions, PgRow}, PgPool};
use sqlx::{FromRow, Row};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    //creation struct person
    struct User{
        account_name: String,
        account_password: String,
    }

    //creation struct new lairs
    struct Lairs{
        account_name: String,
        id: usize,
        title: String,
        image: String,
        description: String,
        lon: usize,
        lat: usize,
    }
    //include static file
    async fn handler() -> Html<&'static str> {
        Html(include_str!("index.html"))
    }

    // route creation
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/users/login", post(user_login))
        .route("/lair", get(look_for_lair).post(new_form))
        .route("/lair/:id", get(one_lair).delete(delete_form));

    // server creation
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // connection to PGSQL database
    let conn_url = "postgresql://localhost:5432";
    let pool = sqlx::PgPool::connect(&conn_url).await?;

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

    //add new user to user table

    async fn create_user(pool: &PgPool, user: User) {
        sqlx::query_as(
            r#"
    INSERT INTO users (user)
    VALUES ( $1 )
    RETURNING id
            "#,
            Json(person) as _
        )
        .fetch_one(pool)
        .await?;
    
    }
        
    async fn user_login() {}
        
    async fn look_for_lair() {}
        
    async fn new_form() {}

    async fn one_lair() {}

    async fn delete_form(Path(id): Path<u64>) {}

    Ok(())
}