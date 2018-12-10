// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
import * as d3 from 'd3'
import { setTimeout } from 'timers';
let promise = import("./src/index.js");


const button = document.getElementById('load_button');
const reduce_button = document.getElementById("reduce_button");
const reduce_auto_button = document.getElementById("reduce_auto_button");
const force_on_button = document.getElementById("force_on_button");
const force_off_button = document.getElementById("force_off_button");
const cancel_button = document.getElementById("cancel_button");
const input = document.getElementById("lambda_input");
const dropdown = document.getElementById("dropdown");
const dropdown_button = document.getElementById("dropdown_button");
const auto_choice = document.getElementById("auto");
const duplicate_choice = document.getElementById("duplicate");
const cancel_choice = document.getElementById("cancel");
var continue_reduce = false;

Promise.all([promise]).then(promises => {
    var olette = promises[0];

    var rule_kind = "auto";
    var current_active = auto_choice;
    var simulation, simulation_flag = true;
    var node, link, port, label;
    var data;
    
    

    dropdown_button.addEventListener("click", event => {
        event.stopPropagation();
        dropdown.classList.toggle("is-active");
    });

    force_on_button.addEventListener("click", event => {
        simulation_flag = true;
        node.each(d => { d.fx = null; d.fy = null; d.fixed = false; });
        simulation.alphaTarget(1);
        for (let i = 0; i < data.nodes.length; ++i) {
            let d = data.nodes[i];
            d.fx = null;
            d.fy = null;
            d.fixed = false;
        }
    });

    force_off_button.addEventListener("click", event => {
        simulation_flag = false;
        node.each(d => { d.fx = d.x; d.fy = d.y; d.fixed = true; });
        for (let i = 0; i < data.nodes.length; ++i) {
            let d = data.nodes[i];
            d.fx = d.x;
            d.fy = d.y;
            d.fixed = true;
        }
    });

    auto_choice.addEventListener("click", event => {
        event.stopPropagation();
        current_active.classList.toggle("is-active");
        auto_choice.classList.toggle("is-active");
        current_active = auto_choice;
        rule_kind = "auto";
        dropdown.classList.toggle("is-active");
    });

    duplicate_choice.addEventListener("click", event => {
        event.stopPropagation();
        current_active.classList.toggle("is-active");
        duplicate_choice.classList.toggle("is-active");
        current_active = duplicate_choice;
        rule_kind = "duplicate";
        dropdown.classList.toggle("is-active");
    });

    cancel_choice.addEventListener("click", event => {
        event.stopPropagation();
        current_active.classList.toggle("is-active");
        cancel_choice.classList.toggle("is-active");
        current_active = cancel_choice;
        rule_kind = "cancel";
        dropdown.classList.toggle("is-active");
    });

    button.addEventListener('click', button_interact(button, load), true);
    reduce_button.addEventListener("click", button_interact(reduce_button, reduce), true);
    reduce_auto_button.addEventListener("click", button_interact(reduce_auto_button, reduce_auto), true);
    cancel_button.addEventListener("click", button_interact(cancel_button, cancel), true);
    function button_interact(button, callback) {
        return (element, event) => {
            button.classList.add("is-loading");
            callback(element, event);
            button.classList.remove("is-loading");
        }
    }
  
    

    var width = document.documentElement.clientWidth;
    var height = document.documentElement.clientHeight;
    var selection, previous_color, selection_x, selection_y;

    d3.select("div.net")
        .append("div")
        .classed("svg-container", true) //container class to make it responsive
        .append("svg")
        .attr("id", "svg-render")
        //responsive SVG needs these 2 attributes and no width and height attr
        .attr("preserveAspectRatio", "xMinYMin meet")
        .attr("viewBox", "0 0 " + width + " " + height)
        //class to make it responsive
        .classed("svg-content-responsive", true); 

    var svg = d3.select("#svg-render")
        .call(d3.zoom().on("zoom", function () {
            svg.attr("transform", d3.event.transform)
        })).append("g");
    clear();
    var color = d3.scaleOrdinal(d3.schemeAccent);

    simulation = d3.forceSimulation()
        .force("link", d3.forceLink().distance(80))
        .force("charge", d3.forceManyBody().strength(-600))
        .force("x", d3.forceX(width / 2))
        .force("y", d3.forceY(height / 2));

    let saved = Storage.get("net");
    if (saved) {
        load(saved);
    }

    function clear() {
        svg.selectAll("*").remove();
        svg.append("g").attr("class", "link")
            .style("stroke", "#aaa");
        svg.append("g").attr("class", "node");
        svg.append("g").attr("class", "port");
        svg.append("g").attr("class", "label");
    }

    function update(alpha) {
        var t = d3.transition().duration(250);
        let drag = d3.drag()
            .on("start", dragstarted)
            .on("drag", dragged)
            .on("end", dragended);

        link = svg.select(".link").selectAll("path")
            .data(data.links, d => d.source.id + "-" + d.target.id);
        link.exit().remove();
        // link.stuff() If we want to update links
        link = link.enter().append("path")
            .attr("fill", "transparent")
            .attr("stroke", d => "#ddd")
            .merge(link);

        node = svg.select(".node").selectAll("circle")
            .data(data.nodes, d => d.id);
        node.exit().transition(t)
            .style("opacity", 1e-6)
            .remove();
        node.attr("stroke", d => d.color)
            .attr("stroke-width", d => d.width);
        node = node.enter().append("circle")
            .attr("r", 15)
            .attr("stroke", d => d.color)
            .attr("stroke-width", d => d.width)
            .attr("fill", d => color(d.kind))
            .attr("fx", d => d.fx)
            .attr("fy", d => d.fy)
            .attr("id", d => d.id)
            .on("click", clicked)
            .merge(node);
            //.append("title", d => d.id)
        node.call(drag);

        port = svg.select(".port").selectAll("circle")
            .data(data.nodes, d => d.id);
        port.exit().transition(t)
            .style("opacity", 1e-6)
            .remove();
        port = port.enter().append("circle")
            .attr("r", 3)
            .attr("fill", "black")
            .merge(port);

        label = svg.select(".label").selectAll("text")
            .data(data.nodes, d => d.id);
        label.exit().transition(t)
            .style("opacity", 1e-6)
            .remove();
        label = label.enter().append("text")
            .attr('text-anchor', 'middle')
            .attr('dominant-baseline', 'central')
            .style('font-family','Helvetica')
            .style('font-size','8px')
            .style("cursor", "pointer")
            .text(d => d.label)
            .on("click", clicked)
            .merge(label);
        label.call(drag);

        simulation.nodes(data.nodes).on("tick", tick);
        simulation.force("link").links(data.links)
            .strength(k => k.force * (1/3));
        simulation.alphaTarget(alpha).restart();
    }

    function tick() {
        let k = 6 * simulation.alpha();

        link
            .each(d => {
                if (!d.source.fixed && !d.target.fixed) {
                    d.source.y -= d.force * k;
                    d.target.y += d.force * k;
                }
            })
            .attr("stroke", d => d.color ? d.color : "#ddd")
            .attr("stroke-width", d => d.width ? d.width : "2")
            .attr("d", d => {
                let tx_normal = Math.cos(toRadians(d.ports.t));
                let ty_normal = Math.sin(toRadians(d.ports.t));
                let sx_normal = Math.cos(toRadians(d.ports.s));
                let sy_normal = Math.sin(toRadians(d.ports.s));

                let r = 15;
                let p = 4;
                if (d.source == d.target) {
                    p = 3;
                }

                let tx = r*tx_normal + d.target.x;
                let ty = r*ty_normal + d.target.y;
                let sx = r*sx_normal + d.source.x;
                let sy = r*sy_normal + d.source.y;

                let sangle = Math.atan2(ty - d.source.y, tx - d.source.x)
                    - Math.atan2(sy - d.source.y, sx - d.source.x);
                let tangle = Math.atan2(sy - d.target.y, sx - d.target.x)
                    - Math.atan2(ty - d.target.y, tx - d.target.x);
                let flag = Math.abs(sangle) >= Math.PI/4 || Math.abs(tangle) >= Math.PI/4;

                let tx_ = p*r*tx_normal + d.target.x;
                let ty_ = p*r*ty_normal + d.target.y;
                let sx_ = p*r*sx_normal + d.source.x;
                let sy_ = p*r*sy_normal + d.source.y;

                let midx = (sx_ + tx_) / 2;
                let midy = (sy_ + ty_) / 2;

                let path = "M" + sx + "," + sy + "L" + tx + "," + ty;
                if (flag) {
                    path = "M" + sx + "," + sy +
                        "S" + sx_ + "," + sy_ + " " + midx + "," + midy +
                        "M" + tx + "," + ty +
                        "S" + tx_ + "," + ty_ + " " + midx + "," + midy;
                }

                return path;
            });
        
        node.attr("cx", d => d.x)
            .attr("cy", d => d.y)
            .attr("stroke", d => d.color);
        
        port.attr("cx", d => 15*Math.cos(toRadians(d.ports[0])) + d.x)
            .attr("cy", d => 15*Math.sin(toRadians(d.ports[0])) + d.y);
        
        label.attr("x", d => d.x)
            .attr("y", d => d.y)
            .text(d => d.label)
            .style("font-size", "20px")
            .style("fill", "#4393c3");
    }

    function clicked(d) {
        let previous = node.filter((d, i) => d.id === selection).node();
        let previous_wire = data.links.filter(
            d => (d.source.id == selection && d.p.s == 0)
            || (d.target.id == selection && d.p.t == 0))[0];
        if (previous !== null) {
            previous.__data__.color = previous_color;
            previous_wire.width = "2";
            previous_wire.color = "#ddd";
        }
        if (d.color === "black") {
            reduce_button.removeAttribute("disabled");
        } else {
            reduce_button.setAttribute("disabled", "");
        }
        reduce_button.disabled = (d.color !== "black");
        selection = d.id;
        selection_x = d.x;
        selection_y = d.y;
        previous_color = d.color;
        d.color = "red";
        let wire = data.links.filter(
            d => (d.source.id == selection && d.p.s == 0)
            || (d.target.id == selection && d.p.t == 0))[0];
        wire.width = "2";
        wire.color = "black";
    }

    function load() {
        clear();
        data = JSON.parse(olette.load_net(input.value));
        reduce_auto_button.removeAttribute("disabled");
        continue_reduce = true;
        update(1.0);
        Storage.set("net", data);
    }

    function reduce() {
        let darray = { "nodes": [] };
        for (let i = 0; i < data.nodes.length; ++i) {
            let d = data.nodes[i];
            let k = {
                "id": d.id,
                "x": d.x,
                "y": d.y,
                "fixed": d.fixed,
                "label": d.label
            };
            darray.nodes.push(k);
        }
        olette.update_net(JSON.stringify(darray));
        var patch = JSON.parse(olette.reduce_net(selection, rule_kind));
        simulation.stop();
        data = patch;
        for (let i = 0; i < data.nodes.length; ++i) {
            let d = data.nodes[i];
            if (d.fixed) {
                d.fx = d.x;
                d.fy = d.y;
            }
        }

        update(0.6);
        Storage.set("net", data);
        selection = undefined;
        reduce_button.setAttribute("disabled", "");
    }
    function reduce_auto() {
        // iterate over all the nodes in data.nodes
        // call a rust function to see if it is in a critical pair
        // if yes, set the selection variable to the current node and reduce
        // repeat until there are no critical pairs to reduce
        
        for (let i = 0; i < data.nodes.length; ++i) {
            if (continue_reduce == false) {
                update(1.0);
                Storage.set("net", data);
                break;
            }
            else{
                    let d = data.nodes[i];

                    clicked(d);
                    update(1.0);
                    Storage.set("net", data);
                    if (!reduce_button.hasAttribute("disabled")) {
                        reduce_button.click();
                        setTimeout(function () {
                            reduce_auto_button.click();
                        }, 1500);  
                        break;
                    }

                }
        }
        reduce_auto_button.setAttribute("disabled", "");
    }
    function cancel() {
        continue_reduce = false;
    }

    var agents_visited = -1;
    document.onkeydown = function (event) {
        var key = event.keyCode;
        if (key == 13) {
            load_button.click();
        } else if (key == 65 && selection != undefined) {
            auto_choice.click();
            dropdown_button.click();
            reduce_button.click();
        } else if (key == 68 && selection != undefined) {
            duplicate_choice.click();
            dropdown_button.click();
            reduce_button.click();
        } else if (key == 67 && selection != undefined) {
            cancel_choice.click();
            dropdown_button.click();
            reduce_button.click();
        } else if (key == 90) {
            let filtered = svg.select(".node").selectAll("circle")
                .filter((d, i) => d.color === "black" || d.color === "red");

            if (agents_visited + 1 >= filtered.size()) {
                agents_visited = -1;
            }
            let found = false;
            filtered.each((d, i) => {
                if (i > agents_visited && !found) {
                    clicked(d);
                    found = true;
                    agents_visited += 1;
                }
            });
        } else if (key >= 48 && key <= 57 && selection != undefined) {
            let d = data.nodes.filter(d => d.id === selection)[0];
            d.label = "" + (key - 48);
        }
    };

    function dragstarted(d) {
        if (!d3.event.active) simulation.alphaTarget(0.3).restart();
        d.fx = d.x;
        d.fy = d.y;
        d.fixed = true;
    }

    function dragged(d) {
        d.fx = d3.event.x;
        d.fy = d3.event.y;
    }

    function dragended(d) {
        if (!d3.event.active) simulation.alphaTarget(0);
    }
});

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

// back  taking snapshots of graph
//calling update_net to sync