use mongodb::bson::oid::ObjectId;

pub fn object_new_hex() -> String {
    ObjectId::new().to_hex()
}