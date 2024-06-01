use crate::{configuration::Config, error::Error, network::ApiResponse};

use sea_query::{ColumnDef, Iden, Query, SqliteQueryBuilder, Table};
use sea_query_binder::SqlxBinder;
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};
use std::path::Path;

#[derive(Iden)]
pub enum Responses {
    Table,
    Id,
    DateTime,
    RequestType,
    Url,
    Data,
}

// #[derive(sqlx::FromRow, Debug)]
// pub struct ResponseStruct {
//     id: i64,
//     date_time: DateTime<Utc>,
//     request_type: String,
//     url: String,
//     data: String,
// }

async fn connect(path: &Path) -> Result<SqlitePool, Error> {
    const PREFIX: &str = "sqlite://";
    const POSTFIX: &str = "?mode=rwc";
    let url: String = format!("{}{}{}", PREFIX, path.to_string_lossy(), POSTFIX);

    Ok(SqlitePool::connect(&url).await?)
}
async fn create_table(pool: &SqlitePool) -> Result<SqliteQueryResult, Error> {
    type R = Responses;

    let table = Table::create()
        .table(R::Table)
        .if_not_exists()
        .col(ColumnDef::new(R::Id).integer().not_null().primary_key())
        .col(ColumnDef::new(R::DateTime).date_time().not_null())
        .col(ColumnDef::new(R::RequestType).string().not_null())
        .col(ColumnDef::new(R::Url).string().not_null())
        .col(ColumnDef::new(R::Data).string().not_null())
        .build(SqliteQueryBuilder);

    Ok(sqlx::query(&table).execute(pool).await?)
}
async fn insert(pool: &SqlitePool, api_response: &ApiResponse) -> Result<SqliteQueryResult, Error> {
    let ApiResponse {
        date_time,
        request_type,
        url,
        data,
        ..
    } = api_response;

    type R = Responses;

    let (sql, values) = Query::insert()
        .into_table(R::Table)
        .columns([R::DateTime, R::RequestType, R::Url, R::Data])
        .values_panic([
            date_time
                .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
                .into(),
            request_type.to_string().into(),
            url.to_string().into(),
            data.into(),
        ])
        .build_sqlx(SqliteQueryBuilder);

    Ok(sqlx::query_with(&sql, values).execute(pool).await?)
}

pub async fn write(
    config: &Config,
    api_response: &ApiResponse,
) -> Result<SqliteQueryResult, Error> {
    let pool = connect(config.get_db_path()).await?;
    create_table(&pool).await?;
    Ok(insert(&pool, api_response).await?)
}
