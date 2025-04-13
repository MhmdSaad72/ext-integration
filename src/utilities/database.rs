use actix_web::web::Data;
use diesel::{r2d2::ConnectionManager, PgConnection};

use crate::{errors::app_error::AppError, DbPool};

pub fn get_db_connection(
    conn: Data<DbPool>,
) -> Result<diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>, AppError> {
    conn.get().map_err(|_| AppError::DatabaseError {
        field: "connection".into(),
        source: diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new("Failed to get database connection".to_string()),
        ),
    })
}
