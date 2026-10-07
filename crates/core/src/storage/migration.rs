//! Schema migrations, applied in order on load: v1 → v2 → … → current.

use super::schema::CURRENT_SCHEMA_VERSION;
use crate::CoreError;

/// Upgrades raw JSON to the current schema version.
///
/// v1 is the first schema, so there is nothing to migrate yet. Each future
/// version adds one `migrate_vN_to_vN1` step here (first expected in v2.0).
pub fn migrate(mut value: serde_json::Value) -> Result<serde_json::Value, CoreError> {
    let version = value
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let version = u32::try_from(version).unwrap_or(u32::MAX);

    if version > CURRENT_SCHEMA_VERSION {
        return Err(CoreError::UnsupportedSchema(version));
    }
    if let Some(obj) = value.as_object_mut() {
        obj.insert("schema_version".into(), CURRENT_SCHEMA_VERSION.into());
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_newer_schema() {
        let v = serde_json::json!({ "schema_version": CURRENT_SCHEMA_VERSION + 1 });
        assert!(matches!(migrate(v), Err(CoreError::UnsupportedSchema(_))));
    }

    #[test]
    fn current_schema_passes_through() {
        let v = serde_json::json!({ "schema_version": CURRENT_SCHEMA_VERSION });
        assert!(migrate(v).is_ok());
    }
}
