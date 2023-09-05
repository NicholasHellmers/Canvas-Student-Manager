use std::fs;
use structopt::StructOpt;


// Check for time-staps in the entire directory "Crate filetime"
// Sleep loop

#[derive(StructOpt)]
struct Cli {
    semester: String,
    courses: Vec<String>,
}

fn test_api() -> String {
    
}


fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    
    println!("The semester is: {}", args.semester);

    println!("The classes being taken are:");

    fs::create_dir("../".to_owned() + &args.semester)?;

    for course in args.courses {
        println!("{}", course);

        let course_path =   "../".to_owned() + 
                            &args.semester + 
                            &"/".to_owned() + 
                            &course;

        let _lecture_notes_path = course_path.clone() + &"/Lecture Notes".to_owned();
        let _homeworks_path = course_path.clone() + &"/Homeworks".to_owned();
        let _projects_path = course_path.clone() + &"/Projects".to_owned();
        let _books_path = course_path.clone() + &"/Books".to_owned();
        let _readings_path = course_path.clone() + &"/Readings".to_owned();

        fs::create_dir(course_path)?;
        fs::create_dir(_lecture_notes_path)?;
        fs::create_dir(_homeworks_path)?;
        fs::create_dir(_projects_path)?;
        fs::create_dir(_books_path)?;
        fs::create_dir(_readings_path)?;
    }
    Ok(())
}
