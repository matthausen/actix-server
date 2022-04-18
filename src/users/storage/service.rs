// The storage service. Contains
// - tableName
// - interface / trait (with PutItem)

#[async_trait::async_trait]
trait DynamoDB {
    async fn put_item(&self)
}
pub struct Storage {
    table_name: String
}