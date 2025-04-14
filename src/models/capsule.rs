use chrono::{DateTime, Utc};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::InsertOneResult,
    Database,
};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CapsuleModel {
    pub _id: Option<ObjectId>,
    pub public_id: String,
    pub name: String,
    pub email: String,
    pub title: String,
    pub message: String,
    pub unlock_at: DateTime<Utc>,

    #[serde(default = "now_utc", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(default = "default_false")]
    pub is_unlocked: Option<bool>,

    #[serde(default = "default_false")]
    pub email_sent: Option<bool>,
}

fn default_false() -> Option<bool> {
    Some(false)
}

fn now_utc() -> Option<DateTime<Utc>> {
    Some(Utc::now())
}

pub trait CapsuleTrait {
    async fn create_capsule(
        db: &Database,
        name: &str,
        email: &str,
        title: &str,
        message: &str,
        unlock_at: &DateTime<Utc>,
    ) -> Result<InsertOneResult, Error>;

    async fn get_capsules(db: &Database) -> Result<Vec<CapsuleModel>, Error>;

    async fn get_capsules_by_public_id(
        db: &Database,
        public_id: &str,
    ) -> Result<CapsuleModel, Error>;
}

impl CapsuleTrait for CapsuleModel {
    async fn create_capsule(
        db: &Database,
        name: &str,
        email: &str,
        title: &str,
        message: &str,
        unlock_at: &DateTime<Utc>,
    ) -> Result<InsertOneResult, Error> {
        let collection = db.collection::<CapsuleModel>("capsule");

        let capsule = CapsuleModel {
            _id: Some(ObjectId::new()),
            public_id: nanoid!(10).to_string(),
            name: name.to_string(),
            email: email.to_string(),
            message: message.to_string(),
            title: title.to_string(),
            unlock_at: *unlock_at,
            created_at: now_utc(),
            is_unlocked: default_false(),
            email_sent: default_false(),
        };

        let capsule_entry = collection.insert_one(capsule).await;

        capsule_entry
    }

    async fn get_capsules(db: &Database) -> Result<Vec<CapsuleModel>, Error> {
        let collection = db.collection::<CapsuleModel>("capsule");

        let mut cursor = collection.find(doc! {}).await?;
        let mut capsules = Vec::new();

        while let Some(capsule) = cursor.try_next().await? {
            capsules.push(capsule);
        }

        Ok(capsules)
    }

    async fn get_capsules_by_public_id(
        db: &Database,
        public_id: &str,
    ) -> Result<CapsuleModel, Error> {
        let collection = db.collection::<CapsuleModel>("capsule");

        let result = collection.find_one(doc! {"public_id":public_id}).await?;

        match result {
            Some(capsule) => Ok(capsule),
            None => Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Capsule with public_id {} not found", public_id),
            ))),
        }
    }
}
