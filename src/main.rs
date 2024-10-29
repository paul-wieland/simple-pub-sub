mod infrastructure;
mod domain;

use std::error::Error;
use crate::domain::model::topic::Topic;
use crate::domain::usecase::create_topic_use_case::CreateTopicUseCase;
use crate::infrastructure::adapter::out::topic::topic_persistence_adapter::TopicPersistenceAdapter;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    let topic_persistence_adapter = TopicPersistenceAdapter::new().await?;
    let create_topic_use_case = CreateTopicUseCase::new(Box::new(topic_persistence_adapter));

    let topic = Topic{
        project: "test-project".into(),
        topic: "v1.users.update".into()
    };

    create_topic_use_case.create_topic(topic).await?;

    Ok(())
}
