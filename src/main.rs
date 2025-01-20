use std::{i32, vec};
use chrono::{DateTime, Utc};
use order_programation::{time_order_by_priority, Timed};


#[derive(Debug, Clone)]
struct Obj {
    id: i32,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    description: String,
    priority: i32,
}

impl Timed<DateTime<Utc>> for Obj {
    fn get_start(&self) -> DateTime<Utc> {
        self.start
    }
    fn get_end(&self) -> DateTime<Utc> {
        self.end
    }
    fn set_start(&mut self, time: DateTime<Utc>) {
        self.start = time;
    }
    fn set_end(&mut self, time: DateTime<Utc>) {
        self.end = time;
    }
    fn get_priority(&self) -> u32 {
        self.priority as u32
    }
}

fn generate_list() -> Vec<Obj> {
    vec![
        Obj {
            id: 1,
            start: "2025-01-17 12:00:00.0 UTC".parse::<DateTime<Utc>>().unwrap(),
            end: "2025-01-17 15:00:00.0 UTC".parse::<DateTime<Utc>>().unwrap(),
            description: "A".to_string(),
            priority: 1,
        },
        Obj {
            id: 2,
            start: "2025-01-17 13:30:00.0 UTC".parse::<DateTime<Utc>>().unwrap(),
            end: "2025-01-17 15:30:00.0 UTC".parse::<DateTime<Utc>>().unwrap(),
            description: "B".to_string(),
            priority: 2,
        },
        Obj {
            id: 3,
            start: "2025-01-17 12:30:00.0 UTC".parse::<DateTime<Utc>>().unwrap(),
            end: "2025-01-17 14:30:00.0 UTC".parse::<DateTime<Utc>>().unwrap(),
            description: "C".to_string(),
            priority: 3,
        },
        Obj {
            id: 4,
            start: "2025-01-17 13:00:00.0 UTC".parse::<DateTime<Utc>>().unwrap(),
            end: "2025-01-17 14:00:00.0 UTC".parse::<DateTime<Utc>>().unwrap(),
            description: "D".to_string(),
            priority: 4,
        },
        // Obj {
        //     id: 5,
        //     start: "2025-01-17 16:00:00.0 UTC".parse::<DateTime<Utc>>().unwrap(),
        //     end: "2025-01-17 16:30:00.0 UTC".parse::<DateTime<Utc>>().unwrap(),
        //     description: "E".to_string(),
        //     priority: 1,
        // },
    ]
}

fn main() {

    let vec = generate_list();
    let vec = time_order_by_priority(vec);

    for item in vec {
        println!("{:?} - {} - {}", item, item.id, item.description);
    }
    
    

}
