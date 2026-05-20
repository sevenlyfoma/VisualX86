use crate::{simulator::{Simulator}};

use std::io::{stdin};

use std::fs;

use serde_json::json;
use serde_json::Value;

/// Environment to test simulator in the command line
pub fn main() {
    let mut simulator = Simulator::new();

    println!("Initial simulator state \n");
    let sim_json: Value;
    match serde_json::from_str(&simulator.jsonify(1)){
        Ok(x) => sim_json = x,
        Err(_) => sim_json = Value::Null,
    };
    println!("{}", serde_json::to_string_pretty(&sim_json).unwrap());

    //Basic User IO https://users.rust-lang.org/t/how-to-get-user-input/5176/3 [2024-10-23]
    let mut s = String::new();
    while s != "exit"{
        s = String::new();
        println!("Please enter either the file name containing the program you wish to load, or 'step'/'ustepl'/'u'/'stepl'/'sl' to advance state: ");
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        //Micro step
        if s == "ustepl" || s == "u"{
            
            let success = simulator.micro_step_linear();
            if success{
                println!("Linear Micro Step successful");
                println!("Simulator state:\n");
                let sim_json: Value;
                match serde_json::from_str(&simulator.jsonify(1)){
                    Ok(x) => sim_json = x,
                    Err(_) => sim_json = Value::Null,
                };
                println!("{}", serde_json::to_string_pretty(&sim_json).unwrap());
            }
            else{
                println!("Linear Micro Step unsucessful");
            }
            
        }

        //Full step
        else if s == "stepl" || s == "sl"{
            
            let success = simulator.step_linear();
            if success{
                println!("Linear Step successful");
                println!("Simulator state:\n");
                let sim_json: Value;
                match serde_json::from_str(&simulator.jsonify(1)){
                    Ok(x) => sim_json = x,
                    Err(_) => sim_json = Value::Null,
                };
                println!("{}", serde_json::to_string_pretty(&sim_json).unwrap());
            }
            else{
                println!("Linear Micro Step unsucessful");
            }
            
        }
        
        else {
            
            if let Ok(contents) = fs::read_to_string(&s){
                let success = simulator.load_program(contents);
                if success{
                    println!("Program '{}' loaded successfully!", s);
                    println!("Simulator state:\n");
                    let sim_json: Value;
                    match serde_json::from_str(&simulator.jsonify(1)){
                        Ok(x) => sim_json = x,
                        Err(_) => sim_json = Value::Null,
                    };
                    println!("{}", serde_json::to_string_pretty(&sim_json).unwrap());

                }
                else{
                    println!("Program '{}' not loaded successfully", s);
                }
            }
            else{
                if s != "exit"{
                    println!("Neither step nor valid file name was entered");
                }
               
            }
            
        }



    }
    
    
    
}