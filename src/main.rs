use std::collections::HashMap;
use easyjson_rs::useful_kt_extensions::KotlinScopeFunction;

fn main() {
    let i = HashMap::new().kotlin_also_by_mut_ref(|x| {
        for i in 1..10{
            x.insert(i,i.to_string());
        }
    });
    for (&k,v) in i.iter(){
        println!("{} {}",k,v);
    }
}
