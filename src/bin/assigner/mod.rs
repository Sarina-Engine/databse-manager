use db_manager::db::establish_connection;
use db_manager::model::{Comment, Score, Sentiment};
use db_manager::services::assigner_rpc::assign_score_client::AssignScoreClient;
use db_manager::services::assigner_rpc::ScorePrediction as SP;
use tonic::Request;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = establish_connection();
    let mut client = AssignScoreClient::connect("http://[::1]:50052").await?;
    let sentiment_vec = Sentiment::get_un_finished(&mut conn);
    for s in sentiment_vec {
        let c_id = s.comment_id;
        let sentiment = SP {
            scores: vec![
                s.furious,
                s.angry,
                s.neutral,
                s.happy2,
                s.delighted,
                s.negative,
                s.positive,
                s.no_idea,
                s.not_recommended,
                s.recommended,
                s.happy,
                s.sad,
            ],
        };
        let sentiment = Request::new(sentiment);
        let resp = client.scoring(sentiment).await?.into_inner();
        let product_id = Comment::get(c_id, &mut conn).product_id;
        Score::add(resp, product_id, &mut conn);
        Comment::set_to_finished(&mut conn, c_id);
    }
    Ok(())
}
