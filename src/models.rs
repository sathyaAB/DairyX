use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use rust_decimal::Decimal;

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

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Truck {
    pub truckid: uuid::Uuid,
    pub trucknumber: String,
    pub model: String,
    pub max_allowance: Option<f64>,
}


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct TruckLoad {
    pub truckloadid: uuid::Uuid,
    pub date: chrono::NaiveDate,
    pub userid: uuid::Uuid,
    pub truckid: uuid::Uuid,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct TruckLoadProduct {
    pub truckloadid: uuid::Uuid,
    pub productid: uuid::Uuid,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Shop {
    pub shopid: uuid::Uuid,               
    pub name: String,                 
    pub address: String,               
    pub city: Option<String>,          
    pub district: Option<String>,      
    pub contact_number: Option<String>, 
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Sale {
    pub salesid: uuid::Uuid,
    pub truckloadid: uuid::Uuid,
    pub shopid: uuid::Uuid,
    pub date: NaiveDate,
    pub status: String,
    pub total_amount: f64,  
    pub paid_amount: f64,    
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct SaleProduct {
    pub salesid: uuid::Uuid,
    pub productid: uuid::Uuid,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Payment {
    pub paymentid: uuid::Uuid,
    pub salesid: uuid::Uuid,
    pub amount: f64,
    pub method: String,
    pub date: chrono::NaiveDate,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Allowance {
    pub allowanceid: uuid::Uuid,
    pub date: NaiveDate,
    pub amount: f64,
    pub notes: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct TruckAllowance {
    pub id: uuid::Uuid,
    pub allowanceid: uuid::Uuid,
    pub truckid: uuid::Uuid,
    pub amount: f64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
