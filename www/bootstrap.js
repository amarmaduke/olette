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
const back_button = document.getElementById("back_button");
const forward_button = document.getElementById("forward_button");
const input = document.getElementById("lambda_input");
const dropdown = document.getElementById("dropdown");
const dropdown_button = document.getElementById("dropdown_button");
const auto_choice = document.getElementById("auto");
const duplicate_choice = document.getElementById("duplicate");
const cancel_choice = document.getElementById("cancel");
const time_input = document.getElementById("time_input");
const timer_set_button = document.getElementById("timer_set_button");
const title_input = document.getElementById("title_input");
const title_set_button = document.getElementById("title_set_button");
const window = document.getElementById("window");

var continue_reduce = false;
var time_delay = 1500;


class Node {
    constructor(value, next, prev) {
        this.value = value;
        this.next = next;
        this.prev = prev;
    }
}

class LinkedList {
    constructor() {
        this.head = null;
        this.tail = null;
        this.cur = null;
        this.size = 0;
    }

    add(value) {
        const node = new Node(value, null, this.tail);
        this.size++;
        if (this.head == null) {
            this.head = node;
            this.tail = node;
        }
        else {
            this.tail.next = node;
        }
        this.tail = node;
        this.cur = this.tail;
    }
    addHead(value) {
        const node = new Node(value, null, null);
        this.head = node;
        this.size = 1;
        this.cur = this.head;
        this.tail = this.head;
    }
}
var history = new LinkedList();

Promise.all([promise]).then(promises => {
    var olette = promises[0];

    var rule_kind = "auto";
    var simulation, simulation_flag = true;
    var node, link, port, label, title;
    var data;




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
        rule_kind = "auto";
    });

    duplicate_choice.addEventListener("click", event => {
        event.stopPropagation();
        rule_kind = "duplicate";
    });

    cancel_choice.addEventListener("click", event => {
        event.stopPropagation();
        rule_kind = "cancel";
    });


    button.addEventListener('click', button_interact(button, load), true);

    reduce_button.addEventListener("click", button_interact(reduce_button, reduce), true);

    reduce_auto_button.addEventListener("click", button_interact(reduce_auto_button, reduce_auto), true);

    cancel_button.addEventListener("click", button_interact(cancel_button, cancel), true);

    back_button.addEventListener("click", button_interact(back_button, back), true);
    back_button.setAttribute("disabled", "");

    forward_button.addEventListener("click", button_interact(forward_button, forward), true);
    forward_button.setAttribute("disabled", "");

    timer_set_button.addEventListener("click", button_interact(timer_set_button, timer_set), true);

    title_set_button.addEventListener("click", button_interact(title_set_button, title_set), true);

    window.addEventListener("keydown", key_press, true);
    window.addEventListener("keyup", key_up, true);


    function button_interact(button, callback) {
        return (element, event) => {
            button.classList.add("is-loading");
            callback(element, event);
            button.classList.remove("is-loading");
        }
    }

    var width = document.documentElement.clientWidth;
    var height = document.documentElement.clientHeight;

    var holder = d3.select("body")
        .append("svg")
        .attr("width", width)
        .attr("height", height);

    // draw the text
    holder.append("text")
        .style("fill", "black")
        .style("font-size", "56px")
        .attr("dy", ".35em")
        .attr("text-anchor", "middle")
        .attr("transform", "translate(300,150) rotate(0)")
        .text("Test Rotation");

    // when the input range changes update the angle 
    d3.select("#nAngle").on("input", function () {
        updateAngle(+this.value);
    });

    // Initial starting angle of the text 
    updateAngle(0);


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
        svg.append("g").attr("class", "title");
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
            .attr("stroke", d => "#708090")
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
            .attr("title", d => d.title)
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
            .style('font-family', 'Helvetica')
            .style('font-size', '8px')
            .style("cursor", "pointer")
            .text(d => d.label)
            .on("click", clicked)
            .merge(label);
        label.call(drag);

        title = svg.select(".title").selectAll("text")
            .data(data.nodes, d => d.id);
        title.exit().transition(t)
            .style("opacity", 1e-6)
            .remove();
        title = title.enter().append("text")
            .attr('text-anchor', 'start')
            .attr('dominant-baseline', 'text-after-edge')
            .style('font-family', 'Helvetica')
            .style('font-size', '4px')
            .style("cursor", "pointer")
            .text(d => d.title)
            .on("click", clicked)
            .merge(title);
        title.call(drag);
            


        simulation.nodes(data.nodes).on("tick", tick);
        simulation.force("link").links(data.links)
            .strength(k => k.force * (1 / 3));
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
            .attr("stroke", d => d.color ? d.color : "#708090")
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

                let tx = r * tx_normal + d.target.x;
                let ty = r * ty_normal + d.target.y;
                let sx = r * sx_normal + d.source.x;
                let sy = r * sy_normal + d.source.y;

                let sangle = Math.atan2(ty - d.source.y, tx - d.source.x)
                    - Math.atan2(sy - d.source.y, sx - d.source.x);
                let tangle = Math.atan2(sy - d.target.y, sx - d.target.x)
                    - Math.atan2(ty - d.target.y, tx - d.target.x);
                let flag = Math.abs(sangle) >= Math.PI / 4 || Math.abs(tangle) >= Math.PI / 4;

                let tx_ = p * r * tx_normal + d.target.x;
                let ty_ = p * r * ty_normal + d.target.y;
                let sx_ = p * r * sx_normal + d.source.x;
                let sy_ = p * r * sy_normal + d.source.y;

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

        port.attr("cx", d => 15 * Math.cos(toRadians(d.ports[0])) + d.x)
            .attr("cy", d => 15 * Math.sin(toRadians(d.ports[0])) + d.y);

        label.attr("x", d => d.x)
            .attr("y", d => d.y)
            .text(d => d.label)
            .style("font-size", "20px")
            .style("fill", "#4393c3");

        title.attr("x", d => d.x +20)
            .attr("y", d => d.y)
            .text(d => d.title)
            .style("font-size", "20px")
            .style("fill", "#203644");
    }
    function updateAngle(nAngle) {

        // adjust the text on the range slider
        d3.select("#nAngle-value").text(nAngle);
        d3.select("#nAngle").property("value", nAngle);

        // rotate the text
        holder.select("text")
            .attr("transform", "translate(300,150) rotate(" + nAngle + ")");
        
    }

    function clicked(d) {
        let previous = node.filter((d, i) => d.id === selection).node();
        let previous_wire = data.links.filter(
            d => (d.source.id == selection && d.p.s == 0)
                || (d.target.id == selection && d.p.t == 0))[0];
        if (previous !== null) {
            previous.__data__.color = previous_color;
            previous_wire.width = "2";
            previous_wire.color = "#708090";
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
        back_button.setAttribute("disabled", "");
        continue_reduce = true;
        history.addHead(JSON.stringify(data));
        update(1.0);
        Storage.set("net", data);
    }

    function reduce() {
        if (!reduce_button.hasAttribute("disabled")) {
            let darray = { "nodes": [] };
            for (let i = 0; i < data.nodes.length; ++i) {
                let d = data.nodes[i];
                let k = {
                    "id": d.id,
                    "x": d.x,
                    "y": d.y,
                    "fixed": d.fixed,
                    "label": d.label,
                    "title": d.title
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
            history.tail = history.cur;
            history.add(JSON.stringify(data));
            history.cur = history.tail;
            update(0.6);
            Storage.set("net", data);
            selection = undefined;
            reduce_button.setAttribute("disabled", "");
            back_button.removeAttribute("disabled");
        }
    }

    function reduce_auto() {
        if (history.cur != null) {
            for (let i = 0; i < data.nodes.length; ++i) {
                reduce_auto_button.setAttribute("disabled", "");
                if (continue_reduce == false) {
                    reduce_auto_button.removeAttribute("disabled");
                    continue_reduce = true;
                    update(1.0);
                    Storage.set("net", data);
                    break;
                }
                else {
                    let d = data.nodes[i];

                    clicked(d);
                    update(1.0);
                    Storage.set("net", data);
                    if (!reduce_button.hasAttribute("disabled")) {

                        reduce_button.click();
                        setTimeout(function () {
                            reduce_auto_button.click();
                        }, time_delay);
                        break;
                    }

                }
            }
        }
    }
    function cancel() {
        continue_reduce = false;
    }

    function timer_set() {
        if (!isNaN(time_input.value)) {
            time_delay = time_input.value * 1000;
        }
        time_input.value = '';
    }

    function title_set() {
        if (typeof title_input.value === 'string' || title_input.value instanceof String) {
            title_set = title_input.value;
        }
        let cur = node.filter((d, i) => d.id === selection).node();
        if (cur != null) {
            cur.__data__.title = title_input.value;
            update(1.0);
            let cur_data = JSON.parse(history.cur.value);
            for (let i = 0; i < cur_data.nodes.length; ++i) {
                if (cur.__data__.id == cur_data.nodes[i].id) {
                    cur_data.nodes[i].title = cur.__data__.title;
                }
            }
            let new_cur = JSON.stringify(cur_data);
            history.cur.value = new_cur;
        }
        title_input.value ='';
    }

    function back() {
        if (!back_button.hasAttribute("disabled")) {
            continue_reduce = false;
            reduce_auto_button.removeAttribute("disabled");
            if (history.cur.prev != null) {
                history.cur = history.cur.prev;
            }
            let previous_data = history.cur.value;
            olette.rebuild_net(previous_data);
            data = JSON.parse(previous_data);
            selection = undefined;
            for (let i = 0; i < data.nodes.length; ++i) {
                data.nodes[i].color = "black";
            }
            for (let i = 0; i < data.links.length; ++i) {
                data.links[i].color = "#708090";

            }

            for (let i = 0; i < data.nodes.length; ++i) {
                let d = data.nodes[i];
                if (d.fixed) {
                    d.fx = d.x;
                    d.fy = d.y;
                }
            }
            update(1);
            continue_reduce = true;
            if (history.cur.prev == null) {
                back_button.setAttribute("disabled", "");
            }
            forward_button.removeAttribute("disabled");
        }

    }

    function forward() {
        if (!forward_button.hasAttribute("disabled")) {
            if (history.cur.next != null) {
                history.cur = history.cur.next;
            }
            let previous_data = history.cur.value;
            olette.rebuild_net(previous_data);
            data = JSON.parse(previous_data);
            selection = undefined;
            for (let i = 0; i < data.nodes.length; ++i) {
                data.nodes[i].color = "black";
            }
            for (let i = 0; i < data.links.length; ++i) {
                data.links[i].color = "#708090";

            }

            for (let i = 0; i < data.nodes.length; ++i) {
                let d = data.nodes[i];
                if (d.fixed) {
                    d.fx = d.x;
                    d.fy = d.y;
                }
            }
            update(1);
            continue_reduce = true;
            if (history.cur.next == null) {
                forward_button.setAttribute("disabled", "");
            }
            back_button.removeAttribute("disabled");
        }
    }



    var alt = false;
    var agents_visited = -1;
    function key_press(event) {
        var key = event.keyCode;
        if (key == 13) { //enter
            if (document.getElementById("titleNavigation").style.width == "250px") {
                title_set_button.click();
            } else if (document.getElementById("timeNavigation").style.width == "250px") {
                timer_set();
            } else {
                load_button.click();
            }
        } else if (key == 18) { // alt
            alt = true;
        } else if (key == 192) { // `
            if (document.getElementById("sideNavigation").style.width == "250px") {
                closeNav();
            } else if (document.getElementById("timeNavigation").style.width == "250px") {
                closeTimeNav();
            } else if (document.getElementById("controlNavigation").style.width == "250px") {
                closeControlNav();
            } else if (document.getElementById("graphNavigation").style.width == "250px") {
                closeGraphNav();
            } else {
                openNav();
            }
        } else if (key == 65 && selection != undefined && alt == true) { //a + alt
            auto_choice.click();
            reduce_button.click();
        } else if (key == 68 && selection != undefined && alt == true) {//d + alt
            duplicate_choice.click();
            reduce_button.click();
        } else if (key == 67 && selection != undefined && alt == true) {//c + alt
            cancel_choice.click();
            reduce_button.click();
        } else if (key == 82 && alt == true) { //r + alt
            reduce_auto_button.click();
        } else if (key == 37 ) { //left arrow
            back_button.click();
        } else if (key == 39) { //right arrow
            forward_button.click();
        } else if (key == 81 && alt == true) { //q + alt
            cancel_button.click();
        } else if (key == 90 && alt == true) { //z + alt
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
    }

    function key_up(event) {
        var key = event.keyCode;
        if (key == 18) {
            alt = false;
        }
    }


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
    set: function (key, value) {
        sessionStorage[key] = JSON.stringify(value);
    },
    get: function (key) {
        return sessionStorage[key] ? JSON.parse(sessionStorage[key]) : null;
    },
    poll: function (key, timer) {
        if (!sessionStorage[key]) return (timer = setTimeout(Storage.poll.bind(null, key), 100));
        clearTimeout(timer);
        return JSON.parse(sessionStorage[key]);
    },
    clear: function (key) {
        sessionStorage[key] = null;
    }
};

function toDegrees(angle) {
    return angle * (180 / Math.PI);
}

function toRadians(angle) {
    return angle * (Math.PI / 180);
}

// back  taking snapshots of graph
//calling update_net to sync