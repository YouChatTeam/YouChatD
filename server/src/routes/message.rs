use serde::{Deserialize, Serialize};
use actix_web::{get, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::models::message::Message;
use crate::models::user::User;
use crate::database::Context;

#[get("/messages/{id}")]
async fn get_message(id: web::Path<Uuid>, ctx: web::Data<Context>) -> impl Responder {
    let redis_key = format!("message:{}", id);
    let redis_connection = ctx.get_redis_connection();
    // borrowing the key since we will use the key to search cassandra if does not exist in redis
    let redis_result = redis_connection.get(&redis_key).await;

    match redis_result {
        Ok(redis_value) => {
            if let Some(redis_value) = redis_value {
                let message: Message = serde_json::from_slice(&redis_value).unwrap();
                return HttpResponse::Ok().json(message);
            }
        }
        Err(e) => {
            eprintln!("Error fetching message from Redis: {}", e);
        }
    }

    // If not found or error, fallback to Cassandra
    let cassandra_session = ctx.get_cassandra_session();
    let cassandra_result = cassandra_session.execute(
        "SELECT * FROM messages WHERE id = ?",
        (id,),
    ).await;


    match cassandra_result {
        Ok(rows) => {
            // If the message is found in Cassandra, return it and update Redis
            if let Some(row) = rows.into_rows().first() {
                let message = Message::from_row(row);
                let message_json = serde_json::to_string(&message).unwrap();

                // move to cache
                redis_connection.set(&redis_key, message_json).await.unwrap();

                // return message
                return HttpResponse::Ok().json(message);
            }
        }
        Err(e) => {
            // Log the error
            eprintln!("Error fetching message from Cassandra: {}", e);
        }
    }

    // If the message is not found in Redis or Cassandra, return a 404 error
    HttpResponse::NotFound().finish()
}

#[post("/messages")]
async fn create_message(message: web::Json<Message>, ctx: web::Data<Context>) -> impl Responder {
    let message_id = Uuid::new_v4();
    let created_at = Utc::now();

    let message = Message {
        id: message_id,
        user: message.user.clone(),
        body: message.body.clone(),
        created_at: created_at.clone(),
    };

    // Store message in Redis cache
    let redis_connection = ctx.get_redis_connection();
    let redis_result = redis_connection.set(format!("message:{}", message_id), message.to_json());
    if let Err(e) = redis_result {
        error!("Failed to store message in Redis: {}", e);
    }

    // Store message in Cassandra database
    let cassandra_session = ctx.get_cassandra_session();
    let cassandra_result = cassandra_session.insert_message(message.clone()).await;
    if let Err(e) = cassandra_result {
        error!("Failed to store message in Cassandra: {}", e);
    }

    // Return success message as JSON
    let response = json!({
        "status": "success",
        "message": "Message created successfully",
        "id": message_id,
        "created_at": created_at.to_rfc3339(),
    });

    Ok(HttpResponse::Ok().json(response))
}