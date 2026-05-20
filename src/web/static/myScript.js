
import init, { Simulator } from "./pkg/model.js"
import {compose_diagram} from "./compose_diagram_funcs.js"

var sim = create_sim();

update_tables();

//Want to jsonify the simulator memory as bytes, not quadwords
var memory_mode = 1;

/**
 * initilises web assembly and creates a simulator object
 * @returns a simulator object
 */
async function create_sim() {

    //https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html [last accessed 2025-04-09]
    //first have to wait for intilisation of the web assembly code
    await init();
    var sim = Simulator.new();
    return sim

}

/**
 * Calls the simulator load function  and updates visuals
 */
async function load_clicked() {
    sim.then(
        function(simu) {
            var prog = document.getElementById("entrybox").value;
            simu.load_program(prog);

            //Get the assembler error (which may be an empty string) and display it
            var assembler_error = JSON.parse(simu.jsonify(memory_mode))["asssmbler_error"];
            document.getElementById("error_section").innerText = assembler_error;

            //Update visuals after loaad
            update_tables();
        },
        function(error) {console.log("Failed to load, simulator not yet initilised, please wait and try again")}
    )
    
}

/**
 * Calls the simulator micro_step_linear function and updates visuals
 */
async function micro_step_clicked() {

    sim.then(
        function(simu) {
            simu.micro_step_linear();

            update_tables();
        },
        function(error) {console.log("Failed to load, simulator not yet initilised, please wait and try again")}
    )
  
}

/**
 * Calls the simulator step_linear function and updates visuals
 */
async function step_clicked() {

    sim.then(
        function(simu) {
            simu.step_linear();
            update_tables();
        },
        function(error) {console.log("Failed to load, simulator not yet initilised, please wait and try again")}
    )
  
}

/**
 * On selecting a sample proram, try to fetch it from the server and load it into the entry box
 */
function  loadFile(){
    var val = document.getElementById("sample_selection").value;
    if (val !== "sample programs"){
        var inp = "";
        if (val === "fibonacci"){
            inp = './sample_programs/fibonacci.txt'
        }
        if (val === "factorial"){
            inp = "./sample_programs/factorial.txt"
        }
        if (val === "power"){
            inp = "./sample_programs/power.txt"
        }
        fetch(inp)
        .then(response => response.text())
        .then(data => {
            document.getElementById("entrybox").value = data;
        })
        .catch(error => {
            console.log("Error loading file");
        })
    }
}

/**
 * Calls compose diagram to update the visuals, parsing in the json of the simulator
 */
async function update_tables(){
    sim.then(
        function(simu) {
            var jsonSim = JSON.parse(simu.jsonify(memory_mode));
            compose_diagram(jsonSim, '#main-area');
        },
        function(error) {console.log("Failed to step, simulator not yet initilised, please wait and try again")}
    )
}


export {micro_step_clicked, step_clicked, load_clicked, loadFile}
