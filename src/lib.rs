use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc, vec};

use sortedlist_rs::SortedList;

pub trait Timed<T>
where T: Ord + Copy
{
    fn get_start(&self) -> T;
    fn get_end(&self) -> T;
    fn set_start(&mut self, time: T);
    fn set_end(&mut self, time: T);
    fn get_priority(&self) -> u32;
}
struct ObjHolder<T> {
    obj: T,
    priority: usize,
}

/**
 * This is a reference to a user object that is wrapped in a RC & RefCell combination
 * this is to prevent unnecessary cloning of the object
 */
type RefObj<T> = Rc<RefCell<ObjHolder<T>>>;

enum TimedEvent<T, U> {
    Start{time: U,reference: RefObj<T>},
    End{time: U,reference: RefObj<T>},
}

fn recreate_priority_index<T, U>(mut vec: Vec::<T>) -> Vec<RefObj<T>>
where T: Timed<U> + Clone,
      U: Ord + Copy
{
    //This sort by priority and create a new priority index to garantee priority uniqueness 
    vec.sort_by(|a,b| a.get_priority().cmp(&b.get_priority()));

    //wrap the object in a RefObj<T> to prevent unnecessary cloning
    vec.into_iter().enumerate().map(|(index, x)| Rc::new(RefCell::new(ObjHolder {
        obj: x,
        priority: index
    })))
    .collect()
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

//get the last element on the list
fn get_top<T>(map: &HashMap::<usize, RefObj<T>>, keys: &SortedList::<usize>) -> Option<RefObj<T>> {
    match keys.last() {
        Some(key_index) => {
            Some(map.get(key_index).unwrap().clone())
        },
        None => None
    }
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
    sorted_list: &mut SortedList<usize>,
    map_data: &mut HashMap<usize, RefObj<T>>,
    inter: &mut vec::IntoIter<TimedEvent<T, U>>,
    reference: RefObj<T>,
    time: U) 
where T: Timed<U> + Clone,
      U: Ord + Copy
{
    if let Ok(index) = sorted_list.binary_search(&reference.borrow().priority) {
        sorted_list.remove(index);
        map_data.remove(&reference.borrow().priority);
    }

    if reference.borrow().priority >= temp_obj.borrow().priority {

        temp_obj.borrow_mut().obj.set_end(time);
        result.push(temp_obj.borrow().obj.clone());

        if let Some(last_item) = get_top(&*map_data, &*sorted_list) {
        
            *temp_obj = last_item.clone().into();
            temp_obj.borrow_mut().obj.set_start(time);
        
        } else if let Some(next) = inter.next() {
            //if finised the conflict and have more elements

            match next {
                TimedEvent::Start { reference , time } => {
                    *temp_obj = reference.clone();
                    temp_obj.borrow_mut().obj.set_start(time);
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
    sorted_list: &mut SortedList<usize>, 
    map_data: &mut HashMap<usize, RefObj<T>>, 
    reference: RefObj<T>, 
    time: U)
where T: Timed<U> + Clone + Debug,
        U: Ord + Copy
{
    map_data.insert(reference.borrow().priority, reference.clone());
    sorted_list.insert(reference.borrow().priority);

    if reference.borrow().priority >= temp_obj.borrow().priority {
        temp_obj.borrow_mut().obj.set_end(time);
        result.push(temp_obj.borrow().obj.clone());
    
        *temp_obj = reference.clone();
        temp_obj.borrow_mut().obj.set_start(time);
    }
}


pub fn time_order_by_priority<T, U>(vec: Vec::<T>) -> Vec::<T>
where T: Timed<U> + Clone + Debug,
U: Ord + Copy + Debug
{

    //trivial case not worth pass through the algorithm
    if vec.len() <= 1 {
        return vec;
    }
    
    let vec: Vec<RefObj<T>> = recreate_priority_index(vec);

    let vec: Vec<TimedEvent<T, U>> = create_time_order_events(vec);

    //This vector will store the final result
    //Here is only place where the object is cloned
    let mut result: Vec<T> = Vec::<T>::new();

    let mut temp_obj: RefObj<T>;

    //sorted list to keep track of the keys on ordey by priorities
    let mut sorted_list = SortedList::<usize>::new();
    let mut map_data = HashMap::<usize, RefObj<T>>::new();

    let mut inter = vec.into_iter();


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

    map_data.insert(temp_obj.borrow().priority, temp_obj.clone());
    sorted_list.insert(temp_obj.borrow().priority);
    

    while let Some(item) = inter.next() {
        match item {
            TimedEvent::Start { reference , time } => {
                process_start_case(&mut result, &mut temp_obj, &mut sorted_list, &mut map_data, reference, time); 
            },
            TimedEvent::End { reference , time } => {
                process_end_case(&mut result, &mut temp_obj, &mut sorted_list, &mut map_data, &mut inter, reference, time);
            },
        }
    }

    result
}
