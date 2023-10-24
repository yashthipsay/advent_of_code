struct College{
    roll_no: u8,
    role: Role,
    cgpa: u8,

}

enum Role{
    Student,
    Faculty,
    Director,
}

fn main(){



}

fn student_struct(roll_no: u8, role: Role, cgpa: u8)->College{
    College {
        roll_no: roll_no,
        role: role,
        cgpa: cgpa
    }
}

fn arrays(){
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let bytes=[0; 5];
}