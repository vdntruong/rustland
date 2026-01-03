use crate::vector;

pub fn test() {
    // Num Vec
    //

    let mut myvec: Vec<i32> = Vec::new();

    myvec.push(1);

    println!("Size of vec: {:?}", myvec.len());
    println!("Capacity of vec: {:?}", myvec.capacity());
    println!("{:#?}", myvec);

    // get return an option -> Some or None
    println!("First element of vec: {:?}", myvec.get(0));

    // String Vec
    //

    let mystrvec: Vec<&str> = vec!["one", "two", "three"];
    // clone -> create a new vec with the same content but different memory location
    for mystr in mystrvec.clone() {
        println!("{}", mystr);
    }
    println!("my string vec: {:?}", mystrvec);

    let mut dest: Vec<&str> = vec!["4", "5", "6"];

    // append -> move the content of mystrvec to dest (by move I mean that mystrvec will be empty after this), that's why we use clone
    (&mut dest).append(&mut mystrvec.clone());
    println!("dest: {:?}", dest);
    println!("mystrvec: {:?}", mystrvec);

    (&mut dest).insert(0, "4");
    println!("dest: {:?}", dest);

    (&mut dest).remove(1);
    println!("dest: {:?}", dest);

    // retain -> remove all element that not match the condition
    (&mut dest).retain(|&x| x != "4");
    println!("dest: {:?}", dest);

    // reserve -> reserve the capacity of the vec
    // if the capacity is already greater than the requested capacity, it will not change anything
    // it will double the capacity if the requested capacity is greater than the current capacity
    // at the example, we request 13, it > 12 (current cap), so it make it 24
    println!("Capacity of vec: {:?}", dest.capacity());
    (&mut dest).reserve(13);
    println!("Capacity of vec: {:?}", dest.capacity());
    println!("dest: {:?}", dest);

    // bonus, 'btop' tool
}
