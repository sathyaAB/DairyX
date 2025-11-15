use core::str;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;
use uuid::Uuid;
use crate::models::{User, UserRole, Product, Delivery};


// Registration, login, user filtering & user responses.
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "Last name is required"))]
    pub last_name: String,
    #[validate(length(min = 1, message = "Email is required"), email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub contact_number: Option<String>,
    pub role: Option<UserRole>, 
}


#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(length(min = 1, message = "Email is required"), email(message = "Email is invalid"))]
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub contact_number: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            first_name: user.first_name.to_owned(),
            last_name: user.last_name.to_owned(),
            email: user.email.to_owned(),
            role: user.role.to_str().to_string(),
            address: user.address.clone(),
            city: user.city.clone(),
            district: user.district.clone(),
            contact_number: user.contact_number.clone(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }
}


// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserData {
//     pub user: FilterUserDto,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserResponseDto {
//     pub status: String,
//     pub data: UserData,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub results: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}
// Product creation and product list responses.

#[derive(Debug, serde::Deserialize)]
pub struct CreateProductDto {
    pub name: String,
    pub price: f64,
    pub unit_type: String,
    pub commission: Option<f64>,
}

#[derive(Debug, serde::Serialize)]
pub struct ProductResponseDto {
    pub status: String,
    pub product: Product,
}

#[derive(Serialize)]
pub struct ProductsListResponseDto {
    pub status: String,
    pub results: usize,
    pub products: Vec<Product>,
}

// Delivery creation and delivery list responses.
#[derive(Debug, Deserialize)] 
pub struct CreateDeliveryDto {
    pub date: String,
    pub products: Vec<DeliveryProductDto>,
}

#[derive(Debug, Deserialize)]
pub struct DeliveryProductDto {
    pub product_id: uuid::Uuid,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryResponseDto {
    pub status: String,
    pub delivery: Delivery,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct DeliveriesListResponseDto {
//     pub status: String,
//     pub deliveries: Vec<Delivery>,
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryListResponseDto {
    pub status: String,
    pub deliveries: Vec<Delivery>,
}

// Creating truck loads and adding products to a load.
#[derive(Debug, Deserialize)]
pub struct CreateTruckLoadRequest {
    pub truck_id: Uuid,              
    pub driver_id: Uuid,               
    pub date: NaiveDate,              
    pub products: Vec<TruckLoadProductItem>,
}


#[derive(Debug, Deserialize)]
pub struct TruckLoadProductItem {
    pub product_id: Uuid,              
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct CreateTruckLoadResponse {
    pub truckloadid: Uuid,    
    pub driver_id: Uuid,               
    pub message: String,
}

// Creating sales and sale product items.

#[derive(Debug, Deserialize)]
pub struct SaleProductItem {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateSaleRequest {
    pub truckload_id: Uuid,  
    pub shop_id: Uuid,      
    pub date: NaiveDate,
    pub products: Vec<SaleProductItem>,
}

#[derive(Debug, Serialize)]
pub struct CreateSaleResponse {
    pub salesid: Uuid,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePaymentRequest {
    pub salesid: Uuid,
    pub amount: f64,
    pub method: String, // e.g., "cash", "card", "online"
    pub date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllSalesResponse {
    pub sales: Vec<SaleDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaleDto {
    pub salesid: Uuid,
    pub truckload_id: Uuid,
    pub shop_id: Uuid,
    pub date: NaiveDate,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub status: String,
}


// Recording payments and payment responses.
#[derive(Debug, Serialize)]
pub struct CreatePaymentResponse {
    pub paymentid: Uuid,
    pub message: String,
}

// Allowances, truck allowances, and distribution data.
#[derive(Debug, Deserialize)]
pub struct CreateAllowanceRequest {
    pub date: NaiveDate,
    pub amount: f64,  
    pub notes: Option<String>, 
}

#[derive(Debug, Serialize)]
pub struct CreateAllowanceResponse {
    pub allowanceid: Uuid,
    pub message: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AllowanceResponse {
//     pub allowanceid: Uuid,
//     pub date: NaiveDate,
//     pub amount: f64,
//     pub created_at: Option<NaiveDateTime>,
// }

#[derive(Debug, Deserialize)]
pub struct CreateTruckAllowanceRequest {
    pub truckid: Uuid,     
    pub date: NaiveDate,  
    pub amount: f64,        
}

#[derive(Debug, Serialize)]
pub struct CreateTruckAllowanceResponse {
    pub truck_allowance_id: Uuid,
    pub message: String,
}

// Daily sales, revenue, and commission reports.

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DailyProductSaleRequest {
    pub date: NaiveDate, 
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DailyProductSaleResponse {
    pub product_name: String,
    pub total_quantity: i64,
    pub total_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyProductSaleListResponse {
    pub status: String,
    pub results: usize,
    pub sales: Vec<DailyProductSaleResponse>,
}

#[derive(Debug, Serialize)]
pub struct DailySalesRevenueResponse {
    pub date: NaiveDate,
    pub total_revenue: f64,
}

#[derive(Debug, Deserialize)]
pub struct DailyCommissionRequest {
    pub date: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct DailyCommissionResponse {
    pub date: NaiveDate,
    pub total_commission: f64,
}

// Allowances, truck allowances, and distribution data.
#[derive(Debug, Deserialize)]
pub struct AllowanceDistributionRequest {
    pub date: NaiveDate,
}


#[derive(Debug, Serialize)]
pub struct TruckAllowanceInfo {
    pub trucknumber: String,
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct AllowanceDistributionResponse {
    pub date: NaiveDate,
    pub total_amount: f64,
    pub distributed: Vec<TruckAllowanceInfo>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct PendingPaymentResponse {
    pub salesid: uuid::Uuid,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub remaining_amount: f64,
    pub shop_name: String,
    pub shop_address: String,
}
// Creating trucks and updating max allowance.


#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTruckLoadQuantityRequest {
    pub truckloadid: Uuid,
    pub items: Vec<TruckLoadQuantityItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TruckLoadQuantityItem {
    pub productid: Uuid,
    pub remaining_quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTruckLoadQuantityResponse {
    pub truckloadid: Uuid,
    pub items: Vec<TruckLoadQuantityItemResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TruckLoadQuantityItemResponse {
    pub productid: Uuid,
    pub remaining_quantity: i32,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTruckRequest {
    pub trucknumber: String,
    pub model: String,
    pub max_allowance: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTruckResponse {
    pub truckid: uuid::Uuid,
    pub message: String,
}


// #[derive(Debug, Serialize, Deserialize)]
// pub struct TruckData {
//     pub truckid: uuid::Uuid,
//     pub trucknumber: String,
//     pub model: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTruckMaxAllowanceRequest {
    pub trucknumber: String,
    pub max_allowance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTruckMaxAllowanceResponse {
    pub truckid: uuid::Uuid,
    pub trucknumber: String,
    pub max_allowance: f64,
}

// Creating shop records.

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShopRequest {
    pub name: String,
    pub address: String,
    pub city: Option<String>,
    pub district: Option<String>,
    pub contact_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShopResponse {
    pub shopid: uuid::Uuid,
    pub message: String,
}
