#[cfg(test)]
mod time_order_by_priority {

    use chrono::NaiveTime;
    use crate::{time_order_by_priority,Timed};

    #[derive(Debug, Clone, PartialEq)]
    #[allow(dead_code)]
    struct Obj {
        start: NaiveTime,
        end: NaiveTime,
        description: String,
        priority: i32,
    }

    impl Timed<NaiveTime> for Obj {
        fn get_start(&self) -> NaiveTime {
            self.start
        }
        fn get_end(&self) -> NaiveTime {
            self.end
        }
        fn set_start(&mut self, time: NaiveTime) {
            self.start = time;
        }
        fn set_end(&mut self, time: NaiveTime) {
            self.end = time;
        }
    }

    impl PartialOrd for Obj {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.priority.cmp(&other.priority))
        }
    }

    /// This will test if the tester object is working as expected for the test cases
    #[test]
    fn test_obj() {
        let mut vec = vec![
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "15:00:00".parse().unwrap(),
                description: "A".to_string(),
                priority: 1,
            },
            Obj {
                start: "14:00:00".parse().unwrap(),
                end: "16:00:00".parse().unwrap(),
                description: "B".to_string(),
                priority: 2,
            },
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "15:00:00".parse().unwrap(),
                description: "A".to_string(),
                priority: 1,
            }
        ];

        assert_eq!(vec[0], vec[2]);
        assert_ne!(vec[0], vec[1]);
        assert_ne!(vec[2], vec[1]);

        assert!(vec[0] < vec[1]);
        assert!(vec[2] < vec[1]);

        vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less));

        assert_eq!(vec[0], vec[1]);
        assert_ne!(vec[0], vec[2]);
        assert_ne!(vec[1], vec[2]);

        assert!(vec[0] < vec[2]);
        assert!(vec[1] < vec[2]);

    }

    
    /// This test will check if the function time_order_by_priority will return the same object
    /// on the trivial case of the input having only one object. The same instace of object should be returned
    #[test]
    fn test_do_nothing() {
        let prograns = vec![
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "15:00:00".parse().unwrap(),
                description: "A".to_string(),
                priority: 1,
            }
        ];
        
        let original_obj_prt = prograns.get(0).unwrap() as *const Obj as usize;
        let ordered = time_order_by_priority(prograns);
        assert!(original_obj_prt == ordered.get(0).unwrap() as *const Obj as usize);
    }

    /// This test has 2 objects with a overlap of 1 hour (14:00 to 15:00)
    /// 
    ///              | -------- B -------- |
    ///   | -------- A -------- |
    /// 
    /// 12:00      14:00      15:00      16:00
    /// 
    /// The expected result is:
    /// 
    ///   | --- A --- | -------- B -------- |
    /// 12:00      14:00      15:00   16:00
    /// 
    /// This because the object B has a higher priority than A
    #[test]
    fn test_simple_case() {
        let prograns = vec![
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "15:00:00".parse().unwrap(),
                description: "A".to_string(),
                priority: 1,
            },
            Obj {
                start: "14:00:00".parse().unwrap(),
                end: "16:00:00".parse().unwrap(),
                description: "B".to_string(),
                priority: 2,
            }
        ];

        let ordered = time_order_by_priority(prograns);
        assert_eq!(ordered.len(), 2);
        let obj = ordered.get(0).unwrap();
        assert!(obj.description == "A");
        assert!(obj.start == "12:00:00".parse().unwrap());
        assert!(obj.end == "14:00:00".parse().unwrap());

        let obj = ordered.get(1).unwrap();
        assert!(obj.description == "B");
        assert!(obj.start == "14:00:00".parse().unwrap());
        assert!(obj.end == "16:00:00".parse().unwrap());
    }

    /// Same as the test_simple_case but the priority is inverted
    /// 
    ///   | -------- A -------- |
    ///              | -------- B -------- |
    /// 
    /// 12:00      14:00      15:00      16:00
    /// 
    /// so the expected result is:
    ///   | -------- A -------- | -- B --- |
    /// 12:00      14:00      15:00      16:00
    #[test]
    fn test_simple_case_inverse() {
        let prograns = vec![
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "15:00:00".parse().unwrap(),
                description: "A".to_string(),
                priority: -1,
            },
            Obj {
                start: "14:00:00".parse().unwrap(),
                end: "16:00:00".parse().unwrap(),
                description: "B".to_string(),
                priority: -2,
            }
        ];

        let ordered = time_order_by_priority(prograns);
        assert_eq!(ordered.len(), 2);
        let obj = ordered.get(0).unwrap();
        assert!(obj.description == "A");
        assert!(obj.start == "12:00:00".parse().unwrap());
        assert!(obj.end == "15:00:00".parse().unwrap());

        let obj = ordered.get(1).unwrap();
        assert!(obj.description == "B");
        assert!(obj.start == "15:00:00".parse().unwrap());
        assert!(obj.end == "16:00:00".parse().unwrap());
    }
    
    #[test]
    /// This test has 2 objects with no overlap
    /// so the expected result is the same as the input
    ///   | -------- A -------- |       | -------- B -------- |
    /// 12:00                 13:00   14:00                 15:00
    /// 
    /// expected result:
    ///   | -------- A -------- |       | -------- B -------- |
    /// 12:00                 13:00   14:00                 15:00
    fn test_non_overlap_case() {
        let prograns = vec![
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "13:00:00".parse().unwrap(),
                description: "A".to_string(),
                priority: 0,
            },
            Obj {
                start: "14:00:00".parse().unwrap(),
                end: "15:00:00".parse().unwrap(),
                description: "B".to_string(),
                priority: 0,
            }
        ];

        let ordered = time_order_by_priority(prograns);
        assert_eq!(ordered.len(), 2);
        let obj = ordered.get(0).unwrap();
        assert!(obj.description == "A");
        assert!(obj.start == "12:00:00".parse().unwrap());
        assert!(obj.end == "13:00:00".parse().unwrap());

        let obj = ordered.get(1).unwrap();
        assert!(obj.description == "B");
        assert!(obj.start == "14:00:00".parse().unwrap());
        assert!(obj.end == "15:00:00".parse().unwrap());
    }


    /// This test has 2 objects with a overlap of 1 hour (14:00 to 15:00) and 
    /// one object with no overlap
    /// 
    ///              | -------- B -------- |
    ///   | -------- A -------- |                 | -------- C -------- |
    /// 
    /// 12:00      14:00      15:00      16:00  17:00                 18:00
    /// 
    /// The expected result is:
    /// 
    ///   | -- A --- | -------- B -------- |      | -------- C -------- |
    /// 12:00      14:00      15:00      16:00  17:00                 18:00
    /// 
    /// This because the object B has a higher priority than A
    #[test]
    fn test_simple_case_with_non_overlap() {
        let prograns = vec![
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "15:00:00".parse().unwrap(),
                description: "A".to_string(),
                priority: 1,
            },
            Obj {
                start: "14:00:00".parse().unwrap(),
                end: "16:00:00".parse().unwrap(),
                description: "B".to_string(),
                priority: 2,
            },
            Obj {
                start: "17:00:00".parse().unwrap(),
                end: "18:00:00".parse().unwrap(),
                description: "C".to_string(),
                priority: 1,
            }
        ];

        let ordered = time_order_by_priority(prograns);

        assert_eq!(ordered.len(), 3);
        let obj = ordered.get(0).unwrap();
        assert!(obj.description == "A");
        assert!(obj.start == "12:00:00".parse().unwrap());
        assert!(obj.end == "14:00:00".parse().unwrap());

        let obj = ordered.get(1).unwrap();
        assert!(obj.description == "B");
        assert!(obj.start == "14:00:00".parse().unwrap());
        assert!(obj.end == "16:00:00".parse().unwrap());

        let obj = ordered.get(2).unwrap();
        assert!(obj.description == "C");
        assert!(obj.start == "17:00:00".parse().unwrap());
        assert!(obj.end == "18:00:00".parse().unwrap());
    }

    /// This test the program will start a new program in the middle of another
    /// and finish before the first one ends so the first one should be split in 2
    /// because the new program has a higher priority
    /// 
    ///           |-- B --|
    ///   |---------- A ----------|
    /// 12:00   14:00   15:00   18:00  
    /// 
    /// The expected result is:
    /// 
    ///   |-- A --|-- B --|-- A --|
    /// 12:00   14:00   15:00   18:00  
    ///
    #[test]
    fn test_a_program_in_the_middle_of_another() {

        let prograns = vec![
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "18:00:00".parse().unwrap(),
                description: "A".to_string(),
                priority: 1,
            },
            Obj {
                start: "14:00:00".parse().unwrap(),
                end: "15:00:00".parse().unwrap(),
                description: "B".to_string(),
                priority: 2,
            }
        ];

        let ordered = time_order_by_priority(prograns);

        assert_eq!(ordered.len(), 3);
        let obj = ordered.get(0).unwrap();
        assert!(obj.description == "A");
        assert!(obj.start == "12:00:00".parse().unwrap());
        assert!(obj.end == "14:00:00".parse().unwrap());

        let obj = ordered.get(1).unwrap();
        assert!(obj.description == "B");
        assert!(obj.start == "14:00:00".parse().unwrap());
        assert!(obj.end == "15:00:00".parse().unwrap());

        let obj = ordered.get(2).unwrap();
        assert!(obj.description == "A");
        assert!(obj.start == "15:00:00".parse().unwrap());
        assert!(obj.end == "18:00:00".parse().unwrap());
    }
    
    /// Same as the test_a_program_in_the_middle_of_another but the priority is inverted so only the first
    /// program should be returned
    /// 
    ///   |---------- A ----------|
    ///           |-- B --|
    /// 12:00   14:00   15:00   18:00  
    /// 
    /// The expected result is:
    /// 
    ///   |---------- A ----------|
    /// 12:00   14:00   15:00   18:00  
    /// 
    #[test]
    fn test_a_program_in_the_middle_of_another_inverted() {

        let prograns = vec![
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "18:00:00".parse().unwrap(),
                description: "A".to_string(),
                priority: std::i32::MAX,
            },
            Obj {
                start: "14:00:00".parse().unwrap(),
                end: "15:00:00".parse().unwrap(),
                description: "B".to_string(),
                priority: 2,
            }
        ];

        let ordered = time_order_by_priority(prograns);

        assert_eq!(ordered.len(), 1);
        let obj = ordered.get(0).unwrap();
        assert!(obj.description == "A");
        assert!(obj.start == "12:00:00".parse().unwrap());
        assert!(obj.end == "18:00:00".parse().unwrap());
    }

    /// This will test the case of a program starting on the same time another is finishing. 3 sub cases will be tested
    /// A and B has sema priority, A has higher priority than B and last case B has higher priority than A
    /// 
    /// On all cases the expected result is the same and it should be equal to the input and no extra cases should be created
    #[test]
    fn test_non_overlap_case_with_same_start() {

        let prograns = vec![
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "13:00:00".parse().unwrap(),
                description: "A".to_string(),
                priority: 0,
            },
            Obj {
                start: "13:00:00".parse().unwrap(),
                end: "14:00:00".parse().unwrap(),
                description: "B".to_string(),
                priority: 0,
            }
        ];

        let mut prograns = time_order_by_priority(prograns);

        let mut iter = prograns.iter();
        let item = iter.next().unwrap();

        assert_eq!(item, &Obj {
            start: "12:00:00".parse().unwrap(),
            end: "13:00:00".parse().unwrap(),
            description: "A".to_string(),
            priority: 0,
        });
        let item = iter.next().unwrap();
        assert_eq!(item, &Obj {
            start: "13:00:00".parse().unwrap(),
            end: "14:00:00".parse().unwrap(),
            description: "B".to_string(),
            priority: 0,
        });
        assert!(iter.next().is_none());

        prograns.get_mut(0).unwrap().priority = 1;
        let mut prograns = time_order_by_priority(prograns);

        let mut iter = prograns.iter();
        let item = iter.next().unwrap();

        assert_eq!(item, &Obj {
            start: "12:00:00".parse().unwrap(),
            end: "13:00:00".parse().unwrap(),
            description: "A".to_string(),
            priority: 1,
        });
        let item = iter.next().unwrap();
        assert_eq!(item, &Obj {
            start: "13:00:00".parse().unwrap(),
            end: "14:00:00".parse().unwrap(),
            description: "B".to_string(),
            priority: 0,
        });
        assert!(iter.next().is_none());
        
        prograns.get_mut(1).unwrap().priority = 2;
        let prograns = time_order_by_priority(prograns);

        let mut iter = prograns.iter();
        let item = iter.next().unwrap();

        assert_eq!(item, &Obj {
            start: "12:00:00".parse().unwrap(),
            end: "13:00:00".parse().unwrap(),
            description: "A".to_string(),
            priority: 1,
        });
        let item = iter.next().unwrap();
        assert_eq!(item, &Obj {
            start: "13:00:00".parse().unwrap(),
            end: "14:00:00".parse().unwrap(),
            description: "B".to_string(),
            priority: 2,
        });
        assert!(iter.next().is_none());
        
    }

    /// This will test the case of 2 program start and stop exatly at the same time
    /// this will create 2 programs one with full length and other with 0 length
    /// the one with full length should have the higher priority
    #[test]
    fn test_start_and_finish_same_time() {
        
        let prograns = vec![
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "13:00:00".parse().unwrap(),
                description: "A".to_string(),
                priority: 1,
            },
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "13:00:00".parse().unwrap(),
                description: "B".to_string(),
                priority: 0,
            }
        ];

        let prograns = time_order_by_priority(prograns);
        let mut iter = prograns.iter();
        let item = iter.next().unwrap();

        assert_eq!(item, &Obj {
            start: "12:00:00".parse().unwrap(),
            end: "12:00:00".parse().unwrap(),
            description: "B".to_string(),
            priority: 0,
        });
        let item = iter.next().unwrap();
        assert_eq!(item, &Obj {
            start: "12:00:00".parse().unwrap(),
            end: "13:00:00".parse().unwrap(),
            description: "A".to_string(),
            priority: 1,
        });
        assert!(iter.next().is_none());


    }
    /// Example most complex with prograns start and finish on the middle of other
    /// this shloud make comteplate all code path
    /// 
    /// On this example the priority is given by the height
    /// 
    /// 
    ///                                   |------ D ------|
    ///                           |-------------- C --------------|
    ///                                           |-------------- B --------------|
    ///   |-- F --|       |---------------------- A ----------------------|               |-- E --|
    /// 11:00   11:30   12:00   12:30   13:00   13:30   14:00   14:30   15:00   15:30   16:00   16:30
    /// 
    /// The expected result is:
    /// 
    ///   |-- F --|       |-- A --|-- C --|------ D ------|-- C --|------ B ------|       |-- E --|
    /// 11:00   11:30   12:00   12:30   13:00   13:30   14:00   14:30   15:00   15:30   16:00   16:30
    ///  
    #[test]
    fn test_complex_example()
    {
        
        let prograns = vec![
            Obj {
                start: "12:00:00".parse().unwrap(),
                end: "15:00:00".parse().unwrap(),
                description: "A".to_string(),
                priority: 1,
            },
            Obj {
                start: "13:30:00".parse().unwrap(),
                end: "15:30:00".parse().unwrap(),
                description: "B".to_string(),
                priority: 2,
            },
            Obj {
                start: "12:30:00".parse().unwrap(),
                end: "14:30:00".parse().unwrap(),
                description: "C".to_string(),
                priority: 3,
            },
            Obj {
                start: "13:00:00".parse().unwrap(),
                end: "14:00:00".parse().unwrap(),
                description: "D".to_string(),
                priority: 4,
            },
            Obj {
                start: "16:00:00".parse().unwrap(),
                end: "16:30:00".parse().unwrap(),
                description: "E".to_string(),
                priority: 1,
            },
            Obj {
                start: "11:00:00".parse().unwrap(),
                end: "11:30:00".parse().unwrap(),
                description: "F".to_string(),
                priority: 1,
            },
        ];
        
        let ordered = time_order_by_priority(prograns);

        assert_eq!(ordered.len(), 7);

        let mut ordered = ordered.iter();
        
        let item = ordered.next().unwrap();
        assert_eq!(item, &Obj {
            start: "11:00:00".parse().unwrap(),
            end: "11:30:00".parse().unwrap(),
            description: "F".to_string(),
            priority: 1,
        });

        let item = ordered.next().unwrap();
        assert_eq!(item, &Obj {
            start: "12:00:00".parse().unwrap(),
            end: "12:30:00".parse().unwrap(),
            description: "A".to_string(),
            priority: 1,
        });

        let item = ordered.next().unwrap();
        assert_eq!(item, &Obj {
            start: "12:30:00".parse().unwrap(),
            end: "13:00:00".parse().unwrap(),
            description: "C".to_string(),
            priority: 3,
        });

        let item = ordered.next().unwrap();
        assert_eq!(item, &Obj {
            start: "13:00:00".parse().unwrap(),
            end: "14:00:00".parse().unwrap(),
            description: "D".to_string(),
            priority: 4,
        });

        let item = ordered.next().unwrap();
        assert_eq!(item, &Obj {
            start: "14:00:00".parse().unwrap(),
            end: "14:30:00".parse().unwrap(),
            description: "C".to_string(),
            priority: 3,
        });

        let item = ordered.next().unwrap();
        assert_eq!(item, &Obj {
            start: "14:30:00".parse().unwrap(),
            end: "15:30:00".parse().unwrap(),
            description: "B".to_string(),
            priority: 2,
        });

        let item = ordered.next().unwrap();
        assert_eq!(item, &Obj {
            start: "16:00:00".parse().unwrap(),
            end: "16:30:00".parse().unwrap(),
            description: "E".to_string(),
            priority: 1,
        });

        assert_eq!(ordered.next(), None);
    }
}