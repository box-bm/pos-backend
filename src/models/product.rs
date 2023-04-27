use crate::schema::products;
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::{ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewProductHandler {
    pub name: String,
    pub description: String,
    pub price: f64,
}

#[derive(Insertable)]
#[diesel(table_name = products)]
struct NewProduct<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub price: BigDecimal,
}

impl Product {
    pub fn get_all_products<'a>(
        conn: &mut PgConnection,
    ) -> Result<Vec<Product>, diesel::result::Error> {
        use self::products::dsl::*;
        products.load::<Product>(conn)
    }

    pub fn get_product<'a>(
        conn: &mut PgConnection,
        product_id: &i32,
    ) -> Result<Product, diesel::result::Error> {
        use self::products::dsl::*;
        products.filter(id.eq(product_id)).first(conn)
    }

    pub fn create_product<'a>(
        conn: &mut PgConnection,
        product: &NewProductHandler,
    ) -> Result<Product, diesel::result::Error> {
        let new = NewProduct {
            description: &product.description,
            name: &product.name,
            price: BigDecimal::from_f64(product.price).unwrap(),
        };

        diesel::insert_into(products::table)
            .values(&new)
            .get_result::<Product>(conn)
    }
}
