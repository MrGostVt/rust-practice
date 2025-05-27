fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    let mut final_grades = vec![];

    for &grade in grades {
        if grade < 38 {
            final_grades.push(grade);
            continue;
        }

        let mut nearest_number: f64 = (grade as f64 / 5 as f64).round() * 5.0;

        if(nearest_number as i32 > grade && grade - (nearest_number as i32) < 3 ) {
            final_grades.push(nearest_number as i32);
            continue;
        }
        final_grades.push(grade);
    }

    final_grades
}

#[test]
fn test(){
    let grades = gradingStudents(&[73,67,38,33]);

    grades.iter().for_each(|&gr| println!("{}", gr));
}