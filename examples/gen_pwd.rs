use todo_list_v1::utils::crypto::{encode_password, verify_password};

fn main() {
    let pwd = "123456";
    let hash_password = encode_password(pwd).unwrap();
    println!("Password: {pwd}");
    println!("Hash: {hash_password}");
    println!("hash len = {:?}", hash_password.len()); // 105
    let res = verify_password(pwd, &hash_password).unwrap();
    println!("Password: {res}");
}
