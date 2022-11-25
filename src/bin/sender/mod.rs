use db_manager::db::establish_connection;
use db_manager::model::{Category, Product};
use db_manager::services::scraper_rpc::scraper_service_client::ScraperServiceClient;
use db_manager::services::scraper_rpc::{
    Category as CategoryS, CategoryList, Product as ProductS, ProductList,
};
use diesel::PgConnection;
use tonic::Request;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = establish_connection();
    send_unfinished_cat(&mut conn).await?;
    send_unfinished_product(&mut conn).await?;
    Ok(())
}

pub async fn send_unfinished_cat(
    conn: &mut PgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ScraperServiceClient::connect("http://[::1]:10000").await?;
    let mut id_vec = Vec::new();
    let cat_vec: Vec<CategoryS> = Category::get_un_finished(conn)
        .into_iter()
        .map(|x| {
            id_vec.push(x.id);
            x.into()
        })
        .collect();
    let cat_vec = CategoryList {
        category_vec: cat_vec,
    };
    let req = Request::new(cat_vec);
    client.send_category(req).await?;
    id_vec
        .iter()
        .for_each(|x| Category::set_to_finished(conn, *x));

    Ok(())
}
pub async fn send_unfinished_product(
    conn: &mut PgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ScraperServiceClient::connect("http://[::1]:10000").await?;
    let mut product_ids = Vec::new();
    let product_vec: Vec<ProductS> = Product::get_un_finished(conn)
        .into_iter()
        .map(|x| {
            product_ids.push(x.id);
            x.into()
        })
        .collect();
    let product_vec = ProductList { product_vec };
    let req = Request::new(product_vec);
    client.send_product(req).await?;
    product_ids
        .iter()
        .for_each(|x| Product::set_to_finished(conn, *x));

    Ok(())
}
