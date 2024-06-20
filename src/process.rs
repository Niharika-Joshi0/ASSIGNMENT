use std::path::PathBuf;
use calamine::{Reader, Xlsx, open_workbook};
use calamine::Xls;
use std::fs::{File};
use std::io::{ BufRead, BufReader};

use crate::structsfile::{EmployeeRecord,DeptRecord,SalRecord,LeaveRecord};
mod structsfile;


pub fn read_employee_data(file_path: &PathBuf) ->Vec<EmployeeRecord>{
    let file = File::open(file_path).expect("Unable to open employee data file");
    let reader = BufReader::new(file);
    let mut employee_data = Vec::new();

    let mut lines = reader.lines();
    if let Some(Ok(header)) = lines.next() {
        for line in lines {
            if let Ok(line) = line {
                let fields: Vec<&str> = line.split('|').collect();
                if fields.len() >= 5 {
                    let record = EmployeeRecord {
                        employee_id: fields[0].to_string(),
                        emp_name: fields[1].to_string(),
                        department_id: fields[2].to_string(),
                        mobile_no: fields[3].to_string(),
                        email: fields[4].to_string(),
                    };
                    employee_data.push(record);
                }
            }
        }
    }

    // println!("EMPLOYEE DATA :{:?}",employee_data);
    employee_data
}


pub fn read_department_data(file_path: &PathBuf) -> Vec<DeptRecord> {
    // println!("Inside");

    let mut workbook: Xls<_> = match open_workbook(file_path) {
        Ok(wb) => wb,
        Err(e) => {
            panic!("Failed to open workbook: {}", e);
        }
    };

    let mut department_data = Vec::new();

    if let Ok(sheet) = workbook.worksheet_range("Sheet1") {
        for row in sheet.rows() {
            if let (Some(department_id), Some(department_title), Some(department_strength)) =
                (row.get(0), row.get(1), row.get(2))
            {
                let record = DeptRecord {
                    department_id:  department_id.to_string(),
                    department_title: department_title.to_string(),
                    department_strength:  department_strength.to_string(),
                };

                department_data.push(record);
            }
        }
    }

    // println!("DEPT DATA : {:?}", department_data);
    department_data
}

pub fn read_salary_data(file_path: &PathBuf) -> Vec<SalRecord> {
    // println!("Inside");

    let mut workbook: Xlsx<_> = match open_workbook(file_path) {
        Ok(wb) => wb,
        Err(e) => {
            panic!("Failed to open workbook: {}", e);
        }
    };

    let mut salary_data = Vec::new();

    if let Ok(sheet) = workbook.worksheet_range("Sheet1") {
        for row in sheet.rows() {
            if let (Some(EmpId), Some(SalaryId), Some(SalaryDate),Some(Salary),Some(SalaryStatus)) =
                (row.get(0), row.get(1),row.get(2),row.get(3),row.get(4))
            {
                let record = SalRecord {
                    EmpId:  EmpId.to_string(),
                    SalaryId: SalaryId.to_string(),
                    SalaryDate:  SalaryDate.to_string(),
                    Salary:  Salary.to_string(),
                    SalaryStatus:  SalaryStatus.to_string(),
                };

                salary_data.push(record);
            }
        }
    }

    // println!("SAlary DATA : {:?}", salary_data);
    salary_data
}

pub fn read_leave_data(file_path: &PathBuf) -> Vec<LeaveRecord> {
    // println!("Inside");

    let mut workbook: Xlsx<_> = match open_workbook(file_path) {
        Ok(wb) => wb,
        Err(e) => {
            panic!("Failed to open workbook: {}", e);
        }
    };

    let mut leave_data = Vec::new();

    if let Ok(sheet) = workbook.worksheet_range("Sheet1") {
        for row in sheet.rows() {
            if let (Some(EmpId), Some(LeaveId), Some(LeaveFrom),Some(LeaveTo),Some(LeaveType)) =
                (row.get(0), row.get(1),row.get(2),row.get(3),row.get(4))
            {
                let record = LeaveRecord {
                    EmpId:  EmpId.to_string(),
                    LeaveId: LeaveId.to_string(),
                    LeaveForm:  LeaveFrom.to_string(),
                    LeaveTo:  LeaveTo.to_string(),
                    LeaveType:  LeaveType.to_string(),
                };

                leave_data.push(record);
            }
        }
    }

    // println!("Leave DATA : {:?}", leave_data);
    leave_data
}


