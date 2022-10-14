//! A level of obstraction over indexed_db_futures that
//! allows us to store file objects in indexed_db
//!
//! This crate will be used to store files, retrieve files
//! and delete files from indexed_db

use crate::editor::EditorFile;
use indexed_db_futures::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::DomException;

pub struct Entry {
    pub key: String,
    pub value: String,
}

impl From<EditorFile> for Entry {
    fn from(file: EditorFile) -> Self {
        Self {
            key: file.name,
            value: file.content,
        }
    }
}

/// Open a connection to the indexed_db
pub async fn open_db() -> Result<IdbDatabase, DomException> {
    let mut db_req = IdbDatabase::open_u32("database", 1)?;
    db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
        // Check if the object store exists; create it if it doesn't
        if !evt.db().object_store_names().any(|n| &n == "files") {
            evt.db().create_object_store("files")?;
        }
        Ok(())
    }));

    let db = db_req.into_future().await?;
    Ok(db)
}

/// PUT a file into the indexed_db
/// This will store `file.content` at index `file.name`
/// This will overwrite any existing entry at `file.name`
pub async fn put_file(db: &IdbDatabase, file: EditorFile) -> Result<(), DomException> {
    let tx = db.transaction_on_one_with_mode("files", IdbTransactionMode::Readwrite)?;
    let store = tx.object_store("files")?;
    store.put_key_val::<JsValue, JsValue>(&file.name.into(), &file.content.into())?;
    tx.await.into_result()
}

/// GET a file from the indexed_db
/// This will retrieve the file at index `name`
/// Wil translate the retrieved value into a `EditorFile`
pub async fn get_file(db: &IdbDatabase, key: String) -> Result<Option<EditorFile>, DomException> {
    let tx = db.transaction_on_one("files")?;
    let store = tx.object_store("files")?;

    let value: Option<JsValue> = store.get_owned(&key)?.await?;
    let value = value.map(|v| v.as_string().unwrap());

    Ok(value.map(|v| EditorFile {
        name: key,
        content: v,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_open_db() {
        let db = open_db().await.unwrap();
        assert_eq!(db.name(), "database");
    }

    #[wasm_bindgen_test]
    pub async fn test_put_file() {
        // Open the database
        let db = open_db().await.unwrap();
        // Create a file, and put it in the database
        let file = EditorFile::new("test".to_string(), "test".to_string());
        put_file(&db, file).await.unwrap();

        // get the same file, check contents are same
        let file = get_file(&db, "test".to_string()).await.unwrap().unwrap();
        assert_eq!(file.name, "test");
        assert_eq!(file.content, "test");
    }
}
