use diesel::{insert_into, prelude::*, result::QueryResult};
use chrono::{DateTime, Utc};

use super::schema::logs::dsl;
use super::model::*;

pub fn logs(
    cn: &PgConnection,
    from: Option<DateTime<Utc>>,
    until: Option<DateTime<Utc>>,
) -> QueryResult<Vec<Log>> {
    // 型エラーを防ぐために into_boxed を呼んでおく
    let mut query = dsl::logs.into_boxed();
    if let Some(from) = from {
        query = query.filter(dsl::timestamp.ge(from.naive_utc()))
    }
    if let Some(until) = until {
        query = query.filter(dsl::timestamp.lt(until.naive_utc()))
    }
    query.order(dsl::timestamp.asc()).load(cn)
}

pub fn insert_log(cn: &PgConnection, log: &NewLog) -> QueryResult<i64> {
    insert_into(dsl::logs)
        .values(log)
        .returning(dsl::id)
        .get_result(cn)
}

pub fn insert_logs(cn: &PgConnection, logs: &[NewLog]) -> QueryResult<Vec<i64>> {
    insert_into(dsl::logs)
        .values(logs)
        .returning(dsl::id)
        .load(cn)
}