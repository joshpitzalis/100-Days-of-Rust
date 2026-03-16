use std::env::args;
use std::fs;

// import ze file
//
// function (filePath){
//     const eachLine = fs.readFileSync(filePath, 'utf8').split('\n');
//     eachLine.map(line => {
//       console.log(line)
//     })
// }

fn main()
// -> Result<(), std::io::Error>
{
    let file = args().nth(1).unwrap_or("src/file.txt".to_string());
    // // let contents = fs::read_to_string("file.txt");
    // let contents = match fs::read_to_string(file) {
    //     Ok(contents) => {
    //         /* or use contents here */
    //         contents
    //     }
    //     Err(e) => {
    //         eprintln!(
    //             "Fallback file may have moved or been renamed - please provide a filepath - {}",
    //             e
    //         );
    //         return;
    //     }
    // };

    let contents = if let Ok(contents) = fs::read_to_string(file) {
        contents
    } else {
        eprintln!("Fallback file may have moved or been renamed - please provide a filepath");
        return;
    };

    contents
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .for_each(|line| println!("{line}"));

    // let each_line = contents.split('\n')
    // for line in each_line {
    //     // println!("{line}");

    //     // if let Ok(value) = line.parse::<usize>() {
    //     //     println!("{value}");
    //     // }

    //     // // rather than an error that needs to eb handled, it just ignores the other case where line cant be parsed to a number

    //     if let Ok(value) = line.parse::<usize>() {
    //         println!("{value}");
    //     } else {
    //         println!("Line not a number");
    //     }
    // }
    // // Ok(())

    // ###

    // let file_name = std:: env:: args).nth(1).expect ("the file name to be passed in");
    //
    // let file = std::fs:: read_to_string(file_name).expect"unable to read the file to string");
    //
    // file.lines(). for_each(|line| {
    // if let Ok(value) = line.parse::<usize>() {
    // println! ("{}", value);
    // } else {
    // println!("Line not a number")
    // }
}
