use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;


#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Manager,
    Driver, // Added new roles
}

impl UserRole {
    pub fn to_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Manager => "manager",
            UserRole::Driver => "driver",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, sqlx::Type, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub contact_number: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Product {
    pub id: uuid::Uuid,
    pub name: String,
    pub price: f64,
    pub unit_type: String,
    pub commission: f64,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Delivery {
    pub deliveryid: uuid::Uuid,
    pub date: NaiveDate,
    pub userid: uuid::Uuid,
}


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct DeliveryProduct {
    pub deliveryid: uuid::Uuid,
    pub productid: uuid::Uuid,
    pub quantity: i32,
}


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct WarehouseStock {
    pub stockid: uuid::Uuid,
    pub productid: uuid::Uuid,
    pub quantity: i32,
}

