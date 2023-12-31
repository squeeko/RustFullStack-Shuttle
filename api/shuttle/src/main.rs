use actix_web::web::ServiceConfig;

use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

// #[get("/")]
// async fn hello_world() -> &'static str {
//     "Hello World!"
// }

// // #[get("/version")]
// // async fn version() -> &'static str {
// //     "version = 0.0.0"
// // }

// #[get("/version")]
// async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
//     tracing::info!("Getting version");
//     let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
//         .fetch_one(db.get_ref())
//         .await;

//     match result {
//         Ok(version) => version,
//         Err(e) => format!("Error: {:?}", e),
//     }
// }

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let pool = actix_web::web::Data::new(pool);
    // let config = move |cfg: &mut ServiceConfig| {
    //     // cfg.service(hello_world).service(version);
    //     cfg.app_data(pool).service(health);
    // };

    let config = move |cfg: &mut ServiceConfig| {
        // cfg.service(hello_world).service(version);

        // Using the configure method to move endpoints around
        // cfg.app_data(pool).configure(|c| {
        //     c.service(health);
        // });

        // Refactor  to use the configure method

        cfg.app_data(pool).configure(api_lib::health::service);
    };

    Ok(config.into())
}
