/* this faile is about the variable mutable 
* the name is a litle diferent
*/

const NUMBER_SPACES:i32 = 12;

fn main()
{
    // first case: using variable mutable
    let mut num = 10;
    println!("Number value is: {}", num);
    num =  2;
    println!("Number value is: {}", num);
    let _num = 123; // this prefix is used about annoucenments warning compilers

    //  second case: use let to change value
    // this concept is know like SHADOWING
    let x = 20;
    // put more here TODO
    let x =  x + 5;
    println!("value x is: {}", x);

    // third case: 
    let space = "   ";
    println!("Input is: {}", space);
    let space = space.len();
    println!("number of spaces is: {}", space);

    println!("default number of space: {}", NUMBER_SPACES);
}