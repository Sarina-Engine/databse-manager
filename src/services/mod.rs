use crate::db::establish_connection;
use scraper_rpc::scraper_service_server::ScraperService;
use scraper_rpc::DbResponse;
use scraper_rpc::{Category, Product};
use scraper_rpc::{CategoryList, CommentList, FeatureList, ProductList};
use tonic::{Request, Response, Status};
use web_server::web_server_server::WebServer;
pub mod scraper_rpc {
    tonic::include_proto!("scraper_rpc");
}

#[derive(Default, Debug)]
pub struct ScraperRPCService;

#[derive(Default, Debug)]
pub struct WebServerRPC;

pub mod prediction_rpc {
    tonic::include_proto!("prediction");
}

pub mod assigner_rpc {
    tonic::include_proto!("assigner");
}

pub mod web_server {
    tonic::include_proto!("web_server");
}

#[tonic::async_trait]
impl WebServer for WebServerRPC {
    async fn send_category(
        &self,
        request: Request<web_server::Empty>,
    ) -> Result<Response<web_server::CategoryList>, Status> {
        use crate::model::Category as CategoryDb;
        let mut conn = establish_connection();
        let cat_list = CategoryDb::all(&mut conn)
            .into_iter()
            .map(|x| x.into())
            .collect();
        let reply = web_server::CategoryList {
            category_vec: cat_list,
        };
        println!("{:?}", reply);
        Ok(Response::new(reply))
    }

    async fn send_product(
        &self,
        request: Request<web_server::CategoryId>,
    ) -> Result<Response<web_server::ProductList>, Status> {
        use crate::model::Product as ProductDb;
        use crate::model::Score as ScoreDb;
        let mut conn = establish_connection();
        let id = request.into_inner().id;
        let mut product_vec: Vec<web_server::Product> = Vec::new();
        let unrated_products = ProductDb::get_category_products(&mut conn, id);
        for p in unrated_products {
            let scores = ScoreDb::get(&mut conn, p.id);
            let product = web_server::Product {
                id: p.id,
                name: p.title_fa,
                rating: scores.overall,
            };
            product_vec.push(product);
        }
        let reply = web_server::ProductList {
            product_vec: product_vec,
        };
        println!("{:?}", reply);
        Ok(Response::new(reply))
    }
}
use crate::model::Category as CategoryDb;
impl From<CategoryDb> for web_server::Category {
    fn from(c: CategoryDb) -> Self {
        Self {
            id: c.id,
            title_fa: c.title_fa,
            parent_cat: c.parent_cat,
            code: c.code,
        }
    }
}

#[tonic::async_trait]
impl ScraperService for ScraperRPCService {
    async fn send_feature(
        &self,
        request: Request<FeatureList>,
    ) -> Result<Response<DbResponse>, Status> {
        use crate::model::Feature as FeatureD;
        let mut conn = establish_connection();

        FeatureD::add(request.into_inner(), &mut conn);

        let reply = DbResponse { status: true };
        println!("{:?}", reply);
        Ok(Response::new(reply))
    }
    async fn send_category(
        &self,
        request: Request<CategoryList>,
    ) -> Result<Response<DbResponse>, Status> {
        use crate::model::Category as CategoryDb;
        let mut conn = establish_connection();
        CategoryDb::add(request.into_inner(), &mut conn);

        let reply = DbResponse { status: true };
        println!("{:?}", reply);
        Ok(Response::new(reply))
    }

    async fn send_product(
        &self,
        request: Request<ProductList>,
    ) -> Result<Response<DbResponse>, Status> {
        use crate::model::Category as CategoryD;
        use crate::model::Product as ProductDb;
        let mut conn = establish_connection();
        let request = request.into_inner();
        let cat_id = request.product_vec[0].cat_id;
        ProductDb::add(request, &mut conn);
        CategoryD::set_to_finished(&mut conn, cat_id);

        let reply = DbResponse { status: true };
        println!("{:?} Category: {} is done", reply, cat_id);
        Ok(Response::new(reply))
    }
    async fn send_comment(
        &self,
        request: Request<CommentList>,
    ) -> Result<Response<DbResponse>, Status> {
        use crate::model::Comment as CommentDb;
        use crate::model::Product as ProductD;
        let mut conn = establish_connection();
        let request = request.into_inner();
        let product_id = request.comment_vec[0].product_id;
        CommentDb::add(request, &mut conn);
        ProductD::set_to_finished(&mut conn, product_id);

        let reply = DbResponse { status: true };
        println!("{:?} Product: {} is done", reply, product_id);
        Ok(Response::new(reply))
    }
}

use crate::model::Category as CategoryD;
impl From<CategoryD> for Category {
    fn from(c: CategoryD) -> Self {
        Self {
            id: c.id,
            title_fa: c.title_fa,
            code: c.code,
            parent_cat: c.parent_cat,
        }
    }
}

use crate::model::Product as ProductD;
impl From<ProductD> for Product {
    fn from(p: ProductD) -> Self {
        Self {
            id: p.id,
            title_fa: p.title_fa,
            rate: p.rate,
            count: p.views,
            cat_id: p.cat_id,
            done: p.done,
        }
    }
}
