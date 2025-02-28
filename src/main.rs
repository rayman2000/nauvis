mod parsing;
mod entities;

use parsing::{decode_blueprint, get_recipes_map};


fn main() {
    
    let recipes = get_recipes_map().unwrap();
    println!("{:?}", recipes);
    
    let encoded = "0eJyNk9FugzAMRf/Fz6QqFNrB435jqqpAXWopJCgJ26qKf59DWoTaatsTENv32M7lCrUasLekPVRXoMZoB9XHFRy1Wqpw5i89QgXksYMEtOymL+3QerQwJvx+xG+o0nGfAGpPnjBq3HKlc9jVinQrOtmcSaNIWak3jlONDgwuF9tVkcCFdVYFix7JYhPD65vs5aCHrmZmlSbAUZr6+jLmiFo0Z3Seu5mpqLjeUiNOg9WywRfE9f+J2UJ54Hltaw0/RY3KPyvnN2GRvVC+71P3Qyh9AG0WIJ6po0Yq0SupX2CK//efLzbGas7zYmpplwtzvSIfrvSJk8ZhNn9jiiTOdWBHGcsR1rXUnkPzZvAPEYWnEDiRCth4lcuOYkDMXnu+wV8X/dDb9k6K5ry7drZ0pO+DiyevV4tfI4FPrpuEs7c035XZLs/KclNyuZJsAs5+n7PH8Qc84Bo2";
    match decode_blueprint(encoded) {
        Ok(decoded) => println!("Decoded: {:?}", decoded),
        Err(e) => eprintln!("Error decoding: {}", e),
    }
}
