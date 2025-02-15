// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this


// Use an enum to represent all types of employees
enum Employees{
    Maintenance,
    Marketing,
    Managers,
    LineSupervisors,
    Cooks,
    AssemblyTech
}

// * Use a struct to store the employee type and whether they are
//   still employed -> status -> Active / Terminated


enum Status {
    Active,
    Terminated
}
struct EmplyeeStatus {
    //
    emp:Employees,
    status:Status
}


// * Use a function that returns a Result to determine if the employee
//   may enter the building
// (ES)-> Result<T,E> // T? E? 
//-> 
/*
    Maintenance, =>
    Marketing,=> 
    Managers,=>
    _=>    Err()
           
*/

fn enter(es:EmplyeeStatus) -> Result<(),String> {
     

    match es.status {
        Status::Terminated => return Err("Status is terminated".to_owned()),
        _=> ()
    }


    match es.emp{
        Employees::Maintenance => Ok(()),
        Employees::Managers => Ok(()),
        Employees::Marketing => Ok(()),
        _=> Err("Acees denied".to_string())
    }


}

// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this


// fn  ? 
fn print(es:EmplyeeStatus) -> Result<(),String> {

    let res = enter(es)?;

    println!("access is ok");

    Ok(())

}


fn main() {

    let es = EmplyeeStatus{
           emp:Employees::Managers,
           status:Status::Active
    };

    let res = print(es);


    
}
