

// insertable which tells diesel we can insert into database table, json can also be transformed into new rustacean object
// thisis what we will require from api clients when inserting into the database
#[derive(serde::Deserialize, Debug, Serialize)]
pub struct SealedData {
    pub sealed_data_received: Vec<u8>, // only these two because the id and created_at are pushed in automatically
}

#[derive(serde::Deserialize, Debug)]
pub struct UnsealedData {
    pub unsealed_data_received: Vec<u8>, // only these two because the id and created_at are pushed in automatically
}
