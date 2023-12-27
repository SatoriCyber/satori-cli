use super::{List, errors::ListErrors};

pub fn run(params: List) -> Result<(), ListErrors>{
    match params {
        List::Datastores => {
            handle_datastores()
        }
    }
}

fn handle_datastores() -> Result<(), ListErrors> {
    let datastores = crate::helpers::datastores::file::load()?;
    let datastores_name = datastores.value.keys().map(|d| {
        d.to_string()
}).collect::<Vec<String>>().join("\n");
    println!("{datastores_name}");    
    Ok(())
}