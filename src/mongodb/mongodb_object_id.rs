use mongodb::bson::oid::ObjectId;

pub fn object_new_hex() -> String {
    ObjectId::new().to_hex()
}

pub use option_hex_string_as_object_id::{
    serialize as serialize_option_hex_string_to_object_id,
    deserialize as deserialize_option_hex_string_from_object_id
};

mod option_hex_string_as_object_id {
    use mongodb::bson::Bson;
    use mongodb::bson::oid::ObjectId;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde::ser::Error as SerError;
    use serde::de::Error as DeError;

    pub fn serialize<S: Serializer>(
        value: &Option<String>, serializer: S
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(hex_id) => {
                ObjectId::parse_str(&hex_id)
                    .map(|object_id| object_id.serialize(serializer))
                    .map_err(SerError::custom)?
            }
            None => { serializer.serialize_none() }
        }
    }

    pub fn deserialize<'d, D: Deserializer<'d>>(
        deserializer: D
    ) -> Result<Option<String>, D::Error> {
         match Bson::deserialize(deserializer)? {
             Bson::ObjectId(object_id) => {
                 Ok(Some(object_id.to_hex()))
             }
             Bson::Null => { Ok(None) }
             _ => {
                 Err(DeError::custom("Invalid parameter type"))
             }
         }
    }

}

#[cfg(test)]
mod test_option_hex_string_as_object_id {
    use mongodb::bson::oid::ObjectId;
    use Rocket::serde::Serialize;
    use serde::Deserialize;
    use crate::mongodb::mongodb_object_id::{
        serialize_option_hex_string_to_object_id,
        deserialize_option_hex_string_from_object_id
    };

    #[derive(Serialize, Deserialize, Debug)]
    struct OID {
        #[serde(serialize_with = "serialize_option_hex_string_to_object_id")]
        #[serde(deserialize_with = "deserialize_option_hex_string_from_object_id")]
        pub id: Option<String>
    }

    #[test]
    fn test_nonnull() {
        let hex_id = ObjectId::new().to_hex();
        let json = format!("{{\"id\":{{\"$oid\":\"{}\"}}}}", hex_id);

        let serialize = serde_json::from_str::<OID>(&json).unwrap();
        let deserialize = serde_json::to_string(&serialize).unwrap();
        assert_eq!(serialize.id.unwrap(), hex_id);
        assert_eq!(deserialize, json);
    }

    #[test]
    fn test_null() {
        let json = r#"{"id":null}"#;
        let serialize = serde_json::from_str::<OID>(&json).unwrap();
        let deserialize = serde_json::to_string(&serialize).unwrap();
        assert_eq!(serialize.id, None);
        assert_eq!(deserialize, json);
    }

}