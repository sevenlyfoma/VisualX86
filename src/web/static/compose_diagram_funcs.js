import {draw_table_general, draw_table_general_with_headings, draw_fu, draw_info_box, draw_transmission_line} from "./draw_diagram_funcs.js"

var unit_size = 22;
var mem_unit_size = 25;
var even_row_colour = "hsl(193, 53%, 64%)";
var odd_row_colour = "hsl(194, 13%, 62%)";
var heading_colour = "rgb(0,0,255)";
var heading_text_colour = "orange";
var text_colour = "black";

var memory_origin = [48*unit_size, 3*unit_size];
var prefetch_origin = [20*unit_size,3*unit_size];
var iq_origin = [1*unit_size,3*unit_size];
var ub_origin = [1*unit_size,16*unit_size];
var alu_origin = [1*unit_size,30*unit_size];
var mu_origin = [25*unit_size,30*unit_size];
var ju_origin = [49*unit_size,30*unit_size];
var log_reg_origin = [20*unit_size,8*unit_size];
var rip_origin = [32*unit_size, 23*unit_size];
var rflags_origin = [36*unit_size, 23*unit_size];

var infobox_origin = [33*unit_size, 8*unit_size];
var infobox_text = "Load Program To Begin";

var mem_pre_start = [memory_origin[0]-1*mem_unit_size, memory_origin[1]-0*mem_unit_size];
var mem_pre_end = [prefetch_origin[0]+16*mem_unit_size, prefetch_origin[1]-1*mem_unit_size];
var mem_pre_col = "black";

var pre_iq_start = [prefetch_origin[0]-1*mem_unit_size, prefetch_origin[1]-0*mem_unit_size+3];
var pre_iq_end = [iq_origin[0]+10*unit_size, iq_origin[1]-1*unit_size];
var pre_iq_col = "black";

var iq_ub_start = [iq_origin[0]+9*unit_size, iq_origin[1]+10*unit_size];
var iq_ub_end = [ub_origin[0]+10*unit_size, ub_origin[1]-1*unit_size];
var iq_ub_col = "black";

var main_line_start = [1*unit_size,27*unit_size];
var main_line_end = [64*unit_size,28*unit_size];
var main_line_col = "black";

var ub_main_start = [ub_origin[0]+9*unit_size, ub_origin[1]+10*unit_size];
var ub_main_end = [ub_origin[0]+10*unit_size, main_line_start[1]];
var ub_main_col = "black";

var reg_main_start = [log_reg_origin[0], log_reg_origin[1]+16*unit_size];
var reg_main_end = [log_reg_origin[0]+1*unit_size, main_line_start[1]];
var reg_main_col = "black";

var main_alu_start = [alu_origin[0], main_line_start[1]+1*unit_size];
var main_alu_end = [alu_origin[0]+1*unit_size, alu_origin[1]-1*unit_size];
var main_alu_col = "black";

var alu_main_one_start = [alu_origin[0]+16*unit_size, alu_origin[1]+6*unit_size];
var alu_main_one_end = [alu_origin[0]+18*unit_size, alu_origin[1]+7*unit_size];
var alu_main_one_col = "black";

var alu_main_two_start = [alu_origin[0]+18*unit_size, main_line_start[1]+1*unit_size];
var alu_main_two_end = [alu_origin[0]+19*unit_size, alu_origin[1]+7*unit_size];
var alu_main_two_col = "black";

var main_mu_start = [mu_origin[0], main_line_start[1]+1*unit_size];
var main_mu_end = [mu_origin[0]+1*unit_size, mu_origin[1]-1*unit_size];
var main_mu_col = "black";

var mu_main_one_start = [mu_origin[0]+16*unit_size, mu_origin[1]+6*unit_size];
var mu_main_one_end = [mu_origin[0]+18*unit_size, mu_origin[1]+7*unit_size];
var mu_main_one_col = "black";

var mu_main_two_start = [mu_origin[0]+18*unit_size, main_line_start[1]+1*unit_size];
var mu_main_two_end = [mu_origin[0]+19*unit_size, mu_origin[1]+7*unit_size];
var mu_main_two_col = "black";

var main_ju_start = [ju_origin[0], main_line_start[1]+1*unit_size];
var main_ju_end = [ju_origin[0]+1*unit_size, ju_origin[1]-1*unit_size];
var main_ju_col = "black";

var ju_main_one_start = [ju_origin[0]+12*unit_size, ju_origin[1]+6*unit_size];
var ju_main_one_end = [ju_origin[0]+14*unit_size, ju_origin[1]+7*unit_size];
var ju_main_one_col = "black";

var ju_main_two_start = [ju_origin[0]+14*unit_size, main_line_start[1]+1*unit_size];
var ju_main_two_end = [ju_origin[0]+15*unit_size, ju_origin[1]+7*unit_size];
var ju_main_two_col = "black";

var mem_main_start = [memory_origin[0], memory_origin[1]+16*mem_unit_size];
var mem_main_end = [memory_origin[0]+1*unit_size, main_line_start[1]];
var mem_main_col = "black";


var highlight_colour = "hsl(307, 73.60%, 71.80%)";

var iq_highlights = [];
var mem_highlights = [];
var pre_highlights = [];
var ub_highlights = [];
var alu_input_highlights = [];
var alu_output_highlights = [];
var mu_input_highlights = [];
var mu_output_highlights = [];
var ju_input_highlights = [];
var ju_output_highlights = [];
var log_reg_highlights = [];
var mic_reg_highlights = [];
var rip_highlights = [];
var rflags_highlights = [];

/**
 * Creates the table for main memory
 * @param {Object} jsonSimulator the json object containing the simulator information
 * @param {String} container the name of the svg item id
 */
function create_main_memory_table(jsonSimulator, container){

    var main_data = jsonSimulator["main_memory"]["contents"];

    var column_headings = ["0x","00","01","02","03","04","05","06","07","08","09","0A","0B","0C","0D","0E","0F",]
    var row_headings = ["00","01","02","03","04","05","06","07","08","09","0A","0B","0C","0D","0E","0F",]

    var table_contents_square_width = 16;
    var table_contents_square_height = 16;

    draw_table_general_with_headings(
      main_data, container, "main_memory_table", memory_origin[0], memory_origin[1],
      mem_unit_size, mem_unit_size, table_contents_square_width, table_contents_square_height,
      odd_row_colour, even_row_colour, text_colour, column_headings, row_headings,
      heading_colour, heading_text_colour, "Memory", [1,1], mem_highlights, highlight_colour,
    );    
}
      
/**
 * Creates the table for the prefetch buffer
 * @param {Object} jsonSimulator the json object containing the simulator information
 * @param {String} container the name of the svg item id
 */
function create_prefetch_table(jsonSimulator, container){
    var main_data = jsonSimulator["prefetch_buffer"]["contents"]

    var column_headings = ["0x","00","01","02","03","04","05","06","07","08","09","0A","0B","0C","0D","0E","0F",]
    var row_headings = ["00","01",]

    var table_contents_square_width = 16;
    var table_contents_square_height = 2;

    draw_table_general_with_headings(
      main_data, container, "prefetch_buffer_table", prefetch_origin[0], prefetch_origin[1],
      mem_unit_size, mem_unit_size, table_contents_square_width, table_contents_square_height,
      odd_row_colour, even_row_colour, text_colour, column_headings, row_headings, heading_colour,
      heading_text_colour, "Prefetch Buffer", [1,1], pre_highlights, highlight_colour,
    );    
}

/**
 * Creates the table for the instruction queue
 * @param {Object} jsonSimulator the json object containing the simulator information
 * @param {String} container the name of the svg item id
 */
function create_instruction_queue_table(jsonSimulator, container){
    var main_data = jsonSimulator["instruction_queue"]["contents"]

    var column_headings = ["Predecoded Instructions",]
    var row_headings = []

    let table_origin = iq_origin;

    var square_pixel_width = 10*unit_size;
    var square_pixel_height = 1*unit_size;
    var table_contents_square_width = 1;
    var table_contents_square_height = 10;

    draw_table_general_with_headings(
      main_data, container, "instruction_queue_table", table_origin[0], table_origin[1],
      square_pixel_width, square_pixel_height, table_contents_square_width, table_contents_square_height,
      odd_row_colour, even_row_colour, text_colour, column_headings, row_headings, heading_colour,
      heading_text_colour, "Instruction Queue", [0,1], iq_highlights, highlight_colour,
    );    
}

/**
 * Creates the table for the UOP buffer
 * @param {Object} jsonSimulator the json object containing the simulator information
 * @param {String} container the name of the svg item id
 */
function create_uop_buffer_table(jsonSimulator, container){
  var main_data = jsonSimulator["uop_buffer"]["contents"]

  var column_headings = ["Microcode Instructions",]
  var row_headings = []

  let table_origin = ub_origin;

  var square_pixel_width = 10*unit_size;
  var square_pixel_height = 1*unit_size;
  var table_contents_square_width = 1;
  var table_contents_square_height = 10;


  draw_table_general_with_headings(
    main_data, container, "uop_buffer_table", table_origin[0], table_origin[1],
    square_pixel_width, square_pixel_height, table_contents_square_width, table_contents_square_height,
    odd_row_colour, even_row_colour, text_colour, column_headings, row_headings,
    heading_colour, heading_text_colour,"UOP Buffer", [0,1], ub_highlights, highlight_colour,
  );    
}

/**
 * Creates the tables for the register file
 * Requires 4 draw_table calls because the draw_table_... function does not permit for headings to be different widths to table body
 * @param {Object} jsonSimulator the json object containing the simulator information
 * @param {String} container the name of the svg item id
 */
function create_register_file_table(jsonSimulator, container){
  var data_dict = jsonSimulator["register_file"]

  var keys = Object.keys(data_dict);
  var values = Object.values(data_dict);

  var logic_keys = keys.slice(0, 16);
  var micro_keys = keys.slice(16,32);
  var logic_vals = values.slice(0,16);
  var micro_vals = values.slice(16,32);

  let logic_table_origin = log_reg_origin;

  var square_pixel_width = 4*unit_size;
  var heading_width = 2*unit_size;
  var square_pixel_height = 1*unit_size;
  var table_contents_square_width = 1;
  var table_contents_square_height = 16;

  draw_table_general_with_headings(
    logic_keys, container, "logic_reg_keys", logic_table_origin[0], logic_table_origin[1],
    heading_width, square_pixel_height, table_contents_square_width, table_contents_square_height,
    heading_colour, heading_colour, heading_text_colour, [], [], heading_colour, heading_text_colour,
    "Register File",[0,0],
  ); 

  draw_table_general_with_headings(
    logic_vals, container, "logic_reg_vals", logic_table_origin[0] + heading_width, logic_table_origin[1],
    square_pixel_width, square_pixel_height, table_contents_square_width, table_contents_square_height,
    odd_row_colour, even_row_colour, text_colour, [], [], heading_colour, heading_text_colour,
    null, null, log_reg_highlights, highlight_colour,
  ); 

  draw_table_general_with_headings(
    micro_keys, container, "micro_reg_keys", logic_table_origin[0] + heading_width + square_pixel_width, logic_table_origin[1],
    heading_width, square_pixel_height, table_contents_square_width, table_contents_square_height, heading_colour,
    heading_colour, heading_text_colour, [], [], heading_colour, heading_text_colour,
  ); 

  draw_table_general_with_headings(
    micro_vals, container, "micro_reg_vals", logic_table_origin[0]  + 2*heading_width + square_pixel_width, logic_table_origin[1],
    square_pixel_width, square_pixel_height, table_contents_square_width, table_contents_square_height, odd_row_colour, even_row_colour,
    text_colour, [], [], heading_colour, heading_text_colour, null, null, mic_reg_highlights, highlight_colour,
  );  

}

/**
 * Creates the alu drawing, with draw_fu
 * @param {Object} jsonSimulator the json object containing the simulator information
 * @param {String} container the name of the svg item id
 */
function create_alu(jsonSimulator, container){
  var data_dict = jsonSimulator["alu"]

  var inputs = [data_dict["input1"], data_dict["input2"], data_dict["input_destination_register"], data_dict["operator"]];
  var input_headings = ["input 1", "input 2", "dest reg", "operator"];

  var outputs = [data_dict["output"], data_dict["output_register"], data_dict["output_optional"], data_dict["output_register_optional"]];
  var output_headings = ["result", "out reg", "result 2", "out reg 2"];

  var rflags_value = [data_dict["rflags"]];

  draw_fu(container, "alu", alu_origin[0], alu_origin[1], 16*unit_size, 8*unit_size, input_headings, inputs,
    output_headings, outputs, true, rflags_value, odd_row_colour, even_row_colour, text_colour, heading_colour,
    heading_text_colour, alu_input_highlights, alu_output_highlights, highlight_colour, "ALU", unit_size,
  );

}

/**
 * Creates the mu drawing, with draw_fu
 * @param {Object} jsonSimulator the json object containing the simulator information
 * @param {String} container the name of the svg item id
 */
function create_mu(jsonSimulator, container){
  var data_dict = jsonSimulator["mu"]

  var inputs = [data_dict["address_to_access"], data_dict["operator"], data_dict["store_input"], data_dict["load_input_register"]];
  var input_headings = ["address", "operator", "input", "dest reg"];

  var outputs = [data_dict["load_output"], data_dict["load_output_register"]];
  var output_headings = ["Loaded value", "Dest reg"];

  draw_fu(container, "mu", mu_origin[0], mu_origin[1], 16*unit_size, 8*unit_size, input_headings, inputs,
    output_headings, outputs, false, "", odd_row_colour, even_row_colour, text_colour, heading_colour,
    heading_text_colour, mu_input_highlights, mu_output_highlights, highlight_colour, "Memory Unit", unit_size,
  );
}

/**
 * Creates the ju drawing, with draw_fu
 * @param {Object} jsonSimulator the json object containing the simulator information
 * @param {String} container the name of the svg item id
 */
function create_ju(jsonSimulator, container){
  var data_dict = jsonSimulator["ju"]

  var inputs = [data_dict["operator"], data_dict["rip_copy"], data_dict["jump_amount"], ];
  var input_headings = ["operator", "rip copy", "jump val"];

  var outputs = [data_dict["jump_address"], jsonSimulator["jump_happened"]];
  var output_headings = ["Jump address", "Jump Happened"];

  var rflags_value = [data_dict["rflags_copy"]];

  draw_fu(container, "ju", ju_origin[0], ju_origin[1], 12*unit_size, 8*unit_size,
    input_headings, inputs, output_headings, outputs, true, rflags_value, odd_row_colour, 
    even_row_colour, text_colour, heading_colour, heading_text_colour, ju_input_highlights,
    ju_output_highlights, highlight_colour, "Jump Unit", unit_size
  );

}

/**
 * Creates two tables for rip and rflags
 * @param {Object} jsonSimulator the json object containing the simulator information
 * @param {String} container the name of the svg item id
 */
function create_rip_and_rflags_table(jsonSimulator, container){
    var rip = jsonSimulator["register_file"]["Rip"];
    var rflags = jsonSimulator["rflags"];

    var table_origin = rip_origin;

    var contents = [rip];
    var headers = ["rip"]

    var label = "rip"

    var square_pixel_width = 4*unit_size;
    var square_pixel_height = 1*unit_size;
    var table_contents_square_width = 1;
    var table_contents_square_height = 1;

    draw_table_general_with_headings(
        contents, container, label+"_table",table_origin[0], table_origin[1],
        square_pixel_width, square_pixel_height, table_contents_square_width, table_contents_square_height,
        odd_row_colour, even_row_colour, text_colour, headers, [], heading_colour, heading_text_colour,
        null, null, rip_highlights, highlight_colour,
    ); 

    // '','','','','O','','','','S','Z','','','','P','','C'
    var contents = [rflags];
    var headers = ["....O...SZ...P.C"]
    var label = "rflags"
    var square_pixel_width = 8*unit_size;
    table_origin = rflags_origin;

    draw_table_general_with_headings(
        contents, container, label+"_table", table_origin[0],table_origin[1],
        square_pixel_width, square_pixel_height, table_contents_square_width, table_contents_square_height,
        odd_row_colour, even_row_colour, text_colour, headers, [], heading_colour,
        heading_text_colour, "rflags", [0,1], rflags_highlights, highlight_colour,
    ); 
  
}

/**
 * Draws the info box
 * @param {String} container the name of the svg item id
 */
function create_info_box(container){
    draw_info_box(container, infobox_origin[0], infobox_origin[1], 12*unit_size, 6*unit_size, unit_size, infobox_text);
}

/**
 * Decides which lines are highlighted, which table cells are highlighted and what the info message is
 * @param {Object} jsonSimulator the json object containing the simulator information
 */
function decide_changing_data(jsonSimulator){
    var pipeline_stage = jsonSimulator["linear_pipeline_stage"];
    var prev_pipe_stage = jsonSimulator["previous_linear_pipeline_stage"];
    var last_loaded = jsonSimulator["last_functional_unit_loaded"];
    var last_executed = jsonSimulator["last_functional_unit_executed"];


    //Reset all highlights
    mem_highlights = [];
    iq_highlights = [];
    pre_highlights = [];
    ub_highlights = [];
    alu_input_highlights = [];
    alu_output_highlights = [];
    mu_input_highlights = [];
    mu_output_highlights = [];
    ju_input_highlights = [];
    ju_output_highlights = [];
    log_reg_highlights = [];
    mic_reg_highlights = [];
    rip_highlights = [];
    rflags_highlights = [];
    
    //Reset all line colours
    mem_pre_col = pre_iq_col = iq_ub_col = ub_main_col = main_line_col = main_alu_col = main_mu_col = main_ju_col = reg_main_col =
    mem_main_col = alu_main_one_col = alu_main_two_col = mu_main_one_col = mu_main_two_col = ju_main_one_col = ju_main_two_col = "black";

    var fus = ["ALU", "MU", "JU"];

    if (jsonSimulator["step_failed"] == true){
      infobox_text = "Processor could not continue to execute, the program has either ended or there was an error";
      return;
    }

    //If just loaded then there is no previous pipeline stage
    if (jsonSimulator["just_loaded"] == true){
        infobox_text = "Program Loaded";
        //Highlight all the loaded bytes
        mem_highlights = Array.from(Array(jsonSimulator["bytes_loaded"]).keys());
    }

    switch(prev_pipe_stage){
        case 0: {
            infobox_text = "Bytes loaded into prefetch buffer";
            mem_pre_col = "red";
            //Highlight whole prefetch buffer
            pre_highlights = Array.from(Array(32).keys());
        };
        break;
        case 1: {
            infobox_text = "Prefetch buffer predecoded";
            pre_iq_col = "red";
            //Highlight instruction queue new values, so from the head the the tail determined by size
            iq_highlights = Array.from(Array(jsonSimulator["instruction_queue"]["iq_size"]).keys().map(
                (x) => (x + jsonSimulator["instruction_queue"]["iq_head"]) % 10)
            );
        };
        break;
        case 2: {
            infobox_text = "Instruction converted to Micro Ops";
            iq_ub_col = "red";
            //Highlight uop buffer new values, so from the head the the tail determined by size
            ub_highlights = Array.from(Array(jsonSimulator["uop_buffer"]["ub_size"]).keys().map(
                (x) => (x + jsonSimulator["uop_buffer"]["ub_head"]) % 10)
            )
        };
        break;
        case 3: {
            infobox_text = fus[last_loaded] + " loaded";
            ub_main_col = main_line_col = reg_main_col = "red";
            if (last_loaded === 0){
                main_alu_col = "red";
                //Highlight all inputs of the fu
                alu_input_highlights = Array.from(Array(4).keys());
            }
            else if (last_loaded == 1){
                main_mu_col = "red";
                mu_input_highlights = Array.from(Array(4).keys());
            }
            else if (last_loaded == 2){
                main_ju_col = "red";
                ju_input_highlights = Array.from(Array(3).keys());
            }
        };
        break;
        case 4: {
            infobox_text = fus[last_executed] + " executed";
            if (last_executed == 0){
                //ALU
                alu_output_highlights = Array.from(Array(4).keys());
            }
            else if (last_executed == 1){
                //MU
                //Highlight lines going to memory
                mu_main_one_col = mu_main_two_col = main_line_col = mem_main_col = "red";
                //Highlight outputs
                mu_output_highlights = Array.from(Array(4).keys());
                //Highlight accessed memory
                var addr = jsonSimulator["mu"]["address_to_access"];
                mem_highlights = Array.from(Array(8).keys()).map((x) => x + parseInt(addr));
            }
            if (last_executed == 2){
                //JU
                ju_output_highlights = Array.from(Array(2).keys());
            }
        };
        break;
        case 5: {
            if (last_executed == 0){
                infobox_text = "Writeback from ALU";
                //Highlight writeback lines to registers
                main_line_col = alu_main_one_col = alu_main_two_col = reg_main_col = "red";

                //Get the registers that output to, highlight them in the register file
                var outreg = jsonSimulator["alu"]["output_register"];
                var outreg2 = jsonSimulator["alu"]["output_register_optional"];          
                var regfile = jsonSimulator["register_file"];
                var logvals = Object.keys(regfile).slice(0,16);
                var micvals = Object.keys(regfile).slice(16,32);
          
                if (outreg === "Rip") {
                  rip_highlights = [0];
                }
                else if (logvals.includes(outreg)){
                  log_reg_highlights.push(logvals.indexOf(outreg));
                }
                else if (micvals.includes(outreg)){
                  mic_reg_highlights.push(micvals.indexOf(outreg));
                }
          
                if (logvals.includes(outreg2)){
                  log_reg_highlights.push(logvals.indexOf(outreg2));
                }
                else if (micvals.includes(outreg2)){
                  mic_reg_highlights.push(micvals.indexOf(outreg2));
                } 
          
                rflags_highlights = [0];
                
            }
            else if (last_executed== 1){
                //Writeback message different if store, since no writeback occurs on a store
                if (jsonSimulator["mu"]["operator"] === "Load"){
                    infobox_text = "Writeback from MU After Load";
                    main_line_col = reg_main_col = mu_main_one_col =  mu_main_two_col = "red";
                }
                else {
                    infobox_text = "Writeback from MU Bubbled After Store";
                }
                //Get the registers that output to, highlight them in the register file
                var outreg = jsonSimulator["mu"]["load_output_register"];
                var regfile = jsonSimulator["register_file"];
                var logvals = Object.keys(regfile).slice(0,16);
                var micvals = Object.keys(regfile).slice(16,32);
          
                if (logvals.includes(outreg)){
                  log_reg_highlights.push(logvals.indexOf(outreg));
                }
                else if (micvals.includes(outreg)){
                  mic_reg_highlights.push(micvals.indexOf(outreg));
                } 
            }
            else if (last_executed == 2){
                if (pipeline_stage == 0) {
                    //This means jump happened
                    //So show everything being flushed
                    infobox_text = "Writeback from JU, Pipeline Flushed After Successful branch";
                    main_line_col = reg_main_col = ju_main_one_col = ju_main_two_col = "red";
                    pre_highlights = Array.from(Array(32).keys());
                    iq_highlights = Array.from(Array(10).keys());
                    ub_highlights = Array.from(Array(10).keys());
                    alu_input_highlights = Array.from(Array(4).keys());;
                    alu_output_highlights = Array.from(Array(4).keys());
                    mu_input_highlights = Array.from(Array(4).keys());
                    mu_output_highlights = Array.from(Array(4).keys());
                    ju_input_highlights = Array.from(Array(3).keys());
                    ju_output_highlights = Array.from(Array(2).keys());
                    rip_highlights = [0];
                }
                else {
                    infobox_text = "Writeback from JU Bubbled After Unsuccessful Branch";
                }
            
            }
        };
        break;
    }

}

/**
 * Calls all functions to create diagram
 * @param {Object} jsonSimulator the json object containing the simulator information
 * @param {String} container the name of the svg item id
 */
function compose_diagram(jsonSimulator, container){
  decide_changing_data(jsonSimulator);

  create_main_memory_table(jsonSimulator, container);
  create_prefetch_table(jsonSimulator, container);
  create_instruction_queue_table(jsonSimulator, container);
  create_uop_buffer_table(jsonSimulator, container);
  create_register_file_table(jsonSimulator, container);
  create_alu(jsonSimulator, container);
  create_mu(jsonSimulator, container);
  create_ju(jsonSimulator, container);
  create_rip_and_rflags_table(jsonSimulator, container);
  create_info_box(container);

  draw_transmission_line(container, "mem_pre_line", mem_pre_start, mem_pre_end, mem_pre_col);
  draw_transmission_line(container, "pre_iq_line", pre_iq_start, pre_iq_end, pre_iq_col);
  draw_transmission_line(container, "iq_ub_line", iq_ub_start,  iq_ub_end, iq_ub_col);
  draw_transmission_line(container, "main_line", main_line_start,  main_line_end, main_line_col);
  draw_transmission_line(container, "ub_main_line", ub_main_start, ub_main_end, ub_main_col);
  draw_transmission_line(container, "reg_main_line", reg_main_start, reg_main_end, reg_main_col);
  draw_transmission_line(container, "main_alu_line", main_alu_start, main_alu_end, main_alu_col);
  draw_transmission_line(container, "alu_main_one_line", alu_main_one_start, alu_main_one_end, alu_main_one_col);
  draw_transmission_line(container, "alu_main_two_line", alu_main_two_start, alu_main_two_end, alu_main_two_col);
  draw_transmission_line(container, "main_mu_line", main_mu_start, main_mu_end, main_mu_col);
  draw_transmission_line(container, "mu_main_one_line", mu_main_one_start, mu_main_one_end, mu_main_one_col);
  draw_transmission_line(container, "mu_main_two_line", mu_main_two_start, mu_main_two_end, mu_main_two_col);
  draw_transmission_line(container, "main_ju_line", main_ju_start, main_ju_end, main_ju_col);
  draw_transmission_line(container, "ju_main_one_line", ju_main_one_start, ju_main_one_end, ju_main_one_col);
  draw_transmission_line(container, "ju_main_two_line", ju_main_two_start, ju_main_two_end, ju_main_two_col);
  draw_transmission_line(container, "memory_main_line", mem_main_start, mem_main_end, mem_main_col);
}

export {compose_diagram}