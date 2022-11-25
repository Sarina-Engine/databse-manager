use super::schema::{categories, comments, products};
use crate::services::scraper_rpc::{
    Category as CategoryS, CategoryList, Comment as CommentS, CommentList, Product as ProductS,
    ProductList,
};
use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct Comment {
    pub vid: i32,
    pub id: i32,
    pub product_id: i32,
    pub title: Option<String>,
    pub body: String,
    pub rating: i32,
}

impl Comment {
    pub fn add(comment_vec: CommentList, conn: &mut PgConnection) -> () {
        let comment_vec: Vec<NewComment> = comment_vec
            .comment_vec
            .into_iter()
            .map(|x| x.into())
            .collect();
        diesel::insert_into(comments::table)
            .values(comment_vec)
            .execute(conn)
            .unwrap();
        ()
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub id: i32,
    pub product_id: i32,
    pub body: String,
    pub rate: f64,
}

impl From<CommentS> for NewComment {
    fn from(c: CommentS) -> Self {
        NewComment {
            id: c.id,
            product_id: c.product_id,
            body: c.body,
            rate: c.rate,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct {
    id: i32,
    title_fa: String,
    rate: f64,
    views: i32,
    cat_id: i32,
    done: bool,
}

#[derive(Debug, Queryable)]
pub struct Product {
    pub vid: i32,
    pub id: i32,
    pub cat_id: i32,
    pub title_fa: String,
    pub rate: f64,
    pub views: i32,
    pub done: bool,
}

impl From<ProductS> for NewProduct {
    fn from(p: ProductS) -> Self {
        NewProduct {
            id: p.id,
            title_fa: p.title_fa,
            rate: p.rate,
            views: p.count,
            cat_id: p.cat_id,
            done: p.done,
        }
    }
}

impl Product {
    pub fn add(products: ProductList, conn: &mut PgConnection) -> () {
        let product_vec: Vec<NewProduct> =
            products.product_vec.into_iter().map(|x| x.into()).collect();
        diesel::insert_into(products::table)
            .values(product_vec)
            .execute(conn)
            .unwrap();
    }

    pub fn get_un_finished(conn: &mut PgConnection) -> Vec<Product> {
        use self::products::dsl::*;
        products
            .filter(done.eq(false))
            .load::<Product>(conn)
            .expect("Error loading")
    }
    pub fn set_to_finished(conn: &mut PgConnection, product_id: i32) -> () {
        use self::products::dsl::*;
        diesel::update(products.filter(id.eq(product_id)))
            .set(done.eq(true))
            .execute(conn)
            .unwrap();
        ()
    }
}

#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name=categories)]
pub struct Category {
    pub vid: i32,
    pub id: i32,
    pub title_fa: String,
    pub code: String,
    pub parent_cat: i32,
    pub done: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name=categories)]
pub struct NewCategory {
    id: i32,
    title_fa: String,
    code: String,
    parent_cat: i32,
}

impl From<CategoryS> for NewCategory {
    fn from(category: CategoryS) -> Self {
        NewCategory {
            id: category.id,
            title_fa: category.title_fa,
            code: category.code,
            parent_cat: category.parent_cat,
        }
    }
}

impl Category {
    pub fn add(categories: CategoryList, conn: &mut PgConnection) -> () {
        let category_vec = categories.category_vec;
        let cat_vec: Vec<NewCategory> = category_vec
            .into_iter()
            .map(|x| NewCategory::from(x))
            .collect();
        diesel::insert_into(categories::table)
            .values(cat_vec)
            .execute(conn)
            .unwrap();
        ()
    }
    pub fn get_un_finished(conn: &mut PgConnection) -> Vec<Category> {
        use self::categories::dsl::*;
        categories
            .filter(done.eq(false))
            .load::<Category>(conn)
            .expect("Error loading")
    }
    pub fn set_to_finished(conn: &mut PgConnection, cat_id: i32) -> () {
        use self::categories::dsl::*;
        diesel::update(categories.filter(id.eq(cat_id)))
            .set(done.eq(true))
            .execute(conn)
            .unwrap();
        ()
    }
}
