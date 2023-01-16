use super::schema::{categories, comments, features, products, scores, sentiments};
use crate::services::assigner_rpc::ScorePrediction;
use crate::services::prediction_rpc::Prediction;
use crate::services::scraper_rpc::{
    Category as CategoryS, CategoryList, Comment as CommentS, CommentList, Feature as FeatureS,
    FeatureList, Product as ProductS, ProductList,
};
use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct Score {
    pub vid: i32,
    pub product_id: i32,
    pub emotion: f64,
    pub satisfaction: f64,
    pub recommended: f64,
    pub feeling: f64,
    pub overall: f64,
}

impl Score {
    pub fn add(scores: ScorePrediction, product_id: i32, conn: &mut PgConnection) -> () {
        let prev_score = match Self::get(conn, product_id) {
            Ok(s) => s,
            Err(_) => Self {
                vid: 0,
                product_id: product_id,
                emotion: 0.0,
                satisfaction: 0.0,
                recommended: 0.0,
                feeling: 0.0,
                overall: 0.0,
            },
        };
        let scores = NewScore::new(scores, prev_score, product_id);
        diesel::insert_into(scores::table)
            .values(scores)
            .execute(conn)
            .unwrap();
        ()
    }
    pub fn get(conn: &mut PgConnection, p_id: i32) -> Result<Score, Box<dyn std::error::Error>> {
        use self::scores::dsl::*;
        let s = scores
            .filter(product_id.eq(p_id))
            .load::<Score>(conn)?
            .pop()
            .unwrap();
        Ok(s)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = scores)]
pub struct NewScore {
    pub product_id: i32,
    pub emotion: f64,
    pub satisfaction: f64,
    pub recommended: f64,
    pub feeling: f64,
    pub overall: f64,
}

impl NewScore {
    pub fn new(scores: ScorePrediction, prev_score: Score, product_id: i32) -> Self {
        let scores = scores.scores;
        let emotion = scores[0] * 0.1 + prev_score.emotion * 0.9;
        let satisfaction = scores[1] * 0.1 + prev_score.satisfaction * 0.9;
        let recommended = scores[2] * 0.1 + prev_score.recommended * 0.9;
        let feeling = scores[3] * 0.1 + prev_score.feeling * 0.9;
        let overall = scores[4] * 0.1 + prev_score.overall * 0.9;

        Self {
            product_id,
            emotion: emotion.into(),
            satisfaction: satisfaction.into(),
            recommended: recommended.into(),
            feeling: feeling.into(),
            overall: overall.into(),
        }
    }
}

#[derive(Debug, Queryable)]
pub struct Sentiment {
    pub vid: i32,
    pub comment_id: i32,
    pub recommended: f64,
    pub not_recommended: f64,
    pub no_idea: f64,
    pub sad: f64,
    pub happy: f64,
    pub positive: f64,
    pub negative: f64,
    pub furious: f64,
    pub angry: f64,
    pub neutral: f64,
    pub happy2: f64,
    pub delighted: f64,
    pub done: bool,
}

impl Sentiment {
    pub fn add(prediction: Prediction, comment_id: i32, conn: &mut PgConnection) -> () {
        let sentiment = NewSentiment::new(prediction, comment_id);
        diesel::insert_into(sentiments::table)
            .values(sentiment)
            .execute(conn)
            .unwrap();
        ()
    }

    pub fn get_un_finished(conn: &mut PgConnection) -> Vec<Sentiment> {
        use self::sentiments::dsl::*;
        sentiments
            .filter(done.eq(false))
            .load::<Sentiment>(conn)
            .expect("Error loading")
    }
    pub fn set_to_finished(conn: &mut PgConnection, c_id: i32) -> () {
        use self::sentiments::dsl::*;
        diesel::update(sentiments.filter(comment_id.eq(c_id)))
            .set(done.eq(true))
            .execute(conn)
            .unwrap();
        ()
    }
}

#[derive(Debug, Insertable, Default)]
#[diesel(table_name = sentiments)]
pub struct NewSentiment {
    pub comment_id: i32,
    pub recommended: f64,
    pub not_recommended: f64,
    pub no_idea: f64,
    pub sad: f64,
    pub happy: f64,
    pub positive: f64,
    pub negative: f64,
    pub furious: f64,
    pub angry: f64,
    pub neutral: f64,
    pub happy2: f64,
    pub delighted: f64,
}

impl NewSentiment {
    pub fn new(prediction: Prediction, comment_id: i32) -> Self {
        let digi = &prediction.digisentiment.get("digi").unwrap().sentiment;
        let snapp = &prediction.snappsentiment.get("snapp").unwrap().sentiment;
        let binary = &prediction.binarysentiment.get("binary").unwrap().sentiment;
        let multi = &prediction.mulitsentiment.get("multi").unwrap().sentiment;
        NewSentiment {
            comment_id: comment_id,
            recommended: digi[0].score.into(),
            not_recommended: digi[1].score.into(),
            no_idea: digi[2].score.into(),
            sad: snapp[0].score.into(),
            happy: snapp[1].score.into(),
            positive: binary[0].score.into(),
            negative: binary[1].score.into(),
            furious: multi[0].score.into(),
            angry: multi[1].score.into(),
            neutral: multi[2].score.into(),
            happy2: multi[3].score.into(),
            delighted: multi[4].score.into(),
        }
    }
}

#[derive(Debug, Queryable)]
pub struct Feature {
    pub vid: i32,
    pub product_id: i32,
    pub name: String,
    pub value: String,
}

impl Feature {
    pub fn add(feature_vec: FeatureList, conn: &mut PgConnection) -> () {
        let feature_vec: Vec<NewFeature> = feature_vec.ft.into_iter().map(|x| x.into()).collect();
        diesel::insert_into(features::table)
            .values(feature_vec)
            .execute(conn)
            .unwrap();
        ()
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name=features)]
pub struct NewFeature {
    pub product_id: i32,
    pub name: String,
    pub value: String,
}

impl From<FeatureS> for NewFeature {
    fn from(c: FeatureS) -> Self {
        NewFeature {
            product_id: c.product_id,
            name: c.name,
            value: c.value,
        }
    }
}

#[derive(Debug, Queryable, Identifiable)]
pub struct Comment {
    pub vid: i32,
    pub id: i32,
    pub product_id: i32,
    pub body: String,
    pub rating: f64,
    pub done: bool,
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
    pub fn get(comment_id: i32, conn: &mut PgConnection) -> Comment {
        use self::comments::dsl::*;
        let mut comment = comments
            .filter(id.eq(comment_id))
            .load::<Comment>(conn)
            .expect("Error Loading!");
        return comment.pop().unwrap();
    }
    pub fn get_un_finished(conn: &mut PgConnection) -> Vec<Comment> {
        use self::comments::dsl::*;
        comments
            .filter(done.eq(false))
            .load::<Comment>(conn)
            .expect("Error loading")
    }
    pub fn set_to_finished(conn: &mut PgConnection, comment_id: i32) -> () {
        use self::comments::dsl::*;
        diesel::update(comments.filter(id.eq(comment_id)))
            .set(done.eq(true))
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

#[derive(Debug, Queryable, Identifiable)]
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
    pub fn get_category_products(conn: &mut PgConnection, cat_id: i32) -> Vec<Product> {
        use self::products::dsl::*;
        products
            .filter(cat_id.eq(cat_id))
            .load::<Product>(conn)
            .expect("Error loading")
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
    pub fn all(conn: &mut PgConnection) -> Vec<Category> {
        use self::categories::dsl::*;
        categories.load::<Category>(conn).expect("Error loading")
    }
}
