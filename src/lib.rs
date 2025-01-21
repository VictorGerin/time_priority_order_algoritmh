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

                    //looks like sorted_list has a bug when all elements is removed
                    //For that a new list is created when the sorted list is empty
                    *sorted_list = SortedList::<RefObj<T>>::new();
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