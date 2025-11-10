use core::str;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{User, UserRole, Product};



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


#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

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
