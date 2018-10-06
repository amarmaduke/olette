import * as d3 from 'd3'

document.addEventListener('DOMContentLoaded', init, false);
function init(){
  var button = document.getElementById('load_button');
  button.addEventListener('click', button_interact(button, load), true);
}

function button_interact(button, callback) {
  return (element, event) => {
    button.classList.add("is-loading");
    callback(element, event);
    button.classList.remove("is-loading");
  }
}

var Storage = {
  set: function(key, value) {
    sessionStorage[key] = JSON.stringify(value);
  },
  get: function(key) {
    return sessionStorage[key] ? JSON.parse(sessionStorage[key]) : null;
  },
  poll: function(key, timer) {
    if (!sessionStorage[key]) return (timer = setTimeout(Storage.poll.bind(null, key), 100));
    clearTimeout(timer);
    return JSON.parse(sessionStorage[key]);
  },
  clear: function(key) {
      sessionStorage[key] = null;
  }
};

function toDegrees (angle) {
  return angle * (180 / Math.PI);
}

function toRadians (angle) {
  return angle * (Math.PI / 180);
}

export { toRadians };

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
var color = d3.scaleOrdinal(d3.schemeAccent);

var simulation = d3.forceSimulation()
  .force("link", d3.forceLink().distance(80))
  .force("charge", d3.forceManyBody())
  .force("center", d3.forceCenter(width / 2, height / 2));

function clear() {
    svg.selectAll("*").remove();
}

function patch() {
  simulation.stop();
  var graph = Storage.poll("net");


}

function load() {
  simulation.stop();
  clear();
  load_net("\\x. x");
  var graph = Storage.poll("net");
  Storage.clear("net");

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
  simulation.restart();

  function tick() {
    link
      .each(d => { d.source.y -= k, d.target.y += k; })
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
