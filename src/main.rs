use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, MySqlPool, Row};
use std::{env, io};

struct RowResponse {
    title: String,
    brand: String,
    color: String,
    description: String,
    category: String,
    currency: String,
    image: String,
    link: String,
    avg_price: f32,
    available_sizes: i32,
    labels: Json<Vec<String>>,
    price: f32,
    product_id: String,
    size: String,
    variant_id: String,
}

#[derive(Serialize)]
struct Variant {
    product_id: String,
    variant_id: String,
    size: String,
    price: f32,
}

#[derive(Serialize)]
struct Product {
    id: String,
    title: String,
    brand: String,
    color: String,
    description: String,
    category: String,
    currency: String,
    image: String,
    link: String,
    avg_price: f32,
    available_sizes: i32,
    labels: Json<Vec<String>>,
    variants: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
struct Request {
    q: String,
    page: Option<u32>,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let conn = MySqlPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn index(request: web::Query<Request>, conn: web::Data<MySqlPool>) -> Result<impl Responder> {
    let response = web::block(|| async move {
        sqlx::query(
            "SELECT
                pc.*,
                v.size as size,
                cast(substring_index(v.price, ' ', 1) as float) as price,
                v.id as variantId
            FROM products_clean pc
            LEFT JOIN products v ON v.item_group_id = pc.productId
            WHERE match(pc.title, pc.description) against (?)
            LIMIT 200 OFFSET ?
            ",
        )
        .bind(&request.q)
        .bind(request.page.unwrap_or(0) * 200)
        .fetch_all(&**conn)
        .await
        .unwrap()
    })
    .await?
    .await;

    let rows = response
        .iter()
        .map(|row| RowResponse {
            title: row.get("title"),
            brand: row.get("brand"),
            color: row.get("color"),
            description: row.get("description"),
            category: row.get("category"),
            currency: row.get("currency"),
            image: row.get("image"),
            link: row.get("link"),
            avg_price: row.get("avgPrice"),
            available_sizes: row.get("availableSizes"),
            labels: row.get("labels"),
            price: row.get("price"),
            product_id: row.get("productId"),
            size: row.get("size"),
            variant_id: row.get("variantId"),
        })
        .group_by(|variant| variant.product_id.clone())
        .into_iter()
        .map(|(_, variants)| variants.collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let products = rows
        .iter()
        .map(|variants| {
            let first = variants.first().unwrap();

            Product {
                id: first.product_id.clone(),
                title: first.title.clone(),
                brand: first.brand.clone(),
                color: first.color.clone(),
                description: first.description.clone(),
                category: first.category.clone(),
                currency: first.currency.clone(),
                image: first.image.clone(),
                link: first.link.clone(),
                avg_price: first.avg_price,
                available_sizes: first.available_sizes,
                labels: first.labels.clone(),
                variants: variants
                    .iter()
                    .map(|variant| Variant {
                        price: variant.price.clone(),
                        product_id: variant.product_id.clone(),
                        size: variant.size.clone(),
                        variant_id: variant.variant_id.clone(),
                    })
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(products))
}
