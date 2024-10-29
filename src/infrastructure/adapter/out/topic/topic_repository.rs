use std::error::Error;
use mongodb::bson::doc;
use mongodb::Collection;
use crate::domain::model::topic::Topic;
use crate::infrastructure::adapter::config::mongo_db_client::MongoDbClient;
use crate::infrastructure::adapter::out::topic::topic_entity::TopicEntity;

pub struct TopicRepository{
    mongodb_client: MongoDbClient,
    topic_collection_name: String
}

impl TopicRepository{

    pub async fn new() -> Result<Self, Box<dyn Error>>{
        let mongodb_client = MongoDbClient::new().await?;
        let topic_collection_name = "topics".into();
        Ok(Self{ mongodb_client , topic_collection_name})
    }

    fn collection<T: Send + Sync>(&self) -> Collection<T>{
        self.mongodb_client.collection(&self.topic_collection_name)
    }

    pub async fn create_topic(&self, topic: Topic) -> Result<(), Box<dyn Error>> {
        let project_name = topic.project.clone();
        let topic_name = topic.topic.clone();

        if let Ok(Some(_)) = self.find_topic(&project_name, &topic_name).await{
            return Err(format!("Error: project {} and topic {:?} are already existing", &project_name, &topic_name).into())
        }

        let topic_entity: TopicEntity = TopicEntity::from(topic);

        self.collection::<TopicEntity>().insert_one(topic_entity).await
            .map(|_| { })
            .map_err(|_| format!("Error: could not create project {} and topic {}", &project_name, &topic_name).into())
    }

    pub async fn find_topic(&self, project: &str, topic: &str) -> Result<Option<Topic>, Box<dyn Error>> {
        let filter = doc! { "project": project, "topic": topic};
        self.collection::<TopicEntity>().find_one(filter)
            .await
            .map(|entity| { entity.map(Topic::from) })
            .map_err(|_| format!("Error: could not query find_one with project {} and topic {}", project, topic).into())
    }

}