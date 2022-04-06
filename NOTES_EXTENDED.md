# NOTES EXTENDED

## Add Dead Pool Postgres to Starter

- https://docs.rs/deadpool-postgres/latest/deadpool_postgres/#example-using-an-existing-tokio_postgresconfig-object

### Create Docker Compose File

TODO:

### Add Start Sql Files

`sql/add_user.sql`

```sql
INSERT INTO testing.users(email, first_name, last_name, username)
VALUES ($1, $2, $3, $4)
RETURNING $table_fields;
```

`sql/delete_user.sql`

```sql
DELETE FROM testing.users 
WHERE username = $1
```


`sql/get_users.sql`

```sql
SELECT $table_fields FROM testing.users $where;
```

`sql/schema.sql`

```sql
DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE testing.users (
  id  BIGSERIAL PRIMARY KEY,
  email       VARCHAR(200) NOT NULL,
  first_name  VARCHAR(200) NOT NULL,
  last_name   VARCHAR(200) NOT NULL,
  username    VARCHAR(50) UNIQUE NOT NULL,
  UNIQUE (username)
);
```

`sql/update_user.sql`

```sql
UPDATE
  testing.users
SET
  email = $2,
  first_name = $3,
  last_name = $4
WHERE
  username = $1
RETURNING 
  $table_fields;
```

### Add to Cargo.toml

```toml
# postgres
tokio-pg-mapper = "0.2.0"
tokio-pg-mapper-derive = "0.2.0"
tokio-postgres = { version = "0.7.5", features = ["with-uuid-0_8"] }
deadpool-postgres = { version = "0.10.1", features = ["serde"] }
# config
config = "0.11.0"
dotenv = "0.15.0"
```

### Add to Source

`src/app/constants.rs`

```rust
// postgres
pub const DEFAULT_PG_USER: &'static str = "test_user";
pub const DEFAULT_PG_PASSWORD: &'static str = "testing";
pub const DEFAULT_PG_HOST: &'static str = "127.0.0.1";
pub const DEFAULT_PG_PORT: u16 = 5432;
pub const DEFAULT_PG_DBNAME: &'static str = "testing_db";
pub const DEFAULT_PG_POOL_MAX_SIZE: u64 = 16;
```

`src/app/db.rs`

```rust
use crate::{
  app::postgres_error::PostgresError,
  models::{Filter, User},
};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn add_user(client: &Client, user_info: User) -> Result<User, PostgresError> {
  let _stmt = include_str!("../../sql/add_user.sql");
  let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
  let stmt = client.prepare(&_stmt).await.unwrap();

  client
    .query(&stmt, &[&user_info.email, &user_info.first_name, &user_info.last_name, &user_info.username])
    .await?
    .iter()
    .map(|row| User::from_row_ref(row).unwrap())
    .collect::<Vec<User>>()
    .pop()
    // more applicable for SELECTs
    .ok_or(PostgresError::NotFound)
}

pub async fn get_users(client: &Client, filter: Option<Filter>) -> Result<Vec<User>, PostgresError> {
  let _stmt = include_str!("../../sql/get_users.sql");
  let mut _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
  match filter {
    Some(value) => _stmt = _stmt.replace("$where", format!("WHERE {}", value.condition).as_str()),
    None => _stmt = _stmt.replace("$where", ""),
  }

  let stmt = client.prepare(&_stmt).await.unwrap();

  let res = client.query(&stmt, &[]).await?.iter().map(|row| User::from_row_ref(row).unwrap()).collect::<Vec<User>>();
  // manually wrap the Vec<User> that comes from collect() in a Result::Ok
  Ok(res)
}

pub async fn update_user(client: &Client, user_info: User) -> Result<User, PostgresError> {
  let _stmt = include_str!("../../sql/update_user.sql");
  let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
  let stmt = client.prepare(&_stmt).await.unwrap();

  client
    .query(
      &stmt,
      &[
        &user_info.username,
        &user_info.email,
        &user_info.first_name,
        &user_info.last_name,
      ],
    )
    .await?
    .iter()
    .map(|row| User::from_row_ref(row).unwrap())
    .collect::<Vec<User>>()
    .pop()
    // more applicable for SELECTs
    .ok_or(PostgresError::NotFound)
}

pub async fn delete_user(client: &Client, username: String) -> Result<(), PostgresError> {
  let _stmt = include_str!("../../sql/delete_user.sql");

  let stmt = client.prepare(&_stmt).await.unwrap();

  let _res = client.query(&stmt, &[&username]).await?;
  Ok(())
}
```

`src/app/postgres_error.rs`

```rust
use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

#[derive(Display, From, Debug)]
pub enum PostgresError {
  NotFound,
  PGError(PGError),
  PGMError(PGMError),
  PoolError(PoolError),
}

impl std::error::Error for PostgresError {}

impl ResponseError for PostgresError {
  fn error_response(&self) -> HttpResponse {
    match *self {
      PostgresError::NotFound => HttpResponse::NotFound().finish(),
      PostgresError::PoolError(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
      _ => HttpResponse::InternalServerError().finish(),
    }
  }
}
```

`src/models/filter.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Filter {
  pub condition: String,
}
```

`src/models/user.rs`

```rust
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
// singular 'user' is a keyword..
pub struct User {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub username: String,
}
```

`servetr/handlers.rs`

```rust
#[post("/users")]
pub async fn add_user(user: web::Json<User>, db_pool: web::Data<Pool>) -> Result<HttpResponse, ActixError> {
  let user_info: User = user.into_inner();

  let client: PostgresClient = db_pool.get().await.map_err(PostgresError::PoolError)?;

  match db::add_user(&client, user_info).await {
    Ok(new_user) => Ok(HttpResponse::Ok().json(new_user)),
    Err(_e) => {
      // error!("error {:?}", e);
      Ok(HttpResponse::Conflict().json(MessageResponse {
        message: format!("{}", I18N_CANT_CREATE_RECORD),
      }))
    }
  }
}

#[get("/users")]
pub async fn get_users(filter: Option<web::Json<Filter>>, db_pool: web::Data<Pool>) -> Result<HttpResponse, ActixError> {
  let mut filter_info: Option<Filter> = None;
  // override none
  if let Some(value) = filter {
    filter_info = Some(value.into_inner())
  };

  let client: PostgresClient = db_pool.get().await.map_err(PostgresError::PoolError)?;

  let users = db::get_users(&client, filter_info).await?;

  Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{username}")]
pub async fn get_user(path: web::Path<String>, db_pool: web::Data<Pool>) -> Result<HttpResponse, ActixError> {
  let username = path.into_inner();
  let filter_info: Option<Filter> = Some(Filter {
    condition: format!("users.username = '{}'", username),
  });

  let client: PostgresClient = db_pool.get().await.map_err(PostgresError::PoolError)?;

  let user = db::get_users(&client, filter_info).await?;

  if user.len() > 0 {
    Ok(HttpResponse::Ok().json(user.get(0)))
  } else {
    Ok(HttpResponse::NotFound().json(MessageResponse {
      message: format!("{}", I18N_RECORD_NOT_FOUND),
    }))
  }
}

#[put("/users")]
pub async fn update_user(
  user: web::Json<User>,
  db_pool: web::Data<Pool>,
) -> Result<HttpResponse, ActixError> {
  let user_info: User = user.into_inner();

  let client: PostgresClient = db_pool.get().await.map_err(PostgresError::PoolError)?;

  match db::update_user(&client, user_info).await {
    Ok(update_user) => Ok(HttpResponse::Ok().json(update_user)),
    Err(_e) => {
      // error!("error {:?}", e);
      Ok(HttpResponse::Conflict().json(MessageResponse {
        message: format!("{}", I18N_CANT_UPDATE_RECORD),
      }))
    }
  }
}

#[delete("/users/{username}")]
pub async fn delete_user(path: web::Path<String>, db_pool: web::Data<Pool>) -> Result<HttpResponse, ActixError> {
  let username = path.into_inner();

  let client: PostgresClient = db_pool.get().await.map_err(PostgresError::PoolError)?;

  let filter_info: Option<Filter> = Some(Filter {
    condition: format!("users.username = '{}'", username),
  });
  let user = db::get_users(&client, filter_info).await?;

  if user.len() > 0 {
    db::delete_user(&client, username).await?;
    Ok(HttpResponse::Ok().finish())
  } else {
    Ok(HttpResponse::NotFound().json(MessageResponse {
      message: format!("{}", I18N_RECORD_NOT_FOUND),
    }))
  }
}
```

`main.rs`

```rust
...
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  ...
  // config postgres
  let mut pg_config = tokio_postgres::Config::new();
  pg_config.user(env::var("PG_USER").unwrap_or(DEFAULT_PG_USER.to_string()).as_str());
  pg_config.password(env::var("PG_PASSWORD").unwrap_or(DEFAULT_PG_PASSWORD.to_string()).as_str());
  pg_config.host(env::var("PG_HOST").unwrap_or(DEFAULT_PG_HOST.to_string()).as_str());
  pg_config.port(env::var("PG_PORT").unwrap_or(DEFAULT_PG_PORT.to_string()).parse::<i16>().unwrap() as u16);
  pg_config.dbname(env::var("PG_DBNAME").unwrap_or(DEFAULT_PG_DBNAME.to_string()).as_str());
  let mgr_config = ManagerConfig {
    recycling_method: RecyclingMethod::Fast,
  };
  let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
  let pool = Pool::builder(mgr)
    .max_size(env::var("PG_POOL_MAX_SIZE").unwrap_or(DEFAULT_PG_POOL_MAX_SIZE.to_string()).parse::<usize>().unwrap())
    .build()
    .unwrap();
  // If you want your application to crash on startup if no database connection can be established just call pool.get().await right after creating the pool.
  match pool.get().await {
    Ok(_) => info!("database connection can be established"),
    Err(e) => error!("{:?}", e),
  }
  ...

  HttpServer::new(move || {
    ...
    App::new()
      ...
      // inject dbPool
      .app_data(web::Data::new(pool.clone()))
```

### Add to Client Http

`client.http`

```conf
### getAllUsers
// @name getAllUsers

GET {{uri}}/users HTTP/1.1
Content-Type: {{contentType}}
Authorization: Bearer {{apiKey}}

{
}

### getFilteredUsers
// @name getFilteredUsers

GET {{uri}}/users HTTP/1.1
Content-Type: {{contentType}}
Authorization: Bearer {{apiKey}}

{
  "condition": "users.email = 'mail@koakh.com'"
}

### getUser
// @name getUser

GET {{uri}}/users/ferreal HTTP/1.1
Content-Type: {{contentType}}
Authorization: Bearer {{apiKey}}

### createUser
// @name createUser

POST {{uri}}/users HTTP/1.1
Content-Type: {{contentType}}
Authorization: Bearer {{apiKey}}

{
  "username": "koakh28",
  "email": "mail@koakh.com",
  "first_name": "Mario", 
  "last_name": "Monteiro"
}

### updateUser
// @name updateUser

PUT {{uri}}/users HTTP/1.1
Content-Type: {{contentType}}
Authorization: Bearer {{apiKey}}

{
  "username": "koakh",
  "email": "koah@koakh.com",
  "first_name": "Koakh", 
  "last_name": "Koakh"
}

### deleteUser
// @name deleteUser

DELETE {{uri}}/users/koakh28 HTTP/1.1
Content-Type: {{contentType}}
Authorization: Bearer {{apiKey}}
```

### Add to make File

> optional

```
PG_USER := ""
PG_PASSWORD := ""
PG_HOST := ""
PG_PORT := ""
PG_DBNAME := ""
PG_POOL_MAX_SIZE := ""
```