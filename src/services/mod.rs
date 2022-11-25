use crate::db::establish_connection;
use scraper_rpc::scraper_service_server::ScraperService;
use scraper_rpc::DbResponse;
use scraper_rpc::{Category, Product};
use scraper_rpc::{CategoryList, CommentList, ProductList};
use tonic::{Request, Response, Status};
pub mod scraper_rpc {
    tonic::include_proto!("scraper_rpc");
}

#[derive(Default, Debug)]
pub struct ScraperRPCService;

#[tonic::async_trait]
impl ScraperService for ScraperRPCService {
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
        use crate::model::Product as ProductDb;
        let mut conn = establish_connection();
        ProductDb::add(request.into_inner(), &mut conn);

        let reply = DbResponse { status: true };
        println!("{:?}", reply);
        Ok(Response::new(reply))
    }
    async fn send_comment(
        &self,
        request: Request<CommentList>,
    ) -> Result<Response<DbResponse>, Status> {
        use crate::model::Comment as CommentDb;
        let mut conn = establish_connection();
        CommentDb::add(request.into_inner(), &mut conn);

        let reply = DbResponse { status: true };
        println!("{:?}", reply);
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
