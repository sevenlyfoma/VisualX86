import * as d3 from "https://cdn.jsdelivr.net/npm/d3@7/+esm";

var font = "courier new"

function class_label(label){
  return "." + label;
}

/**
 * Creates a generalised table
 *
 * @param {Array<String>} contents - The data to be put in the table, 1 dimensional
 * @param {String} svg_container - The string id of the svg to be acted upon
 * @param {String} label - A string identifier that should be unique to identify items within the table
 * @param {int} xorigin - a number representing the x offset the table should be placed at wihtin the svg
 * @param {int} yorigin - a number representing the y offset the table should be placed at wihtin the svg
 * @param {int} square_pixel_width - the number of pixels a cell of the table should take in the x direction
 * @param {int} square_pixel_height - the number of pixels a cell of the table should take in the y direction
 * @param {int} table_contents_square_width - the number of cells in a row of the table, since data is 1d, this is used to find when table should wrap
 * @param {String} odd_row_colour - A string containing the colour odd rows in the table should be
 * @param {String} even_row_colour - A string containing the colour even rows in the table should be
 * @param {String} text_colour - A string containing the colour text in the table should be
 * @param {Array<int>} highlights - An array of ints that signify the position of elements in contents that should be highlighed
 * @param {String} highlight_colour - A string containing the colour highlighted cells in the table should be
 */
function draw_table_general(
  contents, svg_container, label, xorigin, yorigin, square_pixel_width, square_pixel_height,
  table_contents_square_width, odd_row_colour, even_row_colour, text_colour,
  highlights = [], highlight_color = "white",
){
  var svg = d3.select(svg_container);
  var area_label = label + "_area";

  //Create g tag in which for table to exist within
  svg.selectAll(class_label(area_label)).remove()
  var area = svg.selectAll(class_label(area_label))
    .data([""]).enter().append("g") //Empty data to create just 1 of them
    .attr("class", area_label)
    .attr("transform", "translate(" + xorigin + ", " + yorigin +")");

  //Counter variables for the contents of the table
  var column_count = -1;
  var row_count = -1; 
  var colour_count = -1; 
  var label_count = -1;

  //Create the rectangle for each piece of data in the container
  var data_square_label = label + "_data_square"
  var squares = area.selectAll(class_label(data_square_label))
    .data(contents) 
    .enter().append("rect")
    .attr("class",data_square_label)
    .attr("x", function(d) {column_count+=1; return ((column_count%(table_contents_square_width))*(square_pixel_width));})
    .attr("y", function(d) {row_count+=1; return Math.floor((row_count/(table_contents_square_width)))*(square_pixel_height);})
    .attr("width", square_pixel_width)
    .attr("height", square_pixel_height)
    .style("fill", function(d) { 
      //Fill colour determined by odd row or even row
      colour_count+=1; 
      //If highlighted just use that
      if (highlights.includes(colour_count)){
        return highlight_color
      }
      else if ((Math.floor(colour_count/(table_contents_square_width)) % 2) == 0) {
        return even_row_colour
      } else {
        return odd_row_colour
      } 
    })
    //Id dynamically determined
    .attr("id", function (d) { label_count+=1;return (data_square_label + "_" + label_count.toString()); })
    .attr('stroke', 'black')

  var column_count = -1;
  var row_count = table_contents_square_width - 1 //Start at the number of y squares - 1 since text is written bottom to top, so we want to start 1 row down
    
  //Write the data text within their rectangle
  var data_square_text_label = label + "_data_square_text"
  var text = area.selectAll(class_label(data_square_text_label))
    .data(contents)
    .enter()
    .append("text")
    .attr("class",data_square_text_label)
    .text(function(d) { return d; }) 
    .attr("x", function(d) { column_count+=1; return ((column_count%(table_contents_square_width))*(square_pixel_width))+4; })
    .attr("y", function(d) { row_count+=1; return Math.floor((row_count/(table_contents_square_width)))*(square_pixel_height)-7; })
    .style("font-size", 15)
    .style("font-family", font)
    .style("fill", text_colour)
    
}

/**
 * Creates a generalised table with headings
 *
 * @param {Array<String>} contents - The data to be put in the table, 1 dimensional
 * @param {String} svg_container - The string id of the svg to be acted upon
 * @param {String} label - A string identifier that should be unique to identify items within the table
 * @param {int} xorigin - a number representing the x offset the table should be placed at wihtin the svg
 * @param {int} yorigin - a number representing the y offset the table should be placed at wihtin the svg
 * @param {int} square_pixel_width - the number of pixels a cell of the table should take in the x direction
 * @param {int} square_pixel_height - the number of pixels a cell of the table should take in the y direction
 * @param {int} table_contents_square_width - the number of cells in a row of the table, since data is 1d, this is used to find when table should wrap
 * @param {int} table_contents_square_height - the number of cells in a column of the table
 * @param {String} odd_row_colour - A string containing the colour odd rows in the table should be
 * @param {String} even_row_colour - A string containing the colour even rows in the table should be
 * @param {String} text_colour - A string containing the colour text in the table should be
 * @param {Array<String>} column_headings - Array of headings for each column
 * @param {Array<String>} row_headings - Array of headings for each row
 * @param {String} heading_colour - Colour of heading cells
 * @param {String} heading_text_colour - Colour of heading text
 * @param {String} title - title string to label the table
 * @param {Array<int>} title_offset - 2 item array saying how many cells of displacement the title should have from the table
 * @param {Array<int>} highlights - An array of ints that signify the position of elements in contents that should be highlighed
 * @param {String} highlight_colour - A string containing the colour highlighted cells in the table should be
 */
function draw_table_general_with_headings(
  contents, svg_container, label, xorigin, yorigin, square_pixel_width, square_pixel_height,
  table_contents_square_width, table_contents_square_height, odd_row_colour, even_row_colour,
  text_colour, column_headings, row_headings, heading_colour, heading_text_colour,
  title, title_offset, highlights, highlight_color,
){
    //Draw main table
    draw_table_general(
        contents, svg_container, label, xorigin, yorigin, square_pixel_width, square_pixel_height,
        table_contents_square_width, odd_row_colour, even_row_colour, text_colour, highlights, highlight_color,
    );
    //Draw column headings as a table
    if (column_headings.length > 0){
        draw_table_general(
        column_headings, svg_container, label + "_column_headings",
        xorigin - ((column_headings.length - table_contents_square_width) * square_pixel_width),
        yorigin - (square_pixel_height), square_pixel_width, square_pixel_height,
        column_headings.length, heading_colour, heading_colour, heading_text_colour,
        );
    }
    //Draw row headings as a table
    if (row_headings.length > 0){
        draw_table_general(
        row_headings, svg_container, label + "_row_headings", xorigin - (square_pixel_width),
        yorigin -  ((row_headings.length - table_contents_square_height) * square_pixel_height),
        square_pixel_width, square_pixel_height, 1, heading_colour, heading_colour, heading_text_colour,
        );
    }

    //If title exists, then draw it a the offset
    if (!(!title || title.length === 0 )){
        var svg = d3.select(svg_container);
        var title_label = label + "_title_label";
        svg.append("text")
        .attr("class", title_label)
        .attr("x", xorigin + (-(square_pixel_width) * title_offset[0]))
        .attr("y", yorigin + ((-(square_pixel_height) * title_offset[1]) - 7))
        .text(title)
        .style("font-size", "30px")
        .style("font-weight", "bold")
        .style("font-family", font)
        .style("fill", "black")
    } 
}

/**
 * Creates a generalised functional unit
 * @param {String} container - The string id of the svg to be acted upon
 * @param {String} label - The label of the functional unit
 * @param {int} xorigin - a number representing the x offset the table should be placed at wihtin the svg
 * @param {int} yorigin - a number representing the y offset the table should be placed at wihtin the svg
 * @param {int} width - the number of pixels the functional unit is wide
 * @param {int} height - the number of pixels the fu is high
 * @param {Array<String>} input_headings - an array of headings for the input fields of the fu
 * @param {Array<String>} inputs - an array of values for the input fields of the fu
 * @param {Array<String>} output_headings - an array of headings for the output fields of the fu
 * @param {Array<String>} outputs - an array of values for the output fields of the fu
 * @param {boolean} rflags_present - boolean indicating whether the fu has an internal rflagss
 * @param {Array<String>} rflags_value - an array of 1 length containing value of internal rflags
 * @param {String} odd_row_colour - A string containing the colour odd rows in the table should be
 * @param {String} even_row_colour - A string containing the colour even rows in the table should be
 * @param {String} text_colour - A string containing the colour text in the table should be
 * @param {String} heading_colour - Colour of heading cells
 * @param {String} heading_text_colour - Colour of heading text
 * @param {Array<int>} input_highlights - an array of indexs on which the input fields should be highlighted
 * @param {Array<int>} output_highlights - an array of indexs on which the output fields should be highlighted
 * @param {String} highlight_colour - A string containing the colour highlighted cells in the table should be
 * @param {String} fu_text - A string representing the label that should annotate the fu
 * @param {int} unit_size - The standard unit size used for the tabel, determines offsets 
 */
function draw_fu(
  container, label, xorigin, yorigin, width, height, input_headings,inputs,
  output_headings, outputs, rflags_present, rflags_value, odd_row_colour, even_row_colour,
  text_colour, heading_colour, heading_text_colour, input_highlights,
  output_highlights, highlight_color, fu_text, unit_size,
){

  var inputs_widths = width / inputs.length;
  var square_pixel_height = unit_size;

  // Draw input fields table
  draw_table_general_with_headings(
    inputs, container, label+"_inputs_table", xorigin, yorigin, inputs_widths,
    square_pixel_height, inputs.length, 1, odd_row_colour, even_row_colour,
    text_colour, input_headings, [], heading_colour, heading_text_colour,
    null, null, input_highlights, highlight_color,
  ); 
  var output_widths = width / outputs.length;

  //Draw output fields table
  draw_table_general_with_headings(
    outputs, container, label+"_output_table", xorigin, yorigin+6*square_pixel_height,
    output_widths, square_pixel_height, outputs.length, 1, odd_row_colour, even_row_colour,
    text_colour, output_headings, [], heading_colour, heading_text_colour,
    null, null, output_highlights, highlight_color,
  ); 

  //Draw flags table
  if (rflags_present){
    var flags_headings = ["....O...SZ...P.C"];
    var square_pixel_width = 8*unit_size;

    draw_table_general_with_headings(
      rflags_value, container, label + "_flags_table", xorigin + (width - 8*unit_size), yorigin + 3*unit_size,
      square_pixel_width, square_pixel_height, 1, 1, odd_row_colour, even_row_colour, text_colour,
      flags_headings, [], heading_colour, heading_text_colour,
    ); 
  }


  //Select svg area
  var svg = d3.select(container);
  var area_label = label + "_area";
  //Create g tag in which for table to exist within
  svg.selectAll(class_label(area_label)).remove()
  var area = svg.selectAll(class_label(area_label))
    .data([""]).enter().append("g") //Empty data to create just 1 of them
    .attr("class", area_label)
    .attr("transform", "translate(" + xorigin + ", " + yorigin +")");

  //Draw bounding rectangle
  area.append("rect")
  .attr("class", label + "_body_class")
  .attr("x", 0)
  .attr("y", -unit_size)
  .attr("width", width)
  .attr("height", height)
  .attr("fill-opacity", "0%")
  .attr("id", label + "_body")
  .attr('stroke', 'black')

  //Draw wrapping text
  area.append("foreignObject")
  .attr("width", (width)-8*unit_size)
  .attr("height", 4*unit_size)
  .attr("x", 10)
  .attr("y", 40)
  .append("xhtml:div")
  .style("color", "#000")
  .style("text-align", "left")
  .style("width", "100%")
  .style("height", "100%")
  .style("padding", "5px")
  .style("font-size", `25px`)
  .style("font-family", font)
  .style("font-weight", "bold")
  .style("fill", "black")
  .style("overflow-y", "auto")
  .html(fu_text) 

}

/**
 * Creates the info box
 * @param {String} container - The string id of the svg to be acted upon
 * @param {int} xorigin - a number representing the x offset the box should be placed at wihtin the svg
 * @param {int} yorigin - a number representing the y offset the box should be placed at wihtin the svg
 * @param {int} width - the number of pixels the box is wide
 * @param {int} height - the number of pixels the box is high
 * @param {int} unit_size - The standard unit size used for the tabel, determines offsets 
 * @param {String} infobox_text - A string representing the contents of the info box
 */
function draw_info_box(
    container,
    xorigin,
    yorigin,
    width,
    height,
    unit_size,
    infobox_text,
){

    var svg = d3.select(container);

    var area_label = "info_area";

    //Create g tag in which for table to exist within
    svg.selectAll(class_label(area_label)).remove()
    var area = svg.selectAll(class_label(area_label))
        .data([""]).enter().append("g") //Empty data to create just 1 of them
        .attr("class", area_label)
        .attr("transform", "translate(" + xorigin + ", " + yorigin +")");

    //Draw outline rectangle
    area.append("rect")
    .attr("class","info_box_rect")
    .attr("x", 0)
    .attr("y", 0)
    .attr("width", width)
    .attr("height", height)
    .attr("fill-opacity", "100%")
    .attr("fill", "hsl(307, 44.30%, 84.50%)")
    .attr("id", "ju_body")

    //Draw title text
    area.append("text")
    .attr("class", "info_text_class")
    .attr("x", 10)
    .attr("y", 30)
    .text("Info")
    .style("font-size", "30px")
    .style("font-weight", "bold")
    .style("font-family", font)
    .style("fill", "black");

    //https://stackoverflow.com/a/70169316 Last accessed 2025-04-09
    //Draw body text, div allows for wrapping
    area.append("foreignObject")
    .attr("width", 11*unit_size)
    .attr("height", 5*unit_size)
    .attr("x", 10)
    .attr("y", 40)
    .append("xhtml:div")
    .style("color", "#000")
    .style("text-align", "left")
    .style("width", "100%")
    .style("height", "100%")
    .style("padding", "5px")
    .style("font-size", `15px`)
    .style("font-family", font)
    .style("overflow-y", "auto")
    .html(infobox_text)


}

/**
 * Creates a transmission line
 * @param {String} container - The string id of the svg to be acted upon
 * @param {String} label - the name of the transmission line
 * @param {Array<int>} start_origin - [x,y] values representing the start coordinate of the rectangle
 * @param {Array<int>} end_origin - [x,y] values representing the end coordinate of the rectangle
 * @param {String} color - A string representing the color the line should be
 */
function draw_transmission_line
(
  container,
  label,
  start_origin,
  end_origin,
  color,
){

  var svg = d3.select(container);
  var line_label = label + "_line";

  let org_width = Math.abs(start_origin[0] - end_origin[0]);
  let org_height = Math.abs(start_origin[1] - end_origin[1]);
  let width = org_width;
  let height = org_height;

  //Use minimum of 2 points so rectangle draws correctly
  let x = Math.min(start_origin[0], end_origin[0]);
  let y = Math.min(start_origin[1], end_origin[1]);

  svg.select("#"+line_label+"_1").remove();

  svg.append("rect")
  .attr("id",line_label+"_1")
  .attr("x", x)
  .attr("y", y)
  .attr("width", width)
  .attr("height", height)
  .attr("fill-opacity", "100%")
  .attr('stroke', "black")
  .attr("stroke-width", 0)
  .attr("fill", color);
 
}

export {draw_table_general, draw_table_general_with_headings, draw_fu, draw_info_box, draw_transmission_line}