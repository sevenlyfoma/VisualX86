const path = require("path");
const express = require("express");
var morgan = require('morgan');
var process = require('process');
const app = express();
const port = 8080


app.use(morgan('combined'))

app.use(express.static('static'));

app.get("/", function (req, res){
    console.log("home");
    res.sendFile(path.join(__dirname, "/static/home.html"));
});

app.get("/help.html", function (req, res){
    console.log("help");
    res.sendFile(path.join(__dirname, "/static/help.html"));
});

app.get("/myScript.js", function (req, res){
    console.log("script");
    res.sendFile(path.join(__dirname, "/static/myScript.js"));
});

app.get("/d3funcs.js", function (req, res){
    console.log("script");
    res.sendFile(path.join(__dirname, "/static/d3funcs.js"));
});

app.get("/compose_diagram_funcs.js", function (req, res){
    console.log("script");
    res.sendFile(path.join(__dirname, "/static/compose_diagram_funcs.js"));
});

app.get("/draw_diagram_funcs.js", function (req, res){
    console.log("script");
    res.sendFile(path.join(__dirname, "/static/draw_diagram_funcs.js"));
});

app.get("/mystyle.css", function (req, res){
    console.log("script");
    res.sendFile(path.join(__dirname, "/static/mystyle.css"));
});

app.get("/favicon.png", function (req, res){
    console.log("icon");
    res.sendFile(path.join(__dirname, "/static/favicon.png"));
});

app.get("/pkg/model.js", function (req, res){
    console.log("icon");
    res.sendFile(path.join(__dirname, "/static/pkg/model.js"));
});

app.get("/pkg/model_bg.wasm", function (req, res){
    console.log("icon");
    res.sendFile(path.join(__dirname, "/static/pkg/model_bg.wasm"));
});

app.get("/sample_programs/fibonacci.txt", function (req, res){
    console.log("icon");
    res.sendFile(path.join(__dirname, "/static/sample_programs/fibonacci.txt"));
});

app.get("/sample_programs/factorial.txt", function (req, res){
    console.log("icon");
    res.sendFile(path.join(__dirname, "/static/sample_programs/factorial.txt"));
});

app.get("/sample_programs/power.txt", function (req, res){
    console.log("icon");
    res.sendFile(path.join(__dirname, "/static/sample_programs/power.txt"));
});

app.listen(port, '0.0.0.0', function (){
    console.log('Listening on port ' + port);
});

process.on('SIGINT', () => {
  console.info("Interrupted")
  process.exit(0)
})
