use serde_json::Error as JsonError;
use serde::{Serialize, Deserialize};

/// `Schema` is a trait that provides methods for converting a type to and from JSON.
///
/// # Methods
///
/// `to_json`: Converts the type implementing this trait into a JSON string.
/// Returns a `Result` which is an `Ok` of the JSON string, or an `Err` of `JsonError` if the conversion fails.
///
/// `from_json`: Converts a JSON string into a type implementing this trait.
/// Returns a `Result` which is an `Ok` of the type, or an `Err` of `JsonError` if the conversion fails.
///
/// # Type Parameters
///
/// `Self`: The type implementing this trait. It must also implement the `Serialize` and `Deserialize` traits.
pub trait Schema {
    /// Converts the type implementing this trait into a JSON string.
    ///
    /// # Returns
    ///
    /// A `Result` which is an `Ok` of the JSON string, or an `Err` of `JsonError` if the conversion fails.
    fn to_json(&self) -> Result<String, JsonError>
        where
            Self: Serialize,
    {
        serde_json::to_string(self)
    }

    /// Converts a JSON string into a type implementing this trait.
    ///
    /// # Arguments
    ///
    /// * `json` - A JSON string to be converted into the type.
    ///
    /// # Returns
    ///
    /// A `Result` which is an `Ok` of the type, or an `Err` of `JsonError` if the conversion fails.
    fn from_json<'b>(json: &'b str) -> Result<Self, JsonError>
        where
            Self: Deserialize<'b>,
    {
        serde_json::from_str(json)
    }
}

/// `schema` is a macro that simplifies the process of defining a struct that implements the `Schema` trait.
/// It takes a struct name and a list of fields as input and automatically defines the struct with the specified fields.
/// Each field is annotated with `#[serde(rename = $json_name)]` to specify the name of the field when it is serialized or deserialized.
///
/// # Arguments
///
/// * `$name`: The name of the struct.
/// * `$field_name`: The name of a field in the struct.
/// * `$field_type`: The type of the field.
/// * `$json_name`: The name of the field when it is serialized or deserialized.
#[macro_export]
macro_rules! schema {
    (
        $name:ident {
            $(
                $field_name:ident : $field_type:ty as $json_name:expr
            ),*$(,)*
        }
    ) => {
        use serde::{Serialize, Deserialize};
        use crate::schemas::Schema;
        use serde_json::Error as JsonError;

        #[derive(Debug, Clone, Default, PartialEq, schema_macro::Schema, Serialize, Deserialize)]
        pub struct $name {
            $(
                #[serde(rename = $json_name)]
                pub $field_name: $field_type,
            )*
        }
    };
}

#[cfg(test)]
mod tests {

    schema!(
        TestStruct {
            field1: String as "field1",
            field2: i32 as "field2",
            field3: Option<String> as "field3",
        }
    );

    #[test]
    fn test_struct_serialization_works_correctly() {
        let test_struct = TestStruct {
            field1: "Hello, world!".to_string(),
            field2: 42,
            field3: Some("Optional field".to_string()),
        };

        let serialized = test_struct.to_json().unwrap();
        let expected = r#"{"field1":"Hello, world!","field2":42,"field3":"Optional field"}"#;

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_struct_deserialization_works_correctly() {
        let data = r#"{"field1":"Hello, world!","field2":42,"field3":"Optional field"}"#;

        let test_struct: TestStruct = TestStruct::from_json(data).unwrap();
        let expected = TestStruct {
            field1: "Hello, world!".to_string(),
            field2: 42,
            field3: Some("Optional field".to_string()),
        };

        assert_eq!(test_struct, expected);
    }

    #[test]
    fn test_deserialization_with_missing_optional_field_works_correctly() {
        let data = r#"{"field1":"Hello, world!","field2":42}"#;

        let test_struct: TestStruct = TestStruct::from_json(data).unwrap();
        let expected = TestStruct {
            field1: "Hello, world!".to_string(),
            field2: 42,
            field3: None,
        };

        assert_eq!(test_struct, expected);
    }

    #[test]
    fn test_deserialization_without_enough_fields_fails_correctly() {
        let data = r#"{"field1":"Hello, world!"}"#;

        let result: Result<TestStruct, JsonError> = TestStruct::from_json(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialization_with_extra_fields_works_correctly() {
        let data = r#"{"field1":"Hello, world!","field2":42,"field3":"Optional field","extra_field":"Extra field"}"#;

        let test_struct: TestStruct = TestStruct::from_json(data).unwrap();
        let expected = TestStruct {
            field1: "Hello, world!".to_string(),
            field2: 42,
            field3: Some("Optional field".to_string()),
        };

        assert_eq!(test_struct, expected);
    }
}
