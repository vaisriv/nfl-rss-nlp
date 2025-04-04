use rust_bert::pipelines::question_answering::*;
use tokio::task;

pub async fn predict(question: String, context: String) {
    let qa_model = task::spawn_blocking(|| {
        QuestionAnsweringModel::new(Default::default()).expect("Failed to create QA model")
    })
    .await
    .expect("Blocking task failed");

    let answer = qa_model.predict(
        &[QaInput {
            question: question.clone(),
            context: context.clone(),
        }],
        1,
        2048,
    );

    println!("{:?}", question);
    println!("{:?}", answer[0][0].answer);
}
