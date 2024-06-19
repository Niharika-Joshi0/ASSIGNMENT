use clap::{App, Arg};
use std::path::PathBuf;
use calamine::{Reader, Xlsx, open_workbook};
use calamine::Xls;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{self, BufRead, BufReader,Write};
use chrono::prelude::*; 


#[derive(Debug)]
struct EmployeeRecord {
    employee_id: String,
    emp_name: String,
    department_id: String,
    mobile_no: String,
    email: String,
}

fn main() {
    // Define the CLI application with clap
    let matches = App::new("Data Processing Tool")
                    .version("1.0")
                    .author("Your Name")
                    .about("An example CLI tool for processing employee data")
                    .arg(Arg::with_name("emp-data-file-path")
                    .short('e')
                        .long("emp-data-file-path")
                        .value_name("FILE")
                        .help("Sets the path to employee data file")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("dept-data-file-path")
                        .short('d')
                        .long("dept-data-file-path")
                        .value_name("FILE")
                        .help("Sets the path to department data file")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("salary-data-file-path")
                        .short('s')
                        .long("salary-data-file-path")
                        .value_name("FILE")
                        .help("Sets the path to salary data file")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("leave-data-file-path")
                        .short('l')
                        .long("leave-data-file-path")
                        .value_name("FILE")
                        .help("Sets the path to leave data file")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("output-file-path")
                        .short('o')
                        .long("output-file-path")
                        .value_name("FILE")
                        .help("Sets the path to output file")
                        .takes_value(true)
                        .required(true))
                    .get_matches();

    // Extract input file paths
    let emp_data_file = PathBuf::from(matches.value_of("emp-data-file-path").unwrap());
    let dept_data_file = PathBuf::from(matches.value_of("dept-data-file-path").unwrap());
    let salary_data_file = PathBuf::from(matches.value_of("salary-data-file-path").unwrap());
    let leave_data_file = PathBuf::from(matches.value_of("leave-data-file-path").unwrap());
    let output_file = PathBuf::from(matches.value_of("output-file-path").unwrap());

    let empdata=read_employee_data(&emp_data_file);
    let depdata=read_department_data(&dept_data_file);
    let saldata=read_salary_data(&salary_data_file);
    let leavedata=read_leave_data(&leave_data_file);
    let r=generate_output(&empdata,&depdata,&saldata,&leavedata,&output_file);
}


fn read_employee_data(file_path: &PathBuf) ->Vec<EmployeeRecord>{
    let file = File::open(file_path).expect("Unable to open employee data file");
    let reader = BufReader::new(file);
    let mut employee_data = Vec::new();

    // Skip header line
    let mut lines = reader.lines();
    if let Some(Ok(header)) = lines.next() {
        // Parse each subsequent line into EmployeeRecord
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



#[derive(Debug)]
struct DeptRecord {
    department_id: String,
    department_title: String,
    department_strength: String, // Assuming department strength as String, adjust as per your data type
}

fn read_department_data(file_path: &PathBuf) -> Vec<DeptRecord> {
    // println!("Inside");

    // Open the XLS workbook
    let mut workbook: Xls<_> = match open_workbook(file_path) {
        Ok(wb) => wb,
        Err(e) => {
            panic!("Failed to open workbook: {}", e);
        }
    };

    let mut department_data = Vec::new();

    // Get the first worksheet
    if let Ok(sheet) = workbook.worksheet_range("Sheet1") {
        // Iterate over rows
        for row in sheet.rows() {
            // Assuming department data is in specific columns, adjust indices as per your XLS structure
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











#[derive(Debug)]
struct SalRecord {
    EmpId:String,
    SalaryId:String,
    SalaryDate:String,
    Salary:String,
    SalaryStatus:String,
}

fn read_salary_data(file_path: &PathBuf) -> Vec<SalRecord> {
    // println!("Inside");

    // Open the XLS workbook
    let mut workbook: Xlsx<_> = match open_workbook(file_path) {
        Ok(wb) => wb,
        Err(e) => {
            panic!("Failed to open workbook: {}", e);
        }
    };

    let mut salary_data = Vec::new();

    // Get the first worksheet
    if let Ok(sheet) = workbook.worksheet_range("Sheet1") {
        // Iterate over rows
        for row in sheet.rows() {
            // Assuming department data is in specific columns, adjust indices as per your XLS structure
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








#[derive(Debug)]
struct LeaveRecord {
    EmpId:String,
    LeaveId:String,
    LeaveForm:String,
    LeaveTo:String,
    LeaveType:String,
}

fn read_leave_data(file_path: &PathBuf) -> Vec<LeaveRecord> {
    // println!("Inside");

    // Open the XLS workbook
    let mut workbook: Xlsx<_> = match open_workbook(file_path) {
        Ok(wb) => wb,
        Err(e) => {
            panic!("Failed to open workbook: {}", e);
        }
    };

    let mut leave_data = Vec::new();

    // Get the first worksheet
    if let Ok(sheet) = workbook.worksheet_range("Sheet1") {
        // Iterate over rows
        for row in sheet.rows() {
            // Assuming department data is in specific columns, adjust indices as per your XLS structure
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











use chrono::{Utc, Datelike, NaiveDate};

fn generate_output(
    employees: &[EmployeeRecord],
    departments: &[DeptRecord],
    salaries:&[SalRecord],
    leaves:&[LeaveRecord],
    output_file: &PathBuf,
) -> Result<(), std::io::Error> {
    let mut output_file = File::create(output_file)?;

    // Write header to output file
    writeln!(
        output_file,
        "Emp ID~#~Emp Name~#~Dept Title~#~Phone Number~#~Email~#~Salary status~#~On leave"
    )?;
    let now: DateTime<Utc> = Utc::now();

    let current_month = Utc::today().month();
    let current_year = Utc::today().year();
    
    // Get the month abbreviation
    let month_abbr = now.format("%b").to_string();
    
    // Get the year
    let year = now.year();
    
    // Combine month abbreviation and year
    let month_year = format!("{} {}", month_abbr, year);

    // Iterate over employees and find corresponding department titles
    for emp in employees {

        // Find corresponding department record
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
                // Parse leave dates and check if they fall within the current month
                parse_date(&leave.LeaveForm).and_then(|from_date| {
                    parse_date(&leave.LeaveTo).map(|to_date| {
                        if from_date.year() == current_year && from_date.month() == current_month {
                            let days_in_month = days_in_month(current_month, current_year);
                            let start_day = from_date.day();
                            let end_day = to_date.day();

                            // Calculate number of days leave
                            let days = end_day - start_day + 1;

                            // Ensure days are within the current month's range
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

/// Helper function to get number of days in a given month and year.
fn days_in_month(month: u32, year: i32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 },
        _ => 0,
    }
}
