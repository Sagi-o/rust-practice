macro_rules! simple_macro {
    () => {
        println!("This is a basic macro");
    };
}

// Matches function pattern and executes it
macro_rules! x_and_y {
    (x => $e: expr) => (println!("X: {}", $e));
    (y => $e: expr) =>> (println!("Y: {}", $e));
}

// Takes a function name and builds the function
macro_rules! build_fn {
    ($func_name: ident) => {
        fn $func_name() {
            println!("You called {:?}()",
                     stringify!{$func_name});
        }
    };
}

// Takes an expression, and retures the strigified expression and it's result from calling it
macro_rules! print_ex {
    ($e: expr) => {
        println!("{:?} = {:?}", 
                 stringify!{$e},
                 $e);
    };
}

// Match left and right expressions with 'and'/'or' word and execute
macro_rules! exame {
    ($l: expr; and $r: expr) => {
        println!("{:?} and {:?} is {:?}", 
                 stringify!{$l},
                 stringify!{$r},
                 $l && $r); 
    };
    ($l: expr; or $r: expr) => {
        println!("{:?} or {:?} is {:?}", 
                 stringify!{$l},
                 stringify!{$r},
                 $l || $r); 
    };
}

// $id1 | where $id2 is between $start and $end, take condition to check on them
macro_rules! compr {
    ($id1: ident | $id2: ident <- [$start: expr; $end: expr], $condition: expr) => {
        let mut vec = Vec::new();
        for num in $start..$end + 1 {
            if $condition(num) {
                vec.push(num);
            }
        }
        vec
    };
}

// Take infinte number of lines like "key => value," and for each one insert to the map and return map
macro_rules! new_map {
    ($($key: expr => $value: expr),*) => { // change * to + to make 1 line at least required
        let mut map = HashMap::new(); // Happens ones
        $(
            map.insert($key, $value);
        )* // Happens for every line pattern match
        map
    };
}

// Evalute expression/s recursively with the 'eval' keywoard
macro_rules! calc {
    (eval $e: expr) => {{
        {
            let value: usize = $e;
            println!("{} = {}", stringify!{$e}, value);
        }
    }};
    (eval $e: expr, $(eval $es: expr),+) => {
        {
            calc! {eval $e} // called once for first calc
            calc! { $(eval $es),+} // $(...)+ calls for each pattern after the first
        }
    }
}

fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}

fn is_odd(x: i32) -> bool {
    return x % 2 != 0;
}

fn main() {
    simple_macro!();

    x_and_y!(x => 10);
    x_and_y!(y => 20 + 30);

    build_fn!(some_function);
    some_function();

    print_ex!({
        let y = 10;
        let z = 20;
        y + z + 100
    });

    exame!(1 == 1; and 2 == 1 + 1);
    exame!(true; or false);

    let evens = compr![x | x <- [1, 10], is_even];
    let odds = compr![y | y <- [1, 10], is_odd];

    let map = new_map!{
        "one" => 1,
        "two" => 2,
        "three" => 3,
    }

    calc! {
        eval 4 + 5,
        eval 4 + 10,
        eval (10 * 3) - 20
    }
}