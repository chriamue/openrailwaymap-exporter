Feature: Railway graph

  Scenario: Load railway graph from test1
    Given the JSON data from "src/tests/res/test1.json"
    When the railway graph is imported
    Then the graph should have 4 nodes
    And the graph should have 3 edges

  Scenario: Test Vilbel JSON
    Given the JSON data from "src/tests/res/vilbel.json"
    When the railway graph is imported
    Then the graph should have 68 nodes
    And the graph should have 68 edges
