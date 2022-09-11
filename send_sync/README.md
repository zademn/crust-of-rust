# Send

`Send` and `Sync` are used to describe thread safety at the type level. Usually langugages catch things that can go wrong at runtime by performing some checks. In rust these checks are baked into the type system. 

**Similarities**
- Both are marker traits (they have no methods). They simulate a type having a property but they don't provide any behaviour. 
- `auto trait` -- the compiler automatically implements this trait for you if all of the members impl that trait. Although not a rule, in general all auto traits are marker traits but not all marker traits are auto traits (because it's weird to have an auto trait with the same method for many nested types). 

**Send**  
Tells you that it's okay to pass this value to another thread. That thread can do whatever they want to that value. Most types are send. 

Example of types that are not `Send` (types that violate some internal invariant / guarantee):
- `Rc`
- Mutex guards (Some OS have this rule where if one thread takes a lock he must release it too). Not that these are not the mutex types, only the guards. 

**Sync**  
Types safe to share references between threads.  
A type `T` is sync if a reference `&T` to it is `Send`.  

**Remarks**
- A type `T` can be `!Send` but be sync. Ex: a mutex guard can be `Sync` because we can read the lock but not modify it.
- `Rc` is `!Sync`. This is because another thread may try to clone the `Rc` and we know cloning `Rc` is not threadsafe. 
- There are types `Send + !Sync`. Ex: The Cell types. `Cell` can't give out shared references so you have exlusive access. This means that you can safely mutate a `Cell` with a shared reference (the safety comes from the fact that `Cell` is `!Sync`). However, since `Cell` doesn't give shared references, only the current thread can own a `Cell` so it's safe to send it between threads. 


Implementing negative traits (such as `!Sync` and `!Send`) is an unstable feature (available on nightly). Implementing them in stable can be done with a `PhantomData<T>` where `T: !Sync + !Send`. 

# Some types

Raw pointers `*const T` and `*mut T` are `!Sync` to break the chain of auto trait implementation. This is a preventive measure to make the dev think carefully of threading when using these types.  

`Sender`
