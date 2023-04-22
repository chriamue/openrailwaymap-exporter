Feature: Railway Edge position_on_edge
  As a railway simulation developer
  I want to calculate the position on the edge given a current location, distance to travel, and direction coordinate
  So that I can update the object's position in the simulation

  Scenario: Calculate the position on the edge
    Given a RailwayEdge with the following properties:
      | id  | length | path_coordinates                             | source | target |
      | 1   | 1500.0 | (8.6821, 50.1109), (8.6921, 50.1209)          | 2      | 3      |
    And a current location at (8.6821, 50.1109)
    And a distance to travel of 500 meters
    And a direction coordinate of (8.6921, 50.1209)
    When I call position_on_edge with the given parameters
    Then the new position should be approximately (8.6854, 50.1140)
