use bytes::BytesMut;
use postgres_types::{FromSql, ToSql, Type, IsNull};
use rust_decimal::Decimal;
use std::error::Error;
use std::str::FromStr;
use tracing::error;

#[derive(Debug, Clone, Copy)]
pub struct PgDecimal(pub Decimal);

impl<'a> FromSql<'a> for PgDecimal {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        match *ty {
            Type::NUMERIC => {
                // PostgreSQL sends NUMERIC in binary format
                // The format is: [ndigits, weight, sign, dscale, digits...]
                if raw.len() < 8 {
                    return Err("Invalid numeric format: too short".into());
                }

                let ndigits = i16::from_be_bytes([raw[0], raw[1]]);
                let weight = i16::from_be_bytes([raw[2], raw[3]]);
                let sign = raw[4];
                let dscale = i16::from_be_bytes([raw[5], raw[6]]);

                if ndigits == 0 {
                    return Ok(PgDecimal(Decimal::ZERO));
                }

                // Calculate the total number of digits
                let _total_digits = (weight + 1) * 4;
                let mut result = String::new();

                // Handle sign
                if sign == 0 {
                    result.push('-');
                }

                // Process each digit group
                for i in 0..ndigits {
                    let start = 8 + (i as usize * 2);
                    if start + 1 >= raw.len() {
                        return Err("Invalid numeric format: truncated".into());
                    }
                    let digit = i16::from_be_bytes([raw[start], raw[start + 1]]);
                    result.push_str(&format!("{:04}", digit));
                }

                // Insert decimal point
                let decimal_pos = (weight + 1) * 4 - dscale;
                if decimal_pos > 0 && (decimal_pos as usize) < result.len() {
                    result.insert(decimal_pos as usize, '.');
                }

                // Parse the final string
                match Decimal::from_str(&result) {
                    Ok(decimal) => Ok(PgDecimal(decimal)),
                    Err(e) => {
                        error!("Failed to parse decimal: {}", e);
                        error!("Generated string: {}", result);
                        Err(Box::new(e))
                    }
                }
            }
            _ => Err("Unsupported type".into()),
        }
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