use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row, PgPool};
use uuid::Uuid;

use crate::models::{User, UserRole, Product};

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

   
}

#[async_trait]
pub trait ProductExt {
    async fn create_product(
        &self,
        name: &str,
        price: f64,
        unit_type: &str,
        commission: Option<f64>,
    ) -> Result<Product, sqlx::Error>;

    async fn get_product_by_id(&self, product_id: Uuid) -> Result<Option<Product>, sqlx::Error>;
    async fn get_all_products(&self) -> Result<Vec<Product>, sqlx::Error>;
}

#[async_trait]
impl ProductExt for DBClient {
    async fn create_product(
        &self,
        name: &str,
        price: f64,
        unit_type: &str,
        commission: Option<f64>,
    ) -> Result<Product, sqlx::Error> {
        let product = sqlx::query_as::<_, Product>(
            "INSERT INTO products (name, price, unit_type, commission, created_at, updated_at)
             VALUES ($1,$2,$3,$4,NOW(),NOW()) RETURNING *"
        )
        .bind(name)
        .bind(price)
        .bind(unit_type)
        .bind(commission.unwrap_or(0.0))
        .fetch_one(&self.pool)
        .await?;
        Ok(product)
    }

    async fn get_product_by_id(&self, product_id: Uuid) -> Result<Option<Product>, sqlx::Error> {
        let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1")
            .bind(product_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(product)
    }

    async fn get_all_products(&self) -> Result<Vec<Product>, sqlx::Error> {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(&self.pool)
        .await?;
    Ok(products)
}

}
