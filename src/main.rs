use std::{fs, io::Write, ops::Deref, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task::JoinHandle, time};
use rand::{rngs::OsRng, Rng};
use serde::Serialize;

mod format;

#[tokio::main]
async fn main() {
    
    let input_file = fs::read_to_string("./Localization.json").expect("Failed to read input file");
    let mut L: format::L = serde_json::from_str(&input_file).expect("Failed to deserialize JSON");
    let lc = Arc::new(Mutex::new(L));

    let mut tasks = vec![];

    {
        let lc = Arc::clone(&lc);
        let mut lc_guard = lc.lock().await;

        for tab in &mut lc_guard.tables {
            for ent in &mut tab.entries {
                for values in &mut ent.values {
                    let value = Arc::new(Mutex::new(values));
                    let task = tokio::spawn(async move {
                        time::sleep(Duration::from_secs(OsRng.gen_range(0..1))).await;
                        let mut value = value.lock().await;
                        println!("val {:?}", &value);
                        value.value = "ssdf".to_string();
                        println!("val {:?}", &value.value);
                    });
                    tasks.push(task);
                }
            }
        }
    }

    
    for task in tasks {
        task.await.expect("Task failed");
    }
    
    println!("{:?}",lc);
    
    let output_json = serde_json::to_string_pretty(&*lc.lock().await).expect("Failed to serialize JSON");
    //println!("{}",output_json);
    
    let mut output_file = fs::File::create("./Localization_output.json").expect("Failed to create output file");
    output_file.write_all(output_json.as_bytes()).expect("Failed to write to output file");
}