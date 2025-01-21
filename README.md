
# Time Priority Order (tpo)

This is a algoritmh that I have think for create a cronogram of action sorted by start and finish time avoiding time colisions.

## Example

This example is on "test_example.rs"

On this example the priority is given by the height

```
                                    |------ D ------|
                            |-------------- C --------------|
                                            |-------------- B --------------|
    |-- F --|       |---------------------- A ----------------------|               |-- E --|
  11:00   11:30   12:00   12:30   13:00   13:30   14:00   14:30   15:00   15:30   16:00   16:30

The expected result is:

    |-- F --|       |-- A --|-- C --|------ D ------|-- C --|------ B ------|       |-- E --|
  11:00   11:30   12:00   12:30   13:00   13:30   14:00   14:30   15:00   15:30   16:00   16:30
```

## How works

First any object has to define a start and finish mark for example Obj D above starts at 13:00 and 
finish at 14:00 this temporal marks is called TimedEvent

Then a vector of objects is transform in a vector of TimedEvent and them sorted, each TimedEvent still
has a reference for original object. This vector can be called of time_line.

A running_prograns list is created empty this list will conteins the running prograns order by priority

A temp object is create from the first TimedEvent on the time_line the start time is the time of 
the TimeEvent and the end time is not defined yet.

Them tem temp obj is add to running_prograns, the first TimeEvent always is a StartTimeEvent

*pseudo code*

Them a loop starts, each loop can be a StartTimeEvent or EndTimeEvent

    if is a Start:
        Put the loop item object on the running_prograns list order by his priority

        if the loop item has a object with more priority then the temp object :
            set the end time of temp object then put it on final result list
            and create a new temp object for the loop item

    if is a End:
        Remove the loop item object of the running_prograns list

        if the loop item has a object with more or equals priority then the temp object :
            set the end time of temp object then put it on final result list

            if the running_prograns list is not empty:
                temp object is the top of running_prograns
            
            if the running_prograns list is empty and time_line is not finish:
                temp object is the next item with it should be a StartTimeEvent
                and insert it on the running_prograns

**After this the final result list should conteins the final list**