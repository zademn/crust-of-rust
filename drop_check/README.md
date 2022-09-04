# Drop check

[nomicon](https://doc.rust-lang.org/nomicon/dropck.html)


**Drop order**
: Disregarding some more complex situations, variables are dropped in the reverse order of their definition, fields of structs and tuples in order of their definition. 

For a generic type to soundly implement drop, its generics arguments must *strictly outlive* it.


**Drop check assumption**
: When you implement the `Drop` trait for a type that is generic over `T` the compiler **will assume** that it accesses the `T`.

For example if `T = &mut U` without this assumption you might access `T` somewhere in the program and when it will be dropped. This means we are accessing (holding) 2 `&mut U` and this shouldn't happen. 


`#![feature(dropck_eyepatch)]`
: This lets us `unsafe impl <#[may_dangle] T` that gets rid of the drop check assumption. However since it's `unsafe` we (the developers) guarantee that `T` does not get accessed during the drop. 

Notice that we can still drop `T`, just not access it. 
This feature will remain unstable. 


However this may not be enough. We may want to drop the `T` and we want to protect against it. This is where `PhantomData` comes in. 

`PhantomData`
: A type that is generic over another type but doesn't hold it. 
: Ex: `PhatomData<i32>` is 0-sized and doesn't hold anything but the compiler thinks that it holds something.

**Making a type covariant with a twist**  
A way to make a type covariant is to use `PhantomData<fn() -> T>`. However, this opens up a possible bug: Now the type cannot pretend it holds a `T` (because we don't have `PhantomData<T>`)and will not drop check the `T`. Yet, this may have a use: When we want covariant types and we don't want to drop check the `T`. 
Examples
- `trait Deserializer<T>` trait
- `struct EmptyIterator<T>`