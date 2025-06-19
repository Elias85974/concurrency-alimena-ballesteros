use crate::async_operations::AsyncOperations;
use crate::thread_operations::ThreadOperations;
use std::{env, thread};
use std::sync::Arc;
use std::time::Instant;

mod async_operations;
mod thread_operations;

fn main() {
    // Obtener argumentos desde la línea de comandos
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Uso: {} <mode> <n> <m>", args[0]);
        eprintln!("mode: async o thread");
        eprintln!("n: número de llamadas a execute_program");
        eprintln!("m: cantidad de términos para el cálculo de Pi");
        return;
    }

    let mode = &args[1];
    let tasks: usize = args[2].parse().expect("El argumento 'n' debe ser un número entero");
    let terms: usize = args[3].parse().expect("El argumento 'm' debe ser un número entero");

    // Seleccionar la implementación según el modo
    let time = Instant::now();
    match mode.as_str() {
        "async" => {
            async_operations(tasks, terms);
        }
        "thread" => {
            thread_operations(tasks, terms);
        }
        _ => {
            eprintln!("Modo no válido. Usa 'async' o 'thread'.");
            return;
        }
    }

    println!("Tiempo transcurrido: {:?}", time.elapsed());
}

fn thread_operations(tasks: usize, terms: usize) {
    let operation = Arc::new(ThreadOperations);
    let leibniz_operation = operation.clone();
    let leibniz_task = thread::spawn(move || { leibniz_operation.leibniz_operation(terms, 8) });
    for _ in 0..tasks {
        operation.execute_program();
    }
    let leibniz_result = leibniz_task.join().unwrap();
    println!("Resultado de Leibniz: {:?}", leibniz_result);
}

#[tokio::main]
async fn async_operations(tasks: usize, terms: usize) {
    let operation = Arc::new(AsyncOperations);
    let leibniz_task = {
        let leibniz_operation = operation.clone();
        tokio::spawn(async move {
            leibniz_operation.leibniz_operation(terms, 8).await
        })
    };
    
    let mut handles = Vec::new();
    for _ in 0..tasks {
        let op = operation.clone();
        handles.push(tokio::spawn(async move {
            op.execute_program().await;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let leibniz_result = leibniz_task.await.unwrap();
    println!("Resultado de Leibniz: {:.10}", leibniz_result);
}

pub fn leibniz_pi_partial ( start : usize , count : usize ) -> f64 {
    ( start .. start + count )
        .map (| k | {
            let k = k as f64 ;
            ( -1.0f64 ) . powf ( k ) / (2.0 * k + 1.0)
        })
        .sum :: < f64 >() * 4.0
}