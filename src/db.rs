use async_trait::async_trait;
use chrono::{ NaiveDate};
use sqlx::{Pool, Postgres, Transaction};
use uuid::Uuid;

use crate::models::{User, UserRole, Product, TruckLoad, Sale, Payment, Allowance, TruckAllowance};

use crate::models::{Delivery};
use sqlx::Executor; 



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


#[async_trait]
pub trait DeliveryExt {
    async fn create_delivery(
        &self,
        user_id: Uuid,
        date: NaiveDate,
        products: Vec<(Uuid, i32)>, // (product_id, quantity)
    ) -> Result<Delivery, sqlx::Error>;

    async fn get_deliveries_by_user(&self, user_id: Uuid) -> Result<Vec<Delivery>, sqlx::Error>;

}
#[async_trait]
impl DeliveryExt for DBClient {
    async fn create_delivery(
    &self,
    user_id: Uuid,
    date: NaiveDate,
    products: Vec<(Uuid, i32)>,
) -> Result<Delivery, sqlx::Error> {
    let mut tx: Transaction<'_, Postgres> = self.pool.begin().await?;

    // Insert into deliveries
    let delivery = sqlx::query_as::<_, Delivery>(
        "INSERT INTO deliveries (date, userid)
         VALUES ($1, $2)
         RETURNING *"
    )
    .bind(date)
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    // Insert into delivery_product and update warehouse_stock
    for (product_id, quantity) in &products {
        // Insert into delivery_product
        sqlx::query(
            "INSERT INTO delivery_product (deliveryid, productid, quantity)
             VALUES ($1, $2, $3)"
        )
        .bind(delivery.deliveryid)
        .bind(product_id)
        .bind(quantity)
        .execute(&mut *tx)
        .await?;

        // Check if product exists in warehouse_stock
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM warehouse_stock WHERE productid = $1)"
        )
        .bind(product_id)
        .fetch_one(&mut *tx)
        .await?;

        if exists {
            // Update existing stock
            sqlx::query(
                "UPDATE warehouse_stock 
                 SET quantity = quantity + $2 
                 WHERE productid = $1"
            )
            .bind(product_id)
            .bind(quantity)
            .execute(&mut *tx)
            .await?;
        } else {
            // Insert new stock entry
            sqlx::query(
                "INSERT INTO warehouse_stock (productid, quantity)
                 VALUES ($1, $2)"
            )
            .bind(product_id)
            .bind(quantity)
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;

    Ok(delivery)
}


    async fn get_deliveries_by_user(&self, user_id: Uuid) -> Result<Vec<Delivery>, sqlx::Error> {
    let deliveries = sqlx::query_as::<_, Delivery>(
        "SELECT * FROM deliveries WHERE userid = $1 ORDER BY date DESC"
    )
    .bind(user_id)
    .fetch_all(&self.pool)
    .await?;
    Ok(deliveries)
}

}

#[async_trait]
pub trait TruckLoadExt {
    async fn create_truck_load(
        &self,
        user_id: Uuid,
        truck_id: Uuid,
        date: NaiveDate,
        products: Vec<(Uuid, i32)>, // (product_id, quantity)
    ) -> Result<TruckLoad, sqlx::Error>;

    async fn get_all_truck_loads(&self) -> Result<Vec<TruckLoad>, sqlx::Error>;
}

#[async_trait]
impl TruckLoadExt for DBClient {
    async fn create_truck_load(
        &self,
        user_id: Uuid,
        truck_id: Uuid,
        date: NaiveDate,
        products: Vec<(Uuid, i32)>,
    ) -> Result<TruckLoad, sqlx::Error> {
        let mut tx: Transaction<'_, Postgres> = self.pool.begin().await?;

        // 1. Insert into truck_load table
        let truck_load = sqlx::query_as::<_, TruckLoad>(
            "INSERT INTO truck_loads (date, userid, truckid)
             VALUES ($1, $2, $3)
             RETURNING *"
        )
        .bind(date)
        .bind(user_id)
        .bind(truck_id)
        .fetch_one(&mut *tx)
        .await?;

        // 2. Loop through products to insert into truckload_products and decrease warehouse_stock
        for (product_id, quantity) in &products {
            // Check warehouse stock
            let current_stock: i32 = sqlx::query_scalar(
                "SELECT quantity FROM warehouse_stock WHERE productid = $1"
            )
            .bind(product_id)
            .fetch_one(&mut *tx)
            .await?;

            if *quantity > current_stock {
                return Err(sqlx::Error::RowNotFound); // or a custom error
            }

            // Insert into truckload_products
            sqlx::query(
                "INSERT INTO truck_load_products (truckloadid, productid, quantity)
                 VALUES ($1, $2, $3)"
            )
            .bind(truck_load.truckloadid)
            .bind(product_id)
            .bind(quantity)
            .execute(&mut *tx)
            .await?;

            // Decrease warehouse stock
            sqlx::query(
                "UPDATE warehouse_stock
                 SET quantity = quantity - $2
                 WHERE productid = $1"
            )
            .bind(product_id)
            .bind(quantity)
            .execute(&mut *tx)
            .await?;
        }

        // 3. Commit transaction
        tx.commit().await?;

        Ok(truck_load)
    }

    async fn get_all_truck_loads(&self) -> Result<Vec<TruckLoad>, sqlx::Error> {
        let truck_loads = sqlx::query_as::<_, TruckLoad>(
            "SELECT * FROM truck_loads ORDER BY date DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(truck_loads)
    }
}


#[async_trait]
pub trait SalesExt {
    async fn create_sale(
        &self,
        truckload_id: Uuid,
        shop_id: Uuid,
        date: NaiveDate,
        products: Vec<(Uuid, i32)>, // (product_id, quantity)
    ) -> Result<Sale, sqlx::Error>;

}
#[async_trait]
impl SalesExt for DBClient {
    async fn create_sale(
        &self,
        truckload_id: Uuid,
        shop_id: Uuid,
        date: NaiveDate,
        products: Vec<(Uuid, i32)>, // (product_id, quantity)
    ) -> Result<Sale, sqlx::Error> {
        let mut tx: Transaction<'_, Postgres> = self.pool.begin().await?;

        // ✅ Step 1: Calculate total amount
        let mut total_amount: f64 = 0.0;

        for (product_id, quantity) in &products {
            let price: f64 = sqlx::query_scalar(
                "SELECT price FROM products WHERE id = $1"
            )
            .bind(product_id)
            .fetch_one(&mut *tx)
            .await?;

            total_amount += price * (*quantity as f64);
        }

        // ✅ Step 2: Insert into sales table with total_amount and paid_amount = 0
        let sale = sqlx::query_as::<_, Sale>(
            "INSERT INTO sales (truckloadid, shopid, date, status, total_amount, paid_amount)
             VALUES ($1, $2, $3, 'pending', $4, 0)
             RETURNING *"
        )
        .bind(truckload_id)
        .bind(shop_id)
        .bind(date)
        .bind(total_amount)
        .fetch_one(&mut *tx)
        .await?;

        // ✅ Step 3: Insert into sales_product table
        for (product_id, quantity) in &products {
            sqlx::query(
                "INSERT INTO sales_product (salesid, productid, quantity)
                 VALUES ($1, $2, $3)"
            )
            .bind(sale.salesid)
            .bind(product_id)
            .bind(quantity)
            .execute(&mut *tx)
            .await?;
        }

        // ✅ Step 4: Commit transaction
        tx.commit().await?;

        Ok(sale)
    }
}




#[async_trait]
pub trait PaymentExt {
    async fn create_payment(
        &self,
        salesid: Uuid,
        amount: f64,
        method: String,
        date: NaiveDate,
    ) -> Result<Payment, sqlx::Error>;

}

#[async_trait]
impl PaymentExt for DBClient {
   async fn create_payment(
    &self,
    salesid: Uuid,
    amount: f64,
    method: String,
    date: NaiveDate,
) -> Result<Payment, sqlx::Error> {
    let mut tx: Transaction<'_, Postgres> = self.pool.begin().await?;

    // 1️⃣ Insert payment
    let payment = sqlx::query_as::<_, Payment>(
        "INSERT INTO payment (salesid, amount, method, date, created_at, updated_at)
         VALUES ($1, $2, $3, $4, NOW(), NOW())
         RETURNING *"
    )
    .bind(salesid)
    .bind(amount)
    .bind(method)
    .bind(date)
    .fetch_one(&mut *tx)
    .await?;

    // 2️⃣ Update paid_amount in sales table
    sqlx::query(
        "UPDATE sales
         SET paid_amount = COALESCE(paid_amount, 0) + $1,
             updated_at = NOW()
         WHERE salesid = $2"
    )
    .bind(amount)
    .bind(salesid)
    .execute(&mut *tx)
    .await?;

     sqlx::query(
        "UPDATE sales
         SET status = 'paid', updated_at = NOW()
         WHERE salesid = $1 AND COALESCE(paid_amount, 0) >= COALESCE(total_amount, 0)"
    )
    .bind(salesid)
    .execute(&mut *tx)
    .await?;

    // 3️⃣ Commit transaction
    tx.commit().await?;

    Ok(payment)
}


}


#[async_trait]
pub trait AllowanceExt {
    async fn create_allowance(
        &self,
        date: NaiveDate,
        amount: f64,
        notes: Option<String>,
    ) -> Result<Allowance, sqlx::Error>;
    async fn create_truck_allowance(
        &self,
        allowanceid: Uuid,
        truckid: Uuid,
        amount: f64,
    ) -> Result<TruckAllowance, sqlx::Error>;
}

#[async_trait]
impl AllowanceExt for DBClient {
    async fn create_allowance(
        &self,
        date: NaiveDate,
        amount: f64,
        notes: Option<String>,
    ) -> Result<Allowance, sqlx::Error> {
        let mut tx: Transaction<'_, Postgres> = self.pool.begin().await?;

        let allowance = sqlx::query_as::<_, Allowance>(
            "INSERT INTO allowance (date, amount, notes, created_at, updated_at)
             VALUES ($1, $2, $3, NOW(), NOW())
             RETURNING *"
        )
        .bind(date)
        .bind(amount)
        .bind(notes)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(allowance)
    }

   async fn create_truck_allowance(
        &self,
        allowanceid: Uuid,
        truckid: Uuid,
        amount: f64,
    ) -> Result<TruckAllowance, sqlx::Error> {
        let mut tx: Transaction<'_, Postgres> = self.pool.begin().await?;

        let truck_allowance = sqlx::query_as::<_, TruckAllowance>(
            "INSERT INTO truck_allowance (allowanceid, truckid, amount, created_at, updated_at)
             VALUES ($1, $2, $3, NOW(), NOW())
             RETURNING *"
        )
        .bind(allowanceid)
        .bind(truckid)
        .bind(amount)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(truck_allowance)
    }

}