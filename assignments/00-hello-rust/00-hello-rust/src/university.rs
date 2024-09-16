#[derive(PartialEq, Clone, Copy, Debug)]
enum ClassYear {
    Senior,
    Junior,
    Sophomore,
    FirstYear,
}

struct Student {
    name: &'static str,
    class_year: ClassYear,
    gpa: f32,
}

const OLIN_STUDENTS: [Student; 8] = [
    Student {
        name: "Alice",
        class_year: ClassYear::Senior,
        gpa: 3.9,
    },
    Student {
        name: "Foo",
        class_year: ClassYear::Sophomore,
        gpa: 2.3,
    },
    Student {
        name: "Bar",
        class_year: ClassYear::Junior,
        gpa: 3.9,
    },
    Student {
        name: "Ralph",
        class_year: ClassYear::Senior,
        gpa: 3.1,
    },
    Student {
        name: "Ayush",
        class_year: ClassYear::Senior,
        gpa: 0.0,
    },
    Student {
        name: "Anna",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Hanna",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Lorin",
        class_year: ClassYear::Junior,
        gpa: 3.6,
    },
];

fn get_average_gpa() -> f32 {
    let mut gpa_tot: f32 = 0.0;
    let mut stud_count = 0;
    for student_iter in OLIN_STUDENTS {
        if student_iter.class_year == ClassYear::FirstYear {
            continue;
        }
        stud_count += 1;
        gpa_tot += student_iter.gpa;
    }
    let float_div = stud_count as f32;
    gpa_tot / float_div
}

fn get_num_excel_students_for_class(class_year: ClassYear) -> u32 {
    let mut num_accel: u32 = 0;
    let avg_olin = get_average_gpa();

    for student_iter in OLIN_STUDENTS {
        if student_iter.class_year != class_year {
            continue;
        }
        if student_iter.gpa > avg_olin {
            num_accel += 1;
        }
    }
    num_accel
}

fn get_best_class() -> ClassYear {
    let mut best_class: ClassYear = ClassYear::Senior;
    let mut most_students: u32 = 0;

    let mut iter_students: u32;

    let years = [
        ClassYear::Senior,
        ClassYear::Junior,
        ClassYear::Sophomore,
        ClassYear::FirstYear,
    ];

    for class in years.iter() {
        iter_students = get_num_excel_students_for_class(*class);
        if most_students < iter_students {
            most_students = iter_students;
            best_class = *class;
        }
    }
    best_class
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::university::{
        get_average_gpa, get_best_class, get_num_excel_students_for_class, ClassYear,
    };

    #[test]
    fn test_get_average_gpa() {
        assert!(approx_eq!(f32, get_average_gpa(), 2.8))
    }

    #[test]
    fn test_get_num_excel_students_for_class() {
        assert_eq!(get_num_excel_students_for_class(ClassYear::Sophomore), 0);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Junior), 2);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Senior), 2);
    }

    #[test]
    fn test_get_best_class() {
        assert_eq!(get_best_class(), ClassYear::Senior);
    }
}
