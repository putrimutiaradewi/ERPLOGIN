
use std::sync::Arc;

use sqlx::{Pool,Postgres};
use be_erp::{table_list, add_table, update_table, delete_table, add_account, update_Password, user_account};
use tide::{Server};

pub fn path(app: &mut Server<Pool<Postgres>>){
app.at("cek").get(table_list);
app.at("add").post(add_table);
app.at("update").put(update_table);
app.at("delete").delete(delete_table);
app.at("Register").post(add_account);
app.at("Lupa").put(update_Password);
app.at("Login").post (user_account);


}
