// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
import * as d3 from 'd3'
let promise = import("./src/index.js")

const button = document.getElementById('load_button');
const reduce_button = document.getElementById("reduce_button");
const force_button = document.getElementById("force_button");
const input = document.getElementById("lambda_input");
const dropdown = document.getElementById("dropdown");
const dropdown_button = document.getElementById("dropdown_button");
const auto_choice = document.getElementById("auto");
const duplicate_choice = document.getElementById("duplicate");
const cancel_choice = document.getElementById("cancel");

Promise.all([promise]).then(promises => {
    var yalar = promises[0];

    var rule_kind = "auto";
    var current_active = auto_choice;
    var simulation, simulation_flag = true;
    var node, link, port, label;
    var data;

    dropdown_button.addEventListener("click", event => {
        event.stopPropagation();
        dropdown.classList.toggle("is-active");
    });

    force_button.addEventListener("click", event => {
        if (simulation_flag) {
            node.each(d => { d.fx = d.x; d.fy = d.y; });
            simulation_flag = false;
        } else {
            node.each(d => { d.fx = null; d.fy = null; });
            simulation.alphaTarget(1);
            simulation_flag = true;
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
        .force("charge", d3.forceManyBody().strength(-400))
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
        simulation.force("link").links(data.links);
            //.strength(k => k.force * (1/3));
        simulation.alphaTarget(alpha).restart();
    }

    function tick() {
        let k = 6 * simulation.alpha();

        link
            //.each(d => { d.source.y -= k, d.target.y += k; })
            .attr("d", d => {
                let tx_normal = Math.cos(toRadians(d.ports.t));
                let ty_normal = Math.sin(toRadians(d.ports.t));
                let sx_normal = Math.cos(toRadians(d.ports.s));
                let sy_normal = Math.sin(toRadians(d.ports.s));

                let r = 15;
                let p = 4;
                if (d.source == d.target) {
                    p = 2;
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
            .style("font-size", "20px")
            .style("fill", "#4393c3");
    }

    function clicked(d) {
        let previous = node.filter((d, i) => d.id === selection).node();
        if (previous !== null) {
            previous.__data__.color = previous_color;
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
        d.color = "red"
    }

    function load() {
        clear();
        data = JSON.parse(yalar.load_net(input.value));
        update(1.0);
        Storage.set("net", data);
    }

    function reduce() {
        var patch = JSON.parse(yalar.reduce_net(selection, rule_kind));
        simulation.stop();

        // d3js should handle this merging part, but for whatever reason it's not
        data.links = patch.links;
        data.nodes = data.nodes.filter(x => patch.nodes.find(y => x.id === y.id));
        for (let i = 0; i < patch.nodes.length; ++i) {
            let p = patch.nodes[i];
            let d = data.nodes.find(y => y.id === p.id);
            if (d) {
                d.color = p.color;
                d.width = p.width;
            } else {
                p.x = selection_x;
                p.y = selection_y;
                data.nodes.push(p);
            }
        }
        // ....

        update(0.6);
        Storage.set("net", data);
        selection = undefined;
        reduce_button.setAttribute("disabled", "");
    }

    var agents_visited = -1;
    document.onkeydown = function (event) {
        var key = event.keyCode;
        if (key == 13) {
            reduce_button.click();
        } else if (key == 65) {
            auto_choice.click();
            dropdown_button.click();
            reduce_button.click();
        } else if (key == 68) {
            duplicate_choice.click();
            dropdown_button.click();
            reduce_button.click();
        } else if (key == 67) {
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
        }
    };

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
        //d.fx = null;
        //d.fy = null;
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
