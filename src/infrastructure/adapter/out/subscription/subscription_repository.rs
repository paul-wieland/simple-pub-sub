use std::error::Error;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Collection;
use crate::domain::model::service_error::ServiceError;
use crate::domain::model::service_error::ServiceError::InternalServerError;
use crate::infrastructure::adapter::config::mongo_db_client::MongoDbClient;
use crate::infrastructure::adapter::out::subscription::subscription_entity::SubscriptionEntity;

pub struct SubscriptionRepository{
    mongodb_client: MongoDbClient,
    collection_name: String
}

impl SubscriptionRepository{

    pub async fn new() -> Result<Self, Box<dyn Error>>{
        let mongodb_client = MongoDbClient::new().await?;
        let collection_name = "subscriptions".into();
        Ok(Self{ mongodb_client , collection_name})
    }

    fn collection<T: Send + Sync>(&self) -> Collection<T>{
        self.mongodb_client.collection(&self.collection_name)
    }

    pub async fn create_subscription(&self, subscription_entity: SubscriptionEntity) -> Result<(), ServiceError> {

        if let Ok(Some(_)) = self.find_subscription(
            &subscription_entity.project,
            &subscription_entity.topic,
            &subscription_entity.subscription).await{
            return Err(ServiceError::ResourceExists)
        }

        self.collection::<SubscriptionEntity>().insert_one(subscription_entity).await
            .map(|_| { })
            .map_err(|_| ServiceError::InternalServerError)
    }

    pub async fn find_subscription(&self, project: &str, topic: &str, subscription: &str) -> Result<Option<SubscriptionEntity>, Box<dyn Error>> {
        let filter = doc! { "project": project, "topic": topic, "subscription": subscription};
        self.collection::<SubscriptionEntity>().find_one(filter)
            .await
            .map_err(|_| format!("Error: could not query find_one with project {} and topic {}", project, topic).into())
    }

    pub async fn find_many_subscriptions(&self, project: &str, topic: &str) -> Result<Vec<SubscriptionEntity>, ServiceError>{
        let filter = doc! { "project": project, "topic": topic};

        let cursor =  self.collection::<SubscriptionEntity>().find(filter)
            .await.map_err(|_| InternalServerError)?;

        let results: Vec<SubscriptionEntity> = cursor.try_collect().await
            .map_err(|_| InternalServerError)?;

        Ok(results)

            // .map(|r| { r.try_collect() })
            // .map_err(|_| ServiceError::InternalServerError)
    }

}