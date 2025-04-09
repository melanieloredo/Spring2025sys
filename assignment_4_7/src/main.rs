fn main() {
    // In class Assignment
    
    // Create a struct Student (major)
    struct Student {
        major:String,
    }
    // Higher order functions update majors
    fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student,String)) -> Vec<Student> {
        for s in &mut collection {
            behavior(s,"Computer Science".to_string());
        }
        collection
    }
    // First Order functions, assign_major(student,major_declared)
    fn assign_major(s: &mut Student, major:String){
      s.major = major;
    }
    
    // create a vector of students1,2,3 and update all students major
      let student1 = Student { major: String::from("Psychology") };
      let student2 = Student { major: String::from("Biology") };
      let student3 = Student { major: String::from("Physics") };
      
      let students = vec![student1,student2,student3];
    
      //print original students structs
      println!("original majors:");
      for student in &students {
          println!("Student major: {}", student.major);
      }
      
      // update all students' majors to Computer Science
      let updated_students = update_majors(students, assign_major);
    
      //print updated students structs
      println!("\nupdated majors:");
      for student in &updated_students {
          println!("Student major: {}", student.major);
      }
    }
    
    
    //Take a screenshot of your program running and submit link to you github code
    