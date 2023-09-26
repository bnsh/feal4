#! /usr/bin/env python3
# vim: expandtab shiftwidth=4 tabstop=4

"""Finally draw the graph."""

import os
import json
from collections import Counter
import xml.etree.ElementTree as ET

import networkx as nx

from main import encrypt

def create_graph(nodes, edges):
    # Create a directed graph
    graph = nx.DiGraph()

    # Add nodes with labels
    for nodeidx, datum in nodes.items():
        graph.add_node(str(nodeidx), **datum)

    # Add directed edges with labels
    for srcidx, dstidx, label in edges:
        graph.add_edge(str(srcidx), str(dstidx), label=label if label else "")

    return graph

def translate_keys(srcfn, dstfn):
    # Parse the GraphML file
    tree = ET.parse(srcfn)
    root = tree.getroot()

    # Namespace dictionary
    namespaces = {'ns': 'http://graphml.graphdrawing.org/xmlns'}

    oldmap = {}
    for key in root.findall("ns:key", namespaces=namespaces):
        keyid = key.attrib.get("id")
        keyname = key.attrib.get("attr.name")
        key.attrib["id"] = keyname
        oldmap[keyid] = keyname

    for graph in root.findall("ns:graph", namespaces=namespaces):
        for comp in graph.findall("ns:node", namespaces=namespaces) + graph.findall("ns:edge", namespaces=namespaces):
            for data in comp.findall("ns:data", namespaces=namespaces):
                datakey = data.attrib.get("key")
                data.attrib["key"] = oldmap[datakey]

    tree.write(dstfn, encoding='UTF-8', xml_declaration=True)

def label_renamer(node):
    label = node["label"]
    if label == ".":
        label = f"copy{node['bitsize']:d}"
    elif label == "xor":
        label = f"xor{node['bitsize']:d}"
    return label

def convert_node(key, edges, node):
    red, green, blue = node["r"], node["g"], node["b"]

    edges = sorted(set(edges))
    assert all(count == 1 for _, count in Counter(label for src, dst, label in edges if dst == key).items())
    node_dependencies = {label: src for src, dst, label in edges if dst == key}

    retval = {
        "id": key,
        "color": f"#{red:02x}{green:02x}{blue:02x}",
        "label": label_renamer(node),
        "size": node["size"],
        "x": node["x"],
        "y": -node["y"],
        "bitsize": node["bitsize"],
        "radius": 1 if node["label"] == "." else 20
    }
    retval.update(node_dependencies)
    return retval

def make_enum(fname, edges, nodes):
    """Generate the Rust Enum."""

    def depstr(dependencies):
        return ", ".join(f"{dep:s}: i32" for dep, bitsize in sorted(dependencies))

    ordering = {}
    types = {}
    for idx, node in nodes.items():
        label = label_renamer(node)
        types[label] = set()
        if label not in ordering:
            ordering[label] = idx
    for node_label, incoming_label, src in sorted([(label_renamer(node), incoming_label, src) for key, node in nodes.items() for src, dst, incoming_label in edges if dst == key]):
        bitsize = nodes[src]["bitsize"]
        types[node_label].add((incoming_label, bitsize))


    guts = ",\n\n    ".join([f"#[serde(rename = \"{node_type:s}\")]\n    {node_type.capitalize():s} {{{depstr(dependencies):s}}}" for node_type, dependencies in sorted(types.items(), key=lambda x: ordering[x[0]])])
    with open(fname, "wt", encoding="utf-8") as rfp:
        rfp.write(f"""// vim: expandtab shiftwidth=4 tabstop=4:

/* We're going to try to build a webasm helper
 * to help us cryptanalyze FEAL-8. And, I guess
 * in the process learn yew.rs.
 * Remember all the values (src, subkey, value, etc. are node _indices_ not actual _values_!
 */

use serde::{{Deserialize, Serialize}};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "label")]
enum ComputationGraph {{
    {guts:s}
}}
""")

#pylint: disable=too-many-locals
def main():
    dummy_plaintext, dummy_key0, dummy_key1, dummy_key2, dummy_key3, dummy_key4, dummy_key5, dummy_key6, dummy_key7, dummy_key8_11, dummy_key12_15, ciphertext = encrypt()
    nodes = {}
    edges = []

    ciphertext.populate_nodes(nodes)
    ciphertext.populate_edges(edges)
    graph = create_graph(nodes, edges)
    nx.write_graphml(graph, "graph-tmp.graphml")
    translate_keys("graph-tmp.graphml", "graph.graphml")
    os.unlink("graph-tmp.graphml")

    with open("graph-tmp.json", "wt", encoding="utf-8") as jsfp:
        json.dump([convert_node(key, edges, node) for key, node in sorted(nodes.items(), key=lambda x: x[0])], jsfp, indent=4, sort_keys=True)
    with open("graph-tmp.json", "rt", encoding="utf-8") as jsfp:
        arr = json.load(jsfp)
        assert all(node["id"] == idx for idx, node in enumerate(arr))
    os.rename("graph-tmp.json", "graph.json")

    make_enum("computation_graph.rs", edges, nodes)
#pylint: enable=too-many-locals


if __name__ == "__main__":
    main()
