// https://theprimeagen.github.io/rust-for-typescript-devs/lessons/coding-rust/options

// // ts
// function functionThatTakesANumberMaybe(num: number | undefined)
//   {
//     if (num === undefined) return 0;
//     if (num !== undefined) return num;
//   }

fn returns_number(num: Option<usize>) -> usize {
    // match num {
    //     Some(n) => n,
    //     None => 0,
    // }

    // num.unwrap()

    // if let Some(x) = num {
    //     return x;
    // }
    // return 0;

    num.unwrap_or(0)
}

// fn returnsNumberNoMatterWhatX(num: impl Into<usize>) -> usize {
//     num.into()
//     // but doesnt handle negative number

//     // Bare literals like 5 and 0 will be inferred as
//     //  i64 when the function expects i64. This is
//     // the correct solution.
// }

fn returns_number_no_matter_what(num: i64) -> usize {
    num.unsigned_abs() as usize
}

// // ts
// function retutnsUndefines(num: number | undefined)
//   {
//     if (num === undefined) return undefined;
//     if (num !== undefined) return num;
//   }

fn returns_undefined(num: Option<usize>) -> Option<usize> {
    // return num.map(|x| x);

    // let num = num?;
    // return Some(num);

    return Some(num?);
}

// // ts
// function practice(list: number[], index: number) {
// 	index >= list.length ? index : list[index];
// }

// practice([1, 34, 3], 5);

fn number_or_index(list: Vec<usize>, index: usize) -> usize {
    // if list.len() >= index {
    //     return list[index];
    // } else {
    //     return index;
    // }

    // return match list.get(index) {
    //     Some(num) => *num,
    //     None => index,
    // };

    *list.get(index).unwrap_or(&index)
}

fn main() {
    println!("{}", returns_number(Some(5)));
    println!("{}", returns_number(None));
    println!("{}", returns_number_no_matter_what(5));

    println!("{}", returns_number_no_matter_what(0));
    println!("{}", returns_number_no_matter_what(-33));

    println!("{:?}", returns_undefined(None));
    println!("{:?}", returns_undefined(Some(56)));

    println!("{:?}", number_or_index(vec![1, 2, 3], 5));
    println!("{:?}", number_or_index(vec![4, 122, 23], 2));
}
