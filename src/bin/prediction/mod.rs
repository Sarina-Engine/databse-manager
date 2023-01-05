use db_manager::db::establish_connection;
use db_manager::model::{Comment, Sentiment};
use db_manager::services::prediction_rpc::predict_sentiment_client::PredictSentimentClient;
use db_manager::services::prediction_rpc::Comment as CommentRPC;
use tonic::Request;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = establish_connection();
    let mut client = PredictSentimentClient::connect("http://[::1]:50052").await?;
    let comment_vec = Comment::get_un_finished(&mut conn);
    for comment in comment_vec {
        let c_id = comment.id;
        let c = CommentRPC {
            comment: comment.body,
        };
        let c = Request::new(c);
        let resp = client.predict(c).await?;
        Sentiment::add(resp.into_inner(), c_id, &mut conn);
        Comment::set_to_finished(&mut conn, c_id);
    }
    Ok(())
}
