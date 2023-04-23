Feature: Simulation Executor
  As a train simulation user
  I want to run a train simulation with a specific frame rate and runtime
  So that I can observe the train's progress towards its destination

  Scenario: Execute a train simulation for 2 minutes without sleep and 20 fps
    Given the JSON data from "src/tests/res/vilbel.json"
    When the railway graph is imported
    Given a train is placed at node 6204567501 with target 662529467
    And a SimulationExecutor is created with 20 fps and 120 seconds
    When the simulation is executed
    Then the train should be closer to the target node
