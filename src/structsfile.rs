
    #[derive(Debug)]
pub struct EmployeeRecord {
    pub employee_id: String,
    pub emp_name: String,
    pub department_id: String,
    pub mobile_no: String,
    pub email: String,
}

#[derive(Debug)]
pub struct DeptRecord {
    pub department_id: String,
    pub department_title: String,
    pub department_strength: String, 
}

#[derive(Debug)]
pub struct SalRecord {
    pub EmpId:String,
    pub SalaryId:String,
    pub SalaryDate:String,
    pub Salary:String,
    pub SalaryStatus:String,
}
#[derive(Debug)]
pub struct LeaveRecord {
    pub EmpId:String,
    pub LeaveId:String,
    pub LeaveForm:String,
    pub LeaveTo:String,
    pub LeaveType:String,
}