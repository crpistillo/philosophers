extern crate std_semaphore;
extern crate rand;

use std_semaphore::Semaphore;
use std::thread;
use std::time::Duration;
use rand::{thread_rng, Rng};
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

fn main() {

    let philosophers_eating= Arc::new(RwLock::new(vec!(false, false, false, false, false)));

    let fork = Arc::new(vec![
        Semaphore::new(1),
        Semaphore::new(1),
        Semaphore::new(1),
        Semaphore::new(1),
        Semaphore::new(1),
    ]);

    let mut handles = vec![];
    for i in 0..5 {
        let fork_clone = Arc::clone(&fork);
        let philosophers_eating_clone = philosophers_eating.clone();

        let handle = thread::spawn(move || {
            let num = i;

            thread::sleep(Duration::from_millis(thread_rng().gen_range(1000, 2000)));

            loop {

                if let Ok(mut states_mut) = philosophers_eating_clone.write()
                {
                    println!("{:?}", *states_mut);
                    println!("Filosofo {} tratando de agarrar palito", num);
                    if(states_mut[i] == false && states_mut[(i + 1) % 5] == false && states_mut[(i + 4) % 5] == false)
                    {
                        states_mut[i] = true;
                        fork_clone.get(num).unwrap().acquire();
                        println!("Filosofo {} tomando palito {} ", num, num);
                        fork_clone.get((num + 1) % 5).unwrap().acquire();
                        println!("Filosofo {} tomando palito {} ", num, (num + 1) % 5);

                        println!("Filosofo {} comiendo!", num);
                    }
                }

                thread::sleep(Duration::from_millis(thread_rng().gen_range(3000, 4000)));

                if let Ok(mut states_mut) = philosophers_eating_clone.write()
                {
                    if(states_mut[i] == true)
                    {
                        println!("{:?}", *states_mut);
                        fork_clone.get(num).unwrap().release();
                        println!("Filosofo {} soltó palito {} ", num, num);
                        fork_clone.get((num + 1) % 5).unwrap().release();
                        println!("Filosofo {} soltó palito {} ", num, (num + 1) % 5);
                        states_mut[i] = false;

                        println!("Filosofo {} pensando!", num);
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}