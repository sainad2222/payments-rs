use bytes::BytesMut;
use postgres_types::{FromSql, ToSql, Type, IsNull};
use rust_decimal::Decimal;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct PgDecimal(pub Decimal);

impl<'a> FromSql<'a> for PgDecimal {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        // Postgres sends NUMERIC as a string
        let s = std::str::from_utf8(raw)?;
        Ok(PgDecimal(Decimal::from_str(s.trim())?))
    }
    fn accepts(ty: &Type) -> bool {
        *ty == Type::NUMERIC
    }
}

impl ToSql for PgDecimal {
    fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        // Write the decimal as a string
        out.extend_from_slice(self.0.to_string().as_bytes());
        Ok(IsNull::No)
    }
    fn accepts(ty: &Type) -> bool {
        *ty == Type::NUMERIC
    }
    fn to_sql_checked(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        ToSql::to_sql(self, ty, out)
    }
}

impl From<PgDecimal> for Decimal {
    fn from(pg_decimal: PgDecimal) -> Self {
        pg_decimal.0
    }
}

impl From<Decimal> for PgDecimal {
    fn from(decimal: Decimal) -> Self {
        PgDecimal(decimal)
    }
} 