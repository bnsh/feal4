#! /usr/bin/env python3
# vim: expandtab shiftwidth=4 tabstop=4

"""Finally draw the graph."""

import os
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

if __name__ == "__main__":
    main()
