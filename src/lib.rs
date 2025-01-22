
//! ```text
//!     # Time Priority Order (tpo)
//!     
//!     This is a algoritmh that I have think for create a cronogram of action sorted by start and finish time avoiding time colisions.
//!     
//!     ## Example
//!     
//!     This example is on "test_example.rs"
//!     
//!     On this example the priority is given by the height
//! 
//!                                         |------ D ------|
//!                                 |-------------- C --------------|
//!                                                 |-------------- B --------------|
//!         |-- F --|       |---------------------- A ----------------------|               |-- E --|
//!       11:00   11:30   12:00   12:30   13:00   13:30   14:00   14:30   15:00   15:30   16:00   16:30
//!     
//!     The expected result is:
//!     
//!         |-- F --|       |-- A --|-- C --|------ D ------|-- C --|------ B ------|       |-- E --|
//!       11:00   11:30   12:00   12:30   13:00   13:30   14:00   14:30   15:00   15:30   16:00   16:30
//!     
//!     
//!     ## Todo
//!     
//!     1. Create a Benchmark and caracterize memory and time
//!     
//!     ## How works
//!     
//!     First any object has to define a start and finish mark for example Obj D above starts at 13:00 and 
//!     finish at 14:00 this temporal marks is called TimedEvent
//!     
//!     Then a vector of objects is transform in a vector of TimedEvent and them sorted, each TimedEvent still
//!     has a reference for original object. This vector can be called of time_line.
//!     
//!     A running_prograns list is created empty this list will conteins the running prograns order by priority
//!     
//!     A temp object is create from the first TimedEvent on the time_line the start time is the time of 
//!     the TimeEvent and the end time is not defined yet.
//!     
//!     Them tem temp obj is add to running_prograns, the first TimeEvent always is a StartTimeEvent
//!     
//!     *pseudo code*
//!     
//!     Them a loop starts, each loop can be a StartTimeEvent or EndTimeEvent
//!     
//!         if is a Start:
//!             Put the loop item object on the running_prograns list order by his priority
//!     
//!             if the loop item has a object with more priority then the temp object :
//!                 set the end time of temp object then put it on final result list
//!                 and create a new temp object for the loop item
//!     
//!         if is a End:
//!             Remove the loop item object of the running_prograns list
//!     
//!             if the loop item has a object with more or equals priority then the temp object :
//!                 set the end time of temp object then put it on final result list
//!     
//!                 if the running_prograns list is not empty:
//!                     temp object is the top of running_prograns
//!                 
//!                 if the running_prograns list is empty and time_line is not finish:
//!                     temp object is the next item with it should be a StartTimeEvent
//!                     and insert it on the running_prograns
//!     
//!     **After this the final result list should conteins the final list**

mod test_example;

use std::{cell::RefCell, fmt::Debug, rc::Rc, vec};

use sortedlist_rs::SortedList;

pub trait Timed<U> : PartialOrd
where U: PartialOrd + Copy
{
    fn get_start(&self) -> U;
    fn get_end(&self) -> U;
    fn set_start(&mut self, time: U);
    fn set_end(&mut self, time: U);
}
#[derive(Debug)]
struct ObjHolder<T> {
    obj: T,
    priority: usize,
}

impl<T> ObjHolder<T> {
    fn new(data: (usize, T)) -> RefObj<T> {
        let (mut priority, obj) = data;
        priority += 1;
        Rc::new(RefCell::new(Self {
            obj,
            priority,
        }))
    }
}

impl<T> Eq for ObjHolder<T>
where T: PartialEq
{}

impl<T> Ord for ObjHolder<T>
where T: PartialOrd
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap_or(std::cmp::Ordering::Less)
    }
}

impl<T> PartialOrd for ObjHolder<T>
where T: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for ObjHolder<T>
where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

/**
 * This is a reference to a user object that is wrapped in a RC & RefCell combination
 * this is to prevent unnecessary cloning of the object
 */
type RefObj<T> = Rc<RefCell<ObjHolder<T>>>;

#[derive(Debug)]
enum TimedEvent<T, U> {
    Start{time: U,reference: RefObj<T>},
    End{time: U,reference: RefObj<T>},
}

/// recreate the priority index to garantee priority uniqueness
/// this is done by sorting the list by T::Ord and creating a new priority from the index result
fn recreate_priority_index<T, U>(mut vec: Vec::<T>) -> Vec<RefObj<T>>
where T: Timed<U> + Clone,
      U: Ord + Copy
{
    //This sort by priority and create a new priority index to garantee priority uniqueness 
    vec.sort_by(|a,b| a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Less));

    //wrap the object in a RefObj<T> to prevent unnecessary cloning
    vec.into_iter().enumerate().map(ObjHolder::new).collect()
}

fn create_time_order_events<T, U>(vec: Vec<RefObj<T>>) -> Vec<TimedEvent<T, U>>
where T: Clone + Timed<U>,
      U: Ord + Copy
{
    //Create a list of timed events ordered by time
    let mut vec: Vec<TimedEvent<T, U>> = vec.into_iter().map(|x| {
        let start = x.borrow().obj.get_start();
        let end = x.borrow().obj.get_end();
        vec![
            TimedEvent::Start {
                time: start,
                reference: x.clone(),
            },
            TimedEvent::End {
                time: end,
                reference: x,
            },
        ]
    }).flatten()
    .collect();
    vec.sort_by(|a, b| {
        {match a {
            TimedEvent::Start { time, reference: _ } => time,
            TimedEvent::End { time, reference: _ } => time,
        }}.cmp({match b {
            TimedEvent::Start { time, reference: _ } => time,
            TimedEvent::End { time, reference: _ } => time,
        }})
    });
    vec
}

/**
 * On case of End is very similar to Start
 * 
 * Remove the reference from the sorted list and from the map
 * 
 * and if the reference has a higher or equal priority than the current object
 * finish the current object and start a new one for the top of the priority list
 * 
 * if there is no more elements on the list, get the next element from the iterator
 * 
 */
fn process_end_case<T, U>(
    result: &mut Vec<T>,
    temp_obj: &mut RefObj<T>,
    sorted_list: &mut SortedList<RefObj<T>>,
    inter: &mut vec::IntoIter<TimedEvent<T, U>>,
    reference: RefObj<T>,
    time: U) 
where T: Timed<U> + Clone,
      U: Ord + Copy
{
    if let Ok(index) = sorted_list.binary_search(&reference) {
        sorted_list.remove(index);
    }

    if reference.borrow().priority >= temp_obj.borrow().priority {

        temp_obj.borrow_mut().obj.set_end(time);
        result.push(temp_obj.borrow().obj.clone());

        if let Some(last_item) = sorted_list.last() {
        
            *temp_obj = last_item.clone().into();
            temp_obj.borrow_mut().obj.set_start(time);
        
        } else if let Some(item) = inter.next() {
            //if finised the conflict and have more elements

            match item {
                TimedEvent::Start { reference , time } => {
                    *temp_obj = reference.clone();
                    temp_obj.borrow_mut().obj.set_start(time);

                    sorted_list.insert(temp_obj.clone());
                },
                _ => {
                    //This panic should never happen
                    //because to finish the conflict the last End had happened, and the only possible next is a Start
                    //outside the conflict
                    panic!("Error");
                }
            }
        }
    }
}


/**
 * On case of Start
 * 
 * Add the reference to the sorted list
 * 
 * and if the reference has a higher priority than the current object
 * finish the current object and start a new one for the reference
 */
fn process_start_case<T,U>(
    result: &mut Vec<T>, 
    temp_obj: &mut RefObj<T>, 
    sorted_list: &mut SortedList<RefObj<T>>, 
    reference: RefObj<T>, 
    time: U)
where T: Timed<U> + Clone,
        U: Ord + Copy
{
    sorted_list.insert(reference.clone());

    if reference.borrow().priority >= temp_obj.borrow().priority {
        temp_obj.borrow_mut().obj.set_end(time);
        result.push(temp_obj.borrow().obj.clone());
    
        *temp_obj = reference.clone();
        temp_obj.borrow_mut().obj.set_start(time);
    }
}


/// A function to order a list of Timed objects by priority
/// 
/// # Example
/// ### Example 1
/// ```text
/// Here the priority is represented by the height
///                                     |------ D ------|
///                             |-------------- C --------------|
///                                             |-------------- B --------------|
///     |-- F --|       |---------------------- A ----------------------|               |-- E --|
///   11:00   11:30   12:00   12:30   13:00   13:30   14:00   14:30   15:00   15:30   16:00   16:30
/// 
/// The expected result is:
/// 
///     |-- F --|       |-- A --|-- C --|------ D ------|-- C --|------ B ------|       |-- E --|
///   11:00   11:30   12:00   12:30   13:00   13:30   14:00   14:30   15:00   15:30   16:00   16:30
/// ```
/// ### Example 2
/// ```text
///     This test has 2 objects with a overlap of 1 hour (14:00 to 15:00)
///     
///                  | -------- B -------- |
///       | -------- A -------- |
///     
///     12:00      14:00      15:00      16:00
///     
///     The expected result is:
///     
///       | --- A --- | -------- B -------- |
///     12:00      14:00      15:00   16:00
///     
///     This because the object B has a higher priority than A
/// ```
/// ```rust
///     use chrono::NaiveTime;
///     use time_priority_order_algoritmh::{time_order_by_priority,Timed};
/// 
///     #[derive(Debug, Clone, PartialEq)]
///     struct Obj {
///         start: NaiveTime,
///         end: NaiveTime,
///         description: String,
///         priority: i32,
///     }
///     
///     impl Timed<NaiveTime> for Obj {
///         fn get_start(&self) -> NaiveTime {
///             self.start
///         }
///         fn get_end(&self) -> NaiveTime {
///             self.end
///         }
///         fn set_start(&mut self, time: NaiveTime) {
///             self.start = time;
///         }
///         fn set_end(&mut self, time: NaiveTime) {
///             self.end = time;
///         }
///     }
///     
///     impl PartialOrd for Obj {
///         fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
///             Some(self.priority.cmp(&other.priority))
///         }
///     }
/// 
///     let prograns = vec![
///         Obj {
///             start: "12:00:00".parse().unwrap(),
///             end: "15:00:00".parse().unwrap(),
///             description: "A".to_string(),
///             priority: 1,
///         },
///         Obj {
///             start: "14:00:00".parse().unwrap(),
///             end: "16:00:00".parse().unwrap(),
///             description: "B".to_string(),
///             priority: 2,
///         }
///     ];
///     
///     let ordered = time_order_by_priority(prograns);
///     assert_eq!(ordered.len(), 2);
///     let obj = ordered.get(0).unwrap();
///     assert!(obj.description == "A");
///     assert!(obj.start == "12:00:00".parse().unwrap());
///     assert!(obj.end == "14:00:00".parse().unwrap());
///     
///     let obj = ordered.get(1).unwrap();
///     assert!(obj.description == "B");
///     assert!(obj.start == "14:00:00".parse().unwrap());
///     assert!(obj.end == "16:00:00".parse().unwrap());
/// ```
pub fn time_order_by_priority<T, U>(vec: Vec::<T>) -> Vec::<T>
where T: Timed<U> + Clone,
U: Ord + Copy
{

    //trivial case not worth pass through the algorithm
    if vec.len() <= 1 {
        return vec;
    }
    
    let vec: Vec<RefObj<T>> = recreate_priority_index(vec);

    let time_line: Vec<TimedEvent<T, U>> = create_time_order_events(vec);

    //This vector will store the final result
    //Here is only place where the object is cloned
    let mut result: Vec<T> = Vec::<T>::new();

    let mut temp_obj: RefObj<T>;

    //sorted list to keep track of the keys on ordey by priorities
    let mut running_prograns: SortedList<RefObj<T>> = SortedList::new();

    let mut inter = time_line.into_iter();

    //unwrap is safe because we have at least 4 elements on the list
    //The algorithm only works with 2 or more Timed elements each create 2 points on the timeline
    let item = inter.next().unwrap();

    temp_obj = match &item {
        TimedEvent::Start { time: _, reference} => reference,
        TimedEvent::End { time: _, reference } => reference
    }.clone();

    temp_obj.borrow_mut().obj.set_start(match item {
        TimedEvent::Start { time, reference: _ } => time,
        TimedEvent::End { time, reference: _ } => time
    });

    running_prograns.insert(temp_obj.clone());
    

    while let Some(item) = inter.next() {

        match item {
            TimedEvent::Start { reference , time } => {
                process_start_case(&mut result, &mut temp_obj, &mut running_prograns, reference, time); 
            },
            TimedEvent::End { reference , time } => {
                process_end_case(&mut result, &mut temp_obj, &mut running_prograns, &mut inter, reference, time);
            },
        }
    }

    result
}


#[cfg(test)]
mod test {
    use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

    use crate::{recreate_priority_index, Timed};

    #[test]
    fn test_recreate_priority_index() {

        #[derive(Clone, Debug)]
        struct Obj {
            start: i32,
            end: i32,
            priority: i32,
            other: i32,
        }

        impl Timed<i32> for Obj {
            fn get_start(&self) -> i32 {
                self.start
            }
            fn get_end(&self) -> i32 {
                self.end
            }
            fn set_start(&mut self, time: i32) {
                self.start = time;
            }
            fn set_end(&mut self, time: i32) {
                self.end = time;
            }
        }

        impl PartialEq for Obj {
            fn eq(&self, other: &Self) -> bool {
                self.priority == other.priority
            }
        }

        impl PartialOrd for Obj {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                if self.priority == other.priority {
                    return Some(self.other.cmp(&other.other))
                }
                Some(self.priority.cmp(&other.priority))
            }
        }

        let mut vec = vec![
            Obj { start: 1, end: 2, priority: 20, other: 0 },
            Obj { start: 1, end: 2, priority: 40, other: 0 },
            Obj { start: 0, end: 2, priority: 10, other: 0 },
            Obj { start: 0, end: 2, priority: 10, other: 1 },
            Obj { start: 1, end: 2, priority: 30, other: 0 },
        ];
        let options = [
            std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal,
            std::cmp::Ordering::Greater,
        ];
        let mut rng = StdRng::seed_from_u64(42);
        vec.sort_by(|_,_| *options.choose(&mut rng).unwrap());

        let vec = recreate_priority_index(vec);
        assert_eq!(vec.len(), 5);

        let mut vec = vec.iter();
        
        let item = vec.next().unwrap();
        assert_eq!(item.borrow().obj.priority, 10);
        assert_eq!(item.borrow().priority, 1);
        assert_eq!(item.borrow().obj.other, 0);
        
        let item = vec.next().unwrap();
        assert_eq!(item.borrow().obj.priority, 10);
        assert_eq!(item.borrow().priority, 2);
        assert_eq!(item.borrow().obj.other, 1);

        let item = vec.next().unwrap();
        assert_eq!(item.borrow().obj.priority, 20);
        assert_eq!(item.borrow().priority, 3);

        let item = vec.next().unwrap();
        assert_eq!(item.borrow().obj.priority, 30);
        assert_eq!(item.borrow().priority, 4);

        let item = vec.next().unwrap();
        assert_eq!(item.borrow().obj.priority, 40);
        assert_eq!(item.borrow().priority, 5);

    }
}