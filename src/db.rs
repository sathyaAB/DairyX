use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row, PgPool};
use uuid::Uuid;

use crate::models::{User, UserRole};




#[derive(Debug, Clone)]
pub struct DBClient {
    pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }
}

#[async_trait]
pub trait UserExt {
    async fn get_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error>;

    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        first_name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error>;

    async fn get_users(
        &self,
        page: u32,
        limit: usize,
    ) -> Result<Vec<User>, sqlx::Error>;

    async fn save_user(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        password: &str,
        role: UserRole,
        address: Option<&str>,
        city: Option<&str>,
        district: Option<&str>,
        contact_number: Option<&str>,
    ) -> Result<User, sqlx::Error>;

    async fn get_user_count(&self) -> Result<i64, sqlx::Error>;

    async fn update_user_name(
        &self,
        user_id: Uuid,
        first_name: &str,
        last_name: &str,
    ) -> Result<User, sqlx::Error>;

    async fn update_user_role(
        &self,
        user_id: Uuid,
        role: UserRole,
    ) -> Result<User, sqlx::Error>;

    async fn update_user_password(
        &self,
        user_id: Uuid,
        new_password: &str,
    ) -> Result<User, sqlx::Error>;
}

#[async_trait]
impl UserExt for DBClient {
    
  async fn get_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        first_name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error> {
        if let Some(user_id) = user_id {
            return sqlx::query_as!(
                User,
                r#"
                SELECT id, first_name, last_name, email, password, role as "role: UserRole",
                       address, city, district, contact_number, created_at, updated_at
                FROM users
                WHERE id = $1
                "#,
                user_id
            )
            .fetch_optional(&self.pool)
            .await;
        }

        if let Some(first_name) = first_name {
            return sqlx::query_as!(
                User,
                r#"
                SELECT id, first_name, last_name, email, password, role as "role: UserRole",
                       address, city, district, contact_number, created_at, updated_at
                FROM users
                WHERE first_name = $1
                "#,
                first_name
            )
            .fetch_optional(&self.pool)
            .await;
        }

        if let Some(email) = email {
            return sqlx::query_as!(
                User,
                r#"
                SELECT id, first_name, last_name, email, password, role as "role: UserRole",
                       address, city, district, contact_number, created_at, updated_at
                FROM users
                WHERE email = $1
                "#,
                email
            )
            .fetch_optional(&self.pool)
            .await;
        }

        Ok(None)
    }

    async fn get_users(
        &self,
        page: u32,
        limit: usize,
    ) -> Result<Vec<User>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, first_name, last_name, email, password, role as "role: UserRole",
                   address, city, district, contact_number, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn save_user(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        password: &str,
        role: UserRole,
        address: Option<&str>,
        city: Option<&str>,
        district: Option<&str>,
        contact_number: Option<&str>,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (
                first_name, last_name, email, password, role, address, city, district, contact_number
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                id, first_name, last_name, email, password, role as "role: UserRole",
                address, city, district, contact_number, created_at, updated_at
            "#,
            first_name,
            last_name,
            email,
            password,
            role as UserRole,
            address,
            city,
            district,
            contact_number
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_count(&self) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;
        Ok(count.unwrap_or(0))
    }

    async fn update_user_name(
        &self,
        user_id: Uuid,
        first_name: &str,
        last_name: &str,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET first_name = $1, last_name = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING id, first_name, last_name, email, password, role as "role: UserRole",
                      address, city, district, contact_number, created_at, updated_at
            "#,
            first_name,
            last_name,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update_user_role(
        &self,
        user_id: Uuid,
        new_role: UserRole
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET role = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, first_name, last_name, email, password, role as "role: UserRole",
                      address, city, district, contact_number, created_at, updated_at
            "#,
            new_role as UserRole,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update_user_password(
        &self,
        user_id: Uuid,
        new_password: &str
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET password = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, first_name, last_name, email, password, role as "role: UserRole",
                      address, city, district, contact_number, created_at, updated_at
            "#,
            new_password,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
