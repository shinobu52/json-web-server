use actix_web::{web::{self, Json, Query}, HttpResponse, Responder, Error};
use ::csv as ext_csv;
use self::ext_csv::Writer;
use diesel::PgConnection;
use log::debug;
use chrono::{DateTime, Utc};


// TODO: 見直し
use actix_web_multipart_file::{Multiparts, FormData};
use futures::stream::Stream;
use itertools::Itertools;

use super::Server;
use api::{csv, logs};
use super::db;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

// GET /logs
pub async fn handle_get_logs(
    server: web::Data<Server>,
    // クエリパラメータは Query<T> を引数に書くと自動的にデシリアライズされて渡される
    range: Query<logs::get::Query>
) -> impl Responder {
    let conn = server.pool
        .get()
        .expect("Can't get Server connection pool");
    let logs = db::logs(&conn, range.from, range.until)
        .expect("Failed to get log");
    let logs = logs.into_iter().map(|log| api::Log {
        user_agent: log.user_agent,
        response_time: log.response_time,
        timestamp: DateTime::from_utc(log.timestamp, Utc),
    })
    .collect();

    HttpResponse::Ok().json(logs::get::Response(logs))
}

// // POST /logs
pub async fn handle_post_logs(
    server: web::Data<Server>,
    // POST のボディは Json<T> を引数に書くと自動的にデシリアライズされて渡される
    log: Json<logs::post::Request>
) -> impl Responder {
    use super::model::NewLog;

    let log = NewLog {
        user_agent: log.user_agent.clone(),
        response_time: log.response_time,
        timestamp: log.timestamp.unwrap_or_else(|| Utc::now()).naive_utc(),
    };
    let conn = server.pool
        .get()
        .expect("Can't get Server connection pool");
    db::insert_log(&conn, &log).expect("Failed to insert log");

    debug!("received log: {:?}", log);
    HttpResponse::Accepted().finish()
}

// // GET /csv
pub async fn handle_get_csv(
    server: web::Data<Server>,
    range: Query<csv::get::Query>
) -> impl Responder {
    let conn = server.pool.get()
        .expect("Can't get Server connection pool");
    let logs = db::logs(&conn, range.from, range.until)
        .expect("Failed to get log data to convert to csv");
    let v = Vec::new();
    let mut w = Writer::from_writer(v);

    // logs VectorをmoveしてIteratorに変換
    for log in logs.into_iter().map(|log| api::Log {
        user_agent: log.user_agent,
        response_time: log.response_time,
        timestamp: DateTime::from_utc(log.timestamp, Utc),
    }) {
        w.serialize(log).expect("Failed to write log data");
    }

    // CSV ファイルはバイナリデータにして返す
    // Writer<Vec<u8>> -> Vec<u8>
    let csv = w.into_inner().unwrap();
    HttpResponse::Ok().append_header(("Content-Type", "text/csv")).body(csv)
}

// // POST /csv
// pub async fn handle_post_csv(
//     server: web::Data<Server>,
//     mut multiparts: Multiparts,
// ) -> impl Responder {
//     let fut = multiparts.from_err()
//         .filter(|field| field.content_type == "text/csv")
//         .filter_map(|field|
//             match field.form_data {
//                 FormData::File { file, .. } => Some(file),
//                 FormData::Data { .. } => None,
//             }
//         )
//         .and_then(move |file|
//             load_file(
//                 &*server.pool.get().expect(""),
//                 file
//             )
//         )
//         .fold(0, |acc, x| Ok::<_, Error>(acc + x))
//         .map(|sum|
//             HttpResponse::Ok().json(api::csv::post::Response(sum))
//         );

//     HttpResponse::Ok().into(fut)
// }

// fn load_file(conn: &PgConnection, file: impl Read) -> Result<usize, Error> {
//     use super::model::NewLog;

//     let mut ret = 0;
//     // CSV ファイルが渡される csv::Reader を用いて api::Logにデコードしていく
//     let in_csv = BufReader::new(file);
//     let in_log = csv::Reader::from_reader(in_csv)
//         .into_deserialize::<api::Log>();
//     // Itertools のchunks を用いて 1000件ずつ処理する
//     for logs in &in_log.chunks(1000) {
//         let logs = logs.filter_map(Result::ok)
//             .map(|log| NewLog {
//                 user_agent: log.user_agent,
//                 response_time: log.response_time,
//                 timestamp: log.timestamp.naive_utc(),
//             })
//             .collect_vec();

//         let inserted = db::insert_logs(conn, &logs)?;
//         ret += inserted.len();
//     }
//     Ok(ret)
// }
