import pytest
from openrailwaymap_exporter import PyOverpassImporter, PyRailwayGraph


@pytest.fixture
def railway_graph():
    importer = PyOverpassImporter()
    json_data = """{
        "elements": [
            {
                "id": 1,
                "lat": 48.777,
                "lon": 9.234,
                "tags": {
                    "railway": "station"
                },
                "type": "node"
            },
            {
                "id": 2,
                "lat": 48.778,
                "lon": 9.235,
                "tags": {
                    "railway": "station"
                },
                "type": "node"
            },
            {
                "id": 3,
                "nodes": [1, 2],
                "tags": {
                    "railway": "rail"
                },
                "geometry": [
                    {
                        "lat": 50.1109,
                        "lon": 8.6821
                    },
                    {
                        "lat": 50.1073,
                        "lon": 8.6637
                    }
                ],
                "type": "way"
            }
        ],
        "version": 0.6
    }"""
    return importer.import_graph(json_data)


def test_node_count(railway_graph):
    assert railway_graph.node_count() ==  2


def test_edge_count(railway_graph):
    assert railway_graph.edge_count() == 1


def test_get_node_by_id(railway_graph):
    node = railway_graph.get_node_by_id(1)
    assert node is not None
    assert node['id'] == 1


def test_get_edge_by_id(railway_graph):
    edge = railway_graph.get_edge_by_id(3)
    assert edge is not None
    assert edge['id'] == 3
