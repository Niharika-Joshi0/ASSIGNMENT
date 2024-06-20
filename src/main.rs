use clap::{App, Arg};
use std::path::PathBuf;
use calamine::{Reader, Xlsx, open_workbook};
use calamine::Xls;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{self, BufRead, BufReader,Write};
mod process; 
use crate::structsfile::{EmployeeRecord,DeptRecord,SalRecord,LeaveRecord};
mod structsfile;
mod processout;



fn main() {
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

    let emp_data_file = PathBuf::from(matches.value_of("emp-data-file-path").unwrap());
    let dept_data_file = PathBuf::from(matches.value_of("dept-data-file-path").unwrap());
    let salary_data_file = PathBuf::from(matches.value_of("salary-data-file-path").unwrap());
    let leave_data_file = PathBuf::from(matches.value_of("leave-data-file-path").unwrap());
    let output_file = PathBuf::from(matches.value_of("output-file-path").unwrap());

    let empdata=process::read_employee_data(&emp_data_file);
    let depdata=process::read_department_data(&dept_data_file);
    let saldata=process::read_salary_data(&salary_data_file);
    let leavedata=process::read_leave_data(&leave_data_file);
    let r=processout::generate_output(&empdata,&depdata,&saldata,&leavedata,&output_file);
}

