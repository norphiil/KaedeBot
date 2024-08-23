use std::env;
use std::sync::Mutex;
use lazy_static::lazy_static;

use serenity::all::{ChannelId, GuildId, UserId};

pub struct Db {
    database: sqlx::PgPool
}

lazy_static! {
    static ref DB_INSTANCE: Mutex<Option<Db>> = Mutex::new( None );
}

pub fn get_instance() -> &'static Mutex<Option<Db>> {
    &DB_INSTANCE
}

impl Db {
    pub async fn create_new_channel(&self, guild_id: GuildId, channel_id: ChannelId, user_id: UserId, maybe_parent: Option<ChannelId>) -> Result<(), sqlx::Error> {
        println!("Creating new channel for guild {} with id {} and user {}", guild_id, channel_id, user_id);
        let query = "
           INSERT INTO channels (guild_id, channel_id, user_id, parent) VALUES ($1, $2, $3, $4);
        ";
        let mut query = sqlx::query(query)
            .bind(i64::from(guild_id))
            .bind(i64::from(channel_id))
            .bind(i64::from(user_id));
        query = match maybe_parent {
            Some(parent) => query.bind(i64::from(parent)),
            None => query
        };
        query.execute(&self.database)
            .await?;
        println!("Successfully created new channel for guild {} with id {}", guild_id, channel_id);
        Ok(())
    }

    pub async fn create_tables(&self) -> Result<(), sqlx::Error> {
        let query = "
            CREATE TABLE IF NOT EXISTS channels (
                guild_id BIGINT,
                channel_id BIGINT,
                user_id BIGINT,
                tempory BOOLEAN DEFAULT FALSE,
                PRIMARY KEY (guild_id, channel_id)
            )
        ";
        sqlx::query(query).execute(&self.database).await?;
        Ok(())
    }

    pub async fn get_created_channels(&self) -> Result<(), sqlx::Error> {
        let query = "SELECT guild_id, channel_id FROM channels";
        let rows = sqlx::query(query).fetch_all(&self.database).await?;
        println!("Found {} rows", rows.len());
        Ok(())
    }

    pub async fn setup_database()  {
        println!("Setting up database...");
        let db = Db{
            database: sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(env::var("DATABASE_URL").unwrap().as_str())
            .await
            .expect("Failed to connect to the database")
        };

        // Run migrations, which updates the database's schema to the latest version.
        sqlx::migrate!("./migrations")
            .run(&db.database)
            .await
            .expect("Couldn't run database migrations");
        db.get_created_channels().await.unwrap();
        println!("Database setup complete.");
        *DB_INSTANCE.lock().unwrap() = Some(db);
    }
}