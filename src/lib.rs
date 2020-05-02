//pub for macro
#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    // outer curly braces is for macro rules
    // inner curly braces is the block the macro expands to
    ($($element:expr),+ /* + means one or more exprs separated by a comma*/ $(,)? /* Allow for trailing commas*/) => {{
        let mut vs = Vec::new();
        // repeat this operation as many times
        // the same number of times as the pattern that had element in it
        $(vs.push($element);)*
        vs
    }};
    ($element:expr; $count:expr) => {{
        let count = $count;
        //make a vector with the capacity of count
        //to prevent allocation work everytime the vector
        //needs to be doubled in size
        let mut vs = Vec::with_capacity(count);
        //avoid allocation work
        vs.extend(std::iter::repeat($element).take(count));
        vs
    }};

}

#[test]
fn clone_2_nonliteral() {
    let mut y = Some(421);
    let x: Vec<u32> = avec![y.take().unwrap(); 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    //assert eq is a declarative macro and can use any delimiter
    assert_eq![x[0], 421];
    assert_eq![x[1], 421];
}

#[test]
fn clone_2() {
    let x: Vec<u32> = avec![421; 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    //assert eq is a declarative macro and can use any delimiter
    assert_eq![x[0], 421];
    assert_eq![x[1], 421];
}

#[test]
fn trailing_comma() {
    let x: Vec<u32> = avec![
        1, 3, 4, 5, 5, 6, 6, 5, 6, 7, 564, 5, 64, 4, 4, 4, 5, 3, 3, 3, 4, 4, 3, 3, 4, 4, 333, 2, 3,
        433, 4, 3, 3, 3343, 24, 3, 3, 343343, 3, 4, 3, 43, 3, 3, 3, 33, 3, 3, 3, 3, 4, 3, 3, 3, 3,
        3, 3, 4, 3, 3, 3, 4, 334, 3, 3, 3, 3, 3, 4, 4, 444343, 43, 3, 33, 3, 3, 34343, 4, 3, 4,
        43434, 34343, 4, 3, 43, 4, 34, 3, 3, 4, 34, 34, 4,
    ];
    assert!(!x.is_empty());
}

#[test]
fn double() {
    let x: Vec<u32> = avec![421, 43];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    //assert eq is a declarative macro and can use any delimiter
    assert_eq![x[0], 421];
    assert_eq![x[1], 43];
}

#[test]
fn single() {
    let x: Vec<u32> = avec![421];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    //assert eq is a declarative macro and can use any delimiter
    assert_eq![x[0], 421];
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty())
}
