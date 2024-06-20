use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{Write};
use chrono::prelude::*; 
use crate::structsfile::{EmployeeRecord,DeptRecord,SalRecord,LeaveRecord};
use chrono::{Utc, Datelike, NaiveDate};

pub fn generate_output(
    employees: &[EmployeeRecord],
    departments: &[DeptRecord],
    salaries:&[SalRecord],
    leaves:&[LeaveRecord],
    output_file: &PathBuf,
) -> Result<(), std::io::Error> {
    let mut output_file = File::create(output_file)?;

    writeln!(
        output_file,
        "Emp ID~#~Emp Name~#~Dept Title~#~Phone Number~#~Email~#~Salary status~#~On leave"
    )?;
    let now: DateTime<Utc> = Utc::now();

    let current_month = Utc::today().month();
    let current_year = Utc::today().year();
    
    let month_abbr = now.format("%b").to_string();
    
    let year = now.year();
    
    let month_year = format!("{} {}", month_abbr, year);

    for emp in employees {

        if let Some(dept) = departments.iter().find(|d| d.department_id == emp.department_id)   {
            let salary_status = match salaries.iter()
            .find(|sal| sal.EmpId == emp.employee_id && sal.SalaryDate == month_year) {
                Some(sal) => {
                    match sal.SalaryStatus.as_str(){
                        "Credited" => "Credited",
                        "Not Credited" => "Not Credited",
                        _ => "Not Credited",
                    }
                },
                None => "Not Credited",
            };

            let leave_days = leaves.iter()
            .filter_map(|leave| {
                parse_date(&leave.LeaveForm).and_then(|from_date| {
                    parse_date(&leave.LeaveTo).map(|to_date| {
                        if from_date.year() == current_year && from_date.month() == current_month {
                            let days_in_month = days_in_month(current_month, current_year);
                            let start_day = from_date.day();
                            let end_day = to_date.day();

                            let days = end_day - start_day + 1;

                            if days > days_in_month {
                                days_in_month
                            } else {
                                days
                            }
                        } else {
                            0
                        }
                    })
                })
            })
            .sum::<u32>();
        

            writeln!(
                output_file,
                "{}~#~{}~#~{}~#~{}~#~{}~#~{}#~{}",
                emp.employee_id, emp.emp_name, dept.department_title,emp.mobile_no,emp.email,salary_status,leave_days
            )?;
        } else {
            writeln!(
                output_file,
                "{}~#~{}~#~[Dept Title Not Found]{}~#~{}",
                emp.employee_id, emp.emp_name,emp.mobile_no,emp.email
            )?;
        }
    }

    println!("WRITTEN INTO OUTPUT.TXT");
    Ok(())

}
fn parse_date(date_str: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%d-%m-%Y").ok()
}

fn days_in_month(month: u32, year: i32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 },
        _ => 0,
    }
}
