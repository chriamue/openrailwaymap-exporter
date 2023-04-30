import openrailwaymap_exporter

def test_openrailwaymap_exporter():
    # Test input JSON string
    input_json = """
    {
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
    }
    """

    # Create an instance of PyOverpassImporter
    importer = openrailwaymap_exporter.PyOverpassImporter()

    # Import the railway graph data from the JSON string
    railway_graph = importer.import_graph(input_json)

    # Verify the node and edge counts
    assert railway_graph.node_count() == 2
    assert railway_graph.edge_count() == 1

if __name__ == '__main__':
    test_openrailwaymap_exporter()
    print("All tests passed.")
