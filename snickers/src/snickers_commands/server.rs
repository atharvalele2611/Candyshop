use crate::database::Database;

pub(crate) fn flushdb_command(
    db: &mut Database,
    _database_key: &str,
    _request: &[&str],
) -> Result<String, String> {
    db.clear();
    Ok("OK\n".to_string())
}
