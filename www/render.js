document.addEventListener('DOMContentLoaded', init, false);
function init(){
  var button = document.getElementById('load_button');
  button.addEventListener('click', load, true);
}

var Storage = {
    set: function(key, value) {
        localStorage[key] = JSON.stringify(value);
    },
    get: function(key) {
        return localStorage[key] ? localStorage[key] : null;
    }
};

Storage.set("net", {
    "nodes": [
      {"id": 0, "kind": "root", "label": "®", "ports": [90]},
      {"id": 1, "kind": "lambda", "label": "λ", "ports": [270, 45, 135]},
      {"id": 2, "kind": "application", "label": "@", "ports": [135, 270, 45]},
      {"id": 3, "kind": "duplicator", "label": "△", "ports": [270, 45, 135]}
    ],
    "links": [
      {"source": 0, "target": 1, "ports" : {"s": 90, "t": 270}},
  
      {"source": 1, "target": 0, "ports" : {"s": 270, "t": 90}},
      {"source": 1, "target": 2, "ports" : {"s": 45, "t": 270}},
      {"source": 1, "target": 3, "ports" : {"s": 135, "t": 270}},
  
  
      {"source": 2, "target": 3, "ports" : {"s": 135, "t": 45}},
      {"source": 2, "target": 1, "ports" : {"s": 270, "t": 45}},
      {"source": 2, "target": 3, "ports" : {"s": 45, "t": 135}},
  
  
      {"source": 3, "target": 1, "ports" : {"s": 270, "t": 135}},
      {"source": 3, "target": 2, "ports" : {"s": 45, "t": 135}},
      {"source": 3, "target": 2, "ports" : {"s": 135, "t": 45}}
    ]
  });

function toDegrees (angle) {
  return angle * (180 / Math.PI);
}

function toRadians (angle) {
  return angle * (Math.PI / 180);
}

var width = document.documentElement.clientWidth;
var height = document.documentElement.clientHeight;

d3.select("div.net")
  .append("div")
  .classed("svg-container", true) //container class to make it responsive
  .append("svg")
  //responsive SVG needs these 2 attributes and no width and height attr
  .attr("preserveAspectRatio", "xMinYMin meet")
  .attr("viewBox", "0 0 " + width + " " + height)
  //class to make it responsive
  .classed("svg-content-responsive", true); 


var svg = d3.select("svg")
  .call(d3.zoom().on("zoom", function () {
    svg.attr("transform", d3.event.transform)
  })).append("g");
var color = d3.scaleOrdinal(d3.schemeCategory20);

var simulation = d3.forceSimulation()
  .force("link", d3.forceLink().distance(80))
  .force("charge", d3.forceManyBody())
  .force("center", d3.forceCenter(width / 2, height / 2));

function clear() {
    svg.select("g").selectAll("g > *").remove();
}

function load() {
    clear();
  var graph = {
    "nodes": [
      {"id": 0, "kind": "root", "label": "®", "ports": [90]},
      {"id": 1, "kind": "lambda", "label": "λ", "ports": [270, 45, 135]},
      {"id": 2, "kind": "application", "label": "@", "ports": [135, 270, 45]},
      {"id": 3, "kind": "duplicator", "label": "△", "ports": [270, 45, 135]}
    ],
    "links": [
      {"source": 0, "target": 1, "ports" : {"s": 90, "t": 270}},
  
      {"source": 1, "target": 0, "ports" : {"s": 270, "t": 90}},
      {"source": 1, "target": 2, "ports" : {"s": 45, "t": 270}},
      {"source": 1, "target": 3, "ports" : {"s": 135, "t": 270}},
  
  
      {"source": 2, "target": 3, "ports" : {"s": 135, "t": 45}},
      {"source": 2, "target": 1, "ports" : {"s": 270, "t": 45}},
      {"source": 2, "target": 3, "ports" : {"s": 45, "t": 135}},
  
  
      {"source": 3, "target": 1, "ports" : {"s": 270, "t": 135}},
      {"source": 3, "target": 2, "ports" : {"s": 45, "t": 135}},
      {"source": 3, "target": 2, "ports" : {"s": 135, "t": 45}}
    ]
  };

  var link = svg.append("g")
    .style("stroke", "#aaa")
    .selectAll("line")
    .data(graph.links)
    .enter()
    .append("line");

  var node = svg.append("g").attr("class", "node")
    .selectAll("circle")
    .data(graph.nodes)
    .enter().append("circle")
    .attr("r", 15)
    .attr("fill", d => color(d.kind))
    .call(d3.drag()
      .on("start", dragstarted)
      .on("drag", dragged)
      .on("end", dragended));
  node.append("title")
    .text(function(d) { return d.id; });

  var port = svg.append("g").attr("class", "port")
    .selectAll("circle")
    .data(graph.nodes)
    .enter()
    .append("circle")
    .attr("r", 3)
    .attr("fill", "black")
    .call(d3.drag()
      .on("start", dragstarted)
      .on("drag", dragged)
      .on("end", dragended));

  var label = svg.append("g")
    .attr("class", "label")
    .selectAll("text")
    .data(graph.nodes)
    .enter()
    .append("text")
    .attr('text-anchor', 'middle')
    .attr('dominant-baseline', 'central')
    .style('font-family','Helvetica')
    .style('font-size','8px')
    .style('fill','darkOrange')
    .text(function (d) {return d.label;})
    .call(d3.drag()
      .on("start", dragstarted)
      .on("drag", dragged)
      .on("end", dragended));

  simulation
      .nodes(graph.nodes)
      .on("tick", tick);

  simulation.force("link")
      .links(graph.links);

    function tick() {
        link
            .attr("x1", (d) => 15*Math.cos(toRadians(d.ports.t)) + d.target.x)
            .attr("y1", (d) => 15*Math.sin(toRadians(d.ports.t)) + d.target.y)
            .attr("x2", (d) => 15*Math.cos(toRadians(d.ports.s)) + d.source.x)
            .attr("y2", (d) => 15*Math.sin(toRadians(d.ports.s)) + d.source.y);

        node
            .attr("cx", function(d) { return d.x; })
            .attr("cy", function(d) { return d.y; });

        port
            .attr("cx", (d) => 15*Math.cos(toRadians(d.ports[0])) + d.x)
            .attr("cy", (d) => 15*Math.sin(toRadians(d.ports[0])) + d.y);

        label
            .attr("x", function(d) { return d.x; })
            .attr("y", function (d) { return d.y; })
            .style("font-size", "20px").style("fill", "#4393c3");
    }
}

function dragstarted(d) {
  if (!d3.event.active) simulation.alphaTarget(0.3).restart();
  d.fx = d.x;
  d.fy = d.y;
}

function dragged(d) {
  d.fx = d3.event.x;
  d.fy = d3.event.y;
}

function dragended(d) {
  if (!d3.event.active) simulation.alphaTarget(0);
  d.fx = null;
  d.fy = null;
}