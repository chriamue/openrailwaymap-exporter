# The Railway Simulation

In this chapter, we discuss the implementation of a railway simulation, which consists of a railway graph representing the infrastructure and a list of movable railway objects, such as trains, within the simulation. The simulation module provides a `Simulation` struct to manage the state of the simulation, decision agents for the objects, and metrics handlers to process events and gather metrics during the simulation run.

The `Simulation` struct holds a simulation environment, a list of agents, a list of metrics handlers, elapsed time of the simulation, a pause state, and a speedup factor. The environment contains a railway graph and a list of movable railway objects. Agents are used to make decisions for objects, and metrics handlers process events and gather metrics.

The core functionality of the simulation is implemented in several methods:

1. `new`: Creates a new simulation with the given railway graph.
2. `get_observable_environment`: Returns a reference to the observable environment of the simulation, which allows external components to access the state of the simulation without being able to modify it.
3. `add_object`: Adds a movable railway object to the simulation and associates a decision agent with it if provided.
4. `remove_object`: Removes a movable railway object from the simulation.
5. `add_agent_for_object`: Adds a decision agent for an object in the simulation.
6. `register_metrics_handler`: Registers a metrics handler for the simulation.
7. `handle_event`: Handles a simulation event by passing it to all registered metrics handlers.
8. `update`: Updates the simulation state based on the given delta time and the speedup factor. This method is called periodically to advance the simulation.
9. `update_object`: Updates the state of the object with the given id based on the given delta time. It is called internally by the `update` method.
10. `update_object_position`: Updates the position of the object with the given id based on the given delta time. It is called internally by the `update_object` method.
11. `update_train_target`: Updates the target of the train with the given id. It is called internally by the `update_object` method.

The simulation also includes additional modules such as `agents`, `environment`, `commands`, `events`, and `metrics` to provide more specialized functionality.

The `agents` module contains types related to decision-making agents for the movable railway objects. The `environment` module contains types related to the simulation environment and an `ObservableEnvironment` trait to provide read-only access to the environment. The `commands` module contains types related to commands that can be issued to the movable railway objects. The `events` module contains types related to events that occur during the simulation. Finally, the `metrics` module contains types related to metrics handlers that process events and gather metrics during the simulation run.

In conclusion, the railway simulation module provides a comprehensive framework for creating, managing, and updating a railway simulation. The simulation consists of a railway graph, movable railway objects, decision agents, and metrics handlers to process events and gather metrics. The modular structure allows for easy extension and customization, making it a suitable choice for a wide range of railway simulation applications.
