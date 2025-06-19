use crate::leibniz_pi_partial;

pub struct AsyncOperations;

impl AsyncOperations {
    pub(crate) async fn execute_program(&self) {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    pub(crate) async fn leibniz_operation(&self, n: usize, tasks: usize) -> f64 {
        let terms_per_task = (n + 1) / tasks;
        let mut handles = Vec::new();

        for i in 0..tasks {
            handles.push(tokio::spawn(async move {
                leibniz_pi_partial(terms_per_task * i, terms_per_task)
            }));
        }

        let mut leibniz_sum = 0.0;
        for handle in handles {
            leibniz_sum += handle.await.unwrap();
        }

        leibniz_sum
    }
}