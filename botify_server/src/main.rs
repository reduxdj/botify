#![allow(non_camel_case_types, non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
use arangors::{uclient::reqwest::ReqwestClient, AqlQuery, Connection, Database};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptyMutation, EmptySubscription, ErrorExtensions, FieldError, FieldResult, Object,
    Schema, SimpleObject,
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use core::str;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::value::Value;
use std::convert::Infallible;
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio;
use tokio::time::sleep;
use warp::{http::Response as HttpResponse, Filter, Rejection};
#[macro_use]
extern crate thiserror;

#[derive(Debug)]
enum ClientError {
    IoError(io::Error),
    CollectionCreationError(String),
}

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Could not find resource")]
    NotFound,

    #[error("Not Authorized")]
    NotAuthorized,

    #[error("ServerError")]
    ServerError(String),

    #[error("No Extensions")]
    ErrorWithoutExtensions,
}

impl ErrorExtensions for MyError {
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            MyError::NotFound => e.set("code", "NOT_FOUND"),
            MyError::ServerError(reason) => e.set("reason", reason.to_string()),
            MyError::NotAuthorized => e.set("code", 401),
            MyError::ErrorWithoutExtensions => {}
        })
    }
}

#[derive(Debug, SimpleObject, Clone, Serialize, Deserialize)]
pub struct Song {
    title: String,
    artist: String,
    active: bool,
    media_url: String,
    image_url: String,
    created_on: i64,
    last_modified: i64,
}
pub struct ContextData {
    db: Database<ReqwestClient>,
}
pub struct Query;
pub struct Mutation;

#[Object(extends)]
impl Query {
    async fn extend(&self) -> FieldResult<i32> {
        Err(MyError::NotAuthorized.extend())
    }
    async fn get_songs(&self, ctx: &Context<'_>) -> FieldResult<Vec<Song>> {
        let db = get_db_handle_from_context(ctx); // Ensure you have a function to extract the database handle from the context
        let resp = db.aql_str("FOR s IN songs RETURN s").await;
        match resp {
            Ok(resp) => {
                let songs: Vec<Song> = resp;

                Ok(songs)
            }
            Err(e) => Err(FieldError::from(e)),
        }
    }
}

#[Object(extends)]
impl Mutation {
    async fn save_song(&self, ctx: &Context<'_>, title: String, artist: String) -> Option<i32> {
        let _db = get_db_handle_from_context(ctx);
        Some(1)
    }
}

pub fn get_db_handle_from_context(ctx: &Context<'_>) -> Database<ReqwestClient> {
    let data = ctx.data::<ContextData>().unwrap();
    let db = data.db.clone();
    db
}

async fn ensure_collection_exists(db: &Database<ReqwestClient>) -> Result<(), ClientError> {
    match db.collection("songs").await {
        Ok(_) => Ok(()),
        Err(_) => match db.create_collection("songs").await {
            Ok(_) => Ok(()),
            Err(e) => Err(ClientError::CollectionCreationError(format!(
                "Failed to create collection: {}",
                e
            ))),
        },
    }
}

fn seed_songs() -> Vec<Song> {
    vec![
        Song {
            title: "Propagation".to_string(),
            artist: "Com Truise".to_string(),
            active: true,
            media_url: "https://example.com/media/propagation".to_string(),
            image_url: "https://i.scdn.co/image/ab6761610000101f9ee8f1bb8c981f3865d8e7c5"
                .to_string(),
            created_on: 1483228800, // Unix timestamp
            last_modified: 1483228800,
        },
        Song {
            title: "Brokendate".to_string(),
            artist: "Com Truise".to_string(),
            active: true,
            media_url: "https://example.com/media/brokendate".to_string(),
            image_url: "https://i.scdn.co/image/ab6761610000101f9ee8f1bb8c981f3865d8e7c5"
                .to_string(),
            created_on: 1483228800, // Unix timestamp
            last_modified: 1483228800,
        },
        Song {
            title: "Air Cal".to_string(),
            artist: "Com Truise".to_string(),
            active: true,
            media_url: "https://example.com/media/air-cal".to_string(),
            image_url: "https://i.scdn.co/image/ab6761610000101f9ee8f1bb8c981f3865d8e7c5"
                .to_string(),
            created_on: 1483228800, // Unix timestamp
            last_modified: 1483228800,
        },
        Song {
            title: "Memory".to_string(),
            artist: "Com Truise".to_string(),
            active: true,
            media_url: "https://example.com/media/memory".to_string(),
            image_url: "https://i.scdn.co/image/ab6761610000101f9ee8f1bb8c981f3865d8e7c5"
                .to_string(),
            created_on: 1483228800, // Unix timestamp
            last_modified: 1483228800,
        },
    ]
}
async fn insert_songs(
    db: &Database<ReqwestClient>,
    songs: Vec<Song>,
) -> Result<(), Box<dyn std::error::Error>> {
    for song in songs {
        let song_json = serde_json::to_value(&song)?;
        let query = AqlQuery::builder()
            .query("INSERT @song INTO songs")
            .bind_var("song", song_json)
            .count(true)
            .build();

        let _ = match db.aql_query::<()>(query).await {
            Ok(_) => println!("Song inserted"),
            Err(e) => {
                eprintln!("Failed to insert song: {:?}", e);
                return Err(Box::new(e));
            }
        };
    }
    Ok(())
}

async fn remove_songs(db: &Database<ReqwestClient>) -> Result<(), Box<dyn std::error::Error>> {
    let query = AqlQuery::builder()
        .query("FOR song IN songs REMOVE song")
        .build();

    let _ = match db.aql_query::<()>(query).await {
        Ok(_) => println!("Songs Removed"),
        Err(e) => {
            eprintln!("Failed to remove song: {:?}", e);
            return Err(Box::new(e));
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting to connect!");
    sleep(Duration::from_secs(5)).await;
    let conn = Connection::establish_basic_auth("http://arangodb:8529", "root", "password")
        .await
        .unwrap();
    match conn.create_database("botify_server").await {
        Ok(_) => println!("Database created successfully"),
        Err(e) => println!("Error creating database: {:?}", e),
    }
    let db = conn.db("botify_server").await.unwrap();
    let _ = ensure_collection_exists(&db).await;
    let _result = remove_songs(&db).await;
    let _songs_to_seed = seed_songs();
    let result = insert_songs(&db, seed_songs()).await;

    match result {
        Ok(doc) => println!("Document inserted successfully: {:?}", doc),
        Err(e) => eprintln!("Failed to insert document: {:?}", e),
    }

    let context = ContextData { db };
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(context)
        .enable_federation()
        .finish();

    println!("{}", &schema.sdl());

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, Mutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes = graphql_playground
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    warp::http::StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    println!("Playground: http://localhost:{}", "8081");
    warp::serve(routes).run(([0, 0, 0, 0], 8081)).await;
    println!("Connected to ArangoDB!");
    Ok(())
}
