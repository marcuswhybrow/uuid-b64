use sqlx::{encode::IsNull, error::BoxDynError, sqlite::SqliteTypeInfo, Database, Decode, Encode, Sqlite, Type};

use crate::{errors::ErrorKind, UuidB64};

impl sqlx::Type<Sqlite> for UuidB64 {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for UuidB64 { 
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        let string = self.to_string();
        <String as Encode<'_, Sqlite>>::encode(string, buf)
    }
}

impl core::error::Error for ErrorKind {}

impl<'r> Decode<'r, Sqlite> for UuidB64 {
    fn decode(value: <Sqlite as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let string = <String as Decode<'_, Sqlite>>::decode(value)?;
        let parsed = string.parse::<UuidB64>()?;
        Ok(parsed)
    }
}

