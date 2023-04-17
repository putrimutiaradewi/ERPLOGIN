use serde::Deserialize;
use sqlx::PgPool;
use tide::{Request, Response, Body, http};
use crate::ws_response;

#[derive(serde::Serialize, Debug ,Deserialize)]
struct Get {
    nama : Option<String>,
}

#[derive(serde::Serialize, Debug ,Deserialize)]
struct Test {
    nama : Option<String>,
    buku_id : Option<i32>,
}


#[derive(serde::Serialize, Debug ,Deserialize)]
struct Buku {
    buku_id : Option<i32>,
    nama_buku : Option<String>,
}

#[derive(serde::Serialize, Debug ,Deserialize)]
struct DelParam {
    buku_id : Option<i32>
}

#[derive(serde::Serialize, Debug ,Deserialize)]
struct Account {
    Username : String,
    Password : String,
    Email : String,

}   
#[derive(serde::Serialize, Debug ,Deserialize)]
struct LoginResult {
    status : String,
    info : String,
}




pub async fn table_list(req : Request<PgPool>) -> tide::Result<Response>{
    let param : Get = req.query()?;
    let pool = req.state();
    let nama :Vec<Test> = sqlx::query_as!(
Test,"SELECT nama,buku_id from table1 where nama=$1", param.nama)
.fetch_all(pool).await?;
println!("table : {:#?} ", nama);

let response = Response::builder(200)
            .body(Body::from_json(&nama)? ).build();
        Ok(response)
}


pub async fn add_table(mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Buku = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("INSERT INTO table2 (buku_id,nama_buku) VALUES($1,$2);")
     .bind(param.buku_id)
     .bind(param.nama_buku)
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil insert ke table2")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal insert ke table2")
        }

     }

}

pub async fn update_table (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Buku = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("UPDATE table2 SET nama_buku=$2 WHERE buku_id=$1")
     .bind(param.buku_id)
     .bind(param.nama_buku)
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil Update ke table2")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal Update ke table2")
        }

     }

}

pub async fn delete_table ( req : Request<PgPool>) -> tide::Result<Response> {
    match req.query(){
        Ok(x) => {
            let param : DelParam =x;
            let pool = req.state();
             
             match
             sqlx::query("DELETE FROM table2 WHERE buku_id=$1")
             .bind(param.buku_id)
             .execute(pool).await
             {
                Ok(_x) => {ws_response("OK", "Berhasil Delete ke table2")},
                Err(e) => {
                    println!("error delete : {:?}",e);
                    ws_response("Error", "Gagal Delete ke table2")
                }
        
             }
        }
        Err(e) => {
            println!("Error : {:?}",e);
            let msg = format!("{:?}",e);
            ws_response("Error", msg.as_str())

        }
    }


   



}
pub async fn add_account (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Account = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("INSERT INTO login (Username, Email, Password) VALUES ($1,$2,sha256($3));")
     .bind(param.Username)
     .bind(param.Email)
     .bind(param.Password.as_bytes())
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil login")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal login")
        }

     }

}

pub async fn user_account (mut req : Request<PgPool>) -> tide::Result<Response> {
    let form: Account = req.body_json().await?;
    let pool = req.state();
    let mut resp = Response::new(http::StatusCode::Ok);
    if let Ok (record) = sqlx::query!(
        "SELECT username FROM login WHERE username = $1 and password=sha256($2::text::bytea)",
        form.Username, form.Password
    ).fetch_one(pool).await{

       let ret = LoginResult {
        status:"Ok".to_string(),
        info:"Login Berhasil".to_string(),
       };

       resp.set_status(200);
       resp.set_body(Body::from_json(&ret)?);
    }else {
        
        let ret = serde_json::json!({
        "status": "Error",
        "info": "Username/Password is invalid"
    });
    resp.set_status(http::StatusCode::Ok);
    resp.set_body(Body::from_json(&ret)?);
    };

    Ok(resp)
}
    

pub async fn update_Password (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Account = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("UPDATE login SET Password=sha256($3) WHERE Username=$1")
     .bind(param.Username)
     .bind(param.Email)
     .bind(param.Password.as_bytes())
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil Update")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal Update ")
        }

     }

}
